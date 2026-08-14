
## 概述


本节介绍 CXL Type-3 内存设备的配置过程，以及它最终如何作为 `DAX` 设备或经由内核
页分配器的普通内存页暴露给用户。

以项目符号标记的部分是生成特定内核对象的时间点。

1) 早期启动

  a) BIOS、构建与启动参数

    i) EFI_MEMORY_SP
    ii) CONFIG_EFI_SOFT_RESERVE
    iii) CONFIG_MHP_DEFAULT_ONLINE_TYPE
    iv) nosoftreserve

  b) 内存映射创建

    i) 针对 Soft-Reserved 查阅 EFI 内存映射 / E820

      - CXL 内存被预留出来由 CXL 驱动处理

      - 为 CFMWS 条目创建 Soft-Reserved IO 资源

  c) NUMA 节点创建

    - 从 ACPI CEDT CFMWS 与 SRAT 邻近域（PXM）创建节点

  d) 内存分层（Memory Tier）创建

    - 使用所有节点创建一个默认的 memory_tier

  e) 连续内存分配

    - 任何请求的 CMA 都从在线节点分配

  f) 初始化结束，驱动开始探测

2) ACPI 与 PCI 驱动

  a) 检测到 PCI 设备是 CXL，将其标记为交由 CXL 驱动探测

3) CXL 驱动操作

  a) 基础设备创建

    - 创建 root、port 与 memdev 设备
    - 创建 CEDT CFMWS IO 资源

  b) 解码器（Decoder）创建

    - 创建 root、switch 与 endpoint 解码器

  c) 逻辑设备创建

    - 创建 memory_region 与 endpoint 设备

  d) 设备相互关联

    - 如果是 auto-decoder（BIOS 编程的解码器），驱动在探测时验证配置、建立关联并
      锁定配置。

    - 如果是用户配置的，验证与关联在 decoder-commit 时建立。

  e) 区域作为 DAX 区域呈现

    - 创建 dax_region

    - 通过 DAX 驱动创建 DAX 设备

4) DAX 驱动操作

  a) DAX 驱动将 DAX 区域以两种 dax 设备模式之一呈现

    - kmem - dax 设备被转换为热插拔内存块

      - 创建 DAX kmem IO 资源

    - hmem - dax 设备保留为 daxdev，作为文件访问。

      - 如果是 hmem，流程在此结束。

  b) DAX kmem 将内存区域呈现给 Memory Hotplug，以作为“驱动管理内存”加入页分配器

5) 内存热插拔（Memory Hotplug）

  a) mhp 组件将一个 dax 设备内存区域作为多个内存块呈现给页分配器

    - 这些块出现在 `/sys/bus/memory/devices` 中，并链接到一个 NUMA 节点

  b) 这些块被上线到所请求的区（NORMAL 或 MOVABLE）

    - 内存被标记为“Driver Managed”，以避免 kexec 将其用作内核更新的区域
