use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::audio::{
    AudioTranscriptionRequest, AudioTranscriptionResponse, TimestampGranularity, WHISPER_1,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    // Simple transcription
    let simple_req = AudioTranscriptionRequest::new(
        "examples/data/problem.mp3".to_string(),
        WHISPER_1.to_string(),
    );

    let simple_result = client.audio_transcription(simple_req).await?;
    println!("\n=== Simple Transcription ===");
    match simple_result {
        AudioTranscriptionResponse::Simple { text, .. } => {
            println!("Text: {}", text);
        }
        AudioTranscriptionResponse::Verbose { .. } => {
            println!("Unexpected verbose response");
        }
    }

    // Verbose transcription with word timestamps
    let verbose_word_req = AudioTranscriptionRequest::new(
        "examples/data/problem.mp3".to_string(),
        WHISPER_1.to_string(),
    )
    .response_format("verbose_json".to_string())
    .timestamp_granularities(vec![TimestampGranularity::Word]);

    // Debug print the request
    println!("Request: {}", serde_json::to_string_pretty(&verbose_word_req).unwrap());

    let verbose_word_result = client.audio_transcription(verbose_word_req).await?;
    println!("\n=== Verbose Word Request as JSON ===");
    println!("{}", serde_json::to_string_pretty(&verbose_word_result).unwrap());
    println!("\n=== Verbose Transcription with Word Timestamps ===");
    match verbose_word_result {
        AudioTranscriptionResponse::Simple { .. } => {
            println!("Unexpected simple response");
        }
        AudioTranscriptionResponse::Verbose {
            text,
            words,
            language,
            duration,
            ..
        } => {
            println!("Language: {}", language);
            println!("Duration: {:.2} seconds", duration);
            println!("Text: {}", text);
            if let Some(words) = words {
                println!("\nWord Timestamps:");
                for word in words {
                    println!(
                        "Word: '{}' ({}s -> {}s)",
                        word.word, word.start, word.end
                    );
                }
            }
        }
    }

    // Verbose transcription with segment timestamps
    let verbose_segment_req = AudioTranscriptionRequest::new(
        "examples/data/problem.mp3".to_string(),
        WHISPER_1.to_string(),
    )
    .response_format("verbose_json".to_string())
    .timestamp_granularities(vec![TimestampGranularity::Segment]);

    let verbose_segment_result = client.audio_transcription(verbose_segment_req).await?;
    println!("\n=== Verbose Transcription with Segment Timestamps ===");
    match verbose_segment_result {
        AudioTranscriptionResponse::Simple { .. } => {
            println!("Unexpected simple response");
        }
        AudioTranscriptionResponse::Verbose {
            text,
            segments,
            language,
            duration,
            ..
        } => {
            println!("Language: {}", language);
            println!("Duration: {:.2} seconds", duration);
            println!("Text: {}", text);
            if let Some(segments) = segments {
                println!("\nSegment Timestamps:");
                for segment in segments {
                    println!(
                        "Segment {}: '{}' ({}s -> {}s)",
                        segment.id, segment.text, segment.start, segment.end
                    );
                    println!("  Avg Log Prob: {:.4}", segment.avg_logprob);
                    println!("  No Speech Prob: {:.4}", segment.no_speech_prob);
                }
            }
        }
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example audio_transcriptions
