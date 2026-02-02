//! Type definitions for the Ollama simulator
//!
//! This module contains all the data structures used to control
//! simulator behavior and represent Ollama API responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Behavior mode for a simulated server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ServerBehavior {
    /// Server operates normally, streaming responses with realistic timing
    Normal {
        /// Tokens per second for generation (default: 65 for qwen3-32b class models)
        #[serde(default = "default_tokens_per_sec")]
        tokens_per_sec: f64,
        /// Tokens per second for prompt evaluation (default: 2900 for RTX 5090)
        #[serde(default = "default_prompt_eval_tokens_per_sec")]
        prompt_eval_tokens_per_sec: f64,
        /// Number of tokens to generate before stopping
        #[serde(default = "default_num_tokens")]
        num_tokens: usize,
        /// Simulated model loading time in milliseconds (0 = already loaded)
        #[serde(default)]
        load_delay_ms: u64,
    },

    /// Server refuses connections (simulates server being off)
    ConnectionRefused,

    /// Server accepts connection but never responds (hangs indefinitely)
    Hang,

    /// Server accepts connection, starts response, then fails mid-stream
    FailMidStream {
        /// Number of tokens to send before failing
        tokens_before_fail: usize,
        /// Tokens per second for generation
        #[serde(default = "default_tokens_per_sec")]
        tokens_per_sec: f64,
    },

    /// Server accepts connection, sends headers, then times out (no data)
    TimeoutAfterHeaders {
        /// Milliseconds to wait before timing out
        #[serde(default = "default_timeout_ms")]
        timeout_ms: u64,
    },

    /// Server responds very slowly (testing timeout handling)
    Slow {
        /// Tokens per second (very slow, e.g., 0.5)
        tokens_per_sec: f64,
        /// Number of tokens to generate
        num_tokens: usize,
    },

    /// Server returns an HTTP error status code
    HttpError {
        /// HTTP status code to return
        status_code: u16,
        /// Error message body
        message: String,
    },

    /// Server returns 404 for model not found
    ModelNotFound {
        /// Model name that was requested
        model_name: String,
    },

    /// Server abruptly closes connection after partial response
    AbruptClose {
        /// Bytes to send before closing
        bytes_before_close: usize,
    },

    /// Server has custom response behavior per request path
    Custom {
        /// Custom response content
        response_body: String,
        /// HTTP status code
        status_code: u16,
        /// Content-Type header
        content_type: String,
        /// Delay before responding in milliseconds
        delay_ms: u64,
    },
}

fn default_tokens_per_sec() -> f64 {
    65.0  // ~65 tok/s generation for qwen3-32b class models
}

fn default_prompt_eval_tokens_per_sec() -> f64 {
    2900.0  // ~2900 tok/s prompt ingestion on RTX 5090
}

fn default_num_tokens() -> usize {
    20
}

fn default_timeout_ms() -> u64 {
    60000
}

impl Default for ServerBehavior {
    fn default() -> Self {
        ServerBehavior::Normal {
            tokens_per_sec: default_tokens_per_sec(),
            prompt_eval_tokens_per_sec: default_prompt_eval_tokens_per_sec(),
            num_tokens: default_num_tokens(),
            load_delay_ms: 0,
        }
    }
}

/// State of a single simulated server
#[derive(Debug, Clone)]
pub struct SimulatedServerState {
    /// The port this server listens on
    #[allow(dead_code)]
    pub port: u16,
    /// Current behavior mode
    pub behavior: ServerBehavior,
    /// Whether the server is accepting connections
    pub accepting_connections: bool,
    /// Models "installed" on this server
    pub installed_models: Vec<ModelInfo>,
    /// Currently "loaded" model (in VRAM)
    pub loaded_model: Option<String>,
    /// Request counter
    pub request_count: u64,
    /// Active requests (for tracking cancellation)
    pub active_requests: HashMap<String, ActiveRequest>,
    /// KV cache: tokens currently cached (prompt + previous response)
    /// Used for prefix matching - if incoming prompt shares prefix, those tokens are "free"
    pub kv_cache_tokens: Vec<String>,
}

impl SimulatedServerState {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            behavior: ServerBehavior::default(),
            accepting_connections: true,
            installed_models: vec![ModelInfo::default_test_model()],
            loaded_model: None,
            request_count: 0,
            active_requests: HashMap::new(),
            kv_cache_tokens: Vec::new(),
        }
    }

    /// Clear the KV cache (e.g., when model is unloaded or switched)
    #[allow(dead_code)]
    pub fn clear_kv_cache(&mut self) {
        self.kv_cache_tokens.clear();
    }
}

/// Information about an installed model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub digest: String,
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
}

impl ModelInfo {
    pub fn default_test_model() -> Self {
        Self {
            name: "test-model:latest".to_string(),
            size: 4_000_000_000,
            digest: "abc123def456abc123def456abc123def456abc123def456abc123def456abc1".to_string(),
            family: "test".to_string(),
            parameter_size: "7B".to_string(),
            quantization_level: "Q4_K_M".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn deepseek_coder() -> Self {
        Self {
            name: "deepseek-coder:1.3b-instruct-q4_0".to_string(),
            size: 1_300_000_000,
            digest: "def456abc123def456abc123def456abc123def456abc123def456abc123def4".to_string(),
            family: "deepseek".to_string(),
            parameter_size: "1.3B".to_string(),
            quantization_level: "Q4_0".to_string(),
        }
    }
}

/// Active request tracking
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ActiveRequest {
    pub request_id: String,
    pub started_at: std::time::Instant,
    pub tokens_sent: usize,
    pub cancelled: bool,
}

/// Control API request to set server behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBehaviorRequest {
    /// Port of the server to configure (or "all" for all servers)
    pub port: PortSelector,
    /// New behavior to set
    pub behavior: ServerBehavior,
}

/// Selector for which port(s) to target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortSelector {
    Single(u16),
    All(AllPorts),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllPorts {
    pub all: bool,
}

/// Control API request to set installed models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetModelsRequest {
    pub port: PortSelector,
    pub models: Vec<ModelInfo>,
}

/// Control API request to reset all servers to default state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetRequest {
    /// If true, also clears request counters
    #[serde(default)]
    pub clear_counters: bool,
}

/// Control API response for server status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatusResponse {
    pub port: u16,
    pub behavior: String,
    pub accepting_connections: bool,
    pub installed_models: Vec<String>,
    pub loaded_model: Option<String>,
    pub request_count: u64,
    pub active_request_count: usize,
}

/// Control API response for overall status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub servers: Vec<ServerStatusResponse>,
}

/// Ollama API response types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaTagsResponse {
    pub models: Vec<OllamaModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModelEntry {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: OllamaModelDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModelDetails {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaPsResponse {
    pub models: Vec<OllamaPsEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaPsEntry {
    pub name: String,
    pub model: String,
    pub size: u64,
    pub digest: String,
    pub details: OllamaModelDetails,
    pub expires_at: String,
    pub size_vram: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaChatRequest {
    pub model: String,
    #[serde(default)]
    pub messages: Vec<OllamaChatMessage>,
    #[serde(default)]
    pub stream: Option<bool>,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
    #[serde(default)]
    pub keep_alive: Option<serde_json::Value>,
    #[serde(default)]
    pub raw: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaChatMessage,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct OllamaGenerateRequest {
    pub model: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub stream: Option<bool>,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaVersionResponse {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaErrorResponse {
    pub error: String,
}

/// Test scenario results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}

/// Result of prompt evaluation with KV cache consideration
#[derive(Debug, Clone)]
pub struct PromptEvalResult {
    /// All tokens from the prompt (for caching after response)
    pub prompt_tokens: Vec<String>,
    /// Number of tokens that were already cached (prefix match)
    pub cached_token_count: usize,
    /// Number of new tokens that need processing
    #[allow(dead_code)]
    pub new_token_count: usize,
    /// Calculated delay in milliseconds for processing new tokens
    pub prompt_eval_delay_ms: u64,
}

/// Simple tokenizer for simulation purposes
/// Splits on whitespace and punctuation, roughly approximating subword tokenization
pub fn simple_tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in text.chars() {
        if c.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            // Include space as separate token (like real tokenizers often do)
            if c == ' ' {
                tokens.push(" ".to_string());
            }
        } else if c.is_ascii_punctuation() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

/// Calculate prompt eval timing based on KV cache state
pub fn calculate_prompt_eval(
    messages: &[OllamaChatMessage],
    cached_tokens: &[String],
    prompt_eval_tokens_per_sec: f64,
) -> PromptEvalResult {
    // Tokenize all messages into a single token stream
    let mut prompt_tokens = Vec::new();
    for msg in messages {
        // Add role marker
        prompt_tokens.push(format!("<|{}|>", msg.role));
        // Add content tokens
        prompt_tokens.extend(simple_tokenize(&msg.content));
    }

    // Find prefix match with cache
    let cached_token_count = prompt_tokens
        .iter()
        .zip(cached_tokens.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let new_token_count = prompt_tokens.len().saturating_sub(cached_token_count);

    // Calculate delay: new tokens / tokens_per_sec * 1000ms
    let prompt_eval_delay_ms = if prompt_eval_tokens_per_sec > 0.0 {
        ((new_token_count as f64 / prompt_eval_tokens_per_sec) * 1000.0) as u64
    } else {
        0
    };

    PromptEvalResult {
        prompt_tokens,
        cached_token_count,
        new_token_count,
        prompt_eval_delay_ms,
    }
}
