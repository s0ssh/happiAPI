use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    name: String,
    elevenlabs: ElevenLabsConfig,
    openai: OpenAIConfig
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsConfig {
    tts: ElevenLabsTTSConfig,
    stt: ElevenLabsSTTConfig
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsTTSConfig {
    model_id: String,
    output_format: String,
    voice_id: String
}

#[derive(Debug, Deserialize)]
pub struct ElevenLabsSTTConfig {
    model_id: String
}

#[derive(Debug, Deserialize)]
pub struct OpenAIConfig {
    model_id: String,
    model_format: String,
    model_temp: f32,
    model_tokens: i32,
    model_top_p: f32,
    model_store: bool,
    system_prompt: String
} 
