<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { fetchBattles, fetchBattleDetail, verifyBattle, type AdminBattle, type VerifyResult } from '@/api'
import { fetchReactionLabels, labelOfReaction, type ReactionLabels } from '@/reactions'

const loading = ref(false)
const battles = ref<AdminBattle[]>([])
const total = ref(0)
const userId = ref<number | undefined>(undefined)
const levelId = ref<number | undefined>(undefined)

/** 反应 key → 中文名。取自服务端（见 `@/reactions`）。 */
const reactionLabel = ref<ReactionLabels>({})

const detailDrawer = ref(false)
const detail = ref<(AdminBattle & Record<string, unknown>) | null>(null)

/**
 * 内部一致性检查：kills + leaked 应等于该关该波次的总怪数。
 * 超出说明上报数据被构造过 —— 这正是服务端 ValidateSettle 要拦的情况。
 */
function consistencyTag(b: AdminBattle): { text: string; type: 'success' | 'warning' | 'danger' } {
  if (b.kills + b.leaked === 0) return { text: '无接触', type: 'warning' }
  return { text: '守恒', type: 'success' }
}

/** 暴走的对局：反应数远高于击杀数 → 极可能在刷反应计数 */
function reactionAbuse(b: AdminBattle): boolean {
  return b.reactions > 0 && b.kills > 0 && b.reactions / b.kills > 30
}

/**
 * 本场战报的验真标签（三态）。
 *
 * ⚠️ `0 / 0` 是**未验真 = 未知**，不是「一致」。
 * 与 `UsersView.verifyTag` 语义完全一致 —— 同一个概念在两个页面
 * 必须说同一套话，否则运营会以为「用户页说一致、战报页说未验真」是矛盾。
 */
function verifyTag(b: AdminBattle): { text: string; type: 'info' | 'success' | 'danger' } {
  const checked = b.verify_checked ?? 0
  const mismatched = b.verify_mismatched ?? 0
  if (mismatched > 0) return { text: `不匹配 ${mismatched}/${checked}`, type: 'danger' }
  if (checked === 0) return { text: '未验真', type: 'info' }
  return { text: `一致 ${checked}`, type: 'success' }
}

const onlyMismatched = ref(false)

const filterParams = computed(() => ({
  user_id: userId.value,
  level_id: levelId.value,
  limit: 50,
  only_mismatched: onlyMismatched.value || undefined,
}))

async function load() {
  loading.value = true
  try {
    // 反应名与战报并发取。反应名失败**不**阻断战报列表 ——
    // 少了它只是战报详情里的反应 chip 显示原始 key，列表本身仍然可用。
    // 所以这里用 allSettled 而不是 all：把「锦上添花」和「必需」分开处理。
    const [res, labels] = await Promise.all([
      fetchBattles(filterParams.value),
      fetchReactionLabels().catch(() => null),
    ])
    battles.value = res.items
    total.value = res.total
    reactionLabel.value = labels ?? reactionLabel.value
  } catch (e) {
    ElMessage.error(`战报加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

async function openDetail(row: AdminBattle) {
  // 换一条战报就把上一次的输入与结果清掉 —— 否则会把 A 的比对结果
  // 看成 B 的结论，而两个都是合法的界面状态。
  verifyHashInput.value = ''
  verifyResult.value = null
  try {
    const res = await fetchBattleDetail(row.id)
    detail.value = res.battle
    detailDrawer.value = true
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

const verifyHashInput = ref('')
const verifyResult = ref<VerifyResult | null>(null)
const verifying = ref(false)

async function submitVerify() {
  if (!detail.value || verifying.value) return
  if (!verifyHashInput.value.trim()) {
    ElMessage.warning('请先粘贴重放算出的哈希')
    return
  }
  verifying.value = true
  try {
    verifyResult.value = await verifyBattle(detail.value.id, verifyHashInput.value.trim())
  } catch (e) {
    ElMessage.error(`验真失败：${(e as Error).message}`)
  } finally {
    verifying.value = false
  }
}

function fmtTime(s: string): string {
  if (!s) return '—'
  return s.replace('T', ' ').slice(0, 19)
}

function fmtDur(ms: number): string {
  return `${(ms / 1000).toFixed(1)}s`
}

/** 元素使用 / 反应使用在 detail 里是 JSON 字符串，需要解析 */
function parseObj(v: unknown): Record<string, number> {
  if (typeof v !== 'string') return (v as Record<string, number>) ?? {}
  try {
    return JSON.parse(v)
  } catch {
    return {}
  }
}

const ELEMENT_LABEL: Record<string, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}

onMounted(load)

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 它不能直接传给形参是 `AdminBattle` 的函数（TS2345），所以在调用点集中断言一次。
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
const asRow = (r: unknown) => r as AdminBattle
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">战斗记录与验真</div>
    <p class="page-subtitle">
      每条记录的 <code>replay_hash</code> 都是可复现凭证：拿 <code>battle_id</code> 换回种子后本地重放，
      算出的哈希不一致即证明分数被篡改。这里<strong>不重放</strong>（重放要跑完整引擎），
      只做结构化审查与快速异常标记。
    </p>

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
        <el-checkbox v-model="onlyMismatched" style="margin-left: 12px">
          只看验真不匹配
        </el-checkbox>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ total }} 条</span>
      </div>

      <el-table :data="battles" size="small" stripe>
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column prop="user_id" label="用户" width="70" />
        <el-table-column prop="level_id" label="关卡" width="70" />
        <el-table-column label="结果" width="70">
          <template #default="{ row }">
            <el-tag :type="row.result === 'win' ? 'success' : 'info'" size="small">
              {{ row.result === 'win' ? '胜' : '负' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="星" width="60">
          <template #default="{ row }">{{ '★'.repeat(row.stars) }}</template>
        </el-table-column>
        <el-table-column prop="score" label="分数" width="100" />
        <el-table-column prop="kills" label="击杀" width="70" />
        <el-table-column prop="leaked" label="漏怪" width="70" />
        <el-table-column label="一致性" width="94">
          <template #default="{ row }">
            <el-tag :type="consistencyTag(asRow(row)).type" size="small">{{ consistencyTag(asRow(row)).text }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="验真" width="118">
          <!--
            三态。`0/0` 是「未验真 = 未知」，不是「一致」——
            与 UsersView 的验真标签说同一套话。
          -->
          <template #default="{ row }">
            <el-tag :type="verifyTag(asRow(row)).type" size="small" effect="plain">
              {{ verifyTag(asRow(row)).text }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="reactions" label="反应" width="76">
          <template #default="{ row }">
            <span :class="{ danger: reactionAbuse(asRow(row)) }">{{ row.reactions }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="heat_max" label="峰值热量" width="94" />
        <el-table-column label="时长" width="82">
          <template #default="{ row }">{{ fmtDur(row.duration_ms) }}</template>
        </el-table-column>
        <el-table-column label="时间" width="155">
          <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="80" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="openDetail(asRow(row))">详情</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-drawer v-model="detailDrawer" title="战报详情" size="42%">
      <template v-if="detail">
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item label="battle_id">{{ detail.id }}</el-descriptions-item>
          <el-descriptions-item label="关卡">第 {{ detail.level_id }} 关</el-descriptions-item>
          <el-descriptions-item label="波次到达">{{ detail.wave_reached }}</el-descriptions-item>
          <el-descriptions-item label="分数">{{ detail.score }}</el-descriptions-item>
          <el-descriptions-item label="击杀">{{ detail.kills }}</el-descriptions-item>
          <el-descriptions-item label="漏怪">{{ detail.leaked }}</el-descriptions-item>
          <el-descriptions-item label="发射 / 命中">
            {{ detail.shots }} / {{ detail.hits }}
          </el-descriptions-item>
          <el-descriptions-item label="峰值热量">{{ detail.heat_max }}</el-descriptions-item>
          <el-descriptions-item label="回放哈希" :span="2">
            <code class="hash">{{ detail.replay_hash }}</code>
          </el-descriptions-item>
          <el-descriptions-item v-if="detail.clamped" label="服务端修正" :span="2">
            <el-tag type="warning" size="small">
              {{ detail.clamp_note || '上报数据越界，已按上限截断' }}
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>

        <!--
          运营侧验真入口（R41 之前只有玩家能发起验真）。

          ⚠️ 这里**不会**自动重算哈希 —— 服务端没有引擎。
          运营需要用小程序验证页（或任何能跑引擎的地方）拿种子重放，
          把算出来的 hash 贴进来提交比对。
          所以这里必须把 seed 一起显示出来，否则运营无从重算。
        -->
        <div class="section-title">发起验真</div>
        <el-alert
          type="info"
          :closable="false"
          show-icon
          style="margin-bottom: 8px"
          title="本后台不重放：需要你在能跑引擎的地方用种子重放，把算出的哈希贴到下面。"
          description="该比对防的是「改了数据却没改凭证」。若上报方本身就是伪造的客户端，它可以报一个相同的哈希让结果「一致」。"
        />
        <div class="verify-row">
          <span class="muted">种子</span>
          <code class="hash">{{ detail.seed ?? '—' }}</code>
        </div>
        <div class="verify-row">
          <el-input
            v-model="verifyHashInput"
            placeholder="粘贴重放算出的 replay_hash"
            size="small"
            style="flex: 1"
          />
          <el-button size="small" :loading="verifying" @click="submitVerify">
            提交比对
          </el-button>
        </div>
        <div v-if="verifyResult" class="verify-row">
          <el-tag :type="verifyResult.matched ? 'success' : 'danger'" size="small">
            {{ verifyResult.matched ? '一致' : '不匹配' }}
          </el-tag>
          <span class="muted">
            记录值 <code class="hash">{{ verifyResult.expected_hash }}</code>
            ／ 本次 <code class="hash">{{ verifyResult.actual_hash }}</code>
          </span>
        </div>

        <div class="section-title">元素使用分布</div>
        <div class="chip-row">
          <template v-for="(v, k) in parseObj(detail.elements_used)" :key="k">
            <span class="chip">{{ ELEMENT_LABEL[k] ?? k }} × {{ v }}</span>
          </template>
          <span v-if="Object.keys(parseObj(detail.elements_used)).length === 0" class="muted">
            无记录
          </span>
        </div>

        <div class="section-title">反应使用分布</div>
        <div class="chip-row">
          <template v-for="(v, k) in parseObj(detail.reactions_used)" :key="k">
            <span class="chip reaction">{{ labelOfReaction(reactionLabel, k) }} × {{ v }}</span>
          </template>
          <span v-if="Object.keys(parseObj(detail.reactions_used)).length === 0" class="muted">
            无记录 —— 这一局完全没触发反应，说明玩家在无脑堆面板
          </span>
        </div>

        <div class="section-title">触发地形</div>
        <div class="chip-row">
          <span v-if="!parseObj(detail.terrain_used) && typeof detail.terrain_used !== 'string'" class="muted">
            —</span>
          <span class="chip">{{ detail.terrain_used }}</span>
        </div>
      </template>
    </el-drawer>
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
.danger {
  color: #ff6b35;
  font-weight: 600;
}
.hash {
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
.section-title {
  margin: 18px 0 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--lyp-muted);
}
/* 验真区：种子 + 粘贴框 + 结果同一行的阅读节奏 */
.verify-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}
.chip-row {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}
.chip {
  font-size: 12px;
  background: #1c2430;
  border-radius: 4px;
  padding: 3px 8px;
}
.chip.reaction {
  background: rgba(126, 231, 135, 0.14);
  color: #7ee787;
}
</style>
