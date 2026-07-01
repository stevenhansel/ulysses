// Auto-generated TypeScript definitions from Rust (ts-rs)
// This file is regenerated when the Rust API is built.
// See api/src/ with #[derive(TS)] attributes.

// --- Auth ---
export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  token: string
  user: User
}

// --- User ---
export interface User {
  id: string
  username: string
  avatar_url: string | null
  created_at: string
}

// --- Profile (Model Configuration) ---
export interface Profile {
  id: string
  name: string
  backend_endpoint: string
  model_identifier: string
  context_length: number | null
  gpu_layers: number | null
  inference_params: Record<string, unknown> | null
  created_at: string
  updated_at: string
}

export interface ProfileCreateRequest {
  name: string
  backend_endpoint: string
  model_identifier: string
  context_length?: number
  gpu_layers?: number
  inference_params?: Record<string, unknown>
}

export interface ProfileUpdateRequest {
  name?: string
  backend_endpoint?: string
  model_identifier?: string
  context_length?: number
  gpu_layers?: number
  inference_params?: Record<string, unknown>
}

// --- Hardware Metrics ---
export interface HardwareMetrics {
  cpu: CpuMetrics
  gpus: GpuMetrics[]
  ram: RamMetrics
}

export interface CpuMetrics {
  utilization: number
  temperature: number | null
  frequency: number
}

export interface GpuMetrics {
  utilization: number
  vram_used_mb: number
  vram_total_mb: number
  temperature: number
  power_draw_watts: number
}

export interface RamMetrics {
  total_gb: number
  used_gb: number
  available_gb: number
  swap_used_gb: number
}

// --- Inference Metrics ---
export interface InferenceMetrics {
  tokens_per_second: number
  prompt_processing_speed: number
  time_to_first_token_ms: number
  request_latency_ms: number
}
