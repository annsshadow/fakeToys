
## 用户空间 DTX（剪贴板分离系统）接


`surface_dtx` 驱动负责妥善的剪贴板分离与重新连接处理。为此，它提供了
`/dev/surface/dtx` 设备文件，通过它它可以与一个用户空间守护进程交互。该守护进程
最终负责确定并采取必要的动作，例如卸载连接到基座上的设备、卸重新加载图形驱动
向用户发出通知等

本驱动中使用两个基本的通信原则：命令（在文档的其他部分也称为请求）和事件。命
发送给 EC，在不同的上下文中可能有不同的含义。事件由 EC 在某个内部状态发生变化时
发送。命令总是由驱动发起，而事件总是EC 发起


## 术语


- **剪贴板（Clipboard）：**
  Surface Book 的可拆卸上部，容纳了屏幕CPU

- **基座（Base）：**
  Surface Book 的下部，剪贴板可以从其上拆卸下来，并可选地（取决于型号）容纳独
  GPU（dGPU）

- **闩锁（Latch）：**
  在正常操作中将剪贴板固定在基座上的机构，并在被请求时允许其分离

- **被静默忽略的命令（Silently ignored commands）：**
  该命令被 EC 作为一个有效命令接受并确认（遵循标准通信协议），EC 不会对其采取
  行动，即忽略它


## 分离过程


警告：本文档的这一部分基于逆向工程和测试，因此可能包含错误或不完整

### 闩锁状


闩锁机构有两个主要状态：**打开（open* **关闭（closed*。在**关闭**状
（默认）下，剪贴板被固定在基座上，而在**打开**状态下，剪贴板可以被用户取下

此外，闩锁可以被锁定，相应地也可以被解锁，这会影响分离过程。具体而言，该锁定机制
旨在防止位于设备基座中的 dGPU 在使用时被热拔插。更多细节可以在下面分离过程的文
中找到。默认情况下，闩锁是解锁的

### 分离过程


请注意，分离过程完全EC 控制。`surface_dtx` 驱动只把事件EC 转发到用户空间，
并把命令从用户空间转发到 EC，即它不影响此过程

分离过程由用户按下设备基座上*分离（detach*按钮或执`SDTX_IOCTL_LATCH_REQUEST`
IOCTL 开始。随后：

1. EC 打开分离按钮上的指示灯，发送一*分离请求（detach-request*事件
   （`SDTX_EVENT_REQUEST`），并等待进一步的指令/命令。如果闩锁是解锁的，指示灯会
   闪烁绿色。如果闩锁已被锁定，指示灯会常亮红色

2. 该事件经`surface_dtx` 驱动转发到用户空间，在那里一个合适的用户空间守护进程
   可以处理它，并通过本驱动提供的 IOCTL 将指令发回给 EC

3. EC 等待来自用户空间的指令，并据此行动。如EC 在给定时间内没有收到任何指令
   它将超时并按如下方式继续

   - 如果闩锁是解锁的，EC 将打开闩锁，剪贴板可以从基座上分离。这与没有本驱动或任
     用户空间守护进程时的行为完全一致。关EC 的后续行为，详见下面
     `SDTX_IOCTL_LATCH_CONFIRM` 描述

   - 如果闩锁是锁定的，EC *不会**打开闩锁，这意味着剪贴板无法从基座上分离。此外，
     EC 会发送一个取消事件（`SDTX_EVENT_CANCEL`），详细说明这一点，取消原因
     `SDTX_DETACH_TIMEDOUT`（详见事件部分）

用户空间守护进程对分离请求事件的有效响应有：

- 执行 `SDTX_IOCTL_LATCH_REQUEST`。这将立即中止分离过程。此外，EC 会发送一个分
  请求事件，类似于用户按下分离按钮来取消所述过程（见下文）

- 执行 `SDTX_IOCTL_LATCH_CONFIRM`。这将导EC 打开闩锁，之后用户可以将剪贴板与
  基座分开

  由于这会更改闩锁状态，一旦闩锁成功打开，就会发送一*闩锁状态（latch-status*
  事件（`SDTX_EVENT_LATCH_STATUS`）。如EC 未能打开闩锁，例如由于硬件错误或电量
  过低，则会发送一个闩锁取消事件（`SDTX_EVENT_CANCEL`），其取消原因指示具体的
  失败

  如果闩锁当前是锁定的，在打开之前会自动解锁

- 执行 `SDTX_IOCTL_LATCH_HEARTBEAT`。这将重置内部超时。不会执行其他动作，即分
  过程既不会被完成也不会被取消，EC 仍会等待进一步的响应

- 执行 `SDTX_IOCTL_LATCH_CANCEL`。这将中止分离过程，类似于上面描述的
  `SDTX_IOCTL_LATCH_REQUEST` 或下面描述的按钮按下。作为响应会发送一*通用请求**
  事件（`SDTX_EVENT_REQUEST`）。然而，与它们不同的是，如果当前没有正在进行的分
  过程，此命令不会触发一个新的分离过程

- 什么都不做。分离过程最终会如第 3 点所述超时

关于这些响应的更多细节，请参ioctls

需要重点注意的是，如果用户在分离操作进行中的任何时候（EC 已发送初始的
**分离请求**事件（`SDTX_EVENT_REQUEST`）之后，且在它收到结束该过程的相应响应之前）
按下分离按钮，分离过程会EC 层面被取消，并发送一个相同的事件。因此，一
**分离请求**事件本身并不表示分离过程的开始

分离过程还可能因硬件故障或剪贴板电量过低而被 EC 进一步取消。这是通过一个带有相
取消原因的取消事件（`SDTX_EVENT_CANCEL`）完成的


## 用户空间接口文档


### 错误码与状态


错误和状态码被分为不同类别，可用于判断该状态码是否为一个错误，如果是，该错误的
严重性和类型是什么。当前类别有

   :widths: 2 1 3
   :header-rows: 1

   - - Name
     - Value
     - Short Description

   - - `STATUS`
     - `0x0000`
     - 非错误状态码

   - - `RUNTIME_ERROR`
     - `0x1000`
     - 非关键的运行时错误

   - - `HARDWARE_ERROR`
     - `0x2000`
     - 关键硬件故障

   - - `UNKNOWN`
     - `0xF000`
     - 未知错误码

其他类别保留供将来使用。`SDTX_CATEGORY()` 宏可用于判断任意状态值的类别
`SDTX_SUCCESS()` 宏可用于检查该状态值是否为一个成功值（`SDTX_CATEGORY_STATUS`
或是否指示一个失败

EC 发送的未知状态或错误码由驱动归入 `UNKNOWN` 类别，并可能在未来通过它们自己
代码实现

当前使用的错误码有：

   :widths: 2 1 1 3
   :header-rows: 1

   - - Name
     - Category
     - Value
     - Short Description

   - - `SDTX_DETACH_NOT_FEASIBLE`
     - `RUNTIME`
     - `0x1001`
     - 由于剪贴板电量过低，分离不可行

   - - `SDTX_DETACH_TIMEDOUT`
     - `RUNTIME`
     - `0x1002`
     - 闩锁锁定时分离过程超时

   - - `SDTX_ERR_FAILED_TO_OPEN`
     - `HARDWARE`
     - `0x2001`
     - 未能打开闩锁

   - - `SDTX_ERR_FAILED_TO_REMAIN_OPEN`
     - `HARDWARE`
     - `0x2002`
     - 未能保持闩锁打开

   - - `SDTX_ERR_FAILED_TO_CLOSE`
     - `HARDWARE`
     - `0x2003`
     - 未能关闭闩锁

其他错误码保留供将来使用。非错误状态码可能重叠，并且通常仅在其使用场景内是唯一的：

   :widths: 2 1 1 3
   :header-rows: 1

   - - Name
     - Category
     - Value
     - Short Description

   - - `SDTX_LATCH_CLOSED`
     - `STATUS`
     - `0x0000`
     - 闩锁已关已经被关闭

   - - `SDTX_LATCH_OPENED`
     - `STATUS`
     - `0x0001`
     - 闩锁已打开/已经被打开

   :widths: 2 1 1 3
   :header-rows: 1

   - - Name
     - Category
     - Value
     - Short Description

   - - `SDTX_BASE_DETACHED`
     - `STATUS`
     - `0x0000`
     - 基座已分不存在

   - - `SDTX_BASE_ATTACHED`
     - `STATUS`
     - `0x0001`
     - 基座已连存在

同样，其他码保留供将来使用


### 事件


事件可以通过从设备文件读取来接收。默认情况下它们是禁用的，必须首先执
`SDTX_IOCTL_EVENTS_ENABLE` 才能启用。所有事件都遵循 |sdtx_event| 规定的布局
具体的事件类型可以通过它们的事件码识别，事件码|sdtx_event_code| 中描述。注
其他事件码保留供将来使用，因此事件解析器必须能够依靠事件头中给出的有效载荷长度，
优雅地处理任何未不支持的事件类型

当前提供的事件类型有

   :widths: 2 1 1 3
   :header-rows: 1

   - - Name
     - Code
     - Payload
     - Short Description

   - - `SDTX_EVENT_REQUEST`
     - `1`
     - `0` bytes
     - 分离过程已启已中止

   - - `SDTX_EVENT_CANCEL`
     - `2`
     - `2` bytes
     - EC 取消了分离过程

   - - `SDTX_EVENT_BASE_CONNECTION`
     - `3`
     - `4` bytes
     - 基座连接状态已改变

   - - `SDTX_EVENT_LATCH_STATUS`
     - `4`
     - `2` bytes
     - 闩锁状态已改变

   - - `SDTX_EVENT_DEVICE_MODE`
     - `5`
     - `2` bytes
     - 设备模式已改变

各事件的更多细节

##### ``SDTX_EVENT_REQUEST``


当分离过程被用户启动或（如果正在进行）被中止时发送，无论是通过按下分离按钮还是
从用户空间发送分离请求（`SDTX_IOCTL_LATCH_REQUEST`）

没有任何有效载荷

##### ``SDTX_EVENT_CANCEL``


当分离过程因未满足前置条件（例如剪贴板电量过低无法分离）或硬件故障而被 EC 取消
发送。取消原因在下面的事件有效载荷中给出，可以是以下之一

- `SDTX_DETACH_TIMEDOUT`：闩锁锁定时分离超时。闩锁既未被打开也未被解锁

- `SDTX_DETACH_NOT_FEASIBLE`：由于剪贴板电量过低，分离不可行

- `SDTX_ERR_FAILED_TO_OPEN`：无法打开闩锁（硬件故障）

- `SDTX_ERR_FAILED_TO_REMAIN_OPEN`：无法保持闩锁打开（硬件故障）

- `SDTX_ERR_FAILED_TO_CLOSE`：无法关闭闩锁（硬件故障）

此上下文中的其他错误码保留供将来使用

这些码可以通过 `SDTX_CATEGORY()` 宏分类，以区分关键硬件错
（`SDTX_CATEGORY_HARDWARE_ERROR`）或运行时错误（`SDTX_CATEGORY_RUNTIME_ERROR`），
后者在分离的某个前置条件未满足时可能在正常操作中发生

   :widths: 1 1 4
   :header-rows: 1

   - - Field
     - Type
     - Description

   - - `reason`
     - |__u16|
     - 取消原因

##### ``SDTX_EVENT_BASE_CONNECTION``


当基座连接状态发生改变时发送，即当基座已被连接、分离，或由于剪贴板电量过低分离
变得不可行时。新状态以及（如果连接了基座）基座ID 作为类型 |sdtx_base_info| 
有效载荷提供，其布局如下

   :widths: 1 1 4
   :header-rows: 1

   - - Field
     - Type
     - Description

   - - `state`
     - |__u16|
     - 基座连接状态

   - - `base_id`
     - |__u16|
     - 所连接基座的类型（无则0）

`state` 的可能取值有

- `SDTX_BASE_DETACHED`銆。
- `SDTX_BASE_ATTACHED` 以及
- `SDTX_DETACH_NOT_FEASIBLE`銆。

其他值保留供将来使用

##### ``SDTX_EVENT_LATCH_STATUS``


当闩锁状态发生改变时发送，即当闩锁被打开、关闭，或发生错误时。当前状态作为有效载
提供

   :widths: 1 1 4
   :header-rows: 1

   - - Field
     - Type
     - Description

   - - `status`
     - |__u16|
     - 闩锁状态

`status` 的可能取值有

- `SDTX_LATCH_CLOSED`銆。
- `SDTX_LATCH_OPENED`銆。
- `SDTX_ERR_FAILED_TO_OPEN`銆。
- `SDTX_ERR_FAILED_TO_REMAIN_OPEN` 以及
- `SDTX_ERR_FAILED_TO_CLOSE`銆。

其他值保留供将来使用

##### ``SDTX_EVENT_DEVICE_MODE``


当设备模式发生改变时发送。新的设备模式作为有效载荷提供：

   :widths: 1 1 4
   :header-rows: 1

   - - Field
     - Type
     - Description

   - - `mode`
     - |__u16|
     - 设备操作模式

`mode` 的可能取值有

- `SDTX_DEVICE_MODE_TABLET`銆。
- `SDTX_DEVICE_MODE_LAPTOP` 以及
- `SDTX_DEVICE_MODE_STUDIO`銆。

其他值保留供将来使用


### IOCTLs


提供了以IOCTL

   :widths: 1 1 1 1 4
   :header-rows: 1

   - - Type
     - Number
     - Direction
     - Name
     - Description

   - - `0xA5`
     - `0x21`
     - `-`
     - `EVENTS_ENABLE`
     - 为当前文件描述符启用事件

   - - `0xA5`
     - `0x22`
     - `-`
     - `EVENTS_DISABLE`
     - 为当前文件描述符禁用事件

   - - `0xA5`
     - `0x23`
     - `-`
     - `LATCH_LOCK`
     - 锁定闩锁

   - - `0xA5`
     - `0x24`
     - `-`
     - `LATCH_UNLOCK`
     - 解锁闩锁

   - - `0xA5`
     - `0x25`
     - `-`
     - `LATCH_REQUEST`
     - 请求剪贴板分离

   - - `0xA5`
     - `0x26`
     - `-`
     - `LATCH_CONFIRM`
     - 确认剪贴板分离请求

   - - `0xA5`
     - `0x27`
     - `-`
     - `LATCH_HEARTBEAT`
     - EC 发送心跳信号

   - - `0xA5`
     - `0x28`
     - `-`
     - `LATCH_CANCEL`
     - 取消分离过程

   - - `0xA5`
     - `0x29`
     - `R`
     - `GET_BASE_INFO`
     - 获取当前基座/连接信息

   - - `0xA5`
     - `0x2A`
     - `R`
     - `GET_DEVICE_MODE`
     - 获取当前设备操作模式

   - - `0xA5`
     - `0x2B`
     - `R`
     - `GET_LATCH_STATUS`
     - 获取当前设备闩锁状态

##### ``SDTX_IOCTL_EVENTS_ENABLE``


定义`_IO(0xA5, 0x22)`

为当前文件描述符启用事件。如果已启用，可以通过从设备读取来获取事件。默认情况下
事件是禁用的

##### ``SDTX_IOCTL_EVENTS_DISABLE``


定义`_IO(0xA5, 0x22)`

为当前文件描述符禁用事件。如果已启用，可以通过从设备读取来获取事件。默认情况下
事件是禁用的

##### ``SDTX_IOCTL_LATCH_LOCK``


定义`_IO(0xA5, 0x23)`

锁定闩锁，导致分离过程在超时不打开闩锁的情况下中止。默认情况下闩锁是解锁的。如
闩锁已经锁定，此命令会被静默忽略

##### ``SDTX_IOCTL_LATCH_UNLOCK``


定义`_IO(0xA5, 0x24)`

解锁闩锁，导致分离过程在超时打开闩锁。默认情况下闩锁是解锁的。此命令在正在进行的
分离过程中发送时不会打开闩锁。如果闩锁已经解锁，它会被静默忽略

##### ``SDTX_IOCTL_LATCH_REQUEST``


定义`_IO(0xA5, 0x25)`

通用闩锁请求。行为取决于上下文：如果没有活动的分离过程，则请求分离。否则当前活跃的
分离过程将被中止

如果分离过程被此操作取消，将发送一个通用分离请求事件（`SDTX_EVENT_REQUEST`）

这本质上与按下分离按钮的行为相同

##### ``SDTX_IOCTL_LATCH_CONFIRM``


定义`_IO(0xA5, 0x26)`

确认并确认一个闩锁请求。如果在正在进行的分离过程中发送，此命令会导致闩锁立即
打开。即使闩锁已被锁定也会被打开。在这种情况下，闩锁锁被重置为解锁状态

如果当前没有正在进行的分离过程，此命令会被静默忽略

##### ``SDTX_IOCTL_LATCH_HEARTBEAT``


定义`_IO(0xA5, 0x27)`

发送一个心跳，本质上是重置分离超时。此命令可用于在分离成功所需的工作仍在进行时
让分离过程保持存活

如果当前没有正在进行的分离过程，此命令会被静默忽略

##### ``SDTX_IOCTL_LATCH_CANCEL``


定义`_IO(0xA5, 0x28)`

取消正在进行的分离（如果有）。如果分离过程被此操作取消，将发送一个通用分离请求事件
（`SDTX_EVENT_REQUEST`）

如果当前没有正在进行的分离过程，此命令会被静默忽略

##### ``SDTX_IOCTL_GET_BASE_INFO``


定义`_IOR(0xA5, 0x29, struct sdtx_base_info)`

获取当前的基座连接状态（即已连接/已分离）以及连接到剪贴板的基座类型。此命令本质
提供了一种查询由基座连接改变事件（`SDTX_EVENT_BASE_CONNECTION`）提供的信息的方式

`struct sdtx_base_info.state` 的可能取值有

- `SDTX_BASE_DETACHED`銆。
- `SDTX_BASE_ATTACHED` 以及
- `SDTX_DETACH_NOT_FEASIBLE`銆。

其他值保留供将来使用

##### ``SDTX_IOCTL_GET_DEVICE_MODE``


定义`_IOR(0xA5, 0x2A, __u16)`

返回设备操作模式，指示基座是否以及如何连接到剪贴板。此命令本质上提供了一种查询由
设备模式改变事件（`SDTX_EVENT_DEVICE_MODE`）提供的信息的方式

返回的值有

- `SDTX_DEVICE_MODE_LAPTOP`
- `SDTX_DEVICE_MODE_TABLET`
- `SDTX_DEVICE_MODE_STUDIO`

详见 |sdtx_device_mode|。其他值保留供将来使用


##### ``SDTX_IOCTL_GET_LATCH_STATUS``


定义`_IOR(0xA5, 0x2B, __u16)`

获取当前闩锁状态，或（推测）尝试打开/关闭闩锁时遇到的最后一个错误。此命令本质上提
了一种查询由闩锁状态改变事件（`SDTX_EVENT_LATCH_STATUS`）提供的信息的方式

返回的值有

- `SDTX_LATCH_CLOSED`銆。
- `SDTX_LATCH_OPENED`銆。
- `SDTX_ERR_FAILED_TO_OPEN`銆。
- `SDTX_ERR_FAILED_TO_REMAIN_OPEN` 以及
- `SDTX_ERR_FAILED_TO_CLOSE`銆。

其他值保留供将来使用

### 关于基座 ID 的说


通过 `SDTX_EVENT_BASE_CONNECTION` `SDTX_IOCTL_GET_BASE_INFO` 提供的基座类ID
直接在组合的 |__u16| 值的低字节中EC 转发，驱动将提供ID EC 类型存储在高字节
中（没有这个，不EC 类型上的基座 ID 可能会重叠）

`SDTX_DEVICE_TYPE()` 宏可用于判断 EC 设备类型。可以是以下之一

- `SDTX_DEVICE_TYPE_HID`，用于通过 HID Surface Aggregator Module；以

- `SDTX_DEVICE_TYPE_SSH`，用于通过 Surface Serial Hub Surface Aggregator Module

注意，目前仅支持 `SSH` 类型EC，但 `HID` 类型保留供将来使用

### 结构体与枚举



## API 用户


一个使用此 API 的用户空间守护进程可以在
https://github.com/linux-surface/surface-dtx-daemon 找到
