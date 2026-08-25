## 间接目标选择（ITS

ITS 是部分支持增强型 IBRS（Enhanced IBRS）、且Alder Lake 之前发布Intel CPU 中存在的一个漏洞。ITS 可能允许攻击者控制位于缓存行（cacheline）下半部分的间接分支RET 指令的预测目标

ITS 被分配了 CVE-2024-28956，CVSS 评分4.7（中危）

### 影响范围

- **eIBRS 客户宿主机隔*：KVM/内核中的间接分支仍可能被预测为对应客户机中某条分支的非预期目标

- **模式BTI（Intra-Mode BTI*：内核内部的训练，例如通过 cBPF 或其他原gadget

- **间接分支预测屏障（IBPB*：在执行IBPB 之后，间接分支仍可能被预测为 IBPB 之前执行的直接分支所对应的目标。该问题已由 IPU 2025.1 微码修复，应通过发行版更新获取；也可Intel github 仓库获取微码 [#f1]_

### 受影响的 CPU

以下是受 ITS 影响CPU 列表 [#f2]_ [#f3]_

======================== ============ ==================== ===============
通用名称                   Family_Model  eIBRS                模式BTI
                                         瀹㈡埛鏈，瀹夸富鏈洪殧绂。
======================== ============ ==================== ===============
SKYLAKE_X (step >= 6)     06_55H        受影              受影
ICELAKE_X                 06_6AH        不受影响             受影
ICELAKE_D                 06_6CH        不受影响             受影
ICELAKE_L                 06_7EH        不受影响             受影
TIGERLAKE_L               06_8CH        不受影响             受影
TIGERLAKE                 06_8DH        不受影响             受影
KABYLAKE_L (step >= 12)   06_8EH        受影              受影
KABYLAKE (step >= 13)     06_9EH        受影              受影
COMETLAKE                 06_A5H        受影              受影
COMETLAKE_L               06_A6H        受影              受影
ROCKETLAKE                06_A7H        不受影响             受影
======================== ============ ==================== ===============

- 所有受影响CPU 都枚举出增强IBRS 特性
- IBPB 隔离在所有受 ITS 影响CPU 上都受影响，需要微码更新才能缓解
- 受影响的 CPU 均未枚举 BHI_CTRL，该特性在 Golden Cove（Alder Lake Sapphire Rapids）中引入。这有助于客户机判断宿主机的受影响状态
- Intel Atom CPU 不受 ITS 影响

### 缓解措施

由于只有指令最后一个字节位于缓存行下半部分的间接分支与 RET 才受 ITS 影响，缓解措施的基本思路是：不允许间接分支出现在缓存行下半部分

这是通过依赖内核中已有的 retpoline 支持以及编译器来达成的。易ITS 影响retpoline 调用点会在运行时被补丁修改为指向新加入的 ITS 安全 thunk。这些安thunk 的间接分支位于缓存行的下半部分。并非所retpoline 调用点都会被补丁修改thunk：若某个 retpoline 调用点经评估ITS 安全的，则会被替换为内联的间接分支

#### 动thunk

从一个动态分配的 thunk 池中，每个易受影响的调用点都会被替换为一个新thunk，从而得到一个唯一地址。这有助于提升分支预测的准确性，同时也是针对别名（aliasing）问题的纵深防御措施

需要注意的是，为简单起见，eBPF 程序中的间接分支总是被替换为跳转`__x86_indirect_its_thunk_array` 中的静thunk。如有需要，未来可改为使用动thunk

所有易受影响的 RET 都被替换为静thunk，它们不使用动thunk。这是因RET 的预测主要来RSB，基本不依赖源地址。发RSB 下溢RET 可能从动thunk 中受益；RET 的数量远多于间接分支，而唯一源地址带来的任何收益，都可能被增大的指令缓存（icache）占用与 iTLB 压力所抵消

#### Retpoline

Retpoline 序列同样可以缓解 ITS 不安全的间接分支。因此，在启retpoline 时，ITS 缓解措施仅将 RET 重定位到安全 thunk，除非用户显式请RSB 填充（RSB-stuffing）缓解

#### RSB 填充

通过调用深度追踪（Call Depth Tracking）进行的 RSB 填充，是针对 Retbleed RSB 下溢攻击的一种缓解措施，它同时也能缓解受 ITS 影响RET

##### 客户机中的缓

所有客户机默认都会部署 ITS 缓解措施，无论客户机是否枚举 eIBRS、也无论Family/Model 如何。这是因eIBRS 特性对客户机而言可能被隐藏。唯一的例外是当客户机枚举BHI_DIS_S 时，表明该客户机运行在不受影响的宿主机上

为防止客户机在不受影响的平台上不必要地部署缓解措施，Intel MSR `IA32_ARCH_CAPABILITIES` 中定义了 ITS_NO 位（62 位）。当客户机看到该位置位时，不应枚ITS 漏洞。请注意，任何硬件都不会设置该位，它**仅供 VMM 根据宿主机的受影响状态为客户机合*

##### 缓解选项

ITS 缓解措施可通过内核参数 "indirect_target_selection" 进行控制。可用选项如下

======== ===================================================================
on       （默认）部署“对齐分返回 thunk”缓解措施
        spectre_v2 缓解启用retpoline，则对齐 thunk 仅部署到受影响的
        RET 指令上；retpoline 负责缓解间接分支

off      禁用 ITS 缓解措施

vmexit   CPU ITS 的客户机/宿主机隔离部分影响，则等同于on”；
        否则不部署缓解措施。当宿主机用户态不在威胁模型中、仅考虑从客户机
        到宿主机的攻击时，该选项很有用

stuff    在同时部retpoline 时，部署 RSB 填充缓解；否则部署默认缓解
        retpoline 缓解启用时，通过调用深度追踪进行RSB 填充同样
        缓解 ITS

force    强制判定存在 ITS 漏洞并部署默认缓解措施
======== ===================================================================

### Sysfs 报告

显示 ITS 缓解状态的 sysfs 文件为：

/sys/devices/system/cpu/vulnerabilities/indirect_target_selection

请注意，微码缓解状态不会在该文件中报告

该文件可能的取值为

- `Not affected` —处理器不受此漏洞影响
- `Vulnerable` —系统易受攻击，且未应用任何缓解措施
- `Vulnerable, KVM: Not affected` —系统易受模式BTI 攻击，但不受 eIBRS 客户宿主机隔离影响
- `Mitigation: Aligned branch/return thunks` —已启用缓解措施，受影响的间接分支RET 被重定位到安thunk
- `Mitigation: Retpolines, Stuffing RSB` —已通过 retpoline RSB 填充启用缓解措施

### 参考文

.. [#f1] 微码仓库 - https://github.com/intel/Intel-Linux-Processor-Microcode-Data-Files

.. [#f2] 受影响处理器列表 - https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html

.. [#f3] 受影响处理器列表（机器可读） - https://github.com/intel/Intel-affected-processor-list
