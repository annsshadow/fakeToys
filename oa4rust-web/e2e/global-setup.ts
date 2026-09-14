import type { FullConfig } from '@playwright/test'

export default async function globalSetup(config: FullConfig): Promise<void> {
  const project = config.projects[0]
  const baseURL = project?.use.baseURL
  if (typeof baseURL !== 'string' || !baseURL) throw new Error('BASE_URL must resolve to a non-empty absolute URL')
  if (!process.env.E2E_USERNAME || !process.env.E2E_PASSWORD) {
    throw new Error(
      'Live GUI E2E requires E2E_USERNAME and E2E_PASSWORD; credentials are never defaulted or silently skipped',
    )
  }

  const target = new URL('/login', baseURL)
  let response: Response
  try {
    response = await fetch(target, { redirect: 'manual', signal: AbortSignal.timeout(10_000) })
  } catch (error) {
    throw new Error(`Live GUI E2E cannot reach ${target}: ${error instanceof Error ? error.message : String(error)}`)
  }
  if (response.status >= 400) throw new Error(`Live GUI E2E preflight ${target} returned HTTP ${response.status}`)
}
