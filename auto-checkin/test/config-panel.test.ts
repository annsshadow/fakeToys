// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { test } from 'node:test'
import assert from 'node:assert/strict'
import { mkdtempSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { loadConfig } from '../src/config.js'

// 第 39 轮：config.json 的 panel 段不再被静默丢弃。
// 改前 mergeConfig 漏掉 o.panel、loadConfig 用默认值整段覆盖 ——
// `{"panel":{"port":9999}}` 永远不生效，面板仍起 8787。

function tempConfig(panel?: unknown): string {
  const dir = mkdtempSync(join(tmpdir(), 'ac-panel-'))
  const path = join(dir, 'config.json')
  writeFileSync(path, JSON.stringify({ panel }), 'utf-8')
  return path
}

const savedHost = process.env.PANEL_HOST
const savedPort = process.env.PANEL_PORT
const savedToken = process.env.PANEL_TOKEN
test.after(() => {
  if (savedHost === undefined) delete process.env.PANEL_HOST; else process.env.PANEL_HOST = savedHost
  if (savedPort === undefined) delete process.env.PANEL_PORT; else process.env.PANEL_PORT = savedPort
  if (savedToken === undefined) delete process.env.PANEL_TOKEN; else process.env.PANEL_TOKEN = savedToken
})

test('config.json 的 panel.port 生效（无环境变量时）', () => {
  delete process.env.PANEL_HOST
  delete process.env.PANEL_PORT
  delete process.env.PANEL_TOKEN
  const cfg = loadConfig(tempConfig({ port: 9999 }))
  assert.equal(cfg.panel.port, 9999)
})

test('config.json 的 panel.host 生效', () => {
  delete process.env.PANEL_HOST
  delete process.env.PANEL_PORT
  delete process.env.PANEL_TOKEN
  const cfg = loadConfig(tempConfig({ host: '0.0.0.0' }))
  assert.equal(cfg.panel.host, '0.0.0.0')
})

test('环境变量逐项覆盖 config.json 的面板值', () => {
  delete process.env.PANEL_HOST
  process.env.PANEL_PORT = '7777'
  delete process.env.PANEL_TOKEN
  const cfg = loadConfig(tempConfig({ port: 9999, host: '127.0.0.1' }))
  assert.equal(cfg.panel.port, 7777)   // 环境变量覆盖
  assert.equal(cfg.panel.host, '127.0.0.1')  // config.json 值保留
})

test('无 config.json 时回退默认面板配置', () => {
  delete process.env.PANEL_HOST
  delete process.env.PANEL_PORT
  delete process.env.PANEL_TOKEN
  const cfg = loadConfig(join(tmpdir(), 'no-such-config.json'))
  assert.equal(cfg.panel.port, 8787)
})
