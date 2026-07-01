import { useState, useRef } from 'react'
import { Button } from '@/components/ui/button'
import { Upload, User } from 'lucide-react'
import { apiClient } from '@/lib/api-client'

interface AvatarUploaderProps {
  currentUrl?: string | null
  onUploadComplete?: (url: string) => void
}

export function AvatarUploader({ currentUrl, onUploadComplete }: AvatarUploaderProps) {
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const inputRef = useRef<HTMLInputElement>(null)

  async function handleFileChange(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    if (!file) return

    setIsUploading(true)
    setError(null)

    try {
      const formData = new FormData()
      formData.append('avatar', file)
      const result = await apiClient.upload<{ url: string }>('/user/avatar', formData)
      onUploadComplete?.(result.url)
    } catch {
      setError('Failed to upload avatar. Please try again.')
    } finally {
      setIsUploading(false)
    }
  }

  return (
    <div className="flex flex-col items-center gap-3">
      <div className="relative h-20 w-20 rounded-full overflow-hidden border border-border bg-muted flex items-center justify-center">
        {currentUrl ? (
          <img src={currentUrl} alt="Avatar" className="h-full w-full object-cover" />
        ) : (
          <User className="h-8 w-8 text-muted-foreground" />
        )}
      </div>

      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        className="hidden"
        onChange={handleFileChange}
      />

      <Button
        type="button"
        variant="outline"
        size="sm"
        disabled={isUploading}
        onClick={() => inputRef.current?.click()}
      >
        <Upload className="h-4 w-4 mr-2" />
        {isUploading ? 'Uploading...' : 'Upload Avatar'}
      </Button>

      {error && <p className="text-sm text-destructive">{error}</p>}
    </div>
  )
}
