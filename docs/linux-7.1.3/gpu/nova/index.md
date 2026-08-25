## nova NVIDIA GPU 驱动


nova 驱动项目由两个独立的驱动 nova-core nova-drm 组成，旨在取代基GPU 系统处理器（GSP）的 NVIDIA GPU nouveau 驱动

以下文档同时适用nova-core nova-drm

- [guidelines](guidelines)

## nova-core


nova-core 驱动是基GSP NVIDIA GPU 的核心驱动。nova-core 作为第一级驱动，围绕 GPU 的硬件与固件接口提供抽象，为第二级驱动（vGPU 管理VFIO 驱动nova-drm 驱动）提供通用基础

- [core/guidelines](core/guidelines)
- [core/todo](core/todo)
- [core/vbios](core/vbios)
- [core/devinit](core/devinit)
- [core/fwsec](core/fwsec)
- [core/falcon](core/falcon)
