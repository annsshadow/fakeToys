## CPU 到 ISA 版本映射

本文以表格形式列出 PowerPC 各 CPU 版本（如 Power10、Power9、Power8 等）所对应的 Power ISA 架构版本，并标注相关例外情况，供内核移植与指令集兼容性工作参考。



部分 CPU 版本到相关 ISA 版本的映射。

注意 Power4 和 Power4+ 不受支持。

========= ====================================================================
CPU       架构版本
========= ====================================================================
Power10   Power ISA v3.1
Power9    Power ISA v3.0B
Power8    Power ISA v2.07
e6500     Power ISA v2.06（有一些例外）
e5500     Power ISA v2.06（有一些例外，无 Altivec）
Power7    Power ISA v2.06
Power6    Power ISA v2.05
PA6T      Power ISA v2.04
Cell PPU  - Power ISA v2.02（有一些小的例外）
          - 外加 Altivec/VMX ~= 2.03
Power5++  Power ISA v2.04（无 VMX）
Power5+   Power ISA v2.03
Power5    - PowerPC 用户指令集架构 卷 I v2.02
          - PowerPC 虚拟环境架构 卷 II v2.02
          - PowerPC 操作环境架构 卷 III v2.02
PPC970    - PowerPC 用户指令集架构 卷 I v2.01
          - PowerPC 虚拟环境架构 卷 II v2.01
          - PowerPC 操作环境架构 卷 III v2.01
          - 外加 Altivec/VMX ~= 2.03
Power4+   - PowerPC 用户指令集架构 卷 I v2.01
          - PowerPC 虚拟环境架构 卷 II v2.01
          - PowerPC 操作环境架构 卷 III v2.01
Power4    - PowerPC 用户指令集架构 卷 I v2.00
          - PowerPC 虚拟环境架构 卷 II v2.00
          - PowerPC 操作环境架构 卷 III v2.00
========= ====================================================================


### 关键特性


========== ==================
CPU        VMX（即 Altivec）
========== ==================
Power10    是（Yes）
Power9     是（Yes）
Power8     是（Yes）
e6500      是（Yes）
e5500      否（No）
Power7     是（Yes）
Power6     是（Yes）
PA6T       是（Yes）
Cell PPU   是（Yes）
Power5++   否（No）
Power5+    否（No）
Power5     否（No）
PPC970     是（Yes）
Power4+    否（No）
Power4     否（No）
========== ==================

========== ====
CPU        VSX
========== ====
Power10    是（Yes）
Power9     是（Yes）
Power8     是（Yes）
e6500      否（No）
e5500      否（No）
Power7     是（Yes）
Power6     否（No）
PA6T       否（No）
Cell PPU   否（No）
Power5++   否（No）
Power5+    否（No）
Power5     否（No）
PPC970     否（No）
Power4+    否（No）
Power4     否（No）
========== ====

========== ====================================
CPU        事务内存（Transactional Memory）
========== ====================================
Power10    否（No）（* 参见 Power ISA v3.1 中"关于从架构中移除事务内存的附录 A 说明"）
Power9     是（Yes）（* 参见 transactional_memory.txt）
Power8     是（Yes）
e6500      否（No）
e5500      否（No）
Power7     否（No）
Power6     否（No）
PA6T       否（No）
Cell PPU   否（No）
Power5++   否（No）
Power5+    否（No）
Power5     否（No）
PPC970     否（No）
Power4+    否（No）
Power4     否（No）
========== ====================================

