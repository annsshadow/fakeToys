## bttv 驱动


### bttv 与声音迷你操作指


市面上有许多不同的基bt848/849/878/879 的板卡。让视频工作通常不是大问题，
因为这完全由所有板卡共有的 bt8xx 芯片处理。但声音在每个板卡上的处理方
略有不同

为了正确处理这些采集（grabber）板卡，bttv-cards.c 中有一个数tvcards[]
保存每个板卡所需的信息。只有当使用了正确的条目时声音才会工作（对于视频
通常没有区别）。bttv 驱动会向内核打印一
```

	bttv0: model: BT848(Hauppauge old) [autodetected]

```
你应该核实这是正确的。如果不是，你必须将正确的板卡类型作insmod 参数传入
例如 `insmod bttv card=2`。文Documentation/admin-guide/media/bttv-cardlist.rst
card 的有效参数列表

如果你的卡没有列在那里，你可以查看源代码以寻找尚未列出的新条目。如果没
适合你的卡的条目，你可以检查现有条目中是否有对你的卡有效的（只管试错……）

某些板卡有一个额外的处理器用于处理声音，进行立体声解码和其它不错的功能
例如 Hauppauge 使用 msp34xx 芯片。如果你的板卡有这样一个芯片，你可能必
加载一个辅助模块，`msp3400`，以使声音工作。如果你的板卡所用的芯片没有
这样的模块：运气不好。开始写一个吧。嗯，你可能想先查一video4linux
邮件列表存档…

当然，你需要一个正确安装的声卡，除非你把扬声器直接连到采集板卡上。提示：
也检查一下混音器（mixer）设置。例ALSA 默认将所有东西静音


#### 声音如何工作（细节）


仍然不行？看起来需要进行一些驱动修改了。下面是一个供DIY 的说明

bt8xx 芯片32 个通用引脚，以及控制这些引脚的寄存器。一个寄存器是输
使能寄存器（`BT848_GPIO_OUT_EN`），它说明哪些引脚被 bt848 芯片主动驱动
另一个是数据寄存器（`BT848_GPIO_DATA`），你可以在其中获取/设置这些引脚
状态。它们既可用于输入也可用于输出

大多数采集板卡厂商使用这些引脚来控制一个做声音路由的外部芯片。但每块板卡
都有一点不同。这些引脚也被一些公司用来驱动遥控接收芯片。某些板卡使i2c
总线而不gpio 引脚来连接多路复用（mux）芯片

如上所述，有一个数组保存每个已知板卡所需的信息。你基本上必须创建一个新
```

  struct tvcard
  {
	[ ... ]
	u32 gpiomask;
	u32 audiomux[6]; /* Tuner, Radio, external, internal, mute, stereo */
  };

```
gpiomask 指定哪些引脚用于控制音频 mux 芯片。输出使能寄存器
（`BT848_GPIO_OUT_EN`）中相应的位将被置位，因为这些引脚必须由 bt848 芯片
驱动

`audiomux[]` 数组保存不同输入的数据值（即哪些引脚必须为低以实现
tuner/mute/...）。这会被写入数据寄存器（`BT848_GPIO_DATA`）以切换音频 mux


你必须做的是找出 gpiomask audiomux 数组的正确值。如果你安装Windows
以及你板卡的驱动，你可能想看看能否读Windows 驱动使用的这些寄存器值
一个做此事的工具可http://btwincap.sourceforge.net/download.html 获取

你也可以挖掘 Windows 应用程序`*.ini` 文件。你可以查看板卡，看哪些 gpio
引脚被连接，然后开始试错…


0.7.41 版本开始，bttv 有一insmod 选项，使 gpio 调试更容易：

	=================	==============================================
	bttv_gpio=0/1		enable/disable gpio debug messages
	gpiomask=n		set the gpiomask value
	audiomux=i,j,...	set the values of the audiomux array
	audioall=a		set the values of the audiomux array (one
				value for all array elements, useful to check
				out which effect the particular value has).
	=================	==============================================

```

	bttv0: gpio: en=00000027, out=00000024 in=00ffffd8 [audio: off]

	en  =	输出 _en_able 寄存器（BT848_GPIO_OUT_EN
	out =	数据寄存器的 _out_put 位（BT848_GPIO_DATA），
		i.e. BT848_GPIO_DATA & BT848_GPIO_OUT_EN
	in  = 	数据寄存器的 _in_put 位，
		i.e. BT848_GPIO_DATA & ~BT848_GPIO_OUT_EN
```
