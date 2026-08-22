
## Adaptec dpti 驱动


Redistribution and use in source form, with or without modification, are
permitted provided that redistributions of source code must retain the
above copyright notice, this list of conditions and the following disclaimer.

This software is provided `as is` by Adaptec and
any express or implied warranties, including, but not limited to, the
implied warranties of merchantability and fitness for a particular purpose,
are disclaimed. In no event shall Adaptec be
liable for any direct, indirect, incidental, special, exemplary or
consequential damages (including, but not limited to, procurement of
substitute goods or services; loss of use, data, or profits; or business
interruptions) however caused and on any theory of liability, whether in
contract, strict liability, or tort (including negligence or otherwise)
arising in any way out of the use of this driver software, even if advised
of the possibility of such damage.

本驱动支Adaptec I2O RAID DPT SmartRAID V I2O 板卡
## 致谢


原始 Linux 驱动Karen White Dell Computer 期间移植Linux。它移植Bob Pasteur
（DPT）的原始Linux 驱动。Mark Salyzyn Bob Pasteur 对原始驱动提供了咨询
2.0 版本的驱动由 Deanna Bonds Mark Salyzyn 完成
## 历史


该驱动最初被移植linux 2.0.34 版本
==== ==========================================================================
V2.0 驱动重写。基i2o 子系统重新架构。这是第一个完GPL 的版本，因为上一个版本使用的
     i2osig 头文件不GPL 的。开发者测试版本V2.1 内部测试
V2.2 首个发布版本

V2.3 变更
     - 增加Raptor 支持
     - 修复了在负载极重、管理工具运行（kmalloc 标志中移GFP_DMA）时导致系统挂起的缺
V2.4 首个准备好提交并嵌入内核的版
     变更
     - 实现Alan Cox 的建     - sg 层增加了 resid 的计     - 更好的错误处     - 增加了下溢条件检     - 增加DATAPROTECT 检     - 更改了错误返回码
     - 修复了总线复位例程中的指针缺陷
     - 启用了来ioctl hba 复位（允FW 刷写后重启并使用FW，而无需重启系统     - 更改proc 输出
==== ==========================================================================

## 待办


- 64 位架构上编译时增64 位分聚集（Scatter Gather）支- 增加稀LUN 扫描
- 增加scsi-core 发出 test unit ready inquiry 命令时，检查曾被离线（FW 层面）的设备
  现已在线的代- 增加 proc 读接- busrescan 命令
- rescan 命令
- 增加scsi-core 通知新设备的 rescan 例程代码
- 增加 C-PCI（热插拔相关）支- 增加 ioctl 透传错误恢复

## 说明


DPT 卡会优化命令处理的顺序。因此，一条命令在发送到板卡后最多可能需6 分钟才能完成
文件 dpti_ioctl.h、dptsig.h、osd_defs.h、osd_util.h、sys_info.h Adaptec 管理例程的接文件的一部分。它们定义了 ioctl 中使用的结构体。它们被写成可移植的。它们难以阅读，但我需“原样”使用它们，否则我可能会漏掉接口的变更