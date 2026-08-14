## 内核驱动 smsc47m1


支持的芯片：

  - SMSC LPC47B27x, LPC47M112, LPC47M10x, LPC47M13x, LPC47M14x,

    LPC47M15x 与 LPC47M192

    扫描地址：无，地址从 Super I/O 配置空间读取

    Prefix: 'smsc47m1'

    Datasheets:

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47b272.pdf

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47m10x.pdf

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47m112.pdf

	http://www.smsc.com/

  - SMSC LPC47M292

    扫描地址：无，地址从 Super I/O 配置空间读取

    Prefix: 'smsc47m2'

    Datasheet: 不公开

  - SMSC LPC47M997

    扫描地址：无，地址从 Super I/O 配置空间读取

    Prefix: 'smsc47m1'

    Datasheet: 无


Authors:

     - Mark D. Studebaker <mdsxyz123@yahoo.com>,
     - 在 Bruce Allen <ballen@uwm.edu> 及其 fan.c 程序的协助下：

       - http://www.lsc-group.phys.uwm.edu/%7Eballen/driver/

     - Gabriele Gorla <gorlik@yahoo.com>,
     - Jean Delvare <jdelvare@suse.de>

### 描述


标准微系统公司（SMSC）的 47M1xx Super I/O 芯片包含用于两个风扇的监控与 PWM 控制
电路。

LPC47M15x、LPC47M192 与 LPC47M292 芯片除了风扇监控与控制外，还包含一个完整的
“硬件监控块”。该硬件监控块不受本驱动支持，对此请使用 smsc47m192 驱动。

没有 47M997 的文档可用，但它与 47M15x 和 47M192 芯片具有相同的设备 ID，并且似乎
兼容。

风扇转速以 RPM（每分钟转数）报告。如果转速降到可编程限制以下，会触发告警。风扇
读数可以被一个可编程的分频器（1、2、4 或 8）除，以给予读数更大的范围或精度。并非
所有 RPM 值都能精确表示，因此会进行一些舍入。使用分频器 2 时，可表示的最低值约为
2600 RPM。

PWM 值范围为 0 到 255。

如果告警触发，它将一直保持触发状态，直到硬件寄存器至少被读取一次。这意味着告警的
原因可能已经消失了！注意，在当前实现中，只要读取任何数据，就会读取所有硬件寄存器
（除非距离上次更新不到 1.5 秒）。这意味着你可能会轻易错过仅触发一次的告警。

------------------------------------------------------------------

lm_sensors 项目衷心感谢 Intel 在本驱动开发过程中提供的支持。
