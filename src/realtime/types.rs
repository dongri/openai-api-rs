use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Session {
    Realtime(RealtimeSession),
    Transcription(TranscriptionSession),
}
impl Default for Session {
    fn default() -> Self {
        Self::Realtime(Default::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TranscriptionSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<AdditionalServerOutput>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<AdditionalServerOutput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<RealtimeModel>,
    /// Just `Audio` by default. Can also be `Text` for text-only. Both at the same time are not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<OutputModality>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,
    // Todo: Support prompt template reference and variables
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub prompt: Option<PromptReference>,
    // Todo: Support tracing config
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tracing: Option<TracingMode>, // "auto" or config object
    // Todo: Support truncation config (poorly documented atm)
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tracing: Option<TracingMode>, // "auto" or config object
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RealtimeModel {
    #[serde(rename = "gpt-realtime")]
    GptRealtime,
    #[serde(rename = "gpt-4o-realtime-preview")]
    Gpt4oRealtimePreview,
    #[serde(rename = "gpt-4o-mini-realtime-preview")]
    Gpt4oMiniRealtimePreview,
    #[serde(rename = "gpt-realtime-2025-08-28")]
    GptRealtime20250828,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    Gpt4oRealtimePreview20241217,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    Gpt4oRealtimePreview20241001,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    Gpt4oMiniRealtimePreview20241217,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdditionalServerOutput {
    /// Include logprobs for input audio transcription.
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    Logprobs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OutputModality {
    Audio,
    Text,
}

/// Enum representing the only possible value for `type` in the accept call payload.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeCallSessionType {
    Realtime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    Alloy,
    Ash,
    Ballad,
    Cedar,
    Coral,
    Echo,
    Marin,
    Sage,
    Shimmer,
    Verse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    pub input: AudioInput,
    pub output: AudioOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioInput {
    pub format: AudioFormat,
    /// Configuration for input audio noise reduction. This can be set to null to turn off. Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
    /// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
    pub noise_reduction: Option<NoiseReduction>,
    /// Configuration for input audio transcription, defaults to off and can be set to null to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through the /audio/transcriptions endpoint and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
    pub transcription: Option<TranscriptionConfig>,
    /// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to null to turn off, in which case the client must manually trigger model response.
    pub turn_detection: Option<TurnDetection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionConfig {
    /// The language of the input audio in ISO-639-1 (e.g. "en") format. Will improve accuracy and latency if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub model: TranscriptionModel,
    /// An optional text to guide the model's style or continue a previous audio segment. For `whisper-1`, the prompt is a list of keywords. For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TranscriptionModel {
    #[serde(rename = "whisper-1")]
    Whisper1,
    #[serde(rename = "gpt-4o-transcribe-latest")]
    Gpt4oTranscribeLatest,
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VadMode {
    SemanticVad(SemanticVadConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerVadConfig {
    /// Whether or not to automatically generate a response when a VAD stop event occurs.
    pub create_response: bool,
    /// Optional timeout after which a model response will be triggered automatically. This is useful for situations in which a long pause from the user is unexpected, such as a phone call. The model will effectively prompt the user to continue the conversation based on the current context.
    /// The timeout value will be applied after the last model response's audio has finished playing, i.e. it's set to the `response.done` time plus audio playback duration.
    /// An `input_audio_buffer.timeout_triggered` event (plus events associated with the Response) will be emitted when the timeout is reached. Idle timeout is currently only supported for server_vad mode.
    pub idle_timeout_ms: Option<u32>,
    /// Whether or not to automatically interrupt any ongoing response with output to the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
    pub interrupt_response: bool,
    /// Used only for server_vad mode. Amount of audio to include before the VAD detected speech (in milliseconds). Defaults to 300ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u32>,
    /// Used only for server_vad mode. Duration of silence to detect speech stop (in milliseconds). Defaults to 500ms. With shorter values the model will respond more quickly, but may jump in on short pauses from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u32>,
    /// Used only for server_vad mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A higher threshold will require louder audio to activate the model, and thus might perform better in noisy environments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticVadConfig {
    /// Whether or not to automatically generate a response when a VAD stop event occurs.
    pub create_response: bool,
    pub eagerness: SemanticVadEagerness,
    /// Whether or not to automatically interrupt any ongoing response with output to the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
    pub interrupt_response: bool,
}

/// low will wait longer for the user to continue speaking, high will respond more quickly. auto is the default and is equivalent to medium. low, medium, and high have max timeouts of 8s, 4s, and 2s respectively.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SemanticVadEagerness {
    /// Equivalent to Medium.
    Auto,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoiseReduction {
    #[serde(rename = "type")]
    pub reduction_type: NoiseReductionType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NoiseReductionType {
    /// `near_field` is for close-talking microphones such as headphones
    NearField,
    /// `far_field` is for far-field microphones such as laptop or conference room microphones
    FarField,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioOutput {
    pub format: AudioFormat,
    /// The speed of the model's spoken response as a multiple of the original speed. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.
    /// This parameter is a post-processing adjustment to the audio after it is generated, it's also possible to prompt the model to speak faster or slower.
    pub speed: f64,
    /// The voice the model uses to respond. Voice cannot be changed during the session once the model has responded with audio at least once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AudioFormat {
    Pcm(AudioFormatDefinitionWithSampleRate),
    Other(AudioFormatDefinition),
}

/// This form of audio format definition is *only* used for the raw PCM format.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFormatDefinitionWithSampleRate {
    /// This must always be `24000` for PCM.
    pub rate: i32,
    /// Must be `Pcm`.
    #[serde(rename = "type")]
    pub audio_type: AudioFormatIdentifier,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFormatDefinition {
    #[serde(rename = "type")]
    pub audio_type: AudioFormatIdentifier,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AudioFormatIdentifier {
    #[serde(rename = "audio/pcm")]
    Pcm,
    #[serde(rename = "audio/pcmu")]
    G711ULAW,
    #[serde(rename = "audio/pcma")]
    G711ALAW,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioTranscription {
    pub language: Option<String>,
    pub model: Option<String>,
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TurnDetection {
    #[serde(rename = "server_vad")]
    ServerVAD(ServerVadConfig),
    SemanticVAD(SemanticVadConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ToolDefinition {
    #[serde(rename = "function")]
    Function {
        name: String,
        description: String,
        parameters: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    #[serde(untagged)]
    Function {
        r#type: FunctionType,
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MaxOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    Num(u16),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Message,
    FunctionCall,
    FunctionCallOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    Completed,
    InProgress,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemContentType {
    InputText,
    InputAudio,
    Text,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemContent {
    pub r#type: ItemContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ItemRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ItemContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl TryFrom<serde_json::Value> for Item {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIError {
    pub r#type: String,
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub status: ResponseStatus,
    pub status_details: Option<ResponseStatusDetail>,
    pub output: Vec<Item>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ResponseStatusDetail {
    #[serde(rename = "cancelled")]
    Cancelled { reason: CancelledReason },
    #[serde(rename = "incomplete")]
    Incomplete { reason: IncompleteReason },
    #[serde(rename = "failed")]
    Failed { error: Option<FailedError> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailedError {
    pub code: Option<String>,
    pub message: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CancelledReason {
    TurnDetected,
    ClientCancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteReason {
    Interruption,
    MaxOutputTokens,
    ContentFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "audio")]
    Audio {
        audio: Option<String>,
        transcript: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimit {
    pub name: String,
    pub limit: u32,
    pub remaining: u32,
    pub reset_seconds: f32,
}
