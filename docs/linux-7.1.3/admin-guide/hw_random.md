## 硬件随机数生成器


## 简介


hw_random 框架是一个利用你的 CPU 或主板上特殊硬件特性——随机数生成器（RNG）——
的软件。该软件包含两部分：提供一个 /dev/hwrng 字符设备及其 sysfs 支持的
核心部分，以及一个插入该核心的硬件专用驱动。

为了最有效地利用这些机制，你还应当下载相应的支持软件。请从以下地址下载
最新的 "rng-tools" 软件包：

	https://github.com/nhorman/rng-tools

这些工具使用 /dev/hwrng 来填充内核熵池，该熵池在内部使用，并通过 /dev/urandom 与
/dev/random 特殊文件导出。

## 工作原理


字符设备。使用标准的 open() 与 read() 系统调用，你可以从硬件 RNG 设备读取
随机数据。这些数据**未经**任何适用性检测检查，并且可能不可靠（如果硬件存在故障或
遭到篡改）。仅当硬件的 "has-data" 标志置位时才会输出数据，尽管如此，注重安全性的人
会在假定数据真正随机之前，先对这些数据运行适用性检测。

rng-tools 软件包在 "rngd" 中使用此类检测，并允许你通过 "rngtest" 工具手动运行它们。

/dev/hwrng 是主设备号 10、次设备号 183 的字符设备。

类设备。存在一个 /sys/class/misc/hw_random 节点，具有两个独特属性："rng_available"
与 "rng_current"。"rng_available" 属性列出可用的硬件专用驱动，而 "rng_current" 列出
当前连接到 /dev/hwrng 的那个。如果你的系统有多个可用的 RNG，可以通过将 "rng_available"
列表中的某个名称写入 "rng_current" 来更改所使用的 RNG。

==========================================================================

Intel/AMD/VIA 随机数生成器（RNG）硬件驱动
 - 版权所有 2000,2001 Jeff Garzik <jgarzik@pobox.com>
 - 版权所有 2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>

## 关于 Intel RNG 硬件（摘自固件 hub 数据手册）


固件 Hub 集成了一个随机数生成器（RNG），利用硅材料固有的、本质上随机的量子力学
特性所产生的热噪声。当不生成新的随机比特时，RNG 电路会进入低功耗状态。Intel 将提供
一个二进制软件驱动，使第三方软件能够访问我们的 RNG，作为安全特性使用。目前，RNG
仅可在系统处于 OS-present 状态时使用。

## Intel RNG 驱动说明


FIXME：支持 poll(2)

	request_mem_region 已被移除，原因有三：

 1) 该驱动仅支持一个 RNG；
 2) RNG 使用的位置是 MMIO 可寻址内存中的一个固定位置；
 3) 对于正确工作的 BIOS e820 处理的用户，RNG 所在区域总是被保留，因此
	   request_mem_region 调用对于正确配置总是失败。然而，对于使用 mem=XX 的
	   用户，BIOS e820 信息**不在** /proc/iomem 中，此时 request_mem_region(RNG_ADDR)
	   可以成功。

## 驱动细节


基于：
	Intel 82802AB/82802AC Firmware Hub (FWH) Datasheet
	1999 年 5 月 订单号：290658-002 R

Intel 82802 Firmware Hub：
	Random Number Generator
	Programmer's Reference Manual
	1999 年 12 月 订单号：298029-001 R

Intel 82802 Firmware HUB Random Number Generator Driver
	版权所有 (c) 2000 Matt Sottek <msottek@quiknet.com>

特别感谢 Matt Sottek。我做了 "guts"（底层实现），他做了 "brains"（核心设计）以及
全部测试。
