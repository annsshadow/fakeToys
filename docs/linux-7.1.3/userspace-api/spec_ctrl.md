## Speculation Control


相当多的 CPU 具有与推测执行（speculation）相关的缺陷特性，这些实际上是在各种形式下导致数据泄漏的漏洞，甚至会跨越特权域。

内核以各种形式提供针对此类漏洞的缓解（mitigation）措施。其中一些缓解措施在编译时可配置，一些可以通过内核命令行提供。

还有一类缓解措施非常昂贵，但可以将它们限制在受控环境中的某组进程或任务上。控制这些缓解措施的机制是通过 `prctl(2)`。

有两个与此相关的 prctl 选项：

 - PR_GET_SPECULATION_CTRL

 - PR_SET_SPECULATION_CTRL

### PR_GET_SPECULATION_CTRL


PR_GET_SPECULATION_CTRL 返回由 prctl(2) 的 arg2 选择的推测执行缺陷特性的状态。返回值使用位 0-3，含义如下（但要注意，PR_SPEC_L1D_FLUSH 的语义不那么直观，请参阅下面该特定控制的文档）：

==== ====================== ==================================================
Bit  Define                 Description
==== ====================== ==================================================
0    PR_SPEC_PRCTL          Mitigation 可通过 PR_SET_SPECULATION_CTRL 按任务控制。
1    PR_SPEC_ENABLE         推测特性已启用，缓解措施已禁用。
2    PR_SPEC_DISABLE        推测特性已禁用，缓解措施已启用。
3    PR_SPEC_FORCE_DISABLE  与 PR_SPEC_DISABLE 相同，但不可撤销。后续的
                            prctl(..., PR_SPEC_ENABLE) 将会失败。
4    PR_SPEC_DISABLE_NOEXEC 与 PR_SPEC_DISABLE 相同，但该状态会在 `execve(2)` 时清除。
==== ====================== ==================================================

如果所有位都为 0，则该 CPU 不受该推测执行缺陷特性的影响。

如果设置了 PR_SPEC_PRCTL，则可以使用按任务的缓解控制。如果未设置，对该推测执行缺陷特性调用 prctl(PR_SET_SPECULATION_CTRL) 将会失败。


### PR_SET_SPECULATION_CTRL


PR_SET_SPECULATION_CTRL 允许控制由 `prctl(2)` 的 arg2 按任务选择的推测执行缺陷特性。arg3 用于传入控制值，即 PR_SPEC_ENABLE 或 PR_SPEC_DISABLE 或 PR_SPEC_FORCE_DISABLE。

### Common error codes

======= =================================================================
Value   Meaning
======= =================================================================
EINVAL  该 prctl 未由架构实现，或未使用的 prctl(2) 参数不为 0。

ENODEV  arg2 选择了一个不受支持的推测执行缺陷特性。
======= =================================================================

### PR_SET_SPECULATION_CTRL error codes

======= =================================================================
Value   Meaning
======= =================================================================
0       成功

ERANGE  arg3 不正确，即它既不是 PR_SPEC_ENABLE 也不是
        PR_SPEC_DISABLE 也不是 PR_SPEC_FORCE_DISABLE。

ENXIO   对于 PR_SPEC_STORE_BYPASS：由于系统的启动配置，无法通过 prctl
        控制所选的推测执行缺陷特性。

EPERM   已经使用 PR_SPEC_FORCE_DISABLE 禁用了推测，而调用者试图再次
        启用它。

EPERM   对于 PR_SPEC_L1D_FLUSH 和 PR_SPEC_INDIRECT_BRANCH：由于系统的启动
        配置，无法控制缓解措施。

======= =================================================================

### Speculation misfeature controls

- PR_SPEC_STORE_BYPASS: 推测性存储绕过（Speculative Store Bypass）

  调用方式：
   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_FORCE_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_DISABLE_NOEXEC, 0, 0);

- PR_SPEC_INDIR_BRANCH: 用户进程中的间接分支推测
                        （缓解针对用户进程的 Spectre V2 风格攻击）

  调用方式：
   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_FORCE_DISABLE, 0, 0);

- PR_SPEC_L1D_FLUSH: 在任务上下文切换出去时刷新 L1D 缓存
                        （仅在任务运行在非 SMT 核心上时有效）

对于这个控制，PR_SPEC_ENABLE 表示**缓解措施**已启用（L1D 被刷新），PR_SPEC_DISABLE 表示它已禁用。

  调用方式：
   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, PR_SPEC_DISABLE, 0, 0);
