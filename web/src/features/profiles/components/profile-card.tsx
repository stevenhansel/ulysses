import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Pencil, Trash2 } from 'lucide-react'
import type { Profile } from '@/types'

interface ProfileCardProps {
  profile: Profile
  onEdit?: (profile: Profile) => void
  onDelete?: (profile: Profile) => void
}

export function ProfileCard({ profile, onEdit, onDelete }: ProfileCardProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-start justify-between space-y-0">
        <div>
          <CardTitle className="text-base">{profile.name}</CardTitle>
          <CardDescription className="mt-1">
            {profile.backend_endpoint}
          </CardDescription>
        </div>
        <div className="flex items-center gap-1">
          {onEdit && (
            <Button variant="ghost" size="icon" onClick={() => onEdit(profile)} aria-label="Edit profile">
              <Pencil className="h-4 w-4" />
            </Button>
          )}
          {onDelete && (
            <Button variant="ghost" size="icon" onClick={() => onDelete(profile)} aria-label="Delete profile">
              <Trash2 className="h-4 w-4" />
            </Button>
          )}
        </div>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-muted-foreground">Model</span>
            <p className="font-medium truncate" title={profile.model_identifier}>
              {profile.model_identifier}
            </p>
          </div>
          {profile.context_length && (
            <div>
              <span className="text-muted-foreground">Context</span>
              <p className="font-medium">{profile.context_length.toLocaleString()} tokens</p>
            </div>
          )}
          {profile.gpu_layers !== null && profile.gpu_layers !== undefined && (
            <div>
              <span className="text-muted-foreground">GPU Layers</span>
              <p className="font-medium">{profile.gpu_layers}</p>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
