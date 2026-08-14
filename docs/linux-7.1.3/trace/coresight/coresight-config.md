## CoreSight 系统配置管理器


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     October 2020

## 简介


CoreSight 系统配置管理器（System Configuration manager）是一个 API，它允许使用预定义的配置对 CoreSight 系统进行编程，这些配置随后可以轻松地从 sysfs 或 perf 启用。

许多 CoreSight 组件可以以复杂的方式编程 —— 尤其是 ETM。此外，组件之间可以跨越 CoreSight 系统交互，通常经由交叉触发组件（如 CTI 和 CTM）。这些系统设置可以被定义为命名的配置并启用。


## 基本概念


本节介绍 CoreSight 系统配置的基本概念。


### 特性（Features）


一个特性（feature）是一组针对某个 CoreSight 设备的命名编程集合。编程是设备相关的，可以用绝对寄存器值、资源使用和参数值来定义。

特性使用一个描述符（descriptor）来定义。这个描述符用于加载到匹配的设备上，无论是在特性被加载到系统中时，还是在 CoreSight 设备向配置管理器注册时。

加载过程涉及将描述符解释为驱动中的一组寄存器访问 —— 资源使用和参数描述被转换为相应的寄存器访问。这种解释为在需要时高效地将特性编程到设备上提供了便利。

在特性被启用且设备本身也被启用之前，该特性不会在设备上处于活动状态。当设备被启用时，已启用的特性将被编程到设备硬件中。

一个特性是作为在系统上启用的某个配置的一部分而被启用的。


#### 参数值（Parameter Value）


参数值是一个命名的值，用户可以在特性启用之前设置它，以调整由该特性所编程的操作的行为。

例如，这可以是一个在给定速率下重复的编程操作中的计数值。当特性被启用时，参数的当前值会被用于设备编程。

特性描述符为参数定义了一个默认值，如果用户没有提供新值，则使用默认值。

用户可以使用 CoreSight 系统的 configfs API 更新参数值 —— 下文会做介绍。

当特性在该设备上启用时，参数的当前值会被加载到设备中。


### 配置（Configurations）


一个配置定义了一组特性，这些特性将在选择了该配置的跟踪会话中使用。对于任何跟踪会话，只能选择一个配置。

所定义的特性可以位于任何已注册以支持系统配置的设备类型上。一个配置可以选择要在某类设备上启用的特性 —— 即任何 ETMv4，或特定设备，例如系统上的某个特定 CTI。

与特性一样，配置也使用一个描述符来定义。这会定义必须作为配置一部分启用的特性，以及任何可用于覆盖默认参数值的预置值。


#### 预置值（Preset Values）


预置值是配置所用特性的参数值中易于选择的一组集合。单个预置集合中的值的数量，等于配置所用特性的参数值之和。

例如，一个配置由 3 个特性组成，其中一个有 2 个参数，一个有 1 个参数，另一个没有参数。因此，单个预置集合将有 3 个值。

预置由配置可选地定义，最多可以定义 15 个。如果未选择任何预置，则照常使用特性中定义的参数值。


#### 操作（Operation）


配置的操作会经历以下步骤。

1) 在此示例中，配置为 'autofdo'，它有一个关联的特性 'strobing'，作用于 ETMv4 CoreSight 设备。

2) 配置被启用。例如 'perf' 可以像下面这样选择
```
    perf record -e cs_etm/autofdo/ myapp
```

   这将启用 'autofdo' 配置。

3) perf 在系统上开始跟踪。随着 perf 用于跟踪的每个 ETMv4 被启用，配置管理器会检查该 ETMv4 是否具有与当前活动配置相关的特性。在这种情况下，'strobing' 被启用并编程到 ETMv4 中。

4) 当 ETMv4 被禁用时，任何标记为需要保存的寄存器将被读回。

5) 在 perf 会话结束时，配置将被禁用。


## 查看配置与特性


当前加载到系统中的配置和特性集合可以使用 configfs API 查看。

```
    $ ls /config
    cs-syscfg  stp-policy
```

```
    $ cd cs-syscfg/
    $ ls
    configurations  features
```

系统内置了 'autofdo' 配置。可以像下面这样检查它
```
    $ cd configurations/
    $ ls
    autofdo
    $ cd autofdo/
    $ ls
    description  feature_refs  preset1  preset3  preset5  preset7  preset9
    enable       preset        preset2  preset4  preset6  preset8
    $ cat description
    Setup ETMs with strobing for autofdo
    $ cat feature_refs
    strobing
```

每个声明的预置都有一个 'preset<n>' 子目录。其值如
```
    $ cat preset1/values
    strobing.window = 0x1388 strobing.period = 0x2
    $ cat preset2/values
    strobing.window = 0x1388 strobing.period = 0x4
```

'enable' 和 'preset' 文件允许在使用 CoreSight 与 sysfs 时控制一个配置。

配置所引用的特性可以在 features 中检查
```
    $ cd ../../features/strobing/
    $ ls
    description  matches  nr_params  params
    $ cat description
    Generate periodic trace capture windows.
    parameter 'window': a number of CPU cycles (W)
    parameter 'period': trace enabled for W cycles every period x W cycles
    $ cat matches
    SRC_ETMV4
    $ cat nr_params
    2
```

```
    cd params
    $ ls
    period  window
    $ cd period
    $ ls
    value
    $ cat value
    0x2710
    # echo 15000 > value
    # cat value
    0x3a98
```

以这种方式调整的参数会反映到所有已加载该特性的设备实例中。


## 在 perf 中使用配置


加载到 CoreSight 配置管理中的配置也会在 perf 的 'cs_etm' 事件基础设施中声明，以便它们可以
```
    $ ls /sys/devices/cs_etm
    cpu0  cpu2  events  nr_addr_filters		power  subsystem  uevent
    cpu1  cpu3  format  perf_event_mux_interval_ms	sinks  type
```

这里的关键目录是 'events' —— 一个通用的 perf 目录，允许在 perf 命令行上进行选择。与 sinks 条目一样，这提供了配置名称的哈希。

'events' 目录中的条目使用 perf 内置的语法生成器
```
    $ ls events/
    autofdo
    $ cat events/autofdo
    configid=0xa7c3dddd
```

```
    $ perf record -e cs_etm/autofdo/u --per-thread <application>
```

```
    $ perf record -e cs_etm/autofdo,preset=1/u --per-thread <application>
```

当以这种方式选择配置时，所使用的跟踪接收端（sink）会被自动选择。

## 在 sysfs 中使用配置


CoreSight 可以通过 sysfs 控制。当使用 sysfs 时，可以使一个配置对 sysfs 会话中所用的设备处于活动状态。

在一个配置中有 'enable' 和 'preset' 文件。

```
    $ cd configurations/autofdo
    $ echo 1 > enable
```

这将使用特性中的任何默认参数值 —— 这些参数值可以如上所述进行调整。

```
    $ echo 3 > preset
```

这将为配置选择 preset3。preset 的有效值为 0 —— 用于取消选择预置，以及任何存在 preset<n> 子目录的 <n> 值。

请注意，活动的 sysfs 配置是一个全局参数，因此 sysfs 在任何时刻只能有一个活动配置。尝试启用第二个配置将导致错误。此外，尝试在被使用时禁用该配置也会导致错误。

sysfs 对活动配置的使用与 perf 中使用的配置是独立的。


## 创建并加载自定义配置


自定义配置和（或）特性可以通过使用可加载模块动态地加载到系统中。

自定义配置的一个示例位于 ./samples/coresight。

这会创建一个新的配置，该配置使用现有的内置 strobing 特性，但提供了一组不同的预置。

当模块被加载时，该配置会出现在 configfs 文件系统中，并且可以像上文所述的内置配置一样被选择。

配置可以使用之前已加载的特性。系统会确保在当前正在使用的特性无法被卸载，方法是强制卸载顺序严格为加载顺序的逆序。
