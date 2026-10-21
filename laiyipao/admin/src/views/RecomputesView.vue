<script setup lang="ts">
/**
 * 结算重算审计（R1）。
 *
 * ## 这个页面存在的理由
 *
 * 服务端开始自己重算之后，会出现一个**之前没有**的失败模式：
 * 引擎与客户端算法漂移 → 诚实玩家的上报被判不符 → 奖励被拒发。
 *
 * 它最糟的地方不是「会误伤」，而是**误伤在被发现之前不可见** ——
 * 结算返回 422，客户端显示「上报不可信」，玩家以为被作弊，
 * 运营看不出是我方算错了。发现方式是玩家投诉。
 *
 * 所以这个页面第一件要做的事，是把**闸门状态**摆在最上面：
 * 熔断触发时系统「仍在重算、仍在记录、但不再拒发」，
 * 而台账数字看起来可能完全正常。不单独指出来的话，
 * 值班的人会以为防护开着。
 */
import { computed, onMounted, ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchRecomputes,
  fetchRecomputeStats,
  fetchRecomputeMode,
  setRecomputeMode,
  type RecomputeView,
  type RecomputeLevelStat,
  type RecomputeModeState,
  type RecomputeSummary,
} from '@/api'
import { fmtTime } from '@/utils/format'

const loading = ref(false)
const statsLoading = ref(false)
const rows = ref<RecomputeView[]>([])
const total = ref(0)
const summary = ref<RecomputeSummary | null>(null)

/**
 * 引擎侧静默失效告警。
 *
 * ⚠️ 这里**不是**再做一次判据：`status` 字段由服务端给出（见 admin
 * summary 的 EngineTotals）。前端只负责读它、决定要不要弹、以及弹什么颜色。
 *
 * 而且只能是**读** —— 判据在后端，不在前端另写一套，
 * 否则两处各写一遍、必然漂移 —— 这是 §9.2 专门留的钩子。
 */
const engineStatus = computed(() => summary.value?.engine?.status ?? '')
const engineAlertType = computed<'error' | 'warning' | null>(() => {
  const s = engineStatus.value
  if (s.startsWith('验真覆盖为 0')) return 'error'
  if (s.includes('cannot_run 偏高')) return 'warning'
  return null
})

const userId = ref<number | undefined>(undefined)
const levelId = ref<number | undefined>(undefined)
const onlyMismatch = ref(true)
const hours = ref(24)

/** 不符字段 → 中文。未知字段回落到原值，不要吞掉。 */
const FIELD_LABEL: Record<string, string> = {
  score: '分数',
  kills: '击杀',
  duration: '时长',
  hash: '重放哈希',
  leaked: '漏怪',
  replay_skills: '技能段 S',
}
function fieldLabel(f: string): string {
  return FIELD_LABEL[f] ?? f
}

/**
 * 档位标签。
 *
 * ⚠️ `mode` 与 `rejected` 是**两列**，必须分别看：
 * 熔断可以在 enforce 档把单次处置降级，此时台账里就是
 * 「mode=enforce 但 rejected=false」。合成一列就看不出这个差异，
 * 而「为什么这批没被拒」正是运营的第一个问题。
 */
function modeTag(row: RecomputeView): { text: string; type: 'info' | 'warning' | 'danger' } {
  if (row.rejected) return { text: '已拒发', type: 'danger' }
  if (row.mode === 'enforce') {
    // enforce 档却没拒 —— 只可能因为熔断把这次降级了
    return { text: 'enforce（本次降级）', type: 'warning' }
  }
  return { text: row.mode, type: 'info' }
}

/** 重算值缺席时显示「—」而不是 0。见 `RecomputeSide` 的注释。 */
function recomputedText(row: RecomputeView): string {
  if (!row.recomputed) return '—'
  const r = row.recomputed
  return `${r.score} / ${r.kills} / ${r.duration_ms}ms`
}

function reportedText(row: RecomputeView): string {
  const r = row.reported
  return `${r.score} / ${r.kills} / ${r.duration_ms}ms`
}

/** 百分比。千分比整数除法，避免浮点尾数（1200‰ 显示成 1199.99%）。 */
function pct(permille: number): string {
  return `${(permille / 10).toFixed(1)}%`
}

/**
 * 不符率的严重度。
 *
 * 阈值直接取服务端下发的 `breaker_rate_permille`，**不在前端另设一个** ——
 * 两处各写一个数字的话，改了服务端那个前端就会开始
 * 「显示红色但其实没熔断」，或者反过来。
 */
function rateType(permille: number, sum: RecomputeSummary | null): 'success' | 'warning' | 'danger' {
  if (!sum) return 'info' as never
  const limit = sum.breaker_rate_permille
  if (permille >= limit) return 'danger'
  if (permille >= Math.floor(limit / 2)) return 'warning'
  return 'success'
}

/**
 * 「漂移 or 作弊」的提示语。
 *
 * ⚠️ 这是**提示**不是判定 —— 判定需要人工看具体行。
 *   措辞刻意不给结论，因为给错的结论比不给更糟：
 *   运营可能照着「这是引擎 bug」去回滚，而实际是有人在作弊。
 */
function shapeHint(level: RecomputeLevelStat): string {
  if (level.total === 0 || level.mismatched === 0) return ''
  const rate = level.mismatched / level.total
  if (rate >= 0.5) {
    return level.max_user_mismatches >= level.mismatched
      ? '少数几个号占满全部不符 → 像作弊'
      : '几乎所有玩家都不符 → 像引擎漂移'
  }
  if (level.max_user_mismatches === level.mismatched && level.mismatched > 0) {
    return '不符全部来自同一个号 → 像作弊'
  }
  return ''
}

async function loadStats() {
  statsLoading.value = true
  try {
    summary.value = await fetchRecomputeStats(hours.value)
  } catch (e) {
    // 总账拿不到时**不能**把列表一起阻断：列表是逐条证据，
    // 总账只是个概览。反过来（只有概览没证据）才是没用的。
    ElMessage.warning(`总账加载失败：${(e as Error).message}`)
    summary.value = null
  } finally {
    statsLoading.value = false
  }
}

/**
 * 重算档位（2026-10-21 P3-3）。
 *
 * 此前档位**没有任何入口** —— 运营知道有「档位」这回事，
 * 却既看不到当前是哪个档、也切不了。而把 enforce 改成 off
 * 等于**关掉反作弊校验**，所以它必须是一次显式的、留痕的动作。
 */
const modeState = ref<RecomputeModeState | null>(null)
const modeSaving = ref(false)

/** 档位的中文名 + 各自意味着什么。 */
const MODE_LABEL: Record<string, { text: string; desc: string }> = {
  off: { text: '关闭', desc: '完全不重算、不记录台账 —— 等于关掉反作弊校验。' },
  shadow: { text: '影子', desc: '照常重算并记录，但**不拒发**奖励（只观察，不处置）。' },
  enforce: { text: '强制', desc: '重算且不符即拒发奖励（正常的防护档）。' },
}

async function loadMode() {
  try {
    modeState.value = await fetchRecomputeMode()
  } catch (e) {
    // 拿不到档位**不阻断**审计页：台账才是这个页面的主内容。
    ElMessage.warning(`档位读取失败：${(e as Error).message}`)
    modeState.value = null
  }
}

async function switchMode(next: string) {
  if (modeSaving.value || next === modeState.value?.mode) return
  const info = MODE_LABEL[next]
  try {
    await ElMessageBox.confirm(
      `把重算档位切到「${info?.text ?? next}」？${info?.desc ?? ''}` +
        (next === 'off' ? '\n\n**这等于关掉反作弊校验。**' : ''),
      '切换重算档位',
      { type: next === 'off' ? 'error' : 'warning' },
    )
  } catch {
    return // 取消
  }
  modeSaving.value = true
  try {
    modeState.value = await setRecomputeMode(next)
    ElMessage.success(`档位已切到 ${next}`)
  } catch (e) {
    ElMessage.error(`切档失败：${(e as Error).message}`)
  } finally {
    modeSaving.value = false
  }
}

async function load() {
  loading.value = true
  try {
    const res = await fetchRecomputes({
      user_id: userId.value,
      level_id: levelId.value,
      only_mismatch: onlyMismatch.value || undefined,
      limit: 50,
    })
    rows.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`重算台账加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

function reloadAll() {
  // 两个请求并发：任一失败各自的提示里已经写明了后果，
  // 所以这里不串行 —— 串行会让一次总账超时把证据列表也拖住。
  return Promise.all([load(), loadStats(), loadMode()])
}

onMounted(reloadAll)

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 不能直接传给形参是具体类型的函数（TS2345），所以在调用点集中断言一次。
 *
 * cast 编译后被擦除，**运行时行为与逐个 `any` 完全相同**。
 *
 * ⚠️ 与 `BattlesView.asRow` 同源：那个文件曾记录过 —
 *   `components.d.ts` 缺失时 `el-table` 的插槽行是 `any`，
 *   于是几十处调用**全部静默通过类型检查**。
 *   断言让它们重新受检；但它只断言**形状**，
 *   字段名本身拼错仍要靠运行时的 `undefined` 暴露。
 */
const asRow = (r: unknown) => r as RecomputeView
const asLevel = (r: unknown) => r as RecomputeLevelStat
</script>

<template>
  <div>
    <!--
      重算档位（2026-10-21 P3-3）：此前它**不可读也不可改** ——
      运营知道有「档位」这回事，却看不到当前是哪个档、也切不了。

      ⚠️ `mode` 与 `configured_mode` 不一致 = 被熔断降级了
      （此时 enforce 该拒发的奖励正在**放行**）—— 这是最该被看见的状态。
    -->
    <el-card v-if="modeState" shadow="never" style="margin-bottom: 12px">
      <div class="toolbar">
        <span class="toolbar-label">重算档位</span>
        <el-tag
          :type="modeState.demoted ? 'warning' : modeState.mode === 'enforce' ? 'success' : 'info'"
          size="small"
        >
          生效中：{{ modeState.mode }}
        </el-tag>
        <el-tag v-if="modeState.demoted" type="warning" size="small">
          配置为 {{ modeState.configured_mode }}（已被熔断降级）
        </el-tag>
        <el-tag v-else-if="modeState.configured_mode !== modeState.mode" type="info" size="small">
          配置为 {{ modeState.configured_mode }}
        </el-tag>
        <span class="toolbar-spacer" />
        <el-button
          v-for="m in ['enforce', 'shadow', 'off']"
          :key="m"
          size="small"
          :disabled="modeSaving || modeState.mode === m"
          :type="modeState.mode === m ? 'primary' : 'default'"
          @click="switchMode(m)"
        >
          切到 {{ m }}
        </el-button>
      </div>
      <p class="muted" style="margin: 0">
        {{ MODE_LABEL[modeState.mode]?.desc ?? '' }}
        <template v-if="modeState.demoted">
          <br />降级原因：{{ modeState.demoted_reason || '（未记录）' }}
        </template>
      </p>
    </el-card>

    <!--
      闸门状态放在最上面，且用 alert 而不是普通卡片：
      这是唯一一个「数字正常、行为却已经变了」的状态，
      混在一堆统计卡片里就等于没有。
    -->
    <el-alert
      v-if="summary?.demoted"
      type="warning"
      show-icon
      :closable="false"
      style="margin-bottom: 12px"
      title="熔断已触发：引擎侧不再拒发奖励"
      :description="`配置档位是 ${summary.configured_mode}，但近 ${hours} 小时不符率超阈值后已自动降级为 ${summary.mode}。本该拒发的上报正在被放行。原因：${summary.demoted_reason}。修复后需人工把 SETTLE_RECOMPUTE_MODE 改回 enforce —— 不会自动升档。`"
    />

    <!--
      引擎侧「静默失效」告警。
      这个字段比上面的融断更危险：融断时系统**还在重算、仍在记录，
      只是不再拒发**，而这里是在说「校验根本没跑过」—— 台账里也是 0 行，
      但它们的含义完全不同。随时可能就这样静默地开着防护下去。
    -->
    <el-alert
      v-if="engineAlertType"
      :type="engineAlertType === 'error' ? 'error' : 'warning'"
      show-icon
      :closable="false"
      style="margin-bottom: 12px"
      :title="engineAlertType === 'error' ? '重算从未成功：防护其实是死的' : '重算长期算不出来：被放行的理由不是没问题'"
      :description="`${engineStatus}。 verified=${summary?.engine?.verified ?? 0} / mismatch=${summary?.engine?.mismatch ?? 0} / cannot_run=${summary?.engine?.cannot_run ?? 0} / skipped=${summary?.engine?.skipped ?? 0}（从进程启动累计；settled_total=${summary?.engine?.settled_total ?? 0}）`"
    />

    <div class="page-title">结算重算审计</div>
    <p class="page-subtitle">
      服务端重算与上报不符的记录。判读要点：<strong>不符集中在少数关卡</strong>说明引擎某段算错（回滚），
      <strong>不符只来自一个号</strong>说明玩家在作弊（封号）—— 两者的处置完全相反，不要混为一谈。
    </p>

    <el-card shadow="never" class="mb">
      <div class="toolbar">
        <span class="muted">统计窗口</span>
        <el-select v-model="hours" size="small" style="width: 130px" @change="loadStats">
          <el-option :value="1" label="最近 1 小时" />
          <el-option :value="24" label="最近 24 小时" />
          <el-option :value="168" label="最近 7 天" />
          <el-option :value="720" label="最近 30 天" />
        </el-select>
        <el-button size="small" :loading="statsLoading" @click="loadStats">刷新总账</el-button>
      </div>

      <div v-loading="statsLoading" class="stat-row">
        <div class="stat">
          <div class="stat-label">生效档位</div>
          <div class="stat-value">
            <el-tag v-if="summary" :type="summary.mode === 'enforce' ? 'success' : 'warning'" size="small">
              {{ summary.mode }}
            </el-tag>
            <span v-else class="muted">—</span>
          </div>
          <div v-if="summary && summary.configured_mode !== summary.mode" class="stat-note danger">
            配置值 {{ summary.configured_mode }}，已被降级
          </div>
        </div>
        <div class="stat">
          <div class="stat-label">重算总数</div>
          <div class="stat-value">{{ summary?.totals.total ?? '—' }}</div>
          <div class="stat-note muted">窗口自 {{ summary?.since ?? '—' }}</div>
        </div>
        <div class="stat">
          <div class="stat-label">不符数</div>
          <div class="stat-value danger">{{ summary?.totals.mismatched ?? '—' }}</div>
          <div class="stat-note muted">阈值 {{ summary?.breaker_rate_permille ?? '—' }}‰</div>
        </div>
        <div class="stat">
          <div class="stat-label">不符率</div>
          <div class="stat-value">
            <el-tag
              v-if="summary"
              :type="rateType(summary.totals.rate_permille, summary)"
              size="small"
            >
              {{ pct(summary.totals.rate_permille) }}
            </el-tag>
            <span v-else class="muted">—</span>
          </div>
          <div class="stat-note muted">
            样本 {{ summary?.totals.total ?? 0 }} / 熔断需 {{ summary?.breaker_min_samples ?? '—' }}
          </div>
        </div>
        <div class="stat">
          <div class="stat-label">已拒发</div>
          <div class="stat-value">{{ summary?.totals.rejected ?? '—' }}</div>
          <div class="stat-note muted">与不符数不等时，先看是不是被熔断了</div>
        </div>
        <div class="stat">
          <div class="stat-label">验真覆盖</div>
          <div class="stat-value">
            <el-tag
              v-if="summary?.engine"
              :type="engineAlertType === 'error' ? 'danger' : engineAlertType === 'warning' ? 'warning' : 'success'"
              size="small"
            >
              {{ summary.engine.settled_total === 0 ? '—' : Math.round(summary.engine.verified_permille / 10) + '%' }}
            </el-tag>
            <span v-else class="muted">—</span>
          </div>
          <div class="stat-note muted">
            验真 {{ summary?.engine?.verified ?? 0 }} · 不符 {{ summary?.engine?.mismatch ?? 0 }} ·
            算不出来 {{ summary?.engine?.cannot_run ?? 0 }} · 老客户端跳过 {{ summary?.engine?.skipped ?? 0 }} ·
            至今累计 {{ summary?.engine?.settled_total ?? 0 }} 局（进程内，重部署清零）
          </div>
        </div>
      </div>

      <div v-if="summary?.levels.length" class="section-title">按关卡分布</div>
      <el-table v-if="summary?.levels.length" :data="summary.levels" size="small" stripe>
        <el-table-column prop="level_id" label="关卡" width="70" />
        <el-table-column prop="total" label="重算数" width="90" />
        <el-table-column prop="mismatched" label="不符数" width="90" />
        <el-table-column label="不符率" width="110">
          <template #default="{ row }">
            <el-tag :type="rateType(asLevel(row).rate_permille, summary!)" size="small">
              {{ pct(asLevel(row).rate_permille) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="单号最高不符" width="120">
          <template #default="{ row }">{{ asLevel(row).max_user_mismatches }}</template>
        </el-table-column>
        <el-table-column label="形态提示" min-width="220">
          <template #default="{ row }">
            <span :class="{ danger: shapeHint(asLevel(row)) }">{{ shapeHint(asLevel(row)) || '—' }}</span>
          </template>
        </el-table-column>
      </el-table>
      <div v-else class="muted" style="padding: 12px 0">窗口内没有重算记录。</div>
    </el-card>

    <el-card shadow="never">
      <div class="toolbar">
        <el-input-number
          v-model="userId"
          :controls="false"
          placeholder="用户 ID"
          style="width: 120px"
        />
        <el-input-number
          v-model="levelId"
          :controls="false"
          placeholder="关卡 ID"
          style="width: 120px"
        />
        <el-button @click="load">筛选</el-button>
        <el-checkbox v-model="onlyMismatch" style="margin-left: 12px">只看不符</el-checkbox>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ total }} 条</span>
      </div>

      <el-table v-loading="loading" :data="rows" size="small" stripe>
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column label="战报" width="70">
          <!--
            battle_id=0 是「写战报之前就被拒」，没有战报可查。
            显示 0 会让人去查一条不存在的战报，所以显示「—」。
          -->
          <template #default="{ row }">
            <span v-if="asRow(row).battle_id === 0" class="muted" title="结算在写战报之前就被拒，没有战报 id">
              —
            </span>
            <span v-else>{{ asRow(row).battle_id }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="user_id" label="用户" width="80" />
        <el-table-column prop="level_id" label="关卡" width="70" />
        <el-table-column label="不符字段" min-width="140">
          <template #default="{ row }">
            <el-tag
              v-for="f in asRow(row).mismatched_fields"
              :key="f"
              type="danger"
              size="small"
              effect="plain"
              style="margin-right: 4px"
            >
              {{ fieldLabel(f) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="上报（分数/击杀/时长）" min-width="160">
          <template #default="{ row }">{{ reportedText(asRow(row)) }}</template>
        </el-table-column>
        <el-table-column label="重算" min-width="160">
          <template #default="{ row }">
            <span :class="{ muted: !asRow(row).recomputed }">{{ recomputedText(asRow(row)) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="处置" width="150">
          <template #default="{ row }">
            <el-tag :type="modeTag(asRow(row)).type" size="small">{{ modeTag(asRow(row)).text }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="engine_version" label="引擎版本" width="120" />
        <el-table-column label="时间" width="155">
          <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<style scoped>
.mb {
  margin-bottom: 12px;
}
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
.danger {
  color: #ff6b35;
  font-weight: 600;
}
.stat-row {
  display: flex;
  flex-wrap: wrap;
  gap: 28px;
  margin-bottom: 4px;
}
.stat {
  min-width: 120px;
}
.stat-label {
  font-size: 12px;
  color: var(--lyp-muted);
}
.stat-value {
  font-size: 20px;
  font-weight: 600;
  line-height: 1.4;
}
.stat-note {
  font-size: 11px;
}
.section-title {
  margin: 16px 0 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--lyp-muted);
}
</style>
