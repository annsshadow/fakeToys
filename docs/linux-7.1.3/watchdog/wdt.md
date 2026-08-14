## WDT 看门狗定时器接口（适用于 Linux 操作系统）


Last Reviewed: 10/05/2007

Alan Cox <alan@lxorguk.ukuu.org.uk>

 - ICS	WDT501-P
 - ICS	WDT501-P (no fan tachometer)
 - ICS	WDT500-P

所有接口都提供 /dev/watchdog，打开后必须在超时时间内写入，否则机器将重启。每次写入都会将重启时间再延后一个超时周期。对于软件看门狗，能否重启取决于机器和中断的状态。硬件板卡会通过其自身的板载定时器在物理上拉低机器电源，几乎在任何情况下都会重启。

WDT501P 卡上提供第二个温度监控接口。它提供 /dev/temperature。这是机器内部温度，单位为华氏度。每次读取返回一个字节，表示温度。

第三个接口会在额外的告警事件上记录内核消息。

ICS ISA 总线 wdt 卡无法被安全地探测。相反，你需要
```

	wdt.io=0x240 wdt.irq=11

```
其他 "wdt" 驱动参数如下：

	===========	======================================================
	heartbeat	看门狗心跳间隔，单位秒（默认 60）
	nowayout	看门狗一旦启动便无法停止（内核
			编译参数）
	tachometer	WDT501-P 风扇转速计支持（0=禁用，默认=0）
	type		WDT501-P 卡类型（500 或 501，默认=500）
	===========	======================================================

### 特性


================   =======	   =======
		   WDT501P	   WDT500P
================   =======	   =======
Reboot Timer	   重启定时器	   X
External Reboot	   外部重启	   X
I/O Port Monitor   I/O 端口监控   o
Temperature	   温度           X
Fan Speed          风扇转速       X
Power Under	   欠压           X
Power Over         过压           X
Overheat           过热           X
================   =======	   =======

WDT 板卡上的外部事件接口目前不受支持。不过已为它分配了次设备号。


示例看门狗驱动：

	see samples/watchdog/watchdog-simple.c
