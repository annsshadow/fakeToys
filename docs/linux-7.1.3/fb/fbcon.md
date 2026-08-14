## 帧缓冲控制台（Framebuffer Console）


帧缓冲控制台（fbcon），顾名思义，是运行在帧缓冲设备之上的文本控制台。它具有任何标准文本控制台驱动程序（如 VGA 控制台）的功能，并额外具备一些可归因于帧缓冲图形特性的功能。

在 x86 架构中，帧缓冲控制台是可选的，甚至有人将其视为一种玩具。对于其他架构，它是唯一可用的显示设备，无论是文本还是图形。

fbcon 有哪些特性？帧缓冲控制台支持高分辨率、多种字体类型、显示旋转、简单的多显（multihead）等。理论上，由底层显卡提供的多彩字体、混合、抗锯齿以及任何特性也都是可能的。


## A. 配置


可以通过你喜欢的任意内核配置工具来启用帧缓冲控制台。它位于 Device Drivers -> Graphics Support -> Console display driver support -> Framebuffer Console Support 之下。选择 'y' 将其静态编译进内核，或选择 'm' 作为模块支持。模块名将为 fbcon。

为了让 fbcon 激活，至少需要一个帧缓冲驱动程序，因此可从众多可用驱动程序中任选其一。对于 x86 系统，它们几乎都拥有 VGA 卡，因此 vga16fb 和 vesafb 始终可用。不过，使用针对特定芯片组的驱动程序会给你更高的速度和更多特性，例如动态更改视频模式的能力。

要显示企鹅 logo，请在 Graphics support -> Bootup logo 下选择任意可用的 logo。

此外，你还需要选择至少一个编译进内核的字体，但如果你不做任何操作，内核配置工具会为你选择一个，通常是一个 8x16 字体。


   一个常见的错误报告是启用了帧缓冲却没有启用帧缓冲控制台。根据驱动程序的不同，你可能会得到黑屏或乱码显示，但系统仍然会启动完成。如果你幸运地拥有不会修改图形芯片的驱动程序，那么你仍然会得到一个 VGA 控制台。


## B. 加载


可能的场景：

1. 驱动程序和 fbcon 都被静态编译

	 通常，fbcon 会自动接管你的控制台。显著的例外是 vesafb。它需要通过 vga= 启动选项参数显式激活。

2. 驱动程序被静态编译，fbcon 被编译为模块

	 根据驱动程序的不同，你要么得到一个标准控制台，要么得到如上所述的乱码显示。要获得帧缓冲控制台，执行 'modprobe fbcon'。

3. 驱动程序被编译为模块，fbcon 被静态编译

	 你会得到标准控制台。一旦通过 'modprobe xxxfb' 加载驱动程序，fbcon 会自动接管控制台，可能的例外是使用 fbcon=map:n 选项。见下文。

4. 驱动程序和 fbcon 都被编译为模块。

	 你可以以任意顺序加载它们。一旦两者都被加载，fbcon 将接管控制台。


## C. 启动选项


	 帧缓冲控制台有几个鲜为人知的启动选项，可以改变其行为。

1. fbcon=font:<name>

	 选择要使用的初始字体。'name' 的值可以是任意编译进内核的字体：10x18、6x10、6x8、7x14、Acorn8x8、MINI4x6、PEARL8x8、ProFont6x11、SUN12x22、SUN8x16、TER16x32、VGA8x16、VGA8x8。

	 注意，并非所有驱动程序都能处理宽度不能被 8 整除的字体，例如 vga16fb。


2. fbcon=map:<0123>

	 这是一个有趣的选项。它告诉哪个驱动程序被映射到哪个控制台。'0123' 这个值是重复出现的序列，直到总长度达到 64（即可用的控制台数量）。在上述示例中，它被扩展为 012301230123...，映射关系如下
```

		tty | 1 2 3 4 5 6 7 8 9 ...
		fb  | 0 1 2 3 0 1 2 3 0 ...

		（'cat /proc/fb' 会告诉你 fb 的编号是什么）

```
	 一个可能有用的副作用是使用超过已加载 fb 驱动程序数量的映射值。例如，如果只有一个驱动程序可用（fb0），添加 fbcon=map:1 会告知 fbcon 不要接管控制台。

	 之后，当你想将控制台映射到帧缓冲设备时，可以使用 con2fbmap 工具。

```
3. fbcon=vc:<n1>-<n2>

	 此选项告诉 fbcon 只接管由 'n1' 和 'n2' 值指定的控制台范围。给定范围之外的其余控制台仍将由标准控制台驱动程序控制。

```
	   对于 x86 机器，标准控制台是 VGA 控制台，通常位于同一块显卡上。因此，由 VGA 控制台控制的那些控制台将显示为乱码。

```
4. fbcon=rotate:<n>

	 此选项更改控制台显示的朝向角度。'n' 的值接受以下选项：

     - 0 - 正常朝向（0 度）
     - 1 - 顺时针朝向（90 度）
     - 2 - 倒置朝向（180 度）
     - 3 - 逆时针朝向（270 度）

	 之后可以随时通过向 /sys/class/graphics/fbcon 下两个属性中的任意一个"echo"相同的数字来更改角度：

  - rotate     - 旋转活动控制台的显示
  - rotate_all - 旋转所有控制台的显示

	 只有在内核中编译进帧缓冲控制台旋转（Framebuffer Console Rotation）支持时，控制台旋转才会变得可用。

```
	   这纯粹是控制台旋转。任何其他使用帧缓冲的应用程序将保持"正常"朝向。实际上，底层的 fb 驱动程序完全不知道控制台旋转这回事。

```
5. fbcon=margin:<color>

	 此选项指定页边距的颜色。页边距是屏幕右侧和底部未被文本使用的剩余区域。默认情况下，该区域为黑色。'color' 值是一个整数，取决于所使用的帧缓冲驱动程序。

6. fbcon=nodefer

	 如果内核编译时开启了延迟 fbcon 接管（deferred fbcon takeover）支持，通常固件/引导加载程序留下的帧缓冲内容会被保留，直到实际上有文本输出到控制台。此选项使 fbcon 立即绑定到 fbdev 设备。

7. fbcon=logo-pos:<location>

	 唯一可能的 'location' 是 'center'（不带引号），指定后，启动 logo 会从默认的左上角位置移动到帧缓冲的中心。如果由于多个 CPU 而显示多个 logo，则收集起来的一排 logo 会作为一个整体移动。

8. fbcon=logo-count:<n>

	 'n' 的值会覆盖启动 logo 的数量。0 禁用 logo，-1 给出默认值，即在线 CPU 的数量。


## D. 附加、分离与卸载


在介绍如何附加、分离和卸载帧缓冲控制台之前，先了解一下它们的依赖关系可能会有帮助。

控制台层与大多数子系统一样，需要一个与硬件接口的驱动程序
```

	console ---> VGA driver ---> hardware.

```
假设 VGA 驱动程序可以被卸载，那么在卸载驱动程序之前，必须先将 VGA 驱动程序从控制台层解绑。如果 VGA 驱动程序仍然绑定到控制台层，则无法卸载它。（更多信息请参阅 Documentation/driver-api/console.rst）。

对于帧缓冲控制台（fbcon）来说，情况更为复杂
```

	console ---> fbcon ---> fbdev drivers ---> hardware

```
如果绑定到 fbcon，则 fbdev 驱动程序无法卸载；而如果 fbcon 绑定到控制台层，则 fbcon 无法卸载。

因此，要卸载 fbdev 驱动程序，必须先将 fbcon 从控制台解绑，然后再将 fbdev 驱动程序从 fbcon 解绑。幸运的是，将 fbcon 从控制台层解绑会自动将帧缓冲驱动程序从 fbcon 解绑。因此，无需显式地将 fbdev 驱动程序从 fbcon 解绑。

那么，我们如何将 fbcon 从控制台解绑？部分答案在 Documentation/driver-api/console.rst 中。总结如下：

通过向代表帧缓冲控制台的 bind 文件 echo 一个值
```

  echo 1 > /sys/class/vtconsole/vtcon1/bind - 将帧缓冲控制台附加到
					     控制台层
  echo 0 > /sys/class/vtconsole/vtcon1/bind - 将帧缓冲控制台从
					     控制台层分离

```
如果 fbcon 从控制台层分离，你的启动控制台驱动程序（通常是 VGA 文本模式）将接管。少数驱动程序（rivafb 和 i810fb）会为你恢复 VGA 文本模式。对于其余驱动程序，在分离 fbcon 之前，你必须采取一些额外步骤以确保 VGA 文本模式被正确恢复。以下是你可以采用的几种方法之一：

1. 下载或安装 vbetool。这个工具现在已包含在大多数发行版中，通常是挂起/恢复工具的一部分。

2. 在内核配置中，确保 CONFIG_FRAMEBUFFER_CONSOLE 设置为 'y' 或 'm'。启用一个或多个你喜欢的帧缓冲驱动程序。

```

	vbetool vbestate save > <vga state file>

   上述命令将图形硬件的寄存器内容保存到 <vga state file>。你只需执行一次此步骤，因为状态文件可以重复使用。

```
```

       modprobe fbcon

```
```

       vbetool vbestate restore < <vga state file> && \
       echo 0 > /sys/class/vtconsole/vtcon1/bind

```
6. 就是这样，你回到了 VGA 模式。如果你将 fbcon 编译为模块，可以通过 'rmmod fbcon' 卸载它。

```

       echo 1 > /sys/class/vtconsole/vtcon1/bind

```
8. 一旦 fbcon 被解绑，所有注册到系统的驱动程序也将被解绑。这意味着 fbcon 和各个帧缓冲驱动程序可以随意卸载或重新加载。重新加载驱动程序或 fbcon 会自动将控制台、fbcon 和驱动程序绑定在一起。在不卸载 fbcon 的情况下卸载所有驱动程序，将导致控制台无法绑定 fbcon。


## vesafb 用户须知：


遗憾的是，如果你的启动命令行包含将硬件设置为图形模式的 vga=xxx 参数（例如加载 vesafb 时），vgacon 将不会加载。相反，vgacon 会用 dummycon 替换默认启动控制台，并且在分离 fbcon 之后你将得不到任何显示。你的机器仍然处于存活状态，因此可以重新附加 vesafb。不过，要重新附加 vesafb，你需要执行以下操作之一：

变体 1：

```

	vbetool vbemode save > <vesa state file> # 对每个 vesafb 模式执行一次，
						 # 文件可重复使用

    b. 按第 5 步分离 fbcon。

    c. 附加 fbcon::

	vbetool vbestate restore < <vesa state file> && \
	echo 1 > /sys/class/vtconsole/vtcon1/bind

```
变体 2：

```

	echo <ID> > /sys/class/tty/console/bind

	vbetool vbemode get

    b. 记下模式编号

    b. 按第 5 步分离 fbcon。

    c. 附加 fbcon::

	vbetool vbemode set <mode number> && \
	echo 1 > /sys/class/vtconsole/vtcon1/bind

```
## 示例：


以下是 2 个示例 bash 脚本，可用于绑定或解绑
```

  #!/bin/bash
  # Unbind fbcon

  # 将此更改为你的实际 vgastate 文件所在位置
  # 或者使用 VGASTATE=$1 在运行时指明状态文件
  VGASTATE=/tmp/vgastate

  # vbetool 路径
  VBETOOL=/usr/local/bin


  for (( i = 0; i < 16; i++))
  do
    if test -x /sys/class/vtconsole/vtcon$i; then
	if [ `cat /sys/class/vtconsole/vtcon$i/name | grep -c "frame buffer"` \
	     = 1 ]; then
	    if test -x $VBETOOL/vbetool; then
	       echo Unbinding vtcon$i
	       $VBETOOL/vbetool vbestate restore < $VGASTATE
	       echo 0 > /sys/class/vtconsole/vtcon$i/bind
	    fi
	fi
    fi
  done

```
---------------------------------------------------------------------------

```

  #!/bin/bash
  # Bind fbcon

  for (( i = 0; i < 16; i++))
  do
    if test -x /sys/class/vtconsole/vtcon$i; then
	if [ `cat /sys/class/vtconsole/vtcon$i/name | grep -c "frame buffer"` \
	     = 1 ]; then
	  echo Unbinding vtcon$i
	  echo 1 > /sys/class/vtconsole/vtcon$i/bind
	fi
    fi
  done

Antonino Daplas <adaplas@pol.net>
