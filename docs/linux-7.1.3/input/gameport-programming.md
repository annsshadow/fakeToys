#### 编写 gameport 驱动


#### 一个基本的经典 gameport


如果 gameport 没有提供超出 inb()/outb() 之外的功能，
```

	struct gameport gameport;

	gameport.io = MY_IO_ADDRESS;
	gameport_register_port(&gameport);

```
确保 struct gameport 的其他所有字段都初始化为 0。gameport 通用代码会负责处理其余部分

如果你的硬件支持多个 io 地址，并且你的驱动可以选择将硬件编程到哪一个，那么从较不常
的地址开始是更好的选择，因为与标准 0x201 地址发生冲突的可能性更小

例如，如果你的驱动支持地址 0x200x208x210 0x218，那0x218 将是首选地址

如果你的硬件支持gameport 地址未映射到 ISA io 空间（高0x1000），则使用该地址
并且不要映射 ISA 镜像

此外，应始终gameport 占用的整io 空间执行 request_region()。尽管真正使用的只有
一ioport，但 gameport 通常占用 io 空间中从 1 16 个地址

请同时考虑->open() 回调中在卡上启用 gameport（如io 映射ISA 空间）——这样它
仅在有真正使用它的时候才占用 io 空间。在 ->close() 回调中再次禁用它。你也可以在
->open() 回调中选择 io 地址，这样当某些可能的地址已被其他 gameport 占用时它也不会失败

#### 内存映射gameport


当一gameport 可以通过 MMIO 访问时，优先采用这种方式，因为它更快，允许每秒更多的
读取次数。注册这样一gameport
```

	struct gameport gameport;

	void my_trigger(struct gameport *gameport)
	{
		my_mmio = 0xff;
	}

	unsigned char my_read(struct gameport *gameport)
	{
		return my_mmio;
	}

	gameport.read = my_read;
	gameport.trigger = my_trigger;
	gameport_register_port(&gameport);

```

#### Cooked 模式 gameport


有些 gameport 能够将轴值以数字形式上报，这意味着驱动不必再以旧方式测量它们——一ADC
已内置在
```

	struct gameport gameport;

	int my_cooked_read(struct gameport *gameport, int *axes, int *buttons)
	{
		int i;

		for (i = 0; i < 4; i++)
			axes[i] = my_mmio[i];
		buttons[0] = my_mmio[4];
	}

	int my_open(struct gameport *gameport, int mode)
	{
		return -(mode != GAMEPORT_MODE_COOKED);
	}

	gameport.cooked_read = my_cooked_read;
	gameport.open = my_open;
	gameport.fuzz = 8;
	gameport_register_port(&gameport);

```
这里唯一令人困惑的是 fuzz 值。最好通过实验确定，它ADC 数据中的噪声量。完美的
gameport 可以将其设为 0，最常见fuzz 8 32 之间。有fuzz 的处理，请参
analog.c input.c——fuzz 值决定了一个用于消除数据中噪声的高斯滤波器窗口的大小

#### 更复杂的 gameport


gameport 可以同时支持 raw cooked 模式。在这种情况下，将示1+2 1+3 组合即可
gameport 可以支持内部校准——见下文，以lightning.c analog.c 了解其工作方式。如
你的驱动同时支持多个 gameport 实例，请使用 gameport 结构体的 ->private 成员指向你的数据

#### 注销一gameport


```

    gameport_unregister_port(&gameport);

```
#### gameport 结构


```

    struct gameport {

	void *port_data;

```
gameport 驱动自由使用的私有指针。（不是摇杆驱动！）

```

	char name[32];

```
由驱动调gameport_set_name() 设置的驱动名称。仅用于信息展示

```

	char phys[32];

```
由驱动调gameport_set_phys() 设置gameport 物理名称/描述。仅用于信息展示

```

	int io;

```
用于 raw 模式I/O 地址。如果你gameport 支持 raw 模式，你必须将此->read() 设置
某个值

```

	int speed;

```
gameport 读取raw 模式速度，以每秒千次读取计

```

	int fuzz;

```
如果 gameport 支持 cooked 模式，则应将其设置为表示数据中噪声量的一个值。参
gameport_pgm_cooked_mode銆。

```

	void (*trigger)(struct gameport *);

```
触发器（Trigger）。此函数应触ns558 单次采样（oneshots）。如果设NULL，则将使
outb(0xff, io)銆。

```

	unsigned char (*read)(struct gameport *);

```
读取按键ns558 单次采样位。如果设NULL，则将改inb(io)

```

	int (*cooked_read)(struct gameport *, int *axes, int *buttons);

```
如果 gameport 支持 cooked 模式，则应将此指向其 cooked 读取函数。它应将 axes[0..3] 填充
摇杆四个轴的值，并将 buttons[^0^] 填充为表示按键的四个位

```

	int (*calibrate)(struct gameport *, int *axes, int *max);

```
用于校准 ADC 硬件的函数。调用时，axes[0..3] 应由调用者用 cooked 数据预填充，max[0..3]
应用每个轴的预期最大值预填充。calibrate() 函数应设ADC 硬件的灵敏度，使最大值能落入
其量程，并重新计axes[] 值以匹配新的灵敏度，或重新从硬件读取它们以给出有效值

```

	int (*open)(struct gameport *, int mode);

```
open() 有两个用途。首先，驱动raw cooked 模式打开端口，open() 回调可以决定支持
哪些模式。其次，资源分配可以在此处进行。端口也可以在此处启用。在此次调用之前，gameport
结构体的其他字段（即 io 成员）无需有效

```

	void (*close)(struct gameport *);

```
close() 应释放由 open 分配的资源，并可能禁gameport

```

	struct timer_list poll_timer;
	unsigned int poll_interval;     /* in msecs */
	spinlock_t timer_lock;
	unsigned int poll_cnt;
	void (*poll_handler)(struct gameport *);
	struct gameport *parent, *child;
	struct gameport_driver *drv;
	struct mutex drv_mutex;		/* protects serio->drv so attributes can pin driver */
	struct device dev;
	struct list_head node;

```
gameport 层内部使用

```

    };

```
祝使用愉快！
