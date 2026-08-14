
## CXL 驱动操作


```

  /sys/bus/cxl/devices/
  /dev/cxl/

```
`cxl-cli` 库作为 NDTCL 项目的一部分进行维护，可用于编写与这些设备交互的脚本。

## 驱动

CXL 驱动被拆分为多个驱动。

- cxl_core  - 基础初始化接口与核心对象创建
- cxl_port  - 初始化根并提供端口枚举接口。
- cxl_acpi  - 初始化根解码器并与 ACPI 数据交互。
- cxl_p/mem - 初始化内存设备
- cxl_pci   - 使用 cxl_port 枚举实际的 fabric 层级结构。

## 驱动设备

下面是一个来自单路（single-socket）系统、带有 4 个主机桥（host bridge）的示例。其中两个主机桥各挂载了一个内存设备，且这些设备被交错（interleaved）
```

  # ls /sys/bus/cxl/devices/
    dax_region0  decoder3.0  decoder6.0  mem0   port3
    decoder0.0   decoder4.0  decoder6.1  mem1   port4
    decoder1.0   decoder5.0  endpoint5   port1  region0
    decoder2.0   decoder5.1  endpoint6   port2  root0


```
   :alt: 描述主机桥交错的 CXL fabric 有向图
   :caption: 带有主机桥交错内存区域的 CXL fabric 有向图

   digraph foo {
     "root0" -> "port1";
     "root0" -> "port3";
     "root0" -> "decoder0.0";
     "port1" -> "endpoint5";
     "port3" -> "endpoint6";
     "port1" -> "decoder1.0";
     "port3" -> "decoder3.0";
     "endpoint5" -> "decoder5.0";
     "endpoint6" -> "decoder6.0";
     "decoder0.0" -> "region0";
     "decoder0.0" -> "decoder1.0";
     "decoder0.0" -> "decoder3.0";
     "decoder1.0" -> "decoder5.0";
     "decoder3.0" -> "decoder6.0";
     "decoder5.0" -> "region0";
     "decoder6.0" -> "region0";
     "region0" -> "dax_region0";
     "dax_region0" -> "dax0.0";
   }

本节我们将探索此配置中存在的设备，但更多配置将在下面的示例配置中深入讨论。

### 基础设备

CXL fabric 中的大多数设备都是某种类型的 `port`（因为每个设备主要是将请求从一个设备路由到下一个，而非提供直接服务）。

#### Root

`CXL Root` 是一个逻辑对象，由 `cxl_acpi` 驱动在 `cxl_acpi_probe` 期间创建——前提是找到了 `ACPI0017` `Compute Express Link
Root Object`（根对象）设备类。

Root 包含指向以下对象的链接：

- 由 CHBS 在 [CEDT<../platform/acpi/cedt>](CEDT<../platform/acpi/cedt>) 中定义的 `Host Bridge Ports`

- 通常连接到 `Host Bridge Ports` 的 `Downstream Ports`。

- 由 CFMWS 在 [CEDT<../platform/acpi/cedt>](CEDT<../platform/acpi/cedt>) 中定义的 `Root Decoders`

```

  # ls /sys/bus/cxl/devices/root0
    decoder0.0          dport0  dport5    port2  subsystem
    decoders_committed  dport1  modalias  port3  uevent
    devtype             dport4  port1     port4  uport

  # cat /sys/bus/cxl/devices/root0/devtype
    cxl_port

  # cat port1/devtype
    cxl_port

  # cat decoder0.0/devtype
    cxl_decoder_root

```
root 是由 Linux CXL 驱动呈现的 CXL fabric 中第一个 `logical port`（逻辑端口）。`CXL root` 是一种特殊类型的 `switch port`（交换端口），因为它只有下游端口连接。

#### Port

`port` 对象更准确地被描述为一个 `switch port`（交换端口）。它可以表示一个到 root 的主机桥，或者交换机上的一个实际交换端口。一个 `switch port` 包含一个或多个解码器，用于将内存请求路由到下游端口，这些下游端口可能连接到另一个 `switch port` 或一个 `endpoint port`。

```

  # ls /sys/bus/cxl/devices/port1
    decoder1.0          dport0    driver     parent_dport  uport
    decoders_committed  dport113  endpoint5  subsystem
    devtype             dport2    modalias   uevent

  # cat devtype
    cxl_port

  # cat decoder1.0/devtype
    cxl_decoder_switch

  # cat endpoint5/devtype
    cxl_port

```
CXL fabric 中的 `Host Bridges` 在探测 `CXL Root` 的同时，于 `cxl_acpi_probe` 期间被探测。这使得 root 与主机桥之间能够立即建立逻辑连接。

- root 有一个到主机桥的下游端口连接

- 主机桥有一个到 root 的上游端口连接。

- 主机桥有一个或多个到交换机或端点端口的下游端口连接。

`Host Bridge` 是一种特殊类型的 CXL `switch port`。它在 ACPI 规范中通过 `ACPI0016` ID 显式定义。`Host Bridge` 端口将在 `acpi_probe` 时被探测，而实际交换机上的类似端口将在稍后被探测。除此之外，交换机端口与主机桥端口看起来非常相似——它们都包含用于在上下游端口之间路由访问的交换机解码器。

#### Endpoint

`endpoint` 是 fabric 中的一个终端端口。它是一个 `logical device`（逻辑设备），并且可能是由某个内存设备呈现的众多 `logical devices` 之一。在 fabric 中它仍被视为一种 `port`。

一个 `endpoint` 包含 `endpoint decoders`（端点解码器）以及设备的 Coherent Device
```

  # ls /sys/bus/cxl/devices/endpoint5
    CDAT        decoders_committed  modalias      uevent
    decoder5.0  devtype             parent_dport  uport
    decoder5.1  driver              subsystem

  # cat /sys/bus/cxl/devices/endpoint5/devtype
    cxl_port

  # cat /sys/bus/cxl/devices/endpoint5/decoder5.0/devtype
    cxl_decoder_endpoint


```
#### Memory Device（memdev）

`memdev` 由 `cxl_pci` 驱动在 `cxl_pci_probe` 中探测并添加，并由 `cxl_mem` 驱动管理。它主要通过 `/dev/cxl/memN` 提供到内存设备的 `IOCTL` 接口，并暴露各种
```

  # ls /sys/bus/cxl/devices/mem0
    dev       firmware_version    payload_max  security   uevent
    driver    label_storage_size  pmem         serial
    firmware  numa_node           ram          subsystem

```
一个 Memory Device 是一个不属端口类型的离散基础对象。虽然它所属的物理设备也可能承载一个 `endpoint`，但 `endpoint` 与 `memdev` 之间的关系并未在 sysfs 中体现。

#### Port Relationships

在上述示例中，有四个主机桥连接到 root，其中两个主机桥各挂载了一个端点。

   :alt: 描述主机桥交错的 CXL fabric 有向图
   :caption: 带有主机桥交错内存区域的 CXL fabric 有向图

   digraph foo {
     "root0"    -> "port1";
     "root0"    -> "port2";
     "root0"    -> "port3";
     "root0"    -> "port4";
     "port1" -> "endpoint5";
     "port3" -> "endpoint6";
   }

### Decoders

`Decoder`（解码器）是 CXL Host-Managed Device Memory（HDM，主机管理设备内存）Decoder 的简称。它是一个将访问通过 CXL fabric 路由到端点、并在端点处将 `Host Physical`（主机物理地址）转换为 `Device Physical`（设备物理地址）寻址的设备。

CXL 3.1 规范强烈暗示只有端点解码器才应参与 `Host Physical Address` 到 `Device Physical Address` 的转换。
```

  8.2.4.20 CXL HDM Decoder Capability Structure

  IMPLEMENTATION NOTE
  CXL Host Bridge and Upstream Switch Port Decode Flow

  IMPLEMENTATION NOTE
  Device Decode Logic

```
这些注记暗示存在两个逻辑的解码器分组。

- Routing Decoder（路由解码器）- 仅路由访问但不翻译地址（从 HPA 到 DPA）的解码器。

- Translating Decoder（转换解码器）- 为端点服务而将访问从 HPA 转换为 DPA 的解码器。

CXL 驱动区分 3 种解码器类型：root、switch 和 endpoint。只有端点解码器是 Translating Decoder（转换解码器），其余都是 Routing Decoder（路由解码器）。


   Linux 强烈假设端点解码器是 fabric 中唯一主动将 HPA 转换为 DPA 的解码器。Linux 假设路由解码器将 HPA 原样传递给 fabric 中的下一个解码器。

   因此，假设 fabric 中任何给定的解码器的地址范围都是其上游端口解码器地址范围的子集。对此方案的任何偏离在规范中都属于未定义行为。Linux 优先采用规范定义/架构定义的行为。

解码器如果配置为交错内存访问，则可能具有一个或多个 `Downstream Targets`（下游目标）。这将通过 `target_list` 参数在 sysfs 中呈现。

#### Root Decoder

`Root Decoder` 是 :doc:`CEDT
<../platform/acpi/cedt>` 中 CFMWS 字段所表示的物理地址与交错配置的逻辑构造。
Linux 将此信息呈现为存在于 `CXL Root` 中的一个解码器。我们将其视为一个 `Root Decoder`，尽管严格来说它存在于 CXL 规范与平台相关的 CXL root 实现的边界上。

Linux 将这些逻辑解码器视为一种 `Routing Decoder`（路由解码器），并且是 CXL fabric 中第一个接收来自平台内存控制器的内存访问的解码器。

`Root Decoders` 在 `cxl_acpi_probe` 期间创建。每个 CFMWS 条目在 [CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 中创建一个 root 解码器。

`target_list` 参数由 CFMWS 的 target 字段填充。root 解码器的目标是 `Host Bridges`（主机桥），这意味着在 root 解码器级别完成的交错是一种 `Inter-Host-Bridge Interleave`（主机桥间交错）。

只有 root 解码器能够进行 `Inter-Host-Bridge Interleave`（主机桥间交错）。

此类交错必须由平台配置，并描述在 ACPI CEDT CFMWS 中，因为 CFMWS 中的目标 CXL 主机桥 UID 必须与 :doc:`CEDT
<../platform/acpi/cedt>` 的 CHBS 字段中的 CXL 主机桥 UID，以及 [DSDT <../platform/acpi/dsdt>](DSDT <../platform/acpi/dsdt>) 中定义的 CXL 主机桥 UID 字段相匹配。

root 解码器中的交错设置描述的是如何在**直接下游目标**之间交错访问，而非整个交错集合。

root 解码器描述的内存范围用于

1) 创建一个内存区域（本例中为 `region0`），以及

2) 将该区域与一个 IO Memory Resource（`kernel/resource.c`）关联

```

  # ls /sys/bus/cxl/devices/decoder0.0/
    cap_pmem           devtype                 region0
    cap_ram            interleave_granularity  size
    cap_type2          interleave_ways         start
    cap_type3          locked                  subsystem
    create_ram_region  modalias                target_list
    delete_region      qos_class               uevent

  # cat /sys/bus/cxl/devices/decoder0.0/region0/resource
    0xc050000000

```
IO Memory Resource 在早期引导期间创建，此时在 EFI Memory Map 或 E820 表（在 x86 上）中识别到 CFMWS 区域。

Root 解码器被定义为一个独立的 devtype，但它同时也是某种类型
```

  # cat /sys/bus/cxl/devices/decoder0.0/devtype
    cxl_decoder_root

```
#### Switch Decoder

任何非 root 的、进行转换的解码器都被视为 `Switch Decoder`（交换机解码器），并呈现为 `cxl_decoder_switch` 类型。`Host Bridge` 和 `CXL
```

  # ls /sys/bus/cxl/devices/decoder1.0/
    devtype                 locked    size       target_list
    interleave_granularity  modalias  start      target_type
    interleave_ways         region    subsystem  uevent

  # cat /sys/bus/cxl/devices/decoder1.0/devtype
    cxl_decoder_switch

  # cat /sys/bus/cxl/devices/decoder1.0/region
    region0

```
`Switch Decoder` 建立了由 root 解码器定义的区域与下游目标端口之间的关联。在交换机解码器内部完成的交错是多下游端口交错（对于主机桥则是 `Intra-Host-Bridge Interleave`，主机桥内交错）。

交换机解码器中的交错设置描述的是如何在**直接下游目标**之间交错访问，而非整个交错集合。

交换机解码器在 `cxl_port` 驱动的 `cxl_switch_port_probe` 期间创建，并基于 PCI 设备的 DVSEC 寄存器创建。

交换机解码器编程在探测期间进行验证（如果平台在引导时对其进行了编程，见下文 `Auto Decoders`），或在提交时进行验证（如果在运行时编程，见下文 `Runtime Programming`）。

#### Endpoint Decoder

任何连接到 CXL fabric 中**终端**点（`An Endpoint`）的解码器都被视为 `Endpoint Decoder`（端点解码器）。端点解码器的类型为
```

  # ls /sys/bus/cxl/devices/decoder5.0
    devtype                 locked    start
    dpa_resource            modalias  subsystem
    dpa_size                mode      target_type
    interleave_granularity  region    uevent
    interleave_ways         size

  # cat /sys/bus/cxl/devices/decoder5.0/devtype
    cxl_decoder_endpoint

  # cat /sys/bus/cxl/devices/decoder5.0/region
    region0

```
`Endpoint Decoder` 与由 root 解码器定义的区域相关联，并描述与该区域关联的设备本地资源。

与 root 和交换机解码器不同，端点解码器将 `Host Physical`（主机物理地址）转换为 `Device Physical`（设备物理地址）地址范围。因此端点上的交错设置描述的是整个**交错集合**。

`Device Physical Address`（设备物理地址）区域必须按顺序提交。例如，起始于 0x80000000 的 DPA 区域不能在起始于 0x0 的 DPA 区域之前提交。

自 Linux v6.15 起，Linux 不支持**不平衡**的交错配置，交错集合中的所有端点都应具有相同的交错设置（granularity 与 ways 必须相同）。

端点解码器在 `cxl_port` 驱动的 `cxl_endpoint_port_probe` 期间创建，并基于 PCI 设备的 DVSEC 寄存器创建。

#### Decoder Relationships

在上述示例中，存在一个 root 解码器，它通过两个主机桥路由内存访问。每个主机桥有一个解码器，将访问路由到其唯一的端点目标。每个端点有一个解码器，将 HPA 转换为 DPA 并服务于内存请求。

驱动通过解码器编程验证端口之间的关系，因此我们可以将解码器之间的关系视为与端口类似的层级结构。

   :alt: root、switch 与 endpoint 解码器之间层级关系的有向图。
   :caption: CXL root、switch 与 endpoint 解码器的有向图。

   digraph foo {
     "root0"    -> "decoder0.0";
     "decoder0.0" -> "decoder1.0";
     "decoder0.0" -> "decoder3.0";
     "decoder1.0" -> "decoder5.0";
     "decoder3.0" -> "decoder6.0";
   }

### Regions

#### Memory Region

`Memory Region`（内存区域）是一个逻辑构造，它将 fabric 中的一组 CXL 端口连接到一个 IO Memory Resource。它最终用于通过 `DAX Region` 将这些设备上的内存暴露给 DAX 子系统。

```

  # ls /sys/bus/cxl/devices/region0/
    access0      devtype                 modalias  subsystem  uuid
    access1      driver                  mode      target0
    commit       interleave_granularity  resource  target1
    dax_region0  interleave_ways         size      uevent

```
一个内存区域可以在端点探测期间构造（如果解码器由 BIOS/EFI 编程，见 `Auto Decoders`），或者通过 `Root Decoder` 的 `create_ram_region` 或 `create_pmem_region` 接口手动创建。

`Memory Region` 中的交错设置描述了 `Interleave Set`（交错集合）的配置——也就是在端点交错设置中所能预期看到的内容。

   :alt: root 与 endpoint 解码器之间 CXL 内存区域关系的有向图。
   :caption: 区域基于 root 解码器配置创建。端点解码器必须使用与区域相同的交错设置进行编程。

   digraph foo {
     "root0"    -> "decoder0.0";
     "decoder0.0" -> "region0";
     "region0" -> "decoder5.0";
     "region0" -> "decoder6.0";
   }

#### DAX Region

`DAX Region` 用于将一个 CXL `Memory Region` 转换为一个 DAX 设备。随后可通过文件描述符接口直接访问该 DAX 设备，或通过 DAX kmem 驱动转换为 System RAM。参见 DAX 驱动小节
```

  # ls /sys/bus/cxl/devices/dax_region0/
    dax0.0      devtype  modalias   uevent
    dax_region  driver   subsystem

```
### Mailbox Interfaces

```

  /dev/cxl/mem0
  /dev/cxl/mem1

```
这些邮箱可以接收任何规范定义的命令。原始命令（自定义命令）只有在构建配置 `CXL_MEM_RAW_COMMANDS` 被设置时才能发送到这些接口。这被视为一个调试和/或开发接口，并非用于创建厂商特定命令的官方支持机制（相关请参见 `fwctl` 子系统）。

## Decoder Programming

### Runtime Programming

在探测期间，**必须**编程的解码器只有 `Root Decoders`。实际上，`Root Decoders` 是描述主机桥级别内存区域与交错配置的逻辑构造——如 ACPI CEDT CFMWS 中所述。

所有其他 `Switch` 与 `Endpoint` 解码器都可以在运行时由用户编程——前提是平台支持此类配置。

这种交互创造了 `Software Defined Memory`（软件定义内存）环境。

有关如何在运行时配置 CXL 解码器的更多信息，请参阅 `cxl-cli` 文档。

### Auto Decoders

Auto Decoders 是由 BIOS/EFI 在引导时编程的解码器，几乎总是被锁定（不可更改）。这是由可能具有静态配置的平台完成的——或者某些怪异特性可能阻止对解码器进行动态运行时更改（例如在 CXL 范围之外的 CPU 复合体内需要额外的控制器编程）。

只要 Auto Decoders 所关联的设备与内存区域能够无问题地探测，它们就会自动被探测。在探测 Auto Decoders 时，驱动的主要职责是确保 fabric 状态正常（sane）——如同验证运行时编程的区域与解码器一样。

如果 Linux 无法验证 auto-decoder 配置，该内存将不会被作为 DAX 设备呈现——因此也不会暴露给页分配器——实际上被搁置（stranding）了。

### Interleave

Linux CXL 驱动支持 `Cross-Link First`（交叉链路优先）交错。这规定了在每个解码器步骤如何编程交错，因为驱动会验证解码器与其父级之间的关系。

例如，在一个 `Cross-Link First` 交错配置中，16 个端点连接到 4 个主机桥，Linux 期望在 root、主机桥和端点上分别有如下的 ways/granularity：


  - - decoder
    - ways
    - granularity

  - - root
    - 4
    - 256

  - - host bridge
    - 4
    - 1024

  - - endpoint
    - 16
    - 256

在 root 级别，每次给定的访问将被路由到 `((HPA / 256) % 4)` 号目标主机桥。在主机桥内，路由到 `((HPA / 1024) % 4)` 号目标端点。每个端点基于整个 16 设备交错集合进行转换。

不支持不平衡的交错集合——层级结构中相似位置的解码器（例如所有主机桥解码器）必须具有相同的 ways 与 granularity 配置。

#### At Root

Root 解码器交错由 :doc:`CEDT
<../platform/acpi/cedt>` 的 CFMWS 字段定义。CEDT 实际上可能定义多个 CFMWS 配置来描述相同的物理容量，意图是允许用户在运行时决定是将内存作为交错方式上线，还是
```

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000100000000
               Window size : 0000000100000000
  Interleave Members (2^n) : 00
     Interleave Arithmetic : 00
              First Target : 00000007

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000200000000
               Window size : 0000000100000000
  Interleave Members (2^n) : 00
     Interleave Arithmetic : 00
              First Target : 00000006

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000300000000
               Window size : 0000000200000000
  Interleave Members (2^n) : 01
     Interleave Arithmetic : 00
              First Target : 00000007
               Next Target : 00000006

```
在本例中，CFMWS 为每个主机桥定义了两个离散的非交错 4GB 区域，以及一个以两者为目标的 8GB 交错区域。这
```

  # ls /sys/bus/cxl/devices/root0/decoder*
    decoder0.0  decoder0.1  decoder0.2

  # cat /sys/bus/cxl/devices/decoder0.0/target_list start size
    7
    0x100000000
    0x100000000

  # cat /sys/bus/cxl/devices/decoder0.1/target_list start size
    6
    0x200000000
    0x100000000

  # cat /sys/bus/cxl/devices/decoder0.2/target_list start size
    7,6
    0x300000000
    0x200000000

```
这些解码器不可在运行时编程。它们用于生成一个 `Memory Region`，以便通过 `Switch` 与 `Endpoint` 解码器上运行时编程的设置将此内存上线。

#### At Host Bridge or Switch

`Host Bridge` 与 `Switch` 解码器可通过以下字段编程：

- `start` - 与内存区域关联的 HPA 区域
- `size` - 区域的大小
- `target_list` - 下游端口列表
- `interleave_ways` - 要交错跨越的下游端口数量
- `interleave_granularity` - 交错粒度。

Linux 期望交换机解码器的 `interleave_granularity` 由其上游端口连接推导而来。在 `Cross-Link First` 交错配置中，解码器的 `interleave_granularity` 等于 `parent_interleave_granularity * parent_interleave_ways`。

#### At Endpoint

`Endpoint Decoders` 的编程方式与 Host Bridge 和 Switch 解码器类似，不同之处在于 ways 与 granularity 由交错集合定义（例如由相关联的 `Memory Region` 定义的的交错设置）。

- `start` - 与内存区域关联的 HPA 区域
- `size` - 区域的大小
- `interleave_ways` - 交错集合中的端点数量
- `interleave_granularity` - 交错粒度。

这些设置被端点解码器用于将从 HPA **翻译**为 DPA 的内存请求。这就是为什么它们必须了解整个交错集合。

Linux 不支持不平衡的交错配置。因此，交错集合中的所有端点必须具有相同的 ways 与 granularity。

## Example Configurations

- [example-configurations/single-device.rst](example-configurations/single-device.rst)
- [example-configurations/hb-interleave.rst](example-configurations/hb-interleave.rst)
- [example-configurations/intra-hb-interleave.rst](example-configurations/intra-hb-interleave.rst)
- [example-configurations/multi-interleave.rst](example-configurations/multi-interleave.rst)
