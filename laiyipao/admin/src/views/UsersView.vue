<script setup lang="ts">
import {onMounted, ref, computed, onActivated} from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchUsers,
  fetchUserDetail,
  banUser,
  unbanUser,
  grantUser,
  type AdminUserDetailResp,
  type AdminUserView,
} from '@/api'
import { useAdminRole } from '@/utils/adminRole'
import { fmtTime, isCancel } from '@/utils/format'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '没有匹配的用户'
const loading = ref(false)
// 2026-10-21（P5-6）：readonly 账号看不到写按钮。
const { isReadonly } = useAdminRole()
const users = ref<AdminUserView[]>([])

/**
 * 玩家详情抽屉（2026-10-21 P5-7）。
 *
 * 此前后台**只能看到用户列表的一行**。运营要回答的四个问题 ——
 * 「他最近打了什么 / 钱去哪了 / 构筑是什么 / 被验真验出不匹配了吗」——
 * 全都要去别处拼 URL。
 *
 * 一个**不能看的详情**比没有更糟：它让运营以为「没有信息」= 「没有问题」。
 */
const detailDrawer = ref(false)
const detailLoading = ref(false)
const detail = ref<AdminUserDetailResp | null>(null)

/**
 * 详情请求的**序号**（2026-10-21 二轮审计 P2-9）。
 *
 * 慢网下点 A 的详情 → 立刻点 B 的详情 → A 的响应后到 →
 * `detail.value` 被写成 **A 的数据**，而抽屉在运营心智里是 B。
 * 运营据此下判断就是错数据。
 *
 * 序号守卫：只接受**最新一次**请求的响应。
 */
let detailReqSeq = 0

async function openDetail(u: AdminUserView) {
  const seq = ++detailReqSeq
  detailDrawer.value = true
  detailLoading.value = true
  detail.value = null
  try {
    const res = await fetchUserDetail(u.id)
    if (seq !== detailReqSeq) return // 已经有更新的请求了
    detail.value = res
  } catch (e) {
    if (seq !== detailReqSeq) return
    ElMessage.error(`玩家详情加载失败：${(e as Error).message}`)
  } finally {
    if (seq === detailReqSeq) detailLoading.value = false
  }
}
const total = ref(0)
const keyword = ref('')
const limit = ref(50)
const offset = ref(0)

const CURRENCY_LABEL: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
}

const page = computed(() => Math.floor(offset.value / limit.value) + 1)
const pageCount = computed(() => Math.max(1, Math.ceil(total.value / limit.value)))

/** status: 1=正常 2=封禁（与迁移中的 CHECK 约束一致） */
function statusOf(u: AdminUserView): { text: string; type: 'success' | 'danger' } {
  return u.status === 1 ? { text: '正常', type: 'success' } : { text: '封禁', type: 'danger' }
}

/** 高战力但零战斗记录的用户是刷分嫌疑，列表里直接标出来。 */
function suspicious(u: AdminUserView): boolean {
  return u.power > 5000 && u.max_stage <= 1
}

/**
 * 验真信号的三态。
 *
 * ⚠️ **关键：0/0 不是「干净」，是「未知」。**
 *
 * 背景：`replay_verifications` 此前全项目只被一个 COUNT 查询读过 ——
 * 不匹配被记录、被统计，但没有后果，运营在用户列表里看不到任何验真信号。
 * 也就是说看板上的「验真一致率」是个**不可执行的数字**：
 * 看到「1000 场验真、10 场不匹配」却不知道该封谁。
 *
 * 所以这里刻意分三态，而不是简单地「mismatched > 0 就标红」：
 *
 *   0 / 0  →  未验真    = 未知（灰，不标红）
 *   n / 0  →  验过且一致 = 干净（绿）
 *   n / m  →  m > 0     = 可疑（红）
 *
 * 把「未验真」和「验过且一致」都显示成 0 的话，
 * **最不可信的那批用户看起来最干净**。
 */
type VerifyState = 'unknown' | 'clean' | 'suspect'

function verifyState(u: AdminUserView): VerifyState {
  if ((u.verify_mismatched ?? 0) > 0) return 'suspect'
  if ((u.verify_checked ?? 0) === 0) return 'unknown'
  return 'clean'
}

function verifyTag(u: AdminUserView): { text: string; type: 'info' | 'success' | 'danger' } {
  const s = verifyState(u)
  if (s === 'suspect') return { text: `不匹配 ${u.verify_mismatched}/${u.verify_checked}`, type: 'danger' }
  if (s === 'clean') return { text: `一致 ${u.verify_checked}`, type: 'success' }
  return { text: '未验真', type: 'info' }
}

async function load() {
  loading.value = true
  try {
    const res = await fetchUsers({ keyword: keyword.value || undefined, limit: limit.value, offset: offset.value })
    users.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`用户加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

function prevPage() {
  if (offset.value <= 0) return
  offset.value = Math.max(0, offset.value - limit.value)
  load()
}

function nextPage() {
  if (offset.value + limit.value >= total.value) return
  offset.value += limit.value
  load()
}

async function onBan(u: AdminUserView) {
  try {
    const { value } = await ElMessageBox.prompt(`封禁「${u.nickname}」的原因`, '封禁用户', {
      inputType: 'textarea',
      inputPlaceholder: '会记入审计日志',
    })
    if (!value.trim()) return
    const res = await banUser(u.id, value.trim())
    Object.assign(u, res.user)
    ElMessage.success('已封禁')
  } catch (e) {
    // ⚠️ 「取消」与「失败」必须分开（2026-10-21 二轮审计 P2-2）：
    // 此前一个 `catch { /* 取消 */ }` 把两者都吞了 —— readonly 账号点「封禁」
    // 得到 403，**屏幕上什么都没有发生**（连 toast 都没有）。
    if (isCancel(e)) return
    ElMessage.error(`封禁失败：${(e as Error).message}`)
  }
}

/**
 * 解封。
 *
 * 2026-10-21（P5-2）：此前**一次点击直接生效**、无任何确认 ——
 * 而封禁是有确认的（要填理由）。这个不对称让「手滑」的成本完全不对称：
 * 封禁了要再走一次流程才能解开，解封了却只需要点错一下。
 */
async function onUnban(u: AdminUserView) {
  try {
    await ElMessageBox.confirm(
      `解封「${u.nickname}」？该玩家将立即恢复登录与游戏权限。`,
      '解封用户',
      { type: 'warning', confirmButtonText: '解封' },
    )
  } catch {
    return // 取消
  }
  try {
    const res = await unbanUser(u.id)
    Object.assign(u, res.user)
    ElMessage.success('已解封')
  } catch (e) {
    ElMessage.error(`解封失败：${(e as Error).message}`)
  }
}

async function onGrant(u: AdminUserView) {
  try {
    const { value } = await ElMessageBox.prompt(
      `给「${u.nickname}」发放资源。格式：coin 10000 / gem 500 / energy 200 / keys 5（货币名为 coin/gem/energy/keys；数量为负 = 回收）`,
      '发放资源',
      { inputPlaceholder: 'coin 10000' },
    )
    //
    // ⚠️ 第 109 轮：允许负数（回收场景），但文案必须说「回收」而不是「发放」。
    // 服务端同轮把符号写进了审计事件（grant_currency / revoke_currency）
    // 与钱包流水 reason（admin_grant / admin_revoke），
    // UI 是唯一还知道运营「实际做了哪个动作」的界面，不能把回收显示成发放。
    const m = value.trim().match(/^(\S+)\s+(-?\d+)$/)
    if (!m) {
      ElMessage.error('格式错误，应为「货币名 数量」（负数为回收）')
      return
    }
    const currency = m[1]
    if (!(currency in CURRENCY_LABEL)) {
      ElMessage.error(`未知货币：${currency}`)
      return
    }
    const amount = Number(m[2])

    // 2026-10-21（P5-2）：**二次确认**。
    //
    // 此前发/回收资源只有一次输入弹窗，输入完直接提交 ——
    // 一次手滑（`coin -1000000`）会直接回收玩家 100 万金币，
    // 而唯一的反馈是一条几秒后就消失的 toast。
    //
    // 确认框里必须写清**净变化**（回收多少 / 发放多少），
    // 让运营在点确认之前再看一眼那个负号。
    const isRevoke = amount < 0
    try {
      await ElMessageBox.confirm(
        isRevoke
          ? `回收「${u.nickname}」的 ${CURRENCY_LABEL[currency]} ${-amount}？` +
              `该玩家的${CURRENCY_LABEL[currency]}余额会被扣掉这么多，且不可撤销。`
          : `给「${u.nickname}」发放 ${CURRENCY_LABEL[currency]} ${amount}？`,
        isRevoke ? '确认回收资源' : '确认发放资源',
        { type: isRevoke ? 'warning' : 'info' },
      )
    } catch {
      return // 取消
    }

    const res = await grantUser(u.id, currency, amount)
    u.coin = res.wallet.coin
    u.gem = res.wallet.gem
    ElMessage.success(
      amount < 0
        ? `已回收 ${CURRENCY_LABEL[currency]} ${-amount}`
        : `已发放 ${CURRENCY_LABEL[currency]} ${amount}`,
    )
  } catch (e) {
    // 「取消」与「失败」分开（2026-10-21 二轮审计 P2-2）。
    if (isCancel(e)) return
    ElMessage.error(`发放失败：${(e as Error).message}`)
  }
}

onMounted(load)

/**
 * 2026-10-21（P5-9）：缓存视图重新可见时刷新数据。
 *
 * 布局用 `keep-alive`（max=4）缓存视图，而每个视图此前**只在
 * `onMounted` 加载一次** —— 于是「封禁一个用户 → 切到别的页 →
 * 切回来」看到的还是**旧数据**，运营会以为操作没生效。
 *
 * `onActivated` 是 keep-alive 视图重新可见时的钩子。
 */
/**
   * `onActivated` 在**首次挂载**时也会触发（vue 给 keep-alive 的子组件
   * 打标记并把钩子排进 post-render 队列），而 `onMounted` 已经加载过一次 ——
   * 两者都留着 = 每进一次页面发**两份**请求，还留下一个
   * 「后到的先渲染」的乱序窗口。
   *
   * 所以：首次的 `onActivated` 跳过，之后的才刷新。
   */
  let activatedOnce = false
onActivated(() => {
    if (!activatedOnce) {
    activatedOnce = true
    return
  }
load()
})

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 它不能直接传给形参是 `AdminUser` 的函数（TS2345），所以在调用点集中断言一次。
 *
 * cast 编译后被擦除，**运行时行为与之前逐字节相同**。
 *
 * ⚠️ 值得记一笔的是：这些调用此前**根本没有被类型检查**。
 * `components.d.ts` 不存在时 `el-table` 是未知组件、插槽行是 `any`，
 * 于是 22 处调用全部静默通过 —— 也就是说，
 * 「参数名拼错」或「形参类型改窄」这类错误在过去是**查不出来**的。
 * 断言让它们重新受检；但它断言的是**形状**，
 * 字段名本身拼错仍要靠运行时的 `undefined` 暴露。
 */
const asRow = (r: unknown) => r as AdminUserView
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">用户管理</div>
    <p class="page-subtitle">
      全部改动（封禁 / 发资源）都会写入审计日志。
      「战力高但停在第 1 关」是需要人工看一眼的信号 —— 分数正确性由 I-6 哈希兜底，但资源异常仍要查。
    </p>

    <el-card shadow="never">
      <div class="toolbar">
        <el-input
          v-model="keyword"
          placeholder="按昵称 / ID 搜索"
          clearable
          @clear="((offset = 0), load())"
          style="width: 220px"
          @keyup.enter="((offset = 0), load())"
        />
        <el-button @click="((offset = 0), load())">查询</el-button>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ total }} 名用户 · 第 {{ page }} / {{ pageCount }} 页</span>
        <el-button size="small" :disabled="offset <= 0" @click="prevPage">上一页</el-button>
        <el-button size="small" :disabled="offset + limit >= total" @click="nextPage">下一页</el-button>
      </div>

      <el-table :data="users" size="small" stripe>
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column label="昵称" min-width="120">
          <template #default="{ row }">
            {{ row.nickname || '（未命名）' }}
            <el-tag v-if="row.is_guest" size="small" type="info" effect="plain">游客</el-tag>
            <el-tag v-if="suspicious(asRow(row))" size="small" type="warning" effect="dark">需核查</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="76">
          <template #default="{ row }">
            <el-tag :type="statusOf(asRow(row)).type" size="small">{{ statusOf(asRow(row)).text }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="max_stage" label="最高关" width="86" />
        <el-table-column prop="power" label="战力" width="90" />
        <el-table-column prop="coin" label="金币" width="94" />
        <el-table-column prop="gem" label="钻石" width="80" />
        <el-table-column label="验真" width="130">
          <!--
            三态而不是「不匹配 > 0 就标红」。
            「未验真」与「验过且一致」都显示 0，但含义完全相反 ——
            把它们都当成「干净」，最不可信的用户反而看起来最干净。
          -->
          <template #default="{ row }">
            <el-tag :type="verifyTag(asRow(row)).type" size="small" effect="plain">
              {{ verifyTag(asRow(row)).text }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="最后登录" width="160">
          <template #default="{ row }">{{ fmtTime(row.last_login_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="openDetail(asRow(row))">详情</el-button>
            <el-button v-if="!isReadonly" size="small" text @click="onGrant(asRow(row))">
              发资源
            </el-button>
            <el-button
              v-if="!isReadonly"
              size="small"
              text
              :type="row.status === 1 ? 'danger' : 'success'"
              @click="row.status === 1 ? onBan(asRow(row)) : onUnban(asRow(row))"
            >
              {{ row.status === 1 ? '封禁' : '解封' }}
            </el-button>
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
    </el-card>
  </div>

    <!--
      玩家详情抽屉（2026-10-21 P5-7）。
      此前后台只能看到用户列表的一行 —— 运营要回答的四个问题
      （最近打了什么 / 钱去哪了 / 构筑是什么 / 验真结论）全都得去别处拼 URL。
    -->
    <el-drawer v-model="detailDrawer" title="玩家详情" size="48%">
      <div v-loading="detailLoading">
        <template v-if="detail">
          <el-descriptions :column="2" border size="small">
            <el-descriptions-item label="ID">{{ detail.user.id }}</el-descriptions-item>
            <el-descriptions-item label="昵称">{{ detail.user.nickname || '（未命名）' }}</el-descriptions-item>
            <el-descriptions-item label="最高关卡">{{ detail.user.max_stage }}</el-descriptions-item>
            <el-descriptions-item label="战力">{{ detail.user.power }}</el-descriptions-item>
            <el-descriptions-item label="金币">{{ detail.wallet.coin ?? 0 }}</el-descriptions-item>
            <el-descriptions-item label="钻石">{{ detail.wallet.gem ?? 0 }}</el-descriptions-item>
            <el-descriptions-item label="体力">{{ detail.wallet.energy ?? 0 }}</el-descriptions-item>
            <el-descriptions-item label="钥匙">{{ detail.wallet.keys ?? 0 }}</el-descriptions-item>
            <el-descriptions-item label="验真">{{ detail.user.verify_checked }} / 不匹配 {{ detail.user.verify_mismatched }}</el-descriptions-item>
            <el-descriptions-item label="流水条数">{{ detail.flow_count }}</el-descriptions-item>
          </el-descriptions>

          <div class="section-title">构筑</div>
          <el-descriptions :column="1" border size="small">
            <el-descriptions-item label="出战技能">
              {{ detail.equipped_skills.map((id) => (id > 0 ? '#' + id : '空')).join(' / ') }}
            </el-descriptions-item>
            <el-descriptions-item label="已装配装备">
              {{ detail.equipped_equipment.length ? detail.equipped_equipment.map((id) => '#' + id).join('、') : '（无）' }}
            </el-descriptions-item>
            <el-descriptions-item label="专精节点">
              {{ detail.mastery_nodes.length ? detail.mastery_nodes.join('、') : '（未点亮）' }}
            </el-descriptions-item>
          </el-descriptions>

          <div class="section-title">最近战报</div>
          <el-table :data="detail.recent_battles" size="small" border>
            <el-table-column prop="id" label="ID" width="60" />
            <el-table-column prop="level_id" label="关卡" width="70" />
            <el-table-column prop="result" label="结果" width="70" />
            <el-table-column prop="score" label="分数" width="100" />
            <el-table-column prop="kills" label="击杀" width="70" />
            <el-table-column prop="leaked" label="漏怪" width="70" />
            <el-table-column prop="created_at" label="时间" min-width="160">
              <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
            </el-table-column>
            <template #empty>
              <el-empty description="还没有战报" :image-size="60" />
            </template>
          </el-table>

          <div class="section-title">验真记录</div>
          <el-table :data="detail.verifications" size="small" border>
            <el-table-column prop="battle_id" label="战报" width="80" />
            <el-table-column label="结论" width="90">
              <template #default="{ row }">
                <el-tag :type="row.matched ? 'success' : 'danger'" size="small">
                  {{ row.matched ? '一致' : '不匹配' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="actual_hash" label="上报哈希" min-width="180" />
            <el-table-column prop="created_at" label="时间" min-width="160">
              <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
            </el-table-column>
            <template #empty>
              <el-empty description="没有验真记录" :image-size="60" />
            </template>
          </el-table>
        </template>
      </div>
    </el-drawer>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}
.toolbar-spacer {
  flex: 1;
}
.muted {
  color: var(--lyp-muted);
  font-size: 12px;
}
</style>
