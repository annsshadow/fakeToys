
## cdc_mbim - 用于 CDC MBIM 移动宽带调制解调器的驱动

cdc_mbim 驱动支持符合 “Universal Serial Bus Communications Class Subclass Specification for Mobile Broadband Interface Model”（移动宽带接口模型的 USB 通信类子类规范）[^1^] 的 USB 设备，该规范是 “Universal Serial Bus Communications Class Subclass Specifications for Network Control Model Devices”（网络控制模型设备的 USB 通信类子类规范）[^2^] 针对移动宽带设备（即 “3G/LTE 调制解调器”）的进一步优化版本。


## 命令行参数

cdc_mbim 驱动本身没有自己的参数。但是对 NCM 1.0 向后兼容的 MBIM 功能（即 [^1^] 第 3.2 节中定义的 “NCM/MBIM 功能”）的探测行为受 cdc_ncm 驱动参数的影响：

### prefer_mbim

:Type:          Boolean
:Valid Range:   N/Y (0-1)
:Default Value: Y（偏好 MBIM）

该参数设置了针对 NCM/MBIM 功能的系统策略。此类功能将由 cdc_ncm 驱动或 cdc_mbim 驱动处理，取决于 prefer_mbim 设置。将 prefer_mbim 设为 N 会让 cdc_mbim 驱动忽略这些功能，而改由 cdc_ncm 驱动处理它们。

该参数是可写的，可以随时更改。需要手动执行 unbind/bind 才能使更改对绑定到“错误”驱动的 NCM/MBIM 功能生效。


## 基本用法

MBIM 功能在未受管理时处于非活动状态。cdc_mbim 驱动仅提供到 MBIM 控制通道的用户空间接口，并且不会参与该功能的日常管理。这意味着始终需要一个用户空间 MBIM 管理应用来启用一个 MBIM 功能。

此类用户空间应用包括但不限于：

 - mbimcli（包含在 libmbim [^3^] 库中），以及
 - ModemManager [^4^]

建立一个 MBIM IP 会话至少需要管理应用执行以下操作：

 - 打开控制通道
 - 配置网络连接设置
 - 连接到网络
 - 配置 IP 接口

### 管理应用开发

驱动 <-> 用户空间的接口如下文所述。MBIM 控制通道协议在 [^1^] 中描述。


## MBIM 控制通道用户空间 ABI


### /dev/cdc-wdmX 字符设备

驱动使用 cdc-wdm 驱动作为子驱动，创建一个到 MBIM 功能控制通道的双向管道。控制通道管道的用户空间端是一个 /dev/cdc-wdmX 字符设备。

cdc_mbim 驱动不处理或审查控制通道上的消息。该通道完全委托给用户空间管理应用。因此，确保符合 [^1^] 中所有控制通道要求的责任在于该应用。

cdc-wdmX 设备是作为 MBIM 控制接口 USB 设备的子设备创建的。与特定设备关联的字符设备

```
 bjorn@nemi:~$ ls /sys/bus/usb/drivers/cdc_mbim/2-4:2.12/usbmisc
 cdc-wdm0

 bjorn@nemi:~$ grep . /sys/bus/usb/drivers/cdc_mbim/2-4:2.12/usbmisc/cdc-wdm0/dev
 180:0

```
### USB 配置描述符

CDC MBIM 功能描述符的 wMaxControlMessage 字段限制了最大的控制消息大小。管理应用负责协商一个符合 [^1^] 第 9.3.1 节要求的控制消息大小，同时考虑此描述符字段。

用户空间应用可以使用 [^6^] 或 [^7^] 中描述的两种 USB 配置描述符内核接口之一来访问 MBIM 功能的 CDC MBIM 功能描述符。

另见下文关于 ioctl 的文档。


### 分片（Fragmentation）

用户空间应用负责所有的控制消息分片和去分片，如 [^1^] 第 9.5 节所述。


### /dev/cdc-wdmX write()

来自管理应用的 MBIM 控制消息**不得**超过协商的控制消息大小。


### /dev/cdc-wdmX read()

管理应用**必须**接受最大为协商控制消息大小的控制消息。


### /dev/cdc-wdmX ioctl()

IOCTL_WDM_MAX_COMMAND：获取最大命令大小
此 ioctl 返回 MBIM 设备的 CDC MBIM 功能描述符中的 wMaxControlMessage 字段。这作为一种便利而提供，消除了从用户空间解析 USB 描述符的需要。

```
	#include <stdio.h>
	#include <fcntl.h>
	#include <sys/ioctl.h>
	#include <linux/types.h>
	#include <linux/usb/cdc-wdm.h>
	int main()
	{
		__u16 max;
		int fd = open("/dev/cdc-wdm0", O_RDWR);
		if (!ioctl(fd, IOCTL_WDM_MAX_COMMAND, &max))
			printf("wMaxControlMessage is %d\n", max);
	}

```
### 自定义设备服务

MBIM 规范允许供应商自由定义额外的服务。cdc_mbim 驱动完全支持这一点。

对新的 MBIM 服务（包括供应商指定的服务）的支持，与 MBIM 控制协议的其余部分一样，完全在用户空间实现。

新的服务应在 MBIM Registry [^5^] 中注册。


## MBIM 数据通道用户空间 ABI


### wwanY 网络设备

cdc_mbim 驱动将 MBIM 数据通道表示为一个 “wwan” 类型的单一网络设备。该网络设备最初映射到 MBIM IP 会话 0。


### 多路复用的 IP 会话（IPS）

MBIM 允许在单个 USB 数据通道上多路复用多达 256 个 IP 会话。cdc_mbim 驱动将这些 IP 会话建模为主 wwanY 设备的 802.1q VLAN 子设备，将所有大于 0 的 Z 值映射到 MBIM IP 会话 Z 到 VLAN ID Z。

设备最大 Z 值在 [^1^] 第 10.5.1 节描述的 MBIM_DEVICE_CAPS_INFO 结构中给出。

用户空间管理应用负责在建立 SessionId 大于 0 的 MBIM IP 会话之前添加新的 VLAN 链路。这些链路可以使用普通的 VLAN 内核接口（ioctl 或 netlink）来添加。

```
  ip link add link wwan0 name wwan0.3 type vlan id 3

```
驱动将自动把 “wwan0.3” 网络设备映射到 MBIM IP 会话 3。


### 设备服务流（DSS）

MBIM 还允许在同一个共享 USB 数据通道上多路复用多达 256 个非 IP 数据流。cdc_mbim 驱动将这些会话建模为主 wwanY 设备的另一组 802.1q VLAN 子设备，将所有 A 值映射到 MBIM DSS 会话 A 到 VLAN ID（256 + A）。

设备最大 A 值在 [^1^] 第 10.5.29 节描述的 MBIM_DEVICE_SERVICES_INFO 结构中给出。

DSS VLAN 子设备用作共享 MBIM 数据通道与感知 MBIM DSS 的用户空间应用之间的一个实用接口。它不打算原样呈现给最终用户。假设发起 DSS 会话的用户空间应用也会负责 DSS 数据的必要成帧，并以适合该流类型的方式将流呈现给最终用户。

网络设备 ABI 要求为每个被传输的 DSS 数据帧附加一个伪以太网头。该头的内容是任意的，但有以下例外：

 - 使用 IP 协议（0x0800 或 0x86dd）的 TX 帧将被丢弃
 - RX 帧的协议字段将被设为 ETH_P_802_3（但不会被正确格式化为 802.3 帧）
 - RX 帧的目的地址将被设为主设备的硬件地址

支持 DSS 的用户空间管理应用负责在 TX 时添加伪以太网头并在 RX 时剥离它。

这是一个使用常用工具的简单示例，将 DssSessionId 5 导出为指向 /dev/nmea 的 pty 字符设备

```
  ip link add link wwan0 name wwan0.dss5 type vlan id 261
  ip link set dev wwan0.dss5 up
  socat INTERFACE:wwan0.dss5,type=2 PTY:,echo=0,link=/dev/nmea

```
这只是一个示例，最适合用来测试 DSS 服务。支持特定 MBIM DSS 服务的用户空间应用应当使用该服务所需的工具和编程接口。

注意，为 DSS 会话添加 VLAN 链路完全是可选的。管理应用也可以选择将包套接字直接绑定到主网络设备，使用接收到的 VLAN 标签将帧映射到正确的 DSS 会话，并在 TX 时添加带有适当标签的 18 字节 VLAN 以太网头。在这种情况下，建议使用套接字过滤器，只匹配 DSS VLAN 子集。这避免将无关的 IP 会话数据不必要地复制到用户空间。对于

```
  static struct sock_filter dssfilter[] = {
	/* 使用特殊的负偏移来获取 VLAN 标签 */
	BPF_STMT(BPF_LD|BPF_B|BPF_ABS, SKF_AD_OFF + SKF_AD_VLAN_TAG_PRESENT),
	BPF_JUMP(BPF_JMP|BPF_JEQ|BPF_K, 1, 0, 6), /* true */

	/* 验证 DSS VLAN 范围 */
	BPF_STMT(BPF_LD|BPF_H|BPF_ABS, SKF_AD_OFF + SKF_AD_VLAN_TAG),
	BPF_JUMP(BPF_JMP|BPF_JGE|BPF_K, 256, 0, 4),	/* 256 是第一个 DSS VLAN */
	BPF_JUMP(BPF_JMP|BPF_JGE|BPF_K, 512, 3, 0),	/* 511 是最后一个 DSS VLAN */

	/* 验证以太类型 */
	BPF_STMT(BPF_LD|BPF_H|BPF_ABS, 2 * ETH_ALEN),
	BPF_JUMP(BPF_JMP|BPF_JEQ|BPF_K, ETH_P_802_3, 0, 1),

	BPF_STMT(BPF_RET|BPF_K, (u_int)-1),	/* 接受 */
	BPF_STMT(BPF_RET|BPF_K, 0),		/* 忽略 */
  };



```
### 带标签的 IP 会话 0 VLAN

如上所述，MBIM IP 会话 0 被驱动当作特殊的来处理。它最初映射到 wwanY 网络设备上未打标签的帧。

这种映射对多路复用的 IPS 和 DSS 会话施加了一些限制，这些限制可能并不总是实用：

 - 任何 IPS 或 DSS 会话都不能使用大于 IP 会话 0 上 MTU 的帧大小
 - 除非表示 IP 会话 0 的网络设备也处于 up 状态，否则任何 IPS 或 DSS 会话都不能处于 up 状态

这些问题可以通过可选地让驱动将 IP 会话 0 映射到一个 VLAN 子设备来避免，类似于所有其他 IP 会话。这种行为通过为魔术 VLAN ID 4094 添加 VLAN 链路来触发。然后驱动将立即开始将 MBIM IP 会话 0 映射到该 VLAN，并将丢弃主 wwanY 设备上的未打标签帧。

提示：将该 VLAN 子设备以 MBIM SessionID 而不是 VLAN ID 命名，对最终用户来说可能不那么令人困惑。对于

```
  ip link add link wwan0 name wwan0.0 type vlan id 4094


```
### VLAN 映射

总结上述描述的 cdc_mbim 驱动映射，我们有 wwanY 网络设备上的 VLAN 标签与 MBIM 之间的关系

```
  VLAN ID       MBIM type   MBIM SessionID           Notes
  ---------------------------------------------------------
  untagged      IPS         0                        a)
  1 - 255       IPS         1 - 255 <VLANID>
  256 - 511     DSS         0 - 255 <VLANID - 256>
  512 - 4093                                         b)
  4094          IPS         0                        c)

    a) 如果不存在 VLAN ID 4094 链路，则丢弃，否则被丢弃
    b) 不支持的 VLAN 范围，无条件丢弃
    c) 如果存在 VLAN ID 4094 链路，否则丢弃



```
## 参考文献

 1) USB Implementers Forum, Inc. - "Universal Serial Bus
    Communications Class Subclass Specification for Mobile Broadband
    Interface Model", Revision 1.0 (Errata 1), May 1, 2013

      - http://www.usb.org/developers/docs/devclass_docs/

 2) USB Implementers Forum, Inc. - "Universal Serial Bus
    Communications Class Subclass Specifications for Network Control
    Model Devices", Revision 1.0 (Errata 1), November 24, 2010

      - http://www.usb.org/developers/docs/devclass_docs/

 3) libmbim - "a glib-based library for talking to WWAN modems and
    devices which speak the Mobile Interface Broadband Model (MBIM)
    protocol"

      - http://www.freedesktop.org/wiki/Software/libmbim/

 4) ModemManager - "a DBus-activated daemon which controls mobile
    broadband (2G/3G/4G) devices and connections"

      - http://www.freedesktop.org/wiki/Software/ModemManager/

 5) "MBIM (Mobile Broadband Interface Model) Registry"

       - http://compliance.usb.org/mbim/

 6) "/sys/kernel/debug/usb/devices output format"

       - Documentation/driver-api/usb/usb.rst

 7) "/sys/bus/usb/devices/.../descriptors"

       - Documentation/ABI/stable/sysfs-bus-usb
