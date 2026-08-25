
## Linux LAPB 模块接口

版本 1.3

Jonathan Naylor 29.12.96

变更（Henner Eisen000-10-29）：data_indication() 的返回值改int

LAPB 模块将是一个单独编译的模块，供 Linux 操作系统中任何需LAPB 服务的部分使用。本文档
定义了该模块的接口以及它所提供的服务。这里的“模块”一词并不暗LAPB 模块是一个可单独加载模块，尽管它可以是。该术语是在更标准的意义上使用的
LAPB 模块的接口由以下部分组成：调用模块的函数、模块回调以指示重要的状态变化，以及用于
获取和设置模块相关信息的数据结构
### 结构

可能最重要的结构是持有接收和发送数据的 skbuff 结构，但这超出了本文档的范围
两个 LAPB 特有的结构是 LAPB 初始化结构和 LAPB 参数结构。它们将在标准头文件 <linux/lapb.h>
中定义。头文件 <net/lapb.h> LAPB 模块内部使用的，不可使用
### LAPB 初始化结
该结构只在调lapb_register（见下）时使用一次。它包含关于需要该服务的设备驱动的信息
```
	struct lapb_register_struct {
		void (*connect_confirmation)(int token, int reason);
		void (*connect_indication)(int token, int reason);
		void (*disconnect_confirmation)(int token, int reason);
		void (*disconnect_indication)(int token, int reason);
		int  (*data_indication)(int token, struct sk_buff *skb);
		void (*data_transmit)(int token, struct sk_buff *skb);
	};

```
该结构的每个成员都对应设备驱动中的一个函数，LAPB 模块中发生特定事件时会被调用。这些将下面详述。如果不需要某个回调（！！），则可以传NULL
### LAPB 参数结构

该结构与 lapb_getparms lapb_setparms 函数（见下）一起使用。它们用于允许设备驱动获取和设置
```
	struct lapb_parms_struct {
		unsigned int t1;
		unsigned int t1timer;
		unsigned int t2;
		unsigned int t2timer;
		unsigned int n2;
		unsigned int n2count;
		unsigned int window;
		unsigned int state;
		unsigned int mode;
	};

```
T1 T2 是协议时序参数，单位100ms。N2 是在链路被宣告失败之前的最大重试次数。窗口大小是
允许远端未确认的最大在途数据包数量；对于标LAPB 链路，窗口值在 1 7 之间，对于扩LAPB
链路，在 1 127 之间
mode 变量是一个位域，用于设置（目前）三个值。这些位域的含义如下
======  =================================================
Bit	含义
======  =================================================
0	LAPB 操作=LAPB_STANDARD 1=LAPB_EXTENDED）1	[SM]LP 操作=LAPB_SLP 1=LAPB=MLP）2	DTE/DCE 操作=LAPB_DTE 1=LAPB_DCE3-31	保留，必须为 0======  =================================================

扩展 LAPB 操作表示使用扩展序列号，从而允许更大的窗口大小，默认是标准 LAPB 操作。MLP 操作SLP 操作相同，只LAPB 使用的地址不同以指示操作模式，默认是单链路过程（Single Link
Procedure）。DCE DTE 操作的区别在于：(i) 用于命令和响应的地址ii) DCE 未连接时，它
每隔 T1 发送一次不带轮询位（poll）的 DM。这些大写常量名将在公共 LAPB 头文件中定义
### 函数

LAPB 模块提供了多个函数入口点
```
    int lapb_register(void *token, struct lapb_register_struct);

```
这必须在 LAPB 模块被使用之前调用。如果调用成功，则返LAPB_OK。token 必须是设备驱动生成的
唯一标识符，以便唯一标识 LAPB 链路的实例。它LAPB 模块在所有回调中返回，并被设备驱动在
所有对 LAPB 模块的调用中使用。对于单个设备驱动中的多LAPB 链路，必须进行多lapb_register
调用。lapb_register_struct 的格式如上所述。返回值为
=============		=============================
LAPB_OK			LAPB 注册成功LAPB_BADTOKEN		token 已被注册LAPB_NOMEM		内存不足
=============		=============================

```
    int lapb_unregister(void *token);

```
这会释放LAPB 链路关联的所有资源。任何当前的 LAPB 链路都将被放弃，不再传递进一步的消息在此调用之后，token 的值对于任何对 LAPB 函数的调用都不再有效。有效的返回值为
=============		===============================
LAPB_OK			LAPB 注销成功LAPB_BADTOKEN		无效/未知LAPB token=============		===============================

```
    int lapb_getparms(void *token, struct lapb_parms_struct *parms);

```
这允许设备驱动获取当LAPB 变量的值，lapb_parms_struct 如上所述。有效的返回值为
=============		=============================
LAPB_OK			LAPB getparms 成功LAPB_BADTOKEN		无效/未知LAPB token=============		=============================

```
    int lapb_setparms(void *token, struct lapb_parms_struct *parms);

```
这允许设备驱动设置当LAPB 变量的值，lapb_parms_struct 如上所述。t1timer、t2timer n2count
的值会被忽略，同样，在已连接时更改 mode 位也会被忽略。出错意味着没有任何值被改变。有效的
返回值为
=============		=================================================
LAPB_OK			LAPB getparms 成功LAPB_BADTOKEN		无效/未知LAPB tokenLAPB_INVALUE		某个值超出了其允许的范围=============		=================================================

```
    int lapb_connect_request(void *token);

```
使用当前参数设置发起连接。有效的返回值为
==============		=================================
LAPB_OK			LAPB 正在开始连接LAPB_BADTOKEN		无效/未知LAPB tokenLAPB_CONNECTED		LAPB 模块已连接==============		=================================

```
    int lapb_disconnect_request(void *token);

```
发起断开连接。有效的返回值为
=================	===============================
LAPB_OK			LAPB 正在开始断开连接LAPB_BADTOKEN		无效/未知LAPB tokenLAPB_NOTCONNECTED	LAPB 模块未连接=================	===============================

```
    int lapb_data_request(void *token, struct sk_buff *skb);

```
将数据排队到 LAPB 模块，以便通过链路发送。如果调用成功，skbuff LAPB 模块所有，设备驱动
不得再次使用它。有效的返回值为
=================	=============================
LAPB_OK			LAPB 已接受数据LAPB_BADTOKEN		无效/未知LAPB tokenLAPB_NOTCONNECTED	LAPB 模块未连接=================	=============================

```
    int lapb_data_received(void *token, struct sk_buff *skb);

```
将从设备接收到的数据排队LAPB 模块。期望传递给 LAPB 模块的数据的 skb->data 指向 LAPB 数据开头。如果调用成功，skbuff LAPB 模块所有，设备驱动不得再次使用它。有效的返回值为
=============		===========================
LAPB_OK			LAPB 已接受数据LAPB_BADTOKEN		无效/未知LAPB token=============		===========================

### 回调

这些回调是设备驱动提供给 LAPB 模块、在发生事件时调用的函数。它们通过 lapb_register（见上）结构 lapb_register_struct（见上）中向 LAPB 模块注册
```
    void (*connect_confirmation)(void *token, int reason);

```
当在调用 lapb_connect_request（见上）请求之后连接建立时，LAPB 模块调用。reason 总是
LAPB_OK銆。
```
    void (*connect_indication)(void *token, int reason);

```
当链路由远程系统建立时，LAPB 模块调用。reason 的值总是 LAPB_OK
```
    void (*disconnect_confirmation)(void *token, int reason);

```
当设备驱动调lapb_disconnect_request（见上）之后发生事件时，LAPB 模块调用。reason 指示
发生了什么。在所有情况下，LAPB 链路都可视为已终止。reason 的取值为
=================	====================================================
LAPB_OK			LAPB 链路正常终止LAPB_NOTCONNECTED	远程系统未连接LAPB_TIMEDOUT		N2 次尝试中都未收到远程系统的响应=================	====================================================

```
    void (*disconnect_indication)(void *token, int reason);

```
当链路被远程系统终止或发生其他事件导致链路终止时，由 LAPB 模块调用。如果远程系统拒绝了请求这也可能作为lapb_connect_request（见上）的响应而返回。reason 的取值为
=================	====================================================
LAPB_OK			LAPB 链路被远程系统正常终止LAPB_REFUSED		远程系统拒绝了连接请求LAPB_NOTCONNECTED	远程系统未连接LAPB_TIMEDOUT		N2 次尝试中都未收到远程系统的响应=================	====================================================

```
    int (*data_indication)(void *token, struct sk_buff *skb);

```
当从远程系统接收到应传递给协议栈下一层的数据时，LAPB 模块调用。skbuff 成为设备驱动的财产，
LAPB 模块不会再对它执行任何操作。skb->data 指针将指LAPB 头部之后的第一个数据字节
当且仅当该帧在交付给上层之前被丢弃时，该方法应返NET_RX_DROP（定义于头文include/linux/netdevice.h）
```
    void (*data_transmit)(void *token, struct sk_buff *skb);

```
当数据要由设备驱动发送到远程系统时，LAPB 模块调用。skbuff 成为设备驱动的财产，LAPB 模块
不会再对它执行任何操作。skb->data 指针将指LAPB 头部的第一个字节