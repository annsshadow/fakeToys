## udlfb - DisplayLink USB 2.0 驱动


这是用于 DisplayLink USB 2.0 时代图形芯片的驱动。

DisplayLink 芯片提供简单的 hline/blit 操作，并带有一些压缩，将其与 USB 线另一端的
硬件帧缓冲（16MB）配对。该硬件帧缓冲能够驱动 VGA、DVI 或 HDMI 显示器，在像素需要
改变之前无需 CPU 参与。

CPU 或其它本地资源完成所有渲染；可选地，将结果与远程硬件帧缓冲的本地影子进行比较，
以识别已更改的最小像素集合；然后通过 USB 批量传输逐行压缩并发送这些像素。

由于批量传输的效率以及其上无需任何确认（ack）的协议——其效果是极低的延迟，能够
支持令人惊讶的高分辨率，对于非游戏和非视频应用具有良好的性能。

模式设置、EDID 读取等是其它批量或控制传输。模式设置非常灵活——能够从任何时序设置
几乎任意的模式。

USB 图形的一般优点：

 - 能够向任何支持 USB 2.0 的系统添加近乎任意数量的显示器。在 Linux 上，显示器的
   数量受 fbdev 接口限制（FB_MAX 目前为 32）。当然，同一主机控制器上的所有 USB 设备
   共享同一个 480Mbs 的 USB 2.0 接口。

使用内核帧缓冲接口支持 DisplayLink 芯片的优点：

 - DisplayLink 芯片的实际硬件功能与 fbdev 接口几乎一一对应，使得该驱动相对于其
   提供的功能而言相当精简紧凑。
 - X 服务器和其它应用程序可以从用户模式使用标准 fbdev 接口与设备通信，而完全不需要
   了解任何关于 USB 或 DisplayLink 协议的知识。一个 “displaylink” X 驱动和一个稍作
   修改的 “fbdev” X 驱动就是已经这样做了的例子。

缺点：

 - Fbdev 的 mmap 接口假设映射了一个真实的硬件帧缓冲。在 USB 图形情况下，它只是一个
   分配的（虚拟）缓冲区。写入需要被检测，并由 CPU 编码为 USB 批量传输。准确的
   损坏/更改区域通知可以绕过这个问题。未来，希望 fbdev 能通过一个小的标准接口得到
   增强，以允许 mmap 客户端报告损坏，从而造福虚拟或远程帧缓冲。
 - Fbdev 不能很好地仲裁客户端对帧缓冲的拥有权。
 - Fbcon 假设它找到的第一个帧缓冲应当被控制台消费。
 - 鉴于 KMS/DRM 的兴起，fbdev 的未来尚不清晰。

## 如何使用？


Udlfb 在作为模块加载时，将匹配所有 USB 2.0 代的 DisplayLink 芯片（Alex 和 Ollie
系列）。然后它将尝试读取显示器的 EDID，并在 DisplayLink 设备与显示器能力之间设置
最佳的通用模式。

如果 DisplayLink 设备成功，它将绘制一个“绿屏”，这意味着从硬件和 fbdev 软件的角度
来看，一切都很好。

届时，将出现一个 /dev/fb? 接口，供用户模式应用程序打开并开始使用标准 fbdev 调用
写入 DisplayLink 设备的帧缓冲。注意，如果使用 mmap()，默认情况下用户模式应用程序
必须发送损坏通知以触发更改区域的重绘。或者，udlfb 可以重新编译并启用实验性的 defio
支持，以支持基于缺页的检测机制，从而无需显式通知即可工作。

udlfb 最常见的客户端是 xf86-video-displaylink 或一个修改过的 xf86-video-fbdev X
服务器。这些服务器没有真正的 DisplayLink 特定代码。它们写入标准帧缓冲接口，并依赖
udlfb 来完成它的工作。它们拥有的一个额外特性是能够将来自 X DAMAGE 协议扩展的矩形
通过 udlfb 的损坏接口向下报告给 udlfb（希望能够对所有需要损坏信息的虚拟帧缓冲
标准化）。这些损坏通知使 udlfb 能够高效地处理更改的像素。

## 模块选项


udlfb 通常不需要特殊配置。不过有几个选项。

```

  modprobe udlfb fb_defio=0 console=1 shadow=1

```
或者通过在运行时编辑来更改选项
```

  cd /sys/module/udlfb/parameters
  ls # to see a list of parameter names
  sudo nano PARAMETER_NAME
  # change the parameter in place, and save the file.

```
拔下/重新插上 USB 设备以应用新设置。

或者要永久应用选项，创建一个 modprobe 配置文件
```

  options udlfb fb_defio=0 console=1 shadow=1

```
接受的布尔选项：

=============== ================================================================
fb_defio	利用 fb_defio（CONFIG_FB_DEFERRED_IO）内核模块，通过缺页来
		跟踪帧缓冲中被更改的区域。使用 mmap 但不报告损坏的标准
		fbdev 应用程序，应当能够在此启用时工作。当运行支持通过
		ioctl 报告更改区域的 X 服务器时禁用，因为该方法更简单、
		更稳定且性能更高。
		默认：fb_defio=1

console		允许 fbcon 附加到 udlfb 提供的帧缓冲。如果 fbcon 和其它
		客户端（例如带有 --shared-vt 的 X）发生冲突，可以禁用。
		默认：console=1

shadow		分配第二个帧缓冲以影子当前通过 USB 总线在设备内存中的内容。
		如果任何像素未更改，则不传输。消耗主机内存以节省 USB 传输。
		默认启用。仅在极低内存系统上禁用。
		默认：shadow=1
=============== ================================================================

## Sysfs 属性


Udlfb 在 /sys/class/graphics/fb? 中创建多个文件
其中 ? 是该特定 DisplayLink 设备的顺序帧缓冲 id

======================== ========================================================
edid			 如果向此文件写入一个有效的 EDID blob（通常由 udev 规则），
			 那么 udlfb 将使用此 EDID 作为备份，以防读取连接到
			 DisplayLink 设备的显示器的实际 EDID 失败。这对于
			 无法通过 EDID 传达其能力的固定面板等尤其有用。读取
			 此文件会返回所连接显示器的当前 EDID（或最后写入的
			 备份值）。这可用于获取所连接显示器的 EDID，该 EDID
			 可以传递给 parse-edid 之类的工具。

metrics_bytes_rendered	 已渲染像素字节数的 32 位计数

metrics_bytes_identical  上述字节中有多少被发现未更改的 32 位计数
			 基于影子帧缓冲检查

metrics_bytes_sent	 通过 USB 传输以向硬件传达结果更改像素的字节数的
			 32 位计数。包含压缩和协议开销

metrics_cpu_kcycles_used 处理上述像素所使用的 CPU 周期的 32 位计数
			 （以千周期为单位）。

metrics_reset		 只写。对此文件的任何写入都会将上述所有指标重置为零。
			 注意，上面的 32 位计数器翻转得非常快。为获得可靠结果，
			 请将性能测试设计为在非常短的时间内开始和结束（一分钟
			 或更短是安全的）。
======================== ========================================================

Bernie Thompson <bernie@plugable.com>
