## Berkshire Products PC 看门狗卡


最后审阅：2007/10/05

## 对 ISA 卡 A 版与 C 版的支持


文档与驱动由 Ken Hollis <kenji@bitgate.com> 提供

 PC 看门狗是一张提供与 WDT 卡同类功能的卡，只是它不需要 IRQ 即可运行。
 此外，C 版卡允许你监控任何 IO 端口，以自动触发该卡被重置。这样你可以让
 该卡监控硬盘状态，或任何你需要监控的其它东西。

 看门狗驱动有一个基本职责：与卡通信并向其发送信号，使其不会重置你的
 计算机……至少在正常操作期间如此。

 看门狗驱动会自动找到你的看门狗卡，并会挂载一个运行中的驱动供该卡使用。
 在看门狗驱动初始化之后，你就可以使用一个 PC 看门狗程序与该卡通信。

 我建议在 fsck 开始之前放一个 “watchdog -d”，并在 fsck 结束之后立即放一个
 “watchdog -e -t 1”。（记得用 “&” 运行该程序以使其在后台运行！）

 如果你想编写一个与 PC 看门狗驱动兼容的程序，只需使用或修改看门狗测试
 程序：
 tools/testing/selftests/watchdog/watchdog-test.c


 其它 IOCTL 函数包括：

	WDIOC_GETSUPPORT
		返回卡自身的支持信息。这通过结构 “PCWDS” 返回：

			options = WDIOS_TEMPPANIC
				  （该卡支持温度）
			firmware_version = xxxx
				  （卡的固件版本）

	WDIOC_GETSTATUS
		返回卡的状态，将 WDIOF_* 的位按位与到该值中。（注释见
		include/uapi/linux/watchdog.h）

	WDIOC_GETBOOTSTATUS
		返回在启动时报告的卡状态。

	WDIOC_GETTEMP
		返回卡的温度。（你也可以读取 /dev/watchdog，它会每秒
		给出一次温度更新。）

	WDIOC_SETOPTIONS
		这让你设置卡的选项。你可以通过这种方式启用或禁用该卡。

	WDIOC_KEEPALIVE
		这会 ping 该卡，告诉它不要重置你的计算机。

 就这么多了！

 -- Ken Hollis
    (kenji@bitgate.com)
