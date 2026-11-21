<script setup lang="ts">
/**
 * AI 运营洞察卡（AI_VISUAL_SYSTEM.md §1A.3）。
 * PC 看板可观测的 AI 元素：诚实标注「规则引擎」，结论由 buildAiInsight 确定性生成。
 * 令牌全部走 --lyp-ai-*，不硬编码紫色；加载态复用共享骨架类。
 */
import type { AiInsight } from '@/utils/aiInsight'

defineProps<{
  insight: AiInsight | null
  loading: boolean
}>()
</script>

<template>
  <section
    id="ai-insight-card"
    class="ai-insight-card glass-surface"
    :aria-busy="loading ? 'true' : 'false'"
    aria-label="AI 运营洞察"
  >
    <template v-if="loading">
      <div class="ai-loading">
        <div class="ai-loading-mark" aria-hidden="true"><span class="skeleton-bar" style="width: 60px"></span></div>
        <div class="ai-loading-lines">
          <span class="skeleton-bar" style="width: 70%"></span>
          <span class="skeleton-bar" style="width: 45%"></span>
        </div>
      </div>
    </template>

    <template v-else-if="insight && insight.kind !== 'empty'">
      <div class="ai-main">
        <div class="ai-eyebrow">
          <span class="ai-badge">AI</span>
          <span class="ai-title">AI 运营洞察</span>
          <span class="ai-source muted">规则引擎 · 数据截至 {{ insight.asOf }}</span>
        </div>
        <p class="ai-headline">{{ insight.headline }}</p>
        <p class="ai-evidence muted">{{ insight.evidence }}</p>
      </div>
      <div class="ai-side">
        <router-link v-if="insight.ctaPath" :to="insight.ctaPath" class="ai-cta">
          {{ insight.ctaLabel }} <span aria-hidden="true">→</span>
        </router-link>
      </div>
    </template>

    <template v-else>
      <div class="ai-empty">
        <span class="ai-badge">AI</span>
        <span>{{ insight?.headline ?? '数据不足：AI 洞察暂无可用数据' }}</span>
      </div>
    </template>
  </section>
</template>

<style scoped>
.ai-insight-card {
  position: relative;
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: var(--lyp-gap-4);
  align-items: center;
  min-height: 96px;
  max-height: 112px;
  margin-bottom: var(--lyp-gap-3);
  padding: 18px 20px;
  border-radius: var(--lyp-radius);
  border-color: var(--lyp-ai-halo);
  box-shadow: var(--lyp-ai-glow), var(--lyp-shadow);
  overflow: hidden;
}
/* 左侧一道动能紫竖光，AI 元素的唯一标识色 */
.ai-insight-card::before {
  content: '';
  position: absolute;
  left: 0; top: 12px; bottom: 12px;
  width: 3px;
  border-radius: 3px;
  background: var(--lyp-ai-accent);
}
.ai-main { min-width: 0; }
.ai-eyebrow {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  font-size: var(--lyp-fs-xs);
}
.ai-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 1px 7px;
  border-radius: 5px;
  background: var(--lyp-ai-halo);
  color: var(--lyp-ai-accent);
  font-weight: 700;
  letter-spacing: 0.08em;
}
.ai-title { font-weight: 600; color: var(--lyp-text); }
.ai-source { color: var(--lyp-muted); }
.ai-headline {
  margin: 0 0 6px;
  font-size: var(--lyp-fs-md);
  font-weight: 600;
  line-height: 1.45;
  color: var(--lyp-text);
}
.ai-evidence {
  margin: 0;
  font-size: var(--lyp-fs-sm);
  line-height: 1.5;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.ai-side { display: flex; justify-content: flex-end; }
.ai-cta {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border-radius: var(--lyp-radius-sm);
  background: var(--lyp-ai-halo);
  border: 1px solid var(--lyp-ai-accent);
  color: var(--lyp-ai-accent);
  font-size: var(--lyp-fs-sm);
  font-weight: 600;
  text-decoration: none;
  white-space: nowrap;
  transition: transform var(--lyp-motion-fast) var(--lyp-motion-ease);
}
.ai-cta:hover { transform: translateY(-1px); }
.ai-empty {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--lyp-muted);
  font-size: var(--lyp-fs-base);
}
.ai-loading { display: flex; align-items: center; gap: 20px; }
.ai-loading-lines { display: flex; flex-direction: column; gap: 10px; flex: 1; }
@media (max-width: 900px) {
  .ai-insight-card { grid-template-columns: 1fr; max-height: none; }
  .ai-side { justify-content: flex-start; }
}
</style>
