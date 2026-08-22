## vfio-ccw：基础架构


### 简

此处我们描述 Linux/s390 I/O 子通道设备vfio 支持。vfio-ccw 的动机是将子通道透传给虚拟机，vfio 是实现这一目的的手段
与其他硬件架构不同，s390 定义了一种统一I/O 访问方法，即所谓的通道 I/O（Channel I/O）。它有自己的访问模式
- 通道程序在独立的（协）处理器上异步运行- 通道子系统会直接访问调用者在通道程序中指定的任何内存，即不涉IOMMU
因此，当我们为这些设备引vfio 支持时，我们通过中介设备（mdev）实现来完成。vfio mdev 会被加入一IOMMU 组，从而使自身能被 vfio 框架管理。并且我们为特殊vfio I/O 区域添加了读/写回调，以便将通道程序mdev 传递给其父设备（真实的 I/O 子通道设备），进行进一步的地址转换并执I/O 指令
本文档无意详尽解s390 I/O 架构的每一个细节。更多信参考可在此处找到：

- 了解通道 I/O 的一个良好起点：
  https://en.wikipedia.org/wiki/Channel_I/O
- s390 架构  s390 Principles of Operation manual（IBM 表单SA22-7832- 现有QEMU 代码实现了一个简单的仿真通道子系统，也是一个很好的参考。它能帮助你更容易地跟踪流程  qemu/hw/s390x/css.c

关于 vfio 中介设备框架- Documentation/driver-api/vfio-mediated-device.rst

### vfio-ccw 的动

通常，在 s390 上通过 QEMU/KVM 虚拟化的客户机只能经"Virtio Over Channel I/O（virtio-ccw 传输看到半虚拟化virtio 设备。这使得 virtio 设备可通过处理通道设备的标准操作系统算法被发现
然而这还不够。在 s390 上，对于大多数使用基于标准通道 I/O 机制devices，我们还需要提供将它们透传QEMU 虚拟机的功能。这包括没有 virtio 对应物（例如磁带机）或具有客户机希望利用的特定特性的设备
为了将设备透传给客户机，我们希望使用与其他人相同的接口，即 vfio。我们通过 vfio 中介设备框架与子通道设备驱动 "vfio_ccw" 来实现这种对通道设备vfio 支持
### CCW 设备的访问模

s390 架构实现了所谓的通道子系统，它为物理连接到系统的设备提供统一的视图。尽s390 硬件平台支持种类繁多的不同外设，如磁盘设备（DASD）、磁带机、通信控制器等。它们都可以通过定义良好的访问方法来访问，并且都以统一的方式呈I/O 完成：I/O 中断
所I/O 都需要使用通道命令字（CCW）。CCW 是对专用 I/O 通道处理器的指令。通道程序是由 I/O 通道子系统执行的一CCW 序列。要向通道子系统发出通道程序，需要构建一个操作请求块（ORB），它用于向系统指明 CCW 的格式及其他控制信息。操作系统通过 SSCH（start sub-channel，启动子通道）指令通知 I/O 通道子系统开始执行通道程序。随后中央处理器可自由继续执行非 I/O 指令，直到被中断。I/O 完成结果由中断处理程序以中断响应块（IRB）的形式接收
回到 vfio-ccw，简而言之：

- ORB 与通道程序构建于客户机内核中（使用客户机物理地址）- ORB 与通道程序被传递给宿主机内核- 宿主机内核将客户机物理地址转换为真实地址，并通过发出一条特权通道 I/O 指令（例SSCH）来启动 I/O- 通道程序在独立处理器上异步运行- I/O 完成将通过 I/O 中断通知宿主机。它会被作为 IRB 复制到用户空间，从而传递回客户机
### 物理 vfio-ccw 设备及其mdev


如上所述，我们通过 mdev 实现来完vfio-ccw
通道 I/O 没有 IOMMU 硬件支持，因此物vfio-ccw 设备没有 IOMMU 级别的转换或隔离
子通道 I/O 指令都是特权指令。在处理 I/O 指令拦截时，vfio-ccw 会在通道程序被发送到硬件之前，对其进行软件的审查与转换，确定通道程序的编写方式
在此实现中，我们有分别针对两类设备的两个驱动
- 用于物理子通道设备vfio_ccw 驱动  这是一个用于真实子通道设备I/O 子通道驱动。它实现了一组回调，并作为父（物理）设备注册mdev 框架。因此，mdev vfio_ccw 提供了一个用于创mdev 设备的通用接口（sysfs）。随vfio_ccw 可以创建一vfio mdev，并将其加入中介总线。正是这vfio 设备被加入了一IOMMU 组和一vfio 组  vfio_ccw 还提供一I/O 区域，用于接受来自用户空间的通道程序请求，并存储供用户空间取回的 I/O 中断结果。为了向用户空间通知 I/O 完成，它提供了一个用于建eventfd fd 以进行异步通知的接口
- 用于中介 vfio-ccw 设备vfio_mdev 驱动  它由 mdev 框架提供。它vfio_ccw 所创建 mdev 的一vfio 设备驱动  它实现了一vfio 设备驱动回调，将自身加入一vfio 组，并作mdev 驱动注册mdev 框架  它使用一vfio iommu 后端，该后端使用现有map unmap ioctl，但不同于将它们编程进某设备IOMMU，它只是存储这些转换以供后续请求使用。这意味着，在虚拟机中以客户机物理地址编程的设备，可以vfio 内核将该地址转换为进程虚拟地址、固定该页，并一步到位地用宿主机物理地址对硬件进行编程  对于 mdev，vfio iommu 后端不会VFIO_IOMMU_MAP_DMA ioctl 期间固定页面。Mdev 框架在此操作中仅维护一iova<->vaddr 映射的数据库。并且它们从 vfio iommu 后端导出vfio_pin_pages vfio_unpin_pages 接口，供物理设备按需固定与解除固定页面
```
 +-------------+
 |             |
 | +---------+ | mdev_register_driver() +--------------+
 | |  Mdev   | +<-----------------------+              |
 | |  bus    | |                        | vfio_mdev.ko |
 | | driver  | +----------------------->+              |<-> VFIO user
 | +---------+ |    probe()/remove()    +--------------+    APIs
 |             |
 |  MDEV CORE  |
 |   MODULE    |
 |   mdev.ko   |
 | +---------+ | mdev_register_parent() +--------------+
 | |Physical | +<-----------------------+              |
 | | device  | |                        |  vfio_ccw.ko |<-> subchannel
 | |interface| +----------------------->+              |     device
 | +---------+ |       callback         +--------------+
 +-------------+
```

这些组件协同工作的过程
1. vfio_ccw.ko 驱动物理 I/O 子通道，并将物理设备（含回调）注册mdev 框架   vfio_ccw 探查子通道设备时，它将设备指针与回调注册到 mdev 框架。子通道设备sysfs 中设备节点下会创mdev 相关的文件节点，'mdev_create'mdev_destroy' 'mdev_supported_types'2. 创建一个中vfio-ccw 设备   使用 'mdev_create' sysfs 文件，我们需要手动创建一个（在我们的情形中且只能是一个）中介设备3. vfio_mdev.ko 驱动中介 ccw 设备   vfio_mdev 也是 vfio 设备驱动。它会探mdev 并将其加入一iommu_group 与一vfio_group。之后我们就能将mdev 透传给客户机

### VFIO-CCW 区域


vfio-ccw 驱动暴露 MMIO 区域，用于接受来自用户空间的请求并向其返回结果
### vfio-ccw I/O 区域


I/O 区域用于接受来自用户空间的通道程序请求，并存储 I/O 中断结果供用户空间取回。其

```
  struct ccw_io_region {
  #define ORB_AREA_SIZE 12
	  __u8    orb_area[ORB_AREA_SIZE];
  #define SCSW_AREA_SIZE 12
	  __u8    scsw_area[SCSW_AREA_SIZE];
  #define IRB_AREA_SIZE 96
	  __u8    irb_area[IRB_AREA_SIZE];
	  __u32   ret_code;
  } __packed;
```

该区域始终可用
在发I/O 请求时，orb_area 应填入客户机 ORB，scsw_area 应填入虚拟子通道SCSW
irb_area 存储 I/O 结果
ret_code 存储每次访问该区域的返回码。可能出现以下值：

`0`
  操作成功
`-EOPNOTSUPP`
  ORB 指定了传输模式，SCSW 指定了除启动（start）功能以外的功能
`-EIO`
  在设备未处于可接受请求的状态时发出了请求，或发生了内部错误
`-EBUSY`
  子通道处于状态挂起或繁忙状态，或已有请求正在进行
`-EAGAIN`
  请求正在被处理，调用者应重试
`-EACCES`
  用于 I/O 的通道路径被发现不可用
`-ENODEV`
  设备被发现不可用
`-EINVAL`
  ORB 指定了长度超255 CCW 的链，或发生了内部错误

### vfio-ccw cmd 区域


vfio-ccw cmd 区域用于接受异步指令

```
  #define VFIO_CCW_ASYNC_CMD_HSCH (1 << 0)
  #define VFIO_CCW_ASYNC_CMD_CSCH (1 << 1)
  struct ccw_cmd_region {
         __u32 command;
         __u32 ret_code;
  } __packed;
```

该区域通过区域类型 VFIO_REGION_SUBTYPE_CCW_ASYNC_CMD 暴露
目前，CLEAR SUBCHANNEL HALT SUBCHANNEL 使用此区域
command 指定要发出的命令；ret_code 存储每次访问该区域的返回码。可能出现以下值：

`0`
  操作成功
`-ENODEV`
  设备被发现不可用
`-EINVAL`
  指定了除 halt clear 以外的命令
`-EIO`
  在设备未处于可接受请求的状态时发出了请求
`-EAGAIN`
  请求正在被处理，调用者应重试
`-EBUSY`
  在处halt 请求期间，子通道处于状态挂起或繁忙状态
### vfio-ccw schib 区域


vfio-ccw schib 区域用于返回子通道信息（Subchannel-Information
```
  struct ccw_schib_region {
  #define SCHIB_AREA_SIZE 52
         __u8 schib_area[SCHIB_AREA_SIZE];
  } __packed;
```

该区域通过区域类型 VFIO_REGION_SUBTYPE_CCW_SCHIB 暴露
读取该区域会触发向关联硬件发STORE SUBCHANNEL
### vfio-ccw crw 区域


vfio-ccw crw 区域用于返回通道报告字（Channel Report Word，CRW
```
  struct ccw_crw_region {
         __u32 crw;
         __u32 pad;
  } __packed;
```

该区域通过区域类型 VFIO_REGION_SUBTYPE_CCW_CRW 暴露
读取该区域会返回一CRW（如果存在与此子通道相关、且处于等待状态的 CRW，例如报告通道路径状态变化的 CRW），否则返回全零。如果多CRW 处于等待状态（可能包括链式 CRW），再次读取该区域会返回下一个，直到没有更多 CRW 等待并返回零为止。这STORE CHANNEL REPORT WORD 的工作方式类似
### vfio-ccw 操作细节


vfio-ccw 沿用vfio-pci s390 平台上的做法，并使用 vfio-iommu-type1 作为 vfio iommu 后端
- CCW 转换 API
  一组以 `cp_` 开头的 API，用于执CCW 转换。用户空间程序传入的 CCW 以其客户机物理内存地址组织。这API 会将 CCW 复制到内核空间，并通过用相应的宿主机物理地址替换客户机物理地址，组装出一个可运行的內核通道程序  注意，即使对于直接访问的 CCW，我们也必须使用 IDAL，因为所引用的内存可能位于任何位置，包括 2G 以上
- vfio_ccw 设备驱动
  该驱动利CCW 转换 API 并引入了 vfio_ccw，它是你要透传I/O 子通道设备的驱动
```
    VFIO_DEVICE_GET_INFO
    VFIO_DEVICE_GET_IRQ_INFO
    VFIO_DEVICE_GET_REGION_INFO
    VFIO_DEVICE_RESET
    VFIO_DEVICE_SET_IRQS
```

  这提供了一I/O 区域，使用户空间程序能够传入通道程序，以便在将其发出到真实设备之前进行进一步的 CCW 转换  这也提供SET_IRQ ioctl 来建立一个事件通知器，以异步方式通知用户空间程序 I/O 已完成
vfio-ccw 的使用并不限QEMU，不QEMU 无疑是理解这些补丁如何工作的好例子。以下是一个由 QEMU 客户机触发的 I/O 请求将如何被处理（不含错误处理）的更多细节
说明
- Q1-Q7：QEMU 侧流程- K1-K5：内核侧流程
Q1.
    在初始化期间获取 I/O 区域信息
Q2.
    建立事件通知器与处理程序以处I/O 完成
... ...

Q3.
    拦截一ssch 指令Q4.
    将客户机通道程序ORB 写入 I/O 区域
    K1.
	从客户机复制到内核    K2.
	将客户机通道程序转换为宿主机内核空间通道程序，使其成为真实设备可运行的程序    K3.
	利用 QEMU 传入orb 中所含的必要信息，向设备发出 ccwchain    K4.
	返回 ssch CC 码Q5.
    CC 码返回给客户机
... ...

    K5.
	中断处理程序获取 I/O 结果，并将结果写I/O 区域    K6.
	通知 QEMU 取回结果
Q6.
    收到信号后，事件处理程序I/O 区域读出结果Q7.
    为客户机更新 irb
### 限制


当前vfio-ccw 实现仅专注于支持实现 DASD/ECKD 设备块设备功能（写）所需的基本命令。某些命令将来可能需要特殊处理，例如任何与路径分组相关的内容
DASD 是一种存储设备。ECKD 是一种数据记录格式。有DASD ECKD 的更多信息可在此处找到：
https://en.wikipedia.org/wiki/Direct-access_storage_device
https://en.wikipedia.org/wiki/Count_key_data

结合 QEMU 中的相应工作，我们现在可以让透传DASD/ECKD 设备在客户机中上线并作为块设备使用
当前代码允许客户机通过 START SUBCHANNEL 启动通道程序，并发出 HALT SUBCHANNEL、CLEAR SUBCHANNEL STORE SUBCHANNEL
目前所有通道程序都会被预取，无论 ORB p 位的设置如何。因此，不支持自修改的通道程序。出于这个原因，IPL 必须由用户空客户机程序作为特例处理；这已QEMU 4.1 s390-ccw bios 中实现
vfio-ccw 仅支持经典（命令模式）通道 I/O。传输模式（HPF）不受支持
目前不支QDIO 子通道。除 DASD/ECKD 之外的经典设备可能可以工作，但尚未经过测试
### 参考资

1. ESA/s390 Principles of Operation manual（IBM 表单SA22-78322. ESA/390 Common I/O Device Commands manual（IBM 表单SA22-72043. https://en.wikipedia.org/wiki/Channel_I/O
4. Documentation/arch/s390/cds.rst
5. Documentation/driver-api/vfio.rst
6. Documentation/driver-api/vfio-mediated-device.rst
