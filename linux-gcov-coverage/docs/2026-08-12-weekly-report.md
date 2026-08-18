# 内核 gcov 分支覆盖率测量 · 周进展报告（2026-08-05 ~ 2026-08-12）

> 背景：用户要求「跑一次全量覆盖率以量化『分支覆盖率 90%』目标的实际达成度」。
> 本周结论：**测量工具链已打通并能产出真实数字；裸机引导（内核启动 + 最小 initramfs）实测行覆盖率 8.89%、分支覆盖率 5.56%，距离 90% 目标差距巨大，须注入 in-guest 实际负载方能逼近。**

---

## 0. 一句话结论

1. **工具链打通**：x86_64 + QEMU(TCG) 引导 gcov 插桩内核 → initramfs 内 `cp` 导出 `/sys/kernel/debug/gcov` → virtio-9p 落到宿主 → host 侧 harvest 对齐 `.gcno` → `gcov -b` 解析 → 真实分支覆盖率数字。该全链路已端到端跑通。
2. **报告恒为 0 的真正根因已定位并修复**：out-of-tree 构建下 gcov 解析 `.gcno` 内嵌源码路径时相对的是 `.gcno` 自身目录（首级目录被「翻倍」），而源码不在 build 树内，gcov 找不到源文件只写出 4 行 header 的 `.gcov`，解析得 0。修复为建交叉符号链接，`nl80211.c.gcov` 从 4 行 → 35235 行、9705 条分支记录。
3. **90% 目标远未达成**：当前捕获只覆盖「内核启动 + 最小 initramfs init」路径。全量聚合（干净重算已完成）实测 **行覆盖率 8.89%（37754/424469）、分支覆盖率 5.56%（18212/327781）**。单文件采样 `nl80211.c` 仅 9058 行中 0.06% 执行、9705 分支中 0.04% 执行（冷门无线子系统，非引导热路径）。精确聚合值见 `tools/testing/coverage/baseline/baseline_report.json`。

---

## 1. 路线决策

| 路线 | 结论 | 依据 |
|------|------|------|
| UML | ❌ 废弃 | Docker-on-WSL2 下 UML 切用户态执行 init 前 SIGSEGV（UML 运行时 SKAS/TT 与宿主不兼容，非插桩问题，非 KUnit，非源码 bug） |
| x86_64 + QEMU(TCG) | ✅ 采用 | 容器内 qemu 7.2.22 可用、busybox 可做 initramfs、gcc/gcov 12.2.0 一致；镜像 `kernel-coverage-qemu:latest` |

---

## 2. 构建链路修复（使本分支可作为标准 x86_64 内核编译/链接）

本分支（baseline = v7.1.3 initial）改了大量核心文件，留下多处不一致，全量 gcov 构建逐一暴露并修复：

| 文件 | 问题 | 修复 | 性质 |
|------|------|------|------|
| `kernel/auditsc.c` | `struct filename` 被重构（去 `refcnt`），但代码仍 `name->refcnt++`（3 处） | `CONFIG_AUDIT=n` 配置级绕开（可逆，不动源码） | 配置绕开 |
| `mm/slub.c` + `mm/slab.h` | `struct slab` 的 `freelist_aba` 字段重命名未对齐，x86_64 下 `__update_freelist_fast` / `print_slab_info` 编译失败 | 4 处最小化源码补丁（FIXME） | 临时 workaround |
| `drivers/i2c/Makefile` 第 8 行 | `i2c-core-objs := i2c-core- i2c-core-`（尾杠悬空 → `No rule`） | 改为 `i2c-core-base.o i2c-core-smbus.o`（FIXME；`arch/x86/Kconfig` 强制 `select I2C`，不能关） | 临时 workaround |
| `net/netfilter/xt_tcpmss.c` | 磁盘小写 vs Makefile 要 `xt_TCPMSS.o`（Windows 检出 case-collision） | `git mv` 修正为 `xt_TCPMSS.c` | 已修正 |
| 全量配置 | `CONFIG_MODULES=n` 把 netfilter 等 `=m` 降级为 `=n` → 链接 undefined reference | 恢复 `=m→=y` sed 提升 + 保持 `MODULES=y`（defconfig 默认） | 配置修复 |
| 构建健壮性 | 插桩诱发 warning 阻断 `-Werror` 构建 | `CONFIG_WERROR=n` | 配置修复 |

> ⚠️ 上述带 FIXME 的源码补丁是**测量用临时 workaround**，本分支本身无法作为标准 x86_64 内核编译，须回报分支 owner 修复（见 §6）。

---

## 3. 采集 / 解析链路修复（本周关键突破）

### 3.1 已打通的采集侧
- `boot_qemu_collect_gcov`：QEMU `-kernel bzImage` + busybox initramfs + virtio-9p 共享；init 等待 `/sys/kernel/debug/gcov` 出现后 `cp -a` 整树到 9p，写 `done` 哨兵后 `poweroff -f`；宿主轮询 `done` 后终止 QEMU 再 harvest（避开 QEMU 不退出卡死）。
- `_harvest_gcov_share`：剥离 objtree/srctree 前缀，把真实 `.gcda` 落到 build 树挨着 `.gcno`，并跳过 debugfs 里的 `.gcno` 符号链接（避免覆盖真实 notes）。

### 3.2 报告恒为 0 的真正根因（本周定位）
`gcov -b` 生成的 `.gcov` 文件**只有 4 行 header、无任何行/分支记录**，导致解析得 0。逐层定位：

1. 原始 `.gcda` 字节非空（如 `nl80211.gcda` 108844 字节、5443 个非零 32 位字）→ 捕获数据真实存在，非采集 bug。
2. 直接对 `nl80211.gcno` 跑 `gcov -b`，stdout 报 `Cannot open source file ../net/wireless/nl80211.c`，且 `.gcov` 仅 4 行 → **gcov 找不到源文件时不写行/分支记录**。
3. 根因：out-of-tree 构建（`O=build-qemu`）在 `.gcno` 内嵌的源码路径是相对路径（如 `../net/wireless/nl80211.c`），gcov 解析该路径时**相对的是 `.gcno` 自身所在目录**，故实际查找 `build-qemu/net/net/wireless/nl80211.c`（首级目录被「翻倍」），而源码在 `SOURCE/net/wireless/nl80211.c`。

### 3.3 修复
`gcov_parser._ensure_gcov_source_symlinks()`：为每对（build 顶级目录 `<A>` × source 顶级目录 `<B>`）建交叉符号链接 `build-qemu/<A>/<B> → SOURCE/<B>`（仅数百个，Windows 挂载卷上也快）。于是 gcov 查 `build-qemu/net/net/wireless/nl80211.c` 时经 `build-qemu/net/net` 符号链接落到 `SOURCE/net/wireless/nl80211.c`，正确解析源码。

**实证**：`nl80211.c.gcov` 从 4 行 → **35235 行、9705 条 `branch` 记录**，且带真实执行计数（如 `1:23027:int __init nl80211_init(void)`）。

---

## 4. 「分支覆盖率 90%」目标量化

- **测量基础设施已能产出真实数字**（不再恒为 0），精确聚合已完成（`recapture.py` 干净重算，容器 `x86-gcov-regen` 退出码 0）。
- **当前捕获只覆盖裸机引导路径**：内核启动 + 最小 initramfs init。未运行任何用户态负载 / 测试套件。
- **全量聚合结果（baseline_report.json）**：

  | 指标 | 已覆盖 | 总计 | 覆盖率 |
  |------|--------|------|--------|
  | 行（line） | 37754 | 424469 | **8.89%** |
  | 分支（branch） | 18212 | 327781 | **5.56%** |

- **单文件采样佐证**（直接 `gcov -b` 于真实 `.gcda`）：
  - `net/wireless/nl80211.c`：行执行 **0.06% of 9058**，分支执行 **0.04% of 9705**（冷门无线子系统，远低于引导热路径均值，说明覆盖率分布极不均匀）。

> 结论：裸机引导实测仅 5.56% 分支覆盖率，与 90% 目标差距巨大。要逼近 90%，必须在 guest 内运行**实际负载**（KUnit 全套、syscall 压力、网络/文件系统/设备操作等），而不仅是引导。当前 5.56% 仅代表「内核能起来」的底线，不是功能覆盖。

---

## 5. 本轮关键经验（可复用）

1. **out-of-tree gcov 必须解决源码路径解析**：交叉符号链接（build/<A>/<B>→source/<B>）比复制整棵源码树快几个数量级，且对 Windows 绑定挂载友好。
2. **「报告为 0」先查 `.gcov` 是否只有 header**：`gcov` 找不到源文件时静默写空，不是数据问题。
3. **QEMU `-kernel` 需 `bzImage`**（带 boot 头），裸 `vmlinux` ELF 不被 SeaBIOS 引导。
4. **`CONFIG_MODULES=n` 是反模式**（`=m` 受 `depends on m` 约束的子树会链接未定义）；out-of-tree 用 `=m→=y` sed + `MODULES=y` 才正确。
5. **进度可观测**：构建子进程流式透传 + 启动 `python3 -u`，进度监视器每 5 分钟写 `progress_feed.log`。

---

## 6. 待办 / 下一步

- [ ] **回报分支 owner 修复源码不一致**：`auditsc.c`(struct filename)、`slub.c`/`slab.h`(freelist_aba)、`i2c/Makefile`(i2c-core-objs)、`xt_tcpmss.c`(大小写)。这些使本分支无法作为标准 x86_64 内核编译。
- [ ] **在 initramfs 中注入 in-guest 负载**以拉升覆盖率：KUnit 显式跑测试套件 + 用户态负载（网络/文件系统/设备操作）。当前 KUnit 仅作为 initcall 自动跑，未覆盖 90% 代码。
- [x] **重算 `baseline_report.json`（精确聚合）**：已完成（容器 `x86-gcov-regen` 退出码 0）。结果：行 8.89%、分支 5.56%；确认与 90% 目标差距巨大。如引入负载，重测对比。
- [x] 清理 build 树中历史遗留的 6.4 万条 stray 符号链接（已用 `find -delete` 清理，仅保留交叉符号链接）。

---

## 附：产物与位置

| 产物 | 路径 |
|------|------|
| 基线报告（精确聚合） | `tools/testing/coverage/baseline/baseline_report.json` |
| 采集/解析脚本 | `tools/testing/coverage/baseline_measurement.py`、`gcov_parser.py`、`recapture.py` |
| 构建产物 | `build-qemu/vmlinux`、`build-qemu/arch/x86/boot/bzImage` |
| 镜像 | `kernel-coverage-qemu:latest`（qemu 7.2.22 + gcc/gcov 12.2.0） |
| 进度日志 | `tools/testing/coverage/progress_feed.log`、`regen.log` |
