import { renderHook, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { http, HttpResponse } from 'msw'
import { server } from '../../../../tests/mock-server'
import { describe, it, expect, beforeEach } from 'vitest'
import { useProfiles } from './use-profile'
import type { Profile } from '@/types'
import type { ReactNode } from 'react'

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  })

  return function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    )
  }
}

describe('useProfiles', () => {
  beforeEach(() => {
    // Reset to default handlers between tests
    server.resetHandlers()
  })

  it('returns a list of profiles on success', async () => {
    const { result } = renderHook(() => useProfiles(), { wrapper: createWrapper() })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    const profiles = result.current.data as Profile[]
    expect(profiles).toHaveLength(2)
    expect(profiles[0].name).toBe('Llama 3 8B')
  })

  it('returns an empty list when no profiles exist', async () => {
    server.use(
      http.get('/api/profiles', () => {
        return HttpResponse.json<Profile[]>([])
      }),
    )

    const { result } = renderHook(() => useProfiles(), { wrapper: createWrapper() })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    expect(result.current.data).toEqual([])
  })

  it('handles API error', async () => {
    server.use(
      http.get('/api/profiles', () => {
        return HttpResponse.json(
          { message: 'Internal server error' },
          { status: 500 },
        )
      }),
    )

    const { result } = renderHook(() => useProfiles(), { wrapper: createWrapper() })

    await waitFor(() => {
      expect(result.current.isError).toBe(true)
    })
  })
})
