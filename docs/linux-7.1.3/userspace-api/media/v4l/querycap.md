
######## 查询能力


由于 V4L2 涵盖种类广泛的设备，并非 API 的所有方面都同样适用于所有类型的
设备。此外，同类型的设备具有不同的能力，本规范允许省API 中一些复杂且太重要的部分
VIDIOC_QUERYCAP ioctl 可用于检查内核设备是否兼容本规范，并查询设备支持函数 <devices> I/O 方法 <io>
从内核版3.1 开始，VIDIOC_QUERYCAP 将返回驱动使用的 V4L2 API 版本，通常
与内核版本匹配。无需使用 VIDIOC_QUERYCAP 检查特ioctl 是否受支持，如果
驱动不提供对ioctl 的支持，V4L2 核心现在会返`ENOTTY`
其他特性可通过调用相应ioctl 查询，例VIDIOC_ENUMINPUT 可了解设备上
视频连接器的数量、类型和名称。尽管抽象是API 的主要目标，VIDIOC_QUERYCAP
ioctl 也允许特定于驱动的应用程序可靠地识别驱动
所V4L2 驱动都必须支VIDIOC_QUERYCAP。应用程序应在打开设备后始终调ioctl