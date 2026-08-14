## drm/tegra NVIDIA Tegra GPU 与显示驱动


NVIDIA Tegra SoC 通过 host1x 控制器支持一组显示、图形和视频功能。host1x 将命令流（收集自 CPU 直接提供的 push buffer）通过通道提供给它的客户端。软件或各个模块之间可以使用 syncpoint 进行同步。

在 Tegra124（又称 Tegra K1）之前（不含），drm/tegra 驱动支持内置 GPU，它由 gr2d 和 gr3d 引擎组成。从 Tegra124 开始，GPU 基于 NVIDIA 桌面 GPU 架构，并由 drm/nouveau 驱动支持。

drm/tegra 驱动支持自 Tegra20 以来的 NVIDIA Tegra SoC 系列。它由三部分组成：

  - 一个 host1x 驱动，提供基础设施并访问 host1x 服务。

  - 一个 KMS 驱动，支持显示控制器以及若干输出，例如 RGB、HDMI、DSI 和 DisplayPort。

  - 一组定制的用户空间 IOCTL，可用于通过 host1x 向 GPU 和视频引擎提交任务。

## 驱动基础设施


各种 host1x 客户端需要被绑定在一起，形成一个逻辑设备，以便向用户暴露其功能。支持这一点的基础设施在 host1x 驱动中实现。当一个驱动向该基础设施注册时，它会提供一组 compatible 字符串，指定它需要的设备。该基础设施创建一个逻辑设备，并扫描设备树以寻找匹配的设备节点，将所需的客户端加入一个列表。各个客户端的驱动也向该基础设施注册，并被加入逻辑 host1x 设备。

一旦所有客户端都可用，该基础设施将使用驱动提供的函数来初始化逻辑设备，该函数会设置子系统特定的位，并依次初始化它的每个客户端。

类似地，当某个客户端被注销时，该基础设施将通过回调驱动来销毁逻辑设备，从而确保子系统特定的位被拆除，并且客户端被依次销毁。

### Host1x 基础设施参考


   :export:

### Host1x Syncpoint 参考


   :export:

## KMS 驱动


显示硬件在历代 Tegra SoC 之间保持了很大程度的向后兼容，直到 Tegra186 引入了若干更改，使得用参数化驱动来支持变得困难。

### 显示控制器


Tegra SoC 有两个显示控制器，每个都可以与零个或多个输出相关联。输出也可以共享单个显示控制器，但前提是它们以兼容的显示时序运行。两个显示控制器也可以共享单个 framebuffer，从而允许在两种输出的模式不匹配时进行克隆配置。在 KMS 术语中，显示控制器被建模为一个 CRTC。

在 Tegra186 上，显示控制器的数量已增加到三个。一个显示控制器不再能驱动所有输出。虽然其中两个控制器可以驱动两个 DSI 输出和两个 SOR 输出，但第三个不能驱动任何 DSI。

#### Windows


一个显示控制器控制一组窗口，这些窗口可用于将多个缓冲区合成到屏幕上。虽然可以为各个窗口指定任意 Z 排序（通过编程相应的混合寄存器），但驱动目前不支持这一点。相反，它将假定窗口具有固定的 Z 排序（窗口 A 是根窗口，即最低的，而窗口 B 和 C 叠加在窗口 A 之上）。叠加窗口支持多种像素格式，并可以在扫描输出时自动从 YUV 转换为 RGB。这使它们适合用于显示视频内容。在 KMS 中，每个窗口被建模为一个 plane。每个显示控制器都有一个硬件光标，被暴露为光标 plane。

### 输出


支持的输出类型和数量在历代 Tegra SoC 之间有所不同。所有系列都至少支持 HDMI。虽然较早的系列支持非常简单的 RGB 接口（每个显示控制器一个），但近期的系列不再支持，而是提供标准接口，例如 DSI 和 eDP/DP。

输出被建模为一个复合的 encoder/connector 对。

#### RGB/LVDS


此接口自 Tegra124 起不再可用。它已被更标准的 DSI 和 eDP 接口所取代。

#### HDMI


所有 Tegra SoC 都支持 HDMI。从 Tegra210 开始，HDMI 由通用的 SOR 输出提供，它支持 eDP、DP 和 HDMI。SOR 能够支持 HDMI 2.0，尽管对此的支持目前尚未合入。

#### DSI


虽然 Tegra 自 Tegra30 起就支持 DSI，但控制器在 Tegra114 中以多种方式发生了变化。由于在 Dalmore（Tegra114）之前没有任何公开可用的开发板使用过 DSI，因此 drm/tegra 驱动只支持 Tegra114 及更高版本。

#### eDP/DP


eDP 首次引入于 Tegra124，用于驱动笔记本形态规格的显示面板。Tegra210 增加了完整的 DisplayPort 支持，尽管这目前尚未在 drm/tegra 驱动中实现。

## 用户空间接口


drm/tegra 提供的用户空间接口允许应用程序创建 GEM 缓冲区、访问和控制 syncpoint，以及向 host1x 提交命令流。

### GEM 缓冲区


`DRM_IOCTL_TEGRA_GEM_CREATE` IOCTL 用于创建带有 Tegra 特定标志的 GEM 缓冲区对象。这对于应当被平铺（tiled）的缓冲区，或者被上下颠倒地扫描输出（对 3D 内容有用）的缓冲区很有用。

在 GEM 缓冲区对象创建后，应用程序可以使用 `DRM_IOCTL_TEGRA_GEM_MMAP` IOCTL 返回的 mmap offset 来映射其内存。

### Syncpoints


可以通过执行 `DRM_IOCTL_TEGRA_SYNCPT_READ` IOCTL 来获取 syncpoint 的当前值。使用 `DRM_IOCTL_TEGRA_SYNCPT_INCR` IOCTL 来递增 syncpoint。

用户空间也可以请求在某个 syncpoint 上阻塞。为此，它需要执行 `DRM_IOCTL_TEGRA_SYNCPT_WAIT` IOCTL，指定要等待的 syncpoint 值。当 syncpoint 达到该值或在指定的超时之后，内核将释放该应用程序。

### 命令流提交


在应用程序能够向 host1x 提交命令流之前，它需要使用 `DRM_IOCTL_TEGRA_OPEN_CHANNEL` IOCTL 打开到一个引擎的通道。Client ID 用于标识通道的目标。当不再需要某个通道时，可以使用 `DRM_IOCTL_TEGRA_CLOSE_CHANNEL` IOCTL 关闭它。要检索与某个通道关联的 syncpoint，应用程序可以使用 `DRM_IOCTL_TEGRA_GET_SYNCPT`。

打开通道后，提交命令流就很容易了。应用程序将命令写入支持某个 GEM 缓冲区对象的内存，并将这些命令连同其他各种参数（例如任务提交中使用的 syncpoint 或重定位）一起传递给 `DRM_IOCTL_TEGRA_SUBMIT` IOCTL。
