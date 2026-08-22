
# DRM 客户端使用统


DRM 驱动可以选择通过 `fops->show_fdinfo()` 导出部分标准化的文本输出，作为注册到 DRM 核心`struct drm_driver` 对象中注册的驱动特定文件操作的一部分

此输出的一个目的是使得编写尽可能通用的、类`top(1)` 的用户空间监控工具成为可能

鉴于各种 DRM 驱动之间的差异，输出的规范被划分为通用部分和驱动特定部分。尽管如此，在可能的地方仍应尽量实现最大程度的统一

# 文件格式规范


- 文件每行文本应包含一个键值对
- 必须使用冒号字符（`:`）来分隔键与值
- 所有标准化的键都应`drm-` 为前缀
- 驱动特定的键应以 `driver_name-` 为前缀，其driver_name 理想情况下应`struct drm_driver` 中的 `name` 字段相同，但这并非强制要求
- 解析时应忽略分隔符与第一个非空白字符之间的空白
- 键不允许包含空白字符
- 数值型键值对可以以可选的单位字符串结尾
- 值的数据类型固定为规范中所定义的类型

## 键类


1. 强制的，完全标准化
2. 可选的，完全标准化
3. 驱动特定的

## 数据类型


- <uint> - 未定义最大值的上限整数
- <keystr> - 不包含上述任何保留字符或空白的字符串
- <valstr> - 字符串

## 强制的完全标准化


- drm-driver: <valstr>

字符串应包含该驱动通过相应`struct drm_driver` 数据结构注册时使用的名称

## 可选的完全标准化键


#### 标识


- drm-pdev: <aaaa:bb.cc.d>

对于 PCI 设备，它应包含该设备PCI 插槽地址

- drm-client-id: <uint>

一个唯一值，与用于区分重复和共享文件描述符的已打开 DRM 文件描述符相关。从概念上讲，该值应`struct drm_file` 实例的内核表示一一对应

该值的唯一性可以是全局唯一的，也可以在每个设备范围内唯一；在后一种情况下，应同时提供 `drm-pdev`

用户空间应确保使用上述标准，以便将数据关联到各个客户端，从而避免重复统计任何使用统计信息

- drm-client-name: <valstr>

由用户空间使DRM_IOCTL_SET_CLIENT_NAME 选择性设置的字符串


#### 利用


- drm-engine-<keystr>: <uint> ns

GPU 通常包含多个执行引擎。每个引擎应被赋予一个稳定且唯一的名称（keystr），其可能取值记录在驱动特定的文档中

该值应为相GPU 引擎忙于执行属于此客户端的工作负载所花费的时间，单位为指定的时间单位

如果这些值能使驱动实现更容易，则不要求始终保持单调；但要求在合理的时间段内追上之前报告的较大值。当观察到的值低于之前读取的值时，用户空间应沿用该较大的先前值，直到观察到一次单调更新

- drm-engine-capacity-<keystr>: <uint>

引擎标识字符串必须与 drm-engine-<keystr> 标签中指定的相同，并且当导出的引擎对应于一组相同的硬件引擎时，应包含大于零的数字

如果缺少此标签，解析器应假定容量1。不允许容量为零

- drm-cycles-<keystr>: <uint>

引擎标识字符串必须与 drm-engine-<keystr> 标签中指定的相同，并应包含给定引擎的忙循环次数

如果这些值能使驱动实现更容易，则不要求始终保持单调；但要求在合理的时间段内追上之前报告的较大值。当观察到的值低于之前读取的值时，用户空间应沿用该较大的先前值，直到观察到一次单调更新

- drm-total-cycles-<keystr>: <uint>

引擎标识字符串必须与 drm-cycles-<keystr> 标签中指定的相同，并应包含给定引擎的总循环次数

这是一个以 GPU 未指定单位表示的时间戳，其更新速率drm-cycles-<keystr> 相匹配。对于实现了此接口的驱动，可以在 GPU 时钟域内完全计算引擎利用率，而无需考虑两次采样之间CPU 睡眠时间

驱动可以实现此键drm-maxfreq-<keystr> 之一，但不能同时实现两者

- drm-maxfreq-<keystr>: <uint> [Hz|MHz|KHz]

引擎标识字符串必须与 drm-engine-<keystr> 标签中指定的相同，并应包含给定引擎的最大频率。结drm-cycles-<keystr>，这可用于计算引擎的利用率百分比，drm-engine-<keystr> 仅反映活跃时间，不考虑引擎以最大频率的百分比运行的情况

驱动可以实现此键drm-total-cycles-<keystr> 之一，但不能同时实现两者

#### 内存


GPU 可以用来存储缓冲区对象的每种可能内存类型，都应被赋予一个稳定且唯一的名称，用作 region>字符串

区域“memory被保留用于指代普通系统内存

该值应反映当前由此客户端的缓冲区对象在所对应内存区域中消耗的存储量

默认单位应为字节，可选的单位说明符为 ‘KiB‘MiB’，分别表示 kibi- mebi-字节

- drm-total-<region>: <uint> [KiB|MiB]

所有已请求缓冲区的总大小，包括共享内存和私有内存。缓冲区的后备存储无需当前已实例化即可计入此类别。为避免重复计数，如果一个缓冲区可以分配到多个区域，实现应出于统计目的一致地选择单一区域

- drm-shared-<region>: <uint> [KiB|MiB]

与另一个文件共享的缓冲区的总大小（即具有多个句柄）。适用drm-total-<region> 的避免重复计数的要求同样适用于此处

- drm-resident-<region>: <uint> [KiB|MiB]

在某个指定区域中常驻（即其后备存储已存在或已实例化）的缓冲区的总大小

- drm-memory-<region>: <uint> [KiB|MiB]

此键已被弃用，仅amdgpu 打印；它drm-resident-<region> 的别名

- drm-purgeable-<region>: <uint> [KiB|MiB]

常驻且可丢弃（purgeable）的缓冲区的总大小

例如，实现了类似 ‘madvise功能的驱动，可以统计那些已实例化后备存储但被标记为等价于 MADV_DONTNEED 的缓冲区

- drm-active-<region>: <uint> [KiB|MiB]

在一个或多个引擎上活跃的缓冲区的总大小

一个实际例子是 GEM 缓冲区预留对象中存在未发出信号的围栏。因此，active 类别resident 类别的一个子集

# 实现细节


驱动应在`struct file_operations` 中使drm_show_fdinfo()，并在希望提drm_show_fdinfo() 未给出的任何统计信息时实&drm_driver.show_fdinfo。但即便是驱动特定的统计信息，也应在上文加以文档说明，并在可能时与其他驱动保持一致

## 驱动特定的实


- i915-usage-stats
- panfrost-usage-stats
- panthor-usage-stats
- xe-usage-stats

