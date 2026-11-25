# R550 H5 日历页伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 A / 4.1.2 A）
- 轮次：R550（implementation，t65），真父 4c72d0cfe746c4c18f34d3fe1dc83e100b384784（台账§540入账后），attempt 8206f5f0-f7ed-4716-b6ff-a21190be0b6f。
- 缺陷来源：R549 甄别判 1 项 P2（design-lead commit 60812651c / t64 终审 pass），calendar.vue 9 类 <view>/<text> @tap 伪按钮键盘不可达且 AX 语义不可读。
- 环境：自起 uni H5 + Playwright chromium headless 390×844 DPR2（isMobile/hasTouch），page.route 独立 stub 造数五态（常驻/withData/error/errorBanner/empty）；H5 App.vue 无主题机制，light/dark 截图 sha 相同据实记录。<TEST_HOST>:<PORT> 占位，0 真值。

## 一、方案取舍（Fail Loud）
采用方案 B 逐页显式补 a11y 属性。方案 A 全局增强器不采用：先例 R540/R544/R546/R548 已因越界他页、动态名派生不准 Fail Loud 排除；本轮 9 类控件的 aria-label 需按页签语义/事件标题/操作上下文精确生成（如「今日」「本周」「查看 R550event0 详情」「删除 R550event0」），全局脚本无法准确派生，且会越界改动他页，故不在 main.ts 增加任何全局逻辑。main.ts 的 R537 增强器 installH5NativeInputNameA11y 本轮零改动。

## 二、写域与改动（精确 3 文件）
- M oa4rust/mobile/src/pages/calendar/calendar.vue：仅 9 处开标签追加属性（role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler），原 @tap 全部保留。
  ① 范围页签 v-for（:287）：:aria-label=r.label，复用 switchRange(r.key)。
  ② 新建事件（:296）：:aria-label=composing ? 取消新建事件 : 新建事件，复用 toggleCompose。
  ③ 创建日历（:302）：aria-label=创建日历，复用 submitCalendar。
  ④ 保存事件（:318）：aria-label=保存事件，复用 submitEvent。
  ⑤ 错误态重试（:327）与 ⑥ 错误 banner 重试（:332）：aria-label=重新加载日程，复用 load()。
  ⑦ 事件卡片 v-for（:335）：:aria-label=查看 + 事件标题 + 详情，复用 openEvent(e)。
  ⑧ 详情关闭（:351）：aria-label=关闭日程详情，复用 detail = null。
  ⑨ 删除事件（:356）：:aria-label=删除 + 事件标题，复用 removeEvent(detail)。
- A oa4rust/mobile/src/pages/R550.calendarKeyboardA11y.contract.test.ts：14 用例（8 核心 + 6 守卫）。
- A 本文档。
- 业务零改守卫：业务 handler 函数体（switchRange/toggleCompose/submitCalendar/submitEvent/load/openEvent/detail=null/removeEvent）/calendarApi 接口/状态字段（showCalendarForm/newCalendarName/form）/样式 css/他页/main.ts/R537 增强器/同页 :295 正面对照全部逐字零改。git diff 仅属性追加，无函数体/API/样式变更。

## 三、RED→GREEN（干净）
- 契约定点测试：真父 4c72d0cfe 覆盖 calendar.vue → RED EXIT=1，8 核心 failed / 6 守卫 passed；恢复修复版 → GREEN EXIT=0，14 passed。
- 守卫在真父亦 pass，证明守卫钉的是「业务零改」基线而非修复本身。

## 四、三 verify（全 exit 0，workdir oa4rust/mobile）
- V1 定点：pnpm test src/pages/R550.calendarKeyboardA11y.contract.test.ts → 14 passed。
- V2 全量：pnpm test → 86 files / 597 tests passed（历史契约无回归）。
- V3 类型：pnpm typecheck（tsc --noEmit）→ 无报错。

## 五、真浏览器 CDP 决定性判据
- runner：oa4rust/test-results/r550-e2e.cjs，8 组（light/dark × withData/error/errorBanner/empty），390×844 DPR2，page.route stub 独立造数。
- 判据口径：document.activeElement=UNI-VIEW/UNI-TEXT[role=button] 且 Accessibility.getFullAXTree 中 prop(n,focused)===true 的非 RootWebArea 节点 role=button/focusable=true/name=预期；★严禁探宿主 wrapper 假阳性（R535 教训）。
- 结果：R550_E2E_DONE names=58 namesBad=0 fatals=0 pageErrors=0。9 类候选聚焦节点 AX name 全达标：今日/本周/新建事件/创建日历/保存事件/重新加载日程(×2 态)/查看 R550event0 详情/关闭日程详情/删除 R550event0；document.activeElement 均为目标控件本身（isTarget=true, isBody=false）。
- 等价触发：Enter 切页签（今日→本周，on 索引 0→1）、Enter 打开事件详情、Space 关闭详情，均等价原 handler。
- 同页 :295 正面对照：DOM role=button / aria-label=新建日历，partial AX role=button / name=新建日历 语义可读且零回归；据实说明 :295 自身无 tabindex 不进 Tab（R549 已明确，不属本轮 9 类候选），本轮仅取其 AX 语义对照，故判据用语义比对而非 focused。
- 注入对照：移除首个页签 aria-label 后 focused name 失配（removedName 回到 default），重新注入 R550-injected-tab 后 focused name=R550-injected-tab 恢复，证明 AX name 通道有效。
- Tab 命中：withData 态 32 次 Tab 命中页签 8 次、撰写按钮 4 次、事件卡 15 次；详情态命中 detail-close 2 次、c-remove 2 次。
- 截图 780×1687 目视：withData 显示今日/本周页签、3 条 R550event 事件卡、新建日历/新建按钮；errorBanner 显示重试条，名实相符。H5 无主题机制，light/dark 同 sha 不伪造暗色差异。

## 六、证据 SHA-256（gitignored oa4rust/test-results/r550/，未入库）
- calendar.vue（修复版）=7e355af979b1e1fc894082264b4261d631aeb1615b3388c074d12ed1e2479994
- 真父 calendar.vue（RED 基线）=75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838
- R550 契约=6b1d13a6089da82f7158b8cbc644e052ef38793510ea57cd24e0e3d60a70d69b
- r550-e2e.cjs=dd320c6b628b248f6c7d8a55cd35e1e1ecfb477622f97789bc5e720be863aa10
- r550-ax-report.json=f30c02624930920a6d2173b215c5d61d04d362adc55243849c14edfb897ca1cc
- r550-red.log=9e0b4ae82430ae833050133dfe81bc99762b4671ecaa499b91212354e38fd78c
- r550-green.log=a3d3b0f136d513958d7ef74d0096a455fe0ae9eb0bff3fec79a0d8da7cd3d19c
- r550-verify2.log=aee589d2635d2c0c312fc10a9a067a4cad2f22b9a4f572e466e466f60a8160ec
- r550-verify3.log=8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92
- 截图 withData=b742686fd86fae3c8ec550fd63e7d4cb6af3cc4c279dd354152de7cfb0052879（116931B）
- 截图 error=1fb957541ddf8f87826f9ac89baed609ecf6440f255b926a94d9a168a3285863（119359B）
- 截图 errorBanner=3e32bd043fe98ac79656f5c64193db0c11b800bcf6c6fdd322dddb723ce1b081（119891B）
- 截图 empty=ea23c2dc6d852487d2e241663034e6aa3dc6145787add8c870b1997b4b1fd58e（117346B）
- light/dark 同 sha（H5 无主题机制）。

## 七、Fail Loud 披露
- 端口：<PORT> 等前置端口均被陈旧服务占用（curl 200 但页面空壳 .page=0），新服务实际起于另一个回环端口，E2E 用该新端口取证，已据实披露。
- :295 正面对照无 tabindex 不进 Tab，本轮仅作 AX 语义对照，未夸大为键盘可达修复对象。
