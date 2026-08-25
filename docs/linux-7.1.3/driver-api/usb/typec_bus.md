
## 用于 USB Type-C 交替模式（Alternate Mode）驱动的 API


### 简

交替模式需要使USB Type-C USB Power Delivery 规范中定义的供应商定义消息（VDM）与对端通信该通信SVID（Standard Vendor ID，标准或供应ID）特定的，即对每个交替模式都是特定的，因每个交替模式都需要一个自定义驱动
USB Type-C 总线允许通过使用 SVID 和模式号，将驱动绑定到被发现的对端交替模式
USB Type-C Connector Class <typec> 为端口支持的每个交替模式提供一个设备，并为对端支持的每个交替模提供单独的设备。交替模式的驱动被绑定到对端交替模式设备，而端口交替模式设备必须由端口驱动处理
当一个新的对端交替模式设备被注册时，它会被链接到它所连接端口的、具有匹SVID 和模式的交替模式设备端口驱动与交替模式驱动之间的通信将使用相同的 API 进行
端口交替模式设备被用作对端与交替模式驱动之间的代理，因此端口驱动只需将来自交替模式驱动的、SVID 特定命令传递给对端，以及将对端的命令传递给交替模式驱动。端口驱动不需要任何直接的 SVID 特定通信，但端口驱动
需要提供端口交替模式设备的操作回调，就像交替模式驱动需要为对端交替模式设备提供它们一样
### 用法

#### 一

默认情况下，交替模式驱动负责进入该模式。也可以将进入模式的决策留给用户空间（参`Documentation/ABI/testing/sysfs-class-typec`）。端口驱动不应自行进入任何模式
`->vdm` 是操作回调向量中最重要的回调。它将用于把来自对端的、所SVID 特定的命令传递给交替模式驱动对于端口驱动则反之。驱动之间使`typec_altmode_vdm()` 互相发SVID 特定的命令
如果使用 SVID 特定的命令与对端通信的结果需要重新配置连接器上的引脚，交替模式驱动需要使`typec_altmode_notify()` 通知总线。驱动将协商得到SVID 特定引脚配置值作为参数传递给该函数。总线驱动
随后将使用该值作为多路复用器（mux）的状态值，来配置连接器后面的多路复用器
注意：SVID 特定的引脚配置值必须始终从 `TYPEC_STATE_MODAL` 开始。USB Type-C 规范为连接器定义了两默认状态：`TYPEC_STATE_USB` `TYPEC_STATE_SAFE`。这些值被总线保留为状态的前几个可能值。当进入交替模式时，
总线会在发USB Type-C 规范定义Enter Exit Mode 命令之前，将连接器置`TYPEC_STATE_SAFE`，并模式退出后将连接器放回 `TYPEC_STATE_USB`
一SVID 特定引脚配置的可行定义示例为
```

    enum {
        ALTMODEX_CONF_A = TYPEC_STATE_MODAL,
        ALTMODEX_CONF_B,
        ...
    };

```
```

```
#define ALTMODEX_CONF_A = TYPEC_MODAL_STATE(0);
#define ALTMODEX_CONF_B = TYPEC_MODAL_STATE(1);

#### 线缆插头交替模式


交替模式驱动不会被绑定到线缆插头交替模式设备，只绑定到对端交替模式设备。如果该交替模式支持或要求一响应 SOP Prime（以及可选的 SOP Double Prime）消息的线缆，该交替模式的驱动必须使`typec_altmode_get_plug()`
请求线缆插头交替模式的处理句柄，并接管它们的控制
### 驱动 API


#### 交替模式结构

   :functions: typec_altmode_driver typec_altmode_ops

#### 交替模式驱动的注注销


   :functions: typec_altmode_register_driver typec_altmode_unregister_driver

#### 交替模式驱动操作


   :functions: typec_altmode_enter typec_altmode_exit typec_altmode_attention typec_altmode_vdm typec_altmode_notify

#### 用于端口驱动API


   :functions: typec_match_altmode

#### 线缆插头操作


   :functions: typec_altmode_get_plug typec_altmode_put_plug
