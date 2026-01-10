# Ollama API Reference

This document will help us create an Ollama simulator for purposes of accurately and thoroughly testing Ollama Load Balancer **without** opening multiple virtual machines.

**Ollama Version:** `0.13.5`
**Test Host:** `http://172.17.0.1:11434`
**Test Model:** `qwen3:32b` (32.8B parameters, Q4_K_M quantization, qwen3 family)

This document provides exact, versioned documentation of Ollama's HTTP API behavior. All response formats, headers, timing characteristics, and edge cases are documented for creating accurate mock servers.

---

## Table of Contents

1. [Why httpx Instead of the Official Client](#why-httpx-instead-of-the-official-client)
2. [Minimal httpx Examples](#minimal-httpx-examples)
3. [Environment](#environment)
4. [Endpoint Reference](#endpoint-reference)
5. [Streaming Formats](#streaming-formats)
6. [API Options Reference](#api-options-reference)
7. [Thinking/Reasoning Control](#thinkingreasoning-control)
8. [Tool Calling](#tool-calling)
9. [Response Schemas](#response-schemas)
10. [Error Responses](#error-responses)
11. [Timing Characteristics](#timing-characteristics)
12. [HTTP Headers](#http-headers)
13. [Mock Server Implementation Notes](#mock-server-implementation-notes)
14. [Exact Response Bodies](#exact-response-bodies)

---

## Why httpx Instead of the Official Client

The official `ollama` Python client has **reliability issues** that can cause infinite blocking:

1. **No configurable timeout** - The official client doesn't expose timeout parameters, meaning a hung Ollama server will block your Python process forever.

2. **Infinite generation edge case** - Models can sometimes enter infinite generation loops (observed: exactly 239,998 thinking tokens + 2 response tokens). Without timeouts, this grinds the GPU for ~2 hours on gibberish.

3. **Connection hanging** - If Ollama stops sending bytes mid-stream, the official client provides no mechanism to detect or recover.

**Solution:** Use `httpx` directly with explicit timeouts:

```python
import httpx

client = httpx.Client(
    timeout=httpx.Timeout(
        connect=10.0,    # Fail fast if Ollama daemon is down
        read=300.0,      # 5 minute cap on waiting for response
        write=10.0,      # 10 seconds to send request
        pool=None,       # Use default connection pool
    )
)
```

This ensures:
- **Fast failure** on connection issues (10s connect timeout)
- **Bounded wait time** for model loading + inference (read timeout)
- **No infinite hangs** - TimeoutException is raised instead

---

## Minimal httpx Examples

### Non-Streaming Chat (Recommended for Reliability)

```python
import httpx

def chat_complete(
    host: str,
    model: str,
    messages: list[dict],
    options: dict = None,
    think: bool = None,
    timeout_seconds: float = 300.0
) -> dict:
    """
    Non-streaming chat completion with proper timeout handling.

    Args:
        host: Ollama server URL (e.g., "http://127.0.0.1:11434")
        model: Model name (e.g., "qwen3:32b")
        messages: List of {"role": "user"|"assistant"|"system", "content": "..."}
        options: Model options dict (temperature, num_predict, etc.)
        think: Enable/disable thinking for thinking-capable models
        timeout_seconds: Maximum time to wait for response

    Returns:
        Ollama response dict with message, timing stats, done_reason
    """
    client = httpx.Client(
        timeout=httpx.Timeout(
            connect=10.0,
            read=timeout_seconds,
            write=10.0,
        )
    )

    payload = {
        "model": model,
        "messages": messages,
        "stream": False,  # Non-streaming for simplicity
    }

    if options:
        payload["options"] = options

    if think is not None:
        payload["think"] = think

    try:
        response = client.post(f"{host}/api/chat", json=payload)
        response.raise_for_status()
        return response.json()
    finally:
        client.close()


# Example usage with qwen3:32b recommended settings
result = chat_complete(
    host="http://127.0.0.1:11434",
    model="qwen3:32b",
    messages=[{"role": "user", "content": "What is 15 * 7?"}],
    options={
        "num_ctx": 32768,       # Context window
        "num_predict": 28672,   # Max tokens to generate
        "temperature": 0.6,     # Recommended for thinking mode
        "top_p": 0.95,
        "top_k": 20,
        "num_gpu": 65,          # Offload all layers to GPU
    },
    think=True,  # Enable thinking/reasoning
)

# Extract response
content = result["message"]["content"]
thinking = result["message"].get("thinking", "")
done_reason = result.get("done_reason")  # "stop" or "length"

# Extract timing (nanoseconds)
prompt_eval_duration = result.get("prompt_eval_duration")  # Prefill time
eval_duration = result.get("eval_duration")                # Generation time
eval_count = result.get("eval_count")                      # Tokens generated
```

### Streaming Chat with httpx

```python
import httpx
import json

def chat_stream(
    host: str,
    model: str,
    messages: list[dict],
    options: dict = None,
    timeout_seconds: float = 300.0
):
    """
    Streaming chat completion. Yields parsed JSON chunks.

    IMPORTANT: Each line is a complete JSON object (NDJSON format).
    The stream ends when a chunk has "done": true.
    """
    # For streaming, we need longer read timeout per chunk
    client = httpx.Client(
        timeout=httpx.Timeout(
            connect=10.0,
            read=timeout_seconds,  # Timeout between chunks
            write=10.0,
        )
    )

    payload = {
        "model": model,
        "messages": messages,
        "stream": True,  # Enable streaming
    }

    if options:
        payload["options"] = options

    try:
        with client.stream("POST", f"{host}/api/chat", json=payload) as response:
            response.raise_for_status()

            for line in response.iter_lines():
                if line:
                    chunk = json.loads(line)
                    yield chunk

                    if chunk.get("done"):
                        break
    finally:
        client.close()


# Example: Print tokens as they arrive
for chunk in chat_stream(
    host="http://127.0.0.1:11434",
    model="qwen3:32b",
    messages=[{"role": "user", "content": "Hello"}],
    options={"num_predict": 50},
):
    if not chunk.get("done"):
        # Intermediate chunk
        content = chunk["message"].get("content", "")
        thinking = chunk["message"].get("thinking", "")
        print(thinking or content, end="", flush=True)
    else:
        # Final chunk with stats
        print(f"\n\nTokens generated: {chunk.get('eval_count')}")
        print(f"Done reason: {chunk.get('done_reason')}")
```

### OpenAI-Compatible Streaming (SSE Format)

```python
import httpx

def openai_chat_stream(
    host: str,
    model: str,
    messages: list[dict],
    max_tokens: int = 100,
    timeout_seconds: float = 300.0
):
    """
    OpenAI-compatible streaming using Server-Sent Events (SSE).

    IMPORTANT differences from native API:
    - Content-Type: text/event-stream
    - Lines prefixed with "data: "
    - Ends with "data: [DONE]"
    - Thinking field is called "reasoning" (not "thinking")
    """
    client = httpx.Client(
        timeout=httpx.Timeout(connect=10.0, read=timeout_seconds, write=10.0)
    )

    payload = {
        "model": model,
        "messages": messages,
        "stream": True,
        "max_tokens": max_tokens,
    }

    try:
        with client.stream("POST", f"{host}/v1/chat/completions", json=payload) as response:
            response.raise_for_status()

            for line in response.iter_lines():
                if not line:
                    continue

                # SSE format: "data: {json}" or "data: [DONE]"
                if line.startswith("data: "):
                    data = line[6:]  # Remove "data: " prefix

                    if data == "[DONE]":
                        break

                    chunk = json.loads(data)
                    yield chunk
    finally:
        client.close()


# Example usage
for chunk in openai_chat_stream(
    host="http://127.0.0.1:11434",
    model="qwen3:32b",
    messages=[{"role": "user", "content": "Hello"}],
    max_tokens=50,
):
    delta = chunk["choices"][0]["delta"]
    content = delta.get("content", "")
    reasoning = delta.get("reasoning", "")  # Note: "reasoning" not "thinking"
    print(reasoning or content, end="", flush=True)
```

---

## Environment

| Property | Value |
|----------|-------|
| Ollama Version | `0.13.5` |
| Test Model | `qwen3:32b` |
| Model Family | `qwen3` |
| Parameter Count | 32,762,123,264 (32.8B) |
| Quantization | Q4_K_M |
| Context Length (GGUF metadata) | 40,960 tokens |
| Context Length (Runtime observed) | 32,768 tokens |
| VRAM Usage (with 32k context) | 29,148,011,648 bytes (~27.1 GB) |

---

## Endpoint Reference

### Health & Metadata Endpoints

| Endpoint | Method | Content-Type | Description |
|----------|--------|--------------|-------------|
| `/` | GET | `text/plain; charset=utf-8` | Health check - returns `"Ollama is running"` |
| `/` | HEAD | (no body) | Health check headers only |
| `/api/version` | GET | `application/json; charset=utf-8` | Returns `{"version": "0.13.5"}` |
| `/api/tags` | GET | `application/json; charset=utf-8` | List installed models |
| `/api/ps` | GET | `application/json; charset=utf-8` | List currently loaded models |
| `/api/show` | POST | `application/json; charset=utf-8` | Show model details (license, template, tensors) |

### Inference Endpoints (Native Ollama API)

| Endpoint | Method | Streaming Content-Type | Non-Streaming Content-Type |
|----------|--------|------------------------|---------------------------|
| `/api/chat` | POST | `application/x-ndjson` | `application/json; charset=utf-8` |
| `/api/generate` | POST | `application/x-ndjson` | `application/json; charset=utf-8` |
| `/api/embed` | POST | N/A | `application/json; charset=utf-8` |

### OpenAI-Compatible Endpoints

| Endpoint | Method | Streaming Content-Type | Non-Streaming Content-Type |
|----------|--------|------------------------|---------------------------|
| `/v1/models` | GET | N/A | `application/json` |
| `/v1/chat/completions` | POST | `text/event-stream` | `application/json` |

---

## Streaming Formats

### Native Ollama API (`/api/chat`, `/api/generate`)

**Content-Type:** `application/x-ndjson`
**Transfer-Encoding:** `chunked`

Newline-Delimited JSON (NDJSON) - each line is a complete JSON object:

```
{"model":"qwen3:32b","created_at":"2026-01-10T16:33:10.345334612Z","message":{"role":"assistant","content":"","thinking":"Okay"},"done":false}
{"model":"qwen3:32b","created_at":"2026-01-10T16:33:10.361056315Z","message":{"role":"assistant","content":"","thinking":","},"done":false}
{"model":"qwen3:32b","created_at":"2026-01-10T16:33:10.593873744Z","message":{"role":"assistant","content":""},"done":true,"done_reason":"length","total_duration":5977577983,"load_duration":5650132100,"prompt_eval_count":15,"prompt_eval_duration":33848612,"eval_count":20,"eval_duration":279354534}
```

**Format details:**
- Each JSON object on its own line (no `data:` prefix)
- Lines terminated with `\n` (single newline)
- Final object has `"done": true` with timing statistics
- No separate terminator line
- Chunk interval: ~15ms between tokens

### OpenAI-Compatible API (`/v1/chat/completions`)

**Content-Type:** `text/event-stream`
**Transfer-Encoding:** `chunked`

Server-Sent Events (SSE) format:

```
data: {"id":"chatcmpl-774","object":"chat.completion.chunk","created":1768062791,"model":"qwen3:32b","system_fingerprint":"fp_ollama","choices":[{"index":0,"delta":{"role":"assistant","content":"","reasoning":"Okay"},"finish_reason":null}]}

data: {"id":"chatcmpl-774","object":"chat.completion.chunk","created":1768062791,"model":"qwen3:32b","system_fingerprint":"fp_ollama","choices":[{"index":0,"delta":{"role":"assistant","content":""},"finish_reason":"length"}]}

data: [DONE]
```

**Format details:**
- Each line prefixed with `data: ` (note the space)
- Lines terminated with `\n\n` (double newline per SSE spec)
- Stream terminates with literal `data: [DONE]`
- `system_fingerprint` is always `"fp_ollama"`
- **Thinking field is `reasoning`** (not `thinking` like native API)

---

## API Options Reference

All options are passed in the `options` object within the request body.

### Sampling Parameters

| Parameter | Type | Default | Behavior |
|-----------|------|---------|----------|
| `temperature` | float | 0.8 | **0** = deterministic output. **1.5** = more random. |
| `top_p` | float | 0.9 | Nucleus sampling. **0.5** limits to top 50% probability mass. |
| `top_k` | int | 40 | Top-k filtering. **10** limits to top 10 tokens. |
| `min_p` | float | 0.0 | Minimum probability threshold. |
| `repeat_penalty` | float | 1.1 | Penalty for repeated tokens. |
| `presence_penalty` | float | 0.0 | Penalty for token presence. |
| `frequency_penalty` | float | 0.0 | Penalty based on frequency. |
| `seed` | int | random | Fixed seed produces reproducible output. |
| `mirostat` | int | 0 | **2** enables Mirostat 2.0 perplexity-controlled sampling. |
| `mirostat_tau` | float | 5.0 | Target entropy for mirostat. |
| `mirostat_eta` | float | 0.1 | Learning rate for mirostat. |
| `tfs_z` | float | 1.0 | Tail free sampling. |
| `typical_p` | float | 1.0 | Locally typical sampling. |

All sampling parameters are accepted and return status 200.

### Context & Generation Control

| Parameter | Type | Default | Behavior |
|-----------|------|---------|----------|
| `num_ctx` | int | 2048 | Context window size. **512** to **32768** accepted. Affects VRAM usage. |
| `num_predict` | int | 128 | Max tokens to generate. **-1** = unlimited. |
| `num_gpu` | int | varies | GPU layers to offload. **99** offloads all possible layers. |
| `num_batch` | int | 512 | Prompt processing batch size. |
| `num_thread` | int | auto | CPU thread count for generation. |
| `stop` | array | [] | Stop sequences. Generation stops at matching sequence, returns `done_reason: "stop"`. |

Stop sequences work correctly - generation stops **before** the matching sequence (e.g., stops at "4, " when counting before reaching "5"). The stop sequence itself is NOT included in output.

### Model-Specific Recommended Settings

Based on analysis of production configurations:

#### qwen3:32b (Thinking Mode - Alibaba Recommended)
```python
{
    "num_ctx": 32768,        # Max native context, ~29GB VRAM
    "num_predict": 28672,    # Avoid infinite generation
    "temperature": 0.6,
    "top_p": 0.95,
    "top_k": 20,
    "min_p": 0.0,
    "repeat_penalty": 1.0,
    "num_gpu": 65,           # All layers
}
```

#### qwen3:32b (No-Think Mode - Alibaba Recommended)
```python
{
    "num_ctx": 32768,
    "num_predict": 28672,
    "temperature": 0.7,      # Slightly higher than think mode
    "top_p": 0.8,            # Tighter than think mode
    "top_k": 20,
    "min_p": 0.0,
    "repeat_penalty": 1.0,
    "num_gpu": 65,
}
```

#### qwen3:30b-a3b (MoE Variant)
```python
{
    "num_ctx": 140000,       # Massive context window
    "num_predict": 81920,
    "temperature": 0.6,      # (thinking) or 0.7 (instruct)
    "top_p": 0.95,           # (thinking) or 0.8 (instruct)
    "top_k": 20,
    "min_p": 0.0,
    "repeat_penalty": 1.0,
    "num_gpu": 49,           # All layers for 30B MoE
}
```

#### gpt-oss:20b (OpenAI GPT-OSS)
```python
{
    "num_ctx": 131072,       # 128k context
    "num_predict": -1,       # Unlimited (model self-terminates)
    "temperature": 1.0,
    "top_p": 1.0,
    "top_k": 0,              # Disabled
}
# IMPORTANT: Requires think="high" (string, not boolean)
```

#### gemma3:27b (Google)
```python
{
    "num_ctx": 32768,        # Pretrained on 32k context
    "num_predict": 8192,     # Max trained output length
}
```

---

## Thinking/Reasoning Control

The `think` parameter controls reasoning for thinking-capable models. This is a **top-level request field**, not inside `options`.

### Behavior by Model

| Model | `think=true` | `think=false` | `think="low"/"medium"/"high"` |
|-------|--------------|---------------|-------------------------------|
| qwen3:32b | Works - returns `thinking` field | Works - direct answer, no thinking | **400 Error** |
| qwen3:30b-a3b-thinking | Works | Works | **400 Error** |
| gpt-oss:20b | Ignored | Ignored | **Works** - required for this model |
| seed-oss models | Works | Works | **400 Error** |
| Non-thinking models | Ignored | Ignored | Ignored |

### think=true Response

```json
{
  "model": "qwen3:32b",
  "message": {
    "role": "assistant",
    "content": "",
    "thinking": "Okay, so I need to figure out what 15 multiplied by 7 is. Hmm, let's see. I remember that multiplication is just repeated addition, right? So 15 times 7 would be adding 15 seven times..."
  },
  "done": true,
  "done_reason": "length",
  "eval_count": 100
}
```

**Timing with think=true:**
- Prompt tokens/sec: 519.8
- Generation tokens/sec: 67.1
- Total duration: 4125ms for 100 tokens

### think=false Response

```json
{
  "model": "qwen3:32b",
  "message": {
    "role": "assistant",
    "content": "15 * 7 = 105."
  },
  "done": true,
  "done_reason": "stop",
  "eval_count": 12
}
```

**Timing with think=false:**
- Prompt tokens/sec: 771.5
- Generation tokens/sec: 70.8
- Total duration: 265ms for 12 tokens

### Error Response for Unsupported Think Levels

```
HTTP/1.1 400 Bad Request
Content-Type: application/json; charset=utf-8

{"error":"think value \"low\" is not supported for this model"}
```

**Models that require string think levels:** Only `gpt-oss:20b` (must use `"high"`).

---

## Tool Calling

When a `tools` array is provided, the model can return tool calls.

### Request Format

```json
{
  "model": "qwen3:32b",
  "messages": [{"role": "user", "content": "What's the weather in Paris?"}],
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get weather for a location",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {"type": "string", "description": "City name"}
        },
        "required": ["location"]
      }
    }
  }],
  "options": {"num_predict": 100}
}
```

### Response Format

```json
{
  "model": "qwen3:32b",
  "message": {
    "role": "assistant",
    "content": "",
    "thinking": "Okay, the user is asking for the weather in Paris. Let me check the available tools. There's a function called get_weather that requires a location parameter...",
    "tool_calls": [{
      "id": "call_d1imwj4g",
      "function": {
        "index": 0,
        "name": "get_weather",
        "arguments": {"location": "Paris"}
      }
    }]
  },
  "done": true,
  "done_reason": "length"
}
```

**tool_call structure:**
- `id`: Unique identifier string (format: `call_` + 8 alphanumeric chars)
- `function.index`: Integer index into the tools array (0-based)
- `function.name`: String matching the function name
- `function.arguments`: **Parsed JSON object** (not a string!)

---

## Response Schemas

### GET / (Root Health Check)

**Response:** Plain text (not JSON)
```
Ollama is running
```
**Content-Length:** 17 bytes
**Latency:** ~1.1ms

### GET /api/version

```json
{"version": "0.13.5"}
```
**Content-Length:** 20 bytes
**Latency:** ~0.4ms

### GET /api/tags (List Models)

```json
{
  "models": [{
    "name": "qwen3:32b",
    "model": "qwen3:32b",
    "modified_at": "2025-08-26T21:46:36.388995313+03:00",
    "size": 20201253829,
    "digest": "030ee887880fc378860c2dd35101da424377520441ae4bfe7be6deff8ade7840",
    "details": {
      "parent_model": "",
      "format": "gguf",
      "family": "qwen3",
      "families": ["qwen3"],
      "parameter_size": "32.8B",
      "quantization_level": "Q4_K_M"
    }
  }]
}
```

### GET /api/ps (Loaded Models)

```json
{
  "models": [{
    "name": "qwen3:32b",
    "model": "qwen3:32b",
    "size": 29148011648,
    "digest": "030ee887880fc378860c2dd35101da424377520441ae4bfe7be6deff8ade7840",
    "details": {...},
    "expires_at": "2318-04-22T19:11:26.424915319+03:00",
    "size_vram": 29148011648,
    "context_length": 32768
  }]
}
```

**Note:** `expires_at` in year 2318 indicates `OLLAMA_KEEP_ALIVE=-1` (infinite keep-alive).

**Size difference:**
- `/api/tags` `size`: 20,201,253,829 bytes (disk size)
- `/api/ps` `size`: 29,148,011,648 bytes (VRAM including KV cache)

### GET /v1/models (OpenAI-Compatible)

```json
{
  "object": "list",
  "data": [{
    "id": "qwen3:32b",
    "object": "model",
    "created": 1756233996,
    "owned_by": "library"
  }]
}
```

**owned_by values:**
- `"library"`: Official Ollama library model
- `"<username>"`: Community model (e.g., `"milkey"`, `"JollyLlama"`)

---

## Error Responses

### Model Not Found (404)

```json
{"error":"model 'nonexistent-model-12345' not found"}
```

### Unsupported Feature (501)

```json
{"error":"this model does not support embeddings"}
```

### Invalid Option (400)

```json
{"error":"think value \"low\" is not supported for this model"}
```

---

## Timing Characteristics

### Latencies (model already loaded)

| Operation | Time to First Byte | Total Duration |
|-----------|-------------------|----------------|
| `GET /` | ~1.1 ms | ~1.1 ms |
| `GET /api/version` | ~0.4 ms | ~0.4 ms |
| `GET /api/tags` | ~0.5 ms | ~0.5 ms |
| `GET /api/ps` | ~0.5 ms | ~0.5 ms |
| `GET /v1/models` | ~1.4 ms | ~1.4 ms |
| `POST /api/chat` (streaming, 20 tokens) | ~73 ms | ~358 ms |
| `POST /api/chat` (non-streaming, 20 tokens) | ~350 ms | ~350 ms |
| `POST /api/chat` (think=true, 100 tokens) | ~4000 ms | ~4125 ms |
| `POST /api/chat` (think=false, 12 tokens) | ~265 ms | ~265 ms |

### Cold Start Latency (model needs loading)

When the model is not loaded, time-to-first-byte is 5,730 ms.
- `load_duration`: 5,650,132,100 ns (~5.65 seconds)

### Token Generation Rate (qwen3:32b Q4_K_M)

| Mode | Prompt tok/s | Generation tok/s |
|------|--------------|------------------|
| think=true | ~520 | ~67 |
| think=false | ~771 | ~71 |
| With tools | ~2462 | ~67 |

### Streaming Chunk Interval

- **Average interval:** ~15.3 ms between chunks
- **Chunk size:** 94-298 bytes (varies with content)

---

## HTTP Headers

### Response Headers by Endpoint Type

**JSON endpoints:**
```
Content-Type: application/json; charset=utf-8
Date: Sat, 10 Jan 2026 16:33:04 GMT
Content-Length: <varies>
```

**Native streaming:**
```
Content-Type: application/x-ndjson
Date: Sat, 10 Jan 2026 16:33:10 GMT
Transfer-Encoding: chunked
```

**OpenAI streaming:**
```
Content-Type: text/event-stream
Date: Sat, 10 Jan 2026 16:33:11 GMT
Transfer-Encoding: chunked
```

**Plain text (root):**
```
Content-Type: text/plain; charset=utf-8
Date: Sat, 10 Jan 2026 16:33:04 GMT
Content-Length: 17
```

---

## Mock Server Implementation Notes

For creating accurate mock Ollama servers:

1. **Timeout handling:** Real Ollama can hang indefinitely. Mock should respect client timeouts.

2. **Streaming format detection:**
   - Native API: Default to `application/x-ndjson`
   - OpenAI API: Check `stream` field, use `text/event-stream`

3. **Chunk timing:** Emit chunks every ~15ms to simulate realistic generation speed.

4. **Cold start simulation:** Delay first chunk by 5-60 seconds when simulating model loading.

5. **Thinking models:**
   - Native API: Include `thinking` field
   - OpenAI API: Include `reasoning` field (different name!)

6. **done_reason values:**
   - `"stop"`: Model decided to stop (EOS token or stop sequence matched)
   - `"length"`: Hit `num_predict` / `max_tokens` limit

7. **Error codes:**
   - 400: Invalid options/parameters (e.g., unsupported think level)
   - 404: Model not found
   - 501: Feature not supported (e.g., embeddings on non-embedding model)
   - 502: Backend error (for simulating server failures)

8. **Tool calls:**
   - Return `tool_calls` array when tools are provided
   - `arguments` is parsed JSON object, not string
   - `id` format: `call_` + 8 alphanumeric characters

9. **Options validation:**
   - `think`: Boolean works for qwen3/seed-oss, string required for gpt-oss
   - All sampling parameters are accepted

10. **Stop sequences:**
    - Check generated text against `stop` array
    - Set `done_reason: "stop"` when matched
    - Stop sequence itself is NOT included in output

11. **SSE format (OpenAI):**
    - Prefix each line with `data: `
    - Double newline after each message
    - End with literal `data: [DONE]`

---

## Exact Response Bodies

This section contains **exact** JSON responses from Ollama 0.13.5, for creating byte-accurate mock servers.

### POST /api/chat Non-Streaming - Exact Response

**Request:**
```json
{
  "model": "qwen3:32b",
  "messages": [{"role": "user", "content": "What is 2+2? Reply in one word."}],
  "stream": false,
  "options": {"num_predict": 20}
}
```

**Exact Response (372ms latency):**
```json
{
  "model": "qwen3:32b",
  "created_at": "2026-01-10T16:55:30.827221299Z",
  "message": {
    "role": "assistant",
    "content": "",
    "thinking": "Okay, the user is asking \"What is 2+2?\" and wants the"
  },
  "done": true,
  "done_reason": "length",
  "total_duration": 371277452,
  "load_duration": 59260050,
  "prompt_eval_count": 22,
  "prompt_eval_duration": 33387435,
  "eval_count": 20,
  "eval_duration": 267475275
}
```

### POST /api/chat Streaming - Exact Chunks

**First chunk (intermediate):**
```json
{"model":"qwen3:32b","created_at":"2026-01-10T16:55:30.915663454Z","message":{"role":"assistant","content":"","thinking":"Okay"},"done":false}
```

**Last chunk (final with stats):**
```json
{"model":"qwen3:32b","created_at":"2026-01-10T16:55:31.156677988Z","message":{"role":"assistant","content":""},"done":true,"done_reason":"length","total_duration":328393900,"load_duration":40996735,"prompt_eval_count":22,"prompt_eval_duration":15246873,"eval_count":20,"eval_duration":265432931}
```

**Chunk statistics (18 chunks total):**
- Min interval: 0.03 ms
- Max interval: 15.64 ms
- Avg interval: 14.16 ms

### POST /api/generate - Exact Response

**Request:**
```json
{
  "model": "qwen3:32b",
  "prompt": "The capital of France is",
  "stream": false,
  "options": {"num_predict": 10}
}
```

**Exact Response:**
```json
{
  "model": "qwen3:32b",
  "created_at": "2026-01-10T16:55:31.357110147Z",
  "response": "",
  "thinking": "Okay, so I need to figure",
  "done": true,
  "done_reason": "length",
  "context": [151644, 872, 198, 785, 6722, 315, 9625, 374, 608, 26865, 151645, 198, 151644, 77091, 198, 151667, 198, 32313, 11, 773, 358, 1184, 311, 7071],
  "total_duration": 199697039,
  "load_duration": 53554089,
  "prompt_eval_count": 15,
  "prompt_eval_duration": 23534750,
  "eval_count": 10,
  "eval_duration": 119391279
}
```

**Key difference from /api/chat:**
- `thinking` is at top level (not inside `message`)
- `response` field instead of `message.content`
- `context` array contains token IDs for continuation

### GET /api/ps - Exact Response

```json
{
  "models": [{
    "name": "qwen3:32b",
    "model": "qwen3:32b",
    "size": 21579390080,
    "digest": "030ee887880fc378860c2dd35101da424377520441ae4bfe7be6deff8ade7840",
    "details": {
      "parent_model": "",
      "format": "gguf",
      "family": "qwen3",
      "families": ["qwen3"],
      "parameter_size": "32.8B",
      "quantization_level": "Q4_K_M"
    },
    "expires_at": "2318-04-22T19:29:44.962381934+03:00",
    "size_vram": 21579390080,
    "context_length": 4096
  }]
}
```

**Note:** `context_length` varies based on current allocation (4096 with default settings vs 32768 when more context is requested).

### GET /api/tags - Exact Model Entry

```json
{
  "name": "devstral-vibe:latest",
  "model": "devstral-vibe:latest",
  "modified_at": "2026-01-02T01:00:46.891738203+02:00",
  "size": 15177374145,
  "digest": "20377ea31d6edf7c3154fb7dd9a214e4b419611dce389635471a8006ec8ec853",
  "details": {
    "parent_model": "",
    "format": "gguf",
    "family": "mistral3",
    "families": ["mistral3"],
    "parameter_size": "24.0B",
    "quantization_level": "Q4_K_M"
  }
}
```

### POST /v1/chat/completions Non-Streaming - Exact Response

**Request:**
```json
{
  "model": "qwen3:32b",
  "messages": [{"role": "user", "content": "Say hello."}],
  "max_tokens": 20
}
```

**Exact Response:**
```json
{
  "id": "chatcmpl-514",
  "object": "chat.completion",
  "created": 1768064131,
  "model": "qwen3:32b",
  "system_fingerprint": "fp_ollama",
  "choices": [{
    "index": 0,
    "message": {
      "role": "assistant",
      "content": "",
      "reasoning": "Okay, the user asked me to say hello. This is a very simple and common"
    },
    "finish_reason": "length"
  }],
  "usage": {
    "prompt_tokens": 13,
    "completion_tokens": 20,
    "total_tokens": 33
  }
}
```

### POST /v1/chat/completions Streaming - Raw SSE Lines

**First 3 lines exactly as received:**
```
data: {"id":"chatcmpl-492","object":"chat.completion.chunk","created":1768064132,"model":"qwen3:32b","system_fingerprint":"fp_ollama","choices":[{"index":0,"delta":{"role":"assistant","content":"","reasoning":"Okay"},"finish_reason":null}]}

data: {"id":"chatcmpl-492","object":"chat.completion.chunk","created":1768064132,"model":"qwen3:32b","system_fingerprint":"fp_ollama","choices":[{"index":0,"delta":{"role":"assistant","content":"","reasoning":","},"finish_reason":null}]}

data: {"id":"chatcmpl-492","object":"chat.completion.chunk","created":1768064132,"model":"qwen3:32b","system_fingerprint":"fp_ollama","choices":[{"index":0,"delta":{"role":"assistant","content":"","reasoning":" the"},"finish_reason":null}]}
```

**Terminator:** `data: [DONE]`

### POST /api/show - Top-Level Fields

| Field | Type | Size |
|-------|------|------|
| `license` | string | 11,338 chars |
| `modelfile` | string | 13,466 chars |
| `parameters` | string | 227 chars |
| `template` | string | 1,723 chars |
| `details` | dict | 6 keys |
| `model_info` | dict | 26 keys |
| `tensors` | list | 707 items |
| `capabilities` | list | 3 items |
| `modified_at` | string | ISO 8601 timestamp |

### All Sampling Parameters

All parameters accepted with status 200 OK:

| Parameter | Value | Status |
|-----------|-------|--------|
| `temperature` | 0.5 | OK |
| `top_p` | 0.9 | OK |
| `top_k` | 40 | OK |
| `min_p` | 0.05 | OK |
| `repeat_penalty` | 1.1 | OK |
| `presence_penalty` | 0.5 | OK |
| `frequency_penalty` | 0.5 | OK |
| `seed` | 42 | OK |
| `mirostat` | 2 | OK |
| `mirostat_tau` | 5.0 | OK |
| `mirostat_eta` | 0.1 | OK |
| `tfs_z` | 0.95 | OK |
| `typical_p` | 0.9 | OK |
| `num_ctx` | 4096 | OK |
| `num_predict` | 20 | OK |
| `num_gpu` | 99 | OK |
| `num_batch` | 512 | OK |
| `num_thread` | 4 | OK |

