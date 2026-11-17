<template>
  <!-- 2026-11-16：之前根节点是全项目未定义的 `.page`，也没有 `.pad` / `.safe-bottom`
       —— 商城成了唯一左右贴边、底部压 Home indicator 的页面。 -->
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="wallet-bar">
        <view class="res-pill">
          <Icon name="gemcurrency" :size="26" />
          <text class="mono">{{ wallet.gem ?? 0 }}</text>
        </view>
        <view class="res-pill">
          <Icon name="coin" :size="26" />
          <text class="mono">{{ wallet.coin ?? 0 }}</text>
        </view>
        <view class="res-pill">
          <Icon name="bolt" :size="26" />
          <text class="mono">{{ wallet.energy ?? 0 }}</text>
        </view>
      </view>

      <view v-for="g in groups" :key="g.key" class="card">
        <text class="card-title">{{ g.label }}</text>
        <view
          v-for="it in g.items"
          :key="it.id"
          class="shop-row"
          :class="{ 'bought-flash': justBoughtId === it.id }"
        >
          <view class="icon-badge">
            <Icon :name="it.category === 'gem' ? 'gem' : it.category === 'boost' ? 'bolt' : 'bag'" :size="40" />
          </view>
          <view class="flex-1">
            <text class="skill-name">{{ it.name }}</text>
            <text class="price-text block">{{ priceText(it) }}</text>
            <!-- 每日限购进度（2026-11-01 新一轮第 49 轮；2026-11-17 R17 改进度条）：
                 纯文字「今日已购 x/y」看不出还剩多少，做成 mini 条 + 文案。 -->
            <view v-if="it.limit_per_day > 0" class="limit-wrap">
              <view class="limit-bar">
                <view
                  class="limit-fill"
                  :style="{ width: limitPct(it) + '%' }"
                  :class="{ full: soldOut(it), 'ok-pulse': soldOut(it) }"
                />
              </view>
              <text class="dim limit-text">{{ purchaseProgressText(it) }}</text>
            </view>
          </view>
          <view class="skill-act">
            <view v-if="buyingId === it.id" class="row" style="gap: 6rpx; align-items: center">
              <view class="spinner" />
              <text class="dim">购买中…</text>
            </view>
            <text
              v-else
              class="link"
              :class="{ 'is-disabled': soldOut(it) }"
              @click="doBuy(it)"
            >{{ soldOut(it) ? '已售罄' : '购买' }}</text>
          </view>
        </view>
      </view>

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 64rpx; height: 64rpx; border-radius: 12rpx" />
          <view class="skeleton-bar" style="width: 42%" />
          <view class="skeleton-bar" style="width: 12%; margin-left: auto" />
        </view>
      </view>
      <view v-else-if="loadError" class="card"><text class="muted">加载失败：{{ loadError }}</text></view>
      <view v-else-if="items.length === 0" class="card empty-block">
        <text class="empty-glyph">🛒</text>
        <text class="muted">商城暂无可售商品。</text>
        <text class="dim">运营上新后这里会自动出现，稍后再来看看。</text>
      </view>
    </view>
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
import Icon from '@/components/Icon.vue'

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
/** R35 购买成功行高亮（1.2s 后自动熄灭）。 */
const justBoughtId = ref(0)
let boughtTimer: ReturnType<typeof setTimeout> | null = null
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

/** 限购进度百分比（100 轮打磨 R17）：mini 条宽度。 */
function limitPct(it: ShopItem): number {
  if (it.limit_per_day <= 0) return 0
  return Math.min(100, Math.round((it.bought_today / it.limit_per_day) * 100))
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
  // 购买二次确认（2026-11-01 新一轮第 91 轮）：消耗货币，防止误点误购。
  const confirmed = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '确认购买',
      content: `购买「${it.name}」？将花费 ${priceText(it)}。`,
      confirmText: '购买',
      cancelText: '取消',
      success: (r) => resolve(!!r.confirm),
      fail: () => resolve(false),
    })
  })
  if (!confirmed) return
  buyingId.value = it.id
  try {
    await api.buyItem(it.id)
    // 购买同时改变「买了几个」与「余额」，两者都必须以服务端为准：
    // 本地加 1 会在限次边界上显示还能买，而服务端已经拒绝。
    await load()
    // R35 成功反馈：只有刚买的那一行闪烁，玩家一眼确认「买的是这件」
    if (boughtTimer) clearTimeout(boughtTimer)
    justBoughtId.value = it.id
    boughtTimer = setTimeout(() => {
      justBoughtId.value = 0
      boughtTimer = null
    }, 1200)
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
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  padding: 0 0 16rpx;
}
.wallet-bar :deep(svg) {
  color: var(--fire);
}
.shop-row {
  display: flex;
  align-items: center;
  gap: 16rpx;
  padding: 20rpx 0;
  border-bottom: 1rpx solid var(--border-soft);
}
.shop-row:last-child {
  border-bottom: none;
}

/* R35 购买成功：刚买的那一行短促高亮 */
.shop-row.bought-flash {
  animation: bought-flash 0.6s ease-out;
  background: var(--ok-soft, rgba(126, 231, 135, 0.1));
}

@keyframes bought-flash {
  0% { background: rgba(126, 231, 135, 0.28); }
  100% { background: rgba(126, 231, 135, 0); }
}
.skill-name {
  font-size: var(--fs-md);
  font-weight: 600;
}
.price-text {
  color: var(--warn);
  font-size: var(--fs-sm);
  margin-top: 4rpx;
  font-variant-numeric: tabular-nums;
}
/* 每日限购 mini 进度条（100 轮打磨 R17） */
.limit-wrap {
  margin-top: 6rpx;
}
.limit-bar {
  width: 100%;
  height: 8rpx;
  background: var(--panel-3);
  border-radius: 4rpx;
  overflow: hidden;
}
.limit-fill {
  height: 100%;
  border-radius: 4rpx;
  background: var(--fire);
  transition: width 0.3s ease-out;
}
.limit-fill.full {
  background: var(--ok);
}
.limit-text {
  font-size: 22rpx;
  margin-top: 4rpx;
}
.skill-act {
  flex-shrink: 0;
  margin-left: 16rpx;
}
</style>