pub mod decoder;
pub mod doubao;

use crate::tts::decoder::StreamingDecoder;
use crate::tts::doubao::DoubaoStreamClient;
use open_xiaoai::services::connect::message::MessageManager;
use pyo3::prelude::*;
use tokio::sync::mpsc;

const STREAM_BUFFER_THRESHOLD: usize = 8192;

/// Stream TTS: fetch audio from Doubao API, decode to PCM, and play via WebSocket.
/// Supports MP3, OGG Vorbis, WAV, FLAC formats.
/// This runs entirely in Rust/tokio — no Python GIL contention.
#[pyfunction]
#[pyo3(signature = (text, app_id, access_key, resource_id, speaker, speed=1.0, format="mp3".to_string(), sample_rate=24000, emotion=None, context_texts=None))]
pub fn tts_stream_play(
    py: Python<'_>,
    text: String,
    app_id: String,
    access_key: String,
    resource_id: String,
    speaker: String,
    speed: f32,
    format: String,
    sample_rate: u32,
    emotion: Option<String>,
    context_texts: Option<Vec<String>>,
) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let client = DoubaoStreamClient::new(app_id, access_key, resource_id, speaker);

        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(16);

        // Spawn the HTTP streaming task
        let fetch_handle = tokio::spawn({
            let text = text.clone();
            let format = format.clone();
            async move {
                client
                    .stream_audio(&text, &format, sample_rate, speed, context_texts, emotion, tx)
                    .await
            }
        });

        // Accumulate audio chunks + decode + play
        let mut decoder = StreamingDecoder::new(&format, sample_rate);
        let mut accumulated_size: usize = 0;
        let mut first_chunk = true;

        while let Some(chunk) = rx.recv().await {
            accumulated_size += chunk.len();
            decoder.feed(&chunk);

            // Decode and send when we have enough data
            if accumulated_size >= STREAM_BUFFER_THRESHOLD {
                match decoder.decode_all() {
                    Ok(pcm) if !pcm.is_empty() => {
                        if first_chunk {
                            crate::pylog!("[TTS] Stream started playing first chunk");
                            first_chunk = false;
                        }
                        let _ = MessageManager::instance()
                            .send_stream("play", pcm, None)
                            .await;
                    }
                    Ok(_) => {} // Empty decode, continue accumulating
                    Err(e) => {
                        crate::pylog!("[TTS] Decode error (continuing): {}", e);
                    }
                }
                accumulated_size = 0;
            }
        }

        // Decode any remaining data
        match decoder.decode_all() {
            Ok(pcm) if !pcm.is_empty() => {
                if first_chunk {
                    crate::pylog!("[TTS] Stream playing (single chunk)");
                }
                let _ = MessageManager::instance()
                    .send_stream("play", pcm, None)
                    .await;
            }
            Ok(_) => {}
            Err(e) => {
                crate::pylog!("[TTS] Final decode error: {}", e);
            }
        }

        // Check if the fetch task had errors
        if let Ok(Err(e)) = fetch_handle.await {
            crate::pylog!("[TTS] Stream fetch error: {}", e);
        }

        crate::pylog!("[TTS] Stream playback completed");
        Ok(())
    })
}

pub fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tts_stream_play, m)?)?;
    Ok(())
}
