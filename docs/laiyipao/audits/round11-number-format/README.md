# R11 数值浮点格式化真实缺陷修复 · H5 实测记录

## 缺陷与修复

- 缺陷：skills.vue 第 68/86/91/95/237 行等直接插值 r7Impact 的数值，真实接口返回 5.400000000000001、79.62962962962963、0.30000000000000004 这类长浮点尾数直接渲染到界面；equipment/gems/battles/battle 同类。
- 修复：skills/equipment/gems/battles/battle 五页加 fmt(n)/fmtSigned(n)（toFixed(1) + Number.isFinite + 非数显示「—」），所有 r7Impact/r10-gain/r10-growth 数值插值统一走格式化函数；r7StarText 复用 fmt。
- test-first RED：r11NumberFormat.test.ts 契约 + skills_apply_feedback/equipment_apply_feedback 行为测试，注入长浮点 impact，修复前 3 files failed / 6 failed|2 passed；GREEN 62 tests passed。

## 真浏览器实测（非物理真机）

- 环境：agent-browser 0.41.1，桌面 Chrome 模拟移动视口 390×844 与桌面视口 1280×800；页面为本地回环 http://127.0.0.1:5174（H5 dev）+ 8080（API），非物理真机。
- 方法：用 agent-browser network route（含 --allow-origin）注入 rec-v1 接口夹具，baseline.estimated_damage_per_wave=5.400000000000001、damage_per_wave_pct=79.62962962962963、power_delta=0.30000000000000004；真实走 fetchRecommendations/fetchRecommendationImpact 渲染链，截屏前后 DOM 文本用 eval 读取。
- 结果（DOM 文本实测，非编造）：当前每波伤害 5.4、推荐 9.7、每波 +4.3（+79.6%）、战力 +0.3、评分 +4.0、历史三星率 33.3%；长浮点串检测 LONGTAIL=false。
- 对比弹层「确认替换」同样显示 每波伤害 5.4 → 9.7 / 攻击 1000.0‰ → 1120.0‰ / 评分 40.0 → 44.0，数字全部格式化。

## 截图（非物理真机，桌面浏览器模拟视口，本地回环）

| 文件 | 视口 | sha256 |
| --- | --- | --- |
| screenshots/after-skills-mobile.png | 390×844 | 170dbd14abe11c0a83e669f5f1ad43bbafd2b2b24012e0b93feeabc216b22c43 |
| screenshots/after-skills-desktop.png | 1280×800 | 5cc25859e445e0f471644fe8794d416f6551269a5c3547f093493f3f000c1988 |

## 回归验证（全部 exit 0）

- pnpm test：183 files passed | 1 skipped，1724 tests passed | 2 skipped（184 files/1726 tests）。
- pnpm type-check（vue-tsc --noEmit）：exit 0。
- pnpm type-check:test（tsc --noEmit -p tsconfig.vitest.json）：exit 0。
- pnpm build:h5：Build complete。

全程未做 git 操作，未写入任何真实域名/IP/口令/密钥。
