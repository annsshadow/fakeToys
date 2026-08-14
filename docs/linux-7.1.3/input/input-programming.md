## 编写输入设备驱动


#### 最简单的示例


下面是一个极其简单的输入设备驱动示例。该设备只有一个按钮，且按钮可在 i/o 端口 BUTTON_PORT 处访问。当

```

    #include <linux/input.h>
    #include <linux/module.h>
    #include <linux/init.h>

    #include <asm/irq.h>
    #include <asm/io.h>

    static struct input_dev *button_dev;

    static irqreturn_t button_interrupt(int irq, void *dummy)
    {
	    input_report_key(button_dev, BTN_0, inb(BUTTON_PORT) & 1);
	    input_sync(button_dev);
	    return IRQ_HANDLED;
    }

    static int __init button_init(void)
    {
	    int error;

	    if (request_irq(BUTTON_IRQ, button_interrupt, 0, "button", NULL)) {
		    printk(KERN_ERR "button.c: Can't allocate irq %d\n", button_irq);
		    return -EBUSY;
	    }

	    button_dev = input_allocate_device();
	    if (!button_dev) {
		    printk(KERN_ERR "button.c: Not enough memory\n");
		    error = -ENOMEM;
		    goto err_free_irq;
	    }

	    button_dev->evbit[0] = BIT_MASK(EV_KEY);
	    button_dev->keybit[BIT_WORD(BTN_0)] = BIT_MASK(BTN_0);

	    error = input_register_device(button_dev);
	    if (error) {
		    printk(KERN_ERR "button.c: Failed to register device\n");
		    goto err_free_dev;
	    }

	    return 0;

    err_free_dev:
	    input_free_device(button_dev);
    err_free_irq:
	    free_irq(BUTTON_IRQ, button_interrupt);
	    return error;
    }

    static void __exit button_exit(void)
    {
	    input_unregister_device(button_dev);
	    free_irq(BUTTON_IRQ, button_interrupt);
    }

    module_init(button_init);
    module_exit(button_exit);

```
#### 示例的作用


首先它必须包含 <linux/input.h> 文件，该文件是输入子系统的接口。它提供了所需的所有定义。

在 _init 函数中（无论是在加载模块时还是在启动内核时调用），它会获取所需的资源（也应当检查设备是否存在）。

然后它通过 input_allocate_device() 分配一个新的输入设备结构体，并设置输入位域。这样设备驱动就告诉输入系统的其他部分它是什么——该输入设备能生成或接受哪些事件。我们的示例设备只能生成 EV_KEY 类型的事件，而且在这些事件中仅 BTN_0 事件码。因此我们只设置这些位，

```

	set_bit(EV_KEY, button_dev->evbit);
	set_bit(BTN_0, button_dev->keybit);

```
同样如此，但当涉及的位不止一个时，第一种写法往往更简短。

```

	input_register_device(button_dev);

```
这将 button_dev 结构体加入输入驱动的链表中，并调用设备处理模块的 _connect 函数，以告知它们出现了一个新输入设备。input_register_device() 可能会睡眠，因此不能从中断上下文或持有自旋锁时调用。

```

	button_interrupt()

```
它会在按钮每次产生中断时检查其状态，并报告

```

	input_report_key()

```
给输入系统。无需检查中断例程是否向输入系统报告了两个相同值的事件（例如两次按下），因为 input_report_* 函数自身会做这个检查。

```

	input_sync()

```
调用用于告诉那些接收事件的对象：我们已经发送了一份完整的报告。在只有一个按钮的情况下这似乎无关紧要，但对于鼠标移动等情况则相当重要——你不会希望 X 和 Y 值被分别解释，因为那样会导致不同的移动。

#### dev->open() 与 dev->close()


如果驱动必须反复轮询设备，因为设备不会产生中断，而轮询的开销又过大无法一直进行；或者设备使用了宝贵资源（例如中断），那么它可以利用 open 和 close 回调来得知何时可以停止轮询或释放中断，以及何时必须恢复轮询或获取中断。

```

    static int button_open(struct input_dev *dev)
    {
	    if (request_irq(BUTTON_IRQ, button_interrupt, 0, "button", NULL)) {
		    printk(KERN_ERR "button.c: Can't allocate irq %d\n", button_irq);
		    return -EBUSY;
	    }

	    return 0;
    }

    static void button_close(struct input_dev *dev)
    {
	    free_irq(IRQ_AMIGA_VERTB, button_interrupt);
    }

    static int __init button_init(void)
    {
	    ...
	    button_dev->open = button_open;
	    button_dev->close = button_close;
	    ...
    }

```
注意，输入核心会记录设备的用户数量，并确保只有在第一个用户连接到设备时才调用 dev->open()，以及只在最后一个用户断开连接时才调用 dev->close()。对这两个回调的调用是串行化的。

open() 回调成功时应返回 0，失败时应返回任意非零值。close() 回调（其返回类型为 void）必须始终成功。

#### 抑制（inhibit）输入设备


抑制一个设备意味着忽略来自它的输入事件。因此，它关乎与输入处理程序之间关系的维护——无论是已有的关系，还是在设备处于抑制状态期间将要建立的关系。

如果一个设备被抑制，任何输入处理程序都不会收到来自它的事件。

通过利用“没有人需要该设备的事件”这一事实，在抑制（inhibit）和解抑制（uninhibit）操作时，分别调用设备的 close()（如果有用户）和 open()（如果有用户），可以进一步加以利用。的确，close() 的含义是停止向输入核心提供事件，而 open() 的含义是开始向输入核心提供事件。

在抑制时调用设备的 close() 方法（如果有用户）可以让驱动节省功耗。无论是直接关闭设备电源，还是在驱动使用运行时 PM 时释放它在 open() 中获取的运行时 PM 引用都可以。

抑制与解抑制，与输入处理程序打开和关闭设备是正交的。用户空间可能希望在任何一个处理程序被正向匹配之前，就提前抑制某个设备。

抑制与解抑制，也与设备是否为唤醒源是正交的。是否为唤醒源在系统睡眠时起作用，而不是在系统运行时起作用。驱动应当如何编程其抑制、睡眠与作为唤醒源之间的交互，是驱动特定的事情。

打个比方，网卡——把网络接口 down 掉，并不意味着不应能通过此接口在 LAN 上唤醒系统。因此，可能存在即使被抑制也应被视为唤醒源的输入驱动。实际上，在许多 I2C 输入设备中，它们的中断被声明为唤醒中断，其处理发生在驱动核心中，而驱动核心并不知道输入特定的抑制（也不应该知道）。包含多个接口的复合设备可以基于每个接口被抑制，例如抑制某一个接口不应影响该设备作为唤醒源的能力。

如果一个设备在被抑制期间要被视为唤醒源，则在对它的 suspend() 编程时必须格外小心，因为它可能需要调用设备的 open()。取决于 close() 对相应设备的含义，在睡眠前不调用 open() 可能使它无法提供任何唤醒事件。无论如何设备都要进入睡眠。

#### 基本事件类型


最简单的事件类型是 EV_KEY，用于按键和按钮。

```

	input_report_key(struct input_dev *dev, int code, int value)

```
参见 uapi/linux/input-event-codes.h 了解 code 的可允许取值（从 0 到 KEY_MAX）。value 被解释为真值，即任意非零值表示按键按下，零值表示按键松开。输入代码仅在 value 与之前不同的情况下才生成事件。

除了 EV_KEY，还有两种更基本的事件类型：EV_REL 和 EV_ABS。它们用于设备提供的相对值和绝对值。相对值例如可以是鼠标在 X 轴上的移动。鼠标将其报告为相对上次位置的位移，因为它没有任何可工作的绝对坐标系统。绝对事件则用于摇杆和数字化仪——那些确实工作在绝对坐标系统中的设备。

让设备报告 EV_REL 按钮与 EV_KEY 一样简单；只需

```

	input_report_rel(struct input_dev *dev, int code, int value)

```
函数。仅对非零值生成事件。

然而 EV_ABS 需要一点特别留意。在调用 input_register_device 之前，你要为设备的每个绝对轴在 input_dev 结构体中填充额外字段。如果我们的按钮设备还有

```

	button_dev.absmin[ABS_X] = 0;
	button_dev.absmax[ABS_X] = 255;
	button_dev.absfuzz[ABS_X] = 4;
	button_dev.absflat[ABS_X] = 8;

```

```

	input_set_abs_params(button_dev, ABS_X, 0, 255, 4, 8);

```
这个设置适用于摇杆的 X 轴，最小值为 0，最大值为 255（摇杆**必须**能够达到，即使有时报告更大值也没问题，但它必须始终能达到最小值和最大值），数据噪声最大为 ±4，中心平坦区大小为 8。

如果你不需要 absfuzz 和 absflat，可以将它们设为零，这表示该设备精确且总是精确回到中心位置（如果有的话）。

#### BITS_TO_LONGS()、BIT_WORD()、BIT_MASK()


```

	BITS_TO_LONGS(x) - 返回 x 个比特对应的位域数组（以 long 计）的长度
	BIT_WORD(x)	 - 返回比特 x 在数组中的 long 索引
	BIT_MASK(x)	 - 返回比特 x 在一个 long 中的索引

```
#### id* 与 name 字段


dev->name 应由输入设备驱动在注册输入设备之前设置。它是一个形如 'Generic button device' 的字符串，包含设备对用户友好的名称。

id* 字段包含设备的总线 ID（PCI、USB 等）、厂商 ID 和设备 ID。总线 ID 定义于 input.h 中。厂商和设备 ID 定义于 pci_ids.h、usb_ids.h 及类似的头文件中。这些字段应由输入设备驱动在注册之前设置。

idtype 字段可用于输入设备驱动的特定信息。

id 和 name 字段可以通过 evdev 接口传递给用户空间。

#### keycode、keycodemax、keycodesize 字段


这三个字段应由具有密集键映射的输入设备使用。keycode 是一个数组，用于从扫描码映射到输入系统的键码。keycode max 应包含数组的大小，keycodesize 则包含其中每个条目的大小（以字节计）。

用户空间可以使用对应 evdev 接口上的 EVIOCGKEYCODE 和 EVIOCSKEYCODE ioctl 来查询和修改当前的扫描码到键码映射。当一个设备填好了上述全部三个字段，驱动可以依赖内核默认实现的键码映射设置与查询。

#### dev->getkeycode() 与 dev->setkeycode()


getkeycode() 和 setkeycode() 回调允许驱动覆盖由输入核心提供的默认 keycode/keycodesize/keycodemax 映射机制，并实现稀疏键码映射。

#### 按键自动重复


……很简单。它由 input.c 模块处理。不使用硬件自动重复，因为它在许多设备中并不存在，即使在存在的地方有时也是坏的（例如键盘：东芝笔记本）。要为你的设备启用自动重复，只需在 dev->evbit 中设置 EV_REP 即可。其余全部由输入系统处理。

#### 其他事件类型、处理输出事件


到目前为止的其他事件类型有：

- EV_LED - 用于键盘 LED。
- EV_SND - 用于键盘蜂鸣。

它们与例如按键事件非常相似，但方向相反——从系统到输入设备驱动。如果你的输入设备驱动能处理这些事件，它必须在 evbit 中设置相应的位，

```

    button_dev->event = button_event;

    int button_event(struct input_dev *dev, unsigned int type,
		     unsigned int code, int value)
    {
	    if (type == EV_SND && code == SND_BELL) {
		    outb(value, BUTTON_BELL);
		    return 0;
	    }
	    return -1;
    }

```
该回调例程可以从中断或 BH（下半部）中调用（尽管这不是硬性规定），因此绝不能睡眠，且必须尽快完成。

#### 轮询式输入设备


输入轮询通过传入一个输入设备结构体和回调来设置，

```

    int input_setup_polling(struct input_dev *dev,
        void (*poll_fn)(struct input_dev *dev))

```
在回调内部，设备应当使用其他设备所使用的常规 input_report_* 函数和 input_sync。

```

    void input_set_poll_interval(struct input_dev *dev, unsigned int interval)

```
它用于配置设备被轮询的间隔，以毫秒为单位。
