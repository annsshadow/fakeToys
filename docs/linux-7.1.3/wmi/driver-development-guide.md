
## WMI 驱动开发指南


WMI 子系统为实现 WMI 驱动提供了一套丰富的驱动 API，相关文档位于 Documentation/driver-api/wmi.rst。本文档将作为使用此 API 编写 WMI 驱动的入门指南。它是原始 LWN 文章 [^1^]_ 的续篇，那篇文章讨论的是使用已废弃的基于 GUID 的 WMI 接口的 WMI 驱动。

### 获取 WMI 设备信息


在开发 WMI 驱动之前，必须先获取相关 WMI 设备的信息。可以使用 `lswmi <https://pypi.org/project/lswmi>`_ 工具通过以下命令提取详细的 WMI 设备信息：

```

  lswmi -V

```
得到的输出将包含给定机器上所有可用 WMI 设备的信息，以及一些额外信息。

为了进一步了解用于与 WMI 设备通信的接口，可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具来解码用于描述 WMI 设备的二进制 MOF（Managed Object Format，托管对象格式）信息。
`wmi-bmof` 驱动将此信息暴露给用户空间，参见
Documentation/wmi/devices/wmi-bmof.rst。

要获取解码后的二进制 MOF 信息，请使用以下命令（需要 root 权限）：

```

  ./bmf2mof /sys/bus/wmi/devices/05901221-D566-11D1-B2F0-00A0C9062910[-X]/bmof

```
有时，查看用于描述 WMI 设备的反汇编 ACPI 表，有助于理解 WMI 设备应当如何工作。与给定 WMI 设备关联的 ACPI 方法的路径，可以使用上面提到的 `lswmi` 工具获取。

如果您正尝试将某个驱动移植到 Linux，并且是在 Windows 系统上工作，那么 `WMIExplorer <https://github.com/vinaypamnani/wmie2>`_ 工具会很有用，它可以检查可用的 WMI 方法并直接调用它们。

### 基本 WMI 驱动结构


基本的 WMI 驱动围绕 struct wmi_driver 构建，然后通过一个 struct wmi_device_id 表绑定到匹配的 WMI 设备：

```

  static const struct wmi_device_id foo_id_table[] = {
         /* Only use uppercase letters! */
         { "936DA01F-9ABD-4D9D-80C7-02AF85C822A8", NULL },
         { }
  };
  MODULE_DEVICE_TABLE(wmi, foo_id_table);

  static struct wmi_driver foo_driver = {
        .driver = {
                .name = "foo",
                .probe_type = PROBE_PREFER_ASYNCHRONOUS,        /* recommended */
                .pm = pm_sleep_ptr(&foo_dev_pm_ops),            /* optional */
        },
        .id_table = foo_id_table,
        .probe = foo_probe,
        .remove = foo_remove,         /* optional, devres is preferred */
        .shutdown = foo_shutdown,     /* optional, called during shutdown */
        .notify_new = foo_notify,     /* optional, for event handling */
        .min_event_size = X,          /* optional, simplifies event payload size verification */
        .no_singleton = true,         /* required for new WMI drivers */
  };
  module_wmi_driver(foo_driver);

```
当 WMI 驱动绑定到一个匹配的 WMI 设备时，会调用 probe() 回调。通常应该在这个函数中
分配驱动特定的数据结构并初始化到其他内核子系统的接口。

当 WMI 驱动从某个 WMI 设备解绑时，会调用 remove() 回调。为了注销到其他内核子系统的接口
并释放资源，应该使用 devres。这可以简化 probe 期间的错误处理，并且通常可以完全省略该回调，
详见 Documentation/driver-api/driver-model/devres.rst。

shutdown() 回调在关机、重启或 kexec 期间被调用。它的唯一目的是禁用 WMI 设备，并将其置于一个
已知的状态，以便 WMI 驱动在重启或 kexec 之后能够重新接管。大多数 WMI 驱动不需要特殊的关机处理，
因此可以省略该回调。

请注意，新的 WMI 驱动必须能够被多次实例化，并且禁止使用任何已废弃的基于 GUID 或基于 ACPI 的
WMI 函数。这意味着 WMI 驱动应该为给定机器上存在多个匹配 WMI 设备的场景做好准备。

因此，WMI 驱动应该使用 Documentation/driver-api/driver-model/design-patterns.rst 中描述的
状态容器（state container）设计模式。

             在同一设备上同时处理 WMI 事件必然会导致 WMI 设备状态损坏，并可能引发异常行为。

### WMI 方法驱动


WMI 驱动可以使用 wmidev_invoke_method() 调用 WMI 设备方法。对于每次 WMI 方法调用，WMI 驱动
需要提供实例号和方法 ID，以及包含方法参数的缓冲区，还可选地提供一个用于存放结果的缓冲区。
当调用不返回任何值的 WMI 方法时，应该改用 wmidev_invoke_procedure()。

上述缓冲区的布局是设备特定的，由与给定 WMI 设备关联的二进制 MOF 数据描述。该二进制 MOF 数据
还使用 `WmiMethodId` 限定符描述给定 WMI 方法的方法 ID。暴露 WMI 方法的 WMI 设备通常只暴露单个
实例（实例号 0），但理论上也可以暴露多个实例。在这种情况下，可以使用 wmidev_instance_count()
获取实例的数量。

有关 WMI 方法驱动的示例，请参阅 drivers/platform/x86/intel/wmi/thunderbolt.c。

### WMI 数据块驱动


WMI 驱动可以使用 wmidev_query_block() 查询 WMI 数据块，返回缓冲区的布局同样是设备特定的，
并由二进制 MOF 数据描述。一些 WMI 数据块也是可写的，可以使用 wmidev_set_block() 设置。数据块
实例的数量同样可以使用 wmidev_instance_count() 获取。

有关 WMI 数据块驱动的示例，请参阅 drivers/platform/x86/intel/wmi/sbl-fw-update.c。

### WMI 事件驱动


WMI 驱动可以通过 struct wmi_driver 内部的 notify_new() 回调接收 WMI 事件。随后 WMI 子系统会
负责相应地设置该 WMI 事件。请注意，传递给此回调的缓冲区布局是设备特定的，并且缓冲区的释放
由 WMI 子系统自身完成，而不是由驱动完成。

WMI 驱动核心会确保 notify_new() 回调只在 probe() 回调被调用之后才会被调用，并且在调用驱动
的 remove() 或 shutdown() 回调的前后不会收到任何事件。

不过，WMI 驱动开发者应该意识到，多个 WMI 事件可能会被并发接收，因此任何必要的加锁都需要由
WMI 驱动自身提供。

WMI 驱动还可以通过填充 struct wmi_driver 中的 `min_event_size` 字段，指示 WMI 驱动核心自动
拒绝包含过小事件负载的 WMI 事件。因此，将该字段设为 0 将使 WMI 驱动能够接收不带任何事件负载的
WMI 事件。

有关 WMI 事件驱动的示例，请参阅 drivers/platform/x86/xiaomi-wmi.c。

### 与 WMI 驱动核心交换数据


WMI 驱动可以使用 struct wmi_buffer 与 WMI 驱动核心交换数据。这些缓冲区的内部结构是设备特定的，
只有 WMI 驱动才知道。因此，WMI 驱动自身负责解析和校验从其 WMI 设备接收到的数据。

上述缓冲区的结构由相关 WMI 设备的 MOF 数据描述。当这样的缓冲区包含多个数据项时，通常定义一个
C 结构并在解析时使用它是合理的。由于 WMI 驱动核心保证从 WMI 设备接收到的所有缓冲区都按 8 字节
边界对齐，WMI 驱动可以简单地进行 WMI 缓冲区数据与这个 C 结构之间的转换（cast）。

不过，这只有在缓冲区的尺寸被验证为足以容纳整个 C 结构之后才应进行。WMI 驱动应该拒绝过小的缓冲区，
因为它们通常是 WMI 设备用来发出内部错误信号的。但过大的缓冲区应该被接受，以模拟 Windows WMI
实现的行。

在为解析 WMI 缓冲区定义 C 结构时，应该尊重数据项的对齐方式。这对于 64 位整数尤为重要，因为
它们在 64 位（8 字节对齐）和 32 位（4 字节对齐）架构上具有不同的对齐方式。因此，手动指定此类
数据项的对齐方式，或在适当时将整个结构标记为 packed 是个好主意。整数数据项一般是无符号小端整数，
应该使用 `__le64` 等类型显式标记。解析 WMI 字符串数据项时应使用 struct wmi_string，因为 WMI 字符串
的布局与 C 字符串不同。

有关 WMI 数据项二进制格式的更多信息，请参阅 Documentation/wmi/acpi-interface.rst。

### 一次性处理多个 WMI 设备


固件厂商使用多个 WMI 设备来控制单个物理设备的不同方面的情况很多。这可能使 WMI 驱动的开发变得复杂，
因为这些驱动可能需要相互通信，以向用户空间呈现统一的接口。

其中一种情况涉及一个 WMI 事件设备，它需要在收到 WMI 事件时与一个 WMI 数据块设备或 WMI 方法设备
通信。在这种情况下，应该开发两个 WMI 驱动，一个用于 WMI 事件设备，另一个用于另一个 WMI 设备。

WMI 事件设备驱动只有一个目的：接收 WMI 事件、校验任何附加的事件数据并调用一个通知链（notifier chain）。
另一个 WMI 驱动在探测期间将自己加入这个通知链，从而每次收到 WMI 事件时都会得到通知。这个 WMI 驱动
随后可以进一步处理该事件，例如通过使用一个输入设备。

对于其他 WMI 设备的组合，也可以使用类似的机制。

### 需要避免的事项


开发 WMI 驱动时，有几件事应该避免：

- 使用已废弃的基于 GUID 的 WMI 接口，它使用 GUID 而不是 WMI 设备结构体
- 使用已废弃的基于 ACPI 的 WMI 接口，它使用 ACPI 对象而不是普通缓冲区
- 在与 WMI 设备通信时绕过 WMI 子系统
- 无法被多次实例化的 WMI 驱动

许多较旧的 WMI 驱动违反了此列表中的一条或多条。原因是 WMI 子系统在过去二十年中发生了显著演进，
因此较旧的 WMI 驱动中存在大量历史遗留的糟粕。

新的 WMI 驱动还需要符合 Documentation/process/coding-style.rst 中规定的 Linux 内核代码风格。
checkpatch 工具可以捕获许多常见的代码风格违规，您可以使用以下命令调用它：

```

  ./scripts/checkpatch.pl --strict <path to driver file>

```
## 参考文档


