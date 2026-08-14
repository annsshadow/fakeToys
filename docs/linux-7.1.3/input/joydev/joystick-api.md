
## Programming Interface


:作者: Ragnar Hojland Espinosa <ragnar@macula.net> - 1998 年 8 月 7 日

## Introduction


   本文档描述旧的 `js` 接口。建议新客户端切换到通用的事件（`evdev`）接口。

1.0 版驱动采用一种全新的、基于事件的摇杆驱动方式。用户程序不再主动轮询摇杆
数值，而是由摇杆驱动仅在状态发生变化时上报。更多信息请参阅摇杆软件包中附带的
joystick-api.txt、joystick.h 以及 jstest.c。摇杆设备可在阻塞或非阻塞模式下使用，
并支持 select() 调用。

为保持向后兼容，旧的（v0.x）接口依然保留。任何使用旧接口对摇杆驱动的调用都会
返回与旧接口兼容的数值。该接口仍局限于 2 个轴，且使用它的应用程序通常只解码
2 个按键，尽管驱动最多可提供 32 个。

## Initialization


按常规语义（即使用 open）打开摇杆设备。由于驱动现在上报事件而非轮询变化，在
open 之后它会立即发出一系列合成事件（JS_EVENT_INIT），你可以通过读取它们来获得
摇杆的初始状态。

```

	int fd = open ("/dev/input/js0", O_RDONLY);


```
## Event Reading


```

	struct js_event e;
	read (fd, &e, sizeof(e));

```
```

	struct js_event {
		__u32 time;     /* event timestamp in milliseconds */
		__s16 value;    /* value */
		__u8 type;      /* event type */
		__u8 number;    /* axis/button number */
	};

```
如果读取成功，除非你希望像 3.1 节所述那样在一次 read 中读取多个事件，否则它将
返回 sizeof(e)。


### js_event.type


```

	#define JS_EVENT_BUTTON         0x01    /* button pressed/released */
	#define JS_EVENT_AXIS           0x02    /* joystick moved */
	#define JS_EVENT_INIT           0x80    /* initial state of device */

```
如上所述，驱动在 open 时会发出带有 JS_EVENT_INIT 标志（ORed）的合成事件。也就是说，
当它发出一个 INIT BUTTON 事件时，
```

	int type = JS_EVENT_BUTTON | JS_EVENT_INIT;	/* 0x81 */

```
如果你选择不区分合成事件与真实事件
```

	type &= ~JS_EVENT_INIT;				/* 0x01 */


```
### js_event.number


`number` 的值对应于产生该事件的轴或按键。注意它们使用各自独立的编号（即你同时
拥有轴 0 和按键 0）。通常，

        =============== =======
	轴		编号
        =============== =======
	第一轴 X	0
	第一轴 Y	1
	第二轴 X	2
	第二轴 Y	3
	……以此类推
        =============== =======

方向帽（hat）因摇杆类型而异。有些可朝 8 个方向移动，有些只能朝 4 个方向。但无论
硬件是否允许独立移动，驱动始终将方向帽上报为两个独立的轴。


### js_event.value


对于一个轴，`value` 是介于 -32767 与 +32767 之间的有符号整数，表示该摇杆沿该轴的
位置。如果你在摇杆处于 `dead`（死区）时没有读到 0，或者它不能覆盖完整范围，则应
重新校准（例如使用 jscal）。

对于一个按键，按下事件的 `value` 为 1，释放事件的 `value` 为 0。

```

	if (js_event.type == JS_EVENT_BUTTON) {
		buttons_state ^= (1 << js_event.number);
	}

```
如果你单独处理 JS_EVENT_INIT 事件，可能会工作得很好，

```

	if ((js_event.type & ~JS_EVENT_INIT) == JS_EVENT_BUTTON) {
		if (js_event.value)
			buttons_state |= (1 << js_event.number);
		else
			buttons_state &= ~(1 << js_event.number);
	}

```
这样做要安全得多，因为它不会与驱动失去同步。由于你不得不在第一段代码中为
JS_EVENT_INIT 事件编写单独的处理函数，这种方式最终反而更简短。


### js_event.time


事件产生的时间保存在 `js_event.time` 中。它是自……某个过去时刻以来的毫秒数。这
方便了检测双击、判断轴的移动与按键按下是否同时发生等类似任务。


## Reading


如果你以阻塞模式打开设备，一次 read 将一直阻塞（即等待），直到有事件产生并被
实际读取。如果你不能无限等待（诚然，那是很长的一段时间；），有两种替代方案

	a) 使用 select 等待 fd 上有数据可读，或直到超时。select(2) 的手册页上有一个
	   很好的示例。

	b) 以非阻塞模式（O_NONBLOCK）打开设备


### O_NONBLOCK


如果在 O_NONBLOCK 模式下 read 返回 -1，这不一定是“真实”错误（请检查 errno(3)）；
它可能只是表示驱动队列中尚无可读取的事件。你应当读取队列中的所有事件（即一直
读到返回 -1 为止）。

例如，

```

	while (1) {
		while (read (fd, &e, sizeof(e)) > 0) {
			process_event (e);
		}
		/* EAGAIN is returned when the queue is empty */
		if (errno != EAGAIN) {
			/* error */
		}
		/* do something interesting with processed events */
	}

```
清空队列的一个原因是，如果队列变满，由于队列容量有限，你将开始丢失事件，较旧的
事件会被覆盖。

另一个原因是你想知道发生的所有事情，而不是把处理推迟到以后。

队列为何会变满？因为你没有如上所述清空队列，或者因为两次读取之间间隔过长，从而
在队列中产生了过多事件。注意高系统负载可能会进一步拉大这些读取的间隔。

如果读取之间的时间足以填满队列并丢失事件，驱动将切换到启动模式，下次你读取时
会生成合成事件（JS_EVENT_INIT）来告知你摇杆的实际状态。



 自 1.2.8 版本起，队列为环形，可容纳 64 个事件。你可以通过调大 joystick.h 中的
 JS_BUFF_SIZE 并重新编译驱动来增加此大小。


在上面的代码中，你可能还想利用典型的 read(2) 功能一次性读取多个事件。为此，你
```

	struct js_event mybuffer[0xff];
	int i = read (fd, mybuffer, sizeof(mybuffer));

```
这种情况下，如果队列为空，read 将返回 -1，或者返回另一个值，其中读取到的事件
数量为 i / sizeof(js_event)。同样，如果缓冲区已满，最好处理这些事件并继续读取，
直到清空驱动队列。


## IOCTLs


```

				/* function			3rd arg  */
	#define JSIOCGAXES	/* get number of axes		char	 */
	#define JSIOCGBUTTONS	/* get number of buttons	char	 */
	#define JSIOCGVERSION	/* get driver version		int	 */
	#define JSIOCGNAME(len) /* get identifier string	char	 */
	#define JSIOCSCORR	/* set correction values	&js_corr */
	#define JSIOCGCORR	/* get correction values	&js_corr */

```
```

	char number_of_axes;
	ioctl (fd, JSIOCGAXES, &number_of_axes);


```
### JSIOGCVERSION


JSIOGCVERSION 是在运行时检查正在运行的驱动是否为 1.0+ 并支持事件接口的好方法。
如果不是，该 IOCTL 将失败。对于编译期决定，你可以测试
```

	#ifdef JS_VERSION
	#if JS_VERSION > 0xsomething


```
### JSIOCGNAME


JSIOCGNAME(len) 允许你获取摇杆的名称字符串——与启动时打印的相同。'len' 参数是
请求名称的应用程序所提供的缓冲区长度，用于避免
```

	char name[128];
	if (ioctl(fd, JSIOCGNAME(sizeof(name)), name) < 0)
		strscpy(name, "Unknown", sizeof(name));
	printf("Name: %s\n", name);


```
### JSIOC[SG]CORR


关于 JSIOC[SG]CORR 的用法，建议参考 jscal.c。正常程序中并不需要它们，仅在校准
软件（如 jscal 或 kcmjoy）中才需要。这些 IOCTL 及数据类型不被视为 API 的稳定部分，
因此可能在驱动后续版本中不经警告地发生变化。

JSIOCSCORR 和 JSIOCGCORR 都期望 &js_corr 能够容纳所有轴的信息。即 struct
js_corr corr[MAX_AXIS];

```

	struct js_corr {
		__s32 coef[8];
		__u16 prec;
		__u16 type;
	};

```
```

	#define JS_CORR_NONE            0x00    /* returns raw values */
	#define JS_CORR_BROKEN          0x01    /* broken line */


```
## Backward compatibility


0.x 版摇杆驱动 API 相当受限，其用法已被废弃。
```

	struct JS_DATA_TYPE js;
	while (1) {
		if (read (fd, &js, JS_RETURN) != JS_RETURN) {
			/* error */
		}
		usleep (1000);
	}

```
如示例所示，read 会立即返回，
```

	struct JS_DATA_TYPE {
		int buttons;    /* immediate button state */
		int x;          /* immediate x axis value */
		int y;          /* immediate y axis value */
	};

```
```

	#define JS_RETURN       sizeof(struct JS_DATA_TYPE)

```
要测试按键的状态，

```

	first_button_state  = js.buttons & 1;
	second_button_state = js.buttons & 2;

```
在原始的 0.x 驱动中，轴值没有定义的范围，只要求值为非负。1.2.8+ 版驱动使用固定
范围上报数值，1 为最小值，128 为居中，255 为最大值。

v0.8.0.2 版驱动还提供了一个“数字摇杆”（在本驱动中现称 Multisystem 摇杆）接口，
位于 /dev/djsX 下。本驱动不试图与该接口保持兼容。


## Final Notes


```

  ____/|	Comments, additions, and specially corrections are welcome.
  \ o.O|	Documentation valid for at least version 1.2.8 of the joystick
   =(_)=	driver and as usual, the ultimate source for documentation is
     U		to "Use The Source Luke" or, at your convenience, Vojtech ;)

```