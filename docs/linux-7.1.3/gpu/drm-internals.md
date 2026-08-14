## DRM 内部机制


本章记录与驱动作者以及那些为现有驱动添加最新特性支持的开发者相关的 DRM 内部机制。

首先，我们回顾一些典型的驱动初始化要求，例如建立命令缓冲区、创建初始输出配置，以及初始化核心服务。后续章节将更详细地介绍核心内部机制，并提供实现说明与示例。

DRM 层为图形驱动提供若干服务，其中许多由它通过 libdrm（封装了大部分 DRM ioctl 的库）所提供的应用程序接口驱动。这些服务包括 vblank 事件处理、内存管理、输出管理、帧缓冲管理、命令提交与栅栏（fencing）、挂起/恢复支持，以及 DMA 服务。

## 驱动初始化


每个 DRM 驱动的核心都是一个 :c:type:`struct drm_driver <drm_driver>` 结构体。驱动通常会静态初始化一个 drm_driver 结构体，然后将其传给 drm_dev_alloc() 以分配一个设备实例。在设备实例完全初始化之后，就可以使用 drm_dev_register() 将其注册（注册后用户空间即可访问）。

`struct drm_driver <drm_driver>` 结构体包含描述驱动及其所支持特性的静态信息，以及供 DRM 核心调用以实现 DRM API 的方法指针。我们将首先通览 :c:type:`struct drm_driver <drm_driver>` 的静态信息字段，然后在后续章节用到各个操作时再详细描述它们。

### 驱动信息


#### 主版本号、次版本号与补丁级别


int major; int minor; int patchlevel;
DRM 核心通过一个主版本号、次版本号与补丁级别的三元组来标识驱动版本。该信息会在初始化时打印到内核日志，并通过 DRM_IOCTL_VERSION ioctl 传给用户空间。

主版本号与次版本号也用于校验传给 DRM_IOCTL_SET_VERSION 的所请求驱动 API 版本。当驱动 API 在不同次版本之间发生变化时，应用程序可以调用 DRM_IOCTL_SET_VERSION 来选择某个特定的 API 版本。如果所请求的主版本号与驱动主版本号不一致，或者所请求的次版本号大于驱动次版本号，DRM_IOCTL_SET_VERSION 调用将返回错误。否则将以所请求的版本调用驱动的 set_version() 方法。

#### 名称与描述


char \**name; char \**desc; char \*date;
驱动名称会在初始化时打印到内核日志，用于 IRQ 注册，并通过 DRM_IOCTL_VERSION 传给用户空间。

驱动描述是一个纯信息性的字符串，通过 DRM_IOCTL_VERSION ioctl 传给用户空间，内核本身不再使用它。

### 模块初始化


   :doc: overview

### 设备实例与驱动处理


   :doc: driver instance overview

   :internal:

   :internal:

   :export:

### 驱动加载


#### 组件辅助（Component Helper）用法


   :doc: component helper usage recommendations

#### 内存管理器初始化


每个 DRM 驱动都需要一个内存管理器，并且必须在加载时初始化。DRM 目前包含两个内存管理器：转换表管理器（TTM，Translation Table Manager）与图形执行管理器（GEM，Graphics Execution Manager）。本文档仅描述 GEM 内存管理器的使用。详见 ? 。

#### 杂项设备配置


在配置 PCI 设备时，另一项可能需要的任务是映射视频 BIOS（VBIOS）。在许多设备上，VBIOS 描述了设备配置、LCD 面板时序（如果有），并包含指示设备状态的标志位。映射 BIOS 可以使用 pci_map_rom() 调用，这是一个便捷函数，负责映射实际的 ROM——无论它是被影子复制到内存中（通常在地址 0xc0000），还是存在于 PCI 设备的 ROM BAR 中。注意：在 ROM 被映射且提取了任何必要信息之后，应当将其取消映射；在许多设备上，ROM 地址解码器与其他 BAR 共享，因此让其保持映射可能导致挂起或内存损坏等不良行为。

### 受管理资源


   :doc: managed resources

   :export:

   :internal:

## 打开/关闭、文件操作与 IOCTL


### 文件操作


   :doc: file operations

   :internal:

   :export:

## 杂项工具


### 打印器


   :doc: print

   :internal:

   :export:

### 工具函数


   :doc: drm utils

   :internal:


## 单元测试


### KUnit


KUnit（内核单元测试框架）为 Linux 内核中的单元测试提供了一个通用框架。本节介绍 DRM 子系统的具体内容。有关 KUnit 的一般信息，请参阅 Documentation/dev-tools/kunit/start.rst。

#### 如何运行测试？


为便于运行测试套件，`drivers/gpu/drm/tests/.kunitconfig` 中提供了一个配置文件。它可以按如下方式被 `kunit.py` 使用：


	$ ./tools/testing/kunit/kunit.py run --kunitconfig=drivers/gpu/drm/tests \
		--kconfig_add CONFIG_VIRTIO_UML=y \
		--kconfig_add CONFIG_UML_PCI_OVER_VIRTIO=y

	`.kunitconfig` 中包含的配置应当尽可能通用。`CONFIG_VIRTIO_UML` 与
	`CONFIG_UML_PCI_OVER_VIRTIO` 未被包含在内，因为它们仅用于用户模式
	Linux（User Mode Linux）。

#### KUnit 覆盖规则


KUnit 支持正逐步加入到 DRM 框架与辅助函数中。目前对框架和辅助函数并没有必须拥有 KUnit 测试的普遍要求。不过，如果某个补丁影响到已被 KUnit 测试覆盖的函数或辅助函数，且改动需要相应测试，则必须提供测试。

## 旧版支持代码


本节非常简要地介绍一些旧版支持代码，它们仅被那些对底层设备做了所谓 shadow-attach（影子附加）、而非注册为真正驱动的旧 DRM 驱动使用。这也包括一些旧的通用缓冲区管理与命令提交代码。在新的现代驱动中不要使用其中任何内容。

### 旧版挂起/恢复


DRM 核心提供了一些挂起/恢复代码，但想要完整挂起/恢复支持的驱动应当提供 save() 与 restore() 函数。它们会在挂起、休眠或恢复时被调用，并应执行设备在跨挂起或休眠状态时所要求的任何状态保存或恢复。

int (\**suspend) (struct drm_device \**, pm_message_t state); int
(\**resume) (struct drm_device \**);
这些是旧版挂起与恢复方法，**仅**与旧版 shadow-attach 驱动注册函数配合使用。新驱动应当使用其总线类型所提供的电源管理接口（通常通过 `struct device_driver <device_driver>` 的 dev_pm_ops），并将这些方法设为 NULL。

### 旧版 DMA 服务


这里应当介绍核心如何支持 DMA 映射等。这些函数已被弃用，不应使用。
