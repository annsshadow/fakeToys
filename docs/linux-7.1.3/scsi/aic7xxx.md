
## Adaptec Aic7xxx Fast -> Ultra160 Family Manager Set v7.0


Linux 操作系统 README

本文件中包含以下信息
  1. 支持的硬  2. 版本历史
  3. 命令行选项
  4. 联系 Adaptec

## 1. 支持的硬

   aic7xxx 驱动支持以下 Adaptec SCSI 芯片与主机适配器
   ======== ===== ========= ======== ========= ===== ===============
   Chip     MIPS  Host Bus  MaxSync  MaxWidth  SCBs  Notes
   ======== ===== ========= ======== ========= ===== ===============
   aic7770  10    EISA/VL   10MHz    16Bit      4    1
   aic7850  10    PCI/32    10MHz    8Bit       3
   aic7855  10    PCI/32    10MHz    8Bit       3
   aic7856  10    PCI/32    10MHz    8Bit       3
   aic7859  10    PCI/32    20MHz    8Bit       3
   aic7860  10    PCI/32    20MHz    8Bit       3
   aic7870  10    PCI/32    10MHz    16Bit      16
   aic7880  10    PCI/32    20MHz    16Bit      16
   aic7890  20    PCI/32    40MHz    16Bit      16      3 4 5 6 7 8
   aic7891  20    PCI/64    40MHz    16Bit      16      3 4 5 6 7 8
   aic7892  20    PCI/64-66 80MHz    16Bit      16      3 4 5 6 7 8
   aic7895  15    PCI/32    20MHz    16Bit      16    2 3 4 5
   aic7895C 15    PCI/32    20MHz    16Bit      16    2 3 4 5     8
   aic7896  20    PCI/32    40MHz    16Bit      16    2 3 4 5 6 7 8
   aic7897  20    PCI/64    40MHz    16Bit      16    2 3 4 5 6 7 8
   aic7899  20    PCI/64-66 80MHz    16Bit      16    2 3 4 5 6 7 8
   ======== ===== ========= ======== ========= ===== ===============

   1. 多路复用双通道设备 - 单个控制器服务两条总线   2. 多功能双通道设备 - 单芯片上集成两个控制器   3. 命令通道次级 DMA 引擎 - 允许分散/聚集列表SCB 预取   4. 64 字节 SCB 支持 - 允许为所有可能的目标/lun 组合建立断开、无标签请求表   5. 块移动指令支- 使某些时序器操作速度翻倍   6. 'Bayonet' 风格分散/聚集引擎 - 提升 S/G 预取性能   7. 排队寄存- 允许在不暂停时序器的情况下排队新事务   8. 多目ID - 允许控制器作为目标在多个 SCSI ID 上响应选择
   ============== ======= =========== =============== =============== =========
   Controller      Chip   Host-Bus    Int-Connectors  Ext-Connectors  Notes
   ============== ======= =========== =============== =============== =========
   AHA-274X[A]    aic7770   EISA         SE-50M         SE-HD50F
   AHA-274X[A]W   aic7770   EISA         SE-HD68F       SE-HD68F
                                         SE-50M
   AHA-274X[A]T   aic7770   EISA       2 X SE-50M       SE-HD50F
   AHA-2842       aic7770    VL          SE-50M         SE-HD50F
   AHA-2940AU     aic7860   PCI/32       SE-50M         SE-HD50F
   AVA-2902I      aic7860   PCI/32       SE-50M
   AVA-2902E      aic7860   PCI/32       SE-50M
   AVA-2906       aic7856   PCI/32       SE-50M         SE-DB25F
   APC-7850       aic7850   PCI/32       SE-50M                       1
   AVA-2940       aic7860   PCI/32       SE-50M
   AHA-2920B      aic7860   PCI/32       SE-50M
   AHA-2930B      aic7860   PCI/32       SE-50M
   AHA-2920C      aic7856   PCI/32       SE-50M         SE-HD50F
   AHA-2930C      aic7860   PCI/32       SE-50M
   AHA-2930C      aic7860   PCI/32       SE-50M
   AHA-2910C      aic7860   PCI/32       SE-50M
   AHA-2915C      aic7860   PCI/32       SE-50M
   AHA-2940AU/CN  aic7860   PCI/32       SE-50M         SE-HD50F
   AHA-2944W      aic7870   PCI/32     HVD-HD68F        HVD-HD68F
                                       HVD-50M
   AHA-3940W      aic7870   PCI/32     2 X SE-HD68F     SE-HD68F        2
   AHA-2940UW     aic7880   PCI/32       SE-HD68F
                                         SE-50M         SE-HD68F
   AHA-2940U      aic7880   PCI/32       SE-50M         SE-HD50F
   AHA-2940D      aic7880   PCI/32
   aHA-2940 A/T   aic7880   PCI/32
   AHA-2940D A/T  aic7880   PCI/32
   AHA-3940UW     aic7880   PCI/32     2 X SE-HD68F     SE-HD68F          3
   AHA-3940UWD    aic7880   PCI/32     2 X SE-HD68F   2 X SE-VHD68F       3
   AHA-3940U      aic7880   PCI/32     2 X SE-50M       SE-HD50F          3
   AHA-2944UW     aic7880   PCI/32      HVD-HD68F       HVD-HD68F
                                        HVD-50M
   AHA-3944UWD    aic7880   PCI/32     2 X HVD-HD68F  2 X HVD-VHD68F      3
   AHA-4944UW     aic7880   PCI/32
   AHA-2930UW     aic7880   PCI/32
   AHA-2940UW Pro aic7880   PCI/32      SE-HD68F        SE-HD68F            4
                                        SE-50M
   AHA-2940UW/CN  aic7880   PCI/32
   AHA-2940UDual  aic7895   PCI/32
   AHA-2940UWDual aic7895   PCI/32
   AHA-3940UWD    aic7895   PCI/32
   AHA-3940AUW    aic7895   PCI/32
   AHA-3940AUWD   aic7895   PCI/32
   AHA-3940AU     aic7895   PCI/32
   AHA-3944AUWD   aic7895   PCI/32     2 X HVD-HD68F  2 X HVD-VHD68F
   AHA-2940U2B    aic7890   PCI/32      LVD-HD68F       LVD-HD68F
   AHA-2940U2 OEM aic7891   PCI/64
   AHA-2940U2W    aic7890   PCI/32      LVD-HD68F       LVD-HD68F
                                        SE-HD68F
                                        SE-50M
   AHA-2950U2B    aic7891   PCI/64      LVD-HD68F       LVD-HD68F
   AHA-2930U2     aic7890   PCI/32      LVD-HD68F       SE-HD50F
                                        SE-50M
   AHA-3950U2B    aic7897   PCI/64
   AHA-3950U2D    aic7897   PCI/64
   AHA-29160      aic7892   PCI/64-66
   AHA-29160 CPQ  aic7892   PCI/64-66
   AHA-29160N     aic7892   PCI/32      LVD-HD68F       SE-HD50F
                                        SE-50M
   AHA-29160LP    aic7892   PCI/64-66
   AHA-19160      aic7892   PCI/64-66
   AHA-29150LP    aic7892   PCI/64-66
   AHA-29130LP    aic7892   PCI/64-66
   AHA-3960D      aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   AHA-3960D CPQ  aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   AHA-39160      aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   ============== ======= =========== =============== =============== =========

   1. 不支BIOS
   2. 次级总线上带多个控制器芯片的 DEC21050 PCI-PCI 桥接   3. 次级总线上带多个控制器芯片的 DEC2115X PCI-PCI 桥接   4. 三个 SCSI 连接器可同时使用，而不会产SCSI "stub" 效应
## 2. 版本历史


   - 7.0	  (2005骞，鏈，鏃。
 - 更新驱动以使SCSI 传输类基础设施 - Adaptec 发布的上一版驱动中提取了时序器与核心修复
   - 6.2.36 (2003骞，鏈，鏃。
        - 修正禁用 PCI 奇偶校验错误检测的代码        - 修正并简化对忽略宽余量（ignore wide residue）消息的处理。如果事务数据长度为偶数且我们收IWR 消息，旧代码将无法报告剩余数据        - 增加2.5.X EISA 框架的支持        - 针对 2.5.X SCSI proc 文件系统接口的变更进行更新        - 修正域验证（Domain Validation）命令行选项解析        - 当通过 8 WDTR 消息协商异步模式时，发送偏移量0 SDTR，以确保目标设备知道我们处于异步模式。这可规Quantum Atlas 10K 的固件缺陷        - 在驱动挂载期间清PCI 错误状态，以免在我们接管控制器之前其他驱动探测产生的杂散写入导致内存映I/O 被禁用
   - 6.2.35 (2003骞，鏈?4鏃。
        - 修复若干 GCC 3.3 编译器警告        - 修正 EISA 双通道控制器的运行        - 增加2.5.X scsi_report_device_reset() 的支持
   - 6.2.34 (2003骞，鏈，鏃。
        - 修复 6.2.29 引入的锁回归问题，该问题可能导致 io_request_lock 与我们的 per-softc 锁之间出现锁顺序反转。此问题仅在 RH9、SuSE 以及 kernel.org 2.4.X 内核上可能出现
   - 6.2.33 (2003骞，鏈?0鏃。
        - 在已向用户报10 次错误后，动态禁PCI 奇偶校验错误报告。这些错误是由其他设备发出奇偶校验错误的 PCI 事务所致。一旦用户已被告知该问题，继续报告错误只会降低我们的性能
   - 6.2.32 (2003骞，鏈?8鏃。
        - 动态调S/G 列表大小，以避免 SCSI malloc 池碎片化SCSI 中间层死锁
   - 6.2.28 (2003骞，鏈?0鏃。
        - 域验证修        - 增加禁用 PCI 奇偶校验错误检测的能力        - 增强的内存映I/O 探测

   - 6.2.20 (2002骞?1鏈，鏃。
        - 增加域验证（Domain Validation）
## 3. 命令行选项



```

                 ALTERING OR ADDING THESE DRIVER PARAMETERS
                 INCORRECTLY CAN RENDER YOUR SYSTEM INOPERABLE.
                 USE THEM WITH CAUTION.

   Put a .conf file in the /etc/modprobe.d directory and add/edit a
   line containing ``options aic7xxx aic7xxx=[command[,command...]]`` where
   ``command`` is one or more of the following:

```
verbose

    :Definition: 在驱动运行期间启用额外的信息性消息    :Possible Values: 该选项是一个标    :Default Value: 禁用


debug:[value]

    :Definition: 启用各级别的调试信息
    :Possible Values: 0x0000 = 无调 0xffff = 完整调试
    :Default Value: 0x0000

no_probe

probe_eisa_vl

    :Definition: 不探EISA/VLB 控制器		 这是一个开关。如果驱动默认编译为不探EISA/VLB 控制器，
		 指定 "no_probe" 将启用此探测		 如果驱动默认编译为探EISA/VLB
		 控制器，指定 "no_probe" 将禁用此探测
    :Possible Values: 该选项是一个开    :Default Value: EISA/VLB 探测默认被禁用
pci_parity

    :Definition: 切换 PCI 奇偶校验错误的检测		 在许多采VIA 芯片组的主板上，
		 PCI 总线上的奇偶校验生成不正确。硬件无		 区分这些"虚假"奇偶校验错误		 真实奇偶校验错误。其症状```

		    "scsi0:	Data Parity Error Detected during address or write data phase"

		 驱动输出的信息
    :Possible Values: 该选项是一个开    :Default Value: PCI 奇偶校验错误报告默认被禁
```
no_reset

    :Definition: 在初始探测阶段不重置总线

    :Possible Values: 该选项是一个标    :Default Value: 禁用

extended

    :Definition: 在控制器上强制启用扩展转    :Possible Values: 该选项是一个标    :Default Value: 禁用

periodic_otag

    :Definition: 周期性发送有序标签以防止标签饥饿。某些较旧的设备需要此选项
    :Possible Values: 该选项是一个标    :Default Value: 禁用

reverse_scan

    :Definition: 以相反顺序探SCSI 总线，从目标 15 开
    :Possible Values: 该选项是一个标    :Default Value: 禁用

global_tag_depth:[value]

    :Definition: 所有总线上所有目标的全局标签深度		 该选项设置默认标签深度		 可被 tag_info 选项有选择地覆盖
    :Possible Values: 1 - 253
    :Default Value: 32

tag_info:{{value[,value...]}[,{value[,value...]}...]}

    :Definition: 按控制器设置每个目标的标记队列深度		 控制器和目标均可省略，表		 它们应保持默认标签深度
    :Possible Values: 1 - 253
    :Default Value: 32

    示例
```

	        tag_info:{{16,32,32,64,8,8,,32,32,32,32,32,32,32,32,32}

	    在控制器 0 上：

		- 为目0 指定标签深度 16
		- 为目3 指定标签深度 64
		- 为目4 5 指定标签深度 8
		- 目标 6 保持默认深度
		- 为目1-15 指定标签深度 32
		- 所有其他目标保持默认深度
	    ::

                tag_info:{{},{32,,32}}

	    在控制器 1 上：

		- 为目0 2 指定标签深度 32
		- 所有其他目标保持默认深度
```
seltime:[value]

    :Definition: 指定选择超时    :Possible Values: 0 = 256ms, 1 = 128ms, 2 = 64ms, 3 = 32ms
    :Default Value: 0

dv: {value[,value...]}

    :Definition: 按控制器设置域验证（Domain Validation）策略		 控制器可省略，表		 它们应保持默认读取流设置
    :Possible Values:

		      ==== ===============================
		       < 0 使用串行 EEPROM 中的设置                         0 禁用 DV
		       > 0 启用 DV
		      ==== ===============================


    :Default Value: 对于DV SCSI Select 选项的控制器，取 SCSI-Select 设置		   否则，支U160 速度的控制器为开启，其他所有控制器类型为关闭
    示例
```

		dv:{-1,0,,1,1,0}

	   - 控制0 保持 DV 默认设置	   - 控制1 禁用 DV	   - 跳过控制2 的配置	   - 控制3 4 启用 DV	   - 控制5 禁用 DV
```
```

    options aic7xxx aic7xxx=verbose,no_probe,tag_info:{{},{,,10}},seltime:1

```
启用详细日志，禁EISA/VLB 探测，并将控制器 1/目标 2 的标签深度设置为 10
## 4. Adaptec 客户支持


   Adaptec 技术支持需要一个技术支持标识（TSID）编号
    - 12 TSID 可在产品包装盒内的白色条形码标签上找到。TSID 通过准确识别您的产品和支持状态，帮助我们提供更高效的服务
   支持选项
    - http://ask.adaptec.com 搜索 Adaptec 支持知识库（ASK），获取有关您产品的文章、排障技巧和常见问题解答    - 如需通过电子邮件获得支持，请将您的问题提交至 http://ask.adaptec.com/ Adaptec 技术支持专家
   北美
    - 访问我们的网http://www.adaptec.com/    - 有关 Adaptec 支持选项的资讯，请拨408-957-2550，每24 小时，每7 天    - 如需与一名技术支持专家通话
      - 硬件产品请拨408-934-7274，周一至周五，太平洋夏令时 3:00 17:00      - RAID 与光纤通道产品请拨321-207-2000，周一至周五，太平洋夏令时 3:00 17:00
      为加快服务，请准备好您的计算机    - 订购 Adaptec 产品（包括配件和线缆），请拨408-957-7274；在线订购线缆请访问 http://www.adaptec.com/buy-cables/
   欧洲
    - 访问我们的网http://www.adaptec.com/en-US/_common/world_index    - 如需与一名技术支持专家通话，请拨打或发送电子邮件至
      - 德语+49 89 4366 5522，周一至周五，中欧时间 9:00-17:00        http://ask-de.adaptec.com/      - 法语+49 89 4366 5533，周一至周五，中欧时间 9:00-17:00	http://ask-fr.adaptec.com/      - 英语+49 89 4366 5544，周一至周五，格林尼治标准时间 9:00-17:00	http://ask.adaptec.com/
    - 您可以在线订Adaptec 线缆，网址 http://www.adaptec.com/buy-cables/
   日本
    - 访问我们的网http://www.adaptec.co.jp/    - 如需与一名技术支持专家通话，请拨打 +81 3 5308 6120，周一至周五，上午 9:00 12:00，下13:00 18:00
版权 |copy| 2003 Adaptec Inc. 691 S. Milpitas Blvd., Milpitas CA 95035 USA.

保留所有权利
允许您在遵守以下条件的前提下，随同受通用公共许可证（General Public License）约束的软件的再分发，整体或部分地再分发、使用和修改README 文件
1. README 文件的再分发必须保留上述版权声明、本条件列表以及以下免责声明，不得修改2. 未经明确的事先书面许可，不得使用作者姓名来背书或推广源自本软件的产品3. 修改或新增的贡献必须在版权声明中注明作者（"Contributor"），并添加在原始版权声明之下。该版权声明仅用于标识贡献者，不应被视为允许更Adaptec 所授予的权限
README 文件ADAPTEC 及贡献`AS IS` 提供，任何明示或默示的保证（包括但不限于针对非侵权性的保证，或关于适销性与特定用途适用性的默示保证）均被否认。在任何情况下，ADAPTEC 或贡献者均不对因使用本 README 文件（即使已被告知此类损害的可能性）而以任何责任理论（无论是合同、严格责任还是侵权，包括疏忽或其他）引起的任何直接、间接、偶然、特殊、惩戒性或后果性损害（包括但不限于替代商品或服务的采购、使用损失、数据或利润损失，或业务中断）承担责任