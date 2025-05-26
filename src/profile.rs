use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub elevenlabs: ElevenLabsConfig,
    pub openai: OpenAIConfig,
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsConfig {
    pub tts: ElevenLabsTTSConfig,
    pub stt: ElevenLabsSTTConfig,
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsTTSConfig {
    pub model_id: String,
    pub output_format: String,
    pub voice_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsSTTConfig {
    pub model_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIConfig {
    pub model_id: String,
    pub model_format: String,
    pub model_temperature: f32,
    pub model_tokens: i32,
    pub model_top_p: f32,
    pub model_store: bool,
    pub system_prompt: String,
}
