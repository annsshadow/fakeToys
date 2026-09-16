import { expect, test } from '@playwright/test'
import { apiPost, assertNo404OrServerErrors, auditApiResponses, login, uniqueFlag } from './support/live'

/**
 * 本轮行为变更的 live GUI 回归（双栈实跑）：
 *  BBS 版块新建/重命名/删除 + 发帖/我的主题/我的回复（W14 ForumSection/ForumPerson）
 *  ProgramCenter 字典数据编辑器 + 脚本代码编辑器（W14 Dict/ScriptDesigner）
 *  IM 建会话 + 发消息（P5 IM 协议面）
 *  审批驳回（complete 侧由 workflow-runtime.spec.ts 覆盖）
 * 每个用例结束断言全程无 404/5xx——把「端点真实可操作」钉死在 GUI 层。
 */

test('BBS section lifecycle: create, rename, delete via sidebar controls', async ({ page }) => {
  const audit = auditApiResponses(page)
  await login(page)
  await page.goto('/app/bbs')
  await expect(page.locator('.section-list')).toBeVisible()

  // 版块名受输入框 maxlength=30 约束：原名 + 「-renamed」须 ≤30，故用短唯一名（避免 uniqueFlag 的 13 位时间戳把拼接名顶爆截断）。
  const sectionName = 'sec' + Math.random().toString(36).slice(2, 8)
  const renamed = `${sectionName}-renamed`

  // 新建版块（侧栏 + 按钮 → section/create）
  await page.getByRole('button', { name: '新建版块' }).click()
  await page.getByPlaceholder('版块名称').fill(sectionName)
  await page.getByRole('button', { name: '保存', exact: true }).click()
  await expect(page.getByText('版块已创建')).toBeVisible()
  const sectionRow = page.locator('.section-list .section-item', { hasText: sectionName })
  await expect(sectionRow).toHaveCount(1)

  // 重命名（行内 ✎ → section/save/{id}）
  await sectionRow.hover()
  await sectionRow.getByRole('button', { name: '重命名版块' }).click()
  await page.getByPlaceholder('版块名称').fill(renamed)
  await page.getByRole('button', { name: '保存', exact: true }).click()
  await expect(page.getByText('版块已重命名')).toBeVisible()
  await expect(page.locator('.section-list .section-item', { hasText: renamed })).toHaveCount(1)

  // 删除（行内 ✕ → 确认框「确认」→ section/delete/{id}）
  const renamedRow = page.locator('.section-list .section-item', { hasText: renamed })
  await renamedRow.hover()
  await renamedRow.getByRole('button', { name: '删除版块' }).click()
  await page.locator('#oa4-confirm-overlay').getByRole('button', { name: '确认' }).click()
  await expect(page.getByText('版块已删除')).toBeVisible()
  await expect(page.locator('.section-list .section-item', { hasText: renamed })).toHaveCount(0)

  await assertNo404OrServerErrors(audit)
})

test('BBS post shows in my topics; my replies tab lists my reply', async ({ page }) => {
  const audit = auditApiResponses(page)
  await login(page)
  // 会话 Cookie 绑定在 page 的 BrowserContext 上：造数走 page.request（共享 cookie）。
  // 注：内置 request fixture 是独立 APIRequestContext、不与 page 共享 cookie，未登录会 401。
  const topicTitle = uniqueFlag('bbs-topic')
  const replyText = uniqueFlag('bbs-reply')
  const topicRes = await apiPost(page.request, '/jaxrs/bbs/assemble/control/topic/create', {
    title: topicTitle,
    content: 'e2e topic',
    forumId: '',
    creator: 'e2e-live',
    authorId: 'e2e-live',
  })
  expect(topicRes.status(), 'topic/create must succeed').toBeLessThan(300)
  const topicId = (await topicRes.json())?.data?.id as string
  expect(topicId).toBeTruthy()
  const replyRes = await apiPost(page.request, '/jaxrs/bbs/assemble/control/reply/create', {
    topicId,
    content: replyText,
    creator: 'e2e-live',
  })
  expect(replyRes.status(), 'reply/create must succeed').toBeLessThan(300)

  await page.goto('/app/bbs')
  await expect(page.locator('.section-list')).toBeVisible()
  await page.getByRole('button', { name: '我的' }).click()

  // 我的主题（默认子栏，listsubjectinfo 读 x_bbs_topic 按 creator/author_id=e2e-live 过滤）
  await expect(page.getByText(topicTitle)).toBeVisible()

  // 我的回复子栏：PUT user/reply/my/list（按登录人过滤 x_bbs_reply）
  await page.getByRole('button', { name: '我的回复' }).click()
  await expect(page.getByText(replyText)).toBeVisible()

  await assertNo404OrServerErrors(audit)
})

test('ProgramCenter: dict create + data editor, script create + code editor + versions', async ({ page }) => {
  const audit = auditApiResponses(page)
  await login(page)
  await page.goto('/app/program')
  await expect(page.getByRole('heading', { name: /程序中心|ProgramCenter/ })).toBeVisible()

  // ── 字典：弹窗创建 → 数据编辑器写 JSON → 重开校验 ──
  const dictFlag = uniqueFlag('dict')
  await page.getByRole('button', { name: 'Dict' }).click()
  await page.getByRole('button', { name: '+ 新建字典' }).click()
  const dictModal = page.locator('.modal', { has: page.getByRole('heading', { name: '新建字典' }) })
  await dictModal.getByPlaceholder('字典名称').fill(dictFlag)
  await dictModal.getByPlaceholder(/dictFlag/).fill(dictFlag)
  await dictModal.getByRole('button', { name: '创建' }).click()
  await expect(page.getByText('字典已创建')).toBeVisible()

  const dictCard = page.locator('.item-card', { hasText: dictFlag }).first()
  await dictCard.getByRole('button', { name: '数据' }).click()
  await page.getByRole('button', { name: '保存数据' }).waitFor()
  const dictDataBox = page
    .locator('.modal', { has: page.getByRole('heading', { name: `字典数据 · ${dictFlag}` }) })
    .locator('textarea')
  await dictDataBox.fill('{"e2e":"1"}')
  await page.getByRole('button', { name: '保存数据' }).click()
  await expect(page.getByText('字典数据已保存')).toBeVisible()

  // 保存成功后编辑器自动关闭（showDictData=false）；重开数据编辑器校验服务端回读（JSON 缩进格式）。
  // 注：textarea 的值在 .value 而非 textContent，断言须用 toHaveValue（toContainText 对 v-model textarea 恒为空）。
  await dictCard.getByRole('button', { name: '数据' }).click()
  await expect(dictDataBox).toHaveValue(/"e2e"/)
  // 关闭数据编辑器弹窗（它遮住页签按钮），再继续脚本流程
  await page.getByRole('button', { name: '关闭' }).click()

  // ── 脚本：API 创建 → 编辑代码 → 保存 → 版本历史弹窗 ──
  const scriptFlag = uniqueFlag('script')
  const scriptRes = await apiPost(page.request, '/jaxrs/program_center/script', {
    name: scriptFlag,
    flag: scriptFlag,
    content: '// v1',
  })
  expect(scriptRes.status(), 'script create must succeed').toBeLessThan(300)
  // Script 页签无独立「刷新」按钮，经页签切换触发 loadScripts
  await page.getByRole('button', { name: 'Agent' }).click()
  await page.getByRole('button', { name: 'Script' }).click()
  const scriptCard = page.locator('.item-card', { hasText: scriptFlag }).first()
  await expect(scriptCard).toBeVisible()
  await scriptCard.getByRole('button', { name: '编辑代码' }).click()
  const codeBox = page.locator('.modal', { has: page.getByRole('heading', { name: /脚本代码/ }) }).locator('textarea')
  // textarea 值在 .value：用 toHaveValue 校验脚本代码回读（create 时写入 '// v1'）
  await expect(codeBox).toHaveValue(/\/\/ v1/)
  await codeBox.fill('// v1\n// e2e-edit')
  await page.getByRole('button', { name: '保存脚本' }).click()
  await expect(page.getByText('脚本已保存')).toBeVisible()
  // 保存后编辑器自动关闭；版本历史弹窗经卡片「版本」按钮打开
  await scriptCard.getByRole('button', { name: '版本' }).click()
  await expect(page.getByRole('heading', { name: /版本历史/ })).toBeVisible()

  await assertNo404OrServerErrors(audit)
})

test('IM: seed conversation via API, open in GUI and send a message', async ({ page }) => {
  const audit = auditApiResponses(page)
  await login(page)

  const convName = uniqueFlag('im-conv')
  const convRes = await apiPost(page.request, '/jaxrs/message/assemble/communicate/im/conversation', {
    name: convName,
    type: 'single',
  })
  expect(convRes.status(), 'im conversation create must succeed').toBeLessThan(300)
  const conv = (await convRes.json())?.data
  expect(conv?.id).toBeTruthy()

  const msgText = uniqueFlag('im-msg')
  await page.goto('/app/im')
  await expect(page.getByText(convName)).toBeVisible()
  await page.getByText(convName).click()

  const input = page.getByPlaceholder(/输入消息/).first()
  await input.fill(msgText)
  const sendResp = audit.waitForWrite((r) => /\/im\/msg$/.test(new URL(r.url()).pathname))
  await page.getByRole('button', { name: '发送' }).click()
  const sendBody = await (await sendResp).json()
  expect((sendBody as { type?: string })?.type, 'im msg write must not error').not.toBe('error')
  await expect(page.locator('.msg-content', { hasText: msgText })).not.toHaveCount(0)

  await assertNo404OrServerErrors(audit)
})

test('workflow reject: start a process, then reject the pending task from my-started detail', async ({ page }) => {
  const audit = auditApiResponses(page)
  await login(page)
  const title = uniqueFlag('work-reject')

  await page.goto('/app/process')
  await expect(page.getByRole('heading', { name: '工作流待办' })).toBeVisible()
  const processName = process.env.E2E_PROCESS_NAME ?? 'seed-approval-flow'
  await page.getByRole('button', { name: '发起流程' }).click()
  await page.getByLabel(/流程/).selectOption({ label: processName })
  await page.getByLabel(/标题/).fill(title)
  await page.getByLabel(/申请原因/).fill('E2E reject path')
  const startResp = audit.waitForWrite((r) => /\/service\/processing\/work$/.test(new URL(r.url()).pathname))
  await page.getByRole('button', { name: '发起', exact: true }).click()
  const startBody = await (await startResp).json()
  expect((startBody as { type?: string })?.type, 'work start must succeed').not.toBe('error')

  await expect(page.getByRole('button', { name: '我发起的' })).toHaveClass(/active/)
  await page.getByText(title, { exact: true }).click()

  const rejectResp = audit.waitForWrite((r) => /\/jaxrs\/task\/[^/]+\/reject$/.test(new URL(r.url()).pathname))
  await page.getByLabel(/处理意见/).fill('E2E rejected')
  await page.getByRole('button', { name: '驳回' }).click()
  const rejectBody = await (await rejectResp).json()
  expect((rejectBody as { type?: string })?.type, 'task reject must succeed').not.toBe('error')
  await expect(page.getByText('已驳回')).toBeVisible()

  await assertNo404OrServerErrors(audit)
})
