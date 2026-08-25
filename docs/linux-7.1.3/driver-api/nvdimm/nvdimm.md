## LIBNVDIMM：非易失性设
libnvdimm - 内核 / libndctl - 用户空间辅助
nvdimm@lists.linux.dev

版本 13

	术语	概述
	    相关文档
	    Git 代码	LIBNVDIMM PMEM
	    PMEM-REGION、原子扇区与 DAX
	NVDIMM 平台示例
	LIBNVDIMM 内核设备模型LIBNDCTL 用户空间 API
	    LIBNDCTL：上下文
	        libndctl：实例化新的库上下文示例
	    LIBNVDIMM/LIBNDCTL：总线（Bus	        libnvdimmsys/class 中的控制类设	        libnvdimm：总线（bus	        libndctl：总线枚举示例
	    LIBNVDIMM/LIBNDCTL：DIMM（NMEM	        libnvdimm：DIMM（NMEM	        libndctl：DIMM 枚举示例
	    LIBNVDIMM/LIBNDCTL：Region
	        libnvdimm：region
	        libndctl：region 枚举示例
	        为何不把 Region 类型编码Region 名称	        如何确定一Region 的主要类型？
	    LIBNVDIMM/LIBNDCTL：Namespace
	        libnvdimm：namespace
	        libndctl：namespace 枚举示例
	        libndctl：namespace 创建示例
	        为何使用术语 "namespace"	    LIBNVDIMM/LIBNDCTL：块转换"btt"
	        libnvdimm：btt 布局
	        libndctl：btt 创建示例
	LIBNDCTL 图表示意总结

## 术语
PMEM  一个系统物理地址范围，其中的写入是持久化的。由 PMEM 组成的块设备
  能够支持 DAX。一PMEM 地址范围可以跨多DIMM 的交织
DPA  DIMM Physical Address（DIMM 物理地址），是相对于 DIMM 的偏移量  当系统中只有一DIMM 时，系统物理地址DPA 之间1:1 的对应关系  一旦加入更DIMM，就必须对内存控制器交织进行解码，以确定与给  系统物理地址相关联的 DPA
DAX  文件系统扩展，用于绕过页缓存和块层，将来PMEM 块设备的持久化内  直接 mmap 到进程地址空间中
DSM  Device Specific Method（设备特定方法）：用于控制特定设备的 ACPI 方法
  ——在此例中即固件
DCR  NVDIMM Control Region Structure（NVDIMM 控制区域结构），定义ACPI 6
  5.2.25.5 节。它为一个给定的 DIMM 定义vendor-id、device-id 以及
  接口格式
BTT  Block Translation Table（块转换表）：持久化内存是可按字节寻址的  现有的软件可能期望写入的掉电原子性至少为一个扇区，512 字节。BTT   一个具有原子更新语义的重映射表，位PMEM 块设备驱动之前，以呈现任意的
  原子扇区大小
LABEL  存储DIMM 设备上的元数据，用于对分配给不同 PMEM namespace 的容量进  分区并标识（持久命名）。它还指示是否对 namespace 应用了像 BTT 这样  地址抽象。注意，传统的分区表 GPT/MBR 是叠加在 PMEM namespace 之上，或  存在时叠加在BTT 这样的地址抽象之上，但分区支持今后将被弃用
## 概述

LIBNVDIMM 子系统为平台固件或设备驱动所描述PMEM 提供支持。在基于 ACPI 系统上，平台固件通过 ACPI 6 中的 ACPI NFITNVDIMM Firmware Interface
Table"，NVDIMM 固件接口表）传递持久化内存资源。虽LIBNVDIMM 子系统的实现
是通用的并支持 NFIT 之前的平台，但它受到了支持此 ACPI 6 NVDIMM 资源定义
所需能力全集的指导。最初的实现支持 NFIT 中描述的 block-window-aperture（块
窗口孔径）能力，但该支持后来已被放弃，从未在任何产品中发布
### 相关文档

ACPI 6锛?	https://www.uefi.org/sites/default/files/resources/ACPI_6.0.pdf
NVDIMM Namespace锛?	https://pmem.io/documents/NVDIMM_Namespace_Spec.pdf
DSM Interface Example锛?	https://pmem.io/documents/NVDIMM_DSM_Interface_Example.pdf
Driver Writer's Guide锛?	https://pmem.io/documents/NVDIMM_Driver_Writers_Guide.pdf

### Git 代码
LIBNVDIMM锛?	https://git.kernel.org/cgit/linux/kernel/git/nvdimm/nvdimm.git
LIBNDCTL锛?	https://github.com/pmem/ndctl.git

## LIBNVDIMM PMEM

NFIT 出现之前，非易失性内存以各种临时的方式描述给系统。通常只提供最
基本的要素，即一个系统物理地址范围，其中的写入预期在系统掉电后仍然持久现在，NFIT 规范不仅标准化了 PMEM 的描述，还标准化了用于控制和配置平台消息传递入口点
PMEM（nd_pmem.ko）：驱动一个系统物理地址范围。该范围在系统内存中是连续的并且可以跨多DIMM 进行交织（硬件内存控制器条带化）。当进行交织时，平台
可以选择提供哪些 DIMM 参与了该交织的细节
值得注意的是，当检测到标注（labeling）能力时（找到了一EFI namespace
label index block），默认不会创建任何块设备，因为用户空间至少需要对
PMEM 范围进行一DPA 分配。相比之下，一旦注册，ND_NAMESPACE_IO 范围可以
立即挂载nd_pmem。后一种模式称为无标注（label-less）或"legacy"（传统）
### PMEM-REGION、原子扇区与 DAX

对于应用程序或文件系统仍需要原子扇区更新保证的情况，它可以PMEM 设备分区上注册一BTT。参LIBNVDIMM/NDCTL：Block Translation Table "btt"
## NVDIMM 平台示例

本文档的其余部分将使用以下示意图
```

                               (a)               (b)           DIMM
            +-------------------+--------+--------+--------+
  +------+  |       pm0.0       |  free  | pm1.0  |  free  |    0
  | imc0 +--+- - - region0- - - +--------+        +--------+
  +--+---+  |       pm0.0       |  free  | pm1.0  |  free  |    1
     |      +-------------------+--------v        v--------+
  +--+---+                               |                 |
  | cpu0 |                                     region1
  +--+---+                               |                 |
     |      +----------------------------^        ^--------+
  +--+---+  |           free             | pm1.0  |  free  |    2
  | imc1 +--+----------------------------|        +--------+
  +------+  |           free             | pm1.0  |  free  |    3
            +----------------------------+--------+--------+

```
在该平台上，我们在单个插槽中有四DIMM 和两个内存控制器。每PMEM 交织
集由一个具有动态分id region 设备标识
    1. DIMM0 DIMM1 的前半部分作REGION0 交织在一起。一个单一       PMEM namespace 创建REGION0-SPA-range 中，它横跨大部分 DIMM0        DIMM1，用户指定的名称"pm0.0"。部分交织的系统物理地址范围被留       空闲，以便定义另一PMEM namespace
    2. DIMM0 DIMM1 的最后部分，我们有一个交织的系统物理地址范围
       REGION1，它横跨这两DIMM 以及 DIMM2 DIMM3。REGION1 的一部分被分       给一个名"pm1.0" PMEM namespace
    该总线由内核在加载来自 tools/testing/nvdimm nfit_test.ko 模块时，
    于设/sys/devices/platform/nfit_test.0 下提供。该模块LIBNVDIMM     acpi_nfit.ko 驱动的一个单元测试
## LIBNVDIMM 内核设备模型LIBNDCTL 用户空间 API

下面是对 LIBNVDIMM sysfs 布局以及通过 LIBNDCTL API 查看的相应对象层示意图的描述。示sysfs 路径和示意图是相对于 NVDIMM 平台示例的，该示同时也是 LIBNDCTL 单元测试中使用的 LIBNVDIMM 总线
### LIBNDCTL：上下文

LIBNDCTL 库中的每API 调用都需要一context（上下文），它保存日志参数和
其他库实例状态。该库基libabc 模板
	https://git.kernel.org/cgit/linux/kernel/git/kay/libabc.git

##### LIBNDCTL：实例化新的库上下文示例

```
	struct ndctl_ctx *ctx;

	if (ndctl_new(&ctx) == 0)
		return ctx;
	else
		return NULL;
```

### LIBNVDIMM/LIBNDCTL：总线（Bus
一个总线（bus）与一NFIT 之间存在 1:1 的关系。对于基ACPI 的系统，当前
的预期是只有一个平台全局NFIT。也就是说，注册多个 NFIT 是轻而易举的，规并不排除这种情况。该基础设施支持多个总线，我们在单元测试中利用这一能力测试多种 NFIT 配置
### LIBNVDIMMsys/class 中的控制类设
该字符设备接受要传递给 DIMM DSM 消息
```
	/sys/class/nd/ndctl0
	|-- dev
	|-- device -> ../../../ndbus0
	|-- subsystem -> ../../../../../../../class/nd
```

### LIBNVDIMM：总线（bus
```
	struct nvdimm_bus *nvdimm_bus_register(struct device *parent,
	       struct nvdimm_bus_descriptor *nfit_desc);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- commands
	|-- nd
	|-- nfit
	|-- nmem0
	|-- nmem1
	|-- nmem2
	|-- nmem3
	|-- power
	|-- provider
	|-- region0
	|-- region1
	|-- region2
	|-- region3
	|-- region4
	|-- region5
	|-- uevent
	`-- wait_probe
```

##### LIBNDCTL：总线枚举示例

```
	static struct ndctl_bus *get_bus_by_provider(struct ndctl_ctx *ctx,
			const char *provider)
	{
		struct ndctl_bus *bus;

		ndctl_bus_foreach(ctx, bus)
			if (strcmp(provider, ndctl_bus_get_provider(bus)) == 0)
				return bus;

		return NULL;
	}

	bus = get_bus_by_provider(ctx, "nfit_test.0");
```

### LIBNVDIMM/LIBNDCTL：DIMM（NMEM
DIMM 设备提供了一个字符设备用于向硬件发送命令，并且它是 LABEL 的容器。如DIMM NFIT 定义，则提供一个可选的 'nfit' 属性子目录来添NFIT 特有的内容
注意DIMM"的内核设备名"nmemX"。NFIT 通过"Memory Device to System
Physical Address Range Mapping Structure"（内存设备到系统物理地址范围映射
结构）描述这些设备，并且不要求它们实际上必须是物DIMM，因此我们使用了一更通用的名称
##### LIBNVDIMM：DIMM（NMEM
```
	struct nvdimm *nvdimm_create(struct nvdimm_bus *nvdimm_bus, void *provider_data,
			const struct attribute_group **groups, unsigned long flags,
			unsigned long *dsm_mask);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- nmem0
	|   |-- available_slots
	|   |-- commands
	|   |-- dev
	|   |-- devtype
	|   |-- driver -> ../../../../../bus/nd/drivers/nvdimm
	|   |-- modalias
	|   |-- nfit
	|   |   |-- device
	|   |   |-- format
	|   |   |-- handle
	|   |   |-- phys_id
	|   |   |-- rev_id
	|   |   |-- serial
	|   |   `-- vendor
	|   |-- state
	|   |-- subsystem -> ../../../../../bus/nd
	|   `-- uevent
	|-- nmem1
	[..]
```

##### LIBNDCTL：DIMM 枚举示例

注意，在此示例中我们假设的是NFIT 定义DIMM，它们由一32 位值的
"nfit_handle" 标识，其中：

   - Bit 3:0 内存通道内的 DIMM 编号
   - Bit 7:4 内存通道编号
   - Bit 11:8 内存控制ID
   - Bit 15:12 插槽 ID（如果存在节点控制器，则在节点控制器范围内）
   - Bit 27:16 鑺傜偣鎺у埗鍣?ID
   - Bit 31:28 保留

```
	static struct ndctl_dimm *get_dimm_by_handle(struct ndctl_bus *bus,
	       unsigned int handle)
	{
		struct ndctl_dimm *dimm;

		ndctl_dimm_foreach(bus, dimm)
			if (ndctl_dimm_get_handle(dimm) == handle)
				return dimm;

		return NULL;
	}

	#define DIMM_HANDLE(n, s, i, c, d) \
		(((n & 0xfff) << 16) | ((s & 0xf) << 12) | ((i & 0xf) << 8) \
		 | ((c & 0xf) << 4) | (d & 0xf))

	dimm = get_dimm_by_handle(bus, DIMM_HANDLE(0, 0, 0, 0, 0));
```

### LIBNVDIMM/LIBNDCTL：Region

为每PMEM 交织范围注册一个通用REGION 设备。按示例，在 "nfit_test.0"
总线上有 2 PMEM region。region 的主要角色是作为 "mappings"（映射）的容器一mapping 是一个元<DIMM, DPA-start-offset, length>
LIBNVDIMM REGION 设备提供了一个内置驱动。该驱动负责解析所LABEL（如存在），然后发出nd_pmem 驱动使用NAMESPACE 设备
除了 "mapping"interleave_ways"（交织路数）"size"（大小）这些通用属外，REGION 设备还导出了一些便利属性nstype" 指示region 发出namespace 设备的整数类型；"devtype" 复制udev 'add' 事件时存储的
DEVTYPE 变量modalias" 复制udev 'add' 事件时存储的 MODALIAS 变量最后，region SPA 定义的情况下，提供可选的 "spa_index"
```
	struct nd_region *nvdimm_pmem_region_create(struct nvdimm_bus *nvdimm_bus,
			struct nd_region_desc *ndr_desc);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- region0
	|   |-- available_size
	|   |-- btt0
	|   |-- btt_seed
	|   |-- devtype
	|   |-- driver -> ../../../../../bus/nd/drivers/nd_region
	|   |-- init_namespaces
	|   |-- mapping0
	|   |-- mapping1
	|   |-- mappings
	|   |-- modalias
	|   |-- namespace0.0
	|   |-- namespace_seed
	|   |-- numa_node
	|   |-- nfit
	|   |   `-- spa_index
	|   |-- nstype
	|   |-- set_cookie
	|   |-- size
	|   |-- subsystem -> ../../../../../bus/nd
	|   `-- uevent
	|-- region1
	[..]
```

##### LIBNDCTL：region 枚举示例

基于 NFIT 唯一数据（如 "spa_index"，即交织id）的示例 region 检索例程
```
	static struct ndctl_region *get_pmem_region_by_spa_index(struct ndctl_bus *bus,
			unsigned int spa_index)
	{
		struct ndctl_region *region;

		ndctl_region_foreach(bus, region) {
			if (ndctl_region_get_type(region) != ND_DEVICE_REGION_PMEM)
				continue;
			if (ndctl_region_get_spa_index(region) == spa_index)
				return region;
		}
		return NULL;
	}
```

### LIBNVDIMM/LIBNDCTL：Namespace

一REGION 在解析完 DPA 别名LABEL 指定的边界后，会呈现出一个或多个
"namespace" 设备namespace" 设备的出现当前会触发 nd_pmem 驱动加载并注一个磁块设备
##### LIBNVDIMM：namespace

以下是两大类 NAMESPACE 的示例布局，其namespace0.0 代表DIMM 信息支撑PMEM（注意它有一'uuid' 属性），namespace1.0 代表一个匿名的 PMEM
namespace（注意由于没LABEL 支持，它没有 'uuid' 属性）
```
	/sys/devices/platform/nfit_test.0/ndbus0/region0/namespace0.0
	|-- alt_name
	|-- devtype
	|-- dpa_extents
	|-- force_raw
	|-- modalias
	|-- numa_node
	|-- resource
	|-- size
	|-- subsystem -> ../../../../../../bus/nd
	|-- type
	|-- uevent
	`-- uuid
	/sys/devices/platform/nfit_test.1/ndbus1/region1/namespace1.0
	|-- block
	|   `-- pmem0
	|-- devtype
	|-- driver -> ../../../../../../bus/nd/drivers/pmem
	|-- force_raw
	|-- modalias
	|-- numa_node
	|-- resource
	|-- size
	|-- subsystem -> ../../../../../../bus/nd
	|-- type
	`-- uevent
```

##### LIBNDCTL：namespace 枚举示例

Namespace 是相对于其父 region 建立索引的，示例如下。这些索引从启动到启动大是静态的，但子系统在这方面不作任何保证。要获得静态的 namespace 标识符，请使'uuid' 属性
```
  static struct ndctl_namespace
  *get_namespace_by_id(struct ndctl_region *region, unsigned int id)
  {
          struct ndctl_namespace *ndns;

          ndctl_namespace_foreach(region, ndns)
                  if (ndctl_namespace_get_id(ndns) == id)
                          return ndns;

          return NULL;
  }
```

##### LIBNDCTL：namespace 创建示例

如果给定 region 有足够的可用容量来创建新namespace，空闲的 namespace 会由
内核自动创建。namespace 实例化涉及找到一个空namespace 并配置它。在大多情况下，namespace 属性的设置可以以任意顺序进行，唯一的约束是 'uuid' 必须'size' 之前设置。这使得内核能够跟踪 DPA 分配
```
  static int configure_namespace(struct ndctl_region *region,
                  struct ndctl_namespace *ndns,
                  struct namespace_parameters *parameters)
  {
          char devname[50];

          snprintf(devname, sizeof(devname), "namespace%d.%d",
                          ndctl_region_get_id(region), parameters->id);

          ndctl_namespace_set_alt_name(ndns, devname);
          /* 'uuid' 必须在设size 之前设置*/
          ndctl_namespace_set_uuid(ndns, parameters->uuid);
          ndctl_namespace_set_size(ndns, parameters->size);
          /* pmem namespace 不同，blk namespace 有一个扇区大*/
          if (parameters->lbasize)
                  ndctl_namespace_set_sector_size(ndns, parameters->lbasize);
          ndctl_namespace_enable(ndns);
  }
```

##### 为何使用术语 "namespace"
    1. 例如为什么不"volume"（卷）？"volume" 有将 ND（libnvdimm 子系统）       device-mapper 这样的卷管理器混淆的风险
    2. 该术语起源于描述可在 NVME 控制器内创建的子设备（参nvme 规范       https://www.nvmexpress.org/specifications/），NFIT namespace 旨在
       NVME-namespace 的能力和可通过配置性相平行
### LIBNVDIMM/LIBNDCTL：块转换"btt"

BTT（设计文档：https://pmem.io/2014/09/23/btt.html）是一namespace personality 驱动，它将整namespace 作为"地址抽象"呈现于前端
##### LIBNVDIMM：btt 布局

每个 region 一开始至少会有一BTT 设备，即种子（seed）设备。要激活它，需设置
"namespace"uuid" "sector_size" 属性，然后将设备绑定到 nd_pmem 或：

```
	/sys/devices/platform/nfit_test.1/ndbus0/region0/btt0/
	|-- namespace
	|-- delete
	|-- devtype
	|-- modalias
	|-- numa_node
	|-- sector_size
	|-- subsystem -> ../../../../../bus/nd
	|-- uevent
	`-- uuid
```

##### LIBNDCTL：btt 创建示例

namespace 类似，每region 会自动创建一个空闲的 BTT 设备。每次配置并启用
这个"种子"btt 设备时，都会创建一个新的种子。创建一BTT 配置涉及两步：找空闲 BTT 并将其分配以消费一namespace
```
	static struct ndctl_btt *get_idle_btt(struct ndctl_region *region)
	{
		struct ndctl_btt *btt;

		ndctl_btt_foreach(region, btt)
			if (!ndctl_btt_is_enabled(btt)
					&& !ndctl_btt_is_configured(btt))
				return btt;

		return NULL;
	}

	static int configure_btt(struct ndctl_region *region,
			struct btt_parameters *parameters)
	{
		btt = get_idle_btt(region);

		ndctl_btt_set_uuid(btt, parameters->uuid);
		ndctl_btt_set_sector_size(btt, parameters->sector_size);
		ndctl_btt_set_namespace(btt, parameters->ndns);
		/* 关闭原始模式设备 */
		ndctl_namespace_disable(parameters->ndns);
		/* 开btt 访问 */
		ndctl_btt_enable(btt);
	}
```

一旦实例化，一个新的未激btt 种子设备将出现在 region 之下
一旦一"namespace" BTT 中移除，BTT 设备实例将被删除或以其他方式重置默认值。这种删除仅发生在设备模型层面。为了销毁一BTT，需要销毁其 "info
block"（信息块）。注意，要销毁一BTT，需要以原始模式写入介质。默认情况下内核会自动检BTT 的存在并禁用原始模式。此自动检测行为可以通过namespace
启用原始模式来抑制，使用 ndctl_namespace_set_raw_mode() API
### LIBNDCTL 图表示意总结

对于上面给出的示例，以下是该对象通过 API 所看到的视图：

```
              +---+
              |CTX|
              +-+-+
                |
  +-------+     |
  | DIMM0 <-+   |      +---------+   +--------------+  +---------------+
  +-------+ |   |    +-> REGION0 +---> NAMESPACE0.0 +--> PMEM8 "pm0.0" |
  | DIMM1 <-+ +-v--+ | +---------+   +--------------+  +---------------+
  +-------+ +-+BUS0+-| +---------+   +--------------+  +----------------------+
  | DIMM2 <-+ +----+ +-> REGION1 +---> NAMESPACE1.0 +--> PMEM6 "pm1.0" | BTT1 |
  +-------+ |        | +---------+   +--------------+  +---------------+------+
  | DIMM3 <-+
  +-------+
```
