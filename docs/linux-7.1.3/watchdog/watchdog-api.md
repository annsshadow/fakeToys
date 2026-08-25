## Linux 鐪嬮棬鐙楅┍鍔?API


最后审阅：2007/05/10


Copyright 2002 Christer Weingel <wingel@nano-system.com>

本文档的某些部分逐字复制sbc60xxwdt 驱动，该驱动版权(c) Copyright 2000 Jakob
Oestergaard <jakob@ostenfeld.dk>

本文档描述的Linux 2.4.18 内核的状态
## 简

看门狗定时器（Watchdog Timer，WDT）是一种硬件电路，可以在出现软件故障时复位计算系统。你可能已经知道了
通常，一个用户空间守护进程会定期通过 /dev/watchdog 这个特殊的设备文件通知内核看门驱动：用户空间仍然存活。当这样的通知发生时，驱动通常会告诉硬件看门狗一切正常，看门应当再等待一小段时间来复位系统。如果用户空间发生故障（RAM 错误、内核缺陷，无论什原因），通知将停止发生，硬件看门狗会在超时发生后复位系统（导致重启）
Linux 看门API 是一个相当临时的构造，不同的驱动实现了它不同的、有时甚至不兼容部分。本文档试图记录现有的用法，并允许未来的驱动编写者将其作为参考
## 最简单的 API


所有驱动都支持基本的操作模式：一旦打开 /dev/watchdog，看门狗就被激活，除非在一定的
时间喂狗"（ping），否则看门狗将重启系统，这个时间称为超时（timeout）或余量
（margin）。喂狗最简单的方法是向设备写入一些数据。因此，一个非常简单的看门狗守护进看起来像这个源文件：参见 samples/watchdog/watchdog-simple.c

一个更高级的驱动可以在执行写调用喂狗之前，例如检查一HTTP 服务器是否仍在响应
当设备被关闭时，看门狗被禁用，除非支Magic Close"特性（见下文）。这并不总是个好主意因为如果看门狗守护进程有缺陷并崩溃，系统将不会重启。因此，某些驱动支持配置选项
"Disable watchdog shutdown on close"，即 CONFIG_WATCHDOG_NOWAYOUT。如果在编译内核将其设置Y，一旦看门狗启动就再也没有办法禁用它。因此，如果看门狗守护进程崩溃，系统
将在超时过后重启。看门狗设备通常也支nowayout 模块参数，以便可以在运行时控制该选项
## Magic Close 特

如果驱动支持 "Magic Close"，除非在关闭文件之前/dev/watchdog 发送了特定的魔术字'V'，否则驱动不会禁用看门狗。如果用户空间守护进程在没有发送这个特殊字符的情况下关了文件，驱动会假定该守护进程（以及一般意义上的用户空间）已经死亡，并将停止喂狗，而不
先禁用它。如果看门狗没有在足够的时间内被重新打开，这将导致重启
## ioctl API


所有符合规范的驱动也都支持 ioctl API
使用 ioctl 喂狗
所有具ioctl 接口的驱动至少支持一ioctl，即 KEEPALIVE。这ioctl 与向看门狗设写入的作用完全相同，因此上面程序的主循环可以```

	while (1) {
		ioctl(fd, WDIOC_KEEPALIVE, 0);
		sleep(10);
	}

```
ioctl 的参数被忽略
## 设置和获取超

对于某些驱动，可以使SETTIMEOUT ioctl 动态修改看门狗超时，这些驱动在option 字段设置WDIOF_SETTIMEOUT 标志。参数是一个整数，表示以秒为单位的超时。驱动会在同一个变中返回实际使用的超时，这个超时可能不同于
```

    int timeout = 45;
    ioctl(fd, WDIOC_SETTIMEOUT, &timeout);
    printf("The timeout was set to %d seconds\n", timeout);

```
如果设备的超时粒度是分钟，这个例子实际上可能打印"The timeout was set to 60 seconds"
Linux 2.4.18 内核开始，可以查询
```

    ioctl(fd, WDIOC_GETTIMEOUT, &timeout);
    printf("The timeout was is %d seconds\n", timeout);

```
## 预超时（Pretimeouts

某些看门狗定时器可以被设置为在它们实际复位系统的时刻之前触发。这可以通过 NMI、中断或
其他机制完成。这允许 Linux ```

    pretimeout = 10;
    ioctl(fd, WDIOC_SETPRETIMEOUT, &pretimeout);

```
之前记录有用的信息（panic 信息和内核转储）
注意，预超时是距离超时触发时刻之前的秒数。它不是距离预超时的秒数。因此，例如，如果你
将超时设置为 60 秒，预超时设置为 10 秒，预超时将50 秒后触发。将预超时设置为零会禁用
它```

    ioctl(fd, WDIOC_GETPRETIMEOUT, &timeout);
    printf("The pretimeout was is %d seconds\n", timeout);

```
并非所有看门狗驱动都支持预超时
## 获取重启前的剩余秒数


某些看门狗驱动能够报告系统重启前的剩余时间。WDIOC_GETTIMELEFT 就是ioctl
```

    ioctl(fd, WDIOC_GETTIMELEFT, &timeleft);
    printf("The timeout was is %d seconds\n", timeleft);

```
## 环境监控


所有看门狗驱动都需要返回关于系统的更多信息，有些做温度、风扇和电源电平监控，有些可告诉你系统上次重启的原因。GETSUPPORT ioctl ```

	struct watchdog_info ident;
	ioctl(fd, WDIOC_GETSUPPORT, &ident);

```
ident 结构返回的字段为
	================	=============================================
        identity		标识看门狗驱动的一个字符串
	firmware_version	如果可用，则是卡的固件版	options			描述设备支持什么的标志
	================	=============================================

options 字段可以设置以下位，并描GET_STATUS GET_BOOT_STATUS ioctl 可以返回哪种
信息
	================	=========================
	WDIOF_OVERHEAT		由于 CPU 过热而复	================	=========================

机器上次是因为超过了温度限制而被看门狗重启的
	==============		==========
	WDIOF_FANFAULT		风扇故障
	==============		==========

由看门狗卡监控的系统风扇发生了故
	=============		================
	WDIOF_EXTERN1		澶栭儴缁х數鍣?1
	=============		================

外部监控继电1 被触发。用于实际应用的控制器包含外部监控引脚，会触发复位
	=============		================
	WDIOF_EXTERN2		澶栭儴缁х數鍣?2
	=============		================

外部监控继电2 被触
	================	=====================
	WDIOF_POWERUNDER	电源不良/电源故障
	================	=====================

机器显示欠压状
	===============		=============================
	WDIOF_CARDRESET		卡之前复位过 CPU
	===============		=============================

上次重启是由看门狗卡引起
	================	=====================
	WDIOF_POWEROVER		电源过压
	================	=====================

机器显示过压状态。注意，如果一个是欠压一个是过压，两个位都会被设置——这可能看起来奇怪，
但其实是合理的
	===================	=====================
	WDIOF_KEEPALIVEPING	保活 ping 应答
	===================	=====================

看门狗自上次被查询以来看到过一次保ping
	================	=======================
	WDIOF_SETTIMEOUT	可以设置/获取超时
	================	=======================

看门狗可以做预超时
	================	================================
	WDIOF_PRETIMEOUT	预超时（以秒为单位），获设置
	================	================================


对于那些option 字段中返回任何置位位的驱动，可以使用 GETSTATUS GETBOOTSTATUS ioctl
来请求当前的
```

    int flags;
    ioctl(fd, WDIOC_GETSTATUS, &flags);

    or

    ioctl(fd, WDIOC_GETBOOTSTATUS, &flags);

```
注意，并非所有设备都支持这两个调用，有些只支GETBOOTSTATUS 调用
某些驱动可以使用 GETTEMP ioctl 测量温度```

    int temperature;
    ioctl(fd, WDIOC_GETTEMP, &temperature);

```
最后，SETOPTIONS ioctl 可以用来控制
```

    int options = 0;
    ioctl(fd, WDIOC_SETOPTIONS, &options);

```
的某些方面
以下选项可用
	=================	================================
	WDIOS_DISABLECARD	关闭看门狗定时器
	WDIOS_ENABLECARD	打开看门狗定时器
	WDIOS_TEMPPANIC		温度越限时内panic
	=================	================================

[FIXME -- 更好的解释]
