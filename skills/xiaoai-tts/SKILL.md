---
name: xiaoai-tts
description: Control Xiaoai speaker via OpenXiaoAI Voice API for high-quality TTS playback. Use when the user wants to play voice notifications, announcements, or TTS through the Xiaoai speaker using the OpenXiaoAI HTTP API. Supports Doubao (ByteDance) TTS with emotions, voice types, and speed control. Triggers on queries like "小爱播报", "语音播报", "让小爱说", "读出来".
---

# XiaoAI TTS

通过 XiaoAI HTTP API 控制小爱音箱播放语音，支持火山引擎豆包 TTS 多情感、多音色、语速调整等高级功能。

## 前置配置

在 env 中添加：

```bash
OPENXIAOAI_BASE_URL="http://192.168.x.x:9092"  # OpenXiaoAI 服务地址
```

## 使用方法

### 内置音色（小爱同学自带 TTS）

```bash
# 播放文字
python3 scripts/play_text.py "你好，小爱音箱"

# 阻塞模式（等待播放完成）
python3 scripts/play_text.py "你好" --blocking

# 指定超时时间
python3 scripts/play_text.py "你好" --timeout 30000
```

### 云端音色（Doubao TTS）🌟 推荐优先使用 🌟

```bash
# 基础调用（默认音色）
python3 scripts/tts_doubao.py "你好，我是小爱语音助手"

# 指定音色
python3 scripts/tts_doubao.py "你好" --speaker zh_female_vv_uranus_bigtts

# 调整语速（0.8-2.0）
python3 scripts/tts_doubao.py "你好" --speed 1.2

# 指定情感（仅多情感音色支持）
python3 scripts/tts_doubao.py "你怎么能这样！" --speaker zh_male_lengkugege_emo_v2_mars_bigtts --emotion angry

# 2.0 音色 + 上下文指令
python3 scripts/tts_doubao.py "这是一个很长的句子" --speaker zh_female_vv_uranus_bigtts --context "你可以说慢一点吗？"
```

### 其他控制

```bash
# 唤醒小爱（相当于喊"小爱同学"）
python3 scripts/control.py wakeup

# 静默唤醒（不播放提示音）
python3 scripts/control.py wakeup --silent

# 检查服务健康状态
python3 scripts/control.py health

# 获取音箱播放状态
python3 scripts/control.py status

# 播放远程音频 URL
python3 scripts/play_url.py "http://example.com/audio.mp3"

# 播放本地音频文件
python3 scripts/play_file.py /path/to/audio.mp3
```

### 获取音色列表

```bash
# 获取所有音色
python3 scripts/list_doubao_voices.py

# 仅获取 2.0 音色
python3 scripts/list_doubao_voices.py --version 2.0

# 仅获取 1.0 音色
python3 scripts/list_doubao_voices.py --version 1.0
```

## 常用音色推荐

### 2.0 音色（推荐）
| 音色名称 | voice_type | 特点 |
|---------|------------|------|
| Vivi 2.0 | zh_female_vv_uranus_bigtts | 通用场景，情感变化 |
| 小何 2.0 | zh_female_xiaohe_uranus_bigtts | 通用场景 |
| 云舟 2.0 | zh_male_m191_uranus_bigtts | 通用场景 |
| 小天 2.0 | zh_male_taocheng_uranus_bigtts | 通用场景 |

### 1.0 音色（多情感）
| 音色名称 | voice_type | 特点 |
|---------|------------|------|
| 冷酷哥哥 | zh_male_lengkugege_emo_v2_mars_bigtts | 支持 emotion 参数 |
| 高冷御姐 | zh_female_gaolengyujie_emo_v2_mars_bigtts | 支持 emotion 参数 |
| 灿灿 | zh_female_cancan_mars_bigtts | 通用场景 |
| 爽快思思 | zh_female_shuangkuaisisi_moon_bigtts | 通用场景 |

## 情感参数（仅多情感音色）

| 中文情感 | 英文参数 | 说明 |
|---------|----------|------|
| 开心 | happy | 愉快语气 |
| 悲伤 | sad | 悲伤语气 |
| 生气 | angry | 愤怒语气 |
| 惊讶 | surprised | 惊讶语气 |
| 撒娇 | lovey-dovey | 撒娇语气 |
| 温柔 | tender | 温柔语气 |
| 讲故事 | storytelling | 讲故事语气 |
| 新闻播报 | news | 新闻播报语气 |
| 广告营销 | advertising | 广告营销语气 |
| 磁性 | magnetic | 磁性声音 |

完整情感列表参考：`python3 scripts/list_doubao_voices.py`

## API 端点

- Base URL: `http://{host}:9092`
- Content-Type: `application/json`
```
