# Ulysses

> A lightweight proxy and model manager for self-hosted LLMs. Designed for individuals with limited VRAM who need to serve multiple models without manual swapping.

Ulysses sits between your LLM client (e.g., SillyTavern, Open WebUI, a custom script) and one or more inference backends (e.g., llama.cpp, vLLM, anything exposing an OpenAI-compatible API). It routes requests to the right model, hot-swaps models on demand, and gives you real-time visibility into both hardware and inference performance.

## Features

### Model Management (Profiles)

Configure your inference backends and models in one place. Each **profile** defines:

- A backend endpoint (e.g., a llama.cpp server, vLLM instance, or any OpenAI-compatible API)
- A model identifier (e.g., a GGUF file path, a Hugging Face model name, or a model tag)
- Optional overrides like context length, GPU layers, or inference parameters

Ulysses is **backend-agnostic** — you link your own inference tooling. It ships with sensible defaults for llama.cpp but works with any server that speaks the OpenAI Chat Completions API.

### Hot Swapping

Ulysses acts as a smart gateway. When a request arrives:
- **Model is loaded** → the request is processed immediately.
- **Model is not loaded & no active requests** → the model is swapped in immediately, then the request is processed.
- **Model is not loaded & a request is in progress** → the request is queued. As soon as the active request finishes, Ulysses hot-swaps the model and processes the queued request automatically.

This means you never need to manually unload/reload models. Ulysses handles the switching so you can treat your VRAM-constrained setup like a multi-model server.

### Hardware Monitoring

Real-time dashboards for your system's hardware:

- **CPU** — utilization, temperature, frequency
- **GPUs** — utilization, VRAM usage, temperature, power draw (NVIDIA and AMD)
- **RAM** — total, used, available, swap
- Per-process metrics where applicable

### LLM Performance Monitoring

Track live inference metrics per-request and over time:

- **Tokens per second** (tok/s) — generation speed
- **Prompt processing speed** — tokens per second during the prefill / prompt evaluation phase
- **Time to first token** (TTFT)
- **Request latency** — queue wait time + inference time

All metrics are exposed via both a web dashboard and a JSON API for integration with external monitoring tools.

