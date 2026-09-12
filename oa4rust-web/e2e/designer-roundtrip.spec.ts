import { expect, test } from '@playwright/test'
import { assertNo404OrServerErrors, auditApiResponses, expectSuccessfulWrite, login, uniqueFlag } from './support/live'

test.beforeEach(async ({ page }) => login(page))

test('creates and round-trips a process through the web designer', async ({ page }) => {
  const audit = auditApiResponses(page)
  const flag = uniqueFlag('s3-process')
  const name = `S3 流程 ${flag}`

  await page.goto('/app/process-designer')
  await expect(page.getByRole('heading', { name: '流程设计器' })).toBeVisible()
  await page.locator('.sb-header').getByRole('button', { name: '+ 新建' }).click()
  await page.getByPlaceholder('如: 请假审批流程').fill(name)
  await page.getByPlaceholder('如: leave_approval_v2').fill(flag)
  const createResponse = audit.waitForWrite((response) =>
    new URL(response.url()).pathname.endsWith('/designer/process'),
  )
  await page.getByRole('button', { name: '创建', exact: true }).click()
  await expectSuccessfulWrite(await createResponse)
  await expect(page.getByText(name, { exact: true })).toBeVisible()

  await page.reload()
  await expect(page.getByText(name, { exact: true })).toBeVisible()
  await page.getByText(name, { exact: true }).click()
  await expect(page.locator('.pd-palette')).toBeVisible()
  const saveResponse = audit.waitForWrite((response) =>
    /\/designer\/process\/[^/]+$/.test(new URL(response.url()).pathname),
  )
  await page.getByRole('button', { name: /保存/ }).first().click()
  const saved = await saveResponse
  await expectSuccessfulWrite(saved)
  const payload = saved.request().postDataJSON()
  expect(payload.processDefinition.activities).toEqual(expect.any(Array))
  expect(payload.processDefinition.routes).toEqual(expect.any(Array))
  await assertNo404OrServerErrors(audit)
})

test('creates and round-trips a form through the web designer', async ({ page }) => {
  const audit = auditApiResponses(page)
  const flag = uniqueFlag('s3-form')
  const name = `S3 表单 ${flag}`

  await page.goto('/app/form-designer')
  await expect(page.getByRole('heading', { name: '表单设计器' })).toBeVisible()
  await page.getByTitle('新建表单').click()
  await page.getByPlaceholder('表单名称').fill(name)
  await page.getByPlaceholder('唯一标识(Flag)').fill(flag)
  await page.locator('.palette-item').filter({ hasText: '文本' }).first().click()
  await page.getByPlaceholder('字段标签').fill('申请原因')
  await page.getByPlaceholder('唯一标识', { exact: true }).fill('reason')
  const saveResponse = audit.waitForWrite((response) => new URL(response.url()).pathname === '/jaxrs/form')
  await page.getByTitle('保存').click()
  const saved = await saveResponse
  await expectSuccessfulWrite(saved)
  expect(saved.request().postDataJSON().moduleList).toEqual(expect.any(Object))
  await expect(page.getByText(name, { exact: true })).toBeVisible()

  await page.reload()
  await page.getByTitle('刷新列表').click()
  await expect(page.getByText(name, { exact: true })).toBeVisible()
  await page.getByText(name, { exact: true }).click()
  await expect(page.getByPlaceholder('表单名称')).toHaveValue(name)
  await expect(page.getByText('申请原因', { exact: true })).toBeVisible()
  await assertNo404OrServerErrors(audit)
})

test('creates and round-trips a query through the web designer', async ({ page }) => {
  const audit = auditApiResponses(page)
  const flag = uniqueFlag('s3-query')
  const name = `S3 查询 ${flag}`
  const sql = 'SELECT 1 AS contract_value'

  await page.goto('/app/query-designer')
  await expect(page.getByRole('heading', { name: '查询设计器' })).toBeVisible()
  await page.getByRole('button', { name: /新建查询/ }).click()
  await page.getByPlaceholder('查询名称').fill(name)
  await page.getByPlaceholder('如: person, unit').fill('contract')
  await page.getByPlaceholder('SELECT * FROM ...').fill(sql)
  const createResponse = audit.waitForWrite((response) => new URL(response.url()).pathname.endsWith('/designer/create'))
  await page.getByRole('button', { name: '创建', exact: true }).click()
  const created = await createResponse
  await expectSuccessfulWrite(created)
  expect(created.request().postDataJSON()).toMatchObject({ name, category: 'contract', query: sql })
  await expect(page.getByText(name, { exact: true })).toBeVisible()

  await page.reload()
  await expect(page.getByText(name, { exact: true })).toBeVisible()
  await page.getByText(name, { exact: true }).click()
  await page.getByRole('button', { name: /编辑/ }).click()
  await expect(page.getByPlaceholder('SELECT * FROM ...')).toHaveValue(sql)
  await assertNo404OrServerErrors(audit)
})

test('creates and round-trips a portal page through the web designer', async ({ page }) => {
  const audit = auditApiResponses(page)
  const flag = uniqueFlag('s3-portal')
  const name = `S3 门户 ${flag}`
  const layout = '[{"type":"row","widgets":[{"type":"text","text":"S3 portal"}]}]'

  await page.goto('/app/portal-designer')
  await expect(page.getByRole('heading', { name: '门户设计器' })).toBeVisible()
  await page.getByRole('button', { name: /新建页面/ }).click()
  await page.getByPlaceholder('页面名称').fill(name)
  await page.getByPlaceholder('唯一标识').fill(flag)
  await page.getByPlaceholder('[{"type":"row","widgets":[...]}]').fill(layout)
  const createResponse = audit.waitForWrite((response) => new URL(response.url()).pathname.endsWith('/designer/page'))
  await page.getByRole('button', { name: '保存', exact: true }).click()
  const created = await createResponse
  await expectSuccessfulWrite(created)
  expect(created.request().postDataJSON()).toMatchObject({ name, content: { flag, layout: JSON.parse(layout) } })
  await expect(page.getByText(name, { exact: true })).toBeVisible()

  await page.reload()
  await expect(page.getByText(name, { exact: true })).toBeVisible()
  await page.getByText(name, { exact: true }).click()
  await expect(page.getByPlaceholder('页面名称')).toHaveValue(name)
  await expect(page.getByPlaceholder('唯一标识')).toHaveValue(flag)
  await expect(page.getByPlaceholder('[{"type":"row","widgets":[...]}]')).toHaveValue(layout)
  await assertNo404OrServerErrors(audit)
})
