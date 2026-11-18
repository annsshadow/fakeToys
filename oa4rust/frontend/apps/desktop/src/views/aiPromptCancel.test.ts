// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AIChatApp.vue'), 'utf8')

/** Extract one top-level async function from the SFC's <script setup> block. */
function extractFunction(name: string): string {
  const start = source.indexOf(`async function ${name}(`)
  expect(start, `missing ${name} in AIChatApp.vue`).toBeGreaterThan(-1)
  const end = source.indexOf('\n}\n', start)
  expect(end, `unclosed ${name}`).toBeGreaterThan(start)
  // Strip the two TypeScript annotations used in these helpers so the body can
  // be exercised as plain JavaScript in a stubbed environment.
  return source.slice(start, end + 3).replace(/:\s*(string|any)\b/g, '')
}

type ApiCalls = { method: string; url: string }[]

function loadHandlers(promptResult: string | null) {
  const calls: ApiCalls = []
  const api = {
    get: (url: string) => {
      calls.push({ method: 'get', url })
      return Promise.resolve({})
    },
    post: (url: string) => {
      calls.push({ method: 'post', url })
      return Promise.resolve({})
    },
    delete: (url: string) => {
      calls.push({ method: 'delete', url })
      return Promise.resolve({})
    },
  }
  const toast = { success: () => {}, error: () => {} }
  const confirmMsg = () => Promise.resolve(true)
  const prompt = () => promptResult
  const factory = new Function(
    'api',
    'confirmMsg',
    'toast',
    'prompt',
    `function aiPrompt(label) { return prompt(label, '') }
${extractFunction('aiWrite')}
${extractFunction('aiMore')}
${extractFunction('aiMore2')}
return { aiWrite, aiMore, aiMore2 }`,
  )
  const handlers = factory(api, confirmMsg, toast, prompt) as {
    aiWrite: (op: string) => Promise<void>
    aiMore: (op: string) => Promise<void>
    aiMore2: (op: string) => Promise<void>
  }
  return { handlers, calls }
}

/**
 * R197: the AI debug panel used bare prompt() for every destructive/write
 * action and swallowed cancellation with `|| ''`, sending requests with empty
 * IDs/flags. A cancelled prompt must abort the operation completely.
 */
describe('AIChat debug panel: cancelled prompt aborts the operation (R197)', () => {
  const cases: { handler: 'aiWrite' | 'aiMore' | 'aiMore2'; ops: string[] }[] = [
    {
      handler: 'aiWrite',
      ops: ['modelCreate', 'modelUpdate', 'mcpCreate', 'mcpUpdate', 'mcpDelete', 'annSave', 'annDelete', 'chatDelete'],
    },
    { handler: 'aiMore', ops: ['indexDelete', 'fileDownload', 'fileScale', 'fileDelete'] },
    { handler: 'aiMore2', ops: ['mcpUpdate', 'mcpDelete', 'modelDelete', 'chatDelete'] },
  ]

  for (const { handler, ops } of cases) {
    for (const op of ops) {
      it(`${handler}(${op}) sends no request when the prompt is cancelled`, async () => {
        const { handlers, calls } = loadHandlers(null)
        await handlers[handler](op)
        expect(calls).toEqual([])
      })
    }
  }

  it('still executes a confirmed operation with the typed value', async () => {
    const { handlers, calls } = loadHandlers('typed-id')
    await handlers.aiMore('fileDownload')
    expect(calls).toEqual([{ method: 'get', url: '/api/ai/file/typed-id/download' }])
  })
})
