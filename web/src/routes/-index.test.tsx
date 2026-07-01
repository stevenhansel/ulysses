import { render, screen } from '@testing-library/react'
import { createRouter, RouterProvider, createMemoryHistory } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { describe, it, expect } from 'vitest'
import { routeTree } from '@/routeTree.gen'
import type { ReactNode } from 'react'

function renderHomePage() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })

  const router = createRouter({
    routeTree,
    history: createMemoryHistory({ initialEntries: ['/'] }),
    context: { queryClient },
    defaultPreload: false,
  })

  return {
    ...render(
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>,
    ),
    router,
  }
}

describe('HomePage', () => {
  it('renders the title', async () => {
    renderHomePage()
    const headings = await screen.findAllByText('Ulysses')
    expect(headings).toHaveLength(2) // nav brand + h1
  })

  it('renders the tagline', async () => {
    renderHomePage()
    expect(
      await screen.findByText(/A lightweight proxy and model manager for self-hosted LLMs/),
    ).toBeInTheDocument()
  })

  it('renders the feature cards', async () => {
    renderHomePage()
    expect(await screen.findByText('Model Profiles')).toBeInTheDocument()
    expect(await screen.findByText('Hot Swapping')).toBeInTheDocument()
    expect(await screen.findByText('Monitoring')).toBeInTheDocument()
  })

  it('renders navigation links', async () => {
    renderHomePage()
    expect(await screen.findByRole('link', { name: 'Dashboard' })).toBeInTheDocument()
    // Settings appears both in nav and as a button — use getAllByRole
    const settingsLinks = await screen.findAllByRole('link', { name: 'Settings' })
    expect(settingsLinks).toHaveLength(2)
  })
})
