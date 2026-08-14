## ASoC USB 支持


## 概述

为了利用 ALSA 中现有的 USB 声音设备支持，引入了 ASoC USB API，以允许各子系统交换配置信息。

一个潜在用例是支持 USB 音频卸载（USB audio offloading），这是一种实现，允许音频子系统中一条
替代的、经过功耗优化的路径来处理通过 USB 总线传输的音频数据。这将让主处理器能够在更长时间内保持
较低功耗模式。以下是 ASoC 与 ALSA 各部分如何连接在一起以实现此目的的示例设计：

```

               USB                   |            ASoC
                                     |  _________________________
                                     | |   ASoC Platform card    |
                                     | |_________________________|
                                     |         |           |
                                     |      ___V____   ____V____
                                     |     |ASoC BE | |ASoC FE  |
                                     |     |DAI LNK | |DAI LNK  |
                                     |     |________| |_________|
                                     |         ^  ^        ^
                                     |         |  |________|
                                     |      ___V____    |
                                     |     |SoC-USB |   |
     ________       ________               |        |   |
    |USB SND |<--->|USBSND  |<------------>|________|   |
    |(card.c)|     |offld   |<----------                |
    |________|     |________|___     | |                |
        ^               ^       |    | |    ____________V_________
        |               |       |    | |   |IPC                   |
     __ V_______________V_____  |    | |   |______________________|
    |USB SND (endpoint.c)     | |    | |              ^
    |_________________________| |    | |              |
                ^               |    | |   ___________V___________
                |               |    | |->|audio DSP              |
     ___________V_____________  |    |    |_______________________|
    |XHCI HCD                 |<-    |
    |_________________________|      |

```
## SoC USB 驱动

### 结构体

`struct snd_soc_usb`

  - `list`：SND SoC 结构体列表的链表头
  - `component`：对 ASoC 组件的引用
  - `connection_status_cb`：用于通知连接事件的回调
  - `update_offload_route_info`：用于获取所选 USB 声音卡/PCM 设备的回调
  - `priv_data`：驱动数据

snd_soc_usb 结构可以通过 ASoC 平台卡设备，或者一个 USB 设备（udev->dev）来引用。它由 ASoC BE DAI
链路创建，USB 声音实体将能够使用此结构向 ASoC BE DAI 链路传递信息。

`struct snd_soc_usb_device`

  - `card_idx`：与 USB 声音设备关联的声音卡索引
  - `chip_idx`：USB 声音芯片数组索引
  - `cpcm_idx`：与该 USB 声音设备关联的捕获 PCM 设备索引
  - `ppcm_idx`：与该 USB 声音设备关联的回放 PCM 设备索引
  - `num_playback`：回放流的数量
  - `num_capture`：捕获流的数量
  - `list`：USB 声音设备列表的链表头

struct snd_soc_usb_device 由 USB 声音卸载驱动创建。它将携带用于确定此 USB 音频设备可能卸载路径的
基本参数/限制。

### 函数


	int snd_soc_usb_find_supported_format(int card_idx,
			struct snd_pcm_hw_params *params, int direction)
..

  - `card_idx`：USB 声音芯片数组的索引。
  - `params`：来自 USB DPCM BE DAI 链路的请求 PCM 参数
  - `direction`：捕获或回放

**snd_soc_usb_find_supported_format()** 确保外部 DSP 所请求的音频配置文件受 USB 设备支持。

成功时返回 0，失败时返回 -EOPNOTSUPP。


	int snd_soc_usb_connect(struct device **usbdev, struct snd_soc_usb_device **sdev)
..

  - `usbdev`：被发现的 usb 设备
  - `sdev`：设备的能力

**snd_soc_usb_connect()** 将 USB 音频设备的探测通知给 ASoC USB DPCM BE DAI 链路。这可用于 BE DAI
驱动中，以跟踪可用的 USB 音频设备。它预期由驻留在 USB SND 中的 USB 卸载驱动调用。

成功时返回 0，失败时返回负的错误码。


	int snd_soc_usb_disconnect(struct device **usbdev, struct snd_soc_usb_device **sdev)
..

  - `usbdev`：被移除的 usb 设备
  - `sdev`：要释放的能力

**snd_soc_usb_disconnect()** 将 USB 音频设备的移除通知给 ASoC USB DPCM BE DAI 链路。它预期由驻留在
USB SND 中的 USB 卸载驱动调用。


	void **snd_soc_usb_find_priv_data(struct device **usbdev)
..

  - `usbdev`：用于查找私有数据所引用的 usb 设备

**snd_soc_usb_find_priv_data()** 获取保存到 SoC USB 设备的私有数据。

成功时返回指向 priv_data 的指针，失败时返回 NULL。


	int snd_soc_usb_setup_offload_jack(struct snd_soc_component *component,
					struct snd_soc_jack *jack)
..

  - `component`：要添加 jack 的 ASoC 组件
  - `jack`：要填充的 jack 组件

**snd_soc_usb_setup_offload_jack()** 是一个辅助函数，用于向平台声音卡添加一个声音 jack 控制。这将允许
支持 USB 音频卸载的设计使用一致的名称。此外，这将启用 jack 以通知变更。

成功时返回 0，否则返回负值。


	int snd_soc_usb_update_offload_route(struct device *dev, int card, int pcm,
					     int direction, enum snd_soc_usb_kctl path,
					     long *route)
..

  - `dev`：要查找卸载路径映射的 USB 设备
  - `card`：USB 声音卡索引
  - `pcm`：USB 声音 PCM 设备索引
  - `direction`：要获取卸载路由信息的方向
  - `path`：kcontrol 选择器 - pcm 设备或卡索引
  - `route`：卸载路径的声音卡和 pcm 索引映射。这是一个由两个整数组成的数组，按该特定顺序携带卡和
	       pcm 设备索引。它可用作 kcontrol 输出的数组。

**snd_soc_usb_update_offload_route()** 调用注册到 USB BE DAI 链路的回调，以获取关于为执行该设备的
USB 音频卸载而映射的 ASoC 设备的信息。`route` 可以是指向 kcontrol 值输出数组的指针，该数组在读取
kcontrol 时携带值。

成功时返回 0，否则返回负值。


	struct snd_soc_usb **snd_soc_usb_allocate_port(struct snd_soc_component **component,
			void *data);
..

  - `component`：DPCM BE DAI 链路组件
  - `data`：私有数据

**snd_soc_usb_allocate_port()** 分配一个 SoC USB 设备并填充用于后续操作的标准参数。

成功时返回指向 struct soc_usb 的指针，错误时返回负值。


	void snd_soc_usb_free_port(struct snd_soc_usb *usb);
..

  - `usb`：要释放的 SoC USB 设备

**snd_soc_usb_free_port()** 释放一个 SoC USB 设备。


	void snd_soc_usb_add_port(struct snd_soc_usb *usb);
..

  - `usb`：要添加的 SoC USB 设备

**snd_soc_usb_add_port()** 将一个已分配的 SoC USB 设备添加到 SoC USB 框架。一旦添加，该设备即可被
后续操作引用。


	void snd_soc_usb_remove_port(struct snd_soc_usb *usb);
..

  - `usb`：要移除的 SoC USB 设备

**snd_soc_usb_remove_port()** 从 SoC USB 框架中移除一个 SoC USB 设备。移除设备后，任何 SoC USB
操作都将无法引用被移除的设备。

### 如何注册到 SoC USB

ASoC DPCM USB BE DAI 链路是负责在组件绑定时分配和注册 SoC USB 实体的组件。同样，它也负责释放所
分配的资源。示例如下：


	static int q6usb_component_probe(struct snd_soc_component *component)
	{
		...
		data->usb = snd_soc_usb_allocate_port(component, 1, &data->priv);
		if (!data->usb)
			return -ENOMEM;

		usb->connection_status_cb = q6usb_alsa_connection_cb;

		ret = snd_soc_usb_add_port(usb);
		if (ret < 0) {
			dev_err(component->dev, "failed to add usb port\n");
			goto free_usb;
		}
		...
	}

	static void q6usb_component_remove(struct snd_soc_component *component)
	{
		...
		snd_soc_usb_remove_port(data->usb);
		snd_soc_usb_free_port(data->usb);
	}

	static const struct snd_soc_component_driver q6usb_dai_component = {
		.probe = q6usb_component_probe,
		.remove = q6usb_component_remove,
		.name = "q6usb-dai-component",
		...
	};
..

BE DAI 链路可以将供应商特定的信息作为分配 SoC USB 设备调用的一部分传递。这将允许驻留在 USB SND 中
的 USB 卸载驱动访问任何 BE DAI 链路参数或设置。

### USB 音频设备连接流程

USB 设备可以随时热插拔到 USB 端口。BE DAI 链路应当知晓物理 USB 端口的当前状态，即是否连接了任何
带有音频接口的 USB 设备。connection_status_cb() 可用于将任何变更通知给 BE DAI 链路。

每当发生 USB SND 接口绑定或移除事件时，都会使用 snd_soc_usb_connect() 或 snd_soc_usb_disconnect()
调用它：


	static void qc_usb_audio_offload_probe(struct snd_usb_audio *chip)
	{
		...
		snd_soc_usb_connect(usb_get_usb_backend(udev), sdev);
		...
	}

	static void qc_usb_audio_offload_disconnect(struct snd_usb_audio *chip)
	{
		...
		snd_soc_usb_disconnect(usb_get_usb_backend(chip->dev), dev->sdev);
		...
	}
..

为了应对驱动或设备存在无法保证的情况，USB SND 暴露了 snd_usb_rediscover_devices() 以重新发送任何
已识别 USB 音频接口的连接事件。考虑以下情形：

	**usb_audio_probe()**
	  | --> USB 音频流被分配并保存到 usb_chip[]
	  | --> 将连接事件传播给 USB SND 中的 USB 卸载驱动
	  | --> **snd_soc_usb_connect()** 因 USB BE DAI 链路未就绪而退出

	BE DAI 链路组件探测
	  | --> DAI 链路被探测，SoC USB 端口被分配
	  | --> USB 音频设备连接事件被错过

为确保连接事件不被错过，当 SoC USB 设备被注册时执行 **snd_usb_rediscover_devices()**。现在，当
BE DAI 链路组件探测发生时，以下突出了该序列：

	BE DAI 链路组件探测
	  | --> DAI 链路被探测，SoC USB 端口被分配
	  | --> SoC USB 设备已添加，并且 **snd_usb_rediscover_devices()** 运行

	**snd_usb_rediscover_devices()**
	  | --> 遍历 usb_chip[]，并对非 NULL 项发出
	  |     **connection_status_cb()**

在 USB 卸载驱动被解绑而 USB SND 就绪的情况下，**snd_usb_rediscover_devices()** 在模块初始化期间被调用。
这使得卸载路径也能通过以下流程被启用：

	**usb_audio_probe()**
	  | --> USB 音频流被分配并保存到 usb_chip[]
	  | --> 将连接事件传播给 USB SND 中的 USB 卸载驱动
	  | --> USB 卸载驱动**未**就绪！

	BE DAI 链路组件探测
	  | --> DAI 链路被探测，SoC USB 端口被分配
	  | --> 因缺少 USB 卸载驱动，没有 USB 连接事件

	USB 卸载驱动探测
	  | --> **qc_usb_audio_offload_init()**
	  | --> 调用 **snd_usb_rediscover_devices()** 以通知设备

## USB 卸载相关的 Kcontrols

### 细节

一组 kcontrol 可供应用程序使用，以帮助选择正确的声音设备来启用 USB 音频卸载。SoC USB 暴露了
get_offload_dev() 回调，设计可利用它来确保将正确的索引返回给应用程序。

### 实现


**示例：**

  **声音卡**：

```

	  0 [SM8250MTPWCD938]: sm8250 - SM8250-MTP-WCD9380-WSA8810-VA-D
						SM8250-MTP-WCD9380-WSA8810-VA-DMIC
	  1 [Seri           ]: USB-Audio - Plantronics Blackwire 3225 Seri
						Plantronics Plantronics Blackwire
						3225 Seri at usb-xhci-hcd.1.auto-1.1,
						full sp
	  2 [C320M          ]: USB-Audio - Plantronics C320-M
                      Plantronics Plantronics C320-M at usb-xhci-hcd.1.auto-1.2, full speed

  **PCM 设备**：

	::

	  card 0: SM8250MTPWCD938 [SM8250-MTP-WCD9380-WSA8810-VA-D], device 0: MultiMedia1 (*) []
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 0: SM8250MTPWCD938 [SM8250-MTP-WCD9380-WSA8810-VA-D], device 1: MultiMedia2 (*) []
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 1: Seri [Plantronics Blackwire 3225 Seri], device 0: USB Audio [USB Audio]
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 2: C320M [Plantronics C320-M], device 0: USB Audio [USB Audio]
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0

  **USB 声音卡** - card#1：

	::

	  USB Offload Playback Card Route PCM#0   -1 (range -1->32)
	  USB Offload Playback PCM Route PCM#0    -1 (range -1->255)

  **USB 声音卡** - card#2：

	::

	  USB Offload Playback Card Route PCM#0   0 (range -1->32)
	  USB Offload Playback PCM Route PCM#0    1 (range -1->255)

```
上述示例展示了一个系统拥有一个 ASoC 平台卡（card#0）并连接了两个 USB 声音设备（card#1 和 card#2）
的场景。当读取每个 USB 音频设备的可用 kcontrol 时，以下 kcontrol 列出了该特定 USB 设备映射的卸载
卡和 pcm 设备索引：

	`USB Offload Playback Card Route PCM#*`

	`USB Offload Playback PCM Route PCM#*`

该 kcontrol 是带索引的，因为一个 USB 音频设备可能潜在地拥有多个 PCM 设备。上述 kcontrol 定义为：

  - `USB Offload Playback Card Route PCM#` **(R)**：返回映射卸载路径的 ASoC 平台声音卡索引。输出
    **"0"**（卡索引）表示通过 card#0 存在该 USB SND 设备可用的卸载路径。如果看到 **"-1"**，则
    该 USB SND 设备没有可用的卸载路径。该 kcontrol 对系统中存在的每个 USB 音频设备都存在，预期
    根据该 kcontrol 的输出值以及 PCM 路由 kcontrol 来推导卸载的当前状态。

  - `USB Offload Playback PCM Route PCM#` **(R)**：返回映射卸载路径的 ASoC 平台声音卡 PCM 设备索引。
    输出 **"1"**（PCM 设备索引）表示通过 PCM device#0 存在该 USB SND 设备可用的卸载路径。如果看到
    **"-1"**，则该 USB SND 设备没有可用的卸载路径。该 kcontrol 对系统中存在的每个 USB 音频设备都存在，
    预期根据该 kcontrol 的输出值以及卡路由 kcontrol 来推导卸载的当前状态。

### USB 卸载回放路由 Kcontrol

为了允许在音频卸载设备选择上有供应商特定的实现，SoC USB 层暴露了以下内容：


	int (**update_offload_route_info)(struct snd_soc_component **component,
					 int card, int pcm, int direction,
					 enum snd_soc_usb_kctl path,
					 long *route)
..

这些特定于 **USB Offload Playback Card Route PCM#** 和 **USB Offload PCM Route PCM#** kcontrol。

当用户对 kcontrol 发出 get 调用时，注册的 SoC USB 回调将执行注册到 DPCM BE DAI 链路的函数调用。

**回调注册：**


	static int q6usb_component_probe(struct snd_soc_component *component)
	{
	...
	usb = snd_soc_usb_allocate_port(component, 1, &data->priv);
	if (IS_ERR(usb))
		return -ENOMEM;

	usb->connection_status_cb = q6usb_alsa_connection_cb;
	usb->update_offload_route_info = q6usb_get_offload_dev;

	ret = snd_soc_usb_add_port(usb);
..

### 现有 USB 声音 Kcontrol

随着 USB 卸载支持的引入，上述 USB 卸载 kcontrol 将被添加到由 USB 声音框架识别的已有 kcontrol 列表中。
这些 kcontrol 仍然是用于修改与 USB 音频设备相关特性的主控件。

```

	  Number of controls: 9
	  ctl     type    num     name                                    value
	  0       INT     2       Capture Channel Map                     0, 0 (range 0->36)
	  1       INT     2       Playback Channel Map                    0, 0 (range 0->36)
	  2       BOOL    1       Headset Capture Switch                  On
	  3       INT     1       Headset Capture Volume                  10 (range 0->13)
	  4       BOOL    1       Sidetone Playback Switch                On
	  5       INT     1       Sidetone Playback Volume                4096 (range 0->8192)
	  6       BOOL    1       Headset Playback Switch                 On
	  7       INT     2       Headset Playback Volume                 20, 20 (range 0->24)
	  8       INT     1       USB Offload Playback Card Route PCM#0   0 (range -1->32)
	  9       INT     1       USB Offload Playback PCM Route PCM#0    1 (range -1->255)

```
由于 USB 音频设备控制是通过 USB 控制端点处理的，请使用 USB mixer 中现有的机制来设置音量等参数。
