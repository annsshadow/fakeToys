import { expect, test } from '@playwright/test'
import { assertNo404OrServerErrors, auditApiResponses, expectSuccessfulWrite, login, uniqueFlag } from './support/live'

test('starts, renders, and returns a workflow through visible web controls', async ({ page }) => {
  const audit = auditApiResponses(page)
  const title = `S3 运行闭环 ${uniqueFlag('work')}`

  await login(page)
  await page.goto('/app/process')
  await expect(page.getByRole('heading', { name: '工作流待办' })).toBeVisible()

  // This intentionally remains a live acceptance contract: until the runtime implements
  // a visible start action and Xform renderer, the test fails rather than bypassing them
  // with direct API calls or silently skipping the scenario.
  await page.getByRole('button', { name: /发起流程/ }).click()
  await page.getByLabel(/流程/).selectOption({ label: process.env.E2E_PROCESS_NAME ?? 'S3 Contract Process' })
  await page.getByLabel(/标题/).fill(title)
  await page.getByLabel(/申请原因/).fill('S3 GUI E2E round-trip')
  const startResponse = audit.waitForWrite((response) =>
    /\/surface\/(draft\/[^/]+\/start|work\/start)$/.test(new URL(response.url()).pathname),
  )
  await page.getByRole('button', { name: /提交|发起/ }).click()
  await expectSuccessfulWrite(await startResponse)

  await page.getByRole('button', { name: '我发起的' }).click()
  await expect(page.getByText(title, { exact: true })).toBeVisible()
  await page.getByText(title, { exact: true }).click()
  await expect(page.getByLabel(/申请原因/)).toHaveValue('S3 GUI E2E round-trip')

  const returnResponse = audit.waitForWrite((response) =>
    /\/surface\/(work\/processing|task\/processing)\//.test(new URL(response.url()).pathname),
  )
  await page.getByLabel(/意见/).fill('S3 GUI E2E approved')
  await page.getByRole('button', { name: /审批通过|通过/ }).click()
  await expectSuccessfulWrite(await returnResponse)
  await expect(page.getByText(title, { exact: true })).toHaveCount(0)
  await assertNo404OrServerErrors(audit)
})
