## 内核驱动 abituguru


支持的芯片：

  - Abit uGuru 1 和第 2 版（仅硬件监控部分）

    Prefix: 'abituguru'

    Addresses scanned: ISA 0x0E0

    Datasheet: 不可用，此驱动基于逆向工程
    已根据逆向工程编写了一份“Datasheet”，它应与此文件位于同一目录下，
    名称abituguru-datasheet

    Note:
	uGuru 是一个带有板载固件的微控制器，被编程为表现得像一hwmon IC
	有许多不同版本的固件，因此实际上有许多不同版本的 uGuru
	下面是一份不完整的列表，说明哪些版本用于哪些主板

 - uGuru 1.00    ~ 1.24    (AI7, KV8-MAX3, AN7) [^1^]_
 - uGuru 2.0.0.0 ~ 2.0.4.2 (KV8-PRO)
 - uGuru 2.1.0.0 ~ 2.1.2.8 (AS8, AV8, AA8, AG8, AA8XE, AX8)
 - uGuru 2.2.0.0 ~ 2.2.0.6 (AA8 Fatal1ty)
 - uGuru 2.3.0.0 ~ 2.3.0.9 (AN8)
 - uGuru 3.0.0.0 ~ 3.0.x.x (AW8, AL8, AT8, NI8 SLI, AT8 32X, AN8 32X,
	  AW9D-MAX) [^2^]_

	bank1 传感器的 sensortype（电压或温度），对于1 uGuru
	这并不总是有效。对于这uGuru，可以通过 bank1_types 模块参数覆盖自动检测
	对于所3 种已知的1 版主板，该参数的正确使用方式是：
	bank1_types=1,1,0,0,0,0,0,2,0,0,0,0,2,0,0,1
	你可能还需要为这些主板指定 fan_sensors 选项
	fan_sensors=5

	abituguru（不3 ！）驱动将无法在这些主板上工作（反之亦然）！

作者：
 - Hans de Goede <j.w.r.degoede@hhs.nl>,
 - （初始逆向工程Olle Sandberg
	  <ollebull@gmail.com> 完成

### 模块参数


- force: bool
			强制检测。注意此参数仅导致跳过检测，
			从而使 insmod 成功。如果无法读uGuru
			实际hwmon 驱动将不会加载，因此不会注册任何 hwmon 设备
- bank1_types: int[]
			Bank1 传感器类型自动检测覆盖：

     - -1 自动检测（默认
     - 0 电压传感
     - 1 温度传感
     - 2 未连
- fan_sensors: int
			告诉驱动你的主板上有多少个风扇转速传感器
			默认（自动检测）
- pwms: int
			告诉驱动你的主板有多少个风扇转速控制（fan
			pwms）。默认：0（自动检测）
- verbose: int
			驱动应多详细地输出？-3）：

      - 0 正常输出
      - 1 + 详细错误报告
      - 2 + 传感器类型探测信息（默认
      - 3 + 可重试错误报

			默认（驱动仍处于测试阶段

注意：如果你需要上述前三个选项中的任何一个，请用 verbose 设为 3 的方
insmod 该驱动，并将以下命令的输出通过邮件发给<j.w.r.degoede@hhs.nl>
dmesg | grep abituguru


### 描述


本驱动支持位于带Abit uGuru 的主板（大多数现Abit 主板）上Abit uGuru 芯片1 和第 2 版的硬件监控特性

1 和第 2 uGuru 芯片实际上是一个伪装的 Winbond W83L950D（尽Abit 声称它是“由 ABIT 工程师设计的全新微处理器”）。不幸的是这并无帮助，因W83L950D 是一个带有运行在其上的定Abit 应用程序的通用微控制器

尽管 Abit 没有发布任何关于 uGuru 的信息，Olle Sandberg <ollebull@gmail.com> 仍然成功地对 uGuru 的传感器部分进行了逆向工程。没有他的工作，这个驱动就不可能实现

### 已知问题


Abit uGuru 的电压和频率控制部分不受支持

- [abituguru-datasheet.rst](abituguru-datasheet.rst)
