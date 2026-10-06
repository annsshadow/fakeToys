<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { fetchEconomy, updateShopItem } from '@/api'

const loading = ref(false)
const flows = ref<Array<Record<string, unknown>>>([])
const shop = ref<Array<Record<string, unknown>>>([])

const CURRENCY_LABEL: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
}
const REASON_LABEL: Record<string, string> = {
  battle_loot: '战斗掉落',
  signin: '签到',
  task: '任务',
  shop: '商城消费',
  admin_grant: '后台发放',
  admin_revoke: '后台回收',
  defense_steal: '防线窃取',
  redeem: '兑换码',
}

/**
 * 通胀监控：净流入长期为正说明货币在贬值，需要回收。
 * 这直接对应 I-1 的反通胀设计 —— 数值堆叠不能替代正确搭配。
 *
 * ⚠️ 第 112 轮：服务端 AdminEconomy 的流水行字段是 `delta`/`count`
 * （SUM(delta)、COUNT(*)），**不是** `amount`/`cnt`，也没有 `updated_at`。
 * 修前读 `f.amount`（undefined→0），净流入 KPI 恒 0、显示「健康」，
 * 通胀监控形同虚设。这里改读真实字段。
 */
const netFlow = computed(() => {
  const out: Record<string, number> = {}
  for (const f of flows.value) {
    const cur = String(f.currency ?? '')
    const amt = Number(f.delta ?? 0)
    out[cur] = (out[cur] ?? 0) + amt
  }
  return out
})

function str(o: Record<string, unknown>, k: string): string {
  const v = o[k]
  return v === undefined || v === null ? '' : String(v)
}

function num(o: Record<string, unknown>, k: string): number {
  const v = Number(o[k] ?? 0)
  return Number.isFinite(v) ? v : 0
}

function flowTone(net: number): 'success' | 'info' | 'warning' {
  if (net > 0) return 'warning'
  if (net === 0) return 'info'
  return 'success'
}

// 第 112 轮：商城行的 price 是 JSONB map（{coin:1000, gem:5}），不是数字；
// 服务端没有 currency 列，币种就在 price 的 key 上。
// 修前 `num(row, 'price')` 对一个 object 取 Number → NaN → 恒 0，
// `str(row, 'currency')` 恒空 —— 价格列全 0、货币列全空。
function priceMap(o: Record<string, unknown>): Record<string, number> {
  const p = o.price
  if (typeof p !== 'object' || p === null || Array.isArray(p)) return {}
  return p as Record<string, number>
}

function priceText(o: Record<string, unknown>): string {
  const m = priceMap(o)
  const ks = Object.keys(m)
  if (ks.length === 0) return '—'
  return ks.map((k) => `${CURRENCY_LABEL[k] ?? k}:${m[k]}`).join('、')
}

function priceCurrencies(o: Record<string, unknown>): string {
  const ks = Object.keys(priceMap(o))
  if (ks.length === 0) return '—'
  return ks.map((k) => CURRENCY_LABEL[k] ?? k).join('、')
}

/** enabled 是服务端布尔值（JSON true/false），显式 === true 兜住一切脏形状 */
function isOnSale(o: Record<string, unknown>): boolean {
  return o.enabled === true
}

async function load() {
  loading.value = true
  try {
    const res = await fetchEconomy()
    flows.value = res.flows
    shop.value = res.shop
  } catch (e) {
    ElMessage.error(`经济数据加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

/**
 * 第 113 轮：价格输入「coin:100, gem:5」→ {coin:100, gem:5}。
 * 服务端 price 列是 JSONB 的 map[string]int（玩家 Buy 按 map 读），
 * 修前发裸数字 100 → 落库成 JSON 标量 → 该商品购买链路永久 500。
 * 货币 key 限定在 CURRENCY_LABEL（与服务端「货币全集」同一份名字）。
 */
function parsePriceMap(s: string): Record<string, number> | null {
  const out: Record<string, number> = {}
  const t = s.trim()
  if (t === '') return null
  for (const part of t.split(',')) {
    const p = part.trim()
    if (!p) continue
    const i = p.lastIndexOf(':')
    if (i <= 0) return null
    const k = p.slice(0, i).trim()
    const v = Number(p.slice(i + 1).trim())
    if (!(k in CURRENCY_LABEL) || !Number.isInteger(v) || v < 0) return null
    out[k] = v
  }
  return Object.keys(out).length === 0 ? null : out
}

async function patchShop(row: Record<string, unknown>, field: string, label: string) {
  try {
    // price 当前值显示成「金币:100、钻石:5」而不是 raw JSON；
    // 限购读的是 limit_per_day（服务端字段名，修前读 limit 恒空）
    const current = field === 'price' ? priceText(row) : String(num(row, field))
    const { value } = await ElMessageBox.prompt(
      `修改「${str(row, 'name')}」的${label}（当前 ${current}${field === 'price' ? '，格式 货币:数量, 货币:数量' : ''}）`,
      '编辑商城项',
      { inputValue: current },
    )
    let parsed: unknown
    if (field === 'price') {
      parsed = parsePriceMap(value)
      if (parsed === null) {
        ElMessage.error('价格格式应为「货币:数量, 货币:数量」（如 coin:100, gem:5）')
        return
      }
    } else if (field === 'limit_per_day') {
      const n = Number(value)
      if (!Number.isInteger(n) || n < 0) {
        ElMessage.error('限购须为非负整数（0 = 不限）')
        return
      }
      parsed = n
    } else {
      parsed = value
    }
    await updateShopItem(num(row, 'id'), { [field]: parsed })
    // 第 113 轮（A-7 同族）：写后读回不再 Object.assign 响应——
    // 服务端响应只有 {id, updated}，assign 完表格还是旧值却弹「已保存」。
    // 直接整页重拉：表格与 KPI 一起对齐真库。
    ElMessage.success('已保存')
    await load()
  } catch {
    /* 取消 */
  }
}

onMounted(load)
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">经济与商城</div>
    <p class="page-subtitle">
      货币<strong>净流入为正</strong>意味着在贬值。这与 I-1 的反通胀设计直接相关：
      如果金币能靠刷战斗无限增长，「正确元素搭配」的优势就会被数值堆叠淹没。
    </p>

    <el-row :gutter="12" style="margin-bottom: 12px">
      <el-col v-for="cur in Object.keys(CURRENCY_LABEL)" :key="cur" :span="6">
        <el-card shadow="never">
          <div class="kpi-label">{{ CURRENCY_LABEL[cur] }}净流入</div>
          <div class="kpi-value">
            <el-tag :type="flowTone(netFlow[cur] ?? 0)" size="large">
              {{ (netFlow[cur] ?? 0) > 0 ? '+' : '' }}{{ netFlow[cur] ?? 0 }}
            </el-tag>
          </div>
          <div class="kpi-sub">
            {{ netFlow[cur] && netFlow[cur]! > 0 ? '在贬值，考虑回收' : '健康' }}
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card shadow="never" header="货币流水" >
      <el-table :data="flows" size="small" stripe>
        <el-table-column label="货币" width="90">
          <template #default="{ row }">
            {{ CURRENCY_LABEL[str(row, 'currency')] ?? str(row, 'currency') }}
          </template>
        </el-table-column>
        <el-table-column label="来源" width="130">
          <template #default="{ row }">
            {{ REASON_LABEL[str(row, 'reason')] ?? str(row, 'reason') }}
          </template>
        </el-table-column>
        <el-table-column label="总量" width="120">
          <template #default="{ row }">{{ num(row, 'delta').toLocaleString() }}</template>
        </el-table-column>
        <el-table-column label="笔数" width="90">
          <template #default="{ row }">{{ num(row, 'count') }}</template>
        </el-table-column>
        <el-table-column label="人均" width="110">
          <template #default="{ row }">
            {{ num(row, 'count') ? (num(row, 'delta') / num(row, 'count')).toFixed(1) : '—' }}
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-card shadow="never" header="商城商品" style="margin-top: 12px">
      <el-table :data="shop" size="small" stripe>
        <el-table-column prop="id" label="ID" width="55" />
        <el-table-column label="名称" min-width="140">
          <template #default="{ row }">{{ str(row, 'name') }}</template>
        </el-table-column>
        <el-table-column label="货币" width="80">
          <template #default="{ row }">{{ priceCurrencies(row) }}</template>
        </el-table-column>
        <el-table-column label="价格" width="100">
          <template #default="{ row }">{{ priceText(row) }}</template>
        </el-table-column>
        <el-table-column label="限购" width="90">
          <template #default="{ row }">
            {{ num(row, 'limit_per_day') === 0 ? '不限' : num(row, 'limit_per_day') }}
          </template>
        </el-table-column>
        <el-table-column label="上架" width="80">
          <template #default="{ row }">
            <el-tag :type="isOnSale(row) ? 'success' : 'info'" size="small">
              {{ isOnSale(row) ? '是' : '否' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="patchShop(row, 'price', '价格')">改价</el-button>
            <el-button size="small" text @click="patchShop(row, 'limit_per_day', '限购次数')">限购</el-button>
            <el-button size="small" text @click="patchShop(row, 'name', '名称')">改名</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<style scoped>
.kpi-label {
  font-size: 12px;
  color: var(--lyp-muted);
}
.kpi-value {
  margin: 6px 0 4px;
}
.kpi-sub {
  font-size: 12px;
  color: #6e7681;
}
</style>
