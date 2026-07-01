import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/settings')({
  component: SettingsPage,
})

function SettingsPage() {
  return (
    <div className="py-6">
      <h1 className="text-2xl font-bold tracking-tight">Settings</h1>
      <p className="text-muted-foreground mt-1">
        Manage your model profiles and application preferences.
      </p>
      <div className="mt-8 max-w-xl space-y-8">
        <section className="rounded-lg border border-border p-6">
          <h2 className="text-lg font-semibold">Model Profiles</h2>
          <p className="text-sm text-muted-foreground mt-1">
            Configure inference backends and model identifiers.
          </p>
          <p className="text-sm text-muted-foreground mt-4 italic">
            Profile management coming soon.
          </p>
        </section>
      </div>
    </div>
  )
}
