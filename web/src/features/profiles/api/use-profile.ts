import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api-client'
import type { Profile, ProfileCreateRequest, ProfileUpdateRequest } from '@/types'

const PROFILES_KEY = ['profiles'] as const

export function useProfiles() {
  return useQuery({
    queryKey: PROFILES_KEY,
    queryFn: () => apiClient.get<Profile[]>('/profiles'),
  })
}

export function useProfile(id: string) {
  return useQuery({
    queryKey: [...PROFILES_KEY, id],
    queryFn: () => apiClient.get<Profile>(`/profiles/${id}`),
    enabled: !!id,
  })
}

export function useCreateProfile() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: ProfileCreateRequest) =>
      apiClient.post<Profile>('/profiles', data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PROFILES_KEY })
    },
  })
}

export function useUpdateProfile(id: string) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: ProfileUpdateRequest) =>
      apiClient.put<Profile>(`/profiles/${id}`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PROFILES_KEY })
    },
  })
}

export function useDeleteProfile() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) => apiClient.delete(`/profiles/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PROFILES_KEY })
    },
  })
}
