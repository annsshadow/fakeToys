## VMSCAPE


VMSCAPE 是一种漏洞，可能允许客户机影响宿主用户态中的分支预测。它尤其影响
QEMU 这样的虚拟机监控器（hypervisor）
即使某个虚拟机监控器可能没有任何敏感数据（如磁盘加密密钥），客户机用户也可能利用虚拟机监控器作为“被混淆的代理人”（confused deputy）来攻击
客户机内核
### 受影响的处理

以下 CPU 系列VMSCAPE 影响
**Intel 处理器：**
  - Skylake 代（不含 Enhanced-IBRS 的型号）
  - Cascade Lake 代（ITS 客户宿主隔离影响的型号）
  - Alder Lake 及更新（BHI 影响的型号）

注意，使BHB 清除软件缓解措施BHI 受影响型号（例如 Icelake）不VMSCAPE 影响
**AMD 处理器：**
  - Zen 系列（family 0x17x19x1a
** 海光（Hygon）处理器*
 - Family 0x18

### 缓解措施


### 条件IBPB


内核会跟踪某CPU 何时运行过潜在恶意的客户机，并在 VM-exit 之后首次退出到
用户态之前发IBPB。若用户态在 VM-exit 与下一VM-entry 之间没有运行，则
不发IBPB
注意，现有的针对 Spectre-v2 的用户态缓解措施在保护用户态方面是有效的。它不足以保护用户VMM 免受恶意客户机的攻击。这是因Spectre-v2 缓解措施上下文切换时应用，而用户VMM 可以在没有上下文切换的情况下VM-exit 之后
运行
漏洞枚举与缓解措施不会在客户机内部应用。这是因为嵌套的虚拟机监控器应当
已经部署 IBPB 来将自身与嵌套客户机隔离
### SMT 考量


当同时多线程（SMT）启用时，虚拟机监控器可能受到跨线程攻击。为了在 SMT 环境
中获得针VMSCAPE 攻击的完整保护，应启STIBP
SMT 已启用但缺乏足够STIBP 保护，内核将发出警告。以下情况不发出警告
- SMT 已禁- STIBP 已在系统范围内启- Intel eIBRS 已启用（意味着具有 STIBP 保护
### 系统信息与选项


显示 VMSCAPE 缓解状态的 sysfs 文件为：

  /sys/devices/system/cpu/vulnerabilities/vmscape

该文件中可能的值为
 - 'Not affected'（不受影响）
   处理器不VMSCAPE 攻击影响
 - 'Vulnerable'（易受攻击）
   处理器易受攻击且未应用任何缓解措施
 - 'Mitigation: IBPB before exit to userspace'（缓解：退出到用户态前 IBPB）：

   已启用条件IBPB 缓解。内核会跟踪某个 CPU 何时运行过潜在恶意的客户机，
   并在 VM-exit 之后首次退出到用户态之前发IBPB
 - 'Mitigation: IBPB on VMEXIT'（缓解：VMEXIT IBPB）：

   每次 VM-exit 都发IBPB。这发生在其他缓解措施（RETBLEED SRSO   已经VM-exit 时发IBPB 的情况下
### 内核命令行上的缓解控

可通过 `vmscape=` 命令行参数控制缓解措施：

 - `vmscape=off`锛。
   禁用 VMSCAPE 缓解
 - `vmscape=ibpb`锛。
   启用条件IBPB 缓解（当 CONFIG_MITIGATION_VMSCAPE=y 时的默认值）
 - `vmscape=force`锛。
   即使在不已知受影响的处理器上，也强制进行漏洞检测与缓解