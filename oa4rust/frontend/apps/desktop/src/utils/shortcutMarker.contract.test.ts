// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R207 wiring contract: AppShell advertises ⌘K on its topbar search box, so
 * the box must carry a stable marker and the desktop entry must install the
 * global shortcut before mounting (same lifecycle as other a11y installers).
 */
import fs from 'node:fs'
import path from 'node:path'
import { describe, expect, test } from 'vitest'

const shellSource = fs.readFileSync(
  path.resolve(__dirname, '../../../../packages/ui/src/components/AppShell.vue'),
  'utf-8',
)
const mainSource = fs.readFileSync(path.resolve(__dirname, '../main.ts'), 'utf-8')

describe('R207 global search shortcut wiring', () => {
  test('AppShell topbar search input carries the global-search marker', () => {
    expect(shellSource).toContain('data-oa-global-search')
  })

  test('main.ts installs the global shortcut before mounting the app', () => {
    expect(mainSource).toContain("import { installGlobalSearchShortcut } from './utils/globalSearchShortcut'")
    const installAt = mainSource.indexOf('installGlobalSearchShortcut()')
    const mountAt = mainSource.indexOf("app.mount('#o2-app-root')")
    expect(installAt).toBeGreaterThan(-1)
    expect(mountAt).toBeGreaterThan(installAt)
  })
})
