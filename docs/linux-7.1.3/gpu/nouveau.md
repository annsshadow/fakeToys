##  drm/nouveau NVIDIA GPU 驱动


drm/nouveau 驱动支持广泛的 NVIDIA GPU，涵盖 GeForce、Quadro 和 Tesla 系列，从 NV04 架构到最新的 Turing、Ampere、Ada 系列。

## NVKM: NVIDIA 内核管理器


NVKM 组件是 nouveau 驱动内部的核心抽象层，负责在内核层面管理 NVIDIA GPU 硬件。NVKM 为处理各种 GPU 架构提供了统一接口。

它提供 NVIDIA GPU 在 nouveau 驱动下正常运行所需的资源管理、电源控制、内存处理和命令提交。

NVKM 在抽象硬件复杂性以及为驱动栈上层提供一致的 API 方面发挥着关键作用。

### GSP 支持


   :doc: GSP 消息队列元素

   :doc: GSP 消息处理策略
