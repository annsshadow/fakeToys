

## Samsung S5P/Exynos4 FIMC 驱动


Copyright |copy| 2012 - 2013 Samsung Electronics Co., Ltd.

Samsung SoC 应用处理器中FIMC（Fully Interactive Mobile Camera，全交互移动相机
设备是一个集成的摄像头主机接口、色彩空间转换器、图像缩放器和旋转器。它还能够通过
SoC 内部的回写数据路径从 LCD 控制器（FIMD）捕获数据。SoC 中有多个 FIMC 实例（最4 个）
具有略微不同的能力，如像素对齐约束、旋转器可用性、LCD 回写支持等。该驱动位于
drivers/media/platform/samsung/exynos4-is 目录

### 支持SoC


S5PC100（仅 mem-to-mem）、S5PV210、Exynos4210

### 支持的特


- 摄像头并行接口捕获（ITU-R.BT601/565）；
- 摄像头串行接口捕获（MIPI-CSI2）；
- 内存到内存处理（色彩空间转换、缩放、镜像与旋转）；
- 运行时动态流水线重配置（将任FIMC 实例重新连接到任意并行视频输入或任意
  MIPI-CSI 前端）；
- 运行PM 以及系统级挂恢复

### 当前不支


- LCD 回写输入
- 每帧时钟门控（mem-to-mem

### 用户空间接口


#### 媒体设备接口


该驱动支media_controller 中定义的 Media Controller API。媒体设备驱动名称为
"Samsung S5P FIMC"銆。

该接口的目的是允许在运行时改FIMC 实例SoC 外设摄像头输入的分配，并可选地控制
MIPI-CSIS 设备FIMC 实体的内部连接

媒体设备接口允许配置 SoC，以通过一个以上的 FIMC 实例从传感器捕获图像数据
（例如用于取景器和静止图像捕获同时进行的设置）

重配置通过启用/禁用驱动在初始化期间创建的媒体链路来完成。内部设备拓扑可以很容易
通过媒体实体和链路枚举来发现

#### 内存到内存视频节


位于 /dev/video 设备节点V4L2 内存到内存接口。这是一个独立的视频设备，没有媒
pads。但请注意，不允许在同一 FIMC 实例上同时进mem-to-mem 和捕获视频节点操作
驱动会检测此类情况，但应用程序应当避免它们以防止出现未定义行为

#### 捕获视频节点


该驱动支devices. 中定义的 V4L2 视频捕获接口

在捕获和 mem-to-mem 视频节点上，仅支持多平面（multi-planar）API。更多细节见：planar-apis

#### 摄像头捕获子设备


每个 FIMC 实例导出一个子设备节点dev/v4l-subdev），同时为每个在平台级可用且启用
MIPI-CSI 接收设备（目前最多两个）创建一个子设备节点

#### sysfs


为了通过子设API 实现更精确的摄像头流水线控制，驱动创建了一个与 "s5p-fimc-md"
平台设备关联sysfs 条目。其路径为：
/sys/platform/devices/s5p-fimc-md/subdev_conf_mode銆。

一个典型的使用场景可能包含如下捕获流水线配置：
sensor subdev -> mipi-csi subdev -> fimc subdev -> video node

当我们通过用户空间子设API 配置这些设备时，配置流程必须从左到右，视频节点作
最后一个配置

当我们不使用子设备用户空API 时，属于该流水线的所有设备的整体配置都在视频节点
驱动处完成。sysfs 条目允许指示捕获节点驱动不要配置子设备（格式、裁剪），以避免
视频节点执行最后一步配置时重置子设备的配置

用于子设备完全控制支持（在開始流传输之前在用户空间配置子设备）：


	# echo "sub-dev" > /sys/platform/devices/s5p-fimc-md/subdev_conf_mode

仅用V4L2 视频节点控制（子设备由主机驱动内部配置）


	# echo "vid-dev" > /sys/platform/devices/s5p-fimc-md/subdev_conf_mode

这是默认选项

### 5. 设备到视频和子设备节点的映射


硬件中每个设备实例关联两个视频设备节点——视频捕获和 mem-to-mem，此外还有一个子设备
节点用于更精确的 FIMC 捕获子系统控制。另外，为每MIPI-CSIS 设备创建一个单独的
v4l2 子设备节点

如何查明哪个 /dev/video /dev/v4l-subdev 被分配给哪个设备

你可grep 内核日志来查找相关信息，即：


	# dmesg | grep -i fimc

（注意，如果存在 udev，它可能已经重新排布了视频节点，

或者借助 media-ctl 工具/dev/media 获取信息


	# media-ctl -p

### 7. 构建


如果驱动被构建为可加载内核模块（CONFIG_VIDEO_SAMSUNG_S5P_FIMC=m），会创建两个模
（此外还有核v4l2 模块）：s5p-fimc.ko 和可选的 s5p-csis.ko（MIPI-CSI 接收子设备）
