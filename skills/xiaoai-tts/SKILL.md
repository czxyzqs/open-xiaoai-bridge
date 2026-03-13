---
name: xiaoai-tts
description: Control Xiaoai speaker via OpenXiaoAI Voice API for high-quality TTS playback. Use when the user wants to play voice notifications, announcements, or TTS through the Xiaoai speaker using the OpenXiaoAI HTTP API. Supports Doubao (ByteDance) TTS with emotions, voice types, and speed control. Triggers on queries like "小爱播报", "语音播报", "让小爱说", "读出来", "播报"， "xiaoai-tts".
---

# XiaoAI TTS

通过 XiaoAI HTTP API 控制小爱音箱播放语音，支持火山引擎豆包 TTS 多情感、多音色、语速调整等高级功能。

## 前置配置

在 env 中添加：

```bash
OPENXIAOAI_BASE_URL="http://192.168.x.x:9092"  # OpenXiaoAI 服务地址
```

## 使用方法

```bash
# 语音播报（优先 Doubao TTS，失败自动回退小爱自带 TTS）
xiaoai-tts tts "你好，我是小爱语音助手"

# 指定音色
xiaoai-tts tts "你好" -s zh_female_vv_uranus_bigtts

# 调整语速（0.8-2.0）
xiaoai-tts tts "你好" --speed 1.2

# 指定情感（仅多情感音色支持）
xiaoai-tts tts "你怎么能这样！" -s zh_male_lengkugege_emo_v2_mars_bigtts -m angry

# 2.0 音色 + 上下文指令
xiaoai-tts tts "这是一个很长的句子" -s zh_female_vv_uranus_bigtts -c "你可以说慢一点吗？"

# 强制使用小爱自带 TTS
xiaoai-tts text "你好"

# 唤醒小爱
xiaoai-tts wakeup
xiaoai-tts wakeup --silent   # 静默唤醒

# 状态 / 健康检查
xiaoai-tts status
xiaoai-tts health

# 播放音频
xiaoai-tts file /path/to/audio.mp3
xiaoai-tts url "http://example.com/audio.mp3"

# 查看可用音色
xiaoai-tts voices            # 所有音色
xiaoai-tts voices -v 2.0     # 仅 2.0 音色
xiaoai-tts voices -v 1.0     # 仅 1.0 音色
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

完整情感列表参考：`xiaoai-tts voices`

## API 端点

- Base URL: `http://{host}:9092`
- Content-Type: `application/json`
```
