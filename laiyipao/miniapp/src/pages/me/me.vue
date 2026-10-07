<template>
  <view class="page-root">
    <view class="pad" style="padding-top: 100rpx">
      <view class="card">
        <view class="row" style="gap: 20rpx">
          <view class="avatar">炮</view>
          <view class="flex-1">
            <text class="nick">{{ store.nickname || '玩家' }}</text>
            <text class="dim" style="display: block">ID {{ store.userId }}</text>
            <text class="dim">{{ store.isGuest ? '游客账号（未绑定微信）' : '微信账号' }}</text>
          </view>
        </view>
      </view>

      <view class="card">
        <text class="card-title">资源</text>
        <view class="res-grid">
          <view class="res-cell">
            <text class="res-v mono">{{ store.wallet.coin }}</text>
            <text class="res-k">金币</text>
            <text class="res-note">基础升级</text>
          </view>
          <view class="res-cell">
            <text class="res-v mono">{{ store.wallet.gem }}</text>
            <text class="res-k">钻石</text>
            <text class="res-note">洗练 / 专精 / 皮肤</text>
          </view>
          <view class="res-cell">
            <text class="res-v mono">{{ store.wallet.energy }}</text>
            <text class="res-k">体力</text>
            <text class="res-note">进关消耗</text>
          </view>
          <view class="res-cell">
            <text class="res-v mono">{{ store.wallet.keys }}</text>
            <text class="res-k">钥匙</text>
            <text class="res-note">副本 / BOSS</text>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 12rpx">
          四种货币各只有一个用途 —— 这样它们之间无法互相替代，也就没有「哪个更划算」的纠结。
        </text>
      </view>

      <view class="card">
        <text class="card-title">进度</text>
        <view class="kv"><text class="muted">最高关卡</text><text class="mono">第 {{ store.maxStage || 1 }} 关</text></view>
        <view class="kv"><text class="muted">战力</text><text class="mono">{{ store.power }}</text></view>
        <view class="kv">
          <text class="muted">构筑评分</text>
          <text class="mono">{{ store.buildRating?.total ?? 0 }}</text>
        </view>
        <view class="kv">
          <text class="muted">面板攻击力</text>
          <text class="mono">{{ Number(store.attacker.attack) }}‰</text>
        </view>
        <text class="dim" style="display: block; margin-top: 10rpx">
          攻方属性由服务端权威下发（面板攻击力、元素系数、暴击率…）。
          这不是实现细节：回放哈希依赖它，两端各算一套会让验真永远失败。
        </text>
      </view>

      <view class="card">
        <text class="card-title">防线值守</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          你的构筑会被固化成防线快照，其他玩家挑战它时会用<b>同一套引擎在他们的设备上本地模拟</b>，
          攻穿即可窃取你 10% 资源。服务端不跑战斗引擎 —— 所以不需要维护两套引擎保持同步。
        </text>
        <view v-if="myDefense" class="def-box">
          <view class="kv">
            <text class="muted">防线名</text>
            <text>{{ myDefense.name }}</text>
          </view>
          <view class="kv">
            <text class="muted">战力</text>
            <text class="mono">{{ myDefense.power }}</text>
          </view>
          <view class="kv">
            <text class="muted">元素覆盖</text>
            <text class="mono">{{ myDefense.element_coverage }}/5</text>
          </view>
          <view class="kv">
            <text class="muted">战绩</text>
            <text class="mono">{{ myDefense.wins }} 胜 / {{ myDefense.losses }} 负</text>
          </view>
          <view class="kv">
            <text class="muted">快照哈希</text>
            <text class="mono hash">{{ myDefense.snapshot_hash }}</text>
          </view>
        </view>
        <text v-else class="dim">尚未设置防线</text>
        <view class="btn btn-primary" style="margin-top: 16rpx" @click="saveDefense">
          保存当前构筑为防线
        </view>
        <view v-if="myDefense" class="btn" style="margin-top: 12rpx" @click="toggleShield">
          {{ shielded ? '关闭 24h 护盾' : '开启 24h 护盾' }}
        </view>
      </view>

      <view v-if="candidates.length" class="card">
        <text class="card-title">可挑战的防线</text>
        <text class="dim" style="display: block; margin-bottom: 12rpx">
          挑战在<b>你的设备上本地模拟</b>：用你的构筑攻对方快照，战斗数据不上传。
          攻穿即可窃取对方 10% 资源。今日剩余 {{ attackLeft }} 次。
        </text>
        <view v-for="c in candidates" :key="c.id" class="cand">
          <view class="flex-1">
            <text class="cand-name">{{ c.owner_name || '玩家' }} 的防线</text>
            <text class="dim" style="display: block">
              战力 {{ c.power }} · 元素 {{ c.element_coverage }}/5 · {{ c.wins }}胜{{ c.losses }}负
            </text>
            <text class="dim" style="display: block">
              快照 {{ c.snapshot?.skills?.length ?? 0 }} 技能 ·
              {{ (c.snapshot_hash || '').slice(0, 8) || '—' }}
            </text>
            <text v-if="!c.can_challenge" class="dim" style="color: var(--warn)">
              {{ c.challenge_blocked || '暂时无法挑战' }}
            </text>
            <text v-else-if="!snapshotOk(c)" class="dim" style="color: var(--warn)">
              快照不完整，无法模拟
            </text>
          </view>
          <view
            class="btn"
            style="width: 140rpx; height: 64rpx; font-size: 24rpx"
            :class="{ 'btn-disabled': !c.can_challenge || !snapshotOk(c) }"
            @click="challenge(c)"
          >
            {{ simulating === c.id ? '模拟中' : '挑战' }}
          </view>
        </view>
      </view>

      <view
        v-if="lastChallenge"
        class="card"
        :style="{ borderColor: lastChallenge.won ? 'var(--ok)' : 'var(--bad)' }"
      >
        <text class="card-title">挑战结果</text>
        <text class="verdict" :class="lastChallenge.won ? 'ok' : 'bad'">
          {{ lastChallenge.won ? '攻破防线' : '未能攻破' }}
        </text>
        <view class="kv">
          <text class="muted">击杀 / 漏怪</text>
          <text class="mono">{{ lastChallenge.stats.kills }} / {{ lastChallenge.stats.leaked }}</text>
        </view>
        <view class="kv">
          <text class="muted">防线剩余血量</text>
          <text class="mono">{{ lastChallenge.stats.hpLeftPct }}%</text>
        </view>
        <view class="kv">
          <text class="muted">用时</text>
          <text class="mono">{{ (lastChallenge.stats.durationMs / 1000).toFixed(1) }}s</text>
        </view>
        <view class="kv">
          <text class="muted">重放哈希</text>
          <text class="mono hash">{{ lastChallenge.report.replay_hash }}</text>
        </view>
        <text
          v-if="stolenText"
          class="muted"
          style="display: block; margin-top: 12rpx; color: var(--ok)"
        >
          窃取成功：{{ stolenText }}
        </text>
        <text
          v-if="lastChallenge.error"
          class="muted"
          style="display: block; margin-top: 12rpx; color: var(--warn)"
        >
          {{ lastChallenge.error }}
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import {
  runChallenge,
  validateSnapshot,
  type DefenseView,
  type ChallengeOutcome,
} from '@/game/defense'
import type { EquippedSkill } from '@/game/heatmap'
import type { Element } from '@/game/elements'
import type { GeneratedLevel } from '@/game/types'

const store = useGameStore()
const myDefense = ref<DefenseView | null>(null)
const candidates = ref<DefenseView[]>([])
const attackLeft = ref(3)
const shielded = ref(false)
const simulating = ref(0)
const lastChallenge = ref<(ChallengeOutcome & { stolen?: Record<string, number> }) | null>(null)

const DEFENSE_WORK_CODES = ['slow_belt', 'block_wall', 'tesla_grid']

function currencyName(k: string): string {
  return k === 'coin' ? '金币' : k === 'gem' ? '钻石' : k === 'keys' ? '钥匙' : k === 'energy' ? '体力' : k
}

const stolenText = computed(() => {
  const s = lastChallenge.value?.stolen
  if (!s) return ''
  return Object.entries(s)
    .map(([k, v]) => `${currencyName(k)} ${v}`)
    .join(' · ')
})

/** 快照不完整时不给出挑战入口 —— 盲报会污染对方的战绩数据 */
function snapshotOk(c: DefenseView): boolean {
  return validateSnapshot(c).ok
}

async function loadDefenses() {
  try {
    const res = await api.fetchDefenses()
    myDefense.value = res.mine ?? null
    candidates.value = res.candidates ?? []
    attackLeft.value = Math.max(0, (res.attempt_limit ?? 3) - (res.mine?.my_attempts_today ?? 0))
    shielded.value = !!myDefense.value?.shielded_until
  } catch {
    myDefense.value = null
    candidates.value = []
  }
}

async function saveDefense() {
  try {
    const res = await api.saveDefense({
      name: '我的防线',
      skills: store.equippedSkillIds,
      equipment: [],
      mastery_nodes: [],
      works: DEFENSE_WORK_CODES,
      shield_hours: 0,
    })
    myDefense.value = res.defense
    uni.showToast({
      title: `防线已保存（${String(res.defense.snapshot_hash).slice(0, 8)}…）`,
      icon: 'none',
    })
    await loadDefenses()
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

async function toggleShield() {
  try {
    const res = await api.saveDefense({
      name: myDefense.value?.name ?? '我的防线',
      skills: store.equippedSkillIds,
      equipment: [],
      mastery_nodes: [],
      works: DEFENSE_WORK_CODES,
      shield_hours: shielded.value ? 0 : 24,
    })
    myDefense.value = res.defense
    shielded.value = !shielded.value
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

/** 挑战用的标准战场：第 1 关（I-5 规定双方在同一张图上比构筑） */
function challengeLevel(): GeneratedLevel | null {
  return store.levelMap.get(1) ?? store.config?.levels?.[0] ?? null
}

function myEquipped(): EquippedSkill[] {
  const skills = store.skillMap
  return store.equippedSkillIds
    .map((id) => skills.get(id))
    .filter(Boolean)
    // slot 用过滤后的下标。防线挑战不参与 I-6（不与服务端比对哈希），
    // 所以用本地顺序即可，无需与 P1 的服务端 slot 口径对齐。
    .map((s, i) => ({
      skillId: s!.id,
      name: s!.name,
      element: s!.element,
      kind: s!.kind,
      heatCost: BigInt(s!.heat_cost),
      cooldownMs: s!.cooldown_ms,
      pierce: s!.pierce,
      aoeRadius: s!.aoe_radius,
      baseDamage: BigInt(s!.base_damage),
      applyElement: (s!.apply_element || s!.element) as Element | '',
      applyStacks: BigInt(s!.apply_stacks),
      projectileSpeed: s!.projectile_speed,
      chain: s!.chain,
      slot: i,
      cooldownRemaining: 0,
    }))
}

function challenge(target: DefenseView) {
  if (!target.can_challenge) {
    uni.showToast({ title: target.challenge_blocked || '暂时无法挑战', icon: 'none' })
    return
  }
  const level = challengeLevel()
  if (!level) {
    uni.showToast({ title: '关卡数据未加载完成', icon: 'none' })
    return
  }
  const equipped = myEquipped()
  if (equipped.length === 0) {
    uni.showToast({ title: '请先到「背包」页装备出战技能', icon: 'none' })
    return
  }

  simulating.value = target.id
  // 让按钮先进入「模拟中」状态再跑同步模拟
  setTimeout(async () => {
    try {
      const outcome = runChallenge(target, {
        myEquipped: equipped,
        myAttacker: store.attacker,
        level,
        enemies: store.enemyMap,
        skills: store.skillMap,
      })
      if (outcome.error) {
        lastChallenge.value = outcome
        return
      }
      // 模拟完成后才上报，服务端只记账不重跑
      const res = await api.challengeDefense(target.id, {
        // 第 132 轮：seed 按字符串上报（服务端 ChallengeInput.Seed 是 string）。
        // 修前是 Number(outcome.report.seed) —— 引擎用 63-bit bigint 生成种子，
        // 超过 2^53 时 Number() 截断，落库 seed 与本地模拟用的 seed 不一致，
        // replay_hash 从此对不上。
        seed: outcome.report.seed,
        won: outcome.report.won,
        duration_ms: outcome.report.duration_ms,
        hp_left_pct: outcome.report.hp_left_pct,
        replay_hash: outcome.report.replay_hash,
      })
      lastChallenge.value = { ...outcome, stolen: res.stolen ?? {} }
      await store.refreshWallet()
      await loadDefenses()
    } catch (e) {
      lastChallenge.value = {
        won: false,
        report: {
          seed: '0',
          won: false,
          duration_ms: 0,
          hp_left_pct: 100,
          replay_hash: '0000000000000000',
        },
        stats: { kills: 0, leaked: 0, waves: 0, score: 0, durationMs: 0, hpLeftPct: 100 },
        error: (e as Error).message,
      }
    } finally {
      simulating.value = 0
    }
  }, 30)
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  // 第 131 轮：防线快照必须基于**服务端**出战槽位。
  // 先 loadLoadout 拉取 user_skill_slots（权威），再 loadDefenses ——
  // 否则 saveDefense/toggleShield/myEquipped 会用本地初值（修前是捏造的
  // [1,2,3]，修后是空），把玩家没装备过的技能写进防线。
  await store.loadLoadout()
  await loadDefenses()
})

onShow(async () => {
  if (store.loggedIn) {
    await store.refreshProfile()
    // 从「背包」页改过槽位再回来时，这里重新拉一次权威槽位。
    await store.loadLoadout()
    await loadDefenses()
  }
})
</script>

<style scoped>
.avatar {
  width: 96rpx;
  height: 96rpx;
  border-radius: 20rpx;
  background: linear-gradient(135deg, #ff6b35, #ff3d71);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 44rpx;
  font-weight: 700;
  color: #fff;
}

.nick {
  font-size: 32rpx;
  font-weight: 600;
}

.res-grid {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 12rpx;
}

.res-cell {
  width: calc(50% - 6rpx);
  background: var(--panel-2);
  border-radius: 12rpx;
  padding: 18rpx 20rpx;
}

.res-v {
  font-size: 36rpx;
  font-weight: 700;
  display: block;
}

.res-k {
  font-size: 24rpx;
  color: var(--muted);
  display: block;
  margin-top: 4rpx;
}

.res-note {
  font-size: 18rpx;
  color: var(--dim);
}

.kv {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 10rpx 0;
  font-size: 24rpx;
}

.hash {
  font-size: 20rpx;
}

.def-box {
  background: var(--panel-2);
  border-radius: 12rpx;
  padding: 8rpx 20rpx;
  margin-bottom: 12rpx;
}

.cand {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 18rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.cand:last-child {
  border-bottom: none;
}

.cand-name {
  font-size: 26rpx;
  font-weight: 600;
}

.verdict {
  display: block;
  font-size: 30rpx;
  font-weight: 600;
  margin-bottom: 8rpx;
}

.verdict.ok {
  color: var(--ok);
}

.verdict.bad {
  color: var(--bad);
}
</style>
