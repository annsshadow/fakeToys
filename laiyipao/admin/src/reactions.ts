/**
 * 反应 key → 中文名的**唯一**来源。
 *
 * ## 为什么要有这个模块
 *
 * 后台有**两个**页面要显示反应的中文名：
 *   `DashboardView.vue` 的「反应使用分布」图 y 轴
 *   `BattlesView.vue`   的战报详情里的反应 chip
 *
 * 两处一开始各自硬编码了一张 `REACTION_LABEL`。R38 只消除了 dashboard 那张
 * ——因为当时只在**注意到气味的那个页面**上找。
 * 结果 BattlesView 那张留了下来，仍是活的（L221 在用）。
 *
 * > 消除重复时只改了自己看见的那一处，等于没消除。
 *
 * 本模块把「取名 + 查名」收敛到一处，数据来自
 * `GET /admin/reactions`（服务端就是 `domain.AllReactionSpecs()`）。
 *
 * ## 记忆化
 *
 * 两个页面各取一次会变成「每进一次页面请求一次」。
 * 反应表是**静态配置**（7 条，几 KB），一次会话取一次足够。
 *
 * ⚠️ **只缓存成功结果**：失败时把缓存清掉并让错误继续抛。
 * 否则一次网络抖动会把「取不到名字」固化到整个会话 ——
 * 页面从此一直显示原始 key，而且没有任何迹象说明原因。
 */
import { fetchReactions } from './api'

/** key → 中文名。取不到时用原始 key 兜底。 */
export type ReactionLabels = Record<string, string>

let cached: Promise<ReactionLabels> | null = null

/**
 * 取反应名表（同一会话只请求一次）。
 *
 * 失败会**清掉缓存并重新抛出** —— 调用方（视图）据此显示错误，
 * 而下一次进入页面会重新尝试。
 */
export function fetchReactionLabels(): Promise<ReactionLabels> {
  if (!cached) {
    cached = fetchReactions()
      .then((specs) => Object.fromEntries(specs.map((r) => [r.key, r.name])))
      .catch((e) => {
        // 失败不缓存：否则一次抖动会把这个会话永久钉死在「取不到名字」上
        cached = null
        throw e
      })
  }
  return cached
}

/**
 * 查一个反应的中文名。
 *
 * 兜底返回**原始 key** —— 不是空串、不是 `undefined`。
 * 覆盖的场景：后台连的是旧版服务端、响应里没有这个新反应。
 */
export function labelOfReaction(labels: ReactionLabels, key: string): string {
  return labels[key] || key
}

/** 仅供测试：清掉模块级缓存，避免用例之间互相污染。 */
export function __resetReactionLabelCache(): void {
  cached = null
}
