## HIDRAW - 对 USB 和蓝牙人机接口设备（HID）的原始访问


hidraw 驱动为 USB 和蓝牙人机接口设备（HID）提供了原始接口。它与 hiddev 的不同之处在于：发送和接收的报告不会被 HID 解析器解析，而是原封不动地发送给设备并从中接收。

如果用户态应用程序确切知道如何与硬件设备通信，并且能够手动构造 HID 报告，就应当使用 hidraw。在为自定义 HID 设备编写用户态驱动时通常就是这种情况。

hidraw 对于与那些发送和接收数据的方式与其报告描述符不一致的不合规 HID 设备通信也很有用。由于 hiddev 会解析经由它收发、并根据设备报告描述符校验的报告，因此使用 hiddev 无法与这些不合规设备通信。对于这些不合规设备，除了编写自定义内核驱动之外，hidraw 是唯一的替代方案。

hidraw 的一个好处是：用户态应用程序使用它时与底层硬件类型无关。目前，hidraw 针对 USB 和蓝牙实现。未来，随着使用 HID 规范的新硬件总线类型被开发出来，hidraw 将会扩展以支持这些新总线类型。

hidraw 使用动态主设备号，这意味着应当依赖 udev 来创建 hidraw 设备节点。udev 通常会在 /dev 下直接创建设备节点（例如：/dev/hidraw0）。由于该位置依赖于发行版和 udev 规则，应用程序应当使用 libudev 来定位系统中挂载的 hidraw 设备。libudev 的教程如下：

```
	http://www.signal11.us/oss/udev/
	https://web.archive.org/web/2019*/www.signal11.us
```

### HIDRAW API


### read()

read() 将读取从 HID 设备接收的、已排队的报告。在 USB 设备上，使用 read() 读取的报告即设备通过 INTERRUPT IN 端点发送的报告。默认情况下，read() 会阻塞，直到有可供读取的报告为止。可以通过将 O_NONBLOCK 标志传给 open()，或使用 fcntl() 设置 O_NONBLOCK 标志，使 read() 变为非阻塞。

在使用编号报告的设备上，返回数据的第一个字节将是报告编号；报告数据紧随其后，从第二个字节开始。对于不使用编号报告的设备，报告数据将从第一个字节开始。

### write()

write() 函数将向设备写入一个报告。对于 USB 设备，如果设备具有 INTERRUPT OUT 端点，报告将通过该端点发送；否则将使用 SET_REPORT 传输，通过控制端点发送。

传递给 write() 的缓冲区的第一个字节应设为报告编号。如果设备不使用编号报告，第一个字节应设为 0。报告数据本身应从第二个字节开始。

### ioctl()

hidraw 支持以下 ioctl：

HIDIOCGRDESCSIZE:
	获取报告描述符大小（Get Report Descriptor Size）

该 ioctl 将获取设备报告描述符的大小。

HIDIOCGRDESC:
	获取报告描述符（Get Report Descriptor）

该 ioctl 使用 hidraw_report_descriptor 结构体返回设备的报告描述符。务必将 hidraw_report_descriptor 结构体的 size 字段设为 HIDIOCGRDESCSIZE 返回的大小。

HIDIOCGRAWINFO:
	获取原始信息（Get Raw Info）

该 ioctl 将返回一个 hidraw_devinfo 结构体，其中包含设备的总线类型、厂商 ID（VID）和产品 ID（PID）。总线类型可以是以下之一（定义于 uapi/linux/input.h）：

```
	- BUS_USB
	- BUS_HIL
	- BUS_BLUETOOTH
	- BUS_VIRTUAL
```

HIDIOCGRAWNAME(len):
	获取原始名称（Get Raw Name）

该 ioctl 返回一个包含设备厂商字符串和产品字符串的字符串。返回的字符串为 Unicode，采用 UTF-8 编码。

HIDIOCGRAWPHYS(len):
	获取物理地址（Get Physical Address）

该 ioctl 返回表示设备物理地址的字符串。对于 USB 设备，该字符串包含到设备的物理路径（USB 控制器、集线器、端口等）。对于蓝牙设备，该字符串包含设备的硬件（MAC）地址。

HIDIOCSFEATURE(len):
	发送特性报告（Send a Feature Report）

该 ioctl 将向设备发送一个特性报告。根据 HID 规范，特性报告始终使用控制端点发送。将所提供的缓冲区的第一个字节设为报告编号。对于不使用编号报告的设备，将第一个字节设为 0。报告数据从第二个字节开始。务必相应设置 len，使其比报告长度大 1（以计入报告编号）。

HIDIOCGFEATURE(len):
	获取特性报告（Get a Feature Report）

该 ioctl 将使用控制端点从设备请求一个特性报告。所提供的缓冲区的第一个字节应设为所请求报告的报告编号。对于不使用编号报告的设备，将第一个字节设为 0。返回的报告缓冲区将在第一个字节中包含报告编号，其后紧接从设备读取的报告数据。对于不使用编号报告的设备，报告数据将从返回缓冲区的第一个字节开始。

HIDIOCSINPUT(len):
	发送输入报告（Send an Input Report）

该 ioctl 将使用控制端点向设备发送一个输入报告。在大多数情况下，在设备上设置输入 HID 报告没有意义且不产生效果，但某些设备可能会用它来设置或重置某个报告的初始状态。与该报告一起发出的缓冲区格式与 HIDIOCSFEATURE 相同。

HIDIOCGINPUT(len):
	获取输入报告（Get an Input Report）

该 ioctl 将使用控制端点从设备请求一个输入报告。在大多数为常规输入报告提供专用 In 端点的设备上，这会更慢，但它允许主机请求特定报告编号的值。通常，在应用程序通过常规设备 read() 接口监听常规报告之前，用它来请求设备某个输入报告的初始状态。与该报告一起发出的缓冲区格式与 HIDIOCGFEATURE 相同。

HIDIOCSOUTPUT(len):
	发送输出报告（Send an Output Report）

该 ioctl 将使用控制端点向设备发送一个输出报告。在大多数为常规输出报告提供专用 Out 端点的设备上，这会更慢，但出于完整性仍予提供。通常，在应用程序通过常规设备 write() 接口发送更新之前，用它来设置设备某个输出报告的初始状态。与该报告一起发出的缓冲区格式与 HIDIOCSFEATURE 相同。

HIDIOCGOUTPUT(len):
	获取输出报告（Get an Output Report）

该 ioctl 将使用控制端点从设备请求一个输出报告。通常，在应用程序通过 HIDIOCSOUTPUT 请求或常规设备 write() 接口按需更新它之前，用它来检索设备某个输出报告的初始状态。与该报告一起发出的缓冲区格式与 HIDIOCGFEATURE 相同。

### 示例（Example）

在 samples/ 目录中可以找到 hid-example.c，其中展示了 hidraw 的 read()、write() 以及所有 ioctl 的用法。该代码可供任何人出于任何目的使用，并可作为使用 hidraw 开发应用程序的起点。

文档作者：

	Alan Ott <alan@signal11.us>, Signal 11 Software
