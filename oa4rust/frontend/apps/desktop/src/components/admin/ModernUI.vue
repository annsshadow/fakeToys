<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<template>
  <div class="modern-page ai-page-b5 ai-b5-surface">
    <slot />
  </div>
</template>
<style scoped>
.ai-b5-surface {
  background: var(--ai-surface-gradient);
  border-radius: var(--ai-radius-lg, 12px);
}
</style>
