## Linux 力反馈（Force feedback）


:Author: Johann Deneux <johann.deneux@gmail.com>，2001/04/22。
:Updated: Anssi Hannula <anssi.hannula@gmail.com>，2006/04/09。

你可以重新分发本文件。请记得同时包含 shape.svg 和 interactive.svg。

#### 简介（Introduction）


本文档描述如何在 Linux 下使用力反馈设备。目标不是像对待简单的仅输入设备那样支持这些设备（原本已经如此），而是真正启用力效果（force effects）的渲染。
本文档仅描述 Linux 输入接口的力反馈部分。在进一步阅读本文档之前，请先阅读 joydev/joystick.rst 和 input.rst。

#### 给用户的使用说明（Instructions to the user）


要启用力反馈，你必须：

1. 将内核配置为包含 evdev 以及支持你设备的驱动。
2. 确保已加载 evdev 模块，并且已创建 /dev/input/event* 设备文件。

在开始之前，先提醒你：某些设备在初始化阶段会剧烈震动。例如我的 "AVB Top Shot Pegasus" 就会出现这种情况。要停止这种恼人行为，将你的摇杆移到极限位置。无论如何，你都应当用手扶住设备，以便在出现问题时避免其损坏。

如果你有一个串行的 iforce 设备，你需要启动 inputattach。详见 joydev/joystick.rst。

### 它工作吗？（Does it work ?）


```
    % fftest /dev/input/eventXX
```

#### 给开发者的使用说明（Instructions to the developer）


所有交互都使用 event API 完成。也就是说，你可以在 /dev/input/eventXX 上使用 ioctl() 和 write()。
此信息可能会发生变化。

### 查询设备能力（Querying device capabilities）


```
    #include <linux/input.h>
    #include <sys/ioctl.h>

    #define BITS_TO_LONGS(x) \
	    (((x) + 8 * sizeof (unsigned long) - 1) / (8 * sizeof (unsigned long)))
    unsigned long features[BITS_TO_LONGS(FF_CNT)];
    int ioctl(int file_descriptor, int request, unsigned long *features);

```
"request" 必须为 EVIOCGBIT(EV_FF, features 数组的字节大小)

返回设备支持的特性。features 是一个位域，包含以下位：

- FF_CONSTANT	可渲染恒定力效果
- FF_PERIODIC	可渲染具有以下波形的周期效果：

  - FF_SQUARE	  方波波形
  - FF_TRIANGLE	  三角波波形
  - FF_SINE	  正弦波波形
  - FF_SAW_UP	  上升锯齿波波形
  - FF_SAW_DOWN	  下降锯齿波波形
  - FF_CUSTOM	  自定义波形

- FF_RAMP       可渲染斜坡效果
- FF_SPRING	可模拟弹簧的存在
- FF_FRICTION	可模拟摩擦力
- FF_DAMPER	可模拟阻尼效果
- FF_RUMBLE	震动（rumble）效果
- FF_INERTIA    可模拟惯性
- FF_GAIN	增益可调
- FF_AUTOCENTER	自动居中可调


    - 在大多数情况下，你应当使用 FF_PERIODIC 而非 FF_RUMBLE。所有支持 FF_RUMBLE 的设备都支持 FF_PERIODIC（方波、三角波、正弦波），反之亦然。

    - 目前 FF_CUSTOM 的确切语法尚未定义，因为还没有驱动支持它。

```
    int ioctl(int fd, EVIOCGEFFECTS, int *n);
```
返回设备内存中可保存的效果数量。

### 将效果上传到设备（Uploading effects to the device）


```
    #include <linux/input.h>
    #include <sys/ioctl.h>

    int ioctl(int file_descriptor, int request, struct ff_effect *effect);
```
"request" 必须为 EVIOCSFF。

"effect" 指向一个描述待上传效果的结构体。该效果被上传，但不会播放。
effect 的内容可能被修改。特别是，其 "id" 字段会被设为驱动分配的唯一 id。在执行某些操作（移除效果、控制播放）时需要此数据。
用户必须将 "id" 字段设为 -1，以告知驱动分配一个新效果。

效果是与文件描述符相关的。

关于 ff_effect 结构体的说明，请参见 <uapi/linux/input.h>。你也可以从 shape.svg 和 interactive.svg 这两个文件包含的示意图中获得帮助：


    Shape


    Interactive


### 从设备中移除效果（Removing an effect from the device）


```
    int ioctl(int fd, EVIOCRMFF, effect.id);
```
这为设备内存中的新效果腾出空间。注意，如果该效果正在播放，这也会停止它。

### 控制效果的播放（Controlling the playback of effects）


播放控制通过 write() 完成。下面是一个示例：

```
    #include <linux/input.h>
    #include <unistd.h>

	struct input_event play;
	struct input_event stop;
	struct ff_effect effect;
	int fd;
   ...
	fd = open("/dev/input/eventXX", O_RDWR);
   ...
	/* 播放三次 */
	play.type = EV_FF;
	play.code = effect.id;
	play.value = 3;

	write(fd, (const void*) &play, sizeof(play));
   ...
	/* 停止一个效果 */
	stop.type = EV_FF;
	stop.code = effect.id;
	stop.value = 0;

	write(fd, (const void*) &stop, sizeof(stop));

```
### 设置增益（Setting the gain）


并非所有设备的力度都相同。因此，用户应根据希望效果的强度来设置一个增益因子。该设置在多次访问驱动期间保持有效。

```
    /* 设置设备的增益
    int gain;		/* 介于 0 到 100 之间 */
    struct input_event ie;	/* 用于与驱动通信的结构体 */

    ie.type = EV_FF;
    ie.code = FF_GAIN;
    ie.value = 0xFFFFUL * gain / 100;

    if (write(fd, &ie, sizeof(ie)) == -1)
	perror("set gain");

```
### 启用/禁用自动居中（Enabling/Disabling autocenter）


在我看来，自动居中特性相当干扰效果的渲染，我认为它应当是一种效果，其计算取决于游戏类型。但如果你愿意，可以启用它。

```
    int autocenter;		/* 介于 0 到 100 之间 */
    struct input_event ie;

    ie.type = EV_FF;
    ie.code = FF_AUTOCENTER;
    ie.value = 0xFFFFUL * autocenter / 100;

    if (write(fd, &ie, sizeof(ie)) == -1)
	perror("set auto-center");

```
值为 0 表示“无自动居中”。

### 动态更新效果（Dynamic update of an effect）


过程与上传新效果相同，只是不将 id 字段设为 -1，而是将其设为所需的效果 id。
通常，效果不会停止并重新启动。然而，取决于设备类型，并非所有参数都能动态更新。例如，对于 iforce 设备，效果的方向无法更新。在这种情况下，驱动会停止该效果、重新上传并重新启动它。

因此，建议仅在以重放次数为 1 重启效果可接受的情况下，在效果播放时动态更改其方向。

### 关于效果状态的信息（Information about the status of effects）


每次效果的状态发生变化时，都会发送一个事件。其值
```
    struct input_event {
    /* 当效果的状态发生变化时 */
	    struct timeval time;

    /* 设为 EV_FF_STATUS */
	    unsigned short type;

    /* 包含效果的 id */
	    unsigned short code;

    /* 指示状态 */
	    unsigned int value;
    };

    FF_STATUS_STOPPED	效果已停止播放
    FF_STATUS_PLAYING	效果已开始播放

```

    - 状态反馈仅由 iforce 驱动支持。如果你确有充分理由使用它，请联系
      linux-joystick@atrey.karlin.mff.cuni.cz 或 anssi.hannula@gmail.com，
      以便为其余驱动添加对此的支持。
