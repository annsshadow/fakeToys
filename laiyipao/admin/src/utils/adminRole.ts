import { computed, inject, readonly, ref, type ComputedRef, type Ref } from 'vue'

/**
 * 后台当前管理员身份（P5-6）。
 *
 * 为什么需要它：`GET /admin/me` 返回 `{id, username, role}`，
 * 而此前 `adminMe()` **从未被调用** —— 于是 `role` 从未被读取，
 * readonly 账号看得见全部写按钮，只有点下去吃一个 403 才知道自己不能写。
 * 那个 403 出现在**动作已经发生之后**（比如「已封禁」之后才发现没权限）。
 *
 * 用法：
 * ```ts
 * const { isReadonly } = useAdminRole()
 * ```
 *
 * ⚠️ 身份**取不到**时 `isReadonly` 为 false（按可写处理）：
 * 宁可多显示几个按钮，也不要让一个正常的可写账号什么都点不了。
 * 服务端仍然会对写操作做鉴权 —— 这里只是**界面**的提前提示。
 */
export const ADMIN_ROLE_KEY = Symbol('laiyipao-admin-role')

export interface AdminRole {
  role: Readonly<Ref<string>>
  name: Readonly<Ref<string>>
  isReadonly: ComputedRef<boolean>
}

export function useAdminRole(): AdminRole {
  const injected = inject<AdminRole | null>(ADMIN_ROLE_KEY, null)
  if (injected) return injected
  // 没有 Provider（例如视图被单独挂载测试）→ 给一个「可写」的缺省，
  // 这样视图不会因为缺少上下文而崩。
  const role = ref('')
  const name = ref('')
  return { role: readonly(role), name: readonly(name), isReadonly: computed(() => false) }
}
