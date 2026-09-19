import { expect, test } from '@playwright/test'
import { assertNo404OrServerErrors, auditApiResponses, expectSuccessfulWrite, login, uniqueFlag } from './support/live'

test('starts, renders, and returns a workflow through visible web controls', async ({ page }) => {
  const audit = auditApiResponses(page)
  const title = `S3 运行闭环 ${uniqueFlag('work')}`

  await login(page)
  await page.goto('/app/process')
  await expect(page.getByRole('heading', { name: '工作流待办' })).toBeVisible()

  // 通过可见 web 控件从零发起流程：选流程（seed-approval-flow）→ 填标题 →
  // Xform 填报（申请原因）→ 提交 → 我发起的可见 → 发起人办理 begin 任务审批通过。
  const processName = process.env.E2E_PROCESS_NAME ?? 'seed-approval-flow'
  await page.getByRole('button', { name: /发起流程/ }).click()
  await page.getByLabel(/流程/).selectOption({ label: processName })
  await page.getByLabel(/标题/).fill(title)
  await page.getByLabel(/申请原因/).fill('S3 GUI E2E round-trip')
  const startResponse = audit.waitForWrite((response) =>
    /\/service\/processing\/work$/.test(new URL(response.url()).pathname),
  )
  await page.getByRole('button', { name: '发起', exact: true }).click()
  await expectSuccessfulWrite(await startResponse)

  await expect(page.getByRole('button', { name: '我发起的' })).toHaveClass(/active/)
  await expect(page.getByText(title, { exact: true })).toBeVisible()
  await page.getByText(title, { exact: true }).click()
  await expect(page.getByLabel(/申请原因/)).toHaveValue('S3 GUI E2E round-trip')

  const completeResponse = audit.waitForWrite((response) =>
    /\/api\/task\/[^/]+\/complete$/.test(new URL(response.url()).pathname),
  )
  await page.getByLabel(/处理意见/).fill('S3 GUI E2E approved')
  await page.getByRole('button', { name: /审批通过/ }).click()
  await expectSuccessfulWrite(await completeResponse)
  // 审批完成后工作收尾为 completed，待办中不再出现
  await page.getByRole('button', { name: '待我处理' }).click()
  await expect(page.getByText(title, { exact: true })).toHaveCount(0)
  await assertNo404OrServerErrors(audit)
})
