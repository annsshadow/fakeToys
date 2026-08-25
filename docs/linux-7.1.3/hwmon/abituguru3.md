## 内核驱动 abituguru3


支持芯片  - Abit uGuru revision 3（硬件监控部分，仅读取）

    Prefix: 'abituguru3'

    Addresses scanned: ISA 0x0E0

    Datasheet: 不可用，该驱动基于逆向工程
    Note:
	The uGuru 是一个带板载固件的微控制器，固件将其编程为表现得像一	hwmon IC。固件有许多不同的版本，因此实际上也有许多不同版本的 uGuru	以下是各主板使用哪些版本的不完整列表
 - uGuru 1.00    ~ 1.24    (AI7, KV8-MAX3, AN7)
 - uGuru 2.0.0.0 ~ 2.0.4.2 (KV8-PRO)
 - uGuru 2.1.0.0 ~ 2.1.2.8 (AS8, AV8, AA8, AG8, AA8XE, AX8)
 - uGuru 2.3.0.0 ~ 2.3.0.9 (AN8)
 - uGuru 3.0.0.0 ~ 3.0.x.x (AW8, AL8, AT8, NI8 SLI, AT8 32X, AN8 32X,
	  AW9D-MAX)

	abituguru3 驱动仅适用3.0.x.x 版本的主板，该驱动在较旧的主板上
	无法工作。对于较旧的主板，请使用 abituguru（不3！）驱动
Authors:
 - Hans de Goede <j.w.r.degoede@hhs.nl>,
 - （由 Louis Kruger 完成初始逆向工程
### 模块参数


- force: bool
			强制检测。注意该参数只会导致跳过检测，从			insmod 成功。如果无法读uGuru，实际的 hwmon
			驱动将不会加载，因此不会hwmon 设备被注册- verbose: bool
			驱动是否应当输出详细信息
   - 0/off/false  正常输出
   - 1/on/true    + 详细错误报告（默认）

			默认（驱动仍处于测试阶段
### 描述


该驱动支持最近带Abit uGuru 的主板上所使用的第三代 Abit uGuru 芯片的硬件监控特性
uGuru 芯片的第三版实际上是一Winbond W83L951G。遗憾的是这并无帮助，因W83L951G
是一个运行着定制 Abit 应用的通用微控制器
尽管 Abit 没有发布任何关于 uGuru 第三版的信息，Louis Kruger 还是成功逆向工程uGuru
的传感器部分。没有他的工作，该驱动就不可能实现
### 已知问题


Abit uGuru 的电压和频率控制部分不受支持，写入任何传感器设置以及写入/读取风扇转速控寄存器（FanEQ）也不受支持
如果遇到任何问题，请发邮件给<j.w.r.degoede@hhs.nl> 并附上以下命令的输出`dmesg | grep abituguru`
