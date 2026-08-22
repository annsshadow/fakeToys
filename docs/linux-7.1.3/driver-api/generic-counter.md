
## Generic Counter Interface


## 简

计数器设备广泛存在于各行各业。这些设备的普遍存在，需要一个通用的交互与暴露接口及标准。本驱动 API 试图通过引入一个供使用的通用计数器接口，来解决现有计数器设备驱动中存在的代码重复问题。通用计数器接口使驱动能够支持并暴露计数器设备所共有的一组组件与功能
## 原理


计数器设备在设计上可能大相径庭，但无论是正交编码器计数器还是计数累加器，所有计数器设备都由一组核心组件构成。这组被所有计数器设备共享的核心组件，正是通用计数器接口的本质所在
一个计数器有三个核心组件：

- Signal：由计数器评估的数据流
- Synapse：将 Signal 与评估触发条件关联到 Count 的关系
- Count：所连接 Synapse 效果的累积
### SIGNAL

Signal 表示一条数据流。这是由计数器评估以确定计数数据的输入数据；例如旋转编码器的正交信号输出线。并非所有计数器设备都向用户提供 Signal 数据的访问，因此对驱动而言暴露该数据是可选的
Signal 数据可供用户访问时，通用计数器接口提供以下可用的信号取值：

- SIGNAL_LOW：信号线处于低电平状态
- SIGNAL_HIGH：信号线处于高电平状态
一Signal 可以与一个或多个 Count 关联
### SYNAPSE

Synapse 表示 Signal Count 之间的关联。Signal 数据会影响相应的 Count 数据，Synapse 表示这种关系
Synapse 的动作模式（action mode）指定触发相Count 的计数函数评估以更新计数数据Signal 数据条件。通用计数器接口提供以下可用的动作模式
- None：Signal 不触发计数函数。在 Pulse-Direction 计数函数模式下，Signal 被作为方向（Direction）评估
- Rising Edge：低电平状态转换到高电平状态
- Falling Edge：高电平状态转换到低电平状态
- Both Edges：任何状态转换
计数器被定义为一组与计数数据相关联的输入信号，这些计数数据通过对相应计数函数所定义的关联输入信号状态进行评估而生成。在通用计数器接口的语境下，一个计数器由多Count 构成，每Count 关联一Signal，其各自Synapse 实例表示相应 Count 的计数函数更新条件
一Synapse 将一Signal 与一Count 关联
### COUNT

Count 表示所连接 Synapse 效果的累积；即一Signal 的计数数据。通用计数器接口将计数数据表示为自然数
Count 具有一个计数函数模式（count function mode），表示计数数据的更新行为。通用计数器接口提供以下可用的计数函数模式
- Increase：累积计数递增
- Decrease：累积计数递减
- Pulse-Direction：信A 上的上升沿更新相应计数。信B 的输入电平决定方向
- Quadrature：对一对正交编码信号进行评估以确定位置和方向。可用的 Quadrature 模式如下
  - x1 A：若方向为正向，正交对信A 上的上升沿更新相应计数；若方向为反向，信A 上的下降沿更新相应计数。方向由正交编码决定
  - x1 B：若方向为正向，正交对信B 上的上升沿更新相应计数；若方向为反向，信B 上的下降沿更新相应计数。方向由正交编码决定
  - x2 A：正交对信号 A 上的任何状态转换都会更新相应计数。方向由正交编码决定
  - x2 B：正交对信号 B 上的任何状态转换都会更新相应计数。方向由正交编码决定
  - x4：任一正交对信号上的任何状态转换都会更新相应计数。方向由正交编码决定
一Count 具有一组或多组关联 Synapse
## 范式


最基本的计数器设备可以表示为通过单个 Synapse 与单Signal 关联起来的单Count。以一个简单地对某信号上的上升沿进行计数的计数器设备为例：

```
                Count                Synapse        Signal
                -----                -------        ------
        +---------------------+
        | Data: Count         |    Rising Edge     ________
        | Function: Increase  |  <-------------   / Source \
        |                     |                  ____________
        +---------------------+

```
在该示例中，Signal 是一条具有脉冲电压的来源输入线，Count 是一个被反复递增的持久计数值。Signal 通过 Synapse 与相Count 关联。increase 函数Synapse 指定Signal 数据条件触发——在本例中为电压输入线上的上升沿条件。总之，计数器设备的存在与行为恰当地由相应Count、Signal Synapse 组件表示：上升沿条件触发对累积计数值的 increase 函数
计数器设备并不局限于单个 Signal；事实上，理论上许多 Signal 都可与单Count 关联。例如，正交编码器计数器设备可以根据输入信号跟踪位置
```
                   Count                 Synapse     Signal
                   -----                 -------     ------
        +-------------------------+
        | Data: Position          |    Both Edges     ___
        | Function: Quadrature x4 |  <------------   / A \
        |                         |                 _______
        |                         |
        |                         |    Both Edges     ___
        |                         |  <------------   / B \
        |                         |                 _______
        +-------------------------+

```
在该示例中，两个 Signal（正交编码器A B）与单个 Count 关联：A B 上的上升沿或下降沿触"Quadrature x4" 函数，该函数确定运动方向并更新相应的位置数据Quadrature x4" 函数很可能实现于正交编码器计数器设备的硬件中；Count、Signal Synapse 仅仅是这种硬件行为与功能的表示
与同一 Count 关联Signal 可以具有不同Synapse 动作模式条件。例如，运行在非正交 Pulse-Direction 模式下的正交编码器计数器设备可以有一条专用于运动的输入线，以及第二条专用于方向的输入线：

```
                   Count                   Synapse      Signal
                   -----                   -------      ------
        +---------------------------+
        | Data: Position            |    Rising Edge     ___
        | Function: Pulse-Direction |  <-------------   / A \ (Movement)
        |                           |                  _______
        |                           |
        |                           |       None         ___
        |                           |  <-------------   / B \ (Direction)
        |                           |                  _______
        +---------------------------+

```
只有 Signal A 触发 "Pulse-Direction" 更新函数，但仍需 Signal B 的瞬时状态才能确定方向，从而正确更新位置数据。最终，两个 Signal 都通过各自Synapse 与同一 Count 关联，但只有一Synapse 具有触发相应计数函数的活动动作模式条件，而另一个则保持 "None" 条件的动作模式，以表明其相应 Signal 尽管不触发，但仍可用于状态评估
请注意，Signal、Synapse Count 是抽象表示，无需与其各自的物理来源紧密绑定。这使得计数器的使用者可以从物理组件的细微差别（例如输入线是差分还是单端）中解脱出来，转而专注于数据与过程所表示的核心概念（例如从正交编码数据解读出的位置）
## 驱动 API


驱动开发者可以通过包含 include/linux/counter.h 头文件，在自己的代码中使用通用计数器接口。该头文件提供了若干用于定义计数器设备的核心数据结构、函数原型与宏
   :internal:

   :export:

   :export:

## 驱动实现


为支持一个计数器设备，驱动必须首先通过 counter_signal 结构分配可用Counter Signal。这Signal 应存储为数组，并Counter 注册到系统之前，设置到已分配counter_device 结构signals 数组成员中
Counter Count 可通过 counter_count 结构分配，相应的 Counter Signal 关联（Synapse）通过 counter_synapse 结构建立。关联的 counter_synapse 结构存储为数组，并设置到相应 counter_count 结构synapses 数组成员中。这counter_count 结构Counter 注册到系统之前，设置到已分配counter_device 结构counts 数组成员中
必须counter_device 结构提供驱动回调以便与设备通信：读写各Signal Count，并分别设置和获取各Synapse Count "action mode"（动作模式）"function mode"（函数模式）
counter_device 结构使用 counter_alloc() 分配，然后通过将其传给 counter_add() 函数注册到系统，并通过将其传给 counter_unregister 函数注销。存在这些设备的托管变体：devm_counter_alloc() devm_counter_add()
struct counter_comp 结构用于Signal、Synapse Count 定义计数器扩展
"type" 成员指定此扩展所处理的高级数据类型（例如 BOOL、COUNT_DIRECTION 等）。然后，计数器设备驱动可以通过回调设置 "`*_read`" "`*_write`" 成员，以使用原生 C 数据类型（即 u8、u64 等）处理该数据
为驱动开发者提供了诸如 `COUNTER_COMP_COUNT_U64` 之类的便捷宏。特别是，期望驱动开发者对标准 Counter 子系统属性使用所提供的宏，以便为用户空间维持一致的接口。例如，一个计数器的扩展定义如下：

```
        struct counter_comp count_ext[] = {
                COUNTER_COMP_DIRECTION(count_direction_read),
                COUNTER_COMP_ENABLE(count_enable_read, count_enable_write),
                COUNTER_COMP_CEILING(count_ceiling_read, count_ceiling_write),
        };

```
这使得查看、添加和修改该驱动所支持的属性（"direction"enable" "ceiling"）变得简单，并且可以在不至于迷失于层struct 大括号的情况下维护这段代码
回调必须与相应组件或扩展所期望的函数类型匹配。这些函数类型在 struct counter_comp 结构中定义为 "`**_read`" "`**_write`" 联合成员
上述扩展对应的回调原型如下：

```
        int count_direction_read(struct counter_device *counter,
                                 struct counter_count *count,
                                 enum counter_count_direction *direction);
        int count_enable_read(struct counter_device *counter,
                              struct counter_count *count, u8 *enable);
        int count_enable_write(struct counter_device *counter,
                               struct counter_count *count, u8 enable);
        int count_ceiling_read(struct counter_device *counter,
                               struct counter_count *count, u64 *ceiling);
        int count_ceiling_write(struct counter_device *counter,
                                struct counter_count *count, u64 ceiling);

```
确定要创建哪种类型的扩展，取决于其作用范围
- Signal 扩展是暴露特定于某个 Signal 的信控制的属性。这类属性将存在sysfs 中该 Signal 的目录下
  例如，如果你有一Signal 的反相（invert）特性，你可以创建一个名"invert" Signal 扩展来切换该特性：
  /sys/bus/counter/devices/counterX/signalY/invert

- Count 扩展是暴露特定于某个 Count 的信控制的属性。这类属性将存在sysfs 中该 Count 的目录下
  例如，如果你希望对某Count 的更新进行暂恢复，你可以创建一个名"enable" Count 扩展来切换：
  /sys/bus/counter/devices/counterX/countY/enable

- Device 扩展是暴露不特定于某Count Signal 的信控制的属性。你可以在这里放置全局特性或其他杂项功能
  例如，如果你的设备有过温传感器，你可以通过一个名"error_overtemp" Device 扩展报告芯片过热  /sys/bus/counter/devices/counterX/error_overtemp

## 瀛愮郴缁熸灦鏋。

Counter 驱动以原生方式传递和获取数据（即 `u8`、`u64` 等），而共享的 counter 模块负责 sysfs 接口之间的转换。这保证了所counter 驱动的标准用户空间接口，并通过通用的设备驱ABI 实现了通用 Counter chrdev 接口
以下示例说明了计数值如何从 counter 驱动向下传递的高层视图。驱动回调首先注册到 Counter 核心组件，供其使用：

```
        Driver callbacks registration:
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        +----------------------------+
                        | Counter device driver      |
                        +----------------------------+
                        | Processes data from device |
                        +----------------------------+
                                |
                         -------------------
                        / driver callbacks /
                        -------------------
                                |
                                V
                        +----------------------+
                        | Counter core         |
                        +----------------------+
                        | Routes device driver |
                        | callbacks to the     |
                        | userspace interfaces |
                        +----------------------+
                                |
                         -------------------
                        / driver callbacks /
                        -------------------
                                |
                +---------------+---------------+
                |                               |
                V                               V
        +--------------------+          +---------------------+
        | Counter sysfs      |          | Counter chrdev      |
        +--------------------+          +---------------------+
        | Translates to the  |          | Translates to the   |
        | standard Counter   |          | standard Counter    |
        | sysfs output       |          | character device    |
        +--------------------+          +---------------------+

```
此后，数据可以直接在 Counter 设备与用户空间之间传输，如下所示：

```
        Count data request:
        ~~~~~~~~~~~~~~~~~~~
                         ----------------------
                        / Counter device       \
                        +----------------------+
                        | Count register: 0x28 |
                        +----------------------+
                                |
                         -----------------
                        / raw count data /
                        -----------------
                                |
                                V
                        +----------------------------+
                        | Counter device driver      |
                        +----------------------------+
                        | Processes data from device |
                        |----------------------------|
                        | Type: u64                  |
                        | Value: 42                  |
                        +----------------------------+
                                |
                         ----------
                        / u64     /
                        ----------
                                |
                +---------------+---------------+
                |                               |
                V                               V
        +--------------------+          +---------------------+
        | Counter sysfs      |          | Counter chrdev      |
        +--------------------+          +---------------------+
        | Translates to the  |          | Translates to the   |
        | standard Counter   |          | standard Counter    |
        | sysfs output       |          | character device    |
        |--------------------|          |---------------------|
        | Type: const char * |          | Type: u64           |
        | Value: "42"        |          | Value: 42           |
        +--------------------+          +---------------------+
                |                               |
         ---------------                 -----------------------
        / const char * /                / struct counter_event /
        ---------------                 -----------------------
                |                               |
                |                               V
                |                       +-----------+
                |                       | read      |
                |                       +-----------+
                |                       \ Count: 42 /
                |                        -----------
                |
                V
        +--------------------------------------------------+
        | `/sys/bus/counter/devices/counterX/countY/count` |
        +--------------------------------------------------+
        \ Count: "42"                                      /
         --------------------------------------------------

```
涉及四个主要组件
### Counter 设备驱动

与硬件设备通信以读写数据；例如用于正交编码器、定时器等的 counter 驱动
### Counter 核心

counter 设备驱动注册到系统，以便在用户空间交互期间调用相应的回调
### Counter sysfs

counter 数据转换为标Counter sysfs 接口格式，反之亦然
有关可用通用计数器接sysfs 属性的详细说明，请参阅 Documentation/ABI/testing/sysfs-bus-counter 文件
### Counter chrdev

Counter 事件转换为标Counter 字符设备；数据通过标准字符设备read 调用传输，Counter 事件通过 ioctl 调用配置
## Sysfs 接口


通用计数器接口会生成若干 sysfs 属性，它们位于 `/sys/bus/counter/devices/counterX` 目录下，其中 `X` 为相应计数器设备id。有关每个通用计数器接sysfs 属性的详细信息，请参阅 Documentation/ABI/testing/sysfs-bus-counter
通过这些 sysfs 属性，程序与脚本可以与相应计数器设备的通用计数器范Count、Signal Synapse 进行交互
## Counter 字符设备


Counter 字符设备节点`/dev` 目录下以 `counterX` 创建，其`X` 为相应计数器设备id。标Counter 数据类型的定义通过用户空间 `include/uapi/linux/counter.h` 文件暴露
### Counter 事件

Counter 设备驱动可以通过使用如下函数支持 Counter 事件
```
        void counter_push_event(struct counter_device *const counter, const u8 event,
                                const u8 channel);

```
事件 id `event` 参数指定；事件通道 id `channel` 参数指定。调用此函数时，会收集与相应事件关联Counter 数据，并为每个数据项生成一`struct counter_event`，然后推送到用户空间
Counter 事件可由用户配置，以报告感兴趣的各种 Counter 数据。这可以被概念化为一份待执行Counter 组件 read 调用列表。例如：

        +------------------------+------------------------+
        | COUNTER_EVENT_OVERFLOW | COUNTER_EVENT_INDEX    |
        +========================+========================+
        | Channel 0              | Channel 0              |
        +------------------------+------------------------+
        | ** Count 0              | ** Signal 0             |
        | ** Count 1              | ** Signal 0 Extension 0 |
        | ** Signal 3             | ** Extension 4          |
        | * Count 4 Extension 2  +------------------------+
        | * Signal 5 Extension 0 | Channel 1              |
        |                        +------------------------+
        |                        | * Signal 4             |
        |                        | * Signal 4 Extension 0 |
        |                        | * Count 7              |
        +------------------------+------------------------+

当例如调`counter_push_event(counter, COUNTER_EVENT_INDEX, 1)` 时，它会沿着 `COUNTER_EVENT_INDEX` 事件通道 1 的列表向下执Signal 4、Signal 4 Extension 0 Count 7 read 回调——为每个返回的数据生成一`struct counter_event` 并推kfifo，用户空间可以通过对相应字符设备节点执行标read 操作来获取
### 用户空间

用户空间应用程序可以通过Counter 字符设备节点执行 ioctl 操作来配Counter 事件。以下是受支持并`linux/counter.h` 用户空间头文件提供的 ioctl 代码
- `COUNTER_ADD_WATCH_IOCTL`

- `COUNTER_ENABLE_EVENTS_IOCTL`

- `COUNTER_DISABLE_EVENTS_IOCTL`

要配置事件以收集 Counter 数据，用户首先用相关的事id、事件通道 id，以及要从中读取的所需 Counter 组件的信息填充一`struct counter_watch`，然后通过 `COUNTER_ADD_WATCH_IOCTL` ioctl 命令将其传入
注意，通过`component.type` 成员设置`COUNTER_COMPONENT_NONE`，可以在不收Counter 数据的情况下监视一个事件。在此配置下，Counter 字符设备将仅为那些相应的 `struct counter_event` 元素填充事件时间戳，而忽略组件值
`COUNTER_ADD_WATCH_IOCTL` 命令会缓冲这Counter watch。准备就绪后，可以使`COUNTER_ENABLE_EVENTS_IOCTL` ioctl 命令激活这Counter watch
然后，用户空间应用程序可以在 Counter 字符设备节点上执`read` 操作（可选择先调`poll`），以检索带有所需数据`struct counter_event` 元素