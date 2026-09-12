import { expect, type Page, type Response } from '@playwright/test'

export type ResponseAudit = {
  failures: string[]
  waitForWrite: (predicate: (response: Response) => boolean) => Promise<Response>
}

export async function login(page: Page): Promise<void> {
  const username = process.env.E2E_USERNAME
  const password = process.env.E2E_PASSWORD
  if (!username || !password) throw new Error('E2E_USERNAME and E2E_PASSWORD are required')

  await page.goto('/login')
  await page.getByPlaceholder('请输入用户名').fill(username)
  await page.getByPlaceholder('请输入密码').fill(password)
  await page.getByRole('button', { name: '登 录' }).click()
  await expect(page).toHaveURL(/\/app\//)
  await expect(page.getByRole('heading', { name: 'OA4RUST' })).toHaveCount(0)
}

export function auditApiResponses(page: Page): ResponseAudit {
  const failures: string[] = []
  page.on('response', (response) => {
    const url = new URL(response.url())
    if (!url.pathname.startsWith('/jaxrs/')) return
    if (response.status() === 404 || response.status() >= 500) failures.push(`${response.status()} ${url.pathname}`)
  })

  return {
    failures,
    waitForWrite: (predicate) =>
      page.waitForResponse(
        (response) => ['POST', 'PUT', 'PATCH'].includes(response.request().method()) && predicate(response),
      ),
  }
}

export async function expectSuccessfulWrite(response: Response): Promise<unknown> {
  expect(response.status(), `${response.request().method()} ${response.url()}`).toBeGreaterThanOrEqual(200)
  expect(response.status(), `${response.request().method()} ${response.url()}`).toBeLessThan(300)
  const body = await response.json()
  expect(body?.type ?? 'success', JSON.stringify(body)).not.toBe('error')
  return body
}

export async function assertNo404OrServerErrors(audit: ResponseAudit): Promise<void> {
  expect(audit.failures, 'Every exercised /jaxrs route must exist and avoid server errors').toEqual([])
}

export function uniqueFlag(prefix: string): string {
  return `${prefix}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
}
