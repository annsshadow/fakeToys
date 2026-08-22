## Confidential Computing Linux 用于 x86 virtualization



鐢? Elena Reshetova <elena.reshetova@intel.com> 鍜?Carlos Bilbao <carlos.bilbao.osdev@gmail.com>

## Motivation


内核 developers working confidential computing 用于 virtualized
environments x86 operate 在…下 一set assumptions regarding the Linux
内核 threat 型号 differ 来自 the traditional view. Historically,
the Linux threat 型号 acknowledges attackers residing userspace, 作为
well 作为 一limited set 外部 attackers able interact 
the 内核 through 各种 networking limited HW-specific exposed
interfaces (USB, thunderbolt). The goal 鐨，姝?document 鏄，鍒?explain
额外 attack vectors arise the confidential computing space
discuss the proposed protection mechanisms 用于 the Linux 内核.

## Overview 鍜?terminology


Confidential Computing (CoCo) 一broad term covering 一wide range 
安全 technologies aim protect the confidentiality integrity
数据 使用 (对比. 数据 rest 数据 transit). 核心, CoCo
solutions 提供 一Trusted Execution Environment (TEE), 何处 secure 数据
processing performed  因此, 它们typically further
classified 进入 不同 subtypes depending the SW intended
运行 TEE. document focuses 一subclass CoCo technologies
targeting virtualized environments 允许 运行虚拟
Machines (VM) inside TEE. 来自 现在 document referring
subclass CoCo 作为 'Confidential Computing (CoCo) 用于 the
virtualized environments (VE)'.

CoCo, the virtualization 上下 refers 一set HW SW
technologies 允许 用于 stronger 安全 guarantees 用于 the SW 运行
inside 一CoCo VM. Namely, confidential computing allows users 
confirm the trustworthiness 全部 SW pieces 包含 reduced
Trusted Computing Base (TCB) given ability attest the 状这些
trusted components.

同时 the concrete implementation details differ 之间 technologies, 全部
可用 mechanisms aim 提供 increased confidentiality 
integrity 用于 the VM's guest 内存 execution 状(vCPU 寄存,
更多 tightly controlled guest 中断 injection, 以及 一
额外 mechanisms control guest-host 映射. 更多 details 
the x86-特定 solutions found 
[Intel Trust Domain Extensions (TDX) </arch/x86/tdx>](Intel Trust Domain Extensions (TDX) </arch/x86/tdx>) 鍜。
`AMD Memory Encryption <https://www.amd.com/system/files/techdocs/sev-snp-strengthening-vm-isolation-with-integrity-protection-and-more.pdf>`_.

The 基本 CoCo guest layout 包含 the host, guest, the interfaces 
communicate guest host, 一platform capable supporting CoCo VMs, 
一trusted intermediary 之间 the guest VM the underlying platform
acts 作为 一安全 manager. The host-side 虚拟 machine 监视
(VMM) typically consists 一subset traditional VMM 特
仍然 charge the guest lifecycle, i.e. 创建 destroy 一CoCo
VM, manage access 系统 resources,  然 since 
typically stays 超出 CoCo VM TCB, access limited preserve the
安全 objectives.

the 以下 diagram, the "<--->" lines represent bi-directional
communication channels interfaces 之间 the CoCo 安全 manager 
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
The 特定 details the CoCo 安全 manager vastly diverge 之间
technologies. 例如, 一cases, implemented HW
同时 others pure SW.

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
瀛樺湪 涔?communication 涔嬮棿 the bootloader 鍜?the 鍐呮牳 鏈熼棿
the boot 进程, diagram 执行 represent explicitly. The
"Interfaces" box represents the 各种 interfaces 允许
communication 之间 内核 userspace. 包含 系统 calls,
内核 APIs, 设备 驱动, 

The existing Linux 内核 threat 型号 typically assumes execution 一
trusted HW platform 全部 the 固件 bootloaders included 
TCB. The primary attacker resides the userspace, 全部 the 数据
coming 来自 存在 generally considered untrusted, 除非 userspace 
privileged enough perform trusted actions. 此外, 外部
attackers typically considered, including 那些 access 已启
外部 网络 (e.g. 以太 无线, 蓝牙), exposed 硬件
interfaces (e.g. USB, Thunderbolt), 鍜?the ability 鍒?modify the contents
鐨?disks offline.

Regarding 外部 attack vectors, 它是 interesting 注意 大多
cases 外部 attackers try exploit vulnerabilities userspace
第一, 它是 可能 用于 一attacker directly target the
内核; particularly the host 具有 物理 access. 示例 direct
内核 attacks 包含 the vulnerabilities CVE-2019-19524, CVE-2022-0435
鍜?CVE-2020-24490.

## Confidential Computing threat 鍨嬪彿 鍜，鍏，瀹夊叏 objectives


Confidential Computing adds 一类型 attacker the 上文 列出: 一
potentially misbehaving host (包含 一part 一
traditional VMM 全部 , typically placed outside the
CoCo VM TCB 由于 large SW attack surface. 它是 重要 注意
璇，姝?doesn鈥檛 imply 璇?the host 鎴?VMM 鏄?intentionally
malicious, 那里 exists 一安全 having 一small CoCo
VM TCB. 类型 adversary viewed 作为 一更多 powerful 类型
外部 attacker, 作为 resides locally the 相同 物理 machine
(相比之下 一remote 网络 attacker) 具有 control 在…上 the guest
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
同时 traditionally the host 具有 unlimited access guest 数据 
leverage access attack the guest, the CoCo 系统 mitigate 此类
attacks adding 安全 特类似 guest 数据 confidentiality 
integrity protection. threat 型号 assumes 那些 特
可用 intact.

The **Linux 内核 CoCo VM 安全 objectives** summarized 作为 follows:

1. Preserve the confidentiality integrity CoCo guest's 私有
内存 寄存

2. Prevent privileged escalation 来自 一host 进入 一CoCo guest Linux 内核.
同时 它是 true the host (host-side VMM) 需一level 
privilege 创建, destroy, pause the guest, part the goal 
preventing privileged escalation ensure 这些 操作 执行 
提供 一pathway 用于 attackers gain access the guest's 内核.

The 上文 安全 objectives result two primary **Linux 内核 CoCo
VM assets**:

1. Guest 内核 execution 上下
2. Guest 内核 私有 内存.

The host retains full control 在…上 the CoCo guest resources, 拒绝
access them 任何 time. 示例 resources 包含 CPU time, 内存
the guest consume, 网络 bandwidth,  因为  the
host Denial 鐨?Service (DoS) attacks against CoCo guests 鏄?beyond the
scope threat 型号.

The **Linux CoCo VM attack surface** 任何 接口 exposed 来自 一CoCo
guest Linux 内核 towards 一untrusted host covered the
CoCo technology SW/HW protection. 包含 任何 可能
side-channels, 以及 transient execution side channels. 示例 
explicit (side-channel) interfaces 包含 accesses 端口 I/O, MMIO
DMA interfaces, access PCI 配置 space, VMM-specific
hypercalls (towards Host-side VMM), access shared 内存 
中断 allowed injected 进入 the guest 内核 the host, 作为
well 作为 CoCo technology-specific hypercalls, present. Additionally, the
host 一CoCo 系统 typically controls the 进程 creating 一CoCo
guest: 具有 一方法 加载 进入 一guest the 固件 bootloader
images, the 内核 image together the 内核 命令 line. 全部 
数据 应当 considered untrusted 直到 integrity 
authenticity established 通过 attestation.

The 下文 显示 一threat matrix 用于 the CoCo guest Linux 内核 
执行 discuss potential mitigation strategies. The matrix refers 
CoCo-特定 versions the guest, host platform.

   :widths: auto
   :align: center
   :header-rows: 1

   - - Threat name
     - Threat description

   - - Guest malicious 配置
     - 一misbehaving host modifies one the 以下 guest's
       配置:

       1. Guest 固件 bootloader

       2. Guest 内核 模块 binaries

       3. Guest 命令 line 参数

       allows the host break the integrity the code 运行
       inside 一CoCo guest, violates the CoCo 安全 objectives.

   - - CoCo guest 数据 attacks
     - 一misbehaving host retains full control the CoCo guest's 数据
       in-transit 之间 the guest the host-managed 物理 
       虚拟 设备. allows 任何 attack against confidentiality,
       integrity freshness 此类 数据.

   - - Malformed runtime 输入
     - 一misbehaving host injects malformed 输入 通过 任何 communication
       接口 使用 the guest's 内核 code. the code 
       prepared handle 输入 correctly, result 一host
       --> guest 内核 privilege escalation. 包含 traditional
       side-channel 鍜，鎴?transient execution attack vectors.

   - - Malicious runtime 输入
     - 一misbehaving host injects 一特定 输入 通过 任何
       communication 接口 使用 the guest's 内核 code. The
       difference the 前一attack vector (malformed runtime 输入)
       输入 malformed, crafted 
       impact the guest's 内核 安全. 示例 此类 inputs 包含
       providing 一malicious time the guest the entropy the guest
       random 数字 generator. Additionally, the timing 此类 事件 
       一attack vector own, results 一特定 guest
       内核 action (i.e. processing 一host-injected 中断).
       resistant supplied host 输入.

