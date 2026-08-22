## SPI 用户空间 API


SPI 设备拥有一个受限的用户空间 API，支持对 SPI 从设备的基本半双read() write() 访问。通过使用 ioctl() 请求，还可以进行全双工传输和设备 I/O 配置
```
	#include <fcntl.h>
	#include <unistd.h>
	#include <sys/ioctl.h>
	#include <linux/types.h>
	#include <linux/spi/spidev.h>

```
你可能想使用这个编程接口的一些原因包括：

 - 在一个不容易崩溃的环境中做原型开发；用户空间的野指针通常不会导致任何 Linux 系统崩溃
 - 开发用于与充当 SPI 从设备的微控制器通信的简单协议，这些协议你可能需要频繁更改
当然，有些驱动永远无法在用户空间编写，因为它们需要访问用户空间无法触及的内核接口（例IRQ 处理程序或驱动栈的其他层）

## 设备创建、驱动绑

spidev 驱动包含针对不同硬件拓扑表示所支持SPI 设备列表
以下spidev 驱动支持SPI 设备表：

    - struct spi_device_id spidev_spi_ids[]：当使用带有与表中某项匹配的 .modalias 字段struct spi_board_info 定义这些设备时，可以绑定的设备列表
    - struct of_device_id spidev_dt_ids[]：当使用带有与表中某项匹配的 compatible 字符串的 Device Tree 节点定义这些设备时，可以绑定的设备列表
    - struct acpi_device_id spidev_acpi_ids[]：当使用带有与表中某项匹配的 _HID ACPI 设备对象定义这些设备时，可以绑定的设备列表
如果这些表还没有你的 SPI 设备名称的条目，建议你把它加入相关表中。为此，请向 linux-spi@vger.kernel.org 邮件列表提交一个针spidev 的补丁
过去支持使用 "spidev" 名称来定SPI 设备，例.modalias = "spidev" compatible = "spidev"。但 Linux 内核已不再支持这种方式，而必须使用表中列出的真实 SPI 设备名称
没有真实SPI 设备名称会导致打印错误，并且 spidev 驱动探测失败
sysfs 还支持用户空间驱动的、对使用上述表之一无法自动绑定的设备进行的驱动绑定/解绑```

    echo spidev > /sys/bus/spi/devices/spiB.C/driver_override
    echo spiB.C > /sys/bus/spi/drivers/spidev/bind

```
spidev 驱动绑定到一SPI 设备时，该设备的 sysfs 节点将包含一个带"dev" 属性的子设备节点，udev mdev（BusyBox udev 替代品；功能较少，但通常够用）可以识别它
对于总线 B 上片选为 C SPI 设备，你应该会看到：

    /dev/spidevB.C ...
	字符特殊设备，主设备153，次设备号动态分配。这是用户空间程序将打开的节点，"udev" "mdev" 创建
    /sys/devices/.../spiB.C ...
	与通常一样，SPI 设备节点将是SPI 主控制器的一个子节点
    /sys/class/spidev/spidevB.C ...
	"spidev" 驱动绑定到该设备时创建。（是目录还是符号链接，取决于你是否启用"deprecated sysfs files" Kconfig 选项。）

不要试图手动管理 /dev 字符设备特殊文件节点。这容易出错，而且你需要仔细关注系统安全问题；udev/mdev 应该已经被安全地配置好了
如果你从该设备解"spidev" 驱动，那两个 "spidev" 节点（在 sysfs /dev 中）应该会自动被移除（分别由内核udev/mdev 移除）。你可以通过移除 "spidev" 驱动模块来解绑，这会影响所有使用该驱动的设备。你也可以通过让内核代码移除该 SPI 设备来解绑，很可能是通过移除SPI 控制器的驱动（这样它spi_master 就消失了）
由于这是一个标准的 Linux 设备驱动——尽管它恰好向用户空间暴露了一个底API——它可以同时与任意数量的设备关联。只需为每个这样的 SPI 设备提供一spi_board_info 记录，你就会为每个设备获得一/dev 设备节点

## 基本字符设备 API

/dev/spidevB.D 文件的普open() close() 操作与你预期的一致
标准read() write() 操作显然只是半双工的，并且在那些操作之间片选（chipselect）会被取消激活。使SPI_IOC_MESSAGE(N) 请求可以进行全双工访问，以及片选不取消激活的复合操作
多个 ioctl() 请求让你的驱动读取或覆盖设备当前的数据传输参数设置：

    SPI_IOC_RD_MODE, SPI_IOC_WR_MODE ...
	传入一个指向字节的指针，该字节会返回（RD）或赋予（WR）SPI 传输模式。使用常SPI_MODE_0..SPI_MODE_3；或者如果你愿意，也可以组合 SPI_CPOL（时钟极性，置位时空闲为高电平）SPI_CPHA（时钟相位，置位时在尾沿采样）标志。注意该请求仅限于能放入单个字节SPI 模式标志
    SPI_IOC_RD_MODE32, SPI_IOC_WR_MODE32 ...
	传入一个指uin32_t 的指针，该指针会返回（RD）或赋予（WR）完整的 SPI 传输模式，不限于能放入一个字节的位
    SPI_IOC_RD_LSB_FIRST, SPI_IOC_WR_LSB_FIRST ...
	传入一个指向字节的指针，该字节会返回（RD）或赋予（WR）用于传SPI 字的位对齐方式。零表示 MSB 优先；其他值表示较少见LSB 优先编码。在这两种情况下，指定的值在每个字中都是右对齐的，因此未使用的（TX）或未定义的（RX）位位于 MSB 中
    SPI_IOC_RD_BITS_PER_WORD, SPI_IOC_WR_BITS_PER_WORD ...
	传入一个指向字节的指针，该字节会返回（RD）或赋予（WR）每SPI 传输字中的位数。零值表示八位
    SPI_IOC_RD_MAX_SPEED_HZ, SPI_IOC_WR_MAX_SPEED_HZ ...
	传入一个指u32 的指针，该指针会返回（RD）或赋予（WR）最SPI 传输速度（单位为 Hz）。控制器不一定能分配那个特定的时钟速度
注意
    - 目前没有异步 I/O 支持；一切都是纯粹的同步操作
    - 目前无法报告用于向给定设备移移出数据的实际比特率
    - 从用户空间，你目前无法改变片选极性；那可能会破坏对共SPI 总线的其他设备的传输。每SPI 设备在不被主动使用时都会被取消选择，从而允许其他驱动与其他设备通信
    - 每个 I/O 请求可以传输SPI 设备的字节数有限制。默认是一页，但可以通过模块参数更改
    - 因为 SPI 没有底层传输确认，你在与一个不存在的设备通信时通常不会看到任何 I/O 错误

## 全双工字符设API


示例程序 spidev_fdx.c 展示了使用全双工编程接口的一个例子。（尽管它并没有执行全双工传输。）其模型与内核 spi_sync() 请求中使用的相同；各个传输提供与内核驱动可用能力相同的功能（只是它不是异步的）
该示例展示了一个半双工RPC 风格请求和响应消息。这些请求通常要求在请求和响应之间不取消选择芯片。若干个这样的请求可以链接成一个内核请求，甚至允许在每个响应之后取消选择芯片。（其他协议选项包括为每个传输段更改字大小和比特率。）

要进行全双工请求，为同一个传输同时提rx_buf tx_buf。即使两者是同一个缓冲区也没问题