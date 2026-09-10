import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const sourceRoot = resolve(import.meta.dirname)

function readSource(relativePath: string): string {
  return readFileSync(resolve(sourceRoot, relativePath), 'utf8')
}

describe('browser authentication regressions', () => {
  it('keeps browser sessions cookie-only', () => {
    const apiSource = readSource('api.ts')
    const sessionSource = readSource('session.ts')

    expect(apiSource).toContain("credentials: 'include'")
    expect(apiSource).not.toMatch(/Authorization|Bearer|\.getItem\(|\.setItem\(/)
    expect(sessionSource).not.toMatch(/\.getItem\(|\.setItem\(|Authorization|Bearer/)
    expect(sessionSource).toContain('{ credential, password, captchaId, captchaAnswer }')
  })

  it('does not consume tokens from OAuth URLs or response JSON', () => {
    const oauthSource = readSource('../../ui/src/views/OAuthCallback.vue')
    const ssoSource = readSource('../../../apps/desktop/src/views/SSO.vue')

    expect(oauthSource).not.toMatch(/query\.token|hash|resp\.data|response\.json|setSession/)
    expect(oauthSource).toContain('await session.init(true)')
    expect(ssoSource).not.toMatch(/query\.token|response\.json|setSession/)
    expect(ssoSource).toContain('await session.init(true)')
  })
})
