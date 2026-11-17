<template>
  <view class="page-root" :style="{ paddingTop: statusBarPad }">
    <view class="pad safe-bottom">
      <!--
        返回入口提到独立首行（100 轮打磨 R16）。
        此前「‹ 返回」与头像挤在同一 flex 行里：返回文字 + 96rpx 头像 + 昵称区
        三者争宽，昵称一长就把返回挤到半截换行，观感错位（R0 截图取证）。
        改成 `page-head` 式的独立首行后，头像与昵称独占一整行。
      -->
      <view v-if="canGoBack" class="me-back-row">
        <text class="back-btn tap" @click="goBack">‹ 返回</text>
      </view>
      <view class="card">
        <view class="row" style="gap: 20rpx">
          <view class="avatar">炮</view>
          <view class="flex-1">
            <!-- 昵称 + 改名入口（2026-10-30 实质优化第 9 轮）。
                 昵称只在登录/引导时设置一次，此前**永远改不了**。
                 `PUT /me/nickname` 补上改名能力，上限 64 字。 -->
            <view v-if="!editingNickname" class="row" style="gap: 12rpx">
              <text class="nick">{{ store.nickname || '玩家' }}</text>
              <text class="dim link" @click="startEditNickname">改名</text>
            </view>
            <view v-else class="nick-edit-row">
              <input
                v-model="nicknameDraft"
                class="inp nick-inp"
                type="text"
                placeholder="新昵称（≤64 字）"
                placeholder-class="ph"
                @confirm="submitNickname"
              />
              <view class="btn btn-mini btn-primary" @click="submitNickname">保存</view>
              <view class="btn btn-mini" @click="cancelEditNickname">取消</view>
            </view>
            <text
              v-if="nicknameMsg"
              class="dim"
              style="display: block; margin-top: 4rpx"
              :style="{ color: nicknameErr ? 'var(--bad)' : 'var(--ok)' }"
            >
              {{ nicknameMsg }}
            </text>
            <view class="row" style="align-items: center; gap: 12rpx">
              <text class="dim">ID {{ store.userId }}</text>
              <!-- 复制用户 ID（2026-11-01 新一轮第 90 轮）：反馈/社区报问题时用 -->
              <text class="link" style="font-size: 22rpx" @click="copyUserId">复制</text>
            </view>
            <!-- 账号信息（2026-10-31 实质优化第 34 轮） -->
            <text v-if="store.accountCreatedAt" class="dim" style="display: block">
              注册于 {{ fmtAccountTime(store.accountCreatedAt) }}
            </text>
            <text v-if="store.accountLastLoginAt" class="dim" style="display: block">
              最近登录 {{ fmtAccountTime(store.accountLastLoginAt) }}
            </text>
            <!-- ⚠️ 这里原本写「微信账号」，但微信通道在本项目**未接通**
                 （WECHAT_APP_ID 未配置 → /auth/wechat 返回 501），
                 而 `is_guest=false` 现在还多了一种来源：**账号口令登录**。
                 继续写「微信账号」会把口令账号说成微信账号 —— 事实不对，
                 而「我明明是用账号密码登的」这种困惑很难自查。 -->
            <text class="dim">{{ store.isGuest ? '游客账号（未绑定微信）' : '账号登录' }}</text>
            <!-- 累计星级：文本 + G79 进度环（比例一眼可见） -->
            <view class="star-progress-line">
              <svg class="star-ring" viewBox="0 0 100 100" aria-label="累计星级进度">
                <circle class="sr-bg" cx="50" cy="50" r="40" />
                <circle
                  class="sr-fill" cx="50" cy="50" r="40" transform="rotate(-90 50 50)"
                  :stroke-dasharray="`${starRing.dash} ${starRing.rest}`"
                />
              </svg>
              <text class="dim">累计星级 {{ totalStars }} / {{ totalStarCap }}（{{ starRing.pct }}%）</text>
            </view>
          </view>
        </view>
      </view>

      <!--
        页签（2026-11-16）：本页此前 14 个卡片纵排（1532 行），「记录」类卡片
        （挑战/被挑战/战报/购买/兑换/订单）占了近一半，却与账号主体无关。
        拆成两个页签；用 `v-show` 保留全部 DOM，数据加载与测试断言不受影响。
      -->
      <view class="chip-row me-tabs">
        <text
          v-for="t in meTabDefs"
          :key="t.key"
          class="chip tap"
          :class="{ hit: meTab === t.key }"
          @click="meTab = t.key"
        >{{ t.label }}</text>
      </view>

      <view v-show="meTab === 'main'" class="card">
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

      <!-- 设置：战斗音效开关（2026-11-01 新一轮第 79 轮） -->
      <view v-show="meTab === 'main'" class="card">
        <text class="card-title">设置</text>
        <view class="row" style="justify-content: space-between; align-items: center">
          <text>战斗音效</text>
          <switch :checked="sfxMuted" color="#ff6b35" @change="onSfxToggle" />
        </view>
        <text class="muted" style="display: block; margin-top: 8rpx">
          关闭后战斗中的射击/反应/胜利等音效将静音，偏好会保存在本机。
        </text>
      </view>

      <!-- 离线收益（第六批 R1）：入口已迁到「签到」页，与每日签到放在一起。
           两个每日领取动作不再分跨两页；此处不再重复渲染。 -->

      <view v-show="meTab === 'main'" class="card">
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

      <view v-show="meTab === 'main'" class="card">
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
        <!-- 护盾剩余时间（2026-10-31 新一轮第 33 轮） -->
        <text v-if="shieldRemainText" class="dim" style="display: block; margin-top: 8rpx; text-align: center">
          {{ shieldRemainText }}
        </text>
        <!-- 删除防线（2026-10-31 实质优化第 32 轮）：
             玩家不想被挑战时，此前没有任何办法清空防线。 -->
        <view
          v-if="myDefense"
          class="btn"
          style="margin-top: 12rpx; color: var(--bad)"
          @click="onDeleteMyDefense"
        >
          删除防线
        </view>
      </view>

      <view v-if="candidates.length" v-show="meTab === 'main'" class="card">
        <text class="card-title">可挑战的防线<text class="cand-count">{{ candidates.length }} 条</text></text>
        <text class="dim" style="display: block; margin-bottom: 12rpx">
          挑战在<b>你的设备上本地模拟</b>：用你的构筑攻对方快照，战斗数据不上传。
          攻穿即可窃取对方 10% 资源。今日剩余 {{ attackLeft }} 次。
        </text>
        <view v-for="c in candidates" :key="c.id" class="cand">
          <view class="flex-1">
            <text class="cand-name">{{ c.owner_name || '玩家' }} 的防线</text>
            <text class="dim" style="display: block">
              战力 {{ c.power }}
              <text class="power-diff" :class="powerDiff(c) <= 0 ? 'easier' : 'harder'">
                {{ powerDiff(c) <= 0 ? '弱于我 ' + Math.abs(powerDiff(c)) : '强于我 +' + powerDiff(c) }}
              </text>
              · 元素 {{ c.element_coverage }}/5 · {{ c.wins }}胜{{ c.losses }}负
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

      <!-- R55 候选为空时给解释，而不是整块消失 -->
      <view v-else v-show="meTab === 'main'" class="card empty-block">
        <text class="empty-glyph">🛡️</text>
        <text class="muted">暂时没有其他玩家的防线</text>
        <text class="dim">先保存一套自己的防线，等别人设置后就能在这里挑战。</text>
      </view>

      <view
        v-if="lastChallenge"
        v-show="meTab === 'main'"
        class="card pop-in"
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

      <!-- G110 全账号统一时间轴 -->
      <view v-if="unifiedTimeline.length && meTab === 'records'" class="card timeline-card">
        <view class="card-title">最近动态</view>
        <view v-for="(e, i) in unifiedTimeline" :key="i" class="tl-row">
          <text class="tl-kind">{{ e.kind }}</text>
          <text class="flex-1 tl-text">{{ e.text }}</text>
          <text class="dim">{{ fmtChallengeTime(e.at) }}</text>
        </view>
      </view>

      <!-- 挑战历史（2026-10-30 实质优化第 18 轮）。
           此前玩家只能看到最近一次挑战结果，历史记录没有入口。 -->
      <view v-if="challengeHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">挑战历史<text class="cand-count">{{ challengeHistory.length }} 条</text></text>
        <!-- G89 挑战胜负统计 + 近期序列 -->
        <view class="challenge-stats">
          <text class="dim">{{ challengeStats.win }} 胜 {{ challengeStats.lose }} 负 · 胜率 {{ challengeStats.total ? Math.round(challengeStats.win / challengeStats.total * 100) : 0 }}%</text>
          <view class="challenge-seq">
            <view
              v-for="(w, i) in challengeStats.recent"
              :key="i"
              class="cs-dot"
              :class="w ? 'win' : 'lose'"
            />
          </view>
        </view>
        <view
          v-for="h in challengeHistory"
          :key="h.id"
          class="ch-row"
          :style="{ borderColor: h.won ? 'var(--ok)' : 'var(--bad)' }"
        >
          <view class="flex-1 row" style="gap: 8rpx; min-width: 0">
            <text class="mono" :style="{ color: h.won ? 'var(--ok)' : 'var(--bad)' }">
              {{ h.won ? '胜' : '负' }}
            </text>
            <text class="dim ellipsis">{{ h.defense_name }} · {{ h.owner_nickname }}</text>
            <!-- 偷取明细（2026-10-31 实质优化第 36 轮） -->
            <text v-if="lootText(h.loot)" class="dim ellipsis" style="color: var(--ok)">
              +{{ lootText(h.loot) }}
            </text>
          </view>
          <text class="dim">{{ fmtChallengeTime(h.created_at) }}</text>
        </view>
      </view>

      <!-- 被挑战记录（2026-10-31 新一轮第 3 轮）。
           此前只有挑战者能看历史，防守方不知道谁攻过自己的防线。 -->
      <view v-if="defenseChallengeHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">被挑战记录<text class="cand-count">{{ defenseChallengeHistory.length }} 条</text></text>
        <view
          v-for="h in defenseChallengeHistory"
          :key="h.id"
          class="ch-row"
          :style="{ borderColor: h.won ? 'var(--bad)' : 'var(--ok)' }"
        >
          <view class="flex-1 row" style="gap: 8rpx; min-width: 0">
            <text class="mono" :style="{ color: h.won ? 'var(--bad)' : 'var(--ok)' }">
              {{ h.won ? '被攻破' : '守住' }}
            </text>
            <text class="dim ellipsis">{{ h.challenger_nickname }}</text>
            <!-- 被偷走的东西（防守方视角是损失） -->
            <text v-if="lootText(h.loot)" class="dim ellipsis" style="color: var(--bad)">
              -{{ lootText(h.loot) }}
            </text>
          </view>
          <text class="dim">{{ fmtChallengeTime(h.created_at) }}</text>
        </view>
      </view>

      <!-- G114 被挑战损失排行 -->
      <view v-if="topLooters.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">损失排行<text class="dim">（防线被破 Top5）</text></text>
        <view v-for="(l, i) in topLooters" :key="i" class="tl-row">
          <text class="dim">{{ i + 1 }}. {{ l.name }}</text>
          <view class="flex-1 loot-track"><view class="loot-fill" :style="{ width: Math.min(100, l.total / 5) + '%' }" /></view>
          <text class="mono">{{ l.total }}</text>
        </view>
      </view>

      <!-- 最近战报（2026-10-30 实质优化第 19 轮）。
           玩家打完关卡后没有任何历史战报入口。 -->
      <view v-if="battleHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">最近战报</text>
        <view v-for="b in battleHistory" :key="b.id" class="ch-row">
          <view class="flex-1" style="min-width: 0">
            <view class="row" style="gap: 8rpx; min-width: 0">
              <text class="mono">第 {{ b.level_id }} 关</text>
              <text class="dim ellipsis">
                {{ b.result === 'win' ? '胜利' : '失败' }} · {{ b.stars }} 星 · {{ b.score }} 分
              </text>
              <!-- 从战报进入验真（2026-10-31 实质优化第 35 轮） -->
              <text class="dim link" @click="goVerify(b)">验真</text>
              <!-- 战报详情（2026-10-31 新一轮第 10 轮） -->
              <text class="dim link" @click="toggleBattleDetail(b)">详情</text>
            </view>
            <!-- 展开的单局完整详情 -->
            <view v-if="battleDetail && battleDetail.id === b.id" style="margin-top: 8rpx">
              <view class="kv"><text class="muted">掉落</text><text class="mono">{{ lootText(battleDetail.loot) || '无' }}</text></view>
              <view class="kv"><text class="muted">元素使用</text><text class="mono">{{ mapText(battleDetail.elements_used) || '无' }}</text></view>
              <view class="kv"><text class="muted">反应</text><text class="mono">{{ mapText(battleDetail.reactions_used) || '无' }}</text></view>
              <view class="kv"><text class="muted">地形</text><text class="mono">{{ (battleDetail.terrain_used || []).join('、') || '无' }}</text></view>
              <view class="kv"><text class="muted">命中</text><text class="mono">{{ battleDetail.hits }}/{{ battleDetail.shots }}</text></view>
              <view class="kv"><text class="muted">峰值热量</text><text class="mono">{{ battleDetail.heat_max }}</text></view>
              <view class="kv"><text class="muted">剩余血量</text><text class="mono">{{ battleDetail.hp_left }}%</text></view>
              <view class="kv"><text class="muted">到达波次</text><text class="mono">{{ battleDetail.wave_reached }}</text></view>
              <view class="kv"><text class="muted">时长</text><text class="mono">{{ Math.round(battleDetail.duration_ms / 1000) }}s</text></view>
            </view>
          </view>
          <text class="dim">{{ fmtChallengeTime(b.created_at) }}</text>
        </view>
      </view>

      <!-- 购买记录（2026-10-30 实质优化第 20 轮）。
           玩家每次商城购买都落库，但没有任何入口查看历史。 -->
      <view v-if="purchaseHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">购买记录</text>
        <!-- G84 消费统计摘要 -->
        <view class="spend-summary">
          <text v-for="s in spendSummary" :key="s.key" class="spend-chip">{{ s.label }}累计 {{ s.total }}</text>
          <text class="dim">共 {{ purchaseHistory.length }} 笔</text>
        </view>
        <view v-for="p in purchaseHistory" :key="p.id" class="ch-row">
          <view class="flex-1 row" style="gap: 8rpx; min-width: 0">
            <text class="mono">{{ p.item_name }}</text>
            <text class="dim ellipsis">{{ priceText(p.price) }}</text>
          </view>
          <text class="dim">{{ fmtChallengeTime(p.created_at) }}</text>
        </view>
      </view>

      <!-- 兑换记录（2026-10-30 实质优化第 21 轮）。
           玩家每次兑换都落库，但没有任何历史入口。 -->
      <view v-if="redemptionHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">兑换记录</text>
        <view v-for="r in redemptionHistory" :key="r.id" class="ch-row">
          <view class="flex-1 row" style="gap: 8rpx; min-width: 0">
            <text class="mono ellipsis">{{ r.code }}</text>
            <text class="dim ellipsis">{{ priceText(r.reward) }}</text>
          </view>
          <text class="dim">{{ fmtChallengeTime(r.created_at) }}</text>
        </view>
      </view>

      <!-- 订单记录（2026-10-31 实质优化第 27 轮）。
           支付订单（mock provider）此前玩家端没有任何入口。 -->
      <view v-if="orderHistory.length" v-show="meTab === 'records'" class="card">
        <text class="card-title">订单记录</text>
        <view v-for="o in orderHistory" :key="o.id" class="ch-row">
          <view class="flex-1" style="min-width: 0">
            <text class="mono ellipsis" style="display: block">
              {{ o.sku }} · {{ (o.amount / 100).toFixed(2) }} 元
            </text>
            <text class="dim ellipsis" style="display: block">
              {{ orderStatusText(o.status) }} · {{ o.order_no }}
            </text>
          </view>
          <text class="dim">{{ fmtChallengeTime(o.created_at) }}</text>
        </view>
      </view>

      <!-- G142 订单与兑换总账 -->
      <view v-if="orderSummary.orders || orderSummary.redemptions" v-show="meTab === 'records'" class="card">
        <text class="card-title">消费总账</text>
        <view class="spend-summary">
          <text class="spend-chip">订单 {{ orderSummary.orders }} 笔 · 共 {{ orderSummary.amount }} 元</text>
          <text class="spend-chip">兑换 {{ orderSummary.redemptions }} 次</text>
        </view>
      </view>

      <!-- 第六批 R40：六类记录全空时此前是纯空白，新玩家切「记录」页以为坏了。 -->
      <view v-if="noRecordsYet && meTab === 'records'" class="card empty-block">
        <text class="empty-glyph">📜</text>
        <text class="card-title">还没有任何记录</text>
        <text class="muted">打一局战斗、挑战他人防线或逛逛商城后，这里会逐条记下。</text>
      </view>
    </view>

    <!--
      账号区。

      ⚠️ 「退出登录」放在这里不是为了功能完整，而是因为
      **服务端此前没有任何切断会话的端点** ——
      手机丢了 / 令牌被读走之后，玩家唯一能做的「登出」只是清本地，
      而 refresh token 仍有 30 天有效期。
      服务端 `POST /me/logout` 补上的是「吊销全部」，
      这个按钮是它唯一的客户端入口（`TestEveryRouteHasClientCaller` 守这条）。

      ⚠️ 必须二次确认：它会**吊销该账号的全部会话**（不只是这台设备），
      在别的设备上登录的玩家会一起掉线。这是刻意的（登出的语义是
      「这个人不信任了」），但也必须让用户知道。

      修改密码（2026-10-30 实质优化第 7 轮）：
      账号口令登录的玩家此前**没有改密入口** —— 口令只能由运维改环境变量
      重启服务来轮换。这里补上 `POST /me/password` 的客户端入口：
      验证原口令 → 更新哈希 → 吊销全部旧会话（其它设备需要重新登录）。
      游客号没有口令，不显示该入口。

      游客转正（2026-10-30 实质优化第 8 轮）：
      游客号的全部数据只靠本地 guest_token 保护，清缓存 / 换设备就永久丢失。
      这里补上 `POST /me/upgrade` 的客户端入口：设置用户名 + 密码后，
      可以在任何设备登录回同一个号（guest_token 保留，老客户端不受影响）。
      已转正的账号显示改密入口，不再显示本入口。
    -->
    <view class="card">
      <text class="card-title">账号</text>

      <template v-if="!store.isGuest">
        <view v-if="!changingPassword">
          <text class="muted" style="display: block; margin-bottom: 16rpx">
            定期更换口令，可以防止旧口令泄露后账号被他人登录。
          </text>
          <button class="btn" style="margin-bottom: 16rpx" @click="changingPassword = true">修改密码</button>
        </view>
        <view v-else class="pwd-form">
          <input
            v-model="oldPassword"
            class="inp"
            type="text"
            password
            placeholder="原口令"
            placeholder-class="ph"
            :disabled="passwordSubmitting"
          />
          <input
            v-model="newPassword"
            class="inp"
            type="text"
            password
            placeholder="新口令（至少 10 位）"
            placeholder-class="ph"
            :disabled="passwordSubmitting"
            @confirm="submitPassword"
          />
          <view
            class="btn btn-primary"
            :class="{ 'btn-disabled': passwordSubmitting || !oldPassword || !newPassword }"
            @click="submitPassword"
          >
            {{ passwordSubmitting ? '提交中…' : '确认修改' }}
          </view>
          <view class="btn" style="margin-top: 12rpx" @click="cancelPassword">取消</view>
          <text
            v-if="passwordMsg"
            class="muted"
            style="display: block; margin-top: 12rpx"
            :style="{ color: passwordErr ? 'var(--bad)' : 'var(--ok)' }"
          >
            {{ passwordMsg }}
          </text>
        </view>
      </template>

      <template v-else>
        <view v-if="!upgrading">
          <text class="muted" style="display: block; margin-bottom: 16rpx">
            当前是游客账号，数据只保存在本机。设置用户名和密码后，
            可以在任何设备登录回同一个账号。
          </text>
          <button class="btn" style="margin-bottom: 16rpx" @click="upgrading = true">设置账号密码</button>
        </view>
        <view v-else class="pwd-form">
          <input
            v-model="upgradeUsername"
            class="inp"
            type="text"
            placeholder="用户名"
            placeholder-class="ph"
            :disabled="upgradeSubmitting"
          />
          <input
            v-model="upgradePassword"
            class="inp"
            type="text"
            password
            placeholder="密码（至少 10 位）"
            placeholder-class="ph"
            :disabled="upgradeSubmitting"
            @confirm="submitUpgrade"
          />
          <view
            class="btn btn-primary"
            :class="{ 'btn-disabled': upgradeSubmitting || !upgradeUsername || !upgradePassword }"
            @click="submitUpgrade"
          >
            {{ upgradeSubmitting ? '提交中…' : '确认升级' }}
          </view>
          <view class="btn" style="margin-top: 12rpx" @click="cancelUpgrade">取消</view>
          <text
            v-if="upgradeMsg"
            class="muted"
            style="display: block; margin-top: 12rpx"
            :style="{ color: upgradeErr ? 'var(--bad)' : 'var(--ok)' }"
          >
            {{ upgradeMsg }}
          </text>
        </view>
      </template>

      <text class="muted" style="display: block; margin: 16rpx 0">
        退出后本机的令牌立即失效，并会通知服务端吊销该账号的全部登录会话（其他设备也会被登出）。
      </text>
      <button class="btn" @click="confirmLogout">退出登录</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { currencyName } from '@/utils/labels'
import { useStatusBarPad } from '@/utils/statusBar'
import { isMuted, setMuted } from '@/audio/sfx'
import { ref, computed, onMounted } from 'vue'
import {onShow, onUnload} from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import {
  runChallenge,
  validateSnapshot,
  isShieldActive,
  type DefenseView,
  type ChallengeOutcome,
} from '@/game/defense'
import { scoreRulesFromServer } from '@/game/score'
import { skillBaseDamageAtLevel, DEFAULT_SKILL_RULES, skillRulesFromServer } from '@/game/skill'
import type { EquippedSkill } from '@/game/heatmap'
import type { Element } from '@/game/elements'
import type { GeneratedLevel } from '@/game/types'

const store = useGameStore()

/**
 * 战斗音效开关（2026-11-01 新一轮第 79 轮）。
 * 音效系统支持静音持久化（lyp_sfx_muted），但此前没有任何 UI 入口。
 */
const sfxMuted = ref(isMuted())

function onSfxToggle(e: { detail?: { value?: boolean } }) {
  const next = e.detail?.value === true
  setMuted(next)
  sfxMuted.value = next
}

/**
 * 复制用户 ID（2026-11-01 新一轮第 90 轮）。
 * 玩家在社区/反馈中报问题时，把 ID 复制给管理员。
 */
function copyUserId() {
  uni.setClipboardData({
    data: String(store.userId),
    success: () => {
      uni.showToast({ title: '已复制用户 ID', icon: 'success' })
    },
  })
}
/**
 * 延迟回调的句柄（2026-10-21 P6-7）。
 *
 * 此前那个 `setTimeout(async () => { ...写请求... })` **从不清除**：
 * 页面已经卸载了，回调照样跑、照样发 POST。
 * 症状是「离开了这个页面，它还在提交」——
 * 对一个会**改数据**的端点（验真、挑战防线）来说这是实打实的副作用。
 */
let pendingTimer: ReturnType<typeof setTimeout> | null = null
/** 页面是否已卸载。卸载后不再执行任何延迟回调。 */
let pageUnloaded = false

/** 延迟执行；若页面已卸载则**不执行**（P6-7）。 */
function schedulePending(fn: () => void, ms: number): void {
  if (pendingTimer) clearTimeout(pendingTimer)
  pendingTimer = setTimeout(() => {
    pendingTimer = null
    if (pageUnloaded) return
    fn()
  }, ms)
}


/** 顶部让开状态栏（2026-10-21 P6-2）。 */
const { statusBarPad } = useStatusBarPad()

/**
 * 返回入口（2026-11-16）。
 *
 * `getCurrentPages()` 是 uni-app 的页面栈：长度 > 1 说明是从别的页面
 * navigateTo 过来的，返回有意义；直接打开 / 刷新后栈长 1，返回无目标 ——
 * 此时**不渲染**按钮，而不是渲染一个点了没反应的按钮。
 */
const canGoBack = (() => {
  try {
    return getCurrentPages().length > 1
  } catch {
    return false
  }
})()

function goBack(): void {
  uni.navigateBack({ delta: 1 })
}

/**
 * 「我的」页签（2026-11-16）。
 * main = 账号 / 资源 / 设置 / 防线（原首屏），records = 六类历史流水。
 * 默认 main，玩家进入本页的第一眼内容不变。
 */
type MeTab = 'main' | 'records'
const meTab = ref<MeTab>('main')
const meTabDefs: Array<{ key: MeTab; label: string }> = [
  { key: 'main', label: '账号与防线' },
  { key: 'records', label: '记录' },
]

/** 第六批 R40：六类历史全空 → 给「记录」页一个可执行空态，而不是纯空白。 */
/** G142 订单与兑换汇总：累计订单金额（分→元）+ 兑换次数，补记录页的总账视角。 */
const orderSummary = computed(() => ({
  orders: orderHistory.value.length,
  amount: (orderHistory.value.reduce((s, o) => s + (o.amount ?? 0), 0) / 100).toFixed(2),
  redemptions: redemptionHistory.value.length,
}))

/** G114 被挑战损失排行：按每次被破防的 loot 总和取 Top5，看防线最常被谁攻破。 */
const topLooters = computed(() =>
  defenseChallengeHistory.value
    // won=true = 挑战者获胜 = 防线被破，loot 才有偷取明细；只统计被破记录
    .filter((d) => d.won)
    .map((d) => ({
      name: d.challenger_nickname || '匿名',
      total: Object.values(d.loot ?? {}).reduce((s, v) => s + v, 0),
    }))
    .sort((a, b) => b.total - a.total)
    .slice(0, 5),
)

/** G110 全账号统一时间轴：合并六类记录按时间倒序，跨类看「最近干了什么」。 */
const unifiedTimeline = computed(() => {
  type Ev = { at: string; kind: string; text: string }
  const evs: Ev[] = []
  for (const b of battleHistory.value) evs.push({ at: b.created_at, kind: '战斗', text: `第 ${b.level_id} 关 · ${b.result === 'win' ? '胜' : '负'} ${b.score}分` })
  for (const c of challengeHistory.value) evs.push({ at: c.created_at, kind: '挑战', text: c.won ? '攻破成功' : '挑战失败' })
  // won=true = 挑战者获胜 = 防线被破（与模板 :350 h.won?'被攻破':'守住' 同口径）
  for (const d of defenseChallengeHistory.value) evs.push({ at: d.created_at, kind: '被挑战', text: d.won ? '防线被破' : '防线守住' })
  for (const p of purchaseHistory.value) evs.push({ at: p.created_at, kind: '购买', text: p.item_name })
  for (const r of redemptionHistory.value) evs.push({ at: r.created_at, kind: '兑换', text: r.code })
  for (const o of orderHistory.value) evs.push({ at: o.created_at, kind: '订单', text: o.sku })
  return evs.filter((e) => e.at).sort((a, b) => new Date(b.at).getTime() - new Date(a.at).getTime()).slice(0, 20)
})

const noRecordsYet = computed(() =>
  !challengeHistory.value.length &&
  !defenseChallengeHistory.value.length &&
  !battleHistory.value.length &&
  !purchaseHistory.value.length &&
  !redemptionHistory.value.length &&
  !orderHistory.value.length,
)

const myDefense = ref<DefenseView | null>(null)
const candidates = ref<DefenseView[]>([])
const attackLeft = ref(3)
const shielded = ref(false)
const simulating = ref(0)
const lastChallenge = ref<(ChallengeOutcome & { stolen?: Record<string, number> }) | null>(null)

/**
 * 护盾剩余时间文案（2026-10-31 新一轮第 33 轮）。
 * 玩家开启护盾后不知道还剩多久 —— 这里用服务端时钟基准计算并展示。
 */
const shieldRemainText = computed(() => {
  const until = myDefense.value?.shielded_until
  if (!shielded.value || !until) return ''
  const remainMs = new Date(until).getTime() - store.estimateServerNowMs()
  if (remainMs <= 0) return ''
  const hours = remainMs / 3600000
  if (hours >= 1) return `护盾剩余 ${hours.toFixed(1)} 小时`
  const minutes = Math.max(1, Math.round(remainMs / 60000))
  return `护盾剩余 ${minutes} 分钟`
})

// 挑战历史（2026-10-30 实质优化第 18 轮）。
const challengeHistory = ref<api.ChallengeHistoryItem[]>([])

/** G89 挑战战绩统计：胜负次数 + 近 N 次胜负序列（数据来自 challengeHistory[].won）。 */
const challengeStats = computed(() => {
  const list = [...challengeHistory.value].reverse() // 时间正序
  const win = list.filter((h) => h.won).length
  return {
    win,
    lose: list.length - win,
    total: list.length,
    recent: list.slice(-12).map((h) => h.won),
  }
})

// 被挑战记录（2026-10-31 新一轮第 3 轮）。
const defenseChallengeHistory = ref<api.DefenseChallengeItem[]>([])

function fmtChallengeTime(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${mm}/${dd} ${hh}:${mi}`
}

/** 账号时间显示（2026-10-31 实质优化第 34 轮）：完整日期 + 时分。 */
function fmtAccountTime(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const y = d.getFullYear()
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${y}-${mm}-${dd} ${hh}:${mi}`
}

async function loadChallengeHistory() {
  try {
    const res = await api.myChallengeHistory()
    challengeHistory.value = res.items ?? []
  } catch {
    // 挑战历史是辅助信息，加载失败不打扰主流程
  }
}

// 被挑战记录（2026-10-31 新一轮第 3 轮）。
async function loadDefenseChallengeHistory() {
  try {
    const res = await api.myDefenseChallenges()
    defenseChallengeHistory.value = res.items ?? []
  } catch {
    // 被挑战历史是辅助信息，加载失败不打扰主流程
  }
}

// 离线收益（第六批 R1）：入口与实现已迁到「签到」页，此处不再重复。

// 累计星级（2026-11-01 新一轮第 64 轮）。
const totalStars = ref(0)
/** 星级上限 = 关卡总数 × 3。 */
const totalStarCap = computed(() => (store.config?.levels?.length ?? 0) * 3)

/** G79 累计星级进度环：已获星/总星占比切成 SVG 弧（文本仍保留，环是可视化升级）。 */
const starRing = computed(() => {
  const pct = totalStarCap.value > 0 ? Math.min(1, totalStars.value / totalStarCap.value) : 0
  const C = 2 * Math.PI * 40
  return { dash: pct * C, rest: C - pct * C, pct: Math.round(pct * 100) }
})

async function loadStars() {
  try {
    const r = await api.fetchMyStars()
    totalStars.value = Object.values(r.stars).reduce((sum, n) => sum + n, 0)
  } catch {
    // 星级是辅助信息，加载失败不打扰主流程
  }
}

// 最近战报（2026-10-30 实质优化第 19 轮）。
const battleHistory = ref<api.BattleHistoryItem[]>([])

async function loadBattleHistory() {
  try {
    const res = await api.myBattleHistory()
    battleHistory.value = res.items ?? []
  } catch {
    // 战报历史是辅助信息，加载失败不打扰主流程
  }
}

/**
 * 从战报进入验真（2026-10-31 实质优化第 35 轮）。
 * verify 页读取 query 参数自动填充 battle_id。
 */
function goVerify(b: api.BattleHistoryItem) {
  uni.navigateTo({
    url: `/pages/verify/verify?battle_id=${b.id}&hash=${encodeURIComponent(b.replay_hash ?? '')}`,
  })
}

// 战报详情（2026-10-31 新一轮第 10 轮）。
// 列表只有摘要，点「详情」展开单局的掉落/元素/反应/地形/命中。
const battleDetail = ref<api.BattleDetail | null>(null)

/** 把 `{fire:12, ice:3}` 转成 `fire 12、ice 3`。 */
function mapText(m: Record<string, number> | undefined): string {
  if (!m) return ''
  return Object.entries(m)
    .map(([k, v]) => `${k} ${v}`)
    .join('、')
}

async function toggleBattleDetail(b: api.BattleHistoryItem) {
  if (battleDetail.value?.id === b.id) {
    battleDetail.value = null
    return
  }
  try {
    battleDetail.value = await api.myBattleDetail(b.id)
  } catch {
    battleDetail.value = null
  }
}

// 购买记录（2026-10-30 实质优化第 20 轮）。
const purchaseHistory = ref<api.PurchaseItem[]>([])

/** G84 消费统计：按货币聚合累计花费与次数（数据来自 purchaseHistory[].price）。 */
const spendSummary = computed(() => {
  const m = new Map<string, number>()
  for (const p of purchaseHistory.value) {
    for (const [k, v] of Object.entries(p.price ?? {})) {
      m.set(k, (m.get(k) ?? 0) + v)
    }
  }
  return [...m.entries()].map(([k, total]) => ({ key: k, label: currencyName(k), total }))
})

function priceText(price: Record<string, number>): string {
  const entries = Object.entries(price)
  if (!entries.length) return ''
  return entries.map(([k, v]) => `${v} ${currencyName(k)}`).join(' + ')
}

/** 挑战偷取明细（2026-10-31 实质优化第 36 轮）：`100 金币`。 */
function lootText(loot: Record<string, number> | undefined): string {
  if (!loot) return ''
  return priceText(loot)
}

async function loadPurchases() {
  try {
    const res = await api.myPurchases()
    purchaseHistory.value = res.items ?? []
  } catch {
    // 购买记录是辅助信息，加载失败不打扰主流程
  }
}

// 兑换记录（2026-10-30 实质优化第 21 轮）。
const redemptionHistory = ref<api.RedemptionItem[]>([])

async function loadRedemptions() {
  try {
    const res = await api.myRedemptions()
    redemptionHistory.value = res.items ?? []
  } catch {
    // 兑换记录是辅助信息，加载失败不打扰主流程
  }
}

// 订单记录（2026-10-31 实质优化第 27 轮）。
const orderHistory = ref<api.OrderItem[]>([])

function orderStatusText(s: string): string {
  const map: Record<string, string> = {
    pending: '待支付',
    paid: '已支付',
    delivered: '已发货',
    refunded: '已退款',
  }
  return map[s] ?? s
}

async function loadOrders() {
  try {
    const res = await api.myOrders()
    orderHistory.value = res.items ?? []
  } catch {
    // 订单记录是辅助信息，加载失败不打扰主流程
  }
}

const DEFENSE_WORK_CODES = ['slow_belt', 'block_wall', 'tesla_grid']

/**
 * 修改密码（2026-10-30 实质优化第 7 轮）。
 *
 * 服务端 `POST /me/password` 会验证原口令，并在同一事务里更新哈希、
 * 吊销该账号**全部** refresh token —— 改密成功后其它设备上的旧会话
 * 将无法续期，需要重新登录。因此成功提示里要如实说明这一点，
 * 否则用户在平板上发现「被登出」会当成 bug。
 */
const changingPassword = ref(false)
const oldPassword = ref('')
const newPassword = ref('')
const passwordSubmitting = ref(false)
const passwordMsg = ref('')
const passwordErr = ref(false)

function cancelPassword() {
  changingPassword.value = false
  oldPassword.value = ''
  newPassword.value = ''
  passwordMsg.value = ''
  passwordErr.value = false
}

async function submitPassword() {
  if (passwordSubmitting.value) return
  passwordMsg.value = ''
  passwordErr.value = false
  if (!oldPassword.value || !newPassword.value) {
    passwordMsg.value = '原口令与新口令都不能为空'
    passwordErr.value = true
    return
  }
  if (newPassword.value.length < 10) {
    passwordMsg.value = '新口令至少 10 位'
    passwordErr.value = true
    return
  }
  passwordSubmitting.value = true
  try {
    const res = await api.changePassword(oldPassword.value, newPassword.value)
    passwordMsg.value = `修改成功，已吊销 ${res.revoked} 个旧会话（其他设备需重新登录）`
    oldPassword.value = ''
    newPassword.value = ''
  } catch (e) {
    passwordErr.value = true
    passwordMsg.value = (e as Error)?.message || '修改失败，请稍后再试'
  } finally {
    passwordSubmitting.value = false
  }
}

/**
 * 游客账号转正（2026-10-30 实质优化第 8 轮）。
 *
 * 服务端 `POST /me/upgrade` 会写入用户名 + 密码哈希，并把 is_guest 翻转为
 * false（guest_token 保留，老客户端冷启动仍能找回同一账号）。
 * 成功后刷新 profile，让页面从「游客」切换为「账号」视图。
 */
const upgrading = ref(false)
const upgradeUsername = ref('')
const upgradePassword = ref('')
const upgradeSubmitting = ref(false)
const upgradeMsg = ref('')
const upgradeErr = ref(false)

function cancelUpgrade() {
  upgrading.value = false
  upgradeUsername.value = ''
  upgradePassword.value = ''
  upgradeMsg.value = ''
  upgradeErr.value = false
}

async function submitUpgrade() {
  if (upgradeSubmitting.value) return
  upgradeMsg.value = ''
  upgradeErr.value = false
  const username = upgradeUsername.value.trim()
  if (!username || !upgradePassword.value) {
    upgradeMsg.value = '用户名与密码都不能为空'
    upgradeErr.value = true
    return
  }
  if (username.includes(' ')) {
    upgradeMsg.value = '用户名不能包含空格'
    upgradeErr.value = true
    return
  }
  if (upgradePassword.value.length < 10) {
    upgradeMsg.value = '密码至少 10 位'
    upgradeErr.value = true
    return
  }
  upgradeSubmitting.value = true
  try {
    await api.upgradeAccount(username, upgradePassword.value)
    upgradeMsg.value = '升级成功，现在可以在任何设备登录这个账号了'
    upgradePassword.value = ''
    // 刷新 /me，让 is_guest 翻转为 false，页面自动切换为「修改密码」视图
    await store.refreshProfile()
  } catch (e) {
    upgradeErr.value = true
    upgradeMsg.value = (e as Error)?.message || '升级失败，请稍后再试'
  } finally {
    upgradeSubmitting.value = false
  }
}

/**
 * 修改昵称（2026-10-30 实质优化第 9 轮）。
 *
 * 服务端 `PUT /me/nickname` 更新 `users.nickname`，上限 64 字。
 * 成功后刷新 profile，让顶部昵称立即生效。
 */
const editingNickname = ref(false)
const nicknameDraft = ref('')
const nicknameMsg = ref('')
const nicknameErr = ref(false)

function startEditNickname() {
  nicknameDraft.value = store.nickname || ''
  nicknameMsg.value = ''
  nicknameErr.value = false
  editingNickname.value = true
}

function cancelEditNickname() {
  editingNickname.value = false
  nicknameMsg.value = ''
  nicknameErr.value = false
}

async function submitNickname() {
  const name = nicknameDraft.value.trim()
  nicknameMsg.value = ''
  nicknameErr.value = false
  if (!name) {
    nicknameMsg.value = '昵称不能为空'
    nicknameErr.value = true
    return
  }
  if (name.length > 64) {
    nicknameMsg.value = '昵称最长 64 个字符'
    nicknameErr.value = true
    return
  }
  try {
    await api.updateNickname(name)
    await store.refreshProfile()
    editingNickname.value = false
  } catch (e) {
    nicknameErr.value = true
    nicknameMsg.value = (e as Error)?.message || '修改失败，请稍后再试'
  }
}

/**
 * 退出登录。
 *
 * ⚠️ 为什么**必须二次确认**：服务端吊销的是「该账号的全部会话」，
 * 不只是这台设备。所以「我在手机上退出」会把平板上的玩家一起踢下线。
 * 这是登出的正确语义（一个不信任了就是全都不信任），
 * 但用户必须**事先知道**，否则会被当成 bug。
 *
 * ⚠️ 为什么「服务端没通知成功」也要如实说出来：
 * 网络不通时本地令牌已清、服务端那份还活着。
 * 谎称「已完全登出」会让用户以为风险已消除 ——
 * 而安全相关的提示一旦撒谎就再也不会被相信。
 */
async function confirmLogout() {
  const confirmed = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '退出登录',
      content: '将吊销本账号的全部登录会话（其他设备也会被登出）。确定继续？',
      confirmText: '退出',
      cancelText: '取消',
      success: (r) => resolve(!!r.confirm),
      fail: () => resolve(false),
    })
  })
  if (!confirmed) return

  const { serverNotified } = await api.logout()
  if (serverNotified) {
    uni.showToast({ title: '已退出登录', icon: 'success' })
  } else {
    uni.showModal({
      title: '已在本机退出',
      content: '未能通知服务端，网络恢复后建议重新登录一次以确保旧会话失效。',
      showCancel: false,
    })
  }
  // 回到首页，让所有页面重新走一次登录与配置加载。
  //
  // ⚠️ 要求账号登录的环境里，回首页没有意义 ——
  // 首页会立刻发现「没有登录态」再把用户弹回登录页，绕一圈。
  // 直接落到登录页，少一次无意义的跳转。
  if (store.loginRequired) {
    uni.reLaunch({ url: '/pages/login/login' })
  } else {
    uni.reLaunch({ url: '/pages/index/index' })
  }
}


const stolenText = computed(() => {
  const s = lastChallenge.value?.stolen
  if (!s) return ''
  return Object.entries(s)
    .map(([k, v]) => `${currencyName(k)} ${v}`)
    .join(' · ')
})

/** 快照不完整时不给出挑战入口 —— 盲报会污染对方的战绩数据 */
/** G124 战力差：对手战力 - 我的战力（负=比我弱，更易攻破）。 */
function powerDiff(c: DefenseView): number {
  return Number(c.power ?? 0) - Number(store.power ?? 0)
}

function snapshotOk(c: DefenseView): boolean {
  // 第 140 轮：护盾过期判断用服务端时钟基准，本地时钟偏几小时不再误判
  return validateSnapshot(c, store.estimateServerNowMs()).ok
}

async function loadDefenses() {
  try {
    const res = await api.fetchDefenses()
    myDefense.value = res.mine ?? null
    candidates.value = res.candidates ?? []
    attackLeft.value = Math.max(0, (res.attempt_limit ?? 3) - (res.mine?.my_attempts_today ?? 0))
    // 第 134 轮：护盾「是否生效」按时间判断，不是按字段存在判断。
    // 旧写法 `!!shielded_until` 会把「已过期但字段仍在」的护盾当成开启。
    // 第 140 轮：「现在」用服务端时钟估算而不是裸 Date.now()。
    shielded.value = isShieldActive(myDefense.value?.shielded_until, store.estimateServerNowMs())
  } catch {
    myDefense.value = null
    candidates.value = []
  }
}

/**
 * 防线快照用的**真实构筑**（2026-10-21 P4-7）。
 *
 * 此前这里硬编码 `equipment: []` / `mastery_nodes: []` ——
 * 别人挑战你的防线时，本地模拟跑的是**一个不是你构筑的构筑**：
 * 你的护甲、你的专精加成，全都不在快照里。
 *
 * 也就是说：防线保护的东西，和被挑战时模拟的东西，**不是同一份**。
 */
const defenseBuild = computed(() => ({
  skills: store.equippedSkillIds,
  equipment: store.equippedEquipmentIds,
  mastery_nodes: store.equippedMasteryNodes,
}))

/**
 * 存防线前确保构筑已加载（2026-10-21 P4-7）。
 *
 * 否则快照里的 `equipment` / `mastery_nodes` 是**空数组** ——
 * 别人挑战时模拟的不是你的构筑。
 *
 * ⚠️ 2026-10-21（二轮审计 P2-9）：加载失败**必须中止保存**。
 * 此前 `loadEquipment` / `loadMasteryNodes` 内部只 `console.warn`，
 * 于是一次网络抖动 → 构筑为空 → **原样提交一份空快照**，
 * 正是 P4-7 要消灭的那个缺陷又回来了。
 */
async function ensureBuildLoaded(): Promise<boolean> {
  try {
    await Promise.all([store.loadEquipment(), store.loadMasteryNodes()])
  } catch (e) {
    uni.showToast({ title: `构筑加载失败，未保存防线：${(e as Error).message}`, icon: 'none' })
    return false
  }
  // 兜底：两处都空（既没数据也没报错）也算加载不到 ——
  // 存一份空快照比不存更糟。
  if (store.equipment.length === 0 && store.masteryNodes.length === 0) {
    uni.showToast({ title: '构筑数据未加载，未保存防线', icon: 'none' })
    return false
  }
  return true
}

async function saveDefense() {
  if (!(await ensureBuildLoaded())) return
  try {
    const res = await api.saveDefense({
      name: '我的防线',
      ...defenseBuild.value,
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
  if (!(await ensureBuildLoaded())) return
  try {
    const res = await api.saveDefense({
      name: myDefense.value?.name ?? '我的防线',
      ...defenseBuild.value,
      works: DEFENSE_WORK_CODES,
      shield_hours: shielded.value ? 0 : 24,
    })
    myDefense.value = res.defense
    shielded.value = !shielded.value
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

/**
 * 删除防线（2026-10-31 实质优化第 32 轮）。
 *
 * 删除后其他玩家无法再挑战你，相关挑战记录也会一并删除。
 * 必须二次确认（不可恢复）。
 */
async function onDeleteMyDefense() {
  const confirmed = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '删除防线',
      content: '删除后其他玩家将无法挑战你，该防线的挑战记录也会一并删除。确定继续？',
      confirmText: '删除',
      cancelText: '取消',
      success: (r) => resolve(!!r.confirm),
      fail: () => resolve(false),
    })
  })
  if (!confirmed) return
  try {
    await api.deleteMyDefense()
    myDefense.value = null
    shielded.value = false
    uni.showToast({ title: '防线已删除', icon: 'none' })
    await loadDefenses()
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

/**
 * 挑战用的标准战场：第 1 关（I-5 规定双方在同一张图上比构筑）。
 *
 * ⚠️ 第 150 轮：`store.levelMap` 现在是**摘要**（没有 `waves`），
 * 而引擎开局必须读 `level.waves`。所以这里走 `GET /api/v1/levels/:id`
 * 按需取完整关卡 —— 那正是 `/config` 把波次剔除后省下的 56% 体积。
 *
 * 为什么不是「进页面就预取」：绝大多数玩家根本不挑战防线，
 * 为他们每次进「我的」页都多发一个请求是白花的流量。
 * 第一次点挑战时取一次，之后在本次会话内复用（关卡是静态生成器产物，
 * 配置版本没变它就不会变）。
 */
const challengeLevelCache = ref<GeneratedLevel | null>(null)

async function challengeLevel(): Promise<GeneratedLevel | null> {
  if (challengeLevelCache.value) return challengeLevelCache.value
  try {
    const lv = await api.fetchLevel(1)
    challengeLevelCache.value = lv
    return lv
  } catch (e) {
    uni.showToast({ title: `关卡数据加载失败：${(e as Error).message}`, icon: 'none' })
    return null
  }
}

function myEquipped(): EquippedSkill[] {
  const skills = store.skillMap
  // 第 147 轮：底伤必须按玩家**实际技能等级**烘进去（与 P1 战斗
  // equippedFromSnapshot 同口径）。修前直接用内容表的 base_damage（恒 1 级），
  // 玩家技能升到 10 级、防线挑战仍按 1 级模拟 —— 攻方伤害被系统性低估，
  // 「挑战窃取」判定与玩家真实构筑脱节（I-6 同族的「口径漂移」）。
  const rules = store.config?.skill_rules
    ? skillRulesFromServer(store.config.skill_rules)
    : DEFAULT_SKILL_RULES
  const levelOf = (id: number): number => {
    const row = (store.build as any)?.skills?.[String(id)]
    return typeof row?.level === 'number' ? row.level : 1
  }
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
      baseDamage: skillBaseDamageAtLevel(rules, BigInt(s!.base_damage), levelOf(s!.id)),
      applyElement: (s!.apply_element || s!.element) as Element | '',
      applyStacks: BigInt(s!.apply_stacks),
      projectileSpeed: s!.projectile_speed,
      chain: s!.chain,
      slot: i,
      cooldownRemaining: 0,
    }))
}

async function challenge(target: DefenseView) {
  if (!target.can_challenge) {
    uni.showToast({ title: target.challenge_blocked || '暂时无法挑战', icon: 'none' })
    return
  }
  const equipped = myEquipped()
  if (equipped.length === 0) {
    uni.showToast({ title: '请先到「背包」页装备出战技能', icon: 'none' })
    return
  }
  // 战场要按需取（第 150 轮）：/config 的关卡是摘要，没有 waves。
  // 放在技能检查**之后** —— 没有技能时本来就打不了，
  // 没必要先为一个用不上的战场发请求。
  const level = await challengeLevel()
  if (!level) return // challengeLevel 已经 toast 过原因

  simulating.value = target.id
  // 让按钮先进入「模拟中」状态再跑同步模拟
  schedulePending(async () => {
    try {
      const deps = {
        myEquipped: equipped,
        myAttacker: store.attacker,
        level,
        enemies: store.enemyMap,
        skills: store.skillMap,
        // 第 133 轮：防线挑战得分与结算同口径 —— 用服务端下发的 score_rules，
        // 修前引擎恒用编译期 DEFAULT_SCORE_RULES（服务端重标定即漂移）。
        scoreRules: store.config?.score_rules
          ? scoreRulesFromServer(store.config.score_rules)
          : undefined,
      }
      // 第 140 轮：now 基准用服务端时钟估算（同时驱动护盾判断与默认种子分钟桶）
      const outcome = runChallenge(target, deps, store.estimateServerNowMs())
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
        // D4（2026-10-20）：构筑 S 段随上报提交，服务端按挑战者的
        // user_skill_slots 重算比对 —— 抓「构筑造假」。空串放行（老客户端）。
        replay_skills: outcome.report.replay_skills,
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
          replay_skills: '',
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
    // 2026-10-31（实质优化第 29 轮）：这些请求互不依赖，此前串行 await
    // 让 me 页首屏要等 N 个 RTT；改为并行后只等最慢的那一个。
    // `refreshProfile`/`loadLoadout` 会写 store，其余只写本页 ref，
    // 并行不会互相覆盖。
    await Promise.all([
      store.refreshProfile(),
      // 从「背包」页改过槽位再回来时，这里重新拉一次权威槽位。
      store.loadLoadout(),
      loadDefenses(),
      loadChallengeHistory(),
      // 被挑战记录（2026-10-31 新一轮第 3 轮）。
      loadDefenseChallengeHistory(),
      // 离线收益已迁至签到页（第六批 R1）。
      loadBattleHistory(),
      loadPurchases(),
      loadRedemptions(),
      loadOrders(),
      // 累计星级（2026-11-01 新一轮第 64 轮）。
      loadStars(),
    ])
  }
})

/**
 * 页面卸载时取消待执行的延迟回调（P6-7）。
 *
 * 否则「离开这个页面之后它还在提交」——
 * 对验真 / 挑战防线这种**会改数据**的端点，那是实打实的副作用。
 */
onUnload(() => {
  pageUnloaded = true
  if (pendingTimer) {
    clearTimeout(pendingTimer)
    pendingTimer = null
  }
})
</script>

<style scoped>
/* G124 战力差标签 */
.power-diff { margin-left: 8rpx; font-weight: 600; }
.power-diff.easier { color: var(--ok); }
.power-diff.harder { color: var(--bad); }

/* G114 损失排行 */
.loot-track { height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; margin: 0 12rpx; }
.loot-fill { height: 100%; background: var(--bad); border-radius: 999rpx; }

/* G110 统一时间轴 */
.tl-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; padding: 8rpx 0; border-bottom: 1rpx solid var(--border-soft); }
.tl-kind { width: 100rpx; flex-shrink: 0; font-size: 22rpx; color: var(--accent); }
.tl-text { font-size: 24rpx; }
.tl-row .dim { font-size: 20rpx; }

/* G89 挑战胜负统计 */
.challenge-stats { margin-bottom: 12rpx; }
.challenge-seq { display: flex; flex-direction: row; gap: 6rpx; margin-top: 8rpx; }
.cs-dot { width: 24rpx; height: 24rpx; border-radius: 6rpx; }
.cs-dot.win { background: var(--ok); }
.cs-dot.lose { background: var(--bad); }

/* G84 消费统计摘要 */
.spend-summary { display: flex; flex-wrap: wrap; gap: 8rpx; align-items: center; margin-bottom: 12rpx; }
.spend-chip {
  background: var(--fire-soft);
  color: var(--fire);
  border-radius: 8rpx;
  padding: 4rpx 12rpx;
  font-size: 22rpx;
}

/* G79 累计星级进度环 */
.star-progress-line { display: flex; flex-direction: row; align-items: center; gap: 10rpx; margin-top: 4rpx; }
.star-ring { width: 40rpx; height: 40rpx; }
.sr-bg { fill: none; stroke: var(--panel-3); stroke-width: 10; }
.sr-fill { fill: none; stroke: var(--warn); stroke-width: 10; stroke-linecap: round; }

.me-tabs {
  margin-bottom: 4rpx;
}

.back-btn {
  flex-shrink: 0;
  min-height: 56rpx;
  padding: 8rpx 14rpx 8rpx 0;
  box-sizing: border-box;
  display: inline-flex;
  align-items: center;
  color: var(--accent);
  font-size: var(--fs-sm);
}

/* R94 返回按钮按下左移，给明确的「退回」方向感 */
.back-btn:active {
  transform: translateX(-6rpx);
  opacity: 0.7;
}

/* 独立返回行（100 轮打磨 R16）：与头像行分离，按钮不再被昵称挤断 */
.me-back-row {
  display: flex;
  align-items: center;
  min-height: 64rpx;
}

.avatar {
  width: 96rpx;
  height: 96rpx;
  border-radius: 20rpx;
  background: var(--accent-grad);
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

/* 修改密码表单的输入框（与 login.vue 同款外观）。 */


.pwd-form {
  margin-bottom: 8rpx;
}

/* 挑战历史一行。 */
.ch-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 12rpx;
  padding: 14rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.ch-row:last-child {
  border-bottom: none;
}


/* 昵称编辑：链接样式的「改名」入口。 */

.nick-edit-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12rpx;
  margin-bottom: 4rpx;
}

.nick-inp {
  flex: 1;
  min-width: 0;
  height: 64rpx;
  margin-bottom: 0;
  font-size: 26rpx;
}

/* 小按钮：昵称编辑行的保存/取消。 */
.btn-mini {
  height: 64rpx;
  padding: 0 24rpx;
  font-size: 24rpx;
  border-radius: 10rpx;
  white-space: nowrap;
}

.res-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12rpx;
}

.res-cell {
  background: var(--panel-2);
  border-radius: 14rpx;
  padding: 16rpx 20rpx;
}

.res-v {
  font-size: 34rpx;
  font-weight: 700;
  display: block;
  font-variant-numeric: tabular-nums;
}

.res-k {
  font-size: 24rpx;
  color: var(--muted);
  display: block;
  margin-top: 4rpx;
}

.res-note {
  font-size: 22rpx;
  color: var(--dim);
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

/* R80 候选防线计数 */
.cand-count {
  margin-left: 12rpx;
  font-size: 20rpx;
  color: var(--dim);
  font-weight: 400;
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
