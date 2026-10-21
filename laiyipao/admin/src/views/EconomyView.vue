<script setup lang="ts">
import {onMounted, ref, computed, onActivated} from 'vue'
import { useAdminRole } from '@/utils/adminRole'
import { ElMessage, ElMessageBox } from 'element-plus'
import { isCancel } from '@/utils/format'
import {
  fetchEconomy,
  updateShopItem,
  type AdminEconomyFlowView,
  type AdminShopItemView,
} from '@/api'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '暂无流水记录'
const loading = ref(false)
// 2026-10-21（P5-6 / 二轮审计 P2-3）：readonly 账号看不到写入口。
const { isReadonly } = useAdminRole()
const flows = ref<AdminEconomyFlowView[]>([])
const shop = ref<AdminShopItemView[]>([])

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
    out[f.currency] = (out[f.currency] ?? 0) + f.delta
  }
  return out
})

function flowTone(net: number): 'success' | 'info' | 'warning' {
  if (net > 0) return 'warning'
  if (net === 0) return 'info'
  return 'success'
}

// 第 112 轮：商城行的 price 是 JSONB map（{coin:1000, gem:5}），不是数字；
// 服务端没有 currency 列，币种就在 price 的 key 上。
// 修前 `num(row, 'price')` 对一个 object 取 Number → NaN → 恒 0，
// `str(row, 'currency')` 恒空 —— 价格列全 0、货币列全空。
function priceMap(o: AdminShopItemView): Record<string, number> {
  return o.price ?? {}
}

function priceText(o: AdminShopItemView): string {
  const m = priceMap(o)
  const ks = Object.keys(m)
  if (ks.length === 0) return '—'
  return ks.map((k) => `${CURRENCY_LABEL[k] ?? k}:${m[k]}`).join('、')
}

function priceCurrencies(o: AdminShopItemView): string {
  const ks = Object.keys(priceMap(o))
  if (ks.length === 0) return '—'
  return ks.map((k) => CURRENCY_LABEL[k] ?? k).join('、')
}

/** enabled 是服务端布尔值（JSON true/false）。 */
function isOnSale(o: AdminShopItemView): boolean {
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

async function patchShop(row: AdminShopItemView, field: string, label: string) {
  try {
    // price 当前值显示成「金币:100、钻石:5」而不是 raw JSON；
    // 限购读的是 limit_per_day（服务端字段名，修前读 limit 恒空）
    const current = field === 'price' ? priceText(row) : String(row[field as keyof AdminShopItemView] ?? '')
    const { value } = await ElMessageBox.prompt(
      `修改「${row.name}」的${label}（当前 ${current}${field === 'price' ? '，格式 货币:数量, 货币:数量' : ''}）`,
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
    await updateShopItem(row.id, { [field]: parsed })
    // 第 113 轮（A-7 同族）：写后读回不再 Object.assign 响应——
    // 服务端响应只有 {id, updated}，assign 完表格还是旧值却弹「已保存」。
    // 直接整页重拉：表格与 KPI 一起对齐真库。
    ElMessage.success('已保存')
    await load()
  } catch (e) {
    // 「取消」与「失败」分开（2026-10-21 二轮审计 P2-2）。
    if (isCancel(e)) return
    ElMessage.error(`保存失败：${(e as Error).message}`)
  }
}

/**
 * P5-5：上架 / 下架。
 *
 * 服务端 `PUT /admin/shop/:id` 早就支持 `enabled`，但后台**没有入口** ——
 * 于是「临时下架一个出问题的商品」这件事运营根本做不到。
 *
 * 下架是破坏性动作（玩家会立刻买不到），所以要二次确认。
 */
async function toggleOnSale(row: AdminShopItemView) {
  const next = !row.enabled
  try {
    await ElMessageBox.confirm(
      next
        ? `上架「${row.name}」？玩家将能立即购买。`
        : `下架「${row.name}」？玩家将立刻买不到它。`,
      next ? '上架商品' : '下架商品',
      { type: next ? 'info' : 'warning', confirmButtonText: next ? '上架' : '下架' },
    )
  } catch {
    return // 取消
  }
  try {
    await updateShopItem(row.id, { enabled: next })
    ElMessage.success(next ? '已上架' : '已下架')
    await load()
  } catch (e) {
    ElMessage.error(`操作失败：${(e as Error).message}`)
  }
}

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 它不能直接传给形参是 `AdminShopItemView` 的函数（TS2345），
 * 所以在调用点集中断言一次（编译后被擦除，运行时行为不变）。
 */
const asRow = (r: unknown) => r as AdminShopItemView

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
            {{ CURRENCY_LABEL[row.currency] ?? row.currency }}
          </template>
        </el-table-column>
        <el-table-column label="来源" width="130">
          <template #default="{ row }">
            {{ REASON_LABEL[row.reason] ?? row.reason }}
          </template>
        </el-table-column>
        <el-table-column label="总量" width="120">
          <template #default="{ row }">{{ row.delta.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column label="笔数" width="90">
          <template #default="{ row }">{{ row.count }}</template>
        </el-table-column>
        <el-table-column label="人均单笔" width="110">
          <template #default="{ row }">
            {{ row.count ? (row.delta / row.count).toFixed(1) : '—' }}
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
    </el-card>

    <el-card shadow="never" header="商城商品" style="margin-top: 12px">
      <el-table :data="shop" size="small" stripe>
        <el-table-column prop="id" label="ID" width="55" />
        <el-table-column label="名称" min-width="140">
          <template #default="{ row }">{{ row.name }}</template>
        </el-table-column>
        <el-table-column prop="category" label="类目" width="90" />
        <el-table-column label="货币" width="80">
          <template #default="{ row }">{{ priceCurrencies(asRow(row)) }}</template>
        </el-table-column>
        <el-table-column label="价格" width="100">
          <template #default="{ row }">{{ priceText(asRow(row)) }}</template>
        </el-table-column>
        <el-table-column label="限购" width="90">
          <template #default="{ row }">
            {{ row.limit_per_day === 0 ? '不限' : row.limit_per_day }}
          </template>
        </el-table-column>
        <el-table-column label="上架" width="80">
          <template #default="{ row }">
            <el-tag :type="isOnSale(asRow(row)) ? 'success' : 'info'" size="small">
              {{ isOnSale(asRow(row)) ? '是' : '否' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="230" fixed="right">
          <template #default="{ row }">
            <el-button v-if="!isReadonly" size="small" text @click="patchShop(asRow(row), 'price', '价格')">改价</el-button>
            <el-button v-if="!isReadonly" size="small" text @click="patchShop(asRow(row), 'limit_per_day', '限购次数')">
              限购
            </el-button>
            <el-button v-if="!isReadonly" size="small" text @click="patchShop(asRow(row), 'name', '名称')">改名</el-button>
            <el-button
              v-if="!isReadonly"
              size="small"
              text
              :type="isOnSale(asRow(row)) ? 'warning' : 'success'"
              @click="toggleOnSale(asRow(row))"
            >
              {{ isOnSale(asRow(row)) ? '下架' : '上架' }}
            </el-button>
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
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
