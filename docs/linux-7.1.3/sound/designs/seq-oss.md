## ALSA 上的 OSS 音序器模

Copyright (c) 1998,1999 by Takashi Iwai

ver.0.1.8; 1999骞?1鏈?6鏃。
## 描述


本目录包ALSA 上的 OSS 音序器模拟驱动。注意，本程序仍处于开发状态
它的作用——提OSS 音序器的模拟，通过 `/dev/sequencer` `/dev/music` 设备访问。只要准备好相应ALSA 音序器，大多数使OSS 的应用程序都能运行
本驱动模拟了以下特性：

- 普通音序器MIDI 事件
    它们被转换为 ALSA 音序器事件，并发送到相应的端口
- 定时器事件：

    定时器不能通过 ioctl 选择。控制速率固定100，与 HZ 无关。也就是说，即使Alpha 系统上，一tick 也始终是 1/100 秒。基准速率和速度（tempo）可以在 `/dev/music` 中更改
- 音色（patch）加载：

    由于音色加载是通过回调到合成器驱动来实现的，是否支持它完全取决于合成器驱动
- I/O 控制
    大多数控制都被接受。有些控制依赖于合成器驱动，就像在原始的 OSS 中也一样
此外，你还能发现以下高级特性：

- 更好的队列机制：

    事件在处理之前会被排入队列
- 多应用程序：

    你可以同时运行两个或更多应用程序（即便是 OSS 音序器）    不过，每MIDI 设备是独占的——也就是说，如果某个 MIDI 设备已被某应用程序打开一次，其他应用程序就无法再使用它。合成器设备没有这种限制
- 实时事件处理
    事件可以在不使用越界 ioctl 的情况下实时处理。要切换到实时模式，发ABSTIME 0 事件。随后的事件将在实时下处理而不入队。要关闭实时模式，发RELTIME 0 事件
- `/proc` 接口
    应用程序和设备的状态可以随时通过 `/proc/asound/seq/oss` 查看。在后续版本中，配置也将通过 `/proc` 接口更改
## 安装


运行 configure 脚本时同时带上音序器支持（`--with-sequencer=yes`）和 OSS 模拟（`--with-oss=yes`）选项。将创建一`snd-seq-oss.o` 模块。如果你的声卡的合成器模块支OSS 模拟（到目前为止只有 Emu8000 驱动），该模块会被自动加载否则，你需要手动加载该模块
一开始，本模块会探测所有已经连接到音序器的 MIDI 端口。此后，端口的创建和删除ALSA 音序器的通告机制监视
可用的合成器MIDI 设备可以proc 接口中找到。运`cat /proc/asound/seq/oss`，检查设备。例如，如果你使AWE64 声卡，你会看到如下内容：
```

    OSS sequencer emulation version 0.1.8
    ALSA client number 63
    ALSA receiver port 0

    Number of applications: 0

    Number of synth devices: 1
    synth 0: [EMU8000]
      type 0x1 : subtype 0x20 : voices 32
      capabilities : ioctl enabled / load_patch enabled

    Number of MIDI devices: 3
    midi 0: [Emu8000 Port-0] ALSA port 65:0
      capability write / opened none

    midi 1: [Emu8000 Port-1] ALSA port 65:1
      capability write / opened none

    midi 2: [0: MPU-401 (UART)] ALSA port 64:0
      capability read/write / opened none

```
注意，设备编号可能不同于 `/proc/asound/oss-devices` 的信息或原始 OSS 驱动的信息请使`/proc/asound/seq/oss` 中列出的设备编号来通过 OSS 音序器模拟播放
## 使用合成器设

运行你喜欢的程序。我测试playmidi-2.4、awemidi-0.4.3、gmod-3.1 xmp-1.1.5。你也可以像 sfxload 那样通过 `/dev/sequencer` 加载样本
如果底层驱动支持对合成器设备的多路访问（Emu8000 驱动），则允许两个或更多应用程序同时运行
## 使用 MIDI 设备


到目前为止，只测试了 MIDI 输出。MIDI 输入完全没有检查过，但有希望可以工作。请使用 `/proc/asound/seq/oss` 中列出的设备编号注意，这些编号大多不同于 `/proc/asound/oss-devices` 中的列表
## 模块选项


可使用以下模块选项
maxqlen
  指定最大读/写队列长度。该队列OSS 音序器私有，因此独立ALSA 音序器的队列长度。默认值为 1024
seq_oss_debug
  指定调试级别，接受零无调试消息）或正整数。默认值为 0
## 队列机制


OSS 音序器模拟使用一ALSA 优先队列来自 `/dev/sequencer` 的事件被处理，并放入由模块选项指定的队列中
来自 `/dev/sequencer` 的所有事件在开头就被解析。定时事件也在此时解析，因此事件可以实时处理。发ABSTIME 0 事件将操作模式切换到实时模式，发RELTIME 0 事件将其关闭在实时模式下，所有事件都立即分发
排队的事件由 ALSA 音序器分发器在预定时间之后分发到相应ALSA 音序器端口
如果写队列已满，在阻塞模式下应用程序会休眠，直到空出一定量（默认是一半）。对写入定时的同步也实现了
来自 MIDI 设备的输入或回显事件被存储在FIFO 队列中。如果应用程序以阻塞模式读取 `/dev/sequencer`，该进程将被唤醒
## 与合成器设备的接

### 注册


要注册一OSS 合成器设备，使用 snd_seq_oss_synth_register() 函数```

  int snd_seq_oss_synth_register(char *name, int type, int subtype, int nvoices,
          snd_seq_oss_callback_t *oper, void *private_data)

```
参数 `name`、`type`、`subtype` `nvoices` 用于构造供 ioctl 使用的相synth_info 结构体。返回值是该设备的索引号。必须记住这个索引以便注销。如果注册失败，将返-errno
要释放该设备，调snd_seq_oss_synth_unregister() 函数```

  int snd_seq_oss_synth_unregister(int index)

```
其中 `index` 是注册函数返回的索引号
### 回调


OSS 合成器设备具备样本下载和 ioctl（如样本重置）等能力。在 OSS 模拟中，这些特殊特性通过回调实现。注册参oper 用于指定这些回调。必须定义以下回调函数：
```

  snd_seq_oss_callback_t:
   int (*open)(snd_seq_oss_arg_t *p, void *closure);
   int (*close)(snd_seq_oss_arg_t *p);
   int (*ioctl)(snd_seq_oss_arg_t *p, unsigned int cmd, unsigned long arg);
   int (*load_patch)(snd_seq_oss_arg_t *p, int format, const char *buf, int offs, int count);
   int (*reset)(snd_seq_oss_arg_t *p);

```
除了 `open` `close` 回调外，其余允许NULL
每个回调函数都以 `snd_seq_oss_arg_t` 类型的参数作为第一个参数```

  struct snd_seq_oss_arg_t {
      int app_index;
      int file_mode;
      int seq_mode;
      snd_seq_addr_t addr;
      void *private_data;
      int event_passing;
  };

```
前三个字`app_index`、`file_mode` `seq_mode` OSS 音序器初始化。`app_index` 是应用程序索引，对每个打开 OSS 音序器的应用程序都是唯一的。`file_mode` 是指示文件操作模式的位标志。其含义`seq_oss.h`。`seq_mode` 是音序器操作模式。在当前版本中，只使`SND_OSSSEQ_MODE_SYNTH`
接下来的两个字段 `addr` `private_data` 必须由合成器驱动open 回调中填写。`addr` 包含分配给该设备ALSA 音序器端口地址。如果驱动为 `private_data` 分配了内存，则必须在 close 回调中自行释放
最后一个字`event_passing` 指示如何翻译 note-on/off 事件。在 `PROCESS_EVENTS` 模式下，音符 255 被视为力度变化，按键压力事件被传递到端口。在 `PASS_EVENTS` 模式下，所note on/off 事件都原样传递到端口而不加修改。`PROCESS_KEYPRESS` 模式检查大128 的音符，并将其视为按键压力事件（主要用于 Emu8000 驱动）
### Open 回调


每当有应用程序通过 OSS 音序器打开该设备时，就会调`open`。它不能NULL。通常，open 回调执行以下过程
#. 分配私有数据记录#. 创建一ALSA 音序器端口#. `arg->addr` 上设置新端口地址#. `arg->private_data` 上设置私有数据记录指针
注意，该合成器端口的 port_info 中的类型位标志不得包`TYPE_MIDI_GENERIC` 位。相反，应当使用 `TYPE_SPECIFIC`。同样，也不应包`CAP_SUBSCRIPTION` 位。这是为了把它与其他普MIDI 设备区分开。如open 过程成功，返回零；否则返-errno
### Ioctl 回调


当音序器收到设备特定ioctl 时，会调`ioctl` 回调。该回调应当处理以下两个 ioctl
IOCTL_SEQ_RESET_SAMPLES
    重置内存中的所有样本——返0

IOCTL_SYNTH_MEMAVL
    返回可用内存大小

FM_4OP_ENABLE
    通常可以忽略

其他 ioctl 在音序器内部处理，不会传递给底层驱动
### Load_Patch 回调


`load_patch` 回调用于样本下载。该回调必须读取用户空间的数据并传输到各个设备。成功返0，失败返-errno。format 参数patch_info 记录中的 patch 键。buf 是存patch_info 记录的用户空间指针。offs 可以忽略。count 是该样本数据的总大小
### Close 回调


当应用程序关闭该设备时，会调`close` 回调。如果在 open 回调中分配了任何私有数据，必须在 close 回调中释放。ALSA 端口的删除也应当在此完成。该回调不能NULL
### Reset 回调


当音序器设备被应用程序重置或关闭时，会调`reset` 回调。该回调应当立刻关闭相关端口上的声音，并初始化端口的状态。如果该回调未定义，OSS seq 会向该端口发送一`HEARTBEAT` 事件
## 事件


大多数事件由音序器处理，并转换为适当ALSA 音序器事件，以便每个合成器设备能通过 ALSA 音序器端口的 input_event 回调接收。驱动应当实现以ALSA 事件
=============	===================
ALSA 事件	原始 OSS 事件
=============	===================
NOTEON		SEQ_NOTEON, MIDI_NOTEON
NOTE		SEQ_NOTEOFF, MIDI_NOTEOFF
KEYPRESS	MIDI_KEY_PRESSURE
CHANPRESS	SEQ_AFTERTOUCH, MIDI_CHN_PRESSURE
PGMCHANGE	SEQ_PGMCHANGE, MIDI_PGM_CHANGE
PITCHBEND	SEQ_CONTROLLER(CTRL_PITCH_BENDER),
		MIDI_PITCH_BEND
CONTROLLER	MIDI_CTL_CHANGE,
		SEQ_BALANCE (with CTL_PAN)
CONTROL14	SEQ_CONTROLLER
REGPARAM	SEQ_CONTROLLER(CTRL_PITCH_BENDER_RANGE)
SYSEX		SEQ_SYSEX
=============	===================

这些行为大多可由 Emu8000 底层驱动中附带的 MIDI 模拟驱动实现。在未来的版本中，本模块将独立出来
一OSS 事件（`SEQ_PRIVATE` `SEQ_VOLUME` 事件）作为事件类SND_SEQ_OSS_PRIVATE 传递。OSS 音序器原样传递这些事件的 8 字节数据包，不作任何修改。底层驱动应当恰当处理这些事件
## MIDI 设备的接

由于 OSS 模拟会通过接收来自 ALSA 音序器的通告，自动探ALSA MIDI 音序器端口的创建和删除，因此 MIDI 设备无需像合成器设备那样显式注册不过，注册到 ALSA 音序器的 MIDI port_info 必须包含一个组`SND_SEQ_GROUP_DEVICE` 和一个能力位 `CAP_READ` `CAP_WRITE`。同时，订阅能力 `CAP_SUBS_READ` `CAP_SUBS_WRITE` 也必须定义。如果不满足这些条件，该端口不会作为 OSS 音序MIDI 设备注册
经由 MIDI 设备的事件在 OSS 音序器中被解析，并转换为相应ALSA 音序器事件。来MIDI 音序器的输入也被 OSS 音序器转换为 MIDI 字节事件。它的工作方式与 seq_midi 模块正好相反
## 已知问题 / TODO


- 通过 ALSA instrument 层的音色加载尚未实现