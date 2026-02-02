# Ollama Simulator

A Rust-based Ollama server simulator for automated testing of `ollama_load_balancer`.

## Overview

This package provides:

1. **Ollama Simulator** (`ollama_simulator`) - Mock Ollama servers with programmatic behavior control
2. **Test Runner** (`load_balancer_test`) - Automated test suite for the load balancer

## Prerequisites

Build the load balancer first:

```bash
cd /path/to/ollama_load_balancer
cargo build --release
```

## Running the Tests

```bash
cd test/ollama_simulator
cargo run --release --bin load_balancer_test
```

Expected output:

```
======================================
  Ollama Load Balancer Test Suite
======================================

[+] Basic single server request
[+] Load balancing across multiple servers
[+] Server unreachable - marked as unreliable
[+] Server fails mid-stream
[+] Server recovery from unreliable to reliable
[+] All servers busy - queue or reject
[+] Second chance mechanism for unreliable servers
[+] No available servers response
[+] GET requests (non-POST)
[+] Streaming response timing

Total: 10 passed, 0 failed
```

## Running the Simulator Standalone

For manual testing or development:

```bash
cargo run --bin ollama_simulator -- --control-port 11500 --server-ports 11501,11502,11503
```

This starts:
- Control API on port 11500
- Three simulated Ollama servers on ports 11501, 11502, 11503

## Control API

The simulator exposes a control API for programmatically changing server behavior:

### Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/status` | GET | Get status of all simulated servers |
| `/behavior` | POST | Set behavior for a server |
| `/models` | POST | Set installed models |
| `/reset` | POST | Reset all servers to default state |
| `/health` | GET | Health check |

### Setting Server Behavior

```bash
# Make server on port 11501 hang (simulate timeout)
curl -X POST http://127.0.0.1:11500/behavior \
  -H "Content-Type: application/json" \
  -d '{"port": 11501, "behavior": {"type": "Hang"}}'

# Make server fail mid-stream after 5 tokens
curl -X POST http://127.0.0.1:11500/behavior \
  -H "Content-Type: application/json" \
  -d '{"port": 11501, "behavior": {"type": "FailMidStream", "tokens_before_fail": 5, "tokens_per_sec": 50.0}}'

# Reset to normal behavior
curl -X POST http://127.0.0.1:11500/behavior \
  -H "Content-Type: application/json" \
  -d '{"port": 11501, "behavior": {"type": "Normal", "tokens_per_sec": 70.0, "num_tokens": 20, "load_delay_ms": 0}}'

# Apply behavior to all servers
curl -X POST http://127.0.0.1:11500/behavior \
  -H "Content-Type: application/json" \
  -d '{"port": {"all": true}, "behavior": {"type": "Normal"}}'
```

### Available Behaviors

| Behavior | Description |
|----------|-------------|
| `Normal` | Normal operation with configurable token rate |
| `Hang` | Accept connection but never respond |
| `FailMidStream` | Send partial response then fail |
| `TimeoutAfterHeaders` | Send headers then hang |
| `Slow` | Very slow token generation |
| `HttpError` | Return specific HTTP error code |
| `ModelNotFound` | Return 404 for model |
| `ConnectionRefused` | Simulate connection refused |
| `AbruptClose` | Close connection after partial data |
| `Custom` | Custom response body/status |

## Test Scenarios

The test suite validates:

1. **Basic single server request** - Simple request/response
2. **Load balancing** - Concurrent requests distributed across servers
3. **Server unreachable** - Timeout handling and unreliable marking
4. **Mid-stream failure** - Recovery from streaming errors
5. **Server recovery** - Transition from unreliable back to reliable
6. **All servers busy** - 503 response when no servers available
7. **Second chance mechanism** - Unreliable servers get retry opportunities
8. **No available servers** - Graceful handling when all servers fail
9. **GET requests** - Non-POST endpoints (`/api/tags`, `/api/version`, `/`)
10. **Streaming response** - NDJSON streaming with proper termination

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Test Runner                              │
│  (starts/stops load balancer, sends requests, validates)    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Load Balancer                              │
│              (http://127.0.0.1:11434)                       │
└─────────────────────────────────────────────────────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ Simulated       │ │ Simulated       │ │ Simulated       │
│ Ollama :11501   │ │ Ollama :11502   │ │ Ollama :11503   │
└─────────────────┘ └─────────────────┘ └─────────────────┘
          │                   │                   │
          └───────────────────┴───────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Control API                               │
│              (http://127.0.0.1:11500)                       │
│         (changes server behavior during tests)              │
└─────────────────────────────────────────────────────────────┘
```

## Simulated Ollama Endpoints

The simulator implements these Ollama API endpoints:

- `GET /` - Health check ("Ollama is running")
- `GET /api/version` - Version info
- `GET /api/tags` - List installed models
- `GET /api/ps` - List loaded models
- `POST /api/chat` - Chat completion (streaming/non-streaming)
- `POST /api/generate` - Text generation
- `GET /v1/models` - OpenAI-compatible models list
- `POST /v1/chat/completions` - OpenAI-compatible chat
