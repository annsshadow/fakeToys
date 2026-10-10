// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * sandbox.ts — iframe-based isolated script execution.
 *
 * Scripts run inside a sandboxed iframe with:
 *   - sandbox="allow-scripts" (no allow-same-origin → no DOM / localStorage access)
 *   - postMessage communication channel
 *
 * This replaces the previous `new Function()` path which ran code in the
 * page's main realm (full access to window, DOM, localStorage, network).
 */

interface SandboxResult {
  ok: boolean
  output?: string
  error?: string
  executionTimeMs: number
}

let iframe: HTMLIFrameElement | null = null
let iframeLoaded = false
let pendingCallback: ((r: SandboxResult) => void) | null = null

/** Lazy-create the sandbox iframe on first use. */
function getOrCreateFrame(): HTMLIFrameElement {
  if (!iframe) {
    iframe = document.createElement('iframe')
    // Strict sandbox: scripts run, but same-origin access is blocked.
    iframe.setAttribute('sandbox', 'allow-scripts')
    // Load the bootstrap page that registers the __oa4rust_sandbox_code listener.
    // Without this the iframe stays on about:blank and the code is never executed,
    // so every runInSandbox call 100% times out.
    iframe.src = `${import.meta.env.BASE_URL}sandbox.html`
    iframe.style.cssText = 'position:absolute;left:-9999px;top:-9999px;width:1px;height:1px;border:0'
    document.body.appendChild(iframe)
    iframe.addEventListener('load', () => {
      iframeLoaded = true
    })

    // Listen for result messages from the sandbox.
    window.addEventListener('message', onSandboxMessage)
  }
  return iframe
}

function onSandboxMessage(e: MessageEvent): void {
  if (e.source !== iframe?.contentWindow) return
  const data = e.data
  if (!data || typeof data !== 'object' || data.__oa4rust_sandbox_result !== true) return

  const callback = pendingCallback
  pendingCallback = null

  if (callback) {
    callback({
      ok: data.ok,
      output: data.output ?? undefined,
      error: data.error ?? undefined,
      executionTimeMs: data.executionTimeMs ?? 0,
    })
  }
}

/**
 * Execute a snippet of JavaScript in the sandbox iframe.
 *
 * @param code   – JavaScript code to run (must not reference `window`, `document`,
 *                 `localStorage`, `fetch` etc. — those are unavailable in the sandbox).
 * @param timeoutMs – maximum execution time before abort (default 5000ms).
 */
export function runInSandbox(code: string, timeoutMs = 5000): Promise<SandboxResult> {
  return new Promise<SandboxResult>((resolve) => {
    const frame = getOrCreateFrame()

    pendingCallback = resolve

    // The sandboxed iframe has an opaque origin (sandbox="allow-scripts" without
    // allow-same-origin), so it must be posted with targetOrigin '*' — a specific
    // origin would throw SecurityError. Post only after the bootstrap page has
    // loaded, otherwise its __oa4rust_sandbox_code listener isn't registered yet
    // and the code would be silently dropped.
    const post = () => {
      frame.contentWindow?.postMessage({ __oa4rust_sandbox_code: code }, '*')
    }
    if (iframeLoaded) post()
    else frame.addEventListener('load', post, { once: true })

    // Force-gc the iframe after a while to prevent memory leak if the user
    // navigates away without the message completing.
    setTimeout(() => {
      if (pendingCallback) {
        const cb = pendingCallback
        pendingCallback = null
        cb({
          ok: false,
          error: `sandbox timed out after ${timeoutMs}ms`,
          executionTimeMs: timeoutMs,
        })
      }
    }, timeoutMs)
  })
}

/**
 * Bootstrap the sandbox iframe content once (called internally via srcdoc).
 */
export function bootstrapSandboxFrame(srcdoc: string): void {
  if (iframe) {
    iframe.srcdoc = srcdoc
  }
}

/** Destroy the sandbox iframe (call on unmount to free resources). */
export function destroySandbox(): void {
  if (iframe) {
    window.removeEventListener('message', onSandboxMessage)
    iframe.remove()
    iframe = null
    iframeLoaded = false
  }
}
