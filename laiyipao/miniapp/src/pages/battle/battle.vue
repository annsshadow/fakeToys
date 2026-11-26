<template>
  <view class="page-root battle-page" @click="unlockSfx()">
    <!-- 画布：整个战斗画面 -->
    <canvas
      id="battle-canvas"
      canvas-id="battle-canvas"
      type="2d"
      class="battle-canvas"
      :style="{ width: canvasW + 'px', height: canvasH + 'px' }"
    />

    <!-- 顶部信息层 -->
    <view class="overlay-top" :style="{ paddingTop: statusBarPad }">
      <view class="hud-row">
        <view class="hp-box">
          <view class="hp-line">
            <text class="hp-label">防线</text>
            <text class="hp-value mono">{{ hpText }}</text>
          </view>
          <view class="hp-bar">
            <view class="hp-fill" :style="{ width: hpPct + '%' }" />
          </view>
          <!-- 第六批 R5：HUD 常驻当前关卡名，切波/结算前都知道自己在打哪一关 -->
          <text v-if="levelTitle" class="hp-level">{{ levelTitle }}</text>
        </view>
        <view class="hud-spacer" />
        <view class="hud-pill"><text class="wave-text">波次 {{ waveText }}</text></view>
        <view class="hud-pill"><text class="kill-text">击杀 {{ engine?.kills ?? 0 }}</text></view>
        <view v-if="liveStarGap" class="hud-pill"><text class="star-gap-text">{{ liveStarGap }}</text></view>
        <!-- R8 缺口3：免钳分倒计时与临近锚点提示。纯 HUD 胶囊，不遮挡战场；
             数值来自关卡波次 + 战斗时钟，绝不在页面里写死 60s/75s。
             服务端是「快于锚点才裁」，锚点前显示钳分中，不能谎称保分窗口。 -->
        <view
          v-if="timeBudget"
          class="hud-pill time-budget"
          :class="{ warn: timeBudget.warning, clamped: timeBudget.clamped }"
        >
          <text class="time-budget-text">{{ timeBudget.text }}</text>
        </view>
        <!-- R27 实时波次推进：只读快照派生，紧凑单行不挤压既有胶囊 -->
        <view v-if="r27Progress" class="r27-wave-progress" :data-r27-progress="r27Progress.waveIndex">
          <view v-if="r27Progress.empty" class="r27-progress-body">
            <text class="r27-progress-empty">本波暂无敌人进度数据</text>
          </view>
          <view v-else class="r27-progress-body">
            <view class="r27-progress-track">
              <view class="r27-progress-fill" :style="{ width: r27Progress.clearedPct + '%' }" />
            </view>
            <text class="r27-progress-text">本波 {{ r27Progress.resolved }}/{{ r27Progress.waveTotal }} · {{ r27Progress.clearedPct.toFixed(1) }}%</text>
          </view>
        </view>
      </view>
      <!-- R27 AI 波次推进解读：同源只读快照；只读展示不拦截点击 -->
      <view v-if="r27Progress && !r27Progress.empty" class="r27-ai-note">
        <text class="r27-ai-title">AI · 波次推进</text>
        <text class="r27-ai-conclusion">{{ r27Progress.ai }}</text>
        <text class="r27-ai-basis">依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash</text>
        <text class="r27-ai-cta">继续清理场上敌人 →</text>
        <text class="r27-ai-honesty">辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准</text>
      </view>
      <!-- R9：最近反应记录。与星级/钳分胶囊同属顶部浮层，不拦截点击。 -->
      <view v-if="recentReactions.length" class="reaction-feed" aria-live="polite">
        <view v-for="r in recentReactions" :key="r.id" class="reaction-line">
          <view class="reaction-line-main">
            <text class="reaction-name">{{ r.pair }} → {{ r.name }}</text>
            <text class="reaction-damage">{{ r.resisted ? '抗性削减 ' : '' }}-{{ r.damage }}</text>
          </view>
          <text class="r28-attribution" :data-r28-wave="r.available ? r.wave : 'unknown'" :data-r28-enemy="r.available ? r.enemy_uid : 'unknown'">{{ r.attribution }}</text>
          <text class="reaction-cue">AI {{ r.cue }}</text>
        </view>
        <!-- R28：每条反应附波次/敌人归因，截断或不匹配诚实标注；AI 说明不拦截点击 -->
        <view class="r28-feed-ai">
          <text class="r28-feed-ai-title">AI · 反应归因</text>
          <text class="r28-feed-ai-conclusion">{{ r28Ai.conclusion }}</text>
          <text class="r28-feed-ai-basis">依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用</text>
          <text class="r28-feed-ai-honesty">辅助归因非权威判据，权威以 replay_hash 为准</text>
          <text class="r28-feed-cta" @click.stop="openReactionTimeline">查看完整反应时间线 →</text>
        </view>
      </view>
      <!-- R24 实时元素层与反应触发预判：只读快照派生，纯展示不拦截点击 -->
      <view class="r24-element-hud">
        <view v-if="r24ElementRows.length" class="r24-hud-rows">
          <view v-for="row in r24ElementRows" :key="row.uid" class="r24-enemy-row" :data-r24-uid="row.uid">
            <text class="r24-enemy-name">{{ row.name }}</text>
            <text class="r24-enemy-stacks">{{ row.elements.length ? row.elements.map(el => r24StackText(el, row.stackCounts[el])).join('·') : '无元素层' }}</text>
            <text v-for="h in row.hints" :key="row.uid + '-' + h.reaction + '-' + h.incoming" class="r24-reaction-hint">{{ r24HintText(h) }}</text>
          </view>
          <text v-if="!r24HasHints" class="r24-hud-empty">当前场上无可预判的元素反应</text>
        </view>
        <text v-else class="r24-hud-empty">当前场上无可预判的元素反应</text>
        <view class="r24-ai-note">
          <text class="r24-ai-title">AI · 实时元素层预判</text>
          <text class="r24-ai-conclusion">{{ r24Conclusion }}</text>
          <text class="r24-ai-basis">依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash</text>
          <text class="r24-ai-cta">去释放对应元素技能 →</text>
          <text class="r24-ai-honesty">辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准</text>
        </view>
      </view>
      <view class="hud-row">
        <!-- 音效开关（2026-10-23）：放在战斗 HUD 而不是设置页 —— 玩家想静音
             的时刻几乎总是"正在打"，让他退出战斗去找设置等于把开关藏起来。 -->
        <view class="mute-chip" :aria-label="muted ? '取消静音' : '静音'" @click="toggleMute">
          <Icon :name="muted ? 'volume-off' : 'volume-on'" :size="30" />
        </view>
        <!--
          战场图例（2026-11-17）：画布底部的热量条与技能槽此前只有颜色块，
          没有任何语义说明（用户反馈「红蓝黄块看不出是干嘛的」）。
          图例遮罩逐项解释 HUD 元素、热量档位、技能槽读法与全部反应链。
          打开图例会暂停战斗，关闭后自动继续（R3 实测发现阅读期间怪还在走）；暂停不污染回放哈希。
        -->
        <!-- 第六批 R70：首次进战斗给图例按钮一个呼吸提示，读过一次后记住，不再打扰 -->
        <view
          class="help-chip"
          :class="{ on: helpVisible, 'help-first': !helpSeen }"
          aria-label="战场图例"
          @click="openHelp"
        >
          <text>?</text>
        </view>
        <!--
          倍速档位（2026-10-22）。
          加它是因为「等一波打完」是测试时最耗时间的一件事：1× 跑完一局
          要几分钟，改一行数值就得重跑一次。
          倍速只压缩等待，不改模拟 —— 理由见 render/canvas.ts 的
          `BATTLE_SPEED_STEPS`（动 TICK_MS 会毁掉 I-6 的可重放性）。
        -->
        <view v-if="!settled" class="speed-row">
          <view
            v-for="s in SPEED_STEPS"
            :key="s"
            class="speed-chip"
            :class="{ on: speed === s }"
            @click="setSpeed(s)"
          >{{ s }}×</view>
        </view>
        <view class="hud-spacer" />
        <!-- 2026-10-21（P6-4）：拿不到 canvas 时明确告诉玩家（此前静默降级）。 -->
        <view v-if="renderMode === 'fallback'" class="mode-note">兼容模式</view>
        <!--
          2026-10-21（P4-5）：放弃入口。
          此前体力在开战时就扣了，而页面**没有**任何放弃路径 ——
          中途退出等于「什么都没发生，但资源没了」。
        -->
        <view v-if="!settled || (settleResult && settleResult.error)" class="abandon-btn" @click="abandon">
          <Icon name="flag" :size="26" />
          <text>{{ abandoning ? '处理中…' : '放弃' }}</text>
        </view>
        <!-- 暂停（2026-10-23）：切后台会自动暂停，回来由玩家自己点继续。 -->
        <view v-if="!settled && !paused" class="pause-btn" @click="doPause()">
          <Icon name="pause" :size="26" />
        </view>
      </view>

      <!-- t13 / F-10-1：五元素连锁 HUD —— 带汉字标签，颜色不是唯一线索 -->
      <view class="battle-sigil-rail" aria-label="五元素连锁">
        <view class="sigil sig-fire"><text>火</text></view>
        <view class="sigil sig-ice"><text>冰</text></view>
        <view class="sigil sig-lightning"><text>雷</text></view>
        <view class="sigil sig-corrosion"><text>蚀</text></view>
        <view class="sigil sig-kinetic"><text>动</text></view>
        <text class="sigil-caption">元素连锁 · 命中挂层触发反应</text>
      </view>
    </view>

    <!-- 暂停遮罩：挡住画布点击，避免「看起来暂停了其实还能点」 -->
    <view v-if="paused" class="pause-overlay">
      <view class="pause-box">
        <text class="pause-title">已暂停</text>
        <text class="muted pause-hint">
          {{ pausedByHide ? '切到后台时自动暂停了。' : '' }}战斗时间不会流逝，回放哈希不受影响。
        </text>
        <view class="btn btn-primary pause-go" @click="doResume">继续战斗</view>
      </view>
    </view>

    <!-- 战场图例（2026-11-17）：解释画布 HUD 的全部视觉语言 -->
    <view v-if="helpVisible" class="legend-overlay" @click="closeHelp">
      <view class="legend-box" @click.stop>
        <view class="legend-head">
          <text class="legend-title">战场图例</text>
          <view class="legend-close" @click="closeHelp"><text>×</text></view>
        </view>
        <scroll-view scroll-y class="legend-scroll">
          <!-- 画布底部：热量条 -->
          <view class="legend-sec">
            <text class="legend-sec-title">热量条（画布底部蓝/黄/橙长条）</text>
            <text class="legend-p">释放技能会积累<b>热量</b>；热量随时间自动下降。</text>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot" style="background: #58a6ff" /><text>蓝色 = 常态，随便放</text></view>
              <view class="legend-row"><view class="dot" style="background: #ffd33d" /><text>黄色 = 已过 80%，快到上限</text></view>
              <view class="legend-row"><view class="dot" style="background: #ff6b35" /><text>橙色 = <b>过热</b>：技能全部锁定 2 秒（元素层数保留，高热量爆发是可用的战术）</text></view>
            </view>
          </view>
          <!-- 画布底部：技能槽 -->
          <view class="legend-sec">
            <text class="legend-sec-title">技能槽（热量条下方的小方格）</text>
            <view class="legend-slot-demo">
              <view class="slot-demo">
                <DefIcon name="elem:fire" :size="30" style="color: #ff6b35" />
                <view class="slot-cost"><text>20</text></view>
              </view>
              <text class="legend-p">每个格子 = 一个出战技能。中央图形是它的元素（和图鉴一致），</text>
            </view>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #ffd33d" /><text>右上角数字 = 热量费用（每次释放消耗多少热量）</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: var(--dim)" /><text>格内变暗 + 剩余秒数 = 冷却中；描边亮起 = 就绪</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #58a6ff" /><text>格子数量 = 你携带的元素种类，「元素层数 N 技能」即来源于此</text></view>
            </view>
          </view>
          <!-- 元素与反应 -->
          <view class="legend-sec">
            <text class="legend-sec-title">元素反应（核心玩法）</text>
            <text class="legend-p">敌人身上先挂一种元素，再用另一种元素命中它就会触发<b>反应</b> —— 反应伤害中元素侧是主力，<b>面板攻击力也有贡献但权重上限 30%</b>，凑齐元素搭配更重要。</text>
            <view v-for="k in REACTION_ORDER" :key="k" class="legend-rxn">
              <view class="legend-rxn-head">
                <text class="legend-rxn-name">{{ REACTIONS[k].name }}</text>
                <view class="pair-chip">
                  <template v-for="(pair, i) in REACTION_PAIRS[k]" :key="i">
                    <text v-if="i > 0" class="dim">/</text>
                    <text class="tag" :class="'tag-' + pair[0]">{{ ELEMENT_NAME[pair[0]] }}</text>
                    <text class="tag" :class="'tag-' + pair[1]">{{ ELEMENT_NAME[pair[1]] }}</text>
                  </template>
                </view>
              </view>
              <text class="legend-rxn-descr">{{ REACTIONS[k].descr }}</text>
            </view>
          </view>
          <!-- 第六批 R4：敌人视觉语言说明 —— R3 给每种敌人加了独立配色/徽记/形状，
               若图例不讲，玩家只看到「花花绿绿」而不懂这意味着什么。 -->
          <view class="legend-sec">
            <text class="legend-sec-title">敌人外观（怎么分辨敌种）</text>
            <text class="legend-p">每种敌人有独立的<b>本体色 + 五官徽记</b>，形状标记它的行动方式：</text>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot" style="background: #6f9e5c" /><text>绿=游荡者、黄=疾行者、灰=石肤兽…同屏扫一眼就能分种</text></view>
              <view class="legend-row"><view class="dot" style="background: #4aa3b8" /><text>圆=普通，三角=飞行，菱形=远程，六边形=特殊，八边形=精英/BOSS</text></view>
              <view class="legend-row"><view class="dot" style="background: #f0f6fc" /><text>白点五官=双眼/护目镜/核心环等徽记；身上的小方块=元素层数</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #58a6ff" /><text>蓝环=护盾；受伤才出红色血条；BOSS 带橙色警示描边</text></view>
            </view>
          </view>
          <!-- 顶部与控制 -->
          <view class="legend-sec">
            <text class="legend-sec-title">顶部信息与控制</text>
            <view class="legend-rows">
              <view class="legend-row"><Icon name="shield" :size="26" style="color: #58a6ff" /><text>防线 = 你的血量，漏怪会扣，扣光即失败</text></view>
              <view class="legend-row"><Icon name="scroll" :size="26" style="color: var(--dim)" /><text>波次 = 本关共几波敌人；击杀 = 已消灭数</text></view>
              <view class="legend-row"><Icon name="volume-on" :size="26" style="color: var(--dim)" /><text>音效开关；倍速档只压缩等待，不影响战斗结果与回放</text></view>
              <view class="legend-row"><Icon name="flag" :size="26" style="color: #ff7b72" /><text>放弃 = 作废本局并退还体力（无战报无奖励）</text></view>
              <view class="legend-row"><Icon name="pause" :size="26" style="color: #58a6ff" /><text>暂停 = 战斗时间冻结，切后台会自动暂停</text></view>
            </view>
          </view>
          <text class="legend-foot">元素 / 反应 / 技能 / 敌人 / 装备 / 宝石 / 皮肤的完整数值见首页「图鉴」入口。打开图例时战斗暂停，关闭后自动继续。</text>
        </scroll-view>
      </view>
    </view>

    <!-- R29：完整反应时间线抽屉（R28 CTA 打开；只读派生，不改引擎权威） -->
    <view v-if="reactionTimelineVisible" class="r29-timeline-overlay" @click="closeReactionTimeline">
      <view class="r29-timeline-box" @click.stop>
        <view class="r29-timeline-head">
          <text class="r29-timeline-title">AI · 完整反应时间线</text>
          <view class="r29-timeline-close" @click="closeReactionTimeline"><text>×</text></view>
        </view>
        <text v-if="r29Timeline.empty" class="r29-timeline-empty">本局暂无逐次反应记录</text>
        <block v-else>
          <text v-if="!r29Timeline.empty" class="r29-timeline-conclusion">本局显示 {{ rows.length }} 条逐次反应记录</text>
          <text v-if="r29Timeline.truncated" class="r29-timeline-truncated">已截断，仅显示前 200 条</text>
          <scroll-view scroll-y class="r29-timeline-scroll">
            <view
              v-for="row in rows"
              :key="row.index"
              class="r29-timeline-row"
              :data-r29-index="row.index"
              :data-r29-wave="row.wave"
              :data-r29-enemy="row.enemy_uid"
            >第{{ row.wave }}波 · {{ row.tick }} · {{ row.reactionName }} · #{{ row.enemy_uid }} · {{ row.damage.toFixed(1) }} 伤害</view>
          </scroll-view>
        </block>
        <text class="r29-timeline-basis">依据：引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准</text>
        <text class="r29-timeline-honesty">辅助展示非权威判据，权威以 replay_hash 为准</text>
      </view>
    </view>

    <!-- 选牌界面（I-2） -->
    <view v-if="showCards" class="sheet">
      <view class="card-header">
        <text class="card-title">这一波给你 3 张卡，点一张就装备上</text>
        <text class="card-hint">
          <text>不想？点卡上的「弃」丢掉，还能回 1 点热量（热量攒满就会过热卡 2 秒）。</text>
          <!-- 第六批 R69：牌堆剩余此前完全不展示，玩家不知道这波卡是从多少张里抽的 -->
          <text> 本波还能弃 {{ deck.discardsLeft }} 张，牌堆剩 {{ deck.size }} 张。</text>
        </text>
        <text v-if="cardChoiceFeedback" class="card-choice-feedback">{{ cardChoiceFeedback }}</text>
      </view>
      <!-- R26 下一波敌情简报：只读派生展示，先看敌情再选卡；不碰引擎/结算/replay_hash -->
      <view v-if="r26Brief" class="r26-wave-brief" role="region">
        <text class="r26-brief-head">下一波 · 第 {{ r26Brief.waveIndex + 1 }} 波</text>
        <view v-if="r26Brief.empty" class="r26-brief-empty">暂无下一波敌情信息</view>
        <view v-else class="r26-enemy-list">
          <view v-for="row in r26Brief.rows" :key="row.enemy_id" class="r26-enemy-row" :data-r26-enemy="row.enemy_id">
            <text class="r26-enemy-name">{{ row.name }}</text>
            <text v-if="row.unknown" class="r26-unknown-tag">[未知敌种]</text>
            <text class="r26-enemy-count">×{{ row.count }}</text>
            <text v-if="row.threat" class="r26-enemy-threat">{{ row.threat }}</text>
          </view>
        </view>
        <view v-if="!r26Brief.empty" class="r26-ai-note" role="note">
          <text class="r26-ai-title">AI · 下一波敌情</text>
          <text class="r26-ai-conclusion">{{ r26Brief.ai }}</text>
          <text class="r26-ai-basis">依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies</text>
          <text class="r26-ai-cta">结合本波威胁与手牌元素协同选卡</text>
          <text class="r26-ai-honesty">辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准</text>
        </view>
      </view>
      <view class="cards">
        <view
          v-for="c in hand"
          :key="c.id"
          class="hand-card"
          :class="[`rarity-${c.rarity}`, `kind-${c.kind}`]"
          @click="takeCard(c.id)"
        >
          <view class="hand-top">
            <text class="hand-kind">{{ kindLabel(c.kind) }}</text>
            <text class="rarity-chip" :class="'rarity-' + c.rarity">{{ rarityLabel(c.rarity) }}</text>
          </view>
          <!-- 卡面图标：技能卡 = 元素图形，属性/机制卡 = 对应类别图形（与图鉴同语言） -->
          <view class="hand-icon" :class="handBadgeClass(c)">
            <DefIcon :name="handIconName(c)" :size="44" />
          </view>
          <text class="hand-name">{{ c.name }}</text>
          <text class="hand-descr">{{ c.descr }}</text>
          <view class="hand-foot">
            <text
              v-if="c.element"
              class="tag"
              :class="'tag-' + c.element"
            >{{ elementName(c.element) }}</text>
            <text class="dim" @click.stop="discardCard(c.id)">弃</text>
          </view>
          <!-- R22 选卡反应协同预判：纯展示派生，不改卡牌点击/弃牌行为 -->
          <view class="r22-synergy-card">
            <template v-if="synergyOf(c).kind === 'added'">
              <text class="r22-synergy-line">AI · 新增【{{ synergyOf(c).addedNames[0] }}】等{{ synergyOf(c).added.length }}种反应</text>
              <view class="r22-synergy-tags">
                <text v-for="k in synergyOf(c).added" :key="k" class="r22-synergy-tag" :data-r22-reaction="k">{{ reactionName(k) }}</text>
              </view>
              <text class="r22-synergy-match">可搭配 {{ synergyOf(c).partners.map(elementName).join('·') }}</text>
            </template>
            <template v-else-if="synergyOf(c).kind === 'reinforced'">
              <text class="r22-synergy-line">AI · 强化已有【{{ synergyOf(c).reinforcedNames[0] }}】</text>
              <view class="r22-synergy-tags">
                <text v-for="k in synergyOf(c).reinforced" :key="k" class="r22-synergy-tag" :data-r22-reaction="k">{{ reactionName(k) }}</text>
              </view>
            </template>
            <text v-else class="r22-synergy-empty">{{ synergyOf(c).emptyText }}</text>
          </view>
        </view>
      </view>
      <view class="btn" @click="skipCards">全部不要了，直接下一波</view>
      <view class="r22-ai-note" role="note">
        <text class="r22-ai-title">AI · 协同预判</text>
        <text class="r22-ai-conclusion">{{ synergyNoteConclusion }}</text>
        <text class="r22-ai-basis">依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生</text>
        <text class="r22-ai-cta">↑ 点上方卡牌即可装备它 →</text>
        <text class="r22-ai-honesty">辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准</text>
      </view>
    </view>

    <!-- 结算界面 -->
    <view v-if="settleResult" class="sheet">
      <text class="result-title" :style="settleFailed ? { color: 'var(--bad)' } : undefined">{{ settleFailed ? (settleResult.win ? '战绩未入账' : '防线失守 · 战绩未入账') : (settleResult.win ? '通关' : '防线失守') }}</text>
      <!-- G91 战斗评级徽章：一个可追求的综合目标 -->
      <view v-if="battleGrade && !settleFailed" class="grade-badge" :style="{ borderColor: battleGrade.color, color: battleGrade.color }">
        <text class="grade-letter">{{ battleGrade.grade }}</text>
        <text class="grade-label">{{ battleGrade.label }}</text>
      </view>
      <!-- G92 下次目标：一句可执行的改进方向 -->
      <text v-if="nextGoal && !settleFailed" class="next-goal">下一步：{{ nextGoal }}</text>
      <view v-if="!settleFailed" class="stars-big">
        <text
          v-for="i in 3"
          :key="i"
          class="star-big"
          :class="{ off: i > settleResult.stars }"
        >★</text>
      </view>
      <text v-if="settleFailed" class="settle-local-note">以下为本地战绩（仅供参考，未入账）</text>
      <view class="result-stats">
        <view class="rstat">
          <Icon name="sword" :size="30" style="color: var(--dim)" />
          <text class="muted">击杀</text><text class="mono">{{ settleResult.kills }}</text>
        </view>
        <view class="rstat">
          <Icon name="flag" :size="30" style="color: #ff7b72" />
          <text class="muted">漏怪</text><text class="mono">{{ settleResult.leaked }}</text>
        </view>
        <view class="rstat">
          <DefIcon name="reaction" :size="30" style="color: #7ee787" />
          <text class="muted">反应</text><text class="mono">{{ settleResult.reactions }}</text>
        </view>
        <view class="rstat">
          <Icon name="trophy" :size="30" style="color: #ffd33d" />
          <text class="muted">分数</text><text class="mono">{{ settleResult.score }}</text>
        </view>
      </view>
      <view v-if="settleResult.clamped" class="clamp-note">
        服务端已修正本次结算：{{ settleResult.clamp_note }}
      </view>
      <text v-if="settleAttribution && !settleFailed" class="settle-attribution">{{ settleAttribution }}</text>
      <text v-if="settleDefeatReview" class="settle-defeat-review">{{ settleDefeatReview }}</text>
      <view v-if="!settleFailed" class="r10-growth r10-enter">
        <text class="r10-growth-title">AI · 养成前后（同一关可信差值）</text>
        <template v-if="growthCompare.available">
          <text class="r10-growth-text">{{ growthCompare.text }}</text>
          <view class="r10-growth-grid">
            <view v-for="row in growthCompare.rows" :key="row.label" class="r10-growth-row">
              <text class="r10-growth-label">{{ row.label }}</text>
              <text class="r10-growth-num">{{ fmt(row.before) }} → {{ fmt(row.after) }}</text>
              <text class="r10-growth-delta" :class="{ negative: row.delta < 0 }">{{ fmtSigned(row.delta) }}</text>
            </view>
          </view>
          <text class="r10-hashes">hash：{{ growthHashes }}</text>
          <text class="r10-growth-basis">依据：{{ growthCompare.basis }}</text>
        </template>
        <text v-else class="r10-growth-empty" aria-live="polite" role="status">还缺养成前那局战报，先打一局同关 · 构筑变更点未上报，不做估算。</text>
      </view>
      <text v-if="settleCardFeedback" class="settle-card-feedback">{{ settleCardFeedback }}</text>
      <!-- 结算接口失败：先说人话，再留原始串排障。绝不能把未入账的一局伪装成通关。 -->
      <view v-if="settleFailed" class="settle-error">
        <text class="settle-error-hint">{{ settleErrorHint }}</text>
        <text class="settle-error-detail">{{ settleResult.error }}</text>
      </view>
      <!-- 结算失败时 token 仍未 used_at，服务端仍允许放弃退体力；
           不给出入口，玩家会白白损失开战时已扣的体力（或被迫再来一局再扣一次）。 -->
      <view v-if="settleResult.error" class="btn" @click="abandon">{{ abandoning ? '处理中…' : '放弃并退回体力' }}</view>
      <view v-if="!settleFailed" class="reward-row">
        <view v-for="(v, k) in settleResult.loot" :key="k" class="reward">
          <Icon :name="currencyIcon(String(k))" :size="28" />
          <text>{{ currencyName(String(k)) }} +{{ v }}</text>
        </view>
      </view>
      <text v-if="settleFailed" class="hash-note">本地回放哈希 {{ settleResult.replay_hash }}（未入账，仅供排障）</text>
      <text v-else class="hash-note">回放哈希 {{ settleResult.replay_hash }}（可验真）</text>
      <!-- t227：结算 sheet 遮挡战斗态 R28 feed CTA，结果页提供真实可见可点的时间线入口 -->
      <view class="r29-result-timeline-cta" @click.stop="openReactionTimeline">查看完整反应时间线 →</view>
      <!-- 查看本局战报（2026-11-01 新一轮第 43 轮）：结算响应带 battle_id，
           跳转战报页并自动展开这一局，玩家可以立即回顾掉落/元素/反应/地形。 -->
      <view
        v-if="settleResult.battle_id && !settleFailed"
        class="btn"
        style="margin-top: 20rpx"
        @click="goBattleReport"
      >
        查看本局战报
      </view>
      <!-- 再来一局（2026-11-01 新一轮第 53 轮）：redirectTo 替换当前页，
           重新进入同一关（重新扣体力/发 token），省去返回选关再点一次的路径。 -->
      <view v-if="!settleFailed" class="btn" style="margin-top: 20rpx" @click="retryLevel">再来一局</view>
      <view
        v-if="canGoNextLevel"
        class="btn btn-primary next-level-btn"
        style="margin-top: 20rpx"
        @click="goNextLevel"
      >前往第 {{ nextLevelId }} 关 →</view>
      <view class="btn btn-primary" style="margin-top: 20rpx" @click="backToStage">返回关卡</view>
    </view>

    <!-- 启动失败提示 -->
    <view v-if="startError" class="sheet">
      <text class="result-title" style="color: var(--bad)">无法开始战斗</text>
      <text class="muted">{{ startError }}</text>
      <view class="btn" style="margin-top: 24rpx" @click="backToStage">返回</view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { currencyName } from '@/utils/labels'
import Icon from '@/components/Icon.vue'
import { useStatusBarPad } from '@/utils/statusBar'
import { ref, computed, getCurrentInstance, triggerRef } from 'vue'
import { onLoad, onUnload, onHide, onShow, onShareAppMessage } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { normalizeGrowthCompare, type GrowthCompareView } from '@/pages/_shared/r10GrowthCompare'
import { BattleEngine, type ElementLayerHudRow } from '@/game/engine'
import { equippedFromSnapshot } from '@/game/replay'
import { buildNextWaveBrief } from './nextWaveBrief'
import { buildWaveProgress } from './waveProgressHud'
import { attributeLiveReactions, attributionText, buildLiveFeedAi } from './liveReactionFeed'
import { buildReactionTimelineView } from './reactionTimelineView'
import { battleShare } from '@/utils/share'
import { playBattleEvents } from '@/audio/battle-sfx'
// 音效解锁（2026-10-23）：模板根节点的 `@touchstart="unlockSfx()"` 是**第二道保险**。
//
// 主入口在 `stage.vue` 的「开始战斗」——那是一次真实的用户手势，且发生在
// 进入战斗之前，所以第一发子弹就有声音。这里再兜一次：玩家在战斗页碰一下
// 屏幕也能解锁，不必等到某个特定按钮。
//
// 微信与 iOS 都要求音频在用户手势之后才能播；`unlockSfx()` 会在同一帧里
// 播一次 0 音量的 tap 把手势用掉（理由见 audio/sfx.ts）。
import { initSfx, unlockSfx, isMuted, setMuted, playSfx } from '@/audio/sfx'
import {
  BattleRenderer,
  BATTLE_SPEED_STEPS,
  logicIntervalMs,
  normalizeSpeed,
} from '@/render/canvas'
import { ELEMENT_NAME, REACTIONS, REACTION_ORDER, lookupReaction, type Element, type ReactionKey } from '@/game/elements'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import { scoreRulesFromServer, scoreTimeBudget } from '@/game/score'
import { damageShareFromEvents, defeatReview, reactionCue, summarizeCardPicks, type DamageShare } from '@/game/feedback'
import type { Card, EquippedSkill } from '@/game/heatmap'
import type { GeneratedLevel } from '@/game/types'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/** 顶部让开状态栏（2026-10-21 P6-2）。 */
const { statusBarPad } = useStatusBarPad()
const inst = getCurrentInstance()

const canvasW = ref(375)
const canvasH = ref(667)
const engine = ref<BattleEngine | null>(null)

/** R24：引擎只读元素层快照，tick 后刷新；纯展示，不写回引擎。 */
const r24ElementHud = ref<ElementLayerHudRow[]>([])
interface R24Hint { reaction: ReactionKey; existing: Element; incoming: Element }
function refreshElementHud(eng: BattleEngine): void {
  const rows = typeof eng.elementLayerHud === 'function' ? eng.elementLayerHud() : []
  r24ElementHud.value = Array.isArray(rows) ? rows : []
}

/** R27：引擎只读波次推进快照，step 后刷新；纯展示，不写回引擎。 */
const r27ProgressRaw = ref<{ waveIndex: number; waveTotal: number; onField: number; pending: number } | null>(null)
function refreshWaveProgress(eng: BattleEngine): void {
  r27ProgressRaw.value = typeof eng.waveProgressHud === 'function' ? eng.waveProgressHud() : null
}
const r27Progress = computed(() => r27ProgressRaw.value ? buildWaveProgress(r27ProgressRaw.value) : null)

const r24ElementRows = computed(() => r24ElementHud.value.map(row => {
  const elements = (Object.keys(row.stacks) as Element[]).filter(el => (row.stacks[el] ?? 0) > 0)
  const hints: R24Hint[] = []
  const seen = new Set<string>()
  for (const reaction of Object.keys(REACTION_PAIRS) as ReactionKey[]) {
    for (const [existing, incoming] of REACTION_PAIRS[reaction]) {
      if (!elements.includes(existing) || elements.includes(incoming)) continue
      if (lookupReaction(existing, incoming) !== reaction) continue
      const id = reaction + '|' + existing + '|' + incoming
      if (seen.has(id)) continue
      seen.add(id)
      hints.push({ reaction, existing, incoming })
    }
  }
  const stackCounts: Partial<Record<Element, number>> = {}
  for (const el of elements) stackCounts[el] = row.stacks[el] ?? 0
  return { ...row, elements, stackCounts, hints }
}))
const r24HasHints = computed(() => r24ElementRows.value.some(row => row.hints.length > 0))
function r24StackText(el: Element, count: number | undefined): string {
  const name: Record<Element, string> = ELEMENT_NAME
  return String(name[el]) + String(count ?? 0)
}
function r24HintText(h: R24Hint): string {
  return '已附着 ' + ELEMENT_NAME[h.existing] + ' + 再补 ' + ELEMENT_NAME[h.incoming] + ' → ' + REACTIONS[h.reaction].name + ' 反应'
}
const r24Conclusion = computed(() => {
  const first = r24ElementRows.value.find(row => row.hints.length > 0)
  if (!first) return '当前场上无可预判的元素反应，不编造建议'
  const priority = ['superconduct', 'overheat', 'steam_burst'] as ReactionKey[]
  const sorted = [...first.hints].sort((a, b) => {
    const ia = priority.indexOf(a.reaction), ib = priority.indexOf(b.reaction)
    return (ia < 0 ? 99 : ia) - (ib < 0 ? 99 : ib)
  })
  const actions = sorted.slice(0, 2).map(h => '再补 ' + ELEMENT_NAME[h.incoming] + ' 可触发' + REACTIONS[h.reaction].name).join('、')
  return 'AI 提示：' + first.name + '已附着' + ELEMENT_NAME[first.hints[0]!.existing] + '，' + actions
})
const renderer = ref<BattleRenderer | null>(null)
const startError = ref('')
const settleResult = ref<any>(null)

/** R10 §1I.2：结算卡上的养成前后视图只认同关两次真实结算的确定性差值。 */
const growthCompare = ref<GrowthCompareView>(normalizeGrowthCompare([]))
const growthHashes = computed(() => growthCompare.value.hashes.join(' / ') || '—')

/** R11：养成前后差值只改展示格式，不改差值本身或结算逻辑。 */
function fmt(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? n.toFixed(1) : '—'
}
function fmtSigned(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? (n >= 0 ? '+' : '') + n.toFixed(1) : '—'
}

async function loadGrowthCompare(detail: import('@/api/client').BattleDetail) {
  let baseline: import('@/api/client').BattleDetail | null = null
  try {
    const history = await api.myBattleHistory(20)
    const prior = (history.items ?? []).filter((x) => x.level_id === detail.level_id && x.replay_hash && x.replay_hash !== detail.replay_hash)[0]
    if (prior) baseline = await api.myBattleDetail(prior.id)
  } catch {
    baseline = null
  }
  growthCompare.value = baseline ? normalizeGrowthCompare([baseline, detail]) : normalizeGrowthCompare([])
}

/**
 * R9：最近反应的可读记录。canvas 粒子只在几帧内闪现，8× 下几乎看不清；
 * HUD 这三条短记录不拦截点击、不盖住战场，让玩家知道「刚刚打出了什么」。
 */
interface ReactionLog { id: number; key: ReactionKey; name: string; pair: string; cue: string; damage: number; resisted: boolean; wave: number | null; enemy_uid: number | null; available: boolean; attribution: string }
const recentReactions = ref<ReactionLog[]>([])
/** R28：最近三条反应的归因 AI 说明（来自真实匹配计数，纯只读派生）。 */
const r28Ai = ref(buildLiveFeedAi({ rows: [], matchedCount: 0, unavailableCount: 0 }))
/** R29：完整反应时间线抽屉的只读展示快照（打开时从引擎只读旁路派生一次）。 */
const reactionTimelineVisible = ref(false)
let timelinePausedByDrawer = false
const r29Timeline = ref(buildReactionTimelineView(null, false, []))
/** 抽屉列表行；模板逐字用 {{ rows.length }} 显示过滤后的真实条数。 */
const rows = computed(() => r29Timeline.value.rows)
let reactionSeq = 0
/** R9：累计本局真实 hit/reaction 事件金额，只用于展示「反应伤害占比」，不进回放哈希。 */
const damageShare = ref<DamageShare>({ reactionDamage: 0, totalDamage: 0, ratioPct: null })
/** cardPicks 是引擎类实例上的普通数组；显式版本号让选卡反馈能随 take/discard/skip 刷新。 */
const cardChoiceVersion = ref(0)
/** 结算时冻结权威 card_picks，避免后续操作改写归因。 */
const settleCardPicks = ref<number[]>([])

/**
 * 结算被服务端拒绝（如 replay_skills 与构筑不符）时，这一局**没有入账**。
 * 此时哪怕引擎跑出了胜利，也不能把它渲染成「通关 + C 评级 + 0 星」——
 * 那会让玩家以为奖励已到账，直到去背包/战报才发现什么都没有（2026-11-21 实测）。
 */
const settleFailed = computed(() => Boolean(settleResult.value?.error))
const settleErrorHint = computed(() => {
  const raw = String(settleResult.value?.error ?? '')
  if (raw.includes('replay_skills') || raw.includes('构筑不符')) {
    return '本地构筑与服务端记录不一致，这一局未计入战绩。可放弃退回体力，或返回关卡重新挑战。'
  }
  return '战绩上报失败，这一局未计入战绩。可放弃退回体力，或稍后重试。'
})

/** R7：星级门槛只认服务端下发的关卡配置；拿不到就不显示差额，绝不硬编码。 */
function levelStarTargets(): number[] {
  const raw = engine.value?.cfg?.level?.star_targets
  return Array.isArray(raw)
    ? raw.map((t: unknown) => Number(t)).filter((t: number) => Number.isFinite(t))
    : []
}

/** G91 战斗评级：基于星级/漏怪/反应的综合评价，给玩家一个可追求的目标（纯客户端派生）。 */
const battleGrade = computed(() => {
  const r = settleResult.value
  if (!r) return null
  if (!r.win) return { grade: 'D', label: '防线失守', color: '#ff7b72' }
  const stars = Number(r.stars ?? 0)
  const leaked = Number(r.leaked ?? 0)
  const reactions = Number(r.reactions ?? 0)
  let score = stars * 30 + Math.min(20, reactions) - leaked * 10
  if (score >= 95) return { grade: 'S', label: '完美防线', color: '#ffd700' }
  if (score >= 70) return { grade: 'A', label: '出色', color: '#7ee787' }
  if (score >= 45) return { grade: 'B', label: '良好', color: '#58a6ff' }
  return { grade: 'C', label: '过关', color: '#8b949e' }
})

/** G92 下次目标：基于本局短板给一句可执行的改进方向（纯客户端派生）。 */
const nextGoal = computed(() => {
  const r = settleResult.value
  if (!r || !r.win) return null
  const leaked = Number(r.leaked ?? 0)
  const reactions = Number(r.reactions ?? 0)
  const stars = Number(r.stars ?? 0)
  if (stars >= 3) return '完美三星！试试更少反应或更高难度关卡。'
  if (leaked > 0) return `漏了 ${leaked} 只怪 —— 补点溅射/控场技能守住防线即可拿更高星。`
  // t63 §1F：反应伤害攻击侧贡献最高 300‰，「关系不大」同样是误导养成，必须说清上限 30%。
  if (reactions < 3) return '多凑一种元素触发反应链，反应伤害中元素侧是主力，面板攻击力也有贡献但权重上限 30%，优先凑齐元素搭配。'
  // R5 真浏览器实测：0 星 / 11646 分（1★ 门槛 11941）还显示「冲 3 星」，
  // 玩家连最低门槛都没摸到，这句建议等于没说。门槛由服务端下发在关卡配置里，
  // 分数用结算返回的权威值（含超时长裁剪），别拿引擎本地分数自说自话。
  const targets = levelStarTargets()
  const nextStar = stars + 1
  if (targets.length >= nextStar) {
    const target = targets[nextStar - 1]!
    const score = Number(r.score ?? 0)
    if (score < target) {
      return `还差 ${target - score} 分就能拿 ${nextStar} 星（门槛 ${target} 分）—— 少漏怪、多触发反应就能提分。`
    }
  }
  return '再稳一点，冲 3 星拿满钥匙。'
})

/** R7 缺口2：战斗中实时显示距下一星差额；门槛来自关卡配置，分数来自引擎当前值。 */
const liveStarGap = computed(() => {
  if (settleResult.value || !engine.value) return null
  const targets = levelStarTargets()
  if (!targets.length) return null
  const score = Math.max(0, Number(engine.value.score ?? 0) || 0)
  for (let i = 0; i < targets.length; i++) {
    const target = targets[i]!
    if (score < target) return `距 ${i + 1} 星 ${target - score}`
  }
  return '已达满星门槛'
})

/**
 * R8 缺口3：战斗中实时显示距离「免钳分锚点」还有多久。
 * 服务端 scoreCapFor 是负向闸（快于锚点按比例裁分），锚点前一直钳分中，
 * 所以文案必须是「距满分 Xs / 即将免钳分」，不能说成「保分窗口」。
 * 锚点由关卡波次按服务端同款系数推导，已用战斗时钟 elapsedMs 更新；
 * 没有波次配置或已结算时不显示，避免把本地猜测当成服务端口径。
 */
const timeBudget = computed(() => {
  if (settled.value || !engine.value) return null
  return scoreTimeBudget(engine.value.cfg?.level, Number(engine.value.elapsedMs ?? 0))
})

/** R7 缺口1：把服务端权威分数、钳分上限、星级门槛与漏怪数串成一句可执行的复盘建议。 */
const settleAttribution = computed(() => {
  const r = settleResult.value
  if (!r || !r.win || settleFailed.value) return null
  const score = Math.max(0, Number(r.score ?? 0) || 0)
  const leaked = Math.max(0, Number(r.leaked ?? 0) || 0)
  const targets = levelStarTargets()
  const nextStar = Math.max(1, Number(r.stars ?? 0) + 1)
  const target = nextStar <= targets.length ? targets[nextStar - 1]! : null

  // 服务端 SettleResp 没有结构化 time_cap；钳分串由 domain/battle.go 生成，
  // 只从真实返回的 clamp_note 解析。没有就只讲真实分数，绝不编造上限。
  const note = String(r.clamp_note ?? '')
  const clamp = note.match(/得分超时长额度：(\d+)\s*→\s*(\d+)（(\d+)s\s*内上限/)
  const parts: string[] = []
  if (clamp) parts.push(`${clamp[3]}s 内得分上限 ${clamp[1]}，实得 ${score}`)
  else parts.push(`实得 ${score}`)
  if (target !== null) {
    if (score < target) parts.push(`距 ${nextStar} 星门槛 ${target} 还差 ${target - score}`)
    else parts.push(`已达 ${nextStar} 星门槛 ${target}`)
  }
  parts.push(`漏怪 ${leaked}`)
  parts.push(clamp ? '建议缩短清波时间或少漏怪、多触发反应提分' : '建议少漏怪、多触发反应提分')
  return parts.join('；') + '。'
})

/** R9 选卡面板：统计来自权威 card_picks，反应数来自引擎，不虚构任何收益。 */
const cardChoiceFeedback = computed(() => {
  const eng = engine.value
  void cardChoiceVersion.value
  if (!eng || settled.value) return null
  const picks = Array.isArray(eng.cardPicks) ? eng.cardPicks : []
  const s = summarizeCardPicks(picks)
  const reactions = Math.max(0, Number(eng.reactionsCount ?? 0) || 0)
  if (s.total === 0 && reactions === 0) {
    return 'AI 本局还没有可归因的选择记录；依据 card_picks / reaction 事件，点卡装备或全部不要了继续。'
  }
  return `AI 已取卡 ${s.taken} 张、弃 ${s.discarded} 张、跳过 ${s.skipped} 波，反应 ${reactions} 次；依据 card_picks / reaction 事件；点卡装备或全部不要了推进下一波。`
})

/** R9 结算：把权威选择、真实反应数与伤害占比串成可复盘结论。缺数据就诚实空态。 */
const settleCardFeedback = computed(() => {
  const r = settleResult.value
  if (!r || settleFailed.value) return null
  const picks = settleCardPicks.value
  if (!Array.isArray(picks) || !picks.length) return 'AI 本局未记录选择收益（缺 card_picks）；下一局可对比不同搭配。'
  const s = summarizeCardPicks(picks)
  const reactions = Math.max(0, Number(r.reactions ?? 0) || 0)
  const share = damageShare.value
  const shareText = share.ratioPct === null ? '本局无完整伤害记录' : `反应伤害占比 ${share.ratioPct}%`
  return `AI 本局取卡 ${s.taken} 张、弃 ${s.discarded} 张、跳过 ${s.skipped} 波，反应 ${reactions} 次；${shareText}；依据 card_picks / hit+reaction 事件；下一局可换元素搭配对比收益。`
})

/** R12 失败复盘：把真实结算数据翻译成「为什么输 + 下一局改什么」，成功/结算失败不冒充。 */
const settleDefeatReview = computed(() => {
  const r = settleResult.value
  if (!r || r.win || settleFailed.value) return null
  const eng = engine.value
  const level = eng?.cfg?.level
  return defeatReview({
    wave: eng ? Number(eng.waveIndex ?? 0) + 1 : null,
    totalWaves: level ? Number(level.wave_count ?? level.waves?.length ?? 0) || null : null,
    kills: Number(r.kills ?? 0),
    leaked: Number(r.leaked ?? 0),
    reactions: Number(r.reactions ?? 0),
    cardPicks: settleCardPicks.value,
  })
})

const hand = ref<Card[]>([])
const tokenId = ref(0)

/**
 * 渲染模式（2026-10-21 P6-4）。
 *
 * 'pending'  —— 还没起
 * 'raf'     —— 正常，用 requestAnimationFrame 驱动
 * 'fallback'—— **拿不到 canvas**，改用定时器推进
 *
 * ⚠️ 此前这个 ref 只写不读：拿不到 canvas 时页面**静默**降级，
 * 玩家只看到一片黑屏，还以为是游戏卡了。
 * 现在 fallback 会明确提示（「兼容模式」）。
 */
const renderMode = ref<'pending' | 'raf' | 'fallback'>('pending')
const levelId = ref(1)

/** 可选档位（与渲染层逻辑时钟共用一份定义，避免两处各写一套数字）。 */
const SPEED_STEPS = BATTLE_SPEED_STEPS

/** 倍速的持久化键。 */
const SPEED_KEY = 'lyp_battle_speed'

/**
 * 倍速（2026-10-22，测试便利）。
 *
 * 为什么持久化：调数值时同一关要反复重开，每次进战斗都重新点一遍
 * 档位纯属浪费；而档位在 HUD 上一直可见，误留 8× 也一眼能发现。
 */
const speed = ref(loadSpeed())

function loadSpeed(): number {
  try {
    const n = Number(uni.getStorageSync(SPEED_KEY))
    return Number.isFinite(n) && n > 1 ? normalizeSpeed(n) : 1
  } catch {
    // 存储不可用（隐私模式 / 老基础库）不该让战斗开不起来 —— 退回 1×
    return 1
  }
}

/**
 * 换挡。
 *
 * 三条路径都要跟着换：
 *   - 有渲染器 → 交给它重建逻辑时钟
 *   - 无 canvas 的降级路径 → 本页自己的定时器，重建
 *   - 还没开局 → 只记下档位，setup() 建时钟时读 speed
 */
function setSpeed(mult: number): void {
  const next = normalizeSpeed(mult)
  if (next === speed.value) return
  speed.value = next
  try {
    uni.setStorageSync(SPEED_KEY, String(next))
  } catch {
    // 写失败只是下次进来回到 1×，不影响本局
  }
  if (renderer.value) {
    renderer.value.setSpeed(next)
  } else if (timerId) {
    startFallbackLoop()
  }
}

let loopStarted = false
let timerId: ReturnType<typeof setInterval> | null = null
// 第 137 轮：页面是否已卸载。setup() 里有多个 await（login/startBattle/getCanvas），
// 若用户在任一 await 期间退出页面，onUnload 先跑（此时 renderer 还是 null、没东西可停），
// 随后 await 完成会继续创建并 start() 渲染器 —— 在一个已卸载的页面上留下
// 永久运行的 rAF/定时器 + 引擎步进（泄漏）。用本标志在每次 await 后拦截。
let pageUnloaded = false

const hpPct = computed(() => {
  if (!engine.value) return 100
  return Math.max(0, (Number(engine.value.baseHp) / Number(engine.value.baseHpMax)) * 100)
})
const hpText = computed(() => {
  if (!engine.value) return '—'
  return `${Number(engine.value.baseHp)} / ${Number(engine.value.baseHpMax)}`
})
const waveText = computed(() => {
  if (!engine.value) return '—'
  const total = engine.value.cfg.level.wave_count
  return `${engine.value.waveIndex + 1} / ${total}`
})
/** 第六批 R5：当前关卡名（引擎公开字段，只读）。 */
const levelTitle = computed(() => {
  if (!engine.value) return ''
  const lv = engine.value.cfg.level
  return `第 ${lv.id} 关 · ${lv.name}`
})
const showCards = computed(() => engine.value?.phase === 'card_select' && !settleResult.value)
/** R26 下一波敌情简报：只读派生自 eng.cfg.level.waves 与 store.config.enemies，不写回引擎。 */
const r26Brief = computed(() => {
  const eng = engine.value
  if (!eng) return null
  return buildNextWaveBrief(eng.cfg.level.waves, eng.waveIndex, store.enemyMap)
})
const deck = computed(() => engine.value?.deck ?? { discardsLeft: 0, size: 0 } as any)

function elementName(e: Element): string {
  return ELEMENT_NAME[e]
}

/** R22：反应名只取引擎真源 REACTIONS，禁止硬编码。 */
function reactionName(k: ReactionKey): string {
  return REACTIONS[k].name
}

/**
 * R22 选卡反应协同预判（纯展示派生，不落库/不进 replay_hash/不改结算）。
 * 覆盖集 C(E)：以 REACTION_PAIRS（源自引擎 lookupReaction）为唯一真源，
 * 对元素集两两查表，非空者收入集合；新卡 e 的差集 A = C(E∪{e}) − C(E)
 * 按 REACTION_ORDER 稳定排序；B = E 中与 e 组合产生 A 的元素。
 */
type R22Synergy = {
  kind: 'added' | 'reinforced' | 'empty'
  added: ReactionKey[]
  addedNames: string[]
  reinforced: ReactionKey[]
  reinforcedNames: string[]
  partners: Element[]
  emptyText: string
}
const equippedElements = ref<Element[]>([])

function r22Covered(elements: Element[]): Set<ReactionKey> {
  const out = new Set<ReactionKey>()
  for (const a of elements) {
    for (const b of elements) {
      if (a === b) continue
      for (const k of REACTION_ORDER) {
        if (REACTION_PAIRS[k].some(([x, y]) => x === a && y === b)) out.add(k)
      }
    }
  }
  return out
}

function r22Partners(e: Element, added: Set<ReactionKey>): Element[] {
  return equippedElements.value.filter((a) =>
    REACTION_ORDER.some((k) => added.has(k) && REACTION_PAIRS[k].some(([x, y]) => x === a && y === e)),
  )
}

function r22Synergy(c: Card): R22Synergy {
  const empty = (emptyText: string): R22Synergy => ({
    kind: 'empty', added: [], addedNames: [], reinforced: [], reinforcedNames: [], partners: [], emptyText,
  })
  if (equippedElements.value.length === 0) return empty('当前构筑还没有元素技能，无法预判反应')
  if (c.kind !== 'skill' || !c.element) return empty('该卡无元素，无法预判反应（属性/机制卡）')
  const before = r22Covered(equippedElements.value)
  const after = r22Covered([...equippedElements.value, c.element])
  const added = REACTION_ORDER.filter((k) => after.has(k) && !before.has(k))
  const reinforced = REACTION_ORDER.filter((k) => after.has(k) && before.has(k))
  if (added.length > 0) {
    return {
      kind: 'added', added, addedNames: added.map(reactionName),
      reinforced: [], reinforcedNames: [], partners: r22Partners(c.element, new Set(added)), emptyText: '',
    }
  }
  if (reinforced.length > 0) {
    return {
      kind: 'reinforced', added: [], addedNames: [], reinforced, reinforcedNames: reinforced.map(reactionName),
      partners: [], emptyText: '',
    }
  }
  return empty('该卡无元素，无法预判反应（属性/机制卡）')
}

function synergyOf(c: Card): R22Synergy {
  return r22Synergy(c)
}

/** AI 结论取当前手牌首张有预判的卡，真实来自差集，不虚构夸张结论。 */
const synergyNoteConclusion = computed(() => {
  for (const c of hand.value) {
    const s = r22Synergy(c)
    if (s.kind === 'added') return `当前手牌「${c.name}」可新增【${s.addedNames[0]}】等${s.added.length}种反应`
    if (s.kind === 'reinforced') return `当前手牌「${c.name}」可强化已有【${s.reinforcedNames[0]}】`
  }
  return '当前手牌暂无可预判的新反应，去装备元素技能或换一张卡'
})

function kindLabel(k: string): string {
  return k === 'skill' ? '技能' : k === 'attribute' ? '属性' : '机制'
}

// ── 战场图例（2026-11-17，见模板内说明）────────────────────────────
const helpVisible = ref(false)
/** 第六批 R70：是否读过图例（本机记忆）。 */
const helpSeen = ref(uni.getStorageSync('battle_help_seen') === '1')
/** R3：图例遮罩挡住整个战场，读说明时由页面代为暂停；关掉时只恢复这类暂停。 */
let helpPausedByPage = false

function openHelp() {
  // R29 互斥：打开图例前无条件关闭时间线抽屉（函数声明 hoisting 可用）
  closeReactionTimeline()
  if (helpVisible.value) {
    closeHelp()
    return
  }
  helpVisible.value = true
  // 手动暂停中就别代劳：玩家自己暂停的状态不能被「看图例」顺手改掉。
  if (!paused.value && !settled.value) {
    doPause()
    helpPausedByPage = true
  }
  if (!helpSeen.value) {
    helpSeen.value = true
    uni.setStorageSync('battle_help_seen', '1')
  }
}

function closeHelp() {
  helpVisible.value = false
  if (helpPausedByPage) {
    helpPausedByPage = false
    doResume()
  }
}

/**
 * R29：打开完整反应时间线抽屉。
 *
 * 先无条件 closeHelp() 保证与图例互斥（图例暂停会被它自己恢复）；
 * 再从引擎只读旁路 + 服务端反应名构建展示快照；仅当当前未暂停且未结算时
 * 才代为暂停，并只记「由本抽屉暂停」，绝不碰图例/倍速的记账。
 */
function openReactionTimeline() {
  closeHelp()
  if (reactionTimelineVisible.value) {
    closeReactionTimeline()
    return
  }
  const eng = engine.value as BattleEngine | null
  const reactionDefs = store.config ? store.config.reactions : []
  const events = eng ? eng.reactionTimeline() : []
  const truncated = eng ? eng.reactionTimelineTruncated() : false
  r29Timeline.value = buildReactionTimelineView(
    events,
    Boolean(truncated),
    reactionDefs,
  )
  reactionTimelineVisible.value = true
  if (!paused.value && !settled.value) {
    doPause()
    timelinePausedByDrawer = true
  }
}

/** R29：关闭抽屉；只恢复本抽屉造成的暂停，不覆盖玩家原有暂停/图例/倍速。 */
function closeReactionTimeline() {
  reactionTimelineVisible.value = false
  if (timelinePausedByDrawer) {
    timelinePausedByDrawer = false
    doResume()
  }
}

/** 手牌稀有度中文名。 */
function rarityLabel(r: string): string {
  return r === 'rare' ? '稀有' : r === 'epic' ? '史诗' : '普通'
}

/** 手卡图标：技能卡用元素图形，属性卡用冲击星，机制卡用双环（反应）。 */
function handIconName(c: Card): string {
  if (c.kind === 'skill' && c.element) return `elem:${c.element}`
  return c.kind === 'mechanic' ? 'reaction' : 'elem:kinetic'
}

/** 手卡图标底座配色：跟随元素；无元素的类别卡走稀有度色。 */
function handBadgeClass(c: Card): string {
  if (c.element) return `badge-${c.element}`
  if (c.rarity === 'rare') return 'badge-rare'
  if (c.rarity === 'epic') return 'badge-epic'
  return ''
}

/** 结算奖励的货币图标（与顶栏资源胶囊同语言）。 */
function currencyIcon(k: string): string {
  if (k === 'gem') return 'gemcurrency'
  if (k === 'energy') return 'bolt'
  return 'coin'
}


/**
 * 取可用于绘制的 canvas 节点。
 *
 * ⚠️ 关键：H5 下 `<canvas type="2d">` 会被编译成 `<uni-canvas>` 自定义元素，
 * 它**没有 getContext 方法**。必须走 uni.createSelectorQuery().fields({node:true})，
 * 它返回平台内部持有的真实 canvas 节点。
 *
 * 之前这里先试 querySelector 拿 DOM 元素 —— 那在 H5 下会拿到无用的
 * `<uni-canvas>`，导致渲染层静默失效（战斗逻辑在跑但画面全黑）。
 * 因此统一只用 createSelectorQuery，mp-weixin 与 H5 行为一致。
 */
function getCanvas(): Promise<any | null> {
  const proxy = (inst as any)?.proxy
  return new Promise((resolve) => {
    uni
      .createSelectorQuery()
      .in(proxy)
      .select('#battle-canvas')
      .fields({ node: true, size: true }, (res: any) => {
        // 不同平台 / uni 版本下 node 可能是真实 canvas，也可能是包裹它的
        // 自定义元素（H5 的 <uni-canvas>）。后者没有 getContext，
        // 必须向下找到内部真正的 <canvas>，否则会静默画到错误元素上。
        let target: any = res?.node ?? null
        if (target && typeof target.getContext !== 'function') {
          target = target.querySelector?.('canvas') ?? null
        }
        if (target && typeof target.getContext === 'function') {
          const dpr = uni.getSystemInfoSync().pixelRatio || 1
          const cssW = res?.width ?? canvasW.value
          const cssH = res?.height ?? canvasH.value
          target.width = Math.floor(cssW * dpr)
          target.height = Math.floor(cssH * dpr)
          resolve(target)
        } else {
          console.warn('[来一炮] 未取得可绘制 canvas，回退到无渲染模式')
          resolve(null)
        }
      })
      .exec()
  })
}

/**
 * 无 canvas 时的逻辑推进（与渲染器的逻辑时钟等价，同样受倍速影响）。
 *
 * 抽成具名函数是为了换挡时能重建定时器 —— 此前它是 setup() 里的一个
 * 内联闭包，倍速改不到它，于是「兼容模式」下档位是个摆设。
 */
function noteReactions(events: unknown[], eng: BattleEngine | null): void {
  const d = damageShareFromEvents(events as Array<{ type?: string; damage?: bigint | number }>)
  if (d.reactionDamage > 0 || d.totalDamage > 0) {
    const prev = damageShare.value
    const totalDamage = prev.totalDamage + d.totalDamage
    const reactionDamage = prev.reactionDamage + d.reactionDamage
    damageShare.value = {
      reactionDamage,
      totalDamage,
      ratioPct: totalDamage > 0 ? Math.round(reactionDamage * 1000 / totalDamage) / 10 : null,
    }
  }
  const reactions = events.filter(
    (e): e is { type: 'reaction'; reaction: ReactionKey; damage: bigint | number; resisted: boolean; existingElement?: Element | ''; incomingElement?: Element | '' } =>
      (e as { type?: string })?.type === 'reaction',
  )
  if (!reactions.length) return
  const recent = reactions.slice(-3)
  const attribution = attributeLiveReactions(
    recent,
    eng?.reactionTimeline?.(),
    Boolean(eng?.reactionTimelineTruncated?.()),
  )
  const fresh = recent.map((e, idx) => {
    const cue = reactionCue(e.reaction, e.existingElement, e.incomingElement, e.damage)
    const row = attribution.rows[idx]
    return {
      id: ++reactionSeq,
      key: e.reaction,
      name: cue.name,
      pair: cue.pair,
      cue: cue.cue,
      damage: Math.max(0, Number(e.damage)),
      resisted: Boolean(e.resisted),
      wave: row?.wave ?? null,
      enemy_uid: row?.enemy_uid ?? null,
      available: Boolean(row?.available),
      attribution: attributionText(row),
    }
  })
  // R28 修复：AI 汇总分母必须与页面最终显示的记录一致，
  // 而不是当前 step 的反应数（否则本步只有 1 条时误报「已归因 1/1」）。
  const merged = [...fresh.reverse(), ...recentReactions.value].slice(0, 3)
  recentReactions.value = merged
  const mergedRows = merged.map(r => ({
    reaction: String(r.key),
    damage: r.damage,
    wave: r.available ? r.wave : null,
    enemy_uid: r.available ? r.enemy_uid : null,
    available: Boolean(r.available),
  }))
  const matchedCount = mergedRows.filter(r => r.available).length
  r28Ai.value = buildLiveFeedAi({
    rows: mergedRows,
    matchedCount,
    unavailableCount: mergedRows.length - matchedCount,
  })
}

function fallbackTick(): void {
  // ⚠️ 必须显式还原类型：`engine` 是 ref，Vue 的 `UnwrapRef` 会把类实例的
  // **类型**摊成结构映射（私有成员不在 keyof 里 → 方法看起来全丢了），
  // 而运行时的值仍是那个原始实例（reactive 不代理类实例）。
  // 不还原的话，`settle(eng)` 会报「缺 49 个属性」——一个纯类型假象。
  const eng = engine.value as BattleEngine | null
  if (!eng) return
  // 暂停守卫：与 rAF 路径同一条理由（定时器可能在暂停后被平台补发一次）
  if (paused.value) return
  const events = eng.step()
  playBattleEvents(events)
  noteReactions(events, eng)
  refreshElementHud(eng)
  refreshWaveProgress(eng)
  // 同上：step() 的内部写不走响应式代理，需手动触发
  triggerRef(engine)
  if (renderer.value) renderer.value.handleEvents(events)
  if (eng.phase === 'card_select') hand.value = [...eng.deck.hand]
  if (eng.phase === 'won' || eng.phase === 'lost') {
    if (timerId) {
      clearInterval(timerId)
      timerId = null
    }
    void settle(eng)
  }
}

/** 按当前倍速（重新）启动降级定时器。重复调用是安全的。 */
function startFallbackLoop(): void {
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  timerId = setInterval(fallbackTick, logicIntervalMs(speed.value))
}

async function setup() {
  const info = uni.getSystemInfoSync()
  // 桌面 H5 内容列限宽 560：画布跟随容器宽，否则画布会横向溢出内容列
  canvasW.value = Math.min(Number(info.windowWidth) || 375, 560)
  canvasH.value = Number(info.windowHeight) || 667

  // 必须等登录完成再申请凭证：否则可能带着上一个账号的 token 去开局，
  // 服务端会按那个账号扣体力，出现"钱包体力充足但开不了局"。
  if (!store.loggedIn) {
    const ok = await store.login()
    if (!ok) {
      startError.value = store.loginError || '登录失败'
      return
    }
  }
  // 第 137 轮：login 的 await 期间可能已退出页面
  if (pageUnloaded) return

  // 启动战斗：向服务端申请一次性凭证（扣体力 + 服务端种子）
  try {
    const bt = await api.startBattle(levelId.value)
    // startBattle 的 await 期间可能已退出页面
    if (pageUnloaded) return
    tokenId.value = bt.token_id
    const level = bt.level as GeneratedLevel
    const skills = store.skillMap

    /**
     * 技能槽位**必须用服务端下发的 slot**，不能用本地数组下标。
     *
     * 槽位顺序参与回放哈希：技能 id 相同但装在 0 号位还是 2 号位，
     * 引擎的事件序列就不同，哈希也不同。所以完全以
     * build.skills[].slot 为准 —— 服务端说什么就是什么。
     *
     * ⚠️ 这里**必须**调 `equippedFromSnapshot`，不能自己再拼一遍。
     *
     * 本文件此前持有该函数的**逐行复制**版本（实战开局拼一次、
     * 重放时 replay.ts 再拼一次）。两份都自洽，但：
     *  - 改一处忘另一处 → 实战伤害与重放伤害不同 → 哈希永远失配，
     *    而症状会把排查方向引向「I-6 验真坏了」，
     *    真因却是「构造逻辑有两份」
     *  - 技能等级系数只在一个地方烘进 baseDamage（见 skill.ts），
     *    两份拼法必然有一份漏掉等级
     */
    const equipped: EquippedSkill[] = equippedFromSnapshot(bt.build, skills)
    if (equipped.length === 0) {
      startError.value = '未装备任何技能，请先到「背包」页配置出战技能'
      return
    }
    store.setEquippedSkills(equipped.map((e) => e.skillId))
    // R22：已装备元素集合 E 直接取同一份权威快照的 applyElement，过滤空串去重；
    // 绝不凭卡牌 optimistic UI 重拼（与伤害/哈希同源，预判才可信）。
    equippedElements.value = Array.from(new Set(
      equipped.map((e) => e.applyElement).filter((e): e is Element => e !== ''),
    ))
    // 攻方属性以服务端下发为准（I-6 的前提）
    store.setBuild(bt.build)

    const eng = new BattleEngine({
      level,
      enemies: store.enemyMap,
      skills,
      equipped,
      attacker: store.attacker,
      // seed 服务端以字符串下发（避免 JSON number 精度损失），转 bigint 供 PRNG 用
      seed: BigInt(bt.seed),
      // 复活币的额外血量（第 153 轮）。**只加当前血量、不动 baseHpMax** ——
      // 漏怪伤害以 baseHpMax 为分母，改 level.base_hp 会把加成整个抵掉
      // （第 148–152 轮踩过的坑，见 domain/revive.go）。
      extraBaseHp: BigInt(bt.extra_base_hp ?? 0),
      // 可用槽位数同样由服务端权威下发（基础 5 槽 + 专精「额外插槽」）。
      //
      // ⚠️ 缺省绝不能变：绝大多数战报是 4 槽的，
      // 引擎缺省取 ACTIVE_SLOTS = 4 才让历史战报照常重放。
      // 专精那 8 个「额外插槽」节点此前完全惰性 ——
      // 客户端用编译期常量，根本不读服务端下发的槽位数。
      activeSlots: bt.build?.active_slots,
      // 第 133 轮：分数规则同样必须来自服务端 /config 的 score_rules。
      // 修前引擎恒用编译期 DEFAULT_SCORE_RULES —— 一旦服务端重标定
      // DefaultScoreRules，客户端星级/得分与结算、I-6 验真全部漂移。
      // config 未到达时 undefined → 引擎回落到 DEFAULT（离线兜底）。
      scoreRules: store.config?.score_rules
        ? scoreRulesFromServer(store.config.score_rules)
        : undefined,
    })
    recentReactions.value = []
    r28Ai.value = buildLiveFeedAi({ rows: [], matchedCount: 0, unavailableCount: 0 })
    reactionTimelineVisible.value = false
    timelinePausedByDrawer = false
    r29Timeline.value = buildReactionTimelineView(null, false, [])
    damageShare.value = { reactionDamage: 0, totalDamage: 0, ratioPct: null }
    settleCardPicks.value = []
    cardChoiceVersion.value++
    engine.value = eng
    eng.start()

    // 渲染
    const canvas = await getCanvas()
    // 第 137 轮：getCanvas 的 await 是关键窗口 —— 若期间已退出页面，
    // 不再创建/启动渲染器（否则会留下无人 stop 的 rAF/定时器 + 引擎步进）。
    if (pageUnloaded) return
    if (canvas) {
      const r = new BattleRenderer(canvas, eng)
      // 视口用 CSS 像素（逻辑缩放基准），而 canvas 内部是 dpr 放大后的像素。
      // 两者混用会把画面缩小 dpr 倍 —— 表现为"画布空白/元素挤在角落"。
      r.setViewport(canvasW.value, canvasH.value)
      // 持久化的档位要在建时钟**之前**交进去：start() 会按当前档位建
      // 逻辑时钟，之后再 setSpeed 也能生效，但会白建一次再拆掉。
      r.setSpeed(speed.value)
      renderer.value = r
      renderMode.value = 'raf'
      // 若在 `await getCanvas()` 期间已经 onHide 过，渲染器此刻才刚建好 ——
      // 把「暂停意图」补交给它，否则 UI 显示暂停而时钟在跑。
      if (paused.value) r.pause()
      r.start(() => {
        // 暂停守卫（2026-10-23）。
        //
        // 正常路径下 `renderer.pause()` 已经停掉了逻辑时钟，这里不会进。
        // 但**多一道守卫是有意的**：暂停的意义是「战斗时间不流逝」，
        // 而它依赖「没有别的路径在调 loop」。真机上定时器可能因平台行为
        // 在暂停后仍被触发一次（例如 JS 线程恢复时补发），
        // 那时若没有这行，玩家会看到「已暂停」却仍在掉血。
        if (paused.value) return
        const events = eng.step()
        // 音效与视觉同源：都从同一批事件来，不另算一份状态
        playBattleEvents(events)
        noteReactions(events, eng)
        refreshElementHud(eng)
        refreshWaveProgress(eng)
        // ⚠️ 引擎在 step() 内部以原始对象身份改写 phase/baseHp 等字段，
        // 这些写入不经过 engine ref 的响应式代理，模板里的 computed
        // （HP/波次/选牌界面）感知不到 —— 必须手动触发一次 ref。
        triggerRef(engine)
        r.handleEvents(events)
        if (eng.phase === 'card_select') {
          hand.value = [...eng.deck.hand]
        }
        if (eng.phase === 'won' || eng.phase === 'lost') {
          void settle(eng)
        }
      })
      loopStarted = true
    } else {
      renderMode.value = 'fallback'
      // 拿不到 canvas 时用定时器推进，保证逻辑仍可运行（便于自动化测试与无 canvas 环境）
      startFallbackLoop()
    }
  } catch (e) {
    startError.value = (e as Error).message
  }
}

// 2026-10-21（P4-5）：改成 ref —— 「放弃」按钮要在模板里读它
//（`v-if="!settled"`），普通 let 不是响应式的，模板不会跟着更新。
const settled = ref(false)

/**
 * 结算返回 404 `not_found` 表示这一局的 token 行不存在。
 *
 * 候选⑭曾计划在 token 签发语句上关掉 `synchronous_commit`。R151 复核后未启用：
 * 该 INSERT 与体力/复活币扣减在同一个事务，关掉会连带钱包一起失去崩溃持久性。
 * 这条路先按「万一令牌行丢了」的场景补齐：PG 自身是崩溃安全的，只有 OS 崩溃/
 * 断电才可能丢掉已下发的 token；这时不能把玩家这一局判掉 ——
 * 重取一次同关凭证，用新 token 重报。
 */
function isInvalidBattleToken(e: unknown): boolean {
  const err = e as { status?: number; code?: string }
  return err?.status === 404 && err?.code === 'not_found'
}

async function settle(eng: BattleEngine) {
  if (settled.value) return
  settled.value = true
  if (loopStarted && renderer.value) {
    renderer.value.stop()
    loopStarted = false
  }
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  try {
    let payload = eng.settleInput(tokenId.value)
    // R15：逐次反应证据随结算旁路上传；不改变权威字段，不改变 replay_hash。
    const timeline = eng.reactionTimeline?.()
    if (timeline !== undefined) {
      payload = { ...payload, reaction_timeline: timeline } as typeof payload
    }
    const reportedPicks = (payload as { card_picks?: unknown }).card_picks
    settleCardPicks.value = Array.isArray(reportedPicks) ? [...reportedPicks] : []
    let res: any
    try {
      res = await api.settleBattle(payload)
    } catch (e) {
      if (!isInvalidBattleToken(e)) throw e
      // 只重试一次：网络故障也可能表现为 404 之外的错误，
      // 但 404 若持续存在，再开局只会多扣一次体力，不会多拿奖励。
      const bt = await api.startBattle(levelId.value)
      tokenId.value = bt.token_id
      payload = { ...payload, token_id: bt.token_id }
      res = await api.settleBattle(payload)
    }
    settleResult.value = {
      battle_id: res.battle_id,
      win: res.win,
      stars: res.stars,
      score: res.score,
      kills: payload.kills,
      leaked: payload.leaked,
      reactions: payload.reactions,
      loot: res.loot ?? {},
      clamped: res.clamped,
      clamp_note: res.clamp_note,
      replay_hash: payload.replay_hash,
    }
    if (res.new_max_stage) store.setMaxStage(res.new_max_stage)
    await store.refreshProfile()
    // R10：本局是真实结算（带权威 replay_hash），用它和同关上一局做确定性差值。
    // 只填服务端/引擎已确证的字段，缺字段留在空态里，不补零冒充数据。
    if (!settleResult.value.error) {
      const current = {
        level_id: levelId.value,
        result: res.win ? 'win' : 'lose',
        stars: Number(res.stars ?? 0),
        score: Number(res.score ?? 0),
        kills: Number(payload.kills ?? 0),
        leaked: Number(payload.leaked ?? 0),
        reactions: Number(payload.reactions ?? 0),
        replay_hash: payload.replay_hash,
        created_at: new Date().toISOString(),
      } as import('@/api/client').BattleDetail
      await loadGrowthCompare(current)
    }
  } catch (e) {
    settleResult.value = {
      win: eng.phase === 'won',
      stars: 0,
      score: eng.score,
      kills: eng.kills,
      leaked: eng.leaked,
      reactions: eng.reactionsCount,
      loot: {},
      clamped: false,
      error: (e as Error).message,
      replay_hash: eng.replayHash(),
    }
  }
}

function takeCard(id: string) {
  if (!engine.value) return
  engine.value.takeCard(id)
  hand.value = [...engine.value.deck.hand]
  cardChoiceVersion.value++
}

function discardCard(id: string) {
  if (!engine.value) return
  engine.value.discardCard(id)
  hand.value = [...engine.value.deck.hand]
  cardChoiceVersion.value++
}

function skipCards() {
  if (!engine.value) return
  engine.value.skipCards()
  hand.value = []
  cardChoiceVersion.value++
}

function backToStage() {
  uni.navigateBack()
}

/**
 * 查看本局战报（2026-11-01 新一轮第 43 轮）。
 *
 * 结算响应带 `battle_id`，但此前没有任何消费 —— 玩家打完只能看到
 * 结算面板上的数字，想回顾掉落/元素/反应/地形没有入口。
 * 这里跳转战报页并携带 battle_id，战报页会自动展开这一局。
 */
function goBattleReport() {
  const id = Number(settleResult.value?.battle_id)
  if (Number.isInteger(id) && id > 0) {
    uni.navigateTo({ url: `/pages/battles/battles?battle_id=${id}` })
  }
}

/**
 * 再来一局（2026-11-01 新一轮第 53 轮）。
 * 用 redirectTo 替换当前战斗页，重新进入同一关。
 * 不用 navigateTo：否则每次重开都会叠加一个战斗页，页面栈无限增长。
 */
function retryLevel() {
  uni.redirectTo({ url: `/pages/battle/battle?level=${levelId.value}` })
}

/** R9 通关后的下一关入口：只认服务端解锁进度与真实存在的关卡配置。 */
const nextLevelId = computed(() => levelId.value + 1)
const canGoNextLevel = computed(() => {
  const r = settleResult.value
  return Boolean(
    r?.win && !settleFailed.value &&
      store.levelMap.has(nextLevelId.value) &&
      nextLevelId.value <= store.unlockedLevel,
  )
})
function goNextLevel() {
  if (!canGoNextLevel.value) return
  uni.redirectTo({ url: `/pages/stage/stage?level=${nextLevelId.value}` })
}

/**
 * 放弃这一局，退回体力（2026-10-21 P4-5）。
 *
 * 此前**没有放弃路径** —— 体力在开战时就扣了，
 * 玩家中途退出就等于「什么都没发生，但资源没了」。
 *
 * 放弃会**作废**这枚开局凭证（服务端标记为已用），
 * 所以放弃之后不能再结算这一局 —— 那是刻意的：
 * 放弃与结算必须是**互斥**的两条路，否则能「打完再退」。
 */
const abandoning = ref(false)

/** 放弃成功后的跳转定时器（页面卸载时必须清掉 —— 否则会弹掉上一页）。 */
let abandonNavTimer: ReturnType<typeof setTimeout> | null = null

async function abandon() {
  // ⚠️ 守卫必须在 showModal **之前**：
  // 放在之后的话，双击会弹出两个确认框，两个都 resolve → 发两次请求，
  // 第二次必然被服务端拒（这局已经结束了）→ 弹一个莫名其妙的错误。
  if (abandoning.value || !tokenId.value) return
  abandoning.value = true
  const ok = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '放弃这一局？',
      content: '体力会退还，但这一局没有任何战报与奖励。',
      confirmText: '放弃',
      cancelText: '继续打',
      success: (res) => resolve(!!res.confirm),
      fail: () => resolve(false),
    })
  })
  if (!ok) {
    abandoning.value = false
    return
  }
  try {
    const res = await api.abandonBattle(tokenId.value)
    await store.refreshWallet()
    uni.showToast({
      title: res.refunded_energy > 0 ? `已放弃，退回 ${res.refunded_energy} 体力` : '已放弃',
      icon: 'none',
    })
    settled.value = true
    // 停掉引擎：画面继续跑 800ms 与「已放弃」矛盾，
    // 而且引擎终局会触发 settle() —— 与 abandon() 并发。
    stopLoop()
    abandonNavTimer = setTimeout(() => {
      abandonNavTimer = null
      uni.navigateBack()
    }, 800)
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  } finally {
    abandoning.value = false
  }
}

/** 停掉渲染器与逻辑定时器（结算与放弃都要用）。 */
function stopLoop(): void {
  if (renderer.value) renderer.value.stop()
  renderer.value = null
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  loopStarted = false
}

onLoad(async (query) => {
  levelId.value = Number(query?.level ?? 1)
  await setup()
})

onUnload(() => {
  // 放弃后的跳转定时器必须清掉：
  // 玩家在这 800ms 内按系统返回键离开 → 定时器在已卸载页面上触发
  // `uni.navigateBack()` → 把**上一页**（选关页）一起弹掉。
  if (abandonNavTimer) {
    clearTimeout(abandonNavTimer)
    abandonNavTimer = null
  }
  // 第 137 轮：先置标志，让仍在飞行中的 setup()（各 await 之后）能感知到
  // 「页面已卸载」并停止创建渲染器/定时器。
  pageUnloaded = true
  if (renderer.value) renderer.value.stop()
  if (timerId) clearInterval(timerId)
})

// ── 暂停（2026-10-23）─────────────────────────────────────────────
//
// ## 为什么必须有
//
// 逻辑时钟是 `setInterval`，**与页面可见性无关** —— 这是 `canvas.ts`
// 当初为「后台也能跑逻辑」刻意做的设计（无头环境 / 真机切后台）。
// 副作用是：玩家切到微信聊天再回来，「战斗推进了多少」取决于
// 平台对后台定时器的处置（小程序挂起 JS、H5 标签页节流到 1Hz）。
// 也就是说这个行为**平台相关且未定义**。
//
// ## 为什么不自动恢复
//
// 回来就自动继续，玩家会发现自己离开的这几秒里防线被打掉一半 ——
// 那是比「暂停着等你点继续」糟糕得多的体感。所以 `onShow` 只**保持暂停**，
// 由玩家自己点「继续」。
//
// ## 为什么暂停不污染 replay_hash
//
// 暂停只是「不再 step」。`tick` 与 `elapsedMs` 都只在 `step()` 内自增，
// 于是 `duration_ms` 与不暂停时完全一致。判据见 battle.test.ts
// 「暂停后结算的 duration_ms 与不暂停相等」—— 那条一旦红，
// 意味着所有暂停过的战报都无法被重放验证。

const paused = ref(false)

/**
 * 音效开关（2026-10-23）。
 *
 * `initSfx()` 必须在读 `isMuted()` 之前调 —— 它负责把持久化偏好读进模块。
 * 这里用 ref 镜像一份，是为了让按钮的图标随点击立刻变（模块内的是普通变量，
 * 不参与响应式）。
 */
initSfx()
const muted = ref(isMuted())

function toggleMute(): void {
  const next = !muted.value
  muted.value = next
  setMuted(next)
  // 开启音效时给一声确认音：否则点了没反应，玩家不知道是开了还是没开
  if (!next) {
    unlockSfx()
    playSfxTap()
  }
}

/** 开关确认音。 */
function playSfxTap(): void {
  playSfx('tap')
}
/**
 * 暂停是由「切后台」触发的（模板里据此显示不同提示）。
 *
 * ⚠️ 必须是 `ref`，不能是普通 `let`：模板在 `v-if="paused"` 块里读它。
 * 第一版是 `let`，于是「玩家先手动点暂停（pausedByHide=false，遮罩已渲染），
 * 随后切后台（改成 true）」这个组合下，遮罩**不会重渲染** ——
 * 提示永远停在「战斗时间不会流逝」而少了「切到后台时自动暂停了」。
 */
const pausedByHide = ref(false)

/**
 * 暂停。
 *
 * `reason` 决定遮罩上的说明文案，也决定 `onShow` 时的行为（两者都不自动恢复）。
 */
function doPause(reason: 'user' | 'hide' = 'user'): void {
  if (paused.value || settled.value) return
  paused.value = true
  pausedByHide.value = reason === 'hide'
  if (renderer.value) renderer.value.pause()
  if (timerId) {
    // 降级路径（无 canvas）：本页自己的定时器也要停
    clearInterval(timerId)
    timerId = null
  }
}

function doResume(): void {
  if (!paused.value) return
  paused.value = false
  pausedByHide.value = false
  if (renderer.value) renderer.value.resume()
  else if (loopStarted || engine.value) startFallbackLoop()
}

onHide(() => {
  // 判据不是 `engine.value` —— 而是「战斗已开始、尚未结算」。
  //
  // 用 `engine.value` 会漏掉一个真实窗口：`eng.start()` 之后、`r.start()` 之前
  // 有一次 `await getCanvas()`。那期间 engine 已存在、renderer 还是 null、
  // timerId 也是 null —— 走 `engine.value` 分支会 `paused=true` 但两条时钟都没起，
  // 随后 setup 继续 `r.start()`，只能靠 loop 里的 `if (paused) return` 兜住。
  // 那行守卫是**第二道防线**，不该成为唯一防线。
  if (settled.value || startError.value) return
  doPause('hide')
})

onShow(() => {
  // 不自动恢复：回来就继续，玩家会发现自己离开的几秒里防线被打掉一半。
  //
  // 这里做一次**对账**：`paused` 是 UI 状态，`renderer.pause()` 是时钟状态，
  // 两者若不一致（例如渲染器尚未 start 时先暂停过），以渲染器为准把 UI 纠正过来。
  // 没有这次对账，`isPaused()` 就是个没人读的字段。
  if (!paused.value && renderer.value?.isPaused()) {
    paused.value = true
  }
})

/**
 * 分享（2026-10-23）。
 *
 * 未结算时**不得**出现分数/星级：那两个值此时还没有权威结果
 * （服务端结算才算数），拼进标题就是假数据。
 */
onShareAppMessage(() =>
  battleShare({
    // ⚠️ 判据是「**结果对象已就位**且没有错误」，不是「settled 已置位」。
    //
    // `settled.value = true` 在 settle() 的第一行，而 settleResult 要等
    // `await api.settleBattle` 返回后才赋值。这段窗口（弱网可达数秒）里
    // settleResult 是 null，`?.error` 得到 undefined，`!undefined === true`
    // → 走「已结算」分支，星级/分数取 `?? 0` —— 分享出去是
    // 「第 3 关 0 星 · 0 分」。share.ts 的注释专门写了要防这个。
    settled: !!settleResult.value && !settleResult.value.error,
    levelId: levelId.value,
    stars: Number(settleResult.value?.stars ?? 0),
    score: Number(settleResult.value?.score ?? 0),
  }),
)
</script>

<style scoped>
.battle-page {
  position: relative;
  /* 100vw 在 uni-page-body 限宽 560 的桌面 H5 下会比内容列更宽（含滚动条宽），
     造成横向溢出；100% 跟随容器即可 */
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

.battle-canvas {
  position: absolute;
  top: 0;
  left: 0;
}

.abandon-btn {
  /*!
   * ⚠️ 必须显式写 `pointer-events: auto`。
   *
   * `.overlay-top` 有 `pointer-events: none`（为了让顶栏不挡住画布点击），
   * 而 `pointer-events` 是**可继承**属性 —— 不写这一行，
   * 按钮就继承到 `none`，**永远点不动**（而且画面上看起来完全正常）。
   *
   * 这条是 P4-5「放弃战斗」的整个功能：按钮可见、点不动，
   * 玩家只会以为游戏卡了。
   */
  pointer-events: auto;
  display: flex;
  align-items: center;
  gap: 6rpx;
  height: 60rpx;
  padding: 0 22rpx;
  border-radius: 14rpx;
  background: rgba(255, 123, 114, 0.18);
  border: 1rpx solid rgba(255, 123, 114, 0.4);
  color: #ff7b72;
  font-size: 24rpx;
}

/*
 * 暂停按钮。
 *
 * ⚠️ 与 `.abandon-btn` / `.speed-row` 同一条坑：`.overlay-top` 的
 * `pointer-events: none` 是**可继承**属性，不写 `auto` 就永远点不动。
 * 这已经是本项目第三次踩它了 —— 所以这里连注释一起复制，
 * 而不是指望下一个加按钮的人记得。
 */
.mute-chip {
  /* 与 .abandon-btn / .speed-row / .pause-btn 同一条坑：父级 pointer-events: none */
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
  color: var(--text-2);
}

.pause-btn {
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(88, 166, 255, 0.18);
  border: 1rpx solid rgba(88, 166, 255, 0.4);
  color: #58a6ff;
}

/*
 * 战场图例按钮（2026-11-17）。
 * ⚠️ 与 .abandon-btn / .speed-row 同一条坑：`.overlay-top` 的
 * pointer-events: none 可继承，不写 auto 就点不动（见同文件其他 chip 注释）。
 */
.help-chip {
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
  color: var(--muted);
  font-size: 30rpx;
  font-weight: 700;
}

.help-chip.on {
  background: rgba(88, 166, 255, 0.18);
  border-color: rgba(88, 166, 255, 0.4);
  color: #58a6ff;
}

/* 第六批 R70 首次未读图例的呼吸提示 */
.help-chip.help-first {
  border-color: var(--fire);
  color: var(--fire);
  animation: glow-pulse 2s ease-in-out infinite;
}

/* 图例遮罩：结构同暂停遮罩，内容可滚动 */
.legend-overlay {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 30;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
}

.legend-box {
  width: 86%;
  max-height: 82vh;
  padding: 28rpx 28rpx 20rpx;
  border-radius: 20rpx;
  background: var(--glass-bg);
  border: 1rpx solid var(--glass-border);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-2);
}

.legend-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14rpx;
}

.legend-title {
  font-size: 32rpx;
  font-weight: 700;
  color: var(--text);
}

.legend-close {
  width: 56rpx;
  height: 56rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: var(--panel-2);
  color: var(--muted);
  font-size: 34rpx;
}

.legend-scroll {
  max-height: 62vh;
}

.legend-sec {
  padding: 14rpx 0;
  border-top: 1rpx solid var(--border-soft);
}

.legend-sec:first-child {
  border-top: none;
}

.legend-sec-title {
  display: block;
  font-size: 28rpx;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 10rpx;
}

.legend-p {
  display: block;
  font-size: 24rpx;
  line-height: 1.6;
  color: var(--muted);
  margin-bottom: 8rpx;
}

.legend-rows {
  display: flex;
  flex-direction: column;
  gap: 10rpx;
}

.legend-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 14rpx;
  font-size: 24rpx;
  color: var(--text-2);
  line-height: 1.5;
}

.legend-row .dot {
  width: 20rpx;
  height: 20rpx;
  border-radius: 50%;
  flex-shrink: 0;
}

.legend-row .dot-ring {
  background: transparent;
  border: 3rpx solid var(--border);
}

.legend-slot-demo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
  margin-bottom: 10rpx;
}

.slot-demo {
  position: relative;
  width: 64rpx;
  height: 64rpx;
  border-radius: 12rpx;
  background: rgba(22, 27, 34, 0.9);
  border: 2rpx solid #ff6b35;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.slot-cost {
  position: absolute;
  top: -10rpx;
  right: -10rpx;
  width: 30rpx;
  height: 30rpx;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
}

.slot-cost text {
  font-size: 16rpx;
  color: #ffd33d;
  font-weight: 700;
}

.legend-rxn {
  padding: 10rpx 0;
}

.legend-rxn-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: wrap;
  gap: 12rpx;
}

.legend-rxn-name {
  font-size: 26rpx;
  font-weight: 600;
  color: var(--text);
}

.pair-chip {
  display: inline-flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: wrap;
  gap: 6rpx;
}

.legend-rxn-descr {
  display: block;
  font-size: 22rpx;
  line-height: 1.5;
  color: var(--muted);
  margin-top: 4rpx;
}

.legend-foot {
  display: block;
  padding: 12rpx 0 4rpx;
  font-size: 22rpx;
  color: var(--dim);
}

/*
 * 暂停遮罩。
 *
 * 除了展示，它还承担一个功能：**挡住画布**。
 * 没有它的话暂停期间玩家仍能点画布，看起来「暂停了但还能操作」——
 * 而实际上那些点击会落到画布上并被忽略，是纯粹的困惑来源。
 */
.pause-overlay {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 30;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
}

.pause-box {
  width: 70%;
  padding: 40rpx 32rpx;
  border-radius: 20rpx;
  background: var(--glass-bg);
  border: 1rpx solid var(--glass-border);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16rpx;
}

.pause-title {
  font-size: 34rpx;
  font-weight: 700;
  color: var(--text);
}

.pause-hint {
  font-size: 22rpx;
  line-height: 1.6;
  text-align: center;
}

.pause-go {
  margin-top: 12rpx;
  width: 100%;
}

.mode-note {
  pointer-events: auto;
  padding: 4rpx 12rpx;
  border-radius: 8rpx;
  background: rgba(255, 209, 61, 0.2);
  color: #ffd33d;
  font-size: 22rpx;
}

/**
 * 倍速档位。
 *
 * ⚠️ 与 `.abandon-btn` 同一条坑：`.overlay-top` 有 `pointer-events: none`，
 * 而它是**可继承**属性 —— 不写 `auto`，四个档位看起来正常但一个都点不动。
 */
.speed-row {
  pointer-events: auto;
  display: flex;
  flex-direction: row;
  gap: 8rpx;
}

.speed-chip {
  /* 24rpx 字号 + 上下 8rpx 内边距 → 触控高度约 40rpx；
     仍比「放弃」按钮小，避免遮挡画面中央的战场 */
  min-width: 64rpx;
  height: 60rpx;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 14rpx;
  border: 1rpx solid var(--border-soft);
  background: rgba(0, 0, 0, 0.5);
  color: var(--muted);
  font-size: 24rpx;
}

.speed-chip.on {
  border-color: rgba(126, 231, 135, 0.6);
  background: rgba(126, 231, 135, 0.16);
  color: #7ee787;
  font-weight: 600;
}

.overlay-top {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  padding: 20rpx 20rpx 0;
  display: flex;
  flex-direction: column;
  gap: 14rpx;
  pointer-events: none;
}

/* HUD 行：胶囊各自排布；窄屏换行但胶囊本身绝不挤压成竖排 */
.hud-row {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  align-content: flex-start;
  align-items: center;
  gap: 8rpx 12rpx;
}

.hud-spacer {
  flex: 1;
}

/* 波次 / 击杀胶囊：真浮层才用玻璃，画布上方的 HUD 允许 */
.hud-pill {
  display: inline-flex;
  flex: 0 0 auto;
  flex-shrink: 0;
  align-items: center;
  height: 52rpx;
  padding: 0 14rpx;
  white-space: nowrap;
  border-radius: 999rpx;
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
  border: 1rpx solid var(--glass-border);
  box-shadow: inset 0 1rpx 0 rgba(255, 255, 255, 0.06);
}

.hp-box {
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
  border: 1rpx solid var(--glass-border);
  border-radius: 16rpx;
  padding: 12rpx 20rpx;
  min-width: 240rpx;
  flex-shrink: 0;
  box-shadow: inset 0 1rpx 0 rgba(255, 255, 255, 0.06), 0 6rpx 20rpx rgba(0, 0, 0, 0.30);
}

.hp-line {
  display: flex;
  flex-direction: row;
  align-items: baseline;
  justify-content: space-between;
  gap: 16rpx;
}

.hp-label {
  font-size: 24rpx;
  color: var(--muted);
}

.hp-value {
  font-size: 28rpx;
  font-weight: 700;
}

.hp-bar {
  height: 10rpx;
  background: rgba(255, 255, 255, 0.12);
  border-radius: 5rpx;
  margin-top: 10rpx;
  overflow: hidden;
}

.hp-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--corrosion), var(--ok));
  box-shadow: 0 0 12rpx rgba(126, 231, 135, 0.45);
  transition: width 0.18s ease-out;
}

/* 第六批 R5 当前关卡名 */
.hp-level {
  display: block;
  margin-top: 6rpx;
  font-size: 20rpx;
  color: var(--muted);
  max-width: 320rpx;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.wave-text,
.kill-text,
.star-gap-text,
.time-budget-text {
  font-size: 22rpx;
  color: var(--text-2);
  white-space: nowrap;
}

/* R8 时间预算胶囊：钳分中低调、临近免钳分变黄、已免钳分变绿，
   只改文字颜色/边框，不改变布局，也不盖住战场。 */
.time-budget {
  border-color: var(--border-soft);
}

.time-budget.warn {
  border-color: rgba(255, 209, 61, 0.7);
}

.time-budget.warn .time-budget-text {
  color: #ffd33d;
}

.time-budget.clamped {
  border-color: rgba(126, 231, 135, 0.7);
}

.time-budget.clamped .time-budget-text {
  color: #7ee787;
  font-weight: 600;
}

/* R28 归因：行内紧凑不换行；说明块可换行且不拦截画布点击，CTA 单独放行 */
.r28-attribution { font-size: 18rpx; color: #ffd33d; white-space: nowrap; }

/* t95：390×844 移动视口是 H5 主场景。让 HUD 行换行、胶囊内文字保持单行，
   并给反应因果链留出宽度；pointer-events 仍由 .overlay-top 关闭，不挡战场。 */
@media (max-width: 430px) {
  .hud-row {
    gap: 6rpx 8rpx;
  }
  .hud-spacer { display: none; }
  .hp-box {
    min-width: 0;
    padding: 8rpx 12rpx;
  }
  .hud-pill {
    height: 44rpx;
    padding: 0 10rpx;
  }
  .wave-text,
  .kill-text,
  .star-gap-text,
  .time-budget-text {
    font-size: 20rpx;
    white-space: nowrap;
  }
  .reaction-feed {
    gap: 4rpx;
  }
  .reaction-line {
    flex-wrap: wrap;
    row-gap: 2rpx;
    padding: 4rpx 10rpx;
    width: 100%;
    box-sizing: border-box;
    max-width: 100%;
    min-width: 0;
  }
  .reaction-line-main {
    gap: 8rpx;
    min-width: 0;
    flex-wrap: wrap;
  }
  .reaction-name,
  .reaction-damage {
    font-size: 20rpx;
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .r28-attribution {
    font-size: 18rpx;
    color: #ffd33d;
    white-space: normal;
    max-width: 100%;
    overflow-wrap: anywhere;
  }
  .reaction-cue {
    font-size: 18rpx;
    max-width: 100%;
    overflow-wrap: anywhere;
  }
}

/* R9 最近反应：只占顶部浮层一行，pointer-events 由 .overlay-top 关闭。 */
.reaction-feed {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 6rpx;
}

.reaction-line {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  align-items: baseline;
  padding: 6rpx 14rpx;
  border-radius: 999rpx;
  background: rgba(13, 17, 23, 0.82);
  border: 1rpx solid var(--border-soft);
}

.reaction-name {
  font-size: 22rpx;
  color: #ffd33d;
}

.reaction-line-main {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  align-items: baseline;
}

.reaction-damage {
  font-size: 22rpx;
  color: var(--text-2);
}

.reaction-cue {
  font-size: 20rpx;
  color: #58a6ff;
  line-height: 1.45;
  max-width: 620rpx;
}

.card-choice-feedback {
  display: block;
  margin-bottom: 16rpx;
  font-size: 22rpx;
  color: #58a6ff;
  line-height: 1.5;
}

.sheet {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(13, 17, 23, 0.96);
  border-top: 1rpx solid var(--border);
  padding: 28rpx 24rpx 40rpx;
  animation: fade-in-up 0.25s ease-out;
}

.card-header {
  margin-bottom: 20rpx;
}

.card-header .card-title {
  display: block;
  margin-bottom: 6rpx;
}

/* 选牌引导（口语化说明，见模板注释） */
.card-header .card-hint {
  display: block;
  font-size: 22rpx;
  line-height: 1.6;
  color: var(--dim);
}

/* R26 下一波敌情简报：限制高度可滚动，390×844 下手卡与按钮始终可见可点 */
.r26-wave-brief {
  max-height: 160rpx;
  overflow-y: auto;
  margin-bottom: 14rpx;
  padding: 8rpx 12rpx;
  border: 1rpx solid var(--border-soft);
  background: var(--panel-2);
  border-radius: 12rpx;
  font-size: 20rpx;
  line-height: 1.4;
}
.r26-brief-head { display: block; font-weight: 700; color: var(--accent); margin-bottom: 4rpx; }
.r26-enemy-list { display: flex; flex-direction: column; gap: 2rpx; }
.r26-enemy-row { display: flex; flex-direction: row; align-items: center; gap: 8rpx; white-space: nowrap; overflow: hidden; }
.r26-enemy-name { color: var(--text); overflow: hidden; text-overflow: ellipsis; }
.r26-enemy-count { color: var(--text); }
.r26-enemy-threat { color: var(--warn); }
.r26-unknown-tag { color: var(--muted); border: 1rpx solid var(--border-soft); border-radius: 6rpx; padding: 0 4rpx; }
.r26-brief-empty { color: var(--muted); }
.r26-ai-note { display: flex; flex-direction: column; gap: 2rpx; margin-top: 6rpx; padding-left: 8rpx; border-left: 3rpx solid var(--accent); }
.r26-ai-title { color: var(--accent); font-weight: 700; }
.r26-ai-conclusion { color: var(--text); }
.r26-ai-basis, .r26-ai-honesty { color: var(--text-2); }
.r26-ai-cta { color: var(--accent); }

.cards {
  display: flex;
  flex-direction: row;
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.hand-card {
  flex: 1;
  background: var(--panel);
  border: 2rpx solid var(--border-soft, var(--border));
  border-radius: 16rpx;
  padding: 20rpx 16rpx;
  display: flex;
  flex-direction: column;
  min-height: 260rpx;
  transition: transform 0.15s, border-color 0.2s, box-shadow 0.2s;
}

.hand-card:active {
  transform: scale(0.97);
}

.hand-card.rarity-rare {
  border-color: var(--ice-soft, rgba(88, 166, 255, 0.5));
  box-shadow: 0 0 0 1rpx rgba(88, 166, 255, 0.25), 0 4rpx 12rpx rgba(88, 166, 255, 0.15);
}
.hand-card.rarity-epic {
  border-color: var(--kinetic-soft, rgba(201, 167, 255, 0.6));
  box-shadow: 0 0 0 1rpx rgba(201, 167, 255, 0.25), 0 4rpx 12rpx rgba(201, 167, 255, 0.15);
}

.hand-kind {
  font-size: 22rpx;
  color: var(--muted);
}

/* 卡面顶部行：类别 + 稀有度徽章 */
.hand-top {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 8rpx;
}

/* 卡面图标座：元素 / 类别的视觉锚点（与图鉴同一套语言） */
.hand-icon {
  width: 84rpx;
  height: 84rpx;
  border-radius: 16rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--panel-2);
  color: var(--text-2);
  margin: 4rpx 0 10rpx;
  align-self: flex-start;
}

.hand-name {
  font-size: 28rpx;
  font-weight: 600;
  margin: 10rpx 0;
}

.hand-descr {
  font-size: 22rpx;
  color: var(--muted);
  line-height: 1.5;
  flex: 1;
}

.hand-foot {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-top: 12rpx;
}

.result-title {
  font-size: 44rpx;
  font-weight: 700;
  text-align: center;
  display: block;
  margin-bottom: 16rpx;
  color: var(--ok);
}

/* G91 战斗评级徽章 */
.grade-badge {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 12rpx;
  width: 240rpx;
  margin: 0 auto 16rpx;
  padding: 8rpx 0;
  border: 2rpx solid;
  border-radius: 999rpx;
}
.grade-letter { font-size: 40rpx; font-weight: 800; }
.grade-label { font-size: 22rpx; }
.next-goal {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: var(--muted);
  margin: -6rpx 0 16rpx;
  padding: 0 24rpx;
}

.stars-big {
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 16rpx;
  margin-bottom: 24rpx;
}

.star-big {
  font-size: 60rpx;
  color: var(--warn);
}

.star-big.off {
  color: var(--border);
}

.result-stats {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  margin-bottom: 20rpx;
}

.rstat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4rpx;
}

.clamp-note {
  font-size: 20rpx;
  color: var(--warn);
  background: rgba(255, 211, 61, 0.1);
  padding: 12rpx 16rpx;
  border-radius: 8rpx;
  margin-bottom: 16rpx;
}

.settle-attribution {
  display: block;
  text-align: center;
  font-size: 20rpx;
  color: var(--muted);
  line-height: 1.6;
  margin: -6rpx 20rpx 16rpx;
}

.settle-defeat-review {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: var(--bad);
  line-height: 1.55;
  margin: 0 20rpx 16rpx;
}

.settle-card-feedback {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: #58a6ff;
  line-height: 1.55;
  margin: 0 20rpx 16rpx;
}

.settle-local-note {
  display: block;
  text-align: center;
  font-size: 20rpx;
  color: var(--warn);
  margin-bottom: 10rpx;
}

.settle-error {
  display: flex;
  flex-direction: column;
  gap: 8rpx;
  background: rgba(255, 123, 114, 0.1);
  border-left: 6rpx solid var(--bad);
  padding: 12rpx 16rpx;
  border-radius: 8rpx;
  margin-bottom: 16rpx;
}

.settle-error-hint {
  font-size: 24rpx;
  color: var(--bad);
  line-height: 1.5;
}

.settle-error-detail {
  font-size: 20rpx;
  color: var(--dim);
  word-break: break-all;
  line-height: 1.4;
}

.reward-row {
  display: flex;
  flex-direction: row;
  gap: 20rpx;
  justify-content: center;
  margin-bottom: 12rpx;
}

.reward {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8rpx;
  font-size: 24rpx;
  color: var(--ok);
}

.hash-note {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: var(--dim);
  font-variant-numeric: tabular-nums;
}

/* t13 / F-10-1：战斗首屏五元素连锁仪表 */
.battle-sigil-rail {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10rpx;
  width: fit-content;
  max-width: 100%;
  padding: 8rpx 16rpx;
  border-radius: 999rpx;
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
  border: 1rpx solid var(--glass-border);
  pointer-events: none;
}

.sigil {
  width: 40rpx;
  height: 40rpx;
  flex: 0 0 40rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  border: 1rpx solid currentColor;
  font-size: 20rpx;
  font-weight: 700;
}

.sig-fire { color: var(--fire); }
.sig-ice { color: var(--ice); }
.sig-lightning { color: var(--lightning); }
.sig-corrosion { color: var(--corrosion); }
.sig-kinetic { color: var(--kinetic); }

.sigil-caption {
  margin-left: 4rpx;
  font-size: 19rpx;
  color: var(--text-2);
  white-space: nowrap;
}

/* R22 选卡反应协同预判：沿用既有 token，零新增视觉令牌 */
.r22-synergy-card {
  margin-top: 8rpx;
  padding-top: 6rpx;
  border-top: 1rpx solid var(--border-soft);
  font-size: 19rpx;
  line-height: 1.4;
}
.r22-synergy-line { color: var(--accent); font-weight: 700; }
.r22-synergy-tags { display: flex; flex-wrap: wrap; gap: 4rpx; margin: 3rpx 0; }
.r22-synergy-tag {
  padding: 2rpx 6rpx;
  border: 1rpx solid var(--fire);
  border-radius: 8rpx;
  color: var(--fire);
}
.r22-synergy-match, .r22-synergy-empty { color: var(--text-2); }
.r22-ai-note {
  margin-top: 14rpx;
  padding: 10rpx 12rpx;
  border-left: 4rpx solid var(--accent);
  background: var(--panel-2);
  border-radius: 8rpx;
  display: flex;
  flex-direction: column;
  gap: 4rpx;
}
.r22-ai-title { color: var(--accent); font-weight: 700; font-size: 21rpx; }
.r22-ai-conclusion { color: var(--text); font-size: 20rpx; }
.r22-ai-basis, .r22-ai-honesty { color: var(--text-2); font-size: 19rpx; }
.r22-ai-cta { color: var(--accent); font-size: 20rpx; font-weight: 700; }

/* R24 实时元素层 HUD：沿用既有 token，零新增视觉令牌；顶部浮层不拦截点击 */
.r24-element-hud {
  margin-top: 8rpx;
  padding: 8rpx 10rpx;
  background: var(--panel-2);
  border-left: 4rpx solid var(--accent);
  border-radius: 8rpx;
  font-size: 19rpx;
  line-height: 1.4;
  pointer-events: none;
}
.r24-enemy-row { display: flex; flex-wrap: wrap; gap: 6rpx; margin-bottom: 4rpx; }
.r24-enemy-name { color: var(--text); font-weight: 700; }
.r24-enemy-stacks { color: var(--fire); }
.r24-reaction-hint { color: var(--accent); flex-basis: 100%; }
.r24-hud-empty { color: var(--text-2); }
.r24-ai-note { display: flex; flex-direction: column; gap: 3rpx; margin-top: 6rpx; }
.r24-ai-title { color: var(--accent); font-weight: 700; font-size: 20rpx; }
.r24-ai-conclusion { color: var(--text); font-size: 19rpx; }
.r24-ai-basis, .r24-ai-honesty { color: var(--text-2); font-size: 18rpx; }
.r24-ai-cta { color: var(--accent); font-size: 19rpx; font-weight: 700; }

/* R27 波次推进：容器自身单行紧凑、不抢占既有胶囊；hud-row 仍保留 flex-wrap: wrap */
.r27-wave-progress {
  display: inline-flex;
  flex-direction: row;
  flex-wrap: nowrap;
  align-items: center;
  flex-shrink: 0;
  max-height: 52rpx;
  overflow: hidden;
  padding: 0 12rpx;
  border-radius: 999rpx;
  background: var(--panel-2);
  white-space: nowrap;
  pointer-events: none;
}
.r27-progress-body { display: inline-flex; flex-direction: row; align-items: center; gap: 8rpx; flex-shrink: 0; }
.r27-progress-track { width: 90rpx; height: 6rpx; border-radius: 999rpx; background: rgba(88,166,255,.16); overflow: hidden; flex-shrink: 0; }
.r27-progress-fill { height: 100%; background: var(--accent); }
.r27-progress-text, .r27-progress-empty { color: var(--text); font-size: 19rpx; flex-shrink: 0; }
.r27-progress-empty { color: var(--text-2); }
.r27-ai-note { display: flex; flex-direction: column; gap: 2rpx; margin-top: 6rpx; padding: 6rpx 10rpx; background: var(--panel-2); border-left: 4rpx solid var(--accent); border-radius: 8rpx; font-size: 18rpx; line-height: 1.35; pointer-events: none; }
.r27-ai-title { color: var(--accent); font-weight: 700; font-size: 19rpx; }
.r27-ai-conclusion { color: var(--text); font-size: 18rpx; }
.r27-ai-basis, .r27-ai-honesty { color: var(--text-2); font-size: 17rpx; }
.r27-ai-cta { color: var(--accent); font-size: 18rpx; font-weight: 700; }
.r28-feed-ai { display: flex; flex-direction: column; gap: 2rpx; max-width: 100%; margin-top: 4rpx; padding: 6rpx 10rpx; background: rgba(13, 17, 23, 0.82); border-left: 4rpx solid var(--accent); border-radius: 8rpx; font-size: 18rpx; line-height: 1.35; pointer-events: none; }
.r28-feed-ai-title { color: var(--accent); font-weight: 700; font-size: 19rpx; }
.r28-feed-ai-conclusion { color: var(--text); font-size: 18rpx; }
.r28-feed-ai-basis, .r28-feed-ai-honesty { color: var(--text-2); font-size: 17rpx; }
.r28-feed-cta { color: var(--accent); font-size: 18rpx; font-weight: 700; pointer-events: auto; }
/* R29 完整反应时间线抽屉：z-index 40 高于图例/暂停(30)与结算 sheet(auto) */
.r29-timeline-overlay {
  position: absolute;
  left: 0; top: 0; right: 0; bottom: 0;
  z-index: 40;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--glass-bg);
  -webkit-backdrop-filter: var(--glass-blur);
  backdrop-filter: var(--glass-blur);
}
.r29-timeline-box {
  width: 90%;
  max-height: 84vh;
  padding: 24rpx 24rpx 18rpx;
  border-radius: 20rpx;
  background: var(--glass-bg);
  border: 1rpx solid var(--glass-border);
  display: flex;
  flex-direction: column;
  gap: 6rpx;
  box-shadow: var(--shadow-2);
  pointer-events: auto;
}
.r29-timeline-head { display: flex; flex-direction: row; align-items: center; justify-content: space-between; }
.r29-timeline-title { color: var(--accent); font-weight: 700; font-size: 30rpx; }
.r29-timeline-close { width: 56rpx; height: 56rpx; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: var(--panel-2); color: var(--muted); font-size: 34rpx; }
.r29-timeline-conclusion { color: var(--text); font-size: 20rpx; }
.r29-timeline-empty { color: var(--text-2); font-size: 20rpx; padding: 20rpx 0; }
.r29-timeline-truncated { color: var(--warn); font-size: 18rpx; }
.r29-timeline-scroll { max-height: 70vh; overflow-y: auto; width: 100%; }
.r29-timeline-row { display: block; padding: 8rpx 0; border-bottom: 1rpx solid var(--border-soft); color: var(--text); font-size: 19rpx; line-height: 1.35; }
.r29-timeline-basis, .r29-timeline-honesty { color: var(--text-2); font-size: 17rpx; line-height: 1.35; }
/* t227 结算结果页时间线入口：可见可点，沿用现有 token，不新增 --r29- */
.r29-result-timeline-cta { margin-top: 20rpx; padding: 18rpx 24rpx; border-radius: 14rpx; background: var(--panel-2); border: 1rpx solid var(--glass-border); color: var(--accent); font-size: 20rpx; font-weight: 700; text-align: center; pointer-events: auto; }
</style>