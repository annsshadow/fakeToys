// @vitest-environment jsdom
import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import { createPinia } from 'pinia'
import { createServer } from 'vite'
import { describe, expect, it } from 'vitest'
import { createSSRApp, h } from 'vue'
import { renderToString } from 'vue/server-renderer'

const file = resolve(import.meta.dirname, 'LoginScreen.vue')

async function renderLogin() {
  const server = await createServer({
    configFile: false,
    logLevel: 'silent',
    plugins: [vue()],
    resolve: {
      alias: {
        '@oa4rust/sdk': resolve(import.meta.dirname, '../../../sdk/src'),
      },
    },
    server: { middlewareMode: true, watch: { ignored: ['**'] } },
  })
  try {
    const module = await server.ssrLoadModule(file)
    const app = createSSRApp({ render: () => h(module.default) })
    app.use(createPinia())
    app.config.warnHandler = () => {}
    const html = await renderToString(app)
    const host = document.createElement('div')
    host.innerHTML = html
    return host
  } finally {
    await server.close()
  }
}

describe('LoginScreen 表单标签可访问性契约', () => {
  it('每个可见输入框都有可访问名称，label 与控件程序化关联', async () => {
    const host = await renderLogin()
    const inputs = Array.from(host.querySelectorAll('input'))
    expect(inputs.length).toBeGreaterThan(0)

    for (const input of inputs) {
      const fromFor = input.id ? host.querySelector(`label[for="${input.id}"]`) : null
      const fromAria = input.getAttribute('aria-label') || input.getAttribute('aria-labelledby')
      expect(
        fromFor || fromAria,
        `输入框 ${input.getAttribute('autocomplete') || input.getAttribute('placeholder')} 应有 label 或 aria 标签`,
      ).toBeTruthy()
    }

    const associations = Array.from(host.querySelectorAll('label[for]'))
    expect(associations.length).toBeGreaterThan(0)
    for (const label of associations) {
      const target = host.querySelector(`#${label.getAttribute('for')}`)
      expect(target, `label 应指向存在的控件`).toBeTruthy()
    }
  })
})
