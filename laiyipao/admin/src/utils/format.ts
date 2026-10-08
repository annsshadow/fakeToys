/**
 * 后台时间戳显示。
 *
 * ⚠️ 第 124 轮：服务端 `time.Time` 由 pgx 按 **UTC** 解码、
 * JSON 下发成 `...Z`（RFC3339）。修前各视图的本地 `fmtTime` 只做
 * `s.replace('T',' ').slice(0,19)`——把服务端的 UTC 墙钟**原样**显示，
 * 运营在 UTC+8 部署下看到的「最后登录 / 审计时间」整体偏 8 小时。
 * 这里统一改成：解析成 Date，渲染成**查看者本地时区**的 `YYYY-MM-DD HH:mm:ss`。
 */
export function fmtTime(s: string): string {
  if (!s) return '—'
  const d = new Date(s)
  // 解析失败（非 RFC3339）：退回旧的纯展示，不崩
  if (Number.isNaN(d.getTime())) return s.replace('T', ' ').slice(0, 19)
  const p = (n: number) => String(n).padStart(2, '0')
  return (
    `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ` +
    `${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
  )
}
