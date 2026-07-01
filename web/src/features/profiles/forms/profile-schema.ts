import { z } from 'zod/v3'

export const profileSchema = z.object({
  name: z.string().min(1, 'Name is required').max(128, 'Name too long'),
  backend_endpoint: z.string().url('Must be a valid URL'),
  model_identifier: z.string().min(1, 'Model identifier is required'),
  context_length: z.number().positive().optional(),
  gpu_layers: z.number().min(0).optional(),
  inference_params: z.record(z.unknown()).optional(),
})

export type ProfileFormData = z.infer<typeof profileSchema>

export const profileUpdateSchema = profileSchema.partial()
export type ProfileUpdateFormData = z.infer<typeof profileUpdateSchema>
