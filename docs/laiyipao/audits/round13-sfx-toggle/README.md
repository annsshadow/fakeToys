# R13 音效开关语义反转修复：战斗音效默认开启

> 环境：本地回环 H5 + 本地回环 API，真实 Chrome 浏览器点击；**非物理真机**。
> 域名一律用 `<YOUR_TEST_DOMAIN>` 占位，报告与截图不写真域名、真 IP、口令或令牌。

## 1. 独立复核根因（代码级证据）

队长给出的定位经独立复核成立：

| 位置 | 原文 | 语义 |
| --- | --- | --- |
| `laiyipao/miniapp/src/pages/me/me.vue:161` | `<switch :checked="sfxMuted" ...>` | 开关绑定的是**静音状态** |
| `me.vue:661` | `const sfxMuted = ref(isMuted())` | 初始值直接取静音态 |
| `me.vue:663-667` | `const next = e.detail?.value === true; setMuted(next); sfxMuted.value = next` | 用户打开开关 → 反而写静音 true |
| `laiyipao/miniapp/src/audio/sfx.ts:44` | `let muted = false` | 音频模块默认有声 |
| `sfx.ts:145-148` | `isMuted()` 返回 `muted` | 新玩家为 false |
| `sfx.ts:156-163` | `setMuted(v)` 写 `lyp_sfx_muted = v ? '1' : '0'` | 持久化的是静音偏好 |

因此新玩家 `isMuted()=false → sfxMuted=false →`（标签为「战斗音效」的）开关显示关闭，正如用户反馈『默认战斗音效是关闭的，需要再打开』；而用户一旦打开开关 `next=true → setMuted(true)`，反而静音。这是开关语义反转，不是音频模块默认值错误。

## 2. test-first RED→GREEN

### RED
- 新增动态行为测试 `src/pages/me/me.test.ts`：「R13 战斗音效开关 checked=true 表示音效开启：默认开启，关闭开关才 setMuted(true)」。mock `@/audio/sfx`，断言 `switch.checked='true'`、`sfxEnabled=true`、`setMuted` 未被默认调用；关闭开关触发 `setMuted(true)` 且 `sfxEnabled=false`；再打开触发 `setMuted(false)`。
- 更新源码守卫 `src/test/me_sfx_toggle_feature.test.ts`：断言 `:checked="sfxEnabled"`、`const sfxEnabled = ref(!isMuted())`、`setMuted(!next)`、`sfxEnabled.value = next`，并禁止再出现 `:checked="sfxMuted"`。

命令：`pnpm vitest run --config vitest.config.ts src/pages/me/me.test.ts src/test/me_sfx_toggle_feature.test.ts --reporter=verbose`
结果：exit 1，**2 failed / 29 passed**。逐字错误：
- `expected '<template>...' to contain ':checked="sfxEnabled"'`
- `expected 'false' to be 'true' // Object.is equality`

### GREEN
只改 `me.vue` 两处，未动 `sfx.ts`：
1. 模板：`:checked="sfxMuted"` → `:checked="sfxEnabled"`；
2. 脚本：`const sfxMuted = ref(isMuted())` → `const sfxEnabled = ref(!isMuted())`；`onSfxToggle` 改为 `const next = e.detail?.value === true; setMuted(!next); sfxEnabled.value = next`，并加注释说明开关表达「音效开启」、模块保存「静音」必须取反。

文案「关闭后战斗中的射击/反应/胜利等音效将静音，偏好会保存在本机。」与修复后的开关语义一致，保留。

定向回归：`src/pages/me/me.test.ts` + `src/test/me_sfx_toggle_feature.test.ts` + `src/audio/sfx.test.ts` + `src/audio/battle-sfx.test.ts` → 4 files passed / 52 tests passed，exit 0。`unlockSfx` 手势解锁、`playSfx`、`setMuted` 持久化与两条既有音频测试均未被破坏。

## 3. 真浏览器实测（非物理真机 / 本地回环）

浏览器会话 `lyp-sfx`，本地回环 H5/API。

1. **新玩家默认开启**：清空 `lyp_sfx_muted` 后进「我的」页。DOM 为 `<uni-switch>...uni-switch-input-checked`，`isMuted()=false`，storage key=null。截图 `screenshots/sfx-01-default-on.png`（sha256 4d27cc2a9824b5c92302ddd77821fe3ccdfea2c6c6536024a3f8dc81807cf5db）。
2. **真实关闭**：滚动后真实鼠标点击开关中心 (854,294)。`isMuted()=true`，storage key='1'，开关视觉关闭。截图 `screenshots/sfx-02-muted.png`（sha256 5ebec7b557a616c751b1a30104c3f9a6376a68f441775c52905e7ab9ae051dd9）。
3. **刷新持久化**：重新打开页面后 `isMuted()=true`，key='1'，开关仍关闭。
4. **真实再开启**：真实点击同一坐标。`isMuted()=false`，key='0'，开关视觉开启。截图 `screenshots/sfx-03-reenabled.png`。
5. **进战斗解锁链路**：打开第 1 关战斗页并在页面真实点击后，`isSfxUnlocked()=true`、`isMuted()=false`、key='0'；调用 `playSfx('tap')` 无异常，`errors=[]`。截图 `screenshots/sfx-04-battle-unlocked.png`。

## 4. 全量验证

| 命令 | 结果 |
| --- | --- |
| `pnpm test` | exit 0，Test Files 184 passed | 1 skipped (185)，Tests 1745 passed | 2 skipped (1747) |
| `pnpm run type-check`（vue-tsc） | exit 0 |
| `pnpm run type-check:test`（tsc） | exit 0 |
| `pnpm run build:h5`（uni build） | exit 0，`DONE Build complete.` |
| `agent-browser errors --json` | `errors=[]` |

未执行 git；未写入真值。

## 5. 结论

用户反馈的「默认关闭、需要再打开」根因是 UI 开关直接绑定静音态导致语义反转，而非音频模块默认静音。修复后：音效默认开启；开关 checked=true 明确表示音效开启；关闭开关写入静音偏好并持久化；再打开恢复有声；战斗手势解锁链路保持正常。
