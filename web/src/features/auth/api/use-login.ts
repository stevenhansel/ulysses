import { useMutation } from '@tanstack/react-query'
import { apiClient } from '@/lib/api-client'
import type { LoginRequest, LoginResponse } from '@/types'

export function useLogin() {
  return useMutation({
    mutationFn: (data: LoginRequest) =>
      apiClient.post<LoginResponse>('/auth/login', data),
  })
}
