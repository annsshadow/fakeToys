## ALSA PCM 通道映射 API


Takashi Iwai <tiwai@suse.de>

## 概述


通道映射 API 允许用户查询可能的通道映射和当前通道映射，还可以选择性地修改当前流的通道映射
通道映射是每PCM 通道位置的一个数组。通常，立体声 PCM 流的通道映射`{ front_left, front_right }`
4.0 环绕 PCM 流的通道映射`{ front left, front right, rear left, rear right }.`

到目前为止，问题在于我们没有显式的标准通道映射，应用程序无法知道哪个通道对应哪个（扬声器）位置。因此，应用程序5.1 输出应用了错误的通道，你会突然从后方听到奇怪的声音。或者，某些设备私下假设 center/LFE 是第第四通道，而其他设备则假设 C/LFE 是第第六通道
此外，某些设备（HDMI）即使在相同的总通道数下也可以配置为不同的扬声器位置。然而，由于缺乏通道映射规范，此前没有办法指定这一点。这些都是新通道映射 API 的主要动机
## 设计


实际上，从内用户空间 ABI 的角度来看，“通道映射 API”并没有引入任何新东西。它仅使用了现有的控制元素特性
在基本设计上，每PCM 子流可以包含一个提供通道映射信息和配置的控制元素。该元素由以下指定：

- iface = SNDRV_CTL_ELEM_IFACE_PCM
- name = "Playback Channel Map" 鎴?"Capture Channel Map"
- device = 所分配 PCM 子流的相同设备号
- index = 所分配 PCM 子流的相同索引号

注意名称取决PCM 子流的方向而不同
每个控制元素至少提供 TLV 读操作和读操作。可选地，可以提供写操作以允许用户动态更改通道映射
### TLV


TLV 操作给出可用通道映射的列表。通道映射的列表项通常`type data-bytes ch0 ch1 ch2...`
其中 type TLV 类型值，第二个参数是通道值的总字节数（不是数量），其余是每个通道的位置值
作为 TLV 类型，可以使`SNDRV_CTL_TLVT_CHMAP_FIXED`、`SNDRV_CTL_TLVT_CHMAP_VAR` `SNDRV_CTL_TLVT_CHMAP_PAIRED`。`_FIXED` 类型用于通道位置固定的通道映射，而后两者用于灵活的通道位置。`_VAR` 类型用于所有通道可自由交换的通道映射，`_PAIRED` 类型用于成对通道可交换的通道映射。例如，当你{FL/FR/RL/RR} 通道映射时，`_PAIRED` 类型只允许你交换 {RL/RR/FL/FR}，`_VAR` 类型甚至允许交换 FL RR
这些新的 TLV 类型定义`sound/tlv.h` 中
可用的通道位置值定义在 `sound/asound.h` 中，以下是节选：

```

  /* channel positions */
  enum {
	SNDRV_CHMAP_UNKNOWN = 0,
	SNDRV_CHMAP_NA,		/* N/A, silent */
	SNDRV_CHMAP_MONO,	/* mono stream */
	/* this follows the alsa-lib mixer channel value + 3 */
	SNDRV_CHMAP_FL,		/* front left */
	SNDRV_CHMAP_FR,		/* front right */
	SNDRV_CHMAP_RL,		/* rear left */
	SNDRV_CHMAP_RR,		/* rear right */
	SNDRV_CHMAP_FC,		/* front center */
	SNDRV_CHMAP_LFE,	/* LFE */
	SNDRV_CHMAP_SL,		/* side left */
	SNDRV_CHMAP_SR,		/* side right */
	SNDRV_CHMAP_RC,		/* rear center */
	/* new definitions */
	SNDRV_CHMAP_FLC,	/* front left center */
	SNDRV_CHMAP_FRC,	/* front right center */
	SNDRV_CHMAP_RLC,	/* rear left center */
	SNDRV_CHMAP_RRC,	/* rear right center */
	SNDRV_CHMAP_FLW,	/* front left wide */
	SNDRV_CHMAP_FRW,	/* front right wide */
	SNDRV_CHMAP_FLH,	/* front left high */
	SNDRV_CHMAP_FCH,	/* front center high */
	SNDRV_CHMAP_FRH,	/* front right high */
	SNDRV_CHMAP_TC,		/* top center */
	SNDRV_CHMAP_TFL,	/* top front left */
	SNDRV_CHMAP_TFR,	/* top front right */
	SNDRV_CHMAP_TFC,	/* top front center */
	SNDRV_CHMAP_TRL,	/* top rear left */
	SNDRV_CHMAP_TRR,	/* top rear right */
	SNDRV_CHMAP_TRC,	/* top rear center */
	SNDRV_CHMAP_LAST = SNDRV_CHMAP_TRC,
  };

```
当一PCM 流可以提供多个通道映射时，你可以在一TLV 容器类型中提供多个通道映射。要返回TLV 数据将包含如下内容：
```

	SNDRV_CTL_TLVT_CONTAINER 96
	    SNDRV_CTL_TLVT_CHMAP_FIXED 4 SNDRV_CHMAP_FC
	    SNDRV_CTL_TLVT_CHMAP_FIXED 8 SNDRV_CHMAP_FL SNDRV_CHMAP_FR
	    SNDRV_CTL_TLVT_CHMAP_FIXED 16 NDRV_CHMAP_FL SNDRV_CHMAP_FR \
		SNDRV_CHMAP_RL SNDRV_CHMAP_RR

```
通道位置在最16 位（LSB）中提供。高位用于位标志```

	#define SNDRV_CHMAP_POSITION_MASK	0xffff
	#define SNDRV_CHMAP_PHASE_INVERSE	(0x01 << 16)
	#define SNDRV_CHMAP_DRIVER_SPEC		(0x02 << 16)

```
`SNDRV_CHMAP_PHASE_INVERSE` 表示该通道相位反转（因此将左右通道相加会导致几乎静音）。某些数字麦克风设备具有此特性
当设置了 `SNDRV_CHMAP_DRIVER_SPEC` 时，所有通道位置值不遵循上述标准定义，而是驱动特定的
### 读操

控制读操作用于提供给定流的当前通道映射。控制元素返回一个包含每个通道位置的整数数组
如果在指定通道数之前（即设hw_params 之前）执行此操作，它应返回所有通道都设置为 `UNKNOWN`
### 写操

控制写操作是可选的，仅适用于可以动态更改通道配置的设备，例如 HDMI。用户需要传递一个整数值，其中包含所分配 PCM 子流所有通道的有效通道位置
此操作仅PCM PREPARED 状态下允许。在其他状态下调用时，应返回错误