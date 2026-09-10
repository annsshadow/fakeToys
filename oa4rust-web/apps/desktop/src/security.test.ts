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
      'PortalDesigner.vue',
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
})
