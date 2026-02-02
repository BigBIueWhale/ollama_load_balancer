//! Simulated Ollama server implementation
//!
//! This module implements a realistic mock Ollama server that can be
//! programmatically controlled to exhibit various behaviors for testing.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use futures_util::stream::{self, StreamExt};
use chrono::Utc;

use crate::types::{
    ServerBehavior, SimulatedServerState, ModelInfo,
    OllamaTagsResponse, OllamaModelEntry, OllamaModelDetails,
    OllamaPsResponse, OllamaPsEntry, OllamaChatRequest, OllamaChatResponse,
    OllamaChatMessage, OllamaVersionResponse, OllamaErrorResponse,
    calculate_prompt_eval, PromptEvalResult,
};

/// Global state for all simulated servers
#[derive(Debug)]
pub struct SimulatorState {
    pub servers: HashMap<u16, SimulatedServerState>,
}

impl SimulatorState {
    pub fn new(ports: Vec<u16>) -> Self {
        let servers = ports
            .into_iter()
            .map(|port| (port, SimulatedServerState::new(port)))
            .collect();
        Self { servers }
    }
}

/// Run a simulated Ollama server on the specified port
pub async fn run_ollama_server(
    port: u16,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(move |_| {
        let state = state.clone();
        let port = port;
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_ollama_request(req, state.clone(), port)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_svc);

    server.await.map_err(|e| {
        format!("Ollama server {} error: {}", port, e).into()
    })
}

async fn handle_ollama_request(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
) -> Result<Response<Body>, Infallible> {
    // Get behavior and increment request count
    let behavior = {
        let mut state_guard = state.write().await;
        if let Some(server) = state_guard.servers.get_mut(&port) {
            server.request_count += 1;
            if !server.accepting_connections {
                // Simulate connection refused by returning an error
                return Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Body::from("Server not accepting connections"))
                    .unwrap());
            }
            server.behavior.clone()
        } else {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Server configuration not found"))
                .unwrap());
        }
    };

    // Handle connection refused behavior
    if matches!(behavior, ServerBehavior::ConnectionRefused) {
        // We can't actually refuse the connection at this level since hyper already accepted it
        // Instead, we'll just close it immediately with no response
        // This simulates the load balancer getting a connection error
        return Ok(Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header("Connection", "close")
            .body(Body::from("Connection refused"))
            .unwrap());
    }

    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match (method, path.as_str()) {
        // Health check
        (Method::GET, "/") | (Method::HEAD, "/") => {
            handle_root(behavior).await
        }
        // Version
        (Method::GET, "/api/version") => {
            handle_version(behavior).await
        }
        // List models
        (Method::GET, "/api/tags") => {
            handle_tags(state, port, behavior).await
        }
        // List loaded models
        (Method::GET, "/api/ps") => {
            handle_ps(state, port, behavior).await
        }
        // Chat completion
        (Method::POST, "/api/chat") => {
            handle_chat(req, state, port, behavior).await
        }
        // Generate completion
        (Method::POST, "/api/generate") => {
            handle_generate(req, state, port, behavior).await
        }
        // OpenAI compatible endpoints
        (Method::GET, "/v1/models") => {
            handle_v1_models(state, port, behavior).await
        }
        (Method::POST, "/v1/chat/completions") => {
            handle_v1_chat_completions(req, state, port, behavior).await
        }
        // Catch all
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"error":"not found"}"#))
                .unwrap())
        }
    }
}

async fn handle_root(behavior: ServerBehavior) -> Result<Response<Body>, Infallible> {
    if let ServerBehavior::Hang = behavior {
        // Hang indefinitely
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from("Ollama is running"))
        .unwrap())
}

async fn handle_version(behavior: ServerBehavior) -> Result<Response<Body>, Infallible> {
    if let ServerBehavior::Hang = behavior {
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    }

    let response = OllamaVersionResponse {
        version: "0.13.5".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(json))
        .unwrap())
}

async fn handle_tags(
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    if let ServerBehavior::Hang = behavior {
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    }

    let state = state.read().await;
    let server = match state.servers.get(&port) {
        Some(s) => s,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Server not found"))
                .unwrap());
        }
    };

    let models: Vec<OllamaModelEntry> = server.installed_models.iter().map(|m| {
        OllamaModelEntry {
            name: m.name.clone(),
            model: m.name.clone(),
            modified_at: Utc::now().to_rfc3339(),
            size: m.size,
            digest: m.digest.clone(),
            details: OllamaModelDetails {
                parent_model: String::new(),
                format: "gguf".to_string(),
                family: m.family.clone(),
                families: vec![m.family.clone()],
                parameter_size: m.parameter_size.clone(),
                quantization_level: m.quantization_level.clone(),
            },
        }
    }).collect();

    let response = OllamaTagsResponse { models };
    let json = serde_json::to_string(&response).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(json))
        .unwrap())
}

async fn handle_ps(
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    if let ServerBehavior::Hang = behavior {
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    }

    let state = state.read().await;
    let server = match state.servers.get(&port) {
        Some(s) => s,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Server not found"))
                .unwrap());
        }
    };

    let models: Vec<OllamaPsEntry> = if let Some(ref loaded) = server.loaded_model {
        // Find the model info
        let model_info = server.installed_models.iter()
            .find(|m| &m.name == loaded)
            .cloned()
            .unwrap_or_else(|| ModelInfo {
                name: loaded.clone(),
                size: 4_000_000_000,
                digest: "unknown".to_string(),
                family: "unknown".to_string(),
                parameter_size: "7B".to_string(),
                quantization_level: "Q4_K_M".to_string(),
            });

        vec![OllamaPsEntry {
            name: model_info.name.clone(),
            model: model_info.name.clone(),
            size: model_info.size + 5_000_000_000, // VRAM includes KV cache
            digest: model_info.digest,
            details: OllamaModelDetails {
                parent_model: String::new(),
                format: "gguf".to_string(),
                family: model_info.family.clone(),
                families: vec![model_info.family],
                parameter_size: model_info.parameter_size,
                quantization_level: model_info.quantization_level,
            },
            expires_at: "2318-04-22T19:11:26.424915319+03:00".to_string(), // Infinite keep-alive
            size_vram: model_info.size + 5_000_000_000,
        }]
    } else {
        vec![]
    };

    let response = OllamaPsResponse { models };
    let json = serde_json::to_string(&response).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(json))
        .unwrap())
}

async fn handle_chat(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    // Parse request body
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => b,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Failed to read request body: {}", e)))
                .unwrap());
        }
    };

    let chat_req: OllamaChatRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(Body::from(format!(r#"{{"error":"Invalid request: {}"}}"#, e)))
                .unwrap());
        }
    };

    // Check if model exists
    {
        let state_guard = state.read().await;
        if let Some(server) = state_guard.servers.get(&port) {
            let model_exists = server.installed_models.iter()
                .any(|m| m.name == chat_req.model);
            if !model_exists {
                let response = OllamaErrorResponse {
                    error: format!("model '{}' not found", chat_req.model),
                };
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(Body::from(json))
                    .unwrap());
            }
        }
    }

    // Mark model as loaded
    {
        let mut state_guard = state.write().await;
        if let Some(server) = state_guard.servers.get_mut(&port) {
            server.loaded_model = Some(chat_req.model.clone());
        }
    }

    let stream = chat_req.stream.unwrap_or(true);
    let model_name = chat_req.model.clone();

    // Handle different behaviors
    match behavior {
        ServerBehavior::Hang => {
            loop {
                sleep(Duration::from_secs(3600)).await;
            }
        }

        ServerBehavior::TimeoutAfterHeaders { timeout_ms } => {
            // Send headers, then hang
            sleep(Duration::from_millis(timeout_ms)).await;
            loop {
                sleep(Duration::from_secs(3600)).await;
            }
        }

        ServerBehavior::HttpError { status_code, message } => {
            let response = OllamaErrorResponse { error: message };
            let json = serde_json::to_string(&response).unwrap();
            return Ok(Response::builder()
                .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
                .header("Content-Type", "application/json; charset=utf-8")
                .body(Body::from(json))
                .unwrap());
        }

        ServerBehavior::ModelNotFound { model_name: err_model } => {
            let response = OllamaErrorResponse {
                error: format!("model '{}' not found", err_model),
            };
            let json = serde_json::to_string(&response).unwrap();
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(Body::from(json))
                .unwrap());
        }

        ServerBehavior::Custom { response_body, status_code, content_type, delay_ms } => {
            sleep(Duration::from_millis(delay_ms)).await;
            return Ok(Response::builder()
                .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK))
                .header("Content-Type", content_type)
                .body(Body::from(response_body))
                .unwrap());
        }

        ServerBehavior::AbruptClose { bytes_before_close } => {
            // Send partial data then close
            let partial = "a".repeat(bytes_before_close);
            // Use a stream that will error after sending partial data
            let error_stream = stream::once(async move {
                Ok::<_, std::io::Error>(bytes::Bytes::from(partial))
            }).chain(stream::once(async {
                Err(std::io::Error::new(std::io::ErrorKind::ConnectionReset, "Connection reset"))
            }));

            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/x-ndjson")
                .header("Transfer-Encoding", "chunked")
                .body(Body::wrap_stream(error_stream))
                .unwrap());
        }

        ServerBehavior::FailMidStream { tokens_before_fail, tokens_per_sec } => {
            if stream {
                // Streaming with failure
                let chunks = generate_streaming_chunks_with_failure(
                    &model_name,
                    tokens_before_fail,
                    tokens_per_sec,
                );
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/x-ndjson")
                    .header("Transfer-Encoding", "chunked")
                    .body(Body::wrap_stream(chunks))
                    .unwrap());
            } else {
                // Non-streaming, fail immediately
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal server error"))
                    .unwrap());
            }
        }

        ServerBehavior::Slow { tokens_per_sec, num_tokens } => {
            if stream {
                let chunks = generate_streaming_chunks(&model_name, num_tokens, tokens_per_sec, 0);
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/x-ndjson")
                    .header("Transfer-Encoding", "chunked")
                    .body(Body::wrap_stream(chunks))
                    .unwrap());
            } else {
                let response = generate_non_streaming_response(&model_name, num_tokens);
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(Body::from(json))
                    .unwrap());
            }
        }

        ServerBehavior::Normal { tokens_per_sec, prompt_eval_tokens_per_sec, num_tokens, load_delay_ms } => {
            // Calculate prompt eval with KV cache consideration
            let prompt_eval_result = {
                let state_guard = state.read().await;
                if let Some(server) = state_guard.servers.get(&port) {
                    calculate_prompt_eval(
                        &chat_req.messages,
                        &server.kv_cache_tokens,
                        prompt_eval_tokens_per_sec,
                    )
                } else {
                    // Fallback: no cache
                    calculate_prompt_eval(&chat_req.messages, &[], prompt_eval_tokens_per_sec)
                }
            };

            // Update KV cache with prompt tokens + response tokens (pre-compute for simulation)
            {
                let mut state_guard = state.write().await;
                if let Some(server) = state_guard.servers.get_mut(&port) {
                    // Cache = prompt tokens + simulated response tokens
                    let mut new_cache = prompt_eval_result.prompt_tokens.clone();
                    // Add assistant role marker and response tokens
                    new_cache.push("<|assistant|>".to_string());
                    let response_words = vec![
                        "Hello", "!", " I", "'m", " a", " helpful", " AI", " assistant", ".",
                        " How", " can", " I", " help", " you", " today", "?",
                    ];
                    for i in 0..num_tokens {
                        new_cache.push(response_words[i % response_words.len()].to_string());
                    }
                    server.kv_cache_tokens = new_cache;
                }
            }

            // Total delay = load delay (if cold) + prompt eval delay
            let total_initial_delay_ms = load_delay_ms + prompt_eval_result.prompt_eval_delay_ms;

            if stream {
                let chunks = generate_streaming_chunks_with_stats(
                    &model_name,
                    num_tokens,
                    tokens_per_sec,
                    total_initial_delay_ms,
                    prompt_eval_result,
                );
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/x-ndjson")
                    .header("Transfer-Encoding", "chunked")
                    .body(Body::wrap_stream(chunks))
                    .unwrap());
            } else {
                // Simulate delays
                if total_initial_delay_ms > 0 {
                    sleep(Duration::from_millis(total_initial_delay_ms)).await;
                }
                let response = generate_non_streaming_response(&model_name, num_tokens);
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(Body::from(json))
                    .unwrap());
            }
        }

        ServerBehavior::ConnectionRefused => {
            // ConnectionRefused is handled earlier in the function, but add fallback
            return Ok(Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .header("Connection", "close")
                .body(Body::from("Connection refused"))
                .unwrap());
        }
    }
}

async fn handle_generate(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    // Similar to handle_chat but with different response format
    // For simplicity, we delegate most logic to handle_chat

    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => b,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Failed to read request body: {}", e)))
                .unwrap());
        }
    };

    #[derive(serde::Deserialize)]
    struct GenerateRequest {
        model: String,
        #[serde(default)]
        stream: Option<bool>,
    }

    let gen_req: GenerateRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(Body::from(format!(r#"{{"error":"Invalid request: {}"}}"#, e)))
                .unwrap());
        }
    };

    // Check if model exists
    {
        let state_guard = state.read().await;
        if let Some(server) = state_guard.servers.get(&port) {
            let model_exists = server.installed_models.iter()
                .any(|m| m.name == gen_req.model);
            if !model_exists {
                let response = OllamaErrorResponse {
                    error: format!("model '{}' not found", gen_req.model),
                };
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(Body::from(json))
                    .unwrap());
            }
        }
    }

    // Mark model as loaded
    {
        let mut state_guard = state.write().await;
        if let Some(server) = state_guard.servers.get_mut(&port) {
            server.loaded_model = Some(gen_req.model.clone());
        }
    }

    let stream = gen_req.stream.unwrap_or(true);
    let model_name = gen_req.model;

    // Use similar logic as chat (simplified - no KV cache for /api/generate)
    match behavior {
        ServerBehavior::Normal { tokens_per_sec, num_tokens, load_delay_ms, .. } => {
            if stream {
                let chunks = generate_streaming_chunks(&model_name, num_tokens, tokens_per_sec, load_delay_ms);
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/x-ndjson")
                    .header("Transfer-Encoding", "chunked")
                    .body(Body::wrap_stream(chunks))
                    .unwrap());
            } else {
                if load_delay_ms > 0 {
                    sleep(Duration::from_millis(load_delay_ms)).await;
                }
                let response = generate_non_streaming_response(&model_name, num_tokens);
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(Body::from(json))
                    .unwrap());
            }
        }
        // Handle other behaviors similarly
        _ => {
            // Default behavior
            let response = generate_non_streaming_response(&model_name, 20);
            let json = serde_json::to_string(&response).unwrap();
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(Body::from(json))
                .unwrap());
        }
    }
}

async fn handle_v1_models(
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    if let ServerBehavior::Hang = behavior {
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    }

    let state = state.read().await;
    let server = match state.servers.get(&port) {
        Some(s) => s,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Server not found"))
                .unwrap());
        }
    };

    #[derive(serde::Serialize)]
    struct V1ModelsResponse {
        object: String,
        data: Vec<V1ModelEntry>,
    }

    #[derive(serde::Serialize)]
    struct V1ModelEntry {
        id: String,
        object: String,
        created: i64,
        owned_by: String,
    }

    let data: Vec<V1ModelEntry> = server.installed_models.iter().map(|m| {
        V1ModelEntry {
            id: m.name.clone(),
            object: "model".to_string(),
            created: Utc::now().timestamp(),
            owned_by: "library".to_string(),
        }
    }).collect();

    let response = V1ModelsResponse {
        object: "list".to_string(),
        data,
    };
    let json = serde_json::to_string(&response).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn handle_v1_chat_completions(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
    port: u16,
    behavior: ServerBehavior,
) -> Result<Response<Body>, Infallible> {
    // Parse request
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => b,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Failed to read request body: {}", e)))
                .unwrap());
        }
    };

    #[derive(serde::Deserialize)]
    struct V1ChatRequest {
        model: String,
        #[serde(default)]
        stream: Option<bool>,
        #[serde(default)]
        max_tokens: Option<usize>,
    }

    let chat_req: V1ChatRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(r#"{{"error":"Invalid request: {}"}}"#, e)))
                .unwrap());
        }
    };

    // Mark model as loaded
    {
        let mut state_guard = state.write().await;
        if let Some(server) = state_guard.servers.get_mut(&port) {
            server.loaded_model = Some(chat_req.model.clone());
        }
    }

    let stream = chat_req.stream.unwrap_or(false);
    let num_tokens = chat_req.max_tokens.unwrap_or(20);
    let model_name = chat_req.model;

    match behavior {
        ServerBehavior::Normal { tokens_per_sec, load_delay_ms, .. } => {
            if stream {
                let chunks = generate_v1_streaming_chunks(&model_name, num_tokens, tokens_per_sec, load_delay_ms);
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/event-stream")
                    .header("Transfer-Encoding", "chunked")
                    .body(Body::wrap_stream(chunks))
                    .unwrap());
            } else {
                if load_delay_ms > 0 {
                    sleep(Duration::from_millis(load_delay_ms)).await;
                }
                let response = generate_v1_non_streaming_response(&model_name, num_tokens);
                let json = serde_json::to_string(&response).unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Body::from(json))
                    .unwrap());
            }
        }
        _ => {
            let response = generate_v1_non_streaming_response(&model_name, num_tokens);
            let json = serde_json::to_string(&response).unwrap();
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json))
                .unwrap());
        }
    }
}

/// Generate streaming response chunks for /api/chat (legacy, for Slow behavior)
fn generate_streaming_chunks(
    model: &str,
    num_tokens: usize,
    tokens_per_sec: f64,
    load_delay_ms: u64,
) -> impl futures_util::Stream<Item = Result<bytes::Bytes, std::io::Error>> {
    // Delegate to new function with dummy prompt eval
    let prompt_eval = PromptEvalResult {
        prompt_tokens: vec![],
        cached_token_count: 0,
        new_token_count: 10,
        prompt_eval_delay_ms: 0,
    };
    generate_streaming_chunks_with_stats(model, num_tokens, tokens_per_sec, load_delay_ms, prompt_eval)
}

/// Generate streaming response chunks with accurate prompt eval stats
fn generate_streaming_chunks_with_stats(
    model: &str,
    num_tokens: usize,
    tokens_per_sec: f64,
    initial_delay_ms: u64,
    prompt_eval: PromptEvalResult,
) -> impl futures_util::Stream<Item = Result<bytes::Bytes, std::io::Error>> {
    let model = model.to_string();
    let interval_ms = (1000.0 / tokens_per_sec) as u64;

    // Sample response words
    let words = vec![
        "Hello", "!", " I", "'m", " a", " helpful", " AI", " assistant", ".",
        " How", " can", " I", " help", " you", " today", "?",
    ];

    // Capture prompt eval stats for final chunk
    let prompt_eval_count = prompt_eval.prompt_tokens.len() as u32;
    let prompt_eval_duration_ns = prompt_eval.prompt_eval_delay_ms * 1_000_000;
    let cached_count = prompt_eval.cached_token_count;

    stream::unfold(
        (0usize, initial_delay_ms, model, words, false, prompt_eval_count, prompt_eval_duration_ns, cached_count),
        move |(i, remaining_delay, model, words, done_sent, pe_count, pe_duration, cached)| async move {
            // If we already sent the final chunk, end the stream
            if done_sent {
                return None;
            }

            // Simulate initial delay (load + prompt eval) on first chunk
            if remaining_delay > 0 {
                sleep(Duration::from_millis(remaining_delay)).await;
            }

            if i >= num_tokens {
                // Final chunk with accurate stats
                let eval_duration_ns = (num_tokens as u64) * (interval_ms * 1_000_000);
                let total_duration_ns = (initial_delay_ms * 1_000_000) + eval_duration_ns;

                let response = OllamaChatResponse {
                    model: model.clone(),
                    created_at: Utc::now().to_rfc3339(),
                    message: OllamaChatMessage {
                        role: "assistant".to_string(),
                        content: String::new(),
                    },
                    done: true,
                    done_reason: Some("stop".to_string()),
                    total_duration: Some(total_duration_ns),
                    load_duration: Some(0), // Load time included in prompt_eval for simplicity
                    prompt_eval_count: Some(pe_count),
                    prompt_eval_duration: Some(pe_duration),
                    eval_count: Some(num_tokens as u32),
                    eval_duration: Some(eval_duration_ns),
                };

                // Log cache hit info if any tokens were cached
                if cached > 0 {
                    // In real Ollama this isn't logged, but useful for debugging tests
                }

                let json = serde_json::to_string(&response).unwrap();
                let chunk = format!("{}\n", json);
                return Some((Ok(bytes::Bytes::from(chunk)), (i + 1, 0, model, words, true, pe_count, pe_duration, cached)));
            }

            // Regular content chunk
            sleep(Duration::from_millis(interval_ms)).await;

            let content = words.get(i % words.len()).unwrap_or(&" ").to_string();
            let response = OllamaChatResponse {
                model: model.clone(),
                created_at: Utc::now().to_rfc3339(),
                message: OllamaChatMessage {
                    role: "assistant".to_string(),
                    content,
                },
                done: false,
                done_reason: None,
                total_duration: None,
                load_duration: None,
                prompt_eval_count: None,
                prompt_eval_duration: None,
                eval_count: None,
                eval_duration: None,
            };
            let json = serde_json::to_string(&response).unwrap();
            let chunk = format!("{}\n", json);
            Some((Ok(bytes::Bytes::from(chunk)), (i + 1, 0, model, words, false, pe_count, pe_duration, cached)))
        },
    )
}

/// Generate streaming chunks that fail mid-stream
fn generate_streaming_chunks_with_failure(
    model: &str,
    tokens_before_fail: usize,
    tokens_per_sec: f64,
) -> impl futures_util::Stream<Item = Result<bytes::Bytes, std::io::Error>> {
    let model = model.to_string();
    let interval_ms = (1000.0 / tokens_per_sec) as u64;

    let words = vec!["Hello", "!", " I", "'m", " streaming", "..."];

    stream::unfold(
        (0usize, model, words, false),
        move |(i, model, words, error_sent)| async move {
            // If we already sent the error, end the stream
            if error_sent {
                return None;
            }

            if i >= tokens_before_fail {
                // Return an error to simulate mid-stream failure
                return Some((
                    Err(std::io::Error::new(std::io::ErrorKind::ConnectionReset, "Connection reset by peer")),
                    (i + 1, model, words, true),
                ));
            }

            sleep(Duration::from_millis(interval_ms)).await;

            let content = words.get(i % words.len()).unwrap_or(&" ").to_string();
            let response = OllamaChatResponse {
                model: model.clone(),
                created_at: Utc::now().to_rfc3339(),
                message: OllamaChatMessage {
                    role: "assistant".to_string(),
                    content,
                },
                done: false,
                done_reason: None,
                total_duration: None,
                load_duration: None,
                prompt_eval_count: None,
                prompt_eval_duration: None,
                eval_count: None,
                eval_duration: None,
            };
            let json = serde_json::to_string(&response).unwrap();
            let chunk = format!("{}\n", json);
            Some((Ok(bytes::Bytes::from(chunk)), (i + 1, model, words, false)))
        },
    )
}

/// Generate non-streaming response for /api/chat
fn generate_non_streaming_response(model: &str, num_tokens: usize) -> OllamaChatResponse {
    let content = "Hello! I'm a helpful AI assistant. How can I help you today?";

    OllamaChatResponse {
        model: model.to_string(),
        created_at: Utc::now().to_rfc3339(),
        message: OllamaChatMessage {
            role: "assistant".to_string(),
            content: content.to_string(),
        },
        done: true,
        done_reason: Some("stop".to_string()),
        total_duration: Some(300_000_000),
        load_duration: Some(50_000_000),
        prompt_eval_count: Some(10),
        prompt_eval_duration: Some(20_000_000),
        eval_count: Some(num_tokens as u32),
        eval_duration: Some(200_000_000),
    }
}

/// Generate OpenAI-compatible streaming chunks
fn generate_v1_streaming_chunks(
    model: &str,
    num_tokens: usize,
    tokens_per_sec: f64,
    load_delay_ms: u64,
) -> impl futures_util::Stream<Item = Result<bytes::Bytes, std::io::Error>> {
    let model = model.to_string();
    let interval_ms = (1000.0 / tokens_per_sec) as u64;

    let words = vec!["Hello", "!", " I", "'m", " a", " helpful", " assistant", "."];

    stream::unfold(
        (0usize, load_delay_ms, model, words, false),
        move |(i, remaining_load_delay, model, words, done_sent)| async move {
            // If we already sent the final chunk, end the stream
            if done_sent {
                return None;
            }

            if remaining_load_delay > 0 {
                sleep(Duration::from_millis(remaining_load_delay)).await;
            }

            if i >= num_tokens {
                // Send finish chunk
                let chunk = format!(
                    r#"data: {{"id":"chatcmpl-{}","object":"chat.completion.chunk","created":{},"model":"{}","system_fingerprint":"fp_ollama","choices":[{{"index":0,"delta":{{"role":"assistant","content":""}},"finish_reason":"stop"}}]}}"#,
                    uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>(),
                    Utc::now().timestamp(),
                    model
                );
                return Some((
                    Ok(bytes::Bytes::from(format!("{}\n\ndata: [DONE]\n\n", chunk))),
                    (i + 1, 0, model, words, true),
                ));
            }

            sleep(Duration::from_millis(interval_ms)).await;

            let content = words.get(i % words.len()).unwrap_or(&" ");
            let chunk = format!(
                r#"data: {{"id":"chatcmpl-{}","object":"chat.completion.chunk","created":{},"model":"{}","system_fingerprint":"fp_ollama","choices":[{{"index":0,"delta":{{"role":"assistant","content":"{}"}},"finish_reason":null}}]}}"#,
                uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>(),
                Utc::now().timestamp(),
                model,
                content
            );
            Some((Ok(bytes::Bytes::from(format!("{}\n\n", chunk))), (i + 1, 0, model, words, false)))
        },
    )
}

/// Generate OpenAI-compatible non-streaming response
fn generate_v1_non_streaming_response(model: &str, num_tokens: usize) -> serde_json::Value {
    serde_json::json!({
        "id": format!("chatcmpl-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>()),
        "object": "chat.completion",
        "created": Utc::now().timestamp(),
        "model": model,
        "system_fingerprint": "fp_ollama",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! I'm a helpful AI assistant."
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": num_tokens,
            "total_tokens": 10 + num_tokens
        }
    })
}
