/**
 * 小程序分享文案（2026-10-23）。
 *
 * # 为什么抽成纯函数
 *
 * 三个页面各写一段内联对象，会有两个后果，都是本仓库反复吃过的：
 *
 *  1. **文案里的数字没人校验**。战斗页若在结算前就拼 `第 N 关 X 星`，
 *     拿到的 `stars/score` 是 0 —— 分享出去是「第 3 关 0 星 · 0 分」，
 *     一个看起来正常、实际是假数据的标题。这种缺陷不会让任何测试红，
 *     因为没人测过分享。
 *  2. **分享路径与页面真实参数脱节**。排行榜分享若写 `?type=stage`，
 *     而接收方页面不读这个参数，点进去看到的仍是战力榜 ——
 *     「分享了一个到不了的位置」。
 *
 * 抽成纯函数之后，这两件事都能在**不需要挂载页面**的情况下断言。
 * 页面只负责把当前状态喂进来。
 */

/** 微信 `onShareAppMessage` 的返回形状（只取本仓库用到的两个字段）。 */
export interface SharePayload {
  title: string
  path: string
}

/** 通用标题。分享标题超过 32 字会被微信截断，这里远低于上限。 */
export const SHARE_TITLE = '来一炮 · 塔防 + 元素反应'

/** 首页分享：没有状态可带，固定文案。 */
export function indexShare(): SharePayload {
  return { title: SHARE_TITLE, path: '/pages/index/index' }
}

/**
 * 排行榜分享。
 *
 * `rank <= 0` 表示「不在榜上」（未进前 N 或没有成绩）——
 * 此时**不能**说「排第 0」，那是个假数字。退回到邀请式文案。
 *
 * `type` 必须原样带进 path，且接收方页面必须真的读它
 * （`rank.vue` 的 `onLoad` 会按 `type` 选中对应 tab）。
 */
export function rankShare(opts: { label: string; rank: number; type: string }): SharePayload {
  const { label, rank, type } = opts
  const title = rank > 0 ? `我在${label}排第 ${rank}` : `${label} · 来看看你排第几`
  return { title, path: `/pages/rank/rank?type=${encodeURIComponent(type)}` }
}

/**
 * 战斗页分享。
 *
 * ⚠️ **未结算时不得出现任何分数/星级**。此时 `stars` 与 `score` 都还没有
 * 权威值（服务端结算才算数），拼进标题就是假数据。
 * 已结算但结算失败（`settled` 为 false）同样走通用文案 ——
 * 失败时 `settleResult.stars` 恒为 0，那更假。
 */
export function battleShare(opts: {
  settled: boolean
  levelId: number
  stars: number
  score: number
}): SharePayload {
  const { settled, levelId, stars, score } = opts
  if (!settled) return { title: SHARE_TITLE, path: '/pages/index/index' }
  const title = `第 ${levelId} 关 ${stars} 星 · ${score} 分，来挑战我`
  return { title, path: '/pages/index/index' }
}
