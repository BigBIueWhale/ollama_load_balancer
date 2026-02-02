//! Control API for the Ollama simulator
//!
//! This module provides a REST API for programmatically controlling
//! the behavior of simulated Ollama servers during testing.
//!
//! # Endpoints
//!
//! - `GET /status` - Get status of all servers
//! - `POST /behavior` - Set behavior for a server
//! - `POST /models` - Set installed models for a server
//! - `POST /reset` - Reset all servers to default state
//! - `GET /request-count/{port}` - Get request count for a server
//! - `POST /cancel/{port}` - Cancel active requests on a server

use std::sync::Arc;
use tokio::sync::RwLock;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;

use crate::simulator::SimulatorState;
use crate::types::{
    SetBehaviorRequest, SetModelsRequest, ResetRequest, StatusResponse,
    ServerStatusResponse, PortSelector, ServerBehavior, ModelInfo,
};

pub async fn run_control_server(
    port: u16,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(move |_| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_control_request(req, state.clone())
            }))
        }
    });

    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_svc);

    server.await.map_err(|e| {
        format!("Control server error: {}", e).into()
    })
}

async fn handle_control_request(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let result = match (method, path.as_str()) {
        (Method::GET, "/status") => handle_status(state).await,
        (Method::POST, "/behavior") => handle_set_behavior(req, state).await,
        (Method::POST, "/models") => handle_set_models(req, state).await,
        (Method::POST, "/reset") => handle_reset(req, state).await,
        (Method::POST, "/loaded-model") => handle_set_loaded_model(req, state).await,
        (Method::GET, path) if path.starts_with("/request-count/") => {
            let port_str = &path["/request-count/".len()..];
            handle_request_count(port_str, state).await
        }
        (Method::POST, path) if path.starts_with("/cancel/") => {
            let port_str = &path["/cancel/".len()..];
            handle_cancel(port_str, state).await
        }
        (Method::GET, "/health") => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"status":"ok"}"#))
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap())
        }
    };

    match result {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("Control API error: {}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(format!(r#"{{"error":"{}"}}"#, e)))
                .unwrap())
        }
    }
}

async fn handle_status(
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let state = state.read().await;

    let servers: Vec<ServerStatusResponse> = state.servers.iter().map(|(port, server)| {
        ServerStatusResponse {
            port: *port,
            behavior: format!("{:?}", server.behavior).split('{').next().unwrap_or("Unknown").trim().to_string(),
            accepting_connections: server.accepting_connections,
            installed_models: server.installed_models.iter().map(|m| m.name.clone()).collect(),
            loaded_model: server.loaded_model.clone(),
            request_count: server.request_count,
            active_request_count: server.active_requests.len(),
        }
    }).collect();

    let response = StatusResponse { servers };
    let json = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn handle_set_behavior(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let request: SetBehaviorRequest = serde_json::from_slice(&body)?;

    let mut state = state.write().await;

    match request.port {
        PortSelector::Single(port) => {
            if let Some(server) = state.servers.get_mut(&port) {
                server.behavior = request.behavior;
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(format!("Server on port {} not found", port)))
                    .unwrap());
            }
        }
        PortSelector::All(_) => {
            for server in state.servers.values_mut() {
                server.behavior = request.behavior.clone();
            }
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap())
}

async fn handle_set_models(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let request: SetModelsRequest = serde_json::from_slice(&body)?;

    let mut state = state.write().await;

    match request.port {
        PortSelector::Single(port) => {
            if let Some(server) = state.servers.get_mut(&port) {
                server.installed_models = request.models;
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(format!("Server on port {} not found", port)))
                    .unwrap());
            }
        }
        PortSelector::All(_) => {
            for server in state.servers.values_mut() {
                server.installed_models = request.models.clone();
            }
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap())
}

async fn handle_set_loaded_model(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let body = hyper::body::to_bytes(req.into_body()).await?;

    #[derive(serde::Deserialize)]
    struct SetLoadedModelRequest {
        port: u16,
        model: Option<String>,
    }

    let request: SetLoadedModelRequest = serde_json::from_slice(&body)?;

    let mut state = state.write().await;

    if let Some(server) = state.servers.get_mut(&request.port) {
        server.loaded_model = request.model;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"status":"ok"}"#))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("Server on port {} not found", request.port)))
            .unwrap())
    }
}

async fn handle_reset(
    req: Request<Body>,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let request: ResetRequest = if body.is_empty() {
        ResetRequest { clear_counters: false }
    } else {
        serde_json::from_slice(&body)?
    };

    let mut state = state.write().await;

    for server in state.servers.values_mut() {
        server.behavior = ServerBehavior::default();
        server.accepting_connections = true;
        server.loaded_model = None;
        server.installed_models = vec![ModelInfo::default_test_model()];
        server.active_requests.clear();
        if request.clear_counters {
            server.request_count = 0;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap())
}

async fn handle_request_count(
    port_str: &str,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let port: u16 = port_str.parse()?;
    let state = state.read().await;

    if let Some(server) = state.servers.get(&port) {
        let json = format!(r#"{{"port":{},"request_count":{}}}"#, port, server.request_count);
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("Server on port {} not found", port)))
            .unwrap())
    }
}

async fn handle_cancel(
    port_str: &str,
    state: Arc<RwLock<SimulatorState>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let port: u16 = port_str.parse()?;
    let mut state = state.write().await;

    if let Some(server) = state.servers.get_mut(&port) {
        // Mark all active requests as cancelled
        for request in server.active_requests.values_mut() {
            request.cancelled = true;
        }
        let count = server.active_requests.len();
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(format!(r#"{{"cancelled":{}}}"#, count)))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("Server on port {} not found", port)))
            .unwrap())
    }
}
