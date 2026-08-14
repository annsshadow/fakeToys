## nova NVIDIA GPU 驱动


nova 驱动项目由两个独立的驱动 nova-core 和 nova-drm 组成，旨在取代基于 GPU 系统处理器（GSP）的 NVIDIA GPU 的 nouveau 驱动。

以下文档同时适用于 nova-core 和 nova-drm。

- [guidelines](guidelines)

## nova-core


nova-core 驱动是基于 GSP 的 NVIDIA GPU 的核心驱动。nova-core 作为第一级驱动，围绕 GPU 的硬件与固件接口提供抽象，为第二级驱动（如 vGPU 管理器 VFIO 驱动和 nova-drm 驱动）提供通用基础。

- [core/guidelines](core/guidelines)
- [core/todo](core/todo)
- [core/vbios](core/vbios)
- [core/devinit](core/devinit)
- [core/fwsec](core/fwsec)
- [core/falcon](core/falcon)
