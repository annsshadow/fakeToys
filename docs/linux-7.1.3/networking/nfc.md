## Linux NFC 子系统


近场通信（NFC）子系统用于标准化 NFC 设备驱动的开发，并创建一个统一的用户空间接口。

本文档涵盖架构概览、设备驱动接口描述以及用户空间接口描述。

## 架构概览


NFC 子系统负责：
      - NFC 适配器管理；
      - 轮询（polling）目标；
      - 底层数据交换；

子系统被划分为若干部分。'core' 负责提供设备驱动接口。另一方面，它也负责提供用于控制操作和
底层数据交换的接口。

控制操作通过 generic netlink 对用户空间可用。

底层数据交换接口由新的套接字族 PF_NFC 提供。NFC_SOCKPROTO_RAW 执行与 NFC 目标的原始通信。


        +--------------------------------------+
        |              USER SPACE              |
        +--------------------------------------+
            ^                       ^
            | low-level             | control
            | data exchange         | operations
            |                       |
            |                       v
            |                  +-----------+
            | AF_NFC           |  netlink  |
            | socket           +-----------+
            | raw                   ^
            |                       |
            v                       v
        +---------+            +-----------+
        | rawsock | <--------> |   core    |
        +---------+            +-----------+
                                    ^
                                    |
                                    v
                               +-----------+
                               |  driver   |
                               +-----------+

## 设备驱动接口


向 NFC 子系统注册时，设备驱动必须告知核心所支持的 NFC 协议集合以及一组 ops 回调。必须实现的
ops 回调如下：

- start_poll - 配置设备以轮询目标
- stop_poll - 停止进行中的轮询操作
- activate_target - 选中并初始化找到的目标之一
- deactivate_target - 取消选中并反初始化选中的目标
- data_exchange - 发送数据并接收响应（transceive 操作）

## 用户空间接口


用户空间接口分为控制操作和底层数据交换操作。

### 控制操作


generic netlink 用于实现控制操作的接口。这些操作由命令和事件组成，全部列举如下：

- NFC_CMD_GET_DEVICE - 获取特定设备信息或转储设备列表
- NFC_CMD_START_POLL - 配置特定设备以轮询目标
- NFC_CMD_STOP_POLL - 停止特定设备中的轮询操作
- NFC_CMD_GET_TARGET - 转储特定设备找到的目标列表

- NFC_EVENT_DEVICE_ADDED - 报告一个 NFC 设备被添加
- NFC_EVENT_DEVICE_REMOVED - 报告一个 NFC 设备被移除
- NFC_EVENT_TARGETS_FOUND - 当找到 1 个或多个目标时报告 START_POLL 的结果

用户必须调用 START_POLL 来轮询 NFC 目标，通过 NFC_ATTR_PROTOCOLS 属性传递期望的 NFC 协议。
设备会一直保持轮询状态，直到找到任何目标。不过，用户可以通过调用 STOP_POLL 命令停止轮询
操作。在这种情况下，会检查 STOP_POLL 的请求者是否与 START_POLL 的相同。

如果轮询操作找到了一个或多个目标，会发送 TARGETS_FOUND 事件（包含设备 id）。用户必须调用
GET_TARGET 来获取该设备找到的所有目标列表。每条回复消息都带有目标属性，其中包含相关信息，
例如所支持的 NFC 协议。

通过一个 netlink 套接字请求的所有轮询操作在其被关闭时停止。

### 底层数据交换


用户空间必须使用 PF_NFC 套接字来执行与
```

        struct sockaddr_nfc {
               sa_family_t sa_family;
               __u32 dev_idx;
               __u32 target_idx;
               __u32 nfc_protocol;
        };

```
目标的任何数据通信。要与一个目标建立连接，用户必须创建一个 NFC_SOCKPROTO_RAW 套接字，并
以正确填充的 sockaddr_nfc 结构调用 'connect' 系统调用。所有信息来自 NFC_EVENT_TARGETS_FOUND
netlink 事件。由于一个目标可以支持多个 NFC 协议，用户必须告知它想使用哪个协议。

在内部，'connect' 会导致对驱动的一次 activate_target 调用。当套接字关闭时，目标被反激活。

通过套接字交换的数据格式取决于 NFC 协议。例如，与 MIFARE 标签通信时，交换的数据是 MIFARE
命令及其响应。

第一个收到的包是对第一个发出的包的响应，依此类推。为了允许有效的“空”响应，每个收到的数据
都有一个 1 字节的 NULL 头。
