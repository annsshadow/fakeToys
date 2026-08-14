## Confidential Computing 在 Linux 用于 x86 virtualization



由: Elena Reshetova <elena.reshetova@intel.com> 和 Carlos Bilbao <carlos.bilbao.osdev@gmail.com>

## Motivation


内核 developers working 在 confidential computing 用于 virtualized
environments 在 x86 operate 在…下 一个 set 的 assumptions regarding the Linux
内核 threat 型号 该 differ 来自 the traditional view. Historically,
the Linux threat 型号 acknowledges attackers residing 在 userspace, 作为
well 作为 一个 limited set 的 外部 attackers 该 是 able 到 interact 与
the 内核 through 各种 networking 或 limited HW-specific exposed
interfaces (USB, thunderbolt). The goal 的 此 document 是 到 explain
额外 attack vectors 该 arise 在 the confidential computing space
和 discuss the proposed protection mechanisms 用于 the Linux 内核.

## Overview 和 terminology


Confidential Computing (CoCo) 是 一个 broad term covering 一个 wide range 的
安全 technologies 该 aim 到 protect the confidentiality 和 integrity
的 数据 在 使用 (对比. 数据 在 rest 或 数据 在 transit). 在 其 核心, CoCo
solutions 提供 一个 Trusted Execution Environment (TEE), 何处 secure 数据
processing 可 为 performed 和, 因此, 它们是 typically further
classified 进入 不同 subtypes depending 在 the SW 即 intended
到 为 运行 在 TEE. 此 document focuses 在 一个 subclass 的 CoCo technologies
该 是 targeting virtualized environments 和 允许 运行中 虚拟
Machines (VM) inside TEE. 来自 现在 在 在 此 document 将 为 referring
到 此 subclass 的 CoCo 作为 'Confidential Computing (CoCo) 用于 the
virtualized environments (VE)'.

CoCo, 在 the virtualization 上下文, refers 到 一个 set 的 HW 和/或 SW
technologies 该 允许 用于 stronger 安全 guarantees 用于 the SW 运行中
inside 一个 CoCo VM. Namely, confidential computing allows 其 users 到
confirm the trustworthiness 的 全部 SW pieces 到 包含 在 其 reduced
Trusted Computing Base (TCB) given 其 ability 到 attest the 状态 的 这些
trusted components.

同时 the concrete implementation details differ 之间 technologies, 全部
可用 mechanisms aim 到 提供 increased confidentiality 和
integrity 用于 the VM's guest 内存 和 execution 状态 (vCPU 寄存器),
更多 tightly controlled guest 中断 injection, 以及 一些
额外 mechanisms 到 control guest-host 页 映射. 更多 details 在
the x86-特定 solutions 可 为 found 在
[Intel Trust Domain Extensions (TDX) </arch/x86/tdx>](Intel Trust Domain Extensions (TDX) </arch/x86/tdx>) 和
`AMD Memory Encryption <https://www.amd.com/system/files/techdocs/sev-snp-strengthening-vm-isolation-with-integrity-protection-and-more.pdf>`_.

The 基本 CoCo guest layout 包含 the host, guest, the interfaces 该
communicate guest 和 host, 一个 platform capable 的 supporting CoCo VMs, 和
一个 trusted intermediary 之间 the guest VM 和 the underlying platform
该 acts 作为 一个 安全 manager. The host-side 虚拟 machine 监视器
(VMM) typically consists 的 一个 subset 的 traditional VMM 特性 和
是 仍然 在 charge 的 the guest lifecycle, i.e. 创建 或 destroy 一个 CoCo
VM, manage 其 access 到 系统 resources, 等. 然而, since 它
typically stays 超出 CoCo VM TCB, 其 access 是 limited 到 preserve the
安全 objectives.

在 the 以下 diagram, the "<--->" lines represent bi-directional
communication channels 或 interfaces 之间 the CoCo 安全 manager 和
```

    +-------------------+      +-----------------------+
    | CoCo guest VM     |<---->|                       |
    +-------------------+      |                       |
      | Interfaces |           | CoCo security manager |
    +-------------------+      |                       |
    | Host VMM          |<---->|                       |
    +-------------------+      |                       |
                               |                       |
    +--------------------+     |                       |
    | CoCo platform      |<--->|                       |
    +--------------------+     +-----------------------+

```
The 特定 details 的 the CoCo 安全 manager vastly diverge 之间
technologies. 例如, 在 一些 cases, 它 将 为 implemented 在 HW
同时 在 others 它 可 为 pure SW.

## Existing Linux 内核 threat 型号


```

     +-----------------------+      +-------------------+
     |                       |<---->| Userspace         |
     |                       |      +-------------------+
     |   External attack     |         | Interfaces |
     |       vectors         |      +-------------------+
     |                       |<---->| Linux Kernel      |
     |                       |      +-------------------+
     +-----------------------+      +-------------------+
                                    | Bootloader/BIOS   |
                                    +-------------------+
                                    +-------------------+
                                    | HW platform       |
                                    +-------------------+

```
存在 也 communication 之间 the bootloader 和 the 内核 期间
the boot 进程, 但 此 diagram 执行 不 represent 它 explicitly. The
"Interfaces" box represents the 各种 interfaces 该 允许
communication 之间 内核 和 userspace. 此 包含 系统 calls,
内核 APIs, 设备 驱动, 等.

The existing Linux 内核 threat 型号 typically assumes execution 在 一个
trusted HW platform 与 全部 的 the 固件 和 bootloaders included 在
其 TCB. The primary attacker resides 在 the userspace, 和 全部 的 the 数据
coming 来自 存在 generally considered untrusted, 除非 userspace 是
privileged enough 到 perform trusted actions. 此外, 外部
attackers 是 typically considered, including 那些 与 access 到 已启用
外部 网络 (e.g. 以太网, 无线, 蓝牙), exposed 硬件
interfaces (e.g. USB, Thunderbolt), 和 the ability 到 modify the contents
的 disks offline.

Regarding 外部 attack vectors, 它是 interesting 到 注意 该 在 大多数
cases 外部 attackers 将 try 到 exploit vulnerabilities 在 userspace
第一, 但 该 它是 可能 用于 一个 attacker 到 directly target the
内核; particularly 若 the host 具有 物理 access. 示例 的 direct
内核 attacks 包含 the vulnerabilities CVE-2019-19524, CVE-2022-0435
和 CVE-2020-24490.

## Confidential Computing threat 型号 和 其 安全 objectives


Confidential Computing adds 一个 新 类型 的 attacker 到 the 上文 列出: 一个
potentially misbehaving host (其 可 也 包含 一些 part 的 一个
traditional VMM 或 全部 的 它), 其 是 typically placed outside 的 the
CoCo VM TCB 由于 其 large SW attack surface. 它是 重要 到 注意
该 此 doesn’t imply 该 the host 或 VMM 是 intentionally
malicious, 但 该 那里 exists 一个 安全 值 在 having 一个 small CoCo
VM TCB. 此 新 类型 的 adversary 可 为 viewed 作为 一个 更多 powerful 类型
的 外部 attacker, 作为 它 resides locally 在 the 相同 物理 machine
(相比之下 到 一个 remote 网络 attacker) 和 具有 control 在…上 the guest
```

                                 +------------------------+
                                 |    CoCo guest VM       |
   +-----------------------+     |  +-------------------+ |
   |                       |<--->|  | Userspace         | |
   |                       |     |  +-------------------+ |
   |   External attack     |     |     | Interfaces |     |
   |       vectors         |     |  +-------------------+ |
   |                       |<--->|  | Linux Kernel      | |
   |                       |     |  +-------------------+ |
   +-----------------------+     |  +-------------------+ |
                                 |  | Bootloader/BIOS   | |
   +-----------------------+     |  +-------------------+ |
   |                       |<--->+------------------------+
   |                       |          | Interfaces |
   |                       |     +------------------------+
   |     CoCo security     |<--->| Host/Host-side VMM |
   |      manager          |     +------------------------+
   |                       |     +------------------------+
   |                       |<--->|   CoCo platform        |
   +-----------------------+     +------------------------+

```
同时 traditionally the host 具有 unlimited access 到 guest 数据 和 可
leverage 此 access 到 attack the guest, the CoCo 系统 mitigate 此类
attacks 由 adding 安全 特性 类似 guest 数据 confidentiality 和
integrity protection. 此 threat 型号 assumes 该 那些 特性 是
可用 和 intact.

The **Linux 内核 CoCo VM 安全 objectives** 可 为 summarized 作为 follows:

1. Preserve the confidentiality 和 integrity 的 CoCo guest's 私有
内存 和 寄存器.

2. Prevent privileged escalation 来自 一个 host 进入 一个 CoCo guest Linux 内核.
同时 它是 true 该 the host (和 host-side VMM) 需要 一些 level 的
privilege 到 创建, destroy, 或 pause the guest, part 的 the goal 的
preventing privileged escalation 是 到 ensure 该 这些 操作 执行 不
提供 一个 pathway 用于 attackers 到 gain access 到 the guest's 内核.

The 上文 安全 objectives result 在 two primary **Linux 内核 CoCo
VM assets**:

1. Guest 内核 execution 上下文.
2. Guest 内核 私有 内存.

The host retains full control 在…上 the CoCo guest resources, 和 可 拒绝
access 到 them 在 任何 time. 示例 的 resources 包含 CPU time, 内存
该 the guest 可 consume, 网络 bandwidth, 等. 因为 的 此, the
host Denial 的 Service (DoS) attacks against CoCo guests 是 beyond the
scope 的 此 threat 型号.

The **Linux CoCo VM attack surface** 是 任何 接口 exposed 来自 一个 CoCo
guest Linux 内核 towards 一个 untrusted host 即 不 covered 由 the
CoCo technology SW/HW protection. 此 包含 任何 可能
side-channels, 以及 transient execution side channels. 示例 的
explicit (不 side-channel) interfaces 包含 accesses 到 端口 I/O, MMIO
和 DMA interfaces, access 到 PCI 配置 space, VMM-specific
hypercalls (towards Host-side VMM), access 到 shared 内存 页,
中断 allowed 到 为 injected 进入 the guest 内核 由 the host, 作为
well 作为 CoCo technology-specific hypercalls, 若 present. Additionally, the
host 在 一个 CoCo 系统 typically controls the 进程 的 creating 一个 CoCo
guest: 它 具有 一个 方法 到 加载 进入 一个 guest the 固件 和 bootloader
images, the 内核 image together 与 the 内核 命令 line. 全部 的 此
数据 应当 也 为 considered untrusted 直到 其 integrity 和
authenticity 是 established 通过 attestation.

The 表 下文 显示 一个 threat matrix 用于 the CoCo guest Linux 内核 但
执行 不 discuss potential mitigation strategies. The matrix refers 到
CoCo-特定 versions 的 the guest, host 和 platform.

   :widths: auto
   :align: center
   :header-rows: 1

   - - Threat name
     - Threat description

   - - Guest malicious 配置
     - 一个 misbehaving host modifies one 的 the 以下 guest's
       配置:

       1. Guest 固件 或 bootloader

       2. Guest 内核 或 模块 binaries

       3. Guest 命令 line 参数

       此 allows the host 到 break the integrity 的 the code 运行中
       inside 一个 CoCo guest, 和 violates the CoCo 安全 objectives.

   - - CoCo guest 数据 attacks
     - 一个 misbehaving host retains full control 的 the CoCo guest's 数据
       in-transit 之间 the guest 和 the host-managed 物理 或
       虚拟 设备. 此 allows 任何 attack against confidentiality,
       integrity 或 freshness 的 此类 数据.

   - - Malformed runtime 输入
     - 一个 misbehaving host injects malformed 输入 通过 任何 communication
       接口 使用 由 the guest's 内核 code. 若 the code 是 不
       prepared 到 handle 此 输入 correctly, 此 可 result 在 一个 host
       --> guest 内核 privilege escalation. 此 包含 traditional
       side-channel 和/或 transient execution attack vectors.

   - - Malicious runtime 输入
     - 一个 misbehaving host injects 一个 特定 输入 值 通过 任何
       communication 接口 使用 由 the guest's 内核 code. The
       difference 与 the 前一个 attack vector (malformed runtime 输入)
       是 该 此 输入 是 不 malformed, 但 其 值 是 crafted 到
       impact the guest's 内核 安全. 示例 的 此类 inputs 包含
       providing 一个 malicious time 到 the guest 或 the entropy 到 the guest
       random 数字 generator. Additionally, the timing 的 此类 事件 可
       为 一个 attack vector 在 其 own, 若 它 results 在 一个 特定 guest
       内核 action (i.e. processing 的 一个 host-injected 中断).
       resistant 到 supplied host 输入.

