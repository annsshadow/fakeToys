
## Device links（设备链接）


默认情况下，驱动核心只在设备层级中由父子关系产生的依赖之间强制排序：在
挂起、恢复或关闭系统时，设备依据这种关系排序，即子设备总是在其父设备之前
挂起，而父设备总是在其子设备之前恢复。

有时需要表达父子关系之外的设备依赖，例如在兄弟设备之间，并让驱动核心
自动处理它们。

其次，驱动核心默认不强制任何驱动存在性依赖，即一个设备必须先绑定到驱动，
另一个设备才能正确探测或工作。

这两种依赖类型常常同时出现，因此一个设备在驱动存在性**以及**挂起/恢复与
关闭排序两方面都依赖于另一个设备。

设备链接允许在驱动核心中表示此类依赖。

在标准形式或**托管**形式下，设备链接结合了**两种**依赖类型：它保证“提供者”
设备与其“消费者”设备之间正确的挂起/恢复与关闭排序，并保证提供者上有驱动
存在。消费者设备在提供者绑定到驱动之前不会被探测，并且在提供者解绑之前
被解绑。

当提供者上的驱动存在性无关紧要、只需要正确的挂起/恢复与关闭排序时，设备
链接可以简单地用 `DL_FLAG_STATELESS` 标志建立。换言之，强制提供者上的驱动
存在性是可选的。

另一个可选特性是运行时 PM 集成：通过在添加设备链接时设置 `DL_FLAG_PM_RUNTIME`
标志，可以指示 PM 核心在消费者运行时恢复期间及其保持活动的整个时段内，运行时
恢复提供者并使其保持活跃。

## Usage（用法）


能够添加设备链接的最早时机，是在已对提供者调用 `device_add()`、并已对消费者
调用 `device_initialize()` 之后。

稍后添加也是合法的，但必须确保系统保持一致的态：例如，设备链接不能在挂起/
恢复转换过程中途添加，因此需要用 `lock_system_sleep()` 阻止此类转换开始，或者
从保证不会与挂起/恢复转换并行运行的函数（例如设备 `->probe` 回调或启动期的
PCI 怪癖）中添加设备链接。

另一个不一致状态的例子是：表示一个驱动存在性依赖的设备链接，却是在提供者的
`->probe` 回调中添加的，而提供者尚未开始探测。若驱动核心更早知道该设备链接，
它一开始就不会去探测消费者。因此，消费者有责任在添加链接后检查提供者的存在性，
并在不存在时推迟探测。[注意，在提供者仍在探测时从消费者的 `->probe` 回调创建
链接是合法的，但消费者必须知道在创建链接时提供者已经可用（例如，若消费者刚刚
获取了某些资源，而若提供者当时不可用这些资源就不会存在，即属此情形）。]

如果在提供者或消费者驱动的 `->probe` 回调中添加了设置了 `DL_FLAG_STATELESS`
（即无状态设备链接）的链接，通常会在其 `->remove` 回调中删除它，以保持对称。
这样，若驱动编译为模块，设备链接会在模块加载时添加、在卸载时有序删除。适用于
设备链接添加的限制（例如排除并行的挂起/恢复转换）同样适用于删除。由驱动核心
管理的设备链接会被它自动删除。

在添加设备链接时可以指定多个标志，其中两个已在上面提到：`DL_FLAG_STATELESS`
表示不需要驱动存在性依赖（仅需正确的挂起/恢复与关闭排序），`DL_FLAG_PM_RUNTIME`
表示需要运行时 PM 集成。

另外两个标志专门针对从消费者 `->probe` 回调添加设备链接的用例：`DL_FLAG_RPM_ACTIVE`
可指定运行时恢复提供者并阻止其在消费者运行时挂起之前挂起。`DL_FLAG_AUTOREMOVE_CONSUMER`
使设备链接在消费者探测失败或后续解绑时自动清除。

类似地，当设备链接从提供者 `->probe` 回调添加时，`DL_FLAG_AUTOREMOVE_SUPPLIER`
使设备链接在提供者探测失败或后续解绑时自动清除。

如果既未设置 `DL_FLAG_AUTOREMOVE_CONSUMER` 也未设置 `DL_FLAG_AUTOREMOVE_SUPPLIER`，
则可以使用 `DL_FLAG_AUTOPROBE_CONSUMER` 来请求驱动核心在提供者设备绑定到驱动后，
自动为消费者驱动探测驱动。

但需注意，任何将 `DL_FLAG_AUTOREMOVE_CONSUMER`、`DL_FLAG_AUTOREMOVE_SUPPLIER` 或
`DL_FLAG_AUTOPROBE_CONSUMER` 与 `DL_FLAG_STATELESS` 组合的做法都是无效的，不可使用。

## Limitations（限制）


驱动作者应当注意，托管设备链接（即添加链接时未指定 `DL_FLAG_STATELESS`）的驱动
存在性依赖，可能导致消费者的探测被无限期推迟。如果消费者需要在某个 initcall
级别到达之前探测，这就会成为问题。更糟的是，如果提供者驱动被列入黑名单或缺失，
消费者将永远不被探测。

此外，托管设备链接无法直接删除。它们会在不再需要时由驱动核心依据
`DL_FLAG_AUTOREMOVE_CONSUMER` 与 `DL_FLAG_AUTOREMOVE_SUPPLIER` 标志自动删除。然而，
无状态设备链接（即设置了 `DL_FLAG_STATELESS` 的设备链接）应由调用 `device_link_add()`
添加它们的一方，借助 `device_link_del()` 或 `device_link_remove()` 来移除。

将 `DL_FLAG_RPM_ACTIVE` 与 `DL_FLAG_STATELESS` 一起传给 `device_link_add()`，可能导致
提供者的 PM-runtime 使用计数在随后调用 `device_link_del()` 或 `device_link_remove()`
移除其返回的设备链接后仍保持非零。这种情况发生在对同一个消费者-提供者对连续两次
调用 `device_link_add()` 而未在两次调用之间移除链接时，此时允许提供者的 PM-runtime
使用计数在尝试移除链接时下降，可能导致提供者在消费者仍处于 PM-runtime 活跃状态时被
挂起，而这必须避免。[要规避此限制，只需让消费者在 `device_link_add()` 与
`device_link_del()` 或 `device_link_remove()` 调用之间至少运行时挂起一次，或在该间隔
以禁用 PM-runtime 的方式对其调用 `pm_runtime_set_suspended()` 即可。]

有时驱动依赖可选资源。当这些资源不存在时，它们能够以降级模式（缩减的功能集或性能）
运作。一个例子是可以使用 DMA 引擎或以 PIO 模式工作的 SPI 控制器。控制器可以在探测时
确定可选资源的存在性，但在不存在时无法知道它们近期是否会变得可用（由于提供者驱动
探测）还是永远不会。因此无法确定是否应推迟探测。可以在探测后当可选资源变得可用时
通知驱动，但对于驱动而言代价高昂，因为基于此类资源的可用性在运行时切换工作模式，
比基于探测延迟的机制复杂得多。无论如何，可选资源不在设备链接的范畴之内。

## Examples（示例）


- 一个 MMU 设备与一个总线主设备并存，两者处于同一电源域。MMU 为总线主设备实现
  DMA 地址转换，并应在总线主设备活跃期间及其整个活跃时段内运行时恢复并保持活跃。
  总线主设备的驱动不应在 MMU 绑定之前绑定。为实现这一点，从总线主设备（消费者）到
  MMU 设备（提供者）添加一条带运行时 PM 集成的设备链接。其在运行时 PM 方面的效果，
  等同于 MMU 是主设备的父设备。

  这两个设备共享同一电源域这一事实，通常暗示应使用 struct dev_pm_domain 或
  struct generic_pm_domain，但它们并非碰巧共享一个电源开关的相互独立设备，而是
  MMU 设备为总线主设备服务、没有它便毫无用处。设备链接在设备之间创建了合成的层级
  关系，因此更为贴切。

- Thunderbolt 主机控制器包含一个 PCIe 热插拔端口集合和一个用于管理 PCIe 交换机的
  NHI 设备。从系统睡眠恢复时，NHI 设备需要在热插拔端口恢复之前，重新建立到挂接
  设备的 PCI 隧道。如果热插拔端口是 NHI 的子设备，该恢复顺序会由 PM 核心自动强制，
  但不巧它们是“表亲”关系。解决办法是从热插拔端口（消费者）到 NHI 设备（提供者）
  添加设备链接。此用例不需要驱动存在性依赖。

- 混合显卡笔记本中的独立 GPU 通常带有一个用于 HDMI/DP 音频的 HDA 控制器。在设备层级
  中，HDA 控制器是 VGA 设备的兄弟设备，但两者共享同一电源域，且 HDA 控制器仅在 HDMI/DP
  显示器挂接到 VGA 设备时才需要。从 HDA 控制器（消费者）到 VGA 设备（提供者）的设备
  链接恰当地表达了这种关系。

- ACPI 允许通过 _DEP 对象定义设备启动顺序。一个经典的例子是：某个设备上的 ACPI 电源
  管理方法以 I\ `2`\ C 访问的方式实现，并需要某个特定的 I\ `2`\ C 控制器存在且可用，
  该设备的电源管理才能工作。

- 在某些 SoC 中，显示、视频编解码与视频处理 IP 核，对处理突发访问与压缩/解压缩的
  透明内存访问 IP 核存在功能依赖。

## Alternatives（替代方案）


- struct dev_pm_domain 可用于覆盖总线、类或设备类型回调。它面向共享单个开/关开关的
  设备，但它不保证特定的挂起/恢复顺序，需要另行实现。它本身也不跟踪相关设备的
  运行时 PM 状态、并在它们全部运行时挂起后才关闭电源开关。此外，它不能用于强制
  特定的关闭顺序或驱动存在性依赖。

- struct generic_pm_domain 比设备链接重得多，且不允许关闭顺序或驱动存在性依赖。它也
  不能在 ACPI 系统上使用。

## Implementation（实现）


设备层级（顾名思义是一棵树）在添加设备链接后，会变成有向无环图。

这些设备在挂起/恢复期间的顺序由 dpm_list 决定，在关闭期间由 devices_kset 决定。在
没有设备链接时，这两个列表是设备树的扁平一维表示，使得一个设备被放在其所有祖先之后。
这是通过自顶向下遍历 ACPI 命名空间或 OpenFirmware 设备树、并在发现设备时将其追加到
列表来实现的。

一旦添加了设备链接，列表就需要满足额外的约束：一个设备（递归地）被放在其所有提供者
之后。为确保这一点，在添加设备链接时，消费者及其下方的整个子图（消费者的所有子设备
与消费者）会被移动到列表末尾。（由 `device_link_add()` 调用 `device_reorder_to_tail()`。）

为防止向图中引入依赖环，在添加设备链接时会验证提供者不依赖于消费者或消费者的任何
子设备/消费者。（由 `device_link_add()` 调用 `device_is_dependent()`。）如果违反该
约束，`device_link_add()` 将返回 `NULL` 并记录一条 `WARNING`。

值得注意的是，这也阻止了从父设备向子设备添加设备链接。但反向是允许的，即从子设备
向父设备添加设备链接。由于驱动核心已经保证了父子之间正确的挂起/恢复与关闭顺序，此类
设备链接只有在还需要驱动存在性依赖时才有意义。在这种情况下，驱动作者应仔细权衡设备
链接是否真的适合此用途。更合适的做法可能是简单地使用延迟探测，或添加一个使父设备
驱动先于子设备探测的设备标志。

## State machine（状态机）


   :functions: device_link_state

```
                 .=============================.
                 |                             |
                 v                             |
 DORMANT <=> AVAILABLE <=> CONSUMER_PROBE => ACTIVE
    ^                                          |
    |                                          |
    '============ SUPPLIER_UNBIND <============'

```
- 设备链接的初始状态由 `device_link_add()` 根据提供者与消费者上的驱动存在性自动
  确定。如果链接在任何设备被探测之前创建，则被设为 `DL_STATE_DORMANT`。

- 当提供者设备绑定到驱动时，到其消费者的链接推进到 `DL_STATE_AVAILABLE`。
  （由 `driver_bound()` 调用 `device_links_driver_bound()`。）

- 在消费者设备被探测之前，通过检查消费者设备不在 wait_for_suppliers 列表中、并检查
  到提供者的链接处于 `DL_STATE_AVAILABLE` 状态，来核实提供者驱动的存在性。链接的状态
  被更新为 `DL_STATE_CONSUMER_PROBE`。
  （由 `really_probe()` 调用 `device_links_check_suppliers()`。）
  这会阻止提供者解绑。
  （由 `device_links_unbind_consumers()` 调用 `wait_for_device_probe()`。）

- 如果探测失败，到提供者的链接回退到 `DL_STATE_AVAILABLE`。
  （由 `really_probe()` 调用 `device_links_no_driver()`。）

- 如果探测成功，到提供者的链接推进到 `DL_STATE_ACTIVE`。
  （由 `driver_bound()` 调用 `device_links_driver_bound()`。）

- 当消费者的驱动随后被移除时，到提供者的链接回退到 `DL_STATE_AVAILABLE`。
  （由 `device_links_driver_cleanup()` 调用 `__device_links_no_driver()`，而后者又由
  `__device_release_driver()` 调用。）

- 在移除提供者的驱动之前，到未绑定驱动的消费者的链接被更新为 `DL_STATE_SUPPLIER_UNBIND`。
  （由 `__device_release_driver()` 调用 `device_links_busy()`。）
  这会阻止消费者绑定。
  （由 `really_probe()` 调用 `device_links_check_suppliers()`。）
  已绑定的消费者会被解除驱动；正在探测的消费者会被等待直至完成。
  （由 `__device_release_driver()` 调用 `device_links_unbind_consumers()`。）
  一旦到消费者的所有链接都处于 `DL_STATE_SUPPLIER_UNBIND` 状态，提供者驱动被释放，链接
  回退到 `DL_STATE_DORMANT`。
  （由 `__device_release_driver()` 调用 `device_links_driver_cleanup()`。）

## API


参见 device_link_add()、device_link_del() 与 device_link_remove()。
