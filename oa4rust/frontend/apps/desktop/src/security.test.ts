// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const sourceRoot = resolve(import.meta.dirname)

function readSource(relativePath: string): string {
  return readFileSync(resolve(sourceRoot, relativePath), 'utf8')
}

describe('desktop security regressions', () => {
  it('builds shared toast and confirmation messages with textContent', () => {
    const source = readSource('utils/toast.ts')

    expect(source).not.toMatch(/\.innerHTML\s*=/)
    expect(source).toContain('messageEl.textContent = message')
    expect(source).toContain('msgEl.textContent = message')
  })

  it('does not execute process scripts in the browser realm', () => {
    const source = readSource('views/ProcessDesigner.vue')

    expect(source).not.toContain('new Function')
    expect(source).not.toMatch(/\beval\s*\(/)
    expect(source).not.toContain('runSandbox')
    // Must import the sandbox helper and call runInSandbox instead.
    expect(source).toContain("from '../utils/sandbox'")
    expect(source).toContain('runInSandbox')
  })

  it('uses the shared safe confirmation dialog in views', () => {
    const views = [
      'AIChatApp.vue',
      'ConfigDesignerApp.vue',
      'DocumentApp.vue',
      'FormApp.vue',
      'MeetingApp.vue',
      // PortalDesigner 重构后不再提供服务端删除等破坏性操作，改由契约层路径保存，故不在此列
      'RecycleApp.vue',
      'RoleManager.vue',
    ]

    for (const view of views) {
      const source = readSource(`views/${view}`)
      expect(source).toMatch(/import \{[^}]*confirmMsg[^}]*\} from '\.\.\/utils\/toast'/)
      expect(source).not.toMatch(/function confirmMsg\s*\(/)
      expect(source).not.toMatch(/\.innerHTML\s*=/)
    }
  })

  it('does not publish source maps in production builds', () => {
    expect(readSource('../vite.config.ts')).toContain('sourcemap: false')
  })

  // 安全审计回归（2026-10）：以下守护覆盖一次完整审查后落地的加固，防止退化。

  it('never assigns untrusted HTML to innerHTML in the SDK widget renderer', () => {
    // widget.ts 清空容器走 textContent（不解析/注入 HTML）；源码层禁止
    // innerHTML= 任意非空字符串赋值。
    const source = readSource('../../../packages/sdk/src/widget.ts')
    expect(source).toContain('container.textContent = \'\'')
    expect(source).not.toMatch(/container\.innerHTML\s*=\s*['"`][^'"`]+['"`]/)
  })

  it('opens new browser windows with noopener/noreferrer (no window.opener leak)', () => {
    // tab-nabbing 防护：window.open(url,'_blank') 必须带 'noopener,noreferrer'。
    for (const file of [
      '../../../packages/sdk/src/router.ts',
      'views/FileInfoApp.vue',
      'views/FileManager.vue',
    ]) {
      const source = readSource(file)
      // 任何含 '_blank' 的 window.open 调用都必须带 noopener,noreferrer。
      const openCalls = source.match(/window\.open\(([^)]*)\)/g) ?? []
      const blankCalls = openCalls.filter((c) => c.includes("'_blank'"))
      for (const call of blankCalls) {
        expect(call).toMatch(/noopener,noreferrer/)
      }
    }
  })

  it('does not echo auth tokens into UI toasts (BBS forum session)', () => {
    // BBS 论坛登录成功提示不得回显 token 前缀（即使截断也降低离线爆破/日志采集面）。
    const source = readSource('views/BBSForum.vue')
    expect(source).not.toMatch(/token\.slice\(/)
  })

  it('sandbox iframe only accepts script commands from its parent window', () => {
    // 安全审计（2026-10）：sandbox.html 的 message 监听器必须校验 e.source ===
    // window.parent，防止其他同源页/脚本伪造 __oa4rust_sandbox_code 指令注入 eval。
    const source = readSource('../public/sandbox.html')
    expect(source).toContain('e.source !== window.parent')
  })
})
