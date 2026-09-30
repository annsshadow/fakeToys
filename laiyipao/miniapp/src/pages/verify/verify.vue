<template>
  <view class="page-root">
    <view class="pad">
      <view class="card">
        <text class="card-title">战绩验真</text>
        <text class="muted" style="display: block">
          战斗是完全确定性的：同样的<b>关卡 + 随机种子 + 构筑</b> → 同样的事件序列 →
          同样的回放哈希。任何人都能用 battle_id 取回这三样东西并本地重放，
          <b>证伪</b>一个可疑分数。
        </text>
        <text class="muted" style="display: block; margin-top: 10rpx">
          注意第三样：<b>构筑</b>。缺了构筑就算不出正确哈希 ——
          所以服务端在结算时把当时的构筑冻结进了 build_snapshot。
        </text>
      </view>

      <view class="card">
        <text class="card-title">输入 battle_id</text>
        <view class="row" style="gap: 12rpx">
          <input
            v-model="battleId"
            class="inp"
            type="number"
            placeholder="例如 1"
            placeholder-class="ph"
          />
          <view class="btn" style="width: 180rpx; height: 72rpx" @click="fetchReplay">
            取复现信息
          </view>
        </view>
        <text v-if="errMsg" class="muted" style="display: block; margin-top: 12rpx; color: var(--bad)">
          {{ errMsg }}
        </text>
      </view>

      <view v-if="replayInfo" class="card">
        <text class="card-title">复现信息</text>
        <view class="kv"><text class="muted">battle_id</text><text class="mono">{{ replayInfo.battle_id }}</text></view>
        <view class="kv"><text class="muted">关卡</text><text class="mono">第 {{ replayInfo.level_id }} 关</text></view>
        <view class="kv"><text class="muted">波次</text><text class="mono">{{ replayInfo.level.wave_count }}</text></view>
        <view class="kv"><text class="muted">随机种子</text><text class="mono">{{ replayInfo.seed }}</text></view>
        <view class="kv">
          <text class="muted">构筑技能</text>
          <text class="mono">{{ buildText }}</text>
        </view>
        <view class="kv">
          <text class="muted">攻方攻击力</text>
          <text class="mono">{{ attackerText }}</text>
        </view>
        <view class="kv"><text class="muted">记录时间</text><text class="mono">{{ replayInfo.created_at }}</text></view>
        <view class="kv">
          <text class="muted">服务端记录的哈希</text>
          <text class="mono hash">{{ pretty(replayInfo.expected_hash) }}</text>
        </view>
      </view>

      <view v-if="replayInfo" class="card">
        <text class="card-title">本地重放</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          用上面的关卡、种子、构筑在本地重跑完整局（约 1 万 tick，毫秒级完成），
          算出哈希后提交比对。
        </text>

        <view v-if="replaying" class="row" style="gap: 12rpx">
          <text class="muted">重放中…</text>
        </view>

        <view v-else-if="outcome?.error" class="clamp-note">
          无法重放：{{ outcome.error }}
        </view>

        <view v-else-if="outcome" class="verdict-box" :class="outcome.matched ? 'ok' : 'bad'">
          <view class="kv">
            <text class="muted">本地重放哈希</text>
            <text class="mono hash">{{ pretty(outcome.computedHash) }}</text>
          </view>
          <view class="verdict">{{ outcome.matched ? '一致 — 分数可复现' : '不一致 — 该分数无法被复现' }}</view>
          <view class="stats">
            <text>击杀 {{ outcome.stats.kills }}</text>
            <text>漏怪 {{ outcome.stats.leaked }}</text>
            <text>波次 {{ outcome.stats.waves }}</text>
            <text>用时 {{ duration(outcome.stats.durationMs) }}</text>
          </view>
        </view>

        <view class="btn btn-primary" style="margin-top: 16rpx" @click="doReplay">
          本地重放并提交验真
        </view>

        <view class="btn" style="margin-top: 12rpx" @click="doFalsify">
          提交伪造哈希（演示证伪）
        </view>
        <text class="dim" style="display: block; margin-top: 12rpx">
          「伪造哈希」提交全零值，必然不一致 —— 这正是「任何人可证伪」的含义：
          分数正确性不靠信任，而靠可复现。
        </text>
      </view>

      <view v-if="result" class="card" :style="{ borderColor: result.matched ? 'var(--ok)' : 'var(--bad)' }">
        <text class="card-title">服务端裁定</text>
        <view class="kv">
          <text class="muted">你提交的哈希</text>
          <text class="mono hash">{{ pretty(result.actual_hash) }}</text>
        </view>
        <view class="kv">
          <text class="muted">服务端记录</text>
          <text class="mono hash">{{ pretty(result.expected_hash) }}</text>
        </view>
        <text class="verdict" :class="result.matched ? 'ok' : 'bad'">
          {{ result.matched ? '验真通过' : '验真失败 — 分数被判定为不可复现' }}
        </text>
        <text class="muted" style="display: block; margin-top: 12rpx">
          验真记录已落库，可在运营后台「战斗记录与验真」页查看一致率统计。
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
// import 时改别名：函数名 replay 与页面状态概念同名会遮蔽模板解析
import {
  replay as runReplay,
  prettyHash,
  prettyDuration,
  type ReplayOutcome,
  type ReplayInfo,
} from '@/game/replay'

const store = useGameStore()
const battleId = ref('')
const replayInfo = ref<ReplayInfo | null>(null)
const outcome = ref<ReplayOutcome | null>(null)
const result = ref<any>(null)
const errMsg = ref('')
const replaying = ref(false)

const buildText = computed(() => {
  const skills = Object.values((replayInfo.value?.build?.skills ?? {}) as Record<string, any>)
  const on = skills.filter((s) => s && s.slot >= 0)
  if (on.length === 0) return '无（全部未装备）'
  return on
    .slice()
    .sort((a, b) => a.slot - b.slot)
    .map((s) => `#${s.slot} ${s.name}(${s.element})`)
    .join(' · ')
})

const attackerText = computed(() => {
  const a = replayInfo.value?.build?.attacker
  if (!a || typeof a.attack !== 'number') return '缺失 — 无法重放'
  return `攻击 ${a.attack}‰ · 暴击 ${a.crit_permille}‰ · 元素系数 ${a.element_coef_permille}‰`
})

function pretty(h: string): string {
  return prettyHash(h)
}

function duration(ms: number): string {
  return prettyDuration(ms)
}

async function fetchReplay() {
  errMsg.value = ''
  replayInfo.value = null
  outcome.value = null
  result.value = null
  const id = Number(battleId.value)
  if (!id || id < 1) {
    errMsg.value = '请输入有效的 battle_id'
    return
  }
  try {
    replayInfo.value = await api.getReplay(id)
  } catch (e) {
    errMsg.value = (e as Error).message
  }
}

async function submit(hash: string) {
  const id = Number(battleId.value)
  try {
    result.value = await api.verifyReplay(id, hash)
  } catch (e) {
    errMsg.value = (e as Error).message
  }
}

function doReplay() {
  const ri = replayInfo.value
  if (!ri) {
    errMsg.value = '请先取复现信息'
    return
  }
  replaying.value = true
  errMsg.value = ''
  // 先让 loading 态渲染出来，再做同步重放（约 1 万 tick，毫秒级）
  setTimeout(async () => {
    try {
      const r = runReplay(ri, {
        level: ri.level,
        enemies: store.enemyMap,
        skills: store.skillMap,
      })
      outcome.value = r
      if (!r.error) await submit(r.computedHash)
    } catch (e) {
      errMsg.value = `重放异常：${(e as Error).message}`
    } finally {
      replaying.value = false
    }
  }, 30)
}

function doFalsify() {
  if (!replayInfo.value) {
    errMsg.value = '请先取复现信息'
    return
  }
  outcome.value = null
  void submit('0000000000000000')
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
})
</script>

<style scoped>
.inp {
  flex: 1;
  height: 72rpx;
  background: var(--panel-2);
  border: 1rpx solid var(--border);
  border-radius: 10rpx;
  padding: 0 20rpx;
  font-size: 26rpx;
  color: var(--text);
}

.ph {
  color: var(--dim);
}

.kv {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 8rpx 0;
  font-size: 24rpx;
  gap: 16rpx;
}

.hash {
  font-size: 20rpx;
  text-align: right;
}

.verdict-box {
  border-radius: 10rpx;
  padding: 16rpx;
  background: rgba(0, 0, 0, 0.25);
}

.verdict-box.ok {
  border: 1rpx solid var(--ok);
}

.verdict-box.bad {
  border: 1rpx solid var(--bad);
}

.verdict {
  display: block;
  font-size: 28rpx;
  font-weight: 600;
  margin-top: 12rpx;
}

.verdict.ok {
  color: var(--ok);
}

.verdict.bad {
  color: var(--bad);
}

.stats {
  display: flex;
  flex-direction: row;
  gap: 24rpx;
  margin-top: 12rpx;
  font-size: 22rpx;
  color: var(--muted);
  flex-wrap: wrap;
}

.clamp-note {
  font-size: 22rpx;
  color: var(--warn);
  background: rgba(255, 211, 61, 0.1);
  padding: 14rpx 16rpx;
  border-radius: 8rpx;
}
</style>
