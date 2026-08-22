## Linux 串行控制

要将某个串口用作控制台，你需要把相关支持编译进内核——默认情况下并未编译进去。对PC 风格的串口，对应的配置选项位于如下菜单项旁边：

`Character devices --> Serial drivers --> 8250/16550 and compatible serial support --> Console on 8250/16550 and compatible serial port`

你必须把串口支持编译进内核，而不能编译为模块
可以指定多个设备用于控制台输出。你可以定义一个新的内核命令行选项来选择将哪些设备用于控制台输出
```
	console=device,options

	device:		tty0 表示前台虚拟控制			ttyX 表示任意其他虚拟控制			ttySx 表示某个串口
			lp0 表示第一个并行端			ttyUSB0 表示第一USB 串行设备

	options:	取决于驱动。对于串口而言，它
			定义了端口的波特校验数据流控			格式BBBBPNF，其BBBB 是速率			P 是校验位（n/o/e），N 是数据位数量			F 是流控（'r' 表示 RTS）。默认为
			9600n8。最大波特率115200```

你可以在内核命令行上指定多个 console= 选项
当每种设备类型仅被提及一次时，行为是明确定义的。此时输出将出现在所有请求的控制台上。而当你打开 `/dev/console` 时，将使用最后一个设备
```
	console=ttyS1,9600 console=tty0
```

表示打开 `/dev/console` 将得到当前的当前前台虚拟控制台，内核消息将同时出现在 VGA 控制台和第二个串口（ttyS1 COM2）上，速率9600 波特
当同一设备类型被定义多次时，行为会更复杂。此时遵循以下两条规则：

1. 输出仅出现在每种已定义类型的第一个设备上
2. `/dev/console` 将与第一个注册的设备关联。注册顺序取决于内核初始化各子系统的次序
   这条规则同样适用于最后一console= 参数因其他原因未被使用的情况。例如因为拼写错误，或者因为硬件不可用
结果可能会令人意外。例如下面两条命
```
	console=ttyS1,9600 console=tty0 console=tty1
	console=tty0 console=ttyS1,9600 console=tty1
```

内核消息只打印到 `tty0` `ttyS1`。`/dev/console` `tty0` 关联。这是因为内核会先于串行控制台尝试注册图形控制台。之所以如此，是因为在没有指定控制台设备时的默认行为，详见下文
注意最后一`console=tty1` 参数仍然有影响。内核命令行也被 systemd 使用。它会使用最后定义的 `tty1` 作为登录控制台
如果没有指定控制台设备，则使用找到的第一个能够充当系统控制台的设备。此时系统首先查VGA 显卡，然后查找串口。因此如果你的系统中没有 VGA 显卡，第一个串口将自动成为控制台，除非内核配置CONFIG_NULL_TTY_DEFAULT_CONSOLE 选项，此时它会默认使ttynull 设备
你需要创建一个新的设备来使用 `/dev/console`。正式的 `/dev/console` 现在是字符设5,1
（你也可以将网络设备用作控制台。相关信息参`Documentation/networking/netconsole.rst`。）

下面是一个以 `/dev/ttyS1`（COM2）作为控制台的示例。请按需替换其中的示例值
1. 创建 `/dev/console`（真实控制台）和 `/dev/tty0`（主虚拟控制台）
```
     cd /dev
     rm -f console tty0
     mknod -m 622 console c 5 1
     mknod -m 622 tty0 c 4 0
```

2. LILO 也可以从串行设备获取输入。这是一个非常有用的选项。要LILO 使用串口
```
     serial  = 1,9600n8 (ttyS1, 9600 bd, no parity, 8 bits)
```

3. 为新的内核调整内核标志：

```
     append = "console=ttyS1,9600"
```

4. 确保有一getty 在该串口上运行，以便系统启动完成后你能登录。这可以通过添加一行来实现
```
     S1:23:respawn:/sbin/getty -L ttyS1 9600 vt100
```

5. Init 鍜?`/etc/ioctl.save`

   Sysvinit 会将stty 设置保存在一个位`/etc` 的文件中，名`/etc/ioctl.save`。在首次使用串行控制台之前，请删除此文件，否init 很可能会将波特率设为 38400（虚拟控制台的波特率）
6. `/dev/console` 涓?X

   想要对虚拟控制台做某些操作的应用程序通常会打开 `/dev/console`。如果你创建了全新的 `/dev/console` 设备，而你的控制台并非虚拟控制台，那么某些程序会失败。这些程序是想要访问 VT 接口、并使用
```
     Xfree86, svgalib, gpm, SVGATextMode
```

的程序。不过在它们的现代版本中应当已经修复了这个问题
   注意，如果你在没`console=` 选项的情况下启动（或带有 `console=/dev/tty0`），那么 `/dev/console` `/dev/tty0` 相同。这种情况下一切仍将正常工作
7. 致谢

   感谢 Geert Uytterhoeven <geert@linux-m68k.org> 将补丁从 2.1.4x 移植2.1.6x，并负责将这些补丁整合进 m68k、ppc alpha
Miquel van Smoorenburg <miquels@cistron.nl>锛?000-06-11
