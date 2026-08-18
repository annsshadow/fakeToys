## Confidential Computing 鍦?Linux 鐢ㄤ簬 x86 virtualization



鐢? Elena Reshetova <elena.reshetova@intel.com> 鍜?Carlos Bilbao <carlos.bilbao.osdev@gmail.com>

## Motivation


鍐呮牳 developers working 鍦?confidential computing 鐢ㄤ簬 virtualized
environments 鍦?x86 operate 鍦ㄢ€︿笅 涓€涓?set 鐨?assumptions regarding the Linux
鍐呮牳 threat 鍨嬪彿 璇?differ 鏉ヨ嚜 the traditional view. Historically,
the Linux threat 鍨嬪彿 acknowledges attackers residing 鍦?userspace, 浣滀负
well 浣滀负 涓€涓?limited set 鐨?澶栭儴 attackers 璇?鏄?able 鍒?interact 涓?
the 鍐呮牳 through 鍚勭 networking 鎴?limited HW-specific exposed
interfaces (USB, thunderbolt). The goal 鐨?姝?document 鏄?鍒?explain
棰濆 attack vectors 璇?arise 鍦?the confidential computing space
鍜?discuss the proposed protection mechanisms 鐢ㄤ簬 the Linux 鍐呮牳.

## Overview 鍜?terminology


Confidential Computing (CoCo) 鏄?涓€涓?broad term covering 涓€涓?wide range 鐨?
瀹夊叏 technologies 璇?aim 鍒?protect the confidentiality 鍜?integrity
鐨?鏁版嵁 鍦?浣跨敤 (瀵规瘮. 鏁版嵁 鍦?rest 鎴?鏁版嵁 鍦?transit). 鍦?鍏?鏍稿績, CoCo
solutions 鎻愪緵 涓€涓?Trusted Execution Environment (TEE), 浣曞 secure 鏁版嵁
processing 鍙?涓?performed 鍜? 鍥犳, 瀹冧滑鏄?typically further
classified 杩涘叆 涓嶅悓 subtypes depending 鍦?the SW 鍗?intended
鍒?涓?杩愯 鍦?TEE. 姝?document focuses 鍦?涓€涓?subclass 鐨?CoCo technologies
璇?鏄?targeting virtualized environments 鍜?鍏佽 杩愯涓?铏氭嫙
Machines (VM) inside TEE. 鏉ヨ嚜 鐜板湪 鍦?鍦?姝?document 灏?涓?referring
鍒?姝?subclass 鐨?CoCo 浣滀负 'Confidential Computing (CoCo) 鐢ㄤ簬 the
virtualized environments (VE)'.

CoCo, 鍦?the virtualization 涓婁笅鏂? refers 鍒?涓€涓?set 鐨?HW 鍜?鎴?SW
technologies 璇?鍏佽 鐢ㄤ簬 stronger 瀹夊叏 guarantees 鐢ㄤ簬 the SW 杩愯涓?
inside 涓€涓?CoCo VM. Namely, confidential computing allows 鍏?users 鍒?
confirm the trustworthiness 鐨?鍏ㄩ儴 SW pieces 鍒?鍖呭惈 鍦?鍏?reduced
Trusted Computing Base (TCB) given 鍏?ability 鍒?attest the 鐘舵€?鐨?杩欎簺
trusted components.

鍚屾椂 the concrete implementation details differ 涔嬮棿 technologies, 鍏ㄩ儴
鍙敤 mechanisms aim 鍒?鎻愪緵 increased confidentiality 鍜?
integrity 鐢ㄤ簬 the VM's guest 鍐呭瓨 鍜?execution 鐘舵€?(vCPU 瀵勫瓨鍣?,
鏇村 tightly controlled guest 涓柇 injection, 浠ュ強 涓€浜?
棰濆 mechanisms 鍒?control guest-host 椤?鏄犲皠. 鏇村 details 鍦?
the x86-鐗瑰畾 solutions 鍙?涓?found 鍦?
[Intel Trust Domain Extensions (TDX) </arch/x86/tdx>](Intel Trust Domain Extensions (TDX) </arch/x86/tdx>) 鍜?
`AMD Memory Encryption <https://www.amd.com/system/files/techdocs/sev-snp-strengthening-vm-isolation-with-integrity-protection-and-more.pdf>`_.

The 鍩烘湰 CoCo guest layout 鍖呭惈 the host, guest, the interfaces 璇?
communicate guest 鍜?host, 涓€涓?platform capable 鐨?supporting CoCo VMs, 鍜?
涓€涓?trusted intermediary 涔嬮棿 the guest VM 鍜?the underlying platform
璇?acts 浣滀负 涓€涓?瀹夊叏 manager. The host-side 铏氭嫙 machine 鐩戣鍣?
(VMM) typically consists 鐨?涓€涓?subset 鐨?traditional VMM 鐗规€?鍜?
鏄?浠嶇劧 鍦?charge 鐨?the guest lifecycle, i.e. 鍒涘缓 鎴?destroy 涓€涓?CoCo
VM, manage 鍏?access 鍒?绯荤粺 resources, 绛? 鐒惰€? since 瀹?
typically stays 瓒呭嚭 CoCo VM TCB, 鍏?access 鏄?limited 鍒?preserve the
瀹夊叏 objectives.

鍦?the 浠ヤ笅 diagram, the "<--->" lines represent bi-directional
communication channels 鎴?interfaces 涔嬮棿 the CoCo 瀹夊叏 manager 鍜?
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
The 鐗瑰畾 details 鐨?the CoCo 瀹夊叏 manager vastly diverge 涔嬮棿
technologies. 渚嬪, 鍦?涓€浜?cases, 瀹?灏?涓?implemented 鍦?HW
鍚屾椂 鍦?others 瀹?鍙?涓?pure SW.

## Existing Linux 鍐呮牳 threat 鍨嬪彿


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
the boot 杩涚▼, 浣?姝?diagram 鎵ц 涓?represent 瀹?explicitly. The
"Interfaces" box represents the 鍚勭 interfaces 璇?鍏佽
communication 涔嬮棿 鍐呮牳 鍜?userspace. 姝?鍖呭惈 绯荤粺 calls,
鍐呮牳 APIs, 璁惧 椹卞姩, 绛?

The existing Linux 鍐呮牳 threat 鍨嬪彿 typically assumes execution 鍦?涓€涓?
trusted HW platform 涓?鍏ㄩ儴 鐨?the 鍥轰欢 鍜?bootloaders included 鍦?
鍏?TCB. The primary attacker resides 鍦?the userspace, 鍜?鍏ㄩ儴 鐨?the 鏁版嵁
coming 鏉ヨ嚜 瀛樺湪 generally considered untrusted, 闄ら潪 userspace 鏄?
privileged enough 鍒?perform trusted actions. 姝ゅ, 澶栭儴
attackers 鏄?typically considered, including 閭ｄ簺 涓?access 鍒?宸插惎鐢?
澶栭儴 缃戠粶 (e.g. 浠ュお缃? 鏃犵嚎, 钃濈墮), exposed 纭欢
interfaces (e.g. USB, Thunderbolt), 鍜?the ability 鍒?modify the contents
鐨?disks offline.

Regarding 澶栭儴 attack vectors, 瀹冩槸 interesting 鍒?娉ㄦ剰 璇?鍦?澶у鏁?
cases 澶栭儴 attackers 灏?try 鍒?exploit vulnerabilities 鍦?userspace
绗竴, 浣?璇?瀹冩槸 鍙兘 鐢ㄤ簬 涓€涓?attacker 鍒?directly target the
鍐呮牳; particularly 鑻?the host 鍏锋湁 鐗╃悊 access. 绀轰緥 鐨?direct
鍐呮牳 attacks 鍖呭惈 the vulnerabilities CVE-2019-19524, CVE-2022-0435
鍜?CVE-2020-24490.

## Confidential Computing threat 鍨嬪彿 鍜?鍏?瀹夊叏 objectives


Confidential Computing adds 涓€涓?鏂?绫诲瀷 鐨?attacker 鍒?the 涓婃枃 鍒楀嚭: 涓€涓?
potentially misbehaving host (鍏?鍙?涔?鍖呭惈 涓€浜?part 鐨?涓€涓?
traditional VMM 鎴?鍏ㄩ儴 鐨?瀹?, 鍏?鏄?typically placed outside 鐨?the
CoCo VM TCB 鐢变簬 鍏?large SW attack surface. 瀹冩槸 閲嶈 鍒?娉ㄦ剰
璇?姝?doesn鈥檛 imply 璇?the host 鎴?VMM 鏄?intentionally
malicious, 浣?璇?閭ｉ噷 exists 涓€涓?瀹夊叏 鍊?鍦?having 涓€涓?small CoCo
VM TCB. 姝?鏂?绫诲瀷 鐨?adversary 鍙?涓?viewed 浣滀负 涓€涓?鏇村 powerful 绫诲瀷
鐨?澶栭儴 attacker, 浣滀负 瀹?resides locally 鍦?the 鐩稿悓 鐗╃悊 machine
(鐩告瘮涔嬩笅 鍒?涓€涓?remote 缃戠粶 attacker) 鍜?鍏锋湁 control 鍦ㄢ€︿笂 the guest
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
鍚屾椂 traditionally the host 鍏锋湁 unlimited access 鍒?guest 鏁版嵁 鍜?鍙?
leverage 姝?access 鍒?attack the guest, the CoCo 绯荤粺 mitigate 姝ょ被
attacks 鐢?adding 瀹夊叏 鐗规€?绫讳技 guest 鏁版嵁 confidentiality 鍜?
integrity protection. 姝?threat 鍨嬪彿 assumes 璇?閭ｄ簺 鐗规€?鏄?
鍙敤 鍜?intact.

The **Linux 鍐呮牳 CoCo VM 瀹夊叏 objectives** 鍙?涓?summarized 浣滀负 follows:

1. Preserve the confidentiality 鍜?integrity 鐨?CoCo guest's 绉佹湁
鍐呭瓨 鍜?瀵勫瓨鍣?

2. Prevent privileged escalation 鏉ヨ嚜 涓€涓?host 杩涘叆 涓€涓?CoCo guest Linux 鍐呮牳.
鍚屾椂 瀹冩槸 true 璇?the host (鍜?host-side VMM) 闇€瑕?涓€浜?level 鐨?
privilege 鍒?鍒涘缓, destroy, 鎴?pause the guest, part 鐨?the goal 鐨?
preventing privileged escalation 鏄?鍒?ensure 璇?杩欎簺 鎿嶄綔 鎵ц 涓?
鎻愪緵 涓€涓?pathway 鐢ㄤ簬 attackers 鍒?gain access 鍒?the guest's 鍐呮牳.

The 涓婃枃 瀹夊叏 objectives result 鍦?two primary **Linux 鍐呮牳 CoCo
VM assets**:

1. Guest 鍐呮牳 execution 涓婁笅鏂?
2. Guest 鍐呮牳 绉佹湁 鍐呭瓨.

The host retains full control 鍦ㄢ€︿笂 the CoCo guest resources, 鍜?鍙?鎷掔粷
access 鍒?them 鍦?浠讳綍 time. 绀轰緥 鐨?resources 鍖呭惈 CPU time, 鍐呭瓨
璇?the guest 鍙?consume, 缃戠粶 bandwidth, 绛? 鍥犱负 鐨?姝? the
host Denial 鐨?Service (DoS) attacks against CoCo guests 鏄?beyond the
scope 鐨?姝?threat 鍨嬪彿.

The **Linux CoCo VM attack surface** 鏄?浠讳綍 鎺ュ彛 exposed 鏉ヨ嚜 涓€涓?CoCo
guest Linux 鍐呮牳 towards 涓€涓?untrusted host 鍗?涓?covered 鐢?the
CoCo technology SW/HW protection. 姝?鍖呭惈 浠讳綍 鍙兘
side-channels, 浠ュ強 transient execution side channels. 绀轰緥 鐨?
explicit (涓?side-channel) interfaces 鍖呭惈 accesses 鍒?绔彛 I/O, MMIO
鍜?DMA interfaces, access 鍒?PCI 閰嶇疆 space, VMM-specific
hypercalls (towards Host-side VMM), access 鍒?shared 鍐呭瓨 椤?
涓柇 allowed 鍒?涓?injected 杩涘叆 the guest 鍐呮牳 鐢?the host, 浣滀负
well 浣滀负 CoCo technology-specific hypercalls, 鑻?present. Additionally, the
host 鍦?涓€涓?CoCo 绯荤粺 typically controls the 杩涚▼ 鐨?creating 涓€涓?CoCo
guest: 瀹?鍏锋湁 涓€涓?鏂规硶 鍒?鍔犺浇 杩涘叆 涓€涓?guest the 鍥轰欢 鍜?bootloader
images, the 鍐呮牳 image together 涓?the 鍐呮牳 鍛戒护 line. 鍏ㄩ儴 鐨?姝?
鏁版嵁 搴斿綋 涔?涓?considered untrusted 鐩村埌 鍏?integrity 鍜?
authenticity 鏄?established 閫氳繃 attestation.

The 琛?涓嬫枃 鏄剧ず 涓€涓?threat matrix 鐢ㄤ簬 the CoCo guest Linux 鍐呮牳 浣?
鎵ц 涓?discuss potential mitigation strategies. The matrix refers 鍒?
CoCo-鐗瑰畾 versions 鐨?the guest, host 鍜?platform.

   :widths: auto
   :align: center
   :header-rows: 1

   - - Threat name
     - Threat description

   - - Guest malicious 閰嶇疆
     - 涓€涓?misbehaving host modifies one 鐨?the 浠ヤ笅 guest's
       閰嶇疆:

       1. Guest 鍥轰欢 鎴?bootloader

       2. Guest 鍐呮牳 鎴?妯″潡 binaries

       3. Guest 鍛戒护 line 鍙傛暟

       姝?allows the host 鍒?break the integrity 鐨?the code 杩愯涓?
       inside 涓€涓?CoCo guest, 鍜?violates the CoCo 瀹夊叏 objectives.

   - - CoCo guest 鏁版嵁 attacks
     - 涓€涓?misbehaving host retains full control 鐨?the CoCo guest's 鏁版嵁
       in-transit 涔嬮棿 the guest 鍜?the host-managed 鐗╃悊 鎴?
       铏氭嫙 璁惧. 姝?allows 浠讳綍 attack against confidentiality,
       integrity 鎴?freshness 鐨?姝ょ被 鏁版嵁.

   - - Malformed runtime 杈撳叆
     - 涓€涓?misbehaving host injects malformed 杈撳叆 閫氳繃 浠讳綍 communication
       鎺ュ彛 浣跨敤 鐢?the guest's 鍐呮牳 code. 鑻?the code 鏄?涓?
       prepared 鍒?handle 姝?杈撳叆 correctly, 姝?鍙?result 鍦?涓€涓?host
       --> guest 鍐呮牳 privilege escalation. 姝?鍖呭惈 traditional
       side-channel 鍜?鎴?transient execution attack vectors.

   - - Malicious runtime 杈撳叆
     - 涓€涓?misbehaving host injects 涓€涓?鐗瑰畾 杈撳叆 鍊?閫氳繃 浠讳綍
       communication 鎺ュ彛 浣跨敤 鐢?the guest's 鍐呮牳 code. The
       difference 涓?the 鍓嶄竴涓?attack vector (malformed runtime 杈撳叆)
       鏄?璇?姝?杈撳叆 鏄?涓?malformed, 浣?鍏?鍊?鏄?crafted 鍒?
       impact the guest's 鍐呮牳 瀹夊叏. 绀轰緥 鐨?姝ょ被 inputs 鍖呭惈
       providing 涓€涓?malicious time 鍒?the guest 鎴?the entropy 鍒?the guest
       random 鏁板瓧 generator. Additionally, the timing 鐨?姝ょ被 浜嬩欢 鍙?
       涓?涓€涓?attack vector 鍦?鍏?own, 鑻?瀹?results 鍦?涓€涓?鐗瑰畾 guest
       鍐呮牳 action (i.e. processing 鐨?涓€涓?host-injected 涓柇).
       resistant 鍒?supplied host 杈撳叆.

