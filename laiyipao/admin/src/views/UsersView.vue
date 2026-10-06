<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { fetchUsers, banUser, unbanUser, grantUser, type AdminUser } from '@/api'

const loading = ref(false)
const users = ref<AdminUser[]>([])
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
function statusOf(u: AdminUser): { text: string; type: 'success' | 'danger' } {
  return u.status === 1 ? { text: '正常', type: 'success' } : { text: '封禁', type: 'danger' }
}

/** 高战力但零战斗记录的用户是刷分嫌疑，列表里直接标出来。 */
function suspicious(u: AdminUser): boolean {
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

function verifyState(u: AdminUser): VerifyState {
  if ((u.verify_mismatched ?? 0) > 0) return 'suspect'
  if ((u.verify_checked ?? 0) === 0) return 'unknown'
  return 'clean'
}

function verifyTag(u: AdminUser): { text: string; type: 'info' | 'success' | 'danger' } {
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

async function onBan(u: AdminUser) {
  try {
    const { value } = await ElMessageBox.prompt(`封禁「${u.nickname}」的原因`, '封禁用户', {
      inputType: 'textarea',
      inputPlaceholder: '会记入审计日志',
    })
    if (!value.trim()) return
    const res = await banUser(u.id, value.trim())
    Object.assign(u, res.user)
    ElMessage.success('已封禁')
  } catch {
    /* 取消 */
  }
}

async function onUnban(u: AdminUser) {
  const res = await unbanUser(u.id)
  Object.assign(u, res.user)
  ElMessage.success('已解封')
}

async function onGrant(u: AdminUser) {
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
    const res = await grantUser(u.id, currency, amount)
    u.coin = res.wallet.coin
    u.gem = res.wallet.gem
    ElMessage.success(
      amount < 0
        ? `已回收 ${CURRENCY_LABEL[currency]} ${-amount}`
        : `已发放 ${CURRENCY_LABEL[currency]} ${amount}`,
    )
  } catch {
    /* 取消 */
  }
}

function fmtTime(s: string): string {
  if (!s) return '—'
  return s.replace('T', ' ').slice(0, 19)
}

onMounted(load)

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
const asRow = (r: unknown) => r as AdminUser
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
            <el-button size="small" text @click="onGrant(asRow(row))">发资源</el-button>
            <el-button
              size="small"
              text
              :type="row.status === 1 ? 'danger' : 'success'"
              @click="row.status === 1 ? onBan(asRow(row)) : onUnban(asRow(row))"
            >
              {{ row.status === 1 ? '封禁' : '解封' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
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
