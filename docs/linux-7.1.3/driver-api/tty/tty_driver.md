## TTY 驱动与 TTY 操作


## 分配

驱动首先要做的是分配一个 struct tty_driver。这通过 tty_alloc_driver()（或
__tty_alloc_driver()）完成。接下来，新分配的结构体被填入信息。关于具体应填入
哪些内容，请参阅本文档末尾的 `TTY 驱动参考`_。

分配例程接受一个驱动最多能处理的设备数量以及一些标志。标志即以 `TTY_DRIVER_`
开头、在 `TTY 驱动标志`_ 中列出并描述的那些。

当驱动即将被释放时，会对其调用 tty_driver_kref_put()。它会递减引用计数，若减
到零则释放该驱动。

作为参考，分配与释放函数在下面详细说明：

   :identifiers: tty_alloc_driver
   :identifiers: __tty_alloc_driver tty_driver_kref_put

### TTY 驱动标志

下面给出 tty_alloc_driver()（或 __tty_alloc_driver()）所接受的标志说明：

   :identifiers: tty_driver_flag

----

## 注册

当一个 struct tty_driver 被分配并填好内容后，可以使用 tty_register_driver()
进行注册。建议在 tty_alloc_driver() 的 flags 中传入 `TTY_DRIVER_DYNAMIC_DEV`。
若不传入，则在 tty_register_driver() 期间会同时注册**所有**设备，此类驱动可
跳过下面关于注册设备的段落。不过 `注册设备`_ 中的 struct tty_port 部分仍然相关。

   :identifiers: tty_register_driver tty_unregister_driver

### 注册设备

每个 TTY 设备都应由一个 struct tty_port 支撑。通常，TTY 驱动将 tty_port 内嵌到
设备的私有结构中。关于处理 tty_port 的更多细节，可参见 [tty_port](tty_port)。驱动
还建议使用 tty_port_get() 和 tty_port_put() 进行 tty_port 的引用计数。最后一次
put 应当释放该 tty_port（包括设备的私有结构）。

除非在 tty_alloc_driver() 的 flags 中传入了 `TTY_DRIVER_DYNAMIC_DEV`，否则 TTY
驱动应当注册系统中发现的每一个设备（后者为推荐做法）。这通过 tty_register_device()
完成；或者，如果驱动希望通过 struct attribute_group 暴露某些信息，则使用
tty_register_device_attr()。二者都会注册第 `index` 个设备，返回后该设备即可被打开。
稍后 `关联设备与端口`_ 中还描述了更推荐的 tty_port 变体。由驱动自行管理空闲索引
并选择正确的那一个。TTY 层只会拒绝注册多于传入 tty_alloc_driver() 数量的设备。

当设备被打开时，TTY 层分配 struct tty_struct 并开始调用 :c`tty_driver.ops`
中的操作，参见 `TTY 操作参考`_。

注册例程说明如下：

   :identifiers: tty_register_device tty_register_device_attr
        tty_unregister_device

----

### 关联设备与端口

如前所述，每个 TTY 设备都应当分配一个 struct tty_port。最迟必须在
:c`tty_driver.ops.install()` 时让 TTY 层知晓它。有少量辅助函数用于**关联**两者。
理想情况下，驱动在注册时使用 tty_port_register_device() 或
tty_port_register_device_attr() 来替代 tty_register_device() 和
tty_register_device_attr()。这样驱动就无需关心后续的关联。

若做不到，驱动仍可在实际注册**之前**通过 tty_port_link_device() 将 tty_port 关联到
某个特定索引。如果仍不合适，作为最后的手段，可以在 :c`tty_driver.ops.install`
钩子中使用 tty_port_install()。后者主要用于 PTY 等内存中设备，其 tty_port 是按需
分配的。

关联例程在此说明：

   :identifiers: tty_port_link_device tty_port_register_device
        tty_port_register_device_attr

----

## TTY 驱动参考

struct tty_driver 的所有成员在此说明。必需的成员在末尾注明。struct tty_operations
在随后说明。

   :identifiers: tty_driver

----

## TTY 操作参考

当 TTY 被注册后，这些驱动钩子可由 TTY 层调用：

   :identifiers: tty_operations
