/**
 * `index.html` 的 CSP（纵深防御）守卫。
 *
 * # 为什么需要它
 *
 * 这条 CSP **不拦任何已知攻击**：全量核查过 admin 源码里没有
 * v-html / innerHTML / eval / new Function（0 命中）。它防的是
 * 「将来某一天有人引入第一个 XSS sink」—— 而 admin 令牌就存在
 * localStorage 里，一次脚本执行 = 令牌外带。
 *
 * 也就是说：**删掉它不产生任何症状**。页面照常打开、测试照常全绿、
 * 功能一个不少。这正是本仓库反复记的那类改动（见 bundle_size.test.ts
 * 里 components.d.ts 那条的同族）：只能靠守卫，不能靠人记得。
 *
 * # 判据要精确，不能靠「文件里有 CSP 这几个字」
 *
 * 这里逐项断言语义，而不是断言「出现了 Content-Security-Policy」——
 * 后者会被一行注释满足，那等于没有守卫。
 */
import { describe, expect, it } from 'vitest'
import { readFileSync, existsSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const htmlPath = join(root, 'index.html')
if (!existsSync(htmlPath)) {
  throw new Error(`守卫找不到 index.html（工作目录 ${root}）—— 文件被改名或挪走了？`)
}
const html = readFileSync(htmlPath, 'utf8')

/**
 * 剥掉 HTML 注释。
 *
 * ⚠️ 必须先剥：index.html 里那段解释 CSP 的注释**本身**就写着
 * `Content-Security-Policy` 与 `<meta>` 这两个词。不剥的话，
 * 「meta 标签被删掉」这一整类改动有可能被注释里的字面量满足 ——
 * 守卫于是变成一条永远绿的装饰。
 * （这与 bundle_size.test.ts 记的是同一个坑，只是方向相反：
 * 那里是注释让**正确的代码**判失败，这里是注释让**错误的代码**判通过。
 * 两者都会激励下一个人删掉守卫。）
 */
const htmlNoComments = html.replace(/<!--[\s\S]*?-->/g, '')

/** 取出 CSP meta 的 content 值（没有则返回 null）。 */
function cspContent(): string | null {
  // ⚠️ 不能一步到位写 `content=["']([^"']+)["']`：CSP 的值里**含单引号**
  // （`'self'` / `'unsafe-inline'`），那会把值截断在第一个关键字处。
  // 第一版就是这么写的 —— 它把 content 读成 "default-src "，
  // 于是 script-src / style-src 两条断言全部误报成「CSP 里没有」。
  // 先定位整段 meta 标签，再按**引号种类**取属性值。
  const tag = htmlNoComments.match(/<meta[^>]*Content-Security-Policy[^>]*>/i)
  if (!tag) return null
  const cm = tag[0].match(/\bcontent\s*=\s*("([^"]*)"|'([^']*)')/i)
  if (!cm) return null
  return cm[2] ?? cm[3]
}

describe('守卫自身的两个前置条件', () => {
  it('剥注释确实生效：只存在于注释里的字面量不应留在剥离结果里', () => {
    // 反向验证剥离器不是空操作 —— 否则「meta 被删」这类改动
    // 会被注释里的同一批字面量满足，守卫变成装饰。
    //
    // ⚠️ 标记词刻意选 `X-Frame-Options`（只出现在注释里），
    // **不选** `frame-ancestors`：后者可能就是上面那条断言在查的东西，
    // 用它当标记会让「CSP 里多了 frame-ancestors」同时打红两条测试，
    // 而其中一条的失败信息会把责任指向剥离器。
    expect(html, '前置条件：注释里应当提到 X-Frame-Options').toContain('X-Frame-Options')
    expect(htmlNoComments, '剥离器没生效：注释里的 X-Frame-Options 还在').not.toContain('X-Frame-Options')
  })

  it('剥离器没有吃掉整份文档', () => {
    expect(htmlNoComments).toContain('<meta charset')
    expect(htmlNoComments).toContain('id="app"')
  })
})

describe('admin index.html 的 CSP', () => {
  it('存在真正的 CSP meta（而不是只写在注释里）', () => {
    // ⚠️ 这一条的存在理由：本仓库刚修过一处「注释声称做了、代码里没有」的
    // 缺陷（client.ts 的 401 处理）。注释是最容易让所有人以为已经做了的形态。
    expect(
      cspContent(),
      'index.html 里没有 http-equiv="Content-Security-Policy" 的 meta —— ' +
        '只有注释提到 CSP 不算做了这件事。',
    ).not.toBeNull()
  })

  it("script-src 必须是 'self'，且不得含 unsafe-inline / unsafe-eval", () => {
    const csp = cspContent() ?? ''
    const m = csp.match(/script-src([^;]*)/)
    expect(m, 'CSP 里没有 script-src —— default-src 会兜住，但不写清楚等于把这条守卫交给别人改').not.toBeNull()
    const scriptSrc = m![1]
    expect(scriptSrc, `script-src 应是 'self'，实际 ${scriptSrc.trim()}`).toContain("'self'")
    // 这两个关键字会让 CSP 对脚本注入完全失去意义：
    // unsafe-inline 允许内联脚本（XSS 最常见的落点），
    // unsafe-eval 允许 eval/new Function（另一条代码执行路径）。
    expect(
      scriptSrc,
      `script-src 里出现了 unsafe-* —— 这条 CSP 就白写了。实际：${scriptSrc.trim()}`,
    ).not.toMatch(/unsafe-inline|unsafe-eval/)
  })

  it("default-src 与 style-src 存在且 style-src 允许 inline（构建产物需要）", () => {
    const csp = cspContent() ?? ''
    expect(csp, "CSP 里没有 default-src 'self'").toMatch(/default-src\s+'self'/)
    // ⚠️ style-src 必须留 'unsafe-inline'：Vite 在**开发模式**下用
    // `document.createElement('style')` 注入样式，没有它 dev server 页面会裸奔。
    // 这条断言是为了防止「为了好看」把它删掉 —— 那会以「开发环境样式全丢」
    // 的形式在别人机器上炸，而不是在这里红。
    expect(csp, 'CSP 里没有 style-src').toMatch(/style-src/)
    expect(csp, "style-src 丢了 'unsafe-inline' —— dev server 的样式注入会被拦掉").toMatch(
      /style-src[^;]*'unsafe-inline'/,
    )
  })

  it('⚠️ 不得在 meta 里写 frame-ancestors（它会被浏览器丢弃）', () => {
    // 这条是本文件**最重要**的一条，因为它防的不是「没做」而是「做错了」。
    //
    // CSP2/CSP3 规范明确把 frame-ancestors 从 meta 传递的指令集里移除：
    // 写在 <meta> 里会被**整个丢弃**，控制台只有一行 warning。
    // 于是它比不写更糟 —— 下一个人看到 index.html 里有 frame-ancestors，
    // 会认为点击劫持已被守住，从而不去配反代的 X-Frame-Options。
    //
    // 点击劫持是本项目唯一「不需要任何服务端漏洞」的路径：
    // 诱导已登录的运营点一下 → 那一击就是合法的、带真实令牌的写请求。
    const csp = cspContent() ?? ''
    expect(
      csp,
      'CSP meta 里出现了 frame-ancestors —— 规范要求浏览器在 meta 场景下**丢弃**它，' +
        '写了不会生效，但会让后来的人以为点击劫持已经被守住。' +
        '它只能由部署侧响应头下发：X-Frame-Options: DENY ' +
        "或 Content-Security-Policy: frame-ancestors 'none'。",
    ).not.toContain('frame-ancestors')
  })

  it('部署前置条件写在 index.html 里（不能只活在某次提交信息里）', () => {
    // 纯注释断言，但值得：上面那条要求「不写 frame-ancestors」，
    // 如果不留下「那该在哪写」的说明，下一次有人加固 CSP 时
    // 最自然的动作就是把 frame-ancestors 加回 meta —— 然后又白写一遍。
    expect(html, 'index.html 里没有说明点击劫持要靠响应头解决').toContain('X-Frame-Options')
  })
})
