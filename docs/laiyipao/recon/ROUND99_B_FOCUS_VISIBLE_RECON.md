# R99-B 侦察报告：键盘焦点可见性（:focus-visible 焦点环）

- 轮次：R99-B（只读侦察，新视角）
- 角色：same-route-reviewer
- 基线 HEAD：05f2acf30950a377b681c09ae05d988f25eac866（clean，A/B 均 0/0，全程未变）
- 日期：2026-11-28
- 视角来源：既有 R60/R95/R96/R97/R98 让大量元素 tabindex=0 可键盘聚焦，但 Tab 聚焦时焦点环是否被系统性抑制从未验证；R96-B 只测 zoom/高对比度，不含 focus ring。本轮补齐该缺口。

## 0. 结论（先给判定）

**无推荐候选。** 四条件第①项不成立——未发现真实用户可见缺陷。miniapp 已有全局 :focus-visible 兜底（common.css:914/918），真 Chromium 三视口 × 4 页共 120 次真实 Tab 采样中，63 个实际聚焦元素 100% 都有可见焦点环（solid 主题色 outline + offset），fv:true。任务所列的潜在抑制路径逐一排除。另记录 3 条正价值发现（见 §6），均不构成候选。

## 1. 任务范围

① 全仓 grep focus-visible / outline:none / :focus 统计与权威口径；② 列代表性键盘可达元素清单（R95 back 键 / R96 空态 CTA / R98 @confirm 输入 / 筛选 chip 等）；③ 真 Chromium 三视口 390/768/1280 用真 Tab 逐元素坐实 computed style（outline / box-shadow / background 变化）并截图；④ 按四条件裁定是否有真实用户可见缺陷与小改方案。

## 2. 静态审计铁证

### 2.1 全仓 grep 统计（laiyipao/miniapp/src 下）

| 模式 | 命中数 | 位置 |
| --- | --- | --- |
| :focus-visible | 2 | 全部在 common.css:914、common.css:918（全局规则） |
| outline:none | 1 | common.css:339（在 .inp:focus,.search-inp:focus 块内） |
| :focus | 5 | bag.vue:414(:focus=presetFocus) / common.css:337-338 / 914 / 918 |
| tabindex | 遍布 17 页 | announcements2/bag1/battle3/battles7/enemies1/equipment9/gems3/mastery2/me2/rank1/reactions9/shop2/signin2/skills8/skins1/stage2/verify3 + common.css1 |
| vue 内 outline 声明 | 0 | 全仓 pages 下 vue 内零 outline，零 !important |

miniapp/src 样式仅 common.css 与 tokens.css 两个文件，无 scoped 样式层写 outline。

### 2.2 全局焦点环规则（common.css:912-921，逐字）

```css
/* LYP-a11y / R97 键盘焦点可见（WCAG 2.4.7）：与 admin 同判据 */
[role="button"] { cursor: pointer; }
:where(a, button, input, textarea, select, [tabindex]):focus-visible {
  outline: 3rpx solid var(--accent);
  outline-offset: 2rpx;
}
:focus-visible {
  outline: 3rpx solid var(--accent);
  outline-offset: 2rpx;
}
```

:where(...) 零特异性，被后面的裸 :focus-visible 兜底再保一层；覆盖 a/button/input/textarea/select/[tabindex] 全部可聚焦交互元素。

### 2.3 唯一 outline:none 处（common.css:336-342，非缺陷）

```css
/* R29 输入框聚焦反馈：让玩家一眼看到当前在填哪个框 */
.inp:focus,
.search-inp:focus {
  outline: none;
  border-color: var(--fire);
  box-shadow: 0 0 0 2rpx var(--fire-soft);
}
```

该处虽抑制原生 outline，但以 border-color + box-shadow 双重替代反馈（R29 既有设计），不构成焦点不可见。且真浏览器实测 H5 编译后的 input class 为 uni-input-input，不匹配 .inp/.search-inp，实际走全局 :focus-visible outline（见 §4），反馈更直接。

## 3. 权威口径

| 守卫 | 位置 | 判据 |
| --- | --- | --- |
| r60A11yKeyboard.contract.test.ts:65-71 | miniapp | 守 focus-visible 覆盖 [tabindex]（:67）、outline 3rpx solid var(--accent)（:68）、声明只一次（:69-70） |
| r60A11yKeyboard.contract.test.ts:41-63 | miniapp | role=button 必带 tabindex=0 + @keydown，cursor:pointer |
| tokens.contract.test.ts:38 | miniapp | 正则守完整 :where(...[tabindex]):focus-visible 声明 |
| z_audit_feature.test.ts:88/106 | miniapp | 审计锚点含 :focus-visible |
| focusVisible.contract.test.ts:24-48 | admin | 四判据：全局规则兜住 outline:none / 覆盖全部交互元素 / outline 2px solid var(--lyp-*) + offset 2px / !important 覆盖 scoped outline:none |
| designTokens.contract.test.ts:135 | admin | 同规则正则守卫 |

admin 权威样式 main.css 用 2px（px 基准），miniapp 用 3rpx（rpx 基准），双端各自成立——注释（focusVisible.contract.test.ts:6-9）明言 admin 因 Element Plus + WikiView.vue:1184 等 scoped outline:none 需 !important 覆盖，miniapp 无此类 scoped 抑制（§2.1 零命中），故无需 !important，差异正当。docs/laiyipao/design/AI_VISUAL_SYSTEM.md 与 TOKENS_QUICK_REF.md 在 R96-B 已确认双端焦点环口径。

## 4. 真浏览器决定性证据

### 4.1 采集方法

agent-browser 0.41.1，H5 常驻 http://127.0.0.1:5174（焦点环为纯 CSS 视觉，不依赖后端数据，未起 API）。每个视口×页面独立 session：close → open hash 路由 → set viewport → set media light reduced-motion → wait 1200ms → 真 press Tab ×10 → 每次 eval getComputedStyle(document.activeElement) 读 outlineStyle/outlineWidth/outlineColor、outlineOffset、boxShadow、borderColor、el.matches(:focus-visible) → 再 Tab 一次后 screenshot。

三视口 390×844 / 768×1024 / 1280×800 × 4 页（me / signin / verify / announcements）= 12 组，共 120 次 Tab 采样。

### 4.2 汇总（data/parsed.txt 第 1 行）

CAPROWS 120 BODY 57 FOCUS 63 RING_OR_SHADOW 63

120 次采样中 57 次 activeElement 回绕到 body（页面仅 1 个可聚焦元素时的自然循环，非缺陷），63 次落在真实可聚焦元素上，63/63 全部有可见焦点环。

### 4.3 computed style（三视口一致规律）

| 视口 | outline | offset | 颜色 | 覆盖元素类型 |
| --- | --- | --- | --- | --- |
| 390×844 | solid 1px | 1px | rgb(255,107,53) | r3-empty-action / r3-ai-cta / r5-note-cta / INPUT |
| 768×1024 | solid 3px | 2px | rgb(255,107,53) | 同上 |
| 1280×800 | solid 1px | 1px | rgb(255,107,53) | 同上 |

- 颜色 rgb(255,107,53) = #ff6b35 = --accent（tokens.css:35）= --fire（tokens.css:43），主题色，与深色底 --bg:#070a10（tokens.css:19）对比强烈。
- 所有采样 fv:true（元素确实匹配 :focus-visible），box-shadow:none（焦点反馈来自 outline 而非 shadow）。
- 元素类型覆盖任务要求的代表性集合：空态 CTA（r3-empty-action）、AI CTA（r3-ai-cta）、验真引导（r5-note-cta）、表单输入（INPUT.uni-input-input）。R95 back 键同为 [tabindex] 元素、被同一全局规则覆盖（guest 态 canGoBack=false 时该键不渲染，故未在本次采样中命中；静态机制已确认）。筛选 chip（speed-chip/chapter-chip）同为 [tabindex] 类，规则同样覆盖。

### 4.4 截图（亲读 2 张，12 张 SHA 独立落盘）

- focus-768-me.png（sha 6161205f4ea36f88f942859d500e65f172dc5bfd10c5158f7ddb7397582da4c2）：亲读——去专精页 CTA 有清晰橙色焦点环包裹。
- focus-390-signin.png（sha 2b97771556ccd3f0f5e729d0298eaba8911681bc009750f74b63c278165f14e8）：亲读——底部兑换码输入框有橙色焦点描边。

12 张截图 SHA-256（UNIQUE 12/12），落 docs/laiyipao/audits/round99-b-focus-visible/screenshots/：

```
4d1bba9bc1ce7202fcee6c355c6096637aca7c5f5363e0e34c93b914f40af2d6  focus-1280-announcements.png
7d7c1405520436ca32e05891a24106fecdf6c3fe52519fe5ef7a3af5b3b7d85a  focus-1280-me.png
a7d31469b91f41abe6e38a6a417f1df9cb0e65d2a82b3bce6e538fc15235490f  focus-1280-signin.png
2c273f28b18b0040805dec018fb06e41d846f97053f0b3e5adc2d3c9f503946c  focus-1280-verify.png
52830a2ea0b85dfd5fc6776fbafe22acb6e841d8dc015db22f078a0504f4b1bf  focus-390-announcements.png
961a33e7ec7367aab230e14777b8750bc89642306a1564b0f7440aad3d133387  focus-390-me.png
2b97771556ccd3f0f5e729d0298eaba8911681bc009750f74b63c278165f14e8  focus-390-signin.png
cab921aade555c2e31cea39771501a12a1645039b6a536eb7cb898dde1aeed65  focus-390-verify.png
80666ec73379b3346bb1d869f24530c1b0124b30a7f5508b5f014640de151394  focus-768-announcements.png
6161205f4ea36f88f942859d500e65f172dc5bfd10c5158f7ddb7397582da4c2  focus-768-me.png
ce2b1958f94ef53409747f90cc896bd005d6b8c8200dad261026dbdb2e28c1ff  focus-768-signin.png
3c7d59d882a3a9eaf4a0bb0867fafcf956bbca295aa55501a8a0cc6c39b5d92f  focus-768-verify.png
```

## 5. 四条件裁定

| 条件 | 判定 | 说明 |
| --- | --- | --- |
| ① 真实可复现用户可见缺陷 | 不成立 | 63/63 聚焦元素焦点环可见，主题色高对比；无键盘用户看不到焦点位置情形 |
| ② 展示层小改 | 不适用 | 无缺陷，无需改 |
| ③ 权威口径明确 | 满足（指向已达标） | r60/tokens/z_audit/focusVisible 契约均要求焦点环存在，实测全部满足 |
| ④ 改动面可控 | 不适用 | 无需改 |

=> 四条件未全中，不列候选。

## 6. 正价值发现（非候选，供后续参考）

1. rpx 响应式换算导致焦点环粗细随视口变化：3rpx 在 390/1280 视口实测 1px、768 视口实测 3px。这是 uni-app rpx→px 换算的正常行为，非缺陷；但若未来提高焦点环可见性标准（如对齐 WCAG 2.4.11 的 2px 最小建议），需注意小屏 1px 偏细，届时应改全局规则而非逐元素补。
2. outline:none 规则在 H5 编译 input 上未命中：common.css:339 .inp:focus 的抑制规则，在 H5 编译后的 input.uni-input-input 上不匹配，input 实际走全局 :focus-visible outline。静态规则与运行时命中的微妙差异——当前结果更好（反馈更直接），但记之以备未来 class 结构变化时复核。
3. miniapp 无 !important vs admin 有，差异正当：静态确认 miniapp 全仓 vue 零 outline 声明、零 !important，不存在需覆盖的 scoped outline:none，故不需要 admin 式 !important 防护；双端口径各自成立，非漂移。

## 7. 诚实边界

- 焦点环为纯 CSS 视觉验证，未起 API；三视口覆盖 4 页，非全部 19 页（其余页同用 common.css 全局规则，机制一致）。
- 12 张截图亲读 2 张（768-me、390-signin），其余 10 张以 computed style 数据坐实。
- R95 back 键在 guest 态 canGoBack=false 不渲染，未在采样中命中，以同为 [tabindex] 元素、被同一全局规则覆盖的静态机制确认。
- 57/120 次 Tab 回绕到 body 属单元素页面自然循环，非缺陷。
- rpx→px 换算的具体系数依 uni-app H5 编译实现，本文只记录实测值，不推断内部算法。

## 8. Hygiene

- HEAD 全程未变：05f2acf30950a377b681c09ae05d988f25eac866。
- git status --porcelain 非 ?? 项：空（本轮零源码/契约/规范/令牌改动）。
- git diff --check RC=0。
- auth 四路径（.env / .local.keys / remote-config.env / laiyipao/deploy/.local.keys / laiyipao/server/.env）全 ABSENT；laiyipao 下 *.pem/*.key 计数 0。
- 自启 job 全部清理：采集 job（bash-94f7a15d1c404a17 首轮、bash-41d03463bef34dcf 重跑）均 completed；job_list running=[]。
- 所有证据文件落 docs/laiyipao/audits/round99-b-focus-visible/（UNTRACKED，不入库），零真值。
