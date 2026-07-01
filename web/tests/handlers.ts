import { http, HttpResponse } from 'msw'
import type { LoginRequest, LoginResponse, Profile } from '@/types'

// --- Auth handlers ---
const authHandlers = [
  http.post('/api/auth/login', async ({ request }) => {
    const body = (await request.json()) as LoginRequest

    if (body.username === 'admin' && body.password === 'password') {
      return HttpResponse.json<LoginResponse>({
        token: 'mock-jwt-token',
        user: {
          id: '1',
          username: 'admin',
          avatar_url: null,
          created_at: new Date().toISOString(),
        },
      })
    }

    return HttpResponse.json(
      { message: 'Invalid username or password' },
      { status: 401 },
    )
  }),
]

// --- Profile handlers ---
const mockProfiles: Profile[] = [
  {
    id: '1',
    name: 'Llama 3 8B',
    backend_endpoint: 'http://localhost:8080/v1',
    model_identifier: 'llama-3-8b-instruct.Q4_K_M.gguf',
    context_length: 8192,
    gpu_layers: 35,
    inference_params: { temperature: 0.7 },
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
  {
    id: '2',
    name: 'Mistral 7B',
    backend_endpoint: 'http://localhost:8081/v1',
    model_identifier: 'mistral-7b-instruct-v0.3.Q4_K_M.gguf',
    context_length: null,
    gpu_layers: null,
    inference_params: null,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
]

const profileHandlers = [
  http.get('/api/profiles', () => {
    return HttpResponse.json<Profile[]>(mockProfiles)
  }),

  http.get('/api/profiles/:id', ({ params }) => {
    const profile = mockProfiles.find((p) => p.id === params.id)
    if (!profile) {
      return HttpResponse.json({ message: 'Profile not found' }, { status: 404 })
    }
    return HttpResponse.json<Profile>(profile)
  }),

  http.post('/api/profiles', async ({ request }) => {
    const body = (await request.json()) as Partial<Profile>
    const newProfile: Profile = {
      id: String(mockProfiles.length + 1),
      name: body.name ?? 'New Profile',
      backend_endpoint: body.backend_endpoint ?? 'http://localhost:8080/v1',
      model_identifier: body.model_identifier ?? 'model.gguf',
      context_length: body.context_length ?? null,
      gpu_layers: body.gpu_layers ?? null,
      inference_params: body.inference_params ?? null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    }
    return HttpResponse.json<Profile>(newProfile, { status: 201 })
  }),

  http.put('/api/profiles/:id', async ({ params, request }) => {
    const body = (await request.json()) as Partial<Profile>
    const profile = mockProfiles.find((p) => p.id === params.id)
    if (!profile) {
      return HttpResponse.json({ message: 'Profile not found' }, { status: 404 })
    }
    return HttpResponse.json<Profile>({ ...profile, ...body, updated_at: new Date().toISOString() })
  }),

  http.delete('/api/profiles/:id', ({ params }) => {
    const profile = mockProfiles.find((p) => p.id === params.id)
    if (!profile) {
      return HttpResponse.json({ message: 'Profile not found' }, { status: 404 })
    }
    return HttpResponse.json<{ success: boolean }>({ success: true })
  }),
]

// --- User handlers ---
const userHandlers = [
  http.post('/api/user/avatar', () => {
    return HttpResponse.json<{ url: string }>({
      url: 'https://example.com/avatars/mock-avatar.png',
    })
  }),
]

export const handlers = [
  ...authHandlers,
  ...profileHandlers,
  ...userHandlers,
]
