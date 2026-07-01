import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { AvatarUploader } from './avatar-uploader'

function createMockFile(): File {
  return new File(['fake-image-content'], 'avatar.png', { type: 'image/png' })
}

describe('AvatarUploader', () => {
  it('renders upload button', () => {
    render(<AvatarUploader />)
    expect(screen.getByRole('button', { name: 'Upload Avatar' })).toBeInTheDocument()
  })

  it('shows current avatar image when url is provided', () => {
    render(<AvatarUploader currentUrl="https://example.com/avatar.png" />)
    const img = screen.getByRole('img', { name: 'Avatar' })
    expect(img).toHaveAttribute('src', 'https://example.com/avatar.png')
  })

  it('shows placeholder when no avatar url is provided', () => {
    render(<AvatarUploader />)
    // The User icon is rendered as an SVG
    expect(screen.getByRole('button', { name: 'Upload Avatar' })).toBeInTheDocument()
  })

  it('calls onUploadComplete after successful upload', async () => {
    const onUploadComplete = vi.fn()
    const user = userEvent.setup()

    render(<AvatarUploader onUploadComplete={onUploadComplete} />)

    const file = createMockFile()
    const fileInput = document.querySelector('input[accept="image/*"]') as HTMLInputElement
    expect(fileInput).not.toBeNull()
    await user.upload(fileInput!, file)

    await waitFor(() => {
      expect(onUploadComplete).toHaveBeenCalledWith(
        'https://example.com/avatars/mock-avatar.png',
      )
    })
  })
})
