import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard')({
  component: DashboardPage,
})

function DashboardPage() {
  return (
    <div className="py-6">
      <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
      <p className="text-muted-foreground mt-1">
        Monitor your models, hardware, and inference performance.
      </p>
      <div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Hardware metrics cards — to be implemented */}
        <div className="rounded-lg border border-border p-6">
          <h2 className="text-sm font-medium text-muted-foreground">CPU</h2>
          <p className="text-2xl font-bold mt-2">—</p>
        </div>
        <div className="rounded-lg border border-border p-6">
          <h2 className="text-sm font-medium text-muted-foreground">GPU</h2>
          <p className="text-2xl font-bold mt-2">—</p>
        </div>
        <div className="rounded-lg border border-border p-6">
          <h2 className="text-sm font-medium text-muted-foreground">RAM</h2>
          <p className="text-2xl font-bold mt-2">—</p>
        </div>
      </div>
    </div>
  )
}
