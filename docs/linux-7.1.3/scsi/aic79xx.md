
## Adaptec Ultra320 系列管理套件


Linux 操作系统自述文件


  1. 支持的硬件
  2. 版本历史
  3. 命令行选项
  4. 补充说明
  5. 联系 Adaptec

## 1. 支持的硬件


   本驱动套件支持以下 Adaptec SCSI 主机适配器。

   =============              =========================================
   Ultra320 ASIC              Description
   =============              =========================================
   AIC-7901A                  Single Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC
   AIC-7901B                  Single Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC with Retained Training
   AIC-7902A4                 Dual Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC
   AIC-7902B                  Dual Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC with Retained Training
   =============              =========================================

   ========================== ===================================== ============
   Ultra320 Adapters          Description                              ASIC
   ========================== ===================================== ============
   Adaptec SCSI Card 39320    Dual Channel 64-bit PCI-X 133MHz to   7902A4/7902B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin)
   Adaptec SCSI Card 39320A   Dual Channel 64-bit PCI-X 133MHz to      7902B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin)
   Adaptec SCSI Card 39320D   Dual Channel 64-bit PCI-X 133MHz to      7902A4
                              Ultra320 SCSI Card (two external VHDC
                              and one internal 68-pin)
   Adaptec SCSI Card 39320D   Dual Channel 64-bit PCI-X 133MHz to      7902A4
                              Ultra320 SCSI Card (two external VHDC
                              and one internal 68-pin) based on the
                              AIC-7902B ASIC
   Adaptec SCSI Card 29320    Single Channel 64-bit PCI-X 133MHz to    7901A
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin, one
                              internal 50-pin)
   Adaptec SCSI Card 29320A   Single Channel 64-bit PCI-X 133MHz to    7901B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin, one
                              internal 50-pin)
   Adaptec SCSI Card 29320LP  Single Channel 64-bit Low Profile        7901A
                              PCI-X 133MHz to Ultra320 SCSI Card
                              (One external VHDC, one internal
                              68-pin)
   Adaptec SCSI Card 29320ALP Single Channel 64-bit Low Profile        7901B
                              PCI-X 133MHz to Ultra320 SCSI Card
                              (One external VHDC, one internal
                              68-pin)
   ========================== ===================================== ============

## 2. 版本历史


 - 3.0	  (2005 年 12 月 1 日)
 - 更新驱动以使用 SCSI transport class 基础设施
 - 从 Adaptec 发布的 2.0.15 版驱动中提取序列器和核心修复

 - 1.3.11 (2003 年 7 月 11 日)
        - 修复若干死锁问题。
        - 添加 29320ALP 和 39320B 的 ID。

 - 1.3.10 (2003 年 6 月 3 日)
        - 将 SCB_TAG 字段对齐到 16 字节边界。这避免了在某些
          PCI-33 总线上出现 SCB 损坏。
        - 修正 Rev B. 硬件上的非零 lun。
        - 针对 2.5.X SCSI proc FS 接口的改动进行更新。
        - 当通过 8bit WDTR 消息协商为异步时，发送一个偏移量为 0 的
          SDTR，以确保目标端知道我们处于异步模式。这绕过了
          Quantum Atlas 10K 固件的一个缺陷。
        - 实现控制器的挂起和恢复。
        - 在驱动挂载期间清除 PCI 错误状态，以免由于在我们声明
          控制器之前其他驱动探测产生的杂散写操作而导致
          内存映射 I/O 被禁用。

 - 1.3.9 (2003 年 5 月 22 日)
        - 修复编译器错误。
        - 移除对跨越 4GB 边界的段进行 S/G 拆分。在 Linux 中
          保证不会发生这种情况。
        - 添加对 2.5.X 内核中 scsi_report_device_reset() 的支持。
        - 添加 7901B 支持。
        - 简化打包 lun Rev A workaround 的处理。
        - 修正并简化对忽略宽残差（ignore wide residue）消息的处理。
          之前的代码在事务数据长度为偶数且我们收到 IWR 消息时
          会无法报告残差。

 - 1.3.8 (2003 年 4 月 29 日)
        - 修复通过命令行接口代码访问的类型。
        - 执行若干固件优化。
        - 修复 "Unexpected PKT busfree" 错误。
        - 使用序列器中断来通知主机存在状态错误的命令。我们将
          通知推迟到没有未决选择时，以确保主机被中断的时间尽可能短。
        - 移除对 2.2.X 之前版本的支持。
        - 添加对新的 2.5.X 中断 API 的支持。
        - 修正大端架构支持。

 - 1.3.7 (2003 年 4 月 16 日)
        - 使用 del_timer_sync() 确保在控制器关闭期间没有
          待处理的超时。
        - 对于 2.5.X 之前的内核，仔细调整我们的段列表大小，以避免
          SCSI malloc 池碎片化。
        - 清理 /proc 输出中的通道显示。
        - 在 add-single-device 期间绕过中间层设备列表中重复的
          设备条目。

 - 1.3.6 (2003 年 3 月 28 日)
        - 修正 Domain Validation 代码中的双重释放。
        - 修正控制器关闭期间对已释放内存的引用。
        - 在 SE->LVD 切换时复位总线。这是为重置我们的收发器所必须的。

 - 1.3.5 (2003 年 3 月 24 日)
        - 修复若干寄存器窗口模式 bug。
        - 在我们诊断以及 /proc 中显示的 PPR 标志里包含读流式。
        - 添加对 2.5.X 内核的 PCI 热插拔支持。
        - 修正 RevA 硬件的默认预补偿值。
        - 修复 Domain Validation 线程关闭。
        - 添加一个固件 workaround，使 H2A4 上打包操作期间的
          LED 闪烁更亮。
        - 修正用户读流式设置的 /proc 显示。
        - 通过从中间层进入驱动时释放 io_request_lock 来简化
          驱动加锁。
        - 清理命令行解析，并将大部分代码移至 aiclib。

 - 1.3.4 (2003 年 2 月 28 日)
        - 修正错误恢复处理程序中的竞态条件。
        - 允许在 Domain Validation 期间 Test Unit Ready 命令占用完整的 5 秒。

 - 1.3.2 (2003 年 2 月 19 日)
        - 修正由于 1.3.1 中包含的 GEM318 兼容性修复导致的 Rev B. 回归。

 - 1.3.1 (2003 年 2 月 11 日)
        - 添加对 39320A 的支持。
        - 改进对某些 PCI-X 错误的恢复。
        - 修复对同一写入事务中可能出现的、中间没有训练的
          LQ/DATA/LQ/DATA 的处理。
        - 修正与 GEM318 机箱服务设备的兼容性问题。
        - 修正在高标签深度写负载下出现的数据损坏问题。
        - 适配 2.5.X daemonize() API 的变更。
        - 修正 "Missing case in ahd_handle_scsiint" 恐慌。

 - 1.3.0 (2003 年 1 月 21 日)
        - 完成所有 U320 产品的完整回归测试。
        - 添加 abort 和目标/lun 复位错误恢复处理程序以及
          中断聚合（interrupt coalescing）。

 - 1.2.0 (2002 年 11 月 14 日)
        - 添加对 Domain Validation 的支持
        - 添加对惠普（Hewlett-Packard）版本的 39320D 和 AIC-7902
          适配器的支持。

        对之前适配器的支持尚未经过完整测试，应仅在客户自行承担
        风险的情况下使用。

 - 1.1.1 (2002 年 9 月 24 日)
        - 添加对 Linux 2.5.X 内核系列的支持

 - 1.1.0 (2002 年 9 月 17 日)
        - 添加对另外四种 SCSI 产品的支持：
          ASC-39320、ASC-29320、ASC-29320LP、AIC-7901。

 - 1.0.0 (2002 年 5 月 30 日)
        - 驱动初始发布。

 - 2.1. 软件/硬件特性
        - 支持 SPI-4 "Ultra320" 标准：
          - 320MB/s 传输速率
          - 160MB/s 和 320MB/s 的打包 SCSI 协议
          - 快速仲裁选择（QAS）
          - 保留训练信息（仅 Rev B. ASIC）
        - 中断聚合（Interrupt Coalescing）
        - 发起者模式（目前不支持目标模式）
        - 支持最高 133MHz 的 PCI-X 标准
        - 支持 PCI v2.2 标准
        - Domain Validation

 - 2.2. 操作系统支持：
        - Redhat Linux 7.2、7.3、8.0、Advanced Server 2.1
        - SuSE Linux 7.3、8.0、8.1、Enterprise Server 7
        - 目前仅支持 Intel 和 AMD x86
        - 支持 >4GB 内存配置。

     更多详情请参阅用户指南。

## 3. 命令行选项


```

	         ALTERING OR ADDING THESE DRIVER PARAMETERS
                 INCORRECTLY CAN RENDER YOUR SYSTEM INOPERABLE.
                 USE THEM WITH CAUTION.

   Put a .conf file in the /etc/modprobe.d/ directory and add/edit a
   line containing ``options aic79xx aic79xx=[command[,command...]]`` where
   ``command`` is one or more of the following:


```
verbose
    :Definition: 在驱动运行期间启用额外的信息性消息。
    :Possible Values: 该选项为一个标志
    :Default Value: 禁用

debug:[value]
    :Definition: 启用不同级别的调试信息。
                 调试掩码的位定义可以在 drivers/scsi/aic7xxx/aic79xx.h
                 的 "Debug" 标题下找到。
    :Possible Values: 0x0000 = 无调试，0xffff = 完整调试
    :Default Value: 0x0000

no_reset
    :Definition: 在初始探测阶段不复位总线
    :Possible Values: 该选项为一个标志
    :Default Value: 禁用

extended
    :Definition: 在控制器上强制使用扩展转换
    :Possible Values: 该选项为一个标志
    :Default Value: 禁用

periodic_otag
    :Definition: 周期性发送一个有序标签以防止标签饥饿。某些较旧的设备需要。
    :Possible Values: 该选项为一个标志
    :Default Value: 禁用

reverse_scan
    :Definition: 以反向顺序探测 scsi 总线，从目标 15 开始
    :Possible Values: 该选项为一个标志
    :Default Value: 禁用

global_tag_depth
    :Definition: 所有总线上所有目标的全局标签深度。
		 该选项设置默认标签深度，可被
		 tag_info 选项有选择地覆盖。

    :Possible Values: 1 - 253
    :Default Value: 32

tag_info:{{value[,value...]}[,{value[,value...]}...]}
    :Definition: 基于每个控制器设置每个目标的标记队列深度。控制器和目标
                 均可省略，表示它们应保留默认标签深度。

    :Possible Values: 1 - 253
    :Default Value: 32

    Examples:


```

	    tag_info:{{16,32,32,64,8,8,,32,32,32,32,32,32,32,32,32}

	在控制器 0 上

	    - 为目标 0 指定标签深度 16
	    - 为目标 3 指定标签深度 64
	    - 为目标 4 和 5 指定标签深度 8
	    - 目标 6 保留默认值
	    - 为目标 1,2,7-15 指定标签深度 32

	所有其他目标保留默认深度。

	::

	    tag_info:{{},{32,,32}}

	在控制器 1 上

	    - 为目标 0 和 2 指定标签深度 32

	所有其他目标保留默认深度。


```
rd_strm: {rd_strm_bitmask[,rd_strm_bitmask...]}
    :Definition: 基于每个目标启用读流式。
		 rd_strm_bitmask 是一个 16 位十六进制值，其中
		 每一位代表一个目标。将该目标的位设为 '1' 即为该
		 目标启用读流式。控制器可以省略，表示它们应保留
		 默认的读流式设置。

    Examples:

```

		rd_strm:{0x0041}

	    在控制器 0 上

		- 为目标 0 和 6 启用读流式。
		- 对目标 1-5,7-15 禁用读流式。

	    所有其他目标保留默认的读流式设置。

	    ::

		rd_strm:{0x0023,,0xFFFF}

	    在控制器 0 上

		- 为目标 1、2 和 5 启用读流式。
		- 对目标 3、4、6-15 禁用读流式。

	    在控制器 2 上

		- 为所有目标启用读流式。

	    所有其他目标保留默认的读流式设置。

    :Possible Values: 0x0000 - 0xffff
    :Default Value: 0x0000

```
dv: {value[,value...]}
    :Definition: 基于每个控制器设置 Domain Validation 策略。
                 控制器可以省略，表示它们应保留默认的读流式设置。

     :Possible Values:

		      ==== ===============================
		       < 0 使用来自串行 EEPROM 的设置。
                         0 禁用 DV
		       > 0 启用 DV
		      ==== ===============================

    :Default Value: DV 串行 EEPROM 配置设置。

    Example:

```

	    dv:{-1,0,,1,1,0}

	- 在控制器 0 上保持 DV 为默认设置。
	- 在控制器 1 上禁用 DV。
	- 跳过控制器 2 的配置。
	- 在控制器 3 和 4 上启用 DV。
	- 在控制器 5 上禁用 DV。

```
seltime:[value]
    :Definition: 指定选择超时值
    :Possible Values: 0 = 256ms，1 = 128ms，2 = 64ms，3 = 32ms
    :Default Value: 0


    以下三个选项只能在技术支持代表的指导下更改。


precomp: {value[,value...]}
    :Definition: 基于每个控制器设置 IO Cell 预补偿值。
                 控制器可以省略，表示它们应保留默认的预补偿设置。

    :Possible Values: 0 - 7
    :Default Value: 随芯片修订版本而不同

    Examples:

```

	    precomp:{0x1}

	在控制器 0 上将预补偿设为 1。

	::

	    precomp:{1,,7}

	- 在控制器 0 上将预补偿设为 1。
	- 在控制器 2 上将预补偿设为 8。

```
slewrate: {value[,value...]}
    :Definition: 基于每个控制器设置 IO Cell 压摆率。
                      控制器可以省略，表示它们应保留默认的压摆率设置。

    :Possible Values: 0 - 15
    :Default Value: 随芯片修订版本而不同

    Examples:

```

	    slewrate:{0x1}

	- 在控制器 0 上将压摆率设为 1。

	::

	    slewrate :{1,,8}

	- 在控制器 0 上将压摆率设为 1。
	- 在控制器 2 上将压摆率设为 8。

```
amplitude: {value[,value...]}
    :Definition: 基于每个控制器设置 IO Cell 信号幅度。
                 控制器可以省略，表示它们应保留默认的读流式设置。

    :Possible Values: 1 - 7
    :Default Value: 随芯片修订版本而不同

    Examples:

```

	amplitude:{0x1}

    在控制器 0 上将幅度设为 1。

    ::

	amplitude :{1,,7}

    - 在控制器 0 上将幅度设为 1。
    - 在控制器 2 上将幅度设为 7。

```
```

    options aic79xx aic79xx=verbose,rd_strm:{{0x0041}}

```
在驱动中启用详细输出，并为控制器 0 的目标 0 和 6 打开读流式。

## 4. 补充说明


### 4.1. 已知/未解决或仅供参考的问题


        - 在 SuSE Linux Enterprise 7 下，由于 Linux 内核中 PCI 中断路由的
          问题，驱动可能无法正确运行。请联系 SuSE 获取更新的 Linux 内核。

### 4.2. 第三方兼容性问题


        - Adaptec 仅支持运行最新可用固件的 Ultra320 硬盘。请与您的硬盘
          制造商确认您拥有最新版本。

### 4.3. 操作系统或技术限制


        - PCI 热插拔未经测试，可能导致操作系统停止响应。
        - 不从 0 开始连续编号的 lun 可能在系统启动期间不会被自动探测。
          这是操作系统的限制。请联系您的 Linux 发行商以获取手动探测
          非连续 lun 的说明。
        - 在 RedHat 下操作系统安装期间使用本软件包的驱动更新盘版本，可能
          导致系统模块目录中安装了本驱动的两个版本。这可能引起
          /sbin/mkinitrd 程序和/或尝试安装系统包的其他 RPM 包出现问题。
          系统运行后纠正此问题的最佳方法是安装本驱动的最新 RPM 包版本，
          可从 http://www.adaptec.com 获取。


## 5. Adaptec 客户支持


   申请 Adaptec 技术支持需要一个技术支持标识（TSID）编号。

    - 12 位的 TSID 可在产品包装盒内所附的白色条形码标签上找到。TSID 可
      通过准确识别您的产品和支持状态，帮助我们提供更高效的服务。

   支持选项
    - 在 http://ask.adaptec.com 搜索 Adaptec 支持知识库（ASK），获取有关
      您产品的文章、故障排除技巧和常见问题解答。
    - 如需通过电子邮件获得支持，请在 http://ask.adaptec.com/ 向 Adaptec 的
      技术支持专家提交您的问题。

   北美
    - 访问我们的网站 http://www.adaptec.com/。
    - 有关 Adaptec 支持选项的信息，请致电 408-957-2550，每天 24 小时，
      每周 7 天。
    - 如需与技术支持专家通话，

      - 硬件产品，请致电 408-934-7274，周一至周五，太平洋夏令时
        凌晨 3:00 至下午 5:00。
      - RAID 和光纤通道产品，请致电 321-207-2000，周一至周五，太平洋夏令时
        凌晨 3:00 至下午 5:00。

      为加快服务速度，请准备好您的计算机。
    - 订购 Adaptec 产品（包括配件和线缆），请致电 408-957-7274。在线订购
      线缆请访问 http://www.adaptec.com/buy-cables/。

   欧洲
    - 访问我们的网站 http://www.adaptec.com/en-US/_common/world_index。
    - 如需与技术支持专家通话，请致电或发电子邮件，

      - 德语： +49 89 4366 5522，周一至周五，中欧时间 9:00-17:00，
        http://ask-de.adaptec.com/。
      - 法语： +49 89 4366 5533，周一至周五，中欧时间 9:00-17:00，
	http://ask-fr.adaptec.com/。
      - 英语： +49 89 4366 5544，周一至周五，格林尼治标准时间 9:00-17:00，
	http://ask.adaptec.com/。

    - 您可以在线订购 Adaptec 线缆：
      http://www.adaptec.com/buy-cables/。

   日本
    - 访问我们的网站 http://www.adaptec.co.jp/。
    - 如需与技术支持专家通话，请致电 +81 3 5308 6120，周一至周五，
      上午 9:00 至 12:00，下午 1:00 至 6:00。

Copyright |copy| 2003 Adaptec Inc. 691 S. Milpitas Blvd., Milpitas CA 95035 USA.
All rights reserved.

You are permitted to redistribute, use and modify this README file in whole
or in part in conjunction with redistribution of software governed by the
General Public License, provided that the following conditions are met:

1. Redistributions of README file must retain the above copyright
   notice, this list of conditions, and the following disclaimer,
   without modification.
2. The name of the author may not be used to endorse or promote products
   derived from this software without specific prior written permission.
3. Modifications or new contributions must be attributed in a copyright
   notice identifying the author ("Contributor") and added below the
   original copyright notice. The copyright notice is for purposes of
   identifying contributors and should not be deemed as permission to alter
   the permissions given by Adaptec.

THIS README FILE IS PROVIDED BY ADAPTEC AND CONTRIBUTORS `AS IS` AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, ANY
WARRANTIES OF NON-INFRINGEMENT OR THE IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
ADAPTEC OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS README
FILE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
