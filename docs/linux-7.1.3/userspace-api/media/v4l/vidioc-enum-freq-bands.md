
######## ioctl VIDIOC_ENUM_FREQ_BANDS


## 名称


VIDIOC_ENUM_FREQ_BANDS - 枚举支持的频段

## 语法


`int ioctl(int fd, VIDIOC_ENUM_FREQ_BANDS, struct v4l2_frequency_band *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_frequency_band` 的指针。

## 描述


枚举调谐器或调制器支持的频段。为此，应用程序初始化 struct `v4l2_frequency_band` 的
`tuner`、`type` 与 `index` 字段，并将 `reserved` 数组清零，然后以指向该结构的指针调用
VIDIOC_ENUM_FREQ_BANDS ioctl。

如果相应调谐器/调制器的 `V4L2_TUNER_CAP_FREQ_BANDS` 能力被设置，则该 ioctl 受支持。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2 1 1

    - - __u32
      - `tuner`
      - 调谐器或调制器索引号。该值与 struct `v4l2_input` 的 `tuner` 字段、struct
	`v4l2_tuner` 的 `index` 字段、struct `v4l2_output` 的 `modulator` 字段以及
	struct `v4l2_modulator` 的 `index` 字段相同。
    - - __u32
      - `type`
      - 调谐器类型。该值与 struct `v4l2_tuner` 的 `type` 字段相同。对于 `/dev/radioX`
	设备节点，该类型必须设为 `V4L2_TUNER_RADIO`；对于所有其他节点，设为
	`V4L2_TUNER_ANALOG_TV`。对于调制器，将该字段设为 `V4L2_TUNER_RADIO`（目前只支持
	无线电调制器）。参见 `v4l2_tuner_type`
    - - __u32
      - `index`
      - 标识频段，由应用程序设置。
    - - __u32
      - `capability`
      - `2` 该频段对应的调谐器/调制器能力标志，参见 tuner-capability。所选
	调谐器/调制器的所有频段必须一致地设置 `V4L2_TUNER_CAP_LOW` 或
	`V4L2_TUNER_CAP_1HZ` 能力。也就是说，要么所有频段都设置该能力，要么都不设置。
    - - __u32
      - `rangelow`
      - `2` 该频段最低可调节频率，单位为 62.5 kHz；若设置了 `capability` 标志
	`V4L2_TUNER_CAP_LOW`，则单位为 62.5 Hz。当设置了 `capability` 标志
	`V4L2_TUNER_CAP_1HZ` 时，使用 1 Hz 单位。
    - - __u32
      - `rangehigh`
      - `2` 该频段最高可调节频率，单位为 62.5 kHz；若设置了 `capability` 标志
	`V4L2_TUNER_CAP_LOW`，则单位为 62.5 Hz。当设置了 `capability` 标志
	`V4L2_TUNER_CAP_1HZ` 时，使用 1 Hz 单位。
    - - __u32
      - `modulation`
      - `2` 该频段支持的调制系统，参见 band-modulation。

```

	  目前每个频段只支持一种调制系统。若需要支持多种调制系统，还需要做更多工作。
	  如果你需要此类功能，请联系 linux-media 邮件列表
	  (`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__)。
    * - __u32
      - ``reserved``\ [9]
      - 为将来扩展保留。

	应用程序与驱动都必须将数组置零。


```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_BAND_MODULATION_VSB`
      - 0x02
      - 残留边带（Vestigial Sideband）调制，用于模拟电视。
    - - `V4L2_BAND_MODULATION_FM`
      - 0x04
      - 调频（Frequency Modulation），常用于模拟无线电。
    - - `V4L2_BAND_MODULATION_AM`
      - 0x08
      - 调幅（Amplitude Modulation），常用于模拟无线电。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    `tuner` 或 `index` 越界，或 `type` 字段错误。
