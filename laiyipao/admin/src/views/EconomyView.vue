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
 */
const netFlow = computed(() => {
  const out: Record<string, number> = {}
  for (const f of flows.value) {
    const cur = String(f.currency ?? '')
    const amt = Number(f.amount ?? 0)
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

async function patchShop(row: Record<string, unknown>, field: string, label: string) {
  try {
    const { value } = await ElMessageBox.prompt(
      `修改「${str(row, 'name')}」的${label}（当前 ${str(row, field)}）`,
      '编辑商城项',
      { inputValue: str(row, field) },
    )
    const parsed = field === 'price' || field === 'limit' || field === 'stock' ? Number(value) : value
    const res = await updateShopItem(num(row, 'id'), { [field]: parsed })
    Object.assign(row, res.item)
    ElMessage.success('已保存')
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
          <template #default="{ row }">{{ num(row, 'amount').toLocaleString() }}</template>
        </el-table-column>
        <el-table-column label="笔数" width="90">
          <template #default="{ row }">{{ num(row, 'cnt') }}</template>
        </el-table-column>
        <el-table-column label="人均" width="110">
          <template #default="{ row }">
            {{ num(row, 'cnt') ? (num(row, 'amount') / num(row, 'cnt')).toFixed(1) : '—' }}
          </template>
        </el-table-column>
        <el-table-column prop="updated_at" label="统计时间" min-width="160" />
      </el-table>
    </el-card>

    <el-card shadow="never" header="商城商品" style="margin-top: 12px">
      <el-table :data="shop" size="small" stripe>
        <el-table-column prop="id" label="ID" width="55" />
        <el-table-column label="名称" min-width="140">
          <template #default="{ row }">{{ str(row, 'name') }}</template>
        </el-table-column>
        <el-table-column label="货币" width="80">
          <template #default="{ row }">
            {{ CURRENCY_LABEL[str(row, 'currency')] ?? str(row, 'currency') }}
          </template>
        </el-table-column>
        <el-table-column label="价格" width="100">
          <template #default="{ row }">{{ num(row, 'price') }}</template>
        </el-table-column>
        <el-table-column label="限购" width="90">
          <template #default="{ row }">
            {{ num(row, 'limit') === 0 ? '不限' : num(row, 'limit') }}
          </template>
        </el-table-column>
        <el-table-column label="上架" width="80">
          <template #default="{ row }">
            <el-tag :type="num(row, 'on_sale') === 1 ? 'success' : 'info'" size="small">
              {{ num(row, 'on_sale') === 1 ? '是' : '否' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="patchShop(row, 'price', '价格')">改价</el-button>
            <el-button size="small" text @click="patchShop(row, 'limit', '限购次数')">限购</el-button>
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
