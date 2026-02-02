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
[+] KV cache prefix matching
[+] Embeddings endpoints

Total: 12 passed, 0 failed
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
| `/models` | POST | Set installed models for a server |
| `/loaded-model` | POST | Set which model is "hot" (loaded in VRAM) |
| `/reset` | POST | Reset all servers to default state |
| `/request-count/{port}` | GET | Get request counter for a server |
| `/kv-cache/{port}` | GET | Get KV cache tokens for a server |
| `/kv-cache/clear` | POST | Clear KV cache for server(s) |
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
| `Normal` | Normal operation with configurable generation/prompt eval rates |
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
11. **KV cache prefix matching** - Cached prompts have faster TTFT

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

- `GET /`, `HEAD /` - Health check ("Ollama is running")
- `GET /api/version`, `HEAD /api/version` - Version info
- `GET /api/tags`, `HEAD /api/tags` - List installed models
- `POST /api/show` - Show model information
- `GET /api/ps` - List loaded models
- `POST /api/chat` - Chat completion (streaming/non-streaming) **with KV cache simulation**
- `POST /api/generate` - Text generation (no KV cache simulation)
- `POST /api/embed` - Generate embeddings (new, supports batch input)
- `POST /api/embeddings` - Generate embeddings (deprecated, single input)
- `GET /v1/models` - OpenAI-compatible models list
- `GET /v1/models/:model` - OpenAI-compatible specific model info
- `POST /v1/embeddings` - OpenAI-compatible embeddings
- `POST /v1/chat/completions` - OpenAI-compatible chat
- `POST /v1/messages` - Anthropic-compatible messages API

> **Note:** KV cache simulation only applies to `/api/chat`. The `/api/generate` and embedding endpoints do not track or benefit from cached context.

## KV Cache Simulation

Each simulated server maintains its own KV cache state, mimicking real Ollama behavior where cached prompt prefixes speed up subsequent requests.

### Timing Model

| Phase | Default Rate | Notes |
|-------|-------------|-------|
| Prompt eval (uncached) | 2900 tok/s | RTX 5090 with qwen3-32b |
| Prompt eval (cached) | Instant | Prefix match from KV cache |
| Token generation | 65 tok/s | Output token rate |

### Prefix Matching

When a request arrives, the simulator:
1. Tokenizes the prompt (simple word/punctuation splitting)
2. Compares with cached tokens from previous request
3. Calculates delay only for uncached (new) tokens
4. Updates cache with full context (prompt + response)

### Control API

```bash
# View KV cache state for server on port 11501
curl http://127.0.0.1:11500/kv-cache/11501

# Clear KV cache for a specific server
curl -X POST http://127.0.0.1:11500/kv-cache/clear \
  -H "Content-Type: application/json" \
  -d '{"port": 11501}'

# Clear KV cache for all servers
curl -X POST http://127.0.0.1:11500/kv-cache/clear \
  -H "Content-Type: application/json" \
  -d '{"port": {"all": true}}'
```

### Configuring Timing

```bash
# Set custom prompt eval and generation rates
curl -X POST http://127.0.0.1:11500/behavior \
  -H "Content-Type: application/json" \
  -d '{
    "port": 11501,
    "behavior": {
      "type": "Normal",
      "tokens_per_sec": 65.0,
      "prompt_eval_tokens_per_sec": 2900.0,
      "num_tokens": 20,
      "load_delay_ms": 0
    }
  }'
```

## Per-Server Configuration

Each simulated server maintains independent state, enabling heterogeneous cluster testing:

```bash
# Configure Server A with qwen3-32b only
curl -X POST http://127.0.0.1:11500/models -d '{
  "port": 11501,
  "models": [{"name": "qwen3-32b", "size": 20000000000, "digest": "abc...",
              "family": "qwen3", "parameter_size": "32.8B", "quantization_level": "Q4_K_M"}]
}'

# Configure Server B with gpt-oss:20b only
curl -X POST http://127.0.0.1:11500/models -d '{
  "port": 11502,
  "models": [{"name": "gpt-oss:20b", "size": 12000000000, "digest": "def...",
              "family": "gpt", "parameter_size": "20B", "quantization_level": "Q4_K_M"}]
}'

# Set which model is "hot" (loaded in VRAM) on Server A
curl -X POST http://127.0.0.1:11500/loaded-model -d '{
  "port": 11501,
  "model": "qwen3-32b"
}'
```

This enables testing:
- Model-aware routing (v1.0.4)
- Hot model preference
- Heterogeneous server configurations

## Platform Support

The simulator is cross-platform and works on:
- Linux
- macOS
- Windows 10/11

On Windows, process termination uses `child.kill()` instead of Unix SIGTERM, but functionality is equivalent for testing purposes.

## Future: v1.0.4 Testing Support

The simulator infrastructure is designed to support v1.0.4 features. See the [main README's v1.0.4 section](../../README.md#vision-for-version-104) for the full roadmap including:

- Model-aware routing and capability tiers
- [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) KV cache API simulation (ports 11601-11603)
- Dynamic KV cache type switching (`q8_0`/`q16`)
- Conversation affinity routing

The per-server state model (`SimulatedServerState`) already supports independent configuration of models, loaded state, KV cache, and behavior per server—ready for these extensions.
