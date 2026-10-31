<template>
  <view class="page">
    <view class="wallet-bar">
      <text class="dim">钻石 {{ wallet.gem ?? 0 }} · 金币 {{ wallet.coin ?? 0 }} · 体力 {{ wallet.energy ?? 0 }}</text>
    </view>

    <view v-for="g in groups" :key="g.key">
      <text class="card-title">{{ g.label }}</text>
      <view v-for="it in g.items" :key="it.id" class="shop-row">
        <view class="flex-1">
          <text class="skill-name">{{ it.name }}</text>
          <text class="dim" style="display: block">{{ priceText(it) }}</text>
          <!-- 每日限购进度（2026-11-01 新一轮第 49 轮） -->
          <text v-if="purchaseProgressText(it)" class="dim">{{ purchaseProgressText(it) }}</text>
        </view>
        <view class="skill-act">
          <text v-if="buyingId === it.id" class="dim">购买中…</text>
          <text
            v-else
            class="link"
            :class="{ disabled: soldOut(it) }"
            @click="doBuy(it)"
          >{{ soldOut(it) ? '已售罄' : '购买' }}</text>
        </view>
      </view>
    </view>

    <text v-if="loading" class="dim">加载中…</text>
    <text v-else-if="loadError" class="dim">加载失败：{{ loadError }}</text>
    <text v-else-if="items.length === 0" class="dim">商城暂无可售商品。</text>
  </view>
</template>

<script setup lang="ts">
/**
 * 商城页。
 *
 * # 为什么这个页面是「C 口径闭环」的一部分，而不是新功能
 *
 * 三种道具（`revive_token` / `mastery_reset` / `gem_wash_token`）
 * **唯一的发放途径就是商城** —— 签到、任务、成就、兑换码都不发它们。
 *
 * 而在补上这个页面之前，客户端里 `fetchShop` / `buyItem` 两个函数
 * 没有任何页面调用方：服务端的消费端点全都在，玩家却拿不到道具。
 *
 * 「服务端有扣减逻辑」不等于「玩家能用」—— 前者是代码完成度，
 * 后者才是机制闭环。这个页面补的是后者。
 */
import { onMounted, ref } from 'vue'
import * as api from '@/api/client'
import { useGameStore } from '@/store/game'

const store = useGameStore()
const items = ref<ShopItem[]>([])
const loading = ref(true)
/**
 * 加载失败的原因；空串表示「确实加载成功了」。
 *
 * 为什么要单独一个字段而不是靠 `items.length === 0` 区分：
 * 「拉不到」与「商城真的空」在界面上都长成空白，而后者会让玩家
 * 以为自己错过了什么，前者会让玩家以为是游戏坏了 —— 只有 toast
 * 知道真相，3 秒后就没了。所以错误必须留在页面上。
 */
const loadError = ref('')
/** 正在购买的商品 id；非 0 时禁用全部购买按钮，防连点连扣。 */
const buyingId = ref(0)
const wallet = ref({ coin: 0, gem: 0, energy: 0, keys: 0 })

/** 与服务端 `economy.ShopItemView` 一一对应。 */
interface ShopItem {
  id: number
  code: string
  name: string
  category: string
  /** 花费：`{coin: n}` / `{gem: n}` / `{energy: n}`，全 0 表示免费。 */
  price: Record<string, number>
  /** 产出：`{coin: n}` / `{token: n}` / `{gem_instance: 1}`。 */
  payload: Record<string, number>
  /** 每日限购；0 = 不限次。 */
  limit_per_day: number
  bought_today: number
}

/** 类目分组。顺序即展示顺序。 */
const CATEGORY_ORDER = [
  { key: 'gem', label: '宝石' },
  { key: 'boost', label: '强化道具' },
  { key: '', label: '其他' },
] as const

const groups = ref<{ key: string; label: string; items: ShopItem[] }[]>([])

const CURRENCY_CN: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
}

const PAYLOAD_CN: Record<string, string> = {
  gem_instance: '随机宝石 1 颗',
  revive_token: '续战令',
  mastery_reset: '专精点重置',
  gem_wash_token: '洗练石',
}

function priceText(it: ShopItem): string {
  const parts = Object.entries(it.price)
    .filter(([, v]) => v > 0)
    .map(([k, v]) => `${v} ${CURRENCY_CN[k] ?? k}`)
  const pay = parts.length ? parts.join(' + ') : '免费'
  const gain = Object.entries(it.payload)
    .map(([k, v]) => (k === 'gem_instance' ? PAYLOAD_CN[k] : `${v} ${PAYLOAD_CN[k] ?? CURRENCY_CN[k] ?? k}`))
    .join(' + ')
  return gain ? `${pay} → ${gain}` : pay
}

/** 今日已买满（`limit_per_day` 为 0 表示不限次）。 */
function soldOut(it: ShopItem): boolean {
  return it.limit_per_day > 0 && it.bought_today >= it.limit_per_day
}

/**
 * 每日限购进度文案（2026-11-01 新一轮第 49 轮）。
 *
 * 此前只有「今日已买满」/「购买」两种状态 —— 玩家看不到还剩几次
 * 购买机会，限购商品只能靠试错。有限购时显示「今日已购 x/y」，
 * 买满显示「今日已买满」；不限购（limit_per_day=0）不显示。
 */
function purchaseProgressText(it: ShopItem): string {
  if (it.limit_per_day <= 0) return ''
  if (it.bought_today >= it.limit_per_day) return '今日已买满'
  return `今日已购 ${it.bought_today}/${it.limit_per_day}`
}

async function load() {
  loading.value = true
  loadError.value = ''
  // 钱包与商品**真正并发**（2026-10-31 实质优化第 30 轮）：
  // 此前的注释写着「并发」，实现却是先等商品再等钱包 ——
  // 两件事互不依赖，串起来会让首屏白等一倍时间。
  // 各自的错误独立处理：商品失败显示错误但不影响钱包；
  // 钱包失败静默（价格已写在每行里，不影响下单判断）。
  const [shopResult, walletResult] = await Promise.all([
    api.fetchShop().then(
      (r): { items: ShopItem[]; error: string } => ({
        items: Array.isArray(r) ? r : (r?.items ?? []),
        error: '',
      }),
      (e: Error): { items: ShopItem[]; error: string } => ({
        items: [],
        error: e.message,
      }),
    ),
    api.fetchWallet().then(
      (w): { wallet: typeof wallet.value; ok: boolean } => ({ wallet: w, ok: true }),
      (): { wallet: typeof wallet.value; ok: boolean } => ({
        wallet: { coin: 0, gem: 0, energy: 0, keys: 0 },
        ok: false,
      }),
    ),
  ])

  items.value = shopResult.items
  loadError.value = shopResult.error
  if (shopResult.error) {
    // 拉不到商城 ≠ 商城是空的；后者会误导玩家去别处找。
    uni.showToast({ title: shopResult.error, icon: 'none' })
  }
  if (walletResult.ok) {
    wallet.value = walletResult.wallet
  }
  groups.value = CATEGORY_ORDER.map((c) => ({
    key: c.key,
    label: c.label,
    items: items.value.filter((x) => (x.category || '') === c.key),
  })).filter((g) => g.items.length > 0)
  loading.value = false
}

async function doBuy(it: ShopItem) {
  if (buyingId.value !== 0) return
  if (soldOut(it)) return
  buyingId.value = it.id
  try {
    await api.buyItem(it.id)
    // 购买同时改变「买了几个」与「余额」，两者都必须以服务端为准：
    // 本地加 1 会在限次边界上显示还能买，而服务端已经拒绝。
    await load()
    uni.showToast({ title: '购买成功', icon: 'none' })
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
    // 失败也重拉：限次/余额可能已被别的请求改变，页面此刻是陈旧的。
    await load()
  } finally {
    buyingId.value = 0
  }
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  await load()
})
</script>

<style scoped>
.wallet-bar {
  padding: 24rpx 0 8rpx;
}
.shop-row {
  display: flex;
  align-items: center;
  padding: 20rpx 0;
  border-bottom: 1rpx solid #1c2430;
}
.disabled {
  color: #55606e;
}
</style>