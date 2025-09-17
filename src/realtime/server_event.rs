use serde::{Deserialize, Serialize};

use crate::realtime::types::{
    APIError, ContentPart, Conversation, Item, RateLimit, Response, Session,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub event_id: String,
    pub error: APIError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionCreated {
    pub event_id: String,
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionUpdated {
    pub event_id: String,
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationCreated {
    pub event_id: String,
    pub conversation: Conversation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudioBufferCommited {
    pub event_id: String,
    pub previous_item_id: Option<String>,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudioBufferCleared {
    pub event_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudioBufferSpeechStarted {
    pub event_id: String,
    pub audio_start_ms: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudioBufferSpeechStopped {
    pub event_id: String,
    pub audio_end_ms: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemAdded {
    pub event_id: String,
    pub previous_item_id: Option<String>,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemDone {
    pub event_id: String,
    pub previous_item_id: Option<String>,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemInputAudioTranscriptionCompleted {
    pub event_id: String,
    pub item_id: String,
    pub content_index: u32,
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemInputAudioTranscriptionDelta {
    pub event_id: String,
    pub item_id: String,
    pub content_index: u32,
    pub delta: String,
    // todo: add logprobs support
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemInputAudioTranscriptionFailed {
    pub event_id: String,
    pub item_id: String,
    pub content_index: u32,
    pub error: APIError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemTruncated {
    pub event_id: String,
    pub item_id: String,
    pub content_index: u32,
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemDeleted {
    pub event_id: String,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputAudioBufferStarted {
    pub event_id: String,
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputAudioBufferStopped {
    pub event_id: String,
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputAudioBufferCleared {
    pub event_id: String,
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseCreated {
    pub event_id: String,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseDone {
    pub event_id: String,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputItemAdded {
    pub event_id: String,
    pub response_id: String,
    pub output_index: u32,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputItemDone {
    pub event_id: String,
    pub response_id: String,
    pub output_index: u32,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseContentPartAdded {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseContentPartDone {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseTextDelta {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseTextDone {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputAudioTranscriptDelta {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputAudioTranscriptDone {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputAudioDelta {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseOutputAudioDone {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseFunctionCallArgumentsDelta {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub call_id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseFunctionCallArgumentsDone {
    pub event_id: String,
    pub response_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub call_id: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMcpCallArgumentsDelta {
    pub event_id: String,
    pub item_id: String,
    #[serde(default)]
    pub obfuscation: Option<String>,
    pub output_index: u32,
    pub response_id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMcpCallArgumentsDone {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
    pub response_id: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMcpCallInProgress {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMcpCallCompleted {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMcpCallFailed {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitsUpdated {
    pub event_id: String,
    pub rate_limits: Vec<RateLimit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpListToolsInProgress {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpListToolsCompleted {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpListToolsFailed {
    pub event_id: String,
    pub item_id: String,
    pub output_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerEvent {
    #[serde(rename = "error")]
    Error(Error),
    #[serde(rename = "session.created")]
    SessionCreated(SessionCreated),
    #[serde(rename = "session.updated")]
    SessionUpdated(SessionUpdated),
    #[serde(rename = "conversation.created")]
    ConversationCreated(ConversationCreated),
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommited(InputAudioBufferCommited),
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared(InputAudioBufferCleared),
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted(InputAudioBufferSpeechStarted),
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped(InputAudioBufferSpeechStopped),
    #[serde(rename = "conversation.item.added")]
    ConversationItemAdded(ConversationItemAdded),
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted(
        ConversationItemInputAudioTranscriptionCompleted,
    ),
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta(ConversationItemInputAudioTranscriptionDelta),
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed(ConversationItemInputAudioTranscriptionFailed),
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated(ConversationItemTruncated),
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted(ConversationItemDeleted),
    #[serde(rename = "conversation.item.done")]
    ConversationItemDone(ConversationItemDone),
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted(OutputAudioBufferStarted),
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped(OutputAudioBufferStopped),
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared(OutputAudioBufferCleared),
    #[serde(rename = "response.created")]
    ResponseCreated(ResponseCreated),
    #[serde(rename = "response.done")]
    ResponseDone(ResponseDone),
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(ResponseOutputItemAdded),
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(ResponseOutputItemDone),
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(ResponseContentPartAdded),
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(ResponseContentPartDone),
    #[serde(rename = "response.text.delta")]
    ResponseTextDelta(ResponseTextDelta),
    #[serde(rename = "response.text.done")]
    ResponseTextDone(ResponseTextDone),
    #[serde(rename = "response.output_audio_transcript.delta")]
    ResponseOutputAudioTranscriptDelta(ResponseOutputAudioTranscriptDelta),
    #[serde(rename = "response.output_audio_transcript.done")]
    ResponseOutputAudioTranscriptDone(ResponseOutputAudioTranscriptDone),
    #[serde(rename = "response.output_audio.delta")]
    ResponseOutputAudioDelta(ResponseOutputAudioDelta),
    #[serde(rename = "response.output_audio.done")]
    ResponseOutputAudioDone(ResponseOutputAudioDone),
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDelta),
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDone),
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta(ResponseMcpCallArgumentsDelta),
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone(ResponseMcpCallArgumentsDone),
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress(ResponseMcpCallInProgress),
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted(ResponseMcpCallCompleted),
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed(ResponseMcpCallFailed),
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated(RateLimitsUpdated),
    #[serde(rename = "mcp_list_tools.in_progress")]
    McpListToolsInProgress(McpListToolsInProgress),
    #[serde(rename = "mcp_list_tools.completed")]
    McpListToolsCompleted(McpListToolsCompleted),
    #[serde(rename = "mcp_list_tools.failed")]
    McpListToolsFailed(McpListToolsFailed),
}
