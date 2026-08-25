
## 简

Linux 计算加速器（compute accelerator）子系统旨在以统一的方式向用户空间
暴露计算加速器，并提供一组通用的功能
这些设备既可以是独立ASIC，也可以SoC/GPU 内部IP 模块。尽管这些设通常设计用于加速机器学习（ML）和/或深度学习（DL）计算，accel 层并不限处理这类加速器
通常，一个计算加速器会属于以下类别之一
- 边缘 AI（Edge AI）——在边缘设备上进行推理。它可以是一个嵌入式 ASIC/FPGA  SoC 内部IP（例如笔记本电脑的摄像头）。这类设备通常通过寄存器配置，
  并且可以在有或没DMA 的情况下工作
- 推理数据中心（Inference data-center）——大型服务器中单用户/多用户的设备  这类设备可以是独立的，也可以SoC GPU 内部IP。它带有板载 DRAM
  （用于存DL 拓扑）、DMA 引擎以及命令提交队列（内核态或用户态队列）  它可能还带有用于管理多用户的 MMU，并可能启用虚拟化（SR-IOV）以在同一设备
  上支持多个虚拟机。此外，这些设备通常还会带有一些工具，例如性能分析  （profiler）和调试器
- 训练数据中心（Training data-center）——与推理数据中心卡类似，但通常具有
  更强的算力和内存带宽（例HBM），并且很可能具备扩缩（scale-up/out）手段，
  即分别连接到服务器内或服务器间的其它训练卡
所有这些设备通常都有各自定制的运行时用户空间软件栈，专门适配其硬件。此外，
它们很可能还包含一个编译器，用于为其定制计算引擎生成程序。通常，用户空间中
的通用层是 DL 框架，例PyTorch TensorFlow
## DRM 共享代码


由于这类设备可以GPU 内部IP，或具有GPU 类似的特征，accel 子系统将
复用 DRM 子系统的代码与功能。也就是说，accel 核心代码将成DRM 子系的一部分，而一accel 设备将是一种新型的 DRM 设备
这将使我们能够利用庞大的 DRM 代码库，并与具有此类设备经验DRM 开发者协作此外，为加速器驱动新增的特性也可能GPU 驱动有用
## GPU 的区

因为我们希望避免庞大的用户空间图形软件栈试图将加速器当作 GPU 来使用，计算
加速器将通过新的主设备号（major number）和新的字符设备文件GPU 区分开来
此外，这些驱动将位于内核树中一个独立的位置——drivers/accel/
加速器设备将以专用261 主设备号暴露给用户空间，并遵循以下约定：

- 字符设备文件 - /dev/accel/accel\*
- sysfs             - /sys/class/accel/accel\*/
- debugfs           - /sys/kernel/debug/accel/\*/

## 入门


首先，阅Documentation/gpu/index.rst 中的 DRM 文档。它不仅会说明如何编一个新DRM 驱动，还会包含关于如何贡献、行为准则（Code Of Conduct）以编码风格/文档的全部信息。所有这些对 accel 子系统同样适用
其次，确保内核配置了 CONFIG_DRM_ACCEL
要将你的设备作为加速器暴露，需要在驱动中（相对于标DRM 驱动）做两处修改
- 在你drm_driver driver_features 字段中添DRIVER_COMPUTE_ACCEL
  特性标志。需要注意，该驱动特性与 DRIVER_RENDER DRIVER_MODESET 互斥  希望同时暴露图形和计算的字符设备文件的设备，应由通过 auxiliary bus
  框架连接的两个驱动来处理
- 将驱fops 结构中的 open 回调改为 accel_open()。或者，你的驱动可以使用
  DEFINE_DRM_ACCEL_FOPS 宏来轻松设置正确的函数操作指针结构
## 外部参

### 邮件列表讨论


- `Initial discussion on the New subsystem for acceleration devices <https://lore.kernel.org/lkml/CAFCwf11=9qpNAepL7NL+YAV_QO=Wv6pnWPhKHKAepK3fNn+2Dg@mail.gmail.com/>`_ - Oded Gabbay (2022)
- `patch-set to add the new subsystem <https://lore.kernel.org/lkml/20221022214622.18042-1-ogabbay@kernel.org/>`_ - Oded Gabbay (2022)

### 会议演讲


- `LPC 2022 Accelerators BOF outcomes summary <https://airlied.blogspot.com/2022/09/accelerators-bof-outcomes-summary.html>`_ - Dave Airlie (2022)
