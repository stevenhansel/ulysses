import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { ProfileCard } from './profile-card'
import type { Profile } from '@/types'

const mockProfile: Profile = {
  id: '1',
  name: 'Llama 3 8B',
  backend_endpoint: 'http://localhost:8080/v1',
  model_identifier: 'llama-3-8b-instruct.Q4_K_M.gguf',
  context_length: 8192,
  gpu_layers: 35,
  inference_params: { temperature: 0.7 },
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
}

describe('ProfileCard', () => {
  it('renders profile information', () => {
    render(<ProfileCard profile={mockProfile} />)

    expect(screen.getByText('Llama 3 8B')).toBeInTheDocument()
    expect(screen.getByText('http://localhost:8080/v1')).toBeInTheDocument()
    expect(screen.getByText('llama-3-8b-instruct.Q4_K_M.gguf')).toBeInTheDocument()
    expect(screen.getByText('8,192 tokens')).toBeInTheDocument()
    expect(screen.getByText('35')).toBeInTheDocument()
  })

  it('calls onEdit when edit button is clicked', async () => {
    const onEdit = vi.fn()
    const user = userEvent.setup()

    render(<ProfileCard profile={mockProfile} onEdit={onEdit} />)

    await user.click(screen.getByRole('button', { name: 'Edit profile' }))
    expect(onEdit).toHaveBeenCalledWith(mockProfile)
  })

  it('calls onDelete when delete button is clicked', async () => {
    const onDelete = vi.fn()
    const user = userEvent.setup()

    render(<ProfileCard profile={mockProfile} onDelete={onDelete} />)

    await user.click(screen.getByRole('button', { name: 'Delete profile' }))
    expect(onDelete).toHaveBeenCalledWith(mockProfile)
  })

  it('renders without optional fields', () => {
    const minimalProfile: Profile = {
      ...mockProfile,
      context_length: null,
      gpu_layers: null,
    }

    render(<ProfileCard profile={minimalProfile} />)

    expect(screen.getByText('Llama 3 8B')).toBeInTheDocument()
    expect(screen.queryByText('tokens')).not.toBeInTheDocument()
    expect(screen.queryByText('GPU Layers')).not.toBeInTheDocument()
  })
})
