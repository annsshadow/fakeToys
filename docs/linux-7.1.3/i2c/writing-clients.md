## 实现 I2C 设备驱动


这是一份为 I2C SMBus 设备编写内核驱动的小指南，使Linux 作为协议主机/主设（master，而非从设slave）
要建立一个驱动，你需要做若干件事。有些是可选的，有些事情可以用略有不同或完全不同的
方式完成。请将本文作为指南，而非规则手册
## 总体说明


尽量保持内核命名空间尽可能干净。最好的办法是为所有全局符号使用一个唯一的前缀。这导出的符号尤其重要，但为非导出的符号这样做也是个好主意。在本教程中我们将使用前缀
`foo_`銆。
## 驱动结构


通常，你会实现一个单独的驱动结构体，并从中实例化所有客户端（client）。请记住，驱结构体包含通用的访问例程，除了你提供数据的字段外，应做零初始化。客户端结构体保设备特定的信息，如驱动模型（driver model）设备节点及I2C 地址
```

  static const struct i2c_device_id foo_idtable[] = {
	{ "foo", my_id_for_foo },
	{ "bar", my_id_for_bar },
	{ }
  };
  MODULE_DEVICE_TABLE(i2c, foo_idtable);

  static struct i2c_driver foo_driver = {
	.driver = {
		.name	= "foo",
		.pm	= &foo_pm_ops,	/* 可*/
	},

	.id_table	= foo_idtable,
	.probe		= foo_probe,
	.remove		= foo_remove,

	.shutdown	= foo_shutdown,	/* 可*/
	.command	= foo_command,	/* 可选，已废*/
  }

```

name 字段是驱动名称，且不能包含空格。它应该与模块名称匹配（如果驱动可以编译为模块）尽管你可以使MODULE_ALIAS（本例中传入“foo”）来为模块添加另一个名称。如果驱动名与模块名称不匹配，模块将不会被自动加载（热插hotplug/冷插拔）
所有其他字段都是回调函数，将在下文说明
## 额外的客户端数据


每个客户端结构体都有一个特殊的 `data` 字段，可以指向任意结构体。你应该用它来保设备特定的数据
```

	/* 瀛樺偍鍊?*/
	void i2c_set_clientdata(struct i2c_client *client, void *data);

	/* 鍙栧嚭鍊?*/
	void *i2c_get_clientdata(const struct i2c_client *client);

```

注意，从内核 2.6.34 起，你不再需要在 remove() 中或 probe() 失败时将`data` 字段
设为 NULL。i2c-core 会在这些情况下自动完成。这些也是核心唯一会触碰该字段的时机
## 访问客户

假设我们有一个有效的客户端结构体。在某些时刻，我们需要从该客户端收集信息，或客户端写入新信息
我发现为此定foo_read foo_write 函数很有用。在某些情况下，直接调用 I2C 函数
会更简单，但许多芯片都有某种寄存器-值的抽象，可以轻松封装
下面的函数是简单示例，不应直接照搬
```

  int foo_read_value(struct i2c_client *client, u8 reg)
  {
	if (reg < 0x10)	/* 字节大小的寄存器 */
		return i2c_smbus_read_byte_data(client, reg);
	else		/* 瀛楀ぇ灏忕殑瀵勫瓨鍣?*/
		return i2c_smbus_read_word_data(client, reg);
  }

  int foo_write_value(struct i2c_client *client, u8 reg, u16 value)
  {
	if (reg == 0x10)	/* 不可- 驱动错误*/
		return -EINVAL;
	else if (reg < 0x10)	/* 字节大小的寄存器 */
		return i2c_smbus_write_byte_data(client, reg, value);
	else			/* 瀛楀ぇ灏忕殑瀵勫瓨鍣?*/
		return i2c_smbus_write_word_data(client, reg, value);
  }


```

## 探测与挂

Linux I2C 协议栈最初是为访PC 主板上的硬件监控芯片而编写的，因此曾内嵌一些更
适用SMBus（以PC）而非 I2C 的假设。其中一个假设是大多数适配器和设备驱动支持
SMBUS_QUICK 协议来探测设备是否存在。另一个假设是，仅使用此类探测原语就足以充分配设备和它们的驱动
随着 Linux 及其 I2C 协议栈在嵌入式系统以DVB 适配器等复杂组件中得到了更广泛的使用这些假设变得问题更大。发出中断的 I2C 设备驱动需要更多（且不同）的配置信息；无法
通过协议探测区分的芯片变体，或需要某些板级特定信息才能正确运行的芯片，其驱动也是如此
### 设备/驱动绑定


系统基础设施（通常是板级特定的初始化代码或引导固件）会报告存在哪些 I2C 设备。例如，
在内核或引导加载程序中可能有一张表，标I2C 设备并将它们与有IRQ 及其他连线信息芯片类型等的板级特定配置相关联。这可用于为每个 I2C 设备创建 i2c_client 对象
使用这种绑定模型I2C 设备驱动Linux 中任何其他类型的驱动工作方式一样：它们提供
一probe() 方法以绑定到这些设备，以及一remove() 方法以解绑
```

	static int foo_probe(struct i2c_client *client);
	static void foo_remove(struct i2c_client *client);

```

请记住，i2c_driver 并不会创建那些客户端句柄。该句柄可能foo_probe() 期间被使用如果 foo_probe() 报告成功（零而非负的状态码），它可以保存该句柄并在 foo_remove() 返回
前一直使用。大多数 Linux 驱动都使用这种绑定模型
id_table name 字段与设备名称匹配时，会调用 probe 函数。如probe 函数需要该
条目，它可以使用以下方式获取

```

	const struct i2c_device_id *id = i2c_match_id(foo_idtable, client);


```

### 设备创建


如果你确切知道某I2C 设备连接到了给定I2C 总线上，你可以通过简单地填充一i2c_board_info 结构体（包含设备地址和驱动名称）并调i2c_new_client_device() 实例化该设备。这将创建设备，然后驱动核心会负责找到正确的驱动并调用其 probe() 方法如果驱动支持不同的设备类型，你可以使type 字段指定你想要的类型。如果需要，你还可以
指定一IRQ 和平台数据（platform data）
有时你知道某设备连接到了给定I2C 总线，但不知道它使用的确切地址。例TV 适配就存在这种情况：同一个驱动支持几十种略有不同的型号，I2C 设备地址在不同型号间会变化在这种情况下，你可以使用 i2c_new_scanned_device() 变体，它i2c_new_client_device()
类似，只是它额外接受一个需要探测的可能I2C 地址列表。会为列表中第一个有响应的地址
创建设备。如果你期望在该地址范围内存在多个设备，只需多次调用 i2c_new_scanned_device() 即可
i2c_new_client_device() i2c_new_scanned_device() 的调用通常发生I2C 总线
驱动中。你可能想保存返回的 i2c_client 引用以便后续使用
### 设备探测


设备探测机制有一些缺点。你需要某种可靠的方式来识别受支持的设备（通常使用设备特定的专用的识别寄存器），否则很可能发生误探，事情会很快变糟。请记住，I2C 协议不包含任检测给定地址上是否存在芯片的标准方法，更不用说识别设备的标准方法了。更糟的是总线
传输缺少语义关联，这意味着同一个传输可能被一个芯片视为读操作，而被另一个芯片视为写操作出于这些原因，设备探测被视为一种遗留机制，不应在新代码中使用
### 设备删除


每个使用 i2c_new_client_device() i2c_new_scanned_device() 创建I2C 设备，都可以
通过调用 i2c_unregister_device() 来注销。如果你不显式调用它，它会在底层 I2C 总线自身
被移除之前自动调用，因为设备无法在驱动模型中存活于其父设备之后
## 初始化驱

当内核启动，或当你的 foo 驱动模块被插入时，你必须做一些初始化工作。幸运的是，通常需注册驱动模块就足够了
```

  static int __init foo_init(void)
  {
	return i2c_add_driver(&foo_driver);
  }
  module_init(foo_init);

  static void __exit foo_cleanup(void)
  {
	i2c_del_driver(&foo_driver);
  }
  module_exit(foo_cleanup);

  module_i2c_driver() 宏可用于精简上述代码
  module_i2c_driver(foo_driver);

```

注意，某些函数被标记`__init`。这些函数可以在内核启动（或模块加载）完成后被移除同样，标记为 `__exit` 的函数在代码被构建进内核时会被编译器丢弃，因为它们永远不会被
调用
## 驱动信息


```

  /* 替换为你自己姓名和邮箱地址 */
  MODULE_AUTHOR("Frodo Looijaard <frodol@dds.nl>"
  MODULE_DESCRIPTION("Driver for Barf Inc. Foo I2C devices");

  /* 也允许少数非 GPL 许可证类*/
  MODULE_LICENSE("GPL");


```

## 电源管理


如果你的 I2C 设备在进入系统低功耗状态时——例如将收发器置于低功耗模式，或激活系唤醒机制——需要特殊处理，请通过为驱动的 dev_pm_ops 实现相应的回调（suspend resume）来完成
这些是标准的驱动模型调用，其工作方式与任何其他驱动协议栈一样。这些调用可以睡眠，并且
可以使用 I2C 消息与被挂起或恢复的设备I2C 消息（因为它们的I2C 适配器在这些调用
发出时是活动的，IRQ 仍然启用）
## 系统关机


如果你的 I2C 设备在系统关机或重启（包kexec）时需要特殊处理——例如关闭某些东西—请使shutdown() 方法
同样，这是一个标准的驱动模型调用，工作方式与其他任何驱动协议栈一样：这些调用可以睡眠并且可以使用 I2C 消息
## 命令函数


支持一个类ioctl 的通用回调函数。你很少需要它，而且它的使用已被废弃，因此新的设不应使用它
## 发送与接收


如果你想与设备通信，有几个函数可以做到。你可以<linux/i2c.h> 中找到它们全部
如果你可以在普I2C 通信SMBus 级别通信之间选择，请使用后者。所有适配器都理解
SMBus 级别命令，但只有部分理解普I2C
### 普I2C 通信


```

	int i2c_master_send(struct i2c_client *client, const char *buf,
			    int count);
	int i2c_master_recv(struct i2c_client *client, char *buf, int count);

```

这些例程从客户端读取或向客户端写入一些字节。客户端包含 I2C 地址，因此你不必包含它第二个参数包含要写的字节，第三个是要写的字节数（必须小于缓冲区长度，且也应小64k，因msg.len u16。）返回的是实际写的字节数
```

	int i2c_transfer(struct i2c_adapter *adap, struct i2c_msg *msg,
			 int num);

```

这会发送一系列消息。每条消息可以是读或写，并且可以以任意方式混合。这些事务被合并事务之间不发出停止（stop）条件。i2c_msg 结构体对每个消息包含客户端地址、消息的字节以及消息数据本身
你可以阅i2c-protocol.rst 文件以了解关于实I2C 协议的更多信息
### SMBus 通信


```

	s32 i2c_smbus_xfer(struct i2c_adapter *adapter, u16 addr,
			   unsigned short flags, char read_write, u8 command,
			   int size, union i2c_smbus_data *data);

```

这是通用SMBus 函数。下面所有函数都基于它实现。绝不要直接使用这个函数
```

	s32 i2c_smbus_read_byte(struct i2c_client *client);
	s32 i2c_smbus_write_byte(struct i2c_client *client, u8 value);
	s32 i2c_smbus_read_byte_data(struct i2c_client *client, u8 command);
	s32 i2c_smbus_write_byte_data(struct i2c_client *client,
				      u8 command, u8 value);
	s32 i2c_smbus_read_word_data(struct i2c_client *client, u8 command);
	s32 i2c_smbus_write_word_data(struct i2c_client *client,
				      u8 command, u16 value);
	s32 i2c_smbus_read_block_data(struct i2c_client *client,
				      u8 command, u8 *values);
	s32 i2c_smbus_write_block_data(struct i2c_client *client,
				       u8 command, u8 length, const u8 *values);
	s32 i2c_smbus_read_i2c_block_data(struct i2c_client *client,
					  u8 command, u8 length, u8 *values);
	s32 i2c_smbus_write_i2c_block_data(struct i2c_client *client,
					   u8 command, u8 length,
					   const u8 *values);

```

这些函数曾因无人使用而从 i2c-core 中移除，但可```

	s32 i2c_smbus_write_quick(struct i2c_client *client, u8 value);
	s32 i2c_smbus_process_call(struct i2c_client *client,
				   u8 command, u16 value);
	s32 i2c_smbus_block_process_call(struct i2c_client *client,
					 u8 command, u8 length, u8 *values);

```

所有这些事务在失败时返回负errno 值。“write”事务成功时返回 0；“read”事务返回读的值，但块（block）事务除外——它们返回读取的值的数量。块缓冲区不必长32 字节
你可以阅smbus-protocol.rst 文件以了解关于实SMBus 协议的更多信息
## 通用例程


下面列出了所有未被提及的通用例程
```

	/* 返回特定适配器的适配器编*/
	int i2c_adapter_id(struct i2c_adapter *adap);

```
