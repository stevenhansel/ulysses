import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import { Link } from '@tanstack/react-router'
import { Cpu, GitFork, Activity } from 'lucide-react'

export const Route = createFileRoute('/')({
  component: HomePage,
})

function HomePage() {
  return (
    <div className="flex flex-col items-center justify-center text-center py-20">
      <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
        Ulysses
      </h1>
      <p className="mt-4 text-lg text-muted-foreground max-w-2xl">
        A lightweight proxy and model manager for self-hosted LLMs.
        Route requests, hot-swap models, and monitor performance — all from one dashboard.
      </p>

      <div className="mt-10 grid grid-cols-1 sm:grid-cols-3 gap-6 max-w-2xl w-full">
        <div className="rounded-lg border border-border p-4 text-left">
          <GitFork className="h-5 w-5 text-primary mb-2" />
          <h3 className="font-semibold text-sm">Model Profiles</h3>
          <p className="text-xs text-muted-foreground mt-1">
            Configure backends, models, and inference parameters.
          </p>
        </div>
        <div className="rounded-lg border border-border p-4 text-left">
          <Cpu className="h-5 w-5 text-primary mb-2" />
          <h3 className="font-semibold text-sm">Hot Swapping</h3>
          <p className="text-xs text-muted-foreground mt-1">
            Automatically load and swap models on demand.
          </p>
        </div>
        <div className="rounded-lg border border-border p-4 text-left">
          <Activity className="h-5 w-5 text-primary mb-2" />
          <h3 className="font-semibold text-sm">Monitoring</h3>
          <p className="text-xs text-muted-foreground mt-1">
            Track hardware and inference metrics in real time.
          </p>
        </div>
      </div>

      <div className="mt-10 flex gap-4">
        <Button asChild>
          <Link to="/dashboard">Go to Dashboard</Link>
        </Button>
        <Button variant="outline" asChild>
          <Link to="/settings">Settings</Link>
        </Button>
      </div>
    </div>
  )
}
