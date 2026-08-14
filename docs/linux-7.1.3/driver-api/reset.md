
## Reset controller API（复位控制器 API）


## 简介（Introduction）


复位控制器（reset controller）是控制多个外设复位信号的中央单元。
复位控制器 API 分为两部分：
`consumer driver interface <#consumer-driver-interface>`__（`API 参考
<#reset-consumer-api>`__），用于让外设驱动请求控制其复位输入信号；
以及 `reset controller driver interface
<#reset-controller-driver-interface>`__（`API 参考
<#reset-controller-driver-api>`__），供复位控制器设备的驱动用来注册其复位
控制，从而提供给使用者（consumer）使用。

虽然某些复位控制器硬件单元也实现了系统重启功能，但重启处理程序不在
复位控制器 API 的范畴之内。

### 术语表（Glossary）


复位控制器 API 对以下术语有特定含义：

Reset line（复位线）

    从复位控制器硬件单元连接到外设模块的、承载复位信号的物理复位线。

Reset control（复位控制）

    决定一条或多条复位线状态的控制方法。最常见的形式是在复位控制器
    寄存器空间中的一个单独位，它要么允许直接控制复位线的物理状态，
    要么是自清零的，可用于在复位线上触发一个预定的脉冲。
    在更为复杂的复位控制中，一次触发动作可以启动多条复位线上一组
    经过精确计时的脉冲序列。

Reset controller（复位控制器）

    一个硬件模块，提供若干复位控制以控制若干复位线。

Reset consumer（复位使用者）

    由复位线上的信号置入复位状态的外设模块或外部 IC。

## Consumer driver interface（使用者驱动接口）


该接口提供的 API 类似于内核时钟框架（clock framework）。使用者驱动
使用 get 和 put 操作来获取和释放复位控制。
还提供了用于断言（assert）和解除断言（deassert）所控制的复位线、触发
复位脉冲以及查询复位线状态的函数。

在请求复位控制时，使用者可以使用其复位输入的符号名称，由核心将其映射到
某个已有复位控制器设备上的实际复位控制。

当复位控制器框架未被使用时，会提供一个该 API 的桩（stub）版本，以尽量
减少对 ifdef 的使用需求。

### Shared and exclusive resets（共享与独占复位）


复位控制器 API 提供引用计数的解除/断言，或者直接、独占的控制。
共享（shared）与独占（exclusive）复位控制的区分在请求复位控制时做出，
既可以通过 devm_reset_control_get_shared()，也可以通过
devm_reset_control_get_exclusive()。
这一选择决定了使用该复位控制时 API 调用的行为。

共享复位的行为类似于内核时钟框架中的时钟。
它们提供引用计数的解除断言：只有第一次解除断言（将解除断言引用计数
增加到 1）和最后一次断言（将解除断言引用计数减回到 0）才会对复位线产生
实际的物理影响。

而独占复位则保证直接控制。也就是说，一次断言会让复位线立即被断言，
一次解除断言会让复位线立即被解除断言。

### Assertion and deassertion（断言与解除断言）


使用者驱动使用 reset_control_assert() 和 reset_control_deassert() 函数
来断言和解除断言复位线。对于共享复位控制，对这两个函数的调用必须保持平衡。

注意，由于多个使用者可能会使用同一个共享复位控制，因此无法保证在共享
复位控制上调用 reset_control_assert() 就一定会使复位线被断言。
使用共享复位控制的消费者驱动应当假定复位线可能始终保持解除断言状态。
该 API 仅保证：只要有任何使用者请求将其解除断言，复位线就不会被断言。

### Triggering（触发）


使用者驱动使用 reset_control_reset() 在自解除断言的复位控制上触发一个
复位脉冲。一般而言，这些复位不能在多个使用者之间共享，因为任何一个使用者
驱动请求脉冲都会复位所有相连的外设。

复位控制器 API 允许将自解除断言的复位控制作为共享请求，但对于此类控制，
只有第一次触发请求才会在复位线上真正发出一个脉冲。
在该复位控制的所有使用者都调用 reset_control_rearm() 之前，对该函数的
后续调用均无效。对于共享复位控制，对这两个函数的调用必须保持平衡。
这使得那些只需要在驱动探测或恢复之前的任意时刻进行一次初始复位的设备
可以共享一条脉冲式复位线。

### Querying（查询）


只有部分复位控制器支持通过 reset_control_status() 查询复位线的当前状态。
若支持，当给定的复位线处于断言状态时，该函数返回正的非零值。
reset_control_status() 函数不接受 `reset control array <#reset-control-arrays>`__
句柄作为其输入参数。

### Optional resets（可选复位）


外设常常在某些平台上需要复位线，而在另一些平台上则不需要。
为此，可以使用 devm_reset_control_get_optional_exclusive() 或
devm_reset_control_get_optional_shared() 将复位控制作为可选来请求。
当所请求的复位控制在设备树中未指定时，这些函数返回一个 NULL 指针而不是
错误。将 NULL 指针传给复位控制函数会使它们安静地返回而不产生错误。

### Reset control arrays（复位控制数组）


某些驱动需要以任意顺序断言一组复位线。devm_reset_control_array_get()
返回一个不透明的复位控制句柄，可用于一次性断言、解除断言或触发所有指定的
复位控制。复位控制 API 不保证其中各个控制被处理的顺序。

## Reset controller driver interface（复位控制器驱动接口）


复位控制器模块的驱动提供断言或解除断言复位信号、在复位线上触发复位脉冲或
查询其当前状态所需的功能。所有函数都是可选的。

### Initialization（初始化）


驱动在探测（probe）函数中填充一个 struct `reset_controller_dev` 结构体，
并通过 reset_controller_register() 注册它。实际的功能通过 struct
`reset_control_ops` 中的回调函数实现。

## API reference（API 参考）


复位控制器 API 在此分为两部分进行说明：
`reset consumer API <#reset-consumer-api>`__ 和 `reset controller
driver API <#reset-controller-driver-api>`__。

### Reset consumer API（使用者 API）


复位使用者可以使用一个不透明的复位控制句柄来控制复位线，该句柄可由
devm_reset_control_get_exclusive() 或 devm_reset_control_get_shared() 获得。
得到复位控制后，使用者可以调用 reset_control_assert() 和
reset_control_deassert()，使用 reset_control_reset() 触发复位脉冲，或使用
reset_control_status() 查询复位线状态。

   :internal:

   :functions: reset_control_reset
               reset_control_assert
               reset_control_deassert
               reset_control_status
               reset_control_acquire
               reset_control_release
               reset_control_rearm
               reset_control_put
               of_reset_control_get_count
               devm_reset_control_array_get
               reset_control_get_count

### Reset controller driver API（控制器驱动 API）


复位控制器驱动应当在 static 常量结构体 `reset_control_ops` 中实现所需的
函数，分配并填充一个 struct `reset_controller_dev`，并通过
devm_reset_controller_register() 注册它。

   :internal:

   :functions: of_reset_simple_xlate
               reset_controller_register
               reset_controller_unregister
               devm_reset_controller_register
