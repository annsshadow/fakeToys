
## 如何bt8xx 系列卡工

Authors:
	 Richard Walker,
	 Jamie Honan,
	 Michael Hunold,
	 Manu Abraham,
	 Uwe Bugla,
	 Michael Krufky

### 一般信

这类卡以 bt878a 作为 PCI 接口，并且需bttv 驱动来访bt8xx 芯片组的 i2c 总线gpio 引脚
关于 Linux 内核支持的基Conexant Bt8xx PCI 桥的卡的完整列表，请参见
`Documentation/admin-guide/media/bttv-cardlist.rst`銆。
为了能够编译内核，应配置一些选项
```

    ./scripts/config -e PCI
    ./scripts/config -e INPUT
    ./scripts/config -m I2C
    ./scripts/config -m MEDIA_SUPPORT
    ./scripts/config -e MEDIA_PCI_SUPPORT
    ./scripts/config -e MEDIA_ANALOG_TV_SUPPORT
    ./scripts/config -e MEDIA_DIGITAL_TV_SUPPORT
    ./scripts/config -e MEDIA_RADIO_SUPPORT
    ./scripts/config -e RC_CORE
    ./scripts/config -m VIDEO_BT848
    ./scripts/config -m DVB_BT8XX

```
如果你希望自动支Bt8xx 的所有可能变```

    ./scripts/config -e MEDIA_SUBDRV_AUTOSELECT

```
   请谨慎使用以下选项，因为取消选择实际必需的驱动可能导致因缺少驱动支持而无法调谐的 DVB 设备
如果你的目标只是支持某个特定的板卡，你可以禁MEDIA_SUBDRV_AUTOSELECT 并手动选择你的板卡所需前端驱动。这样，你可以节省一RAM
你可以通过调用 make xconfig/qconfig/menuconfig，并查看这些菜单选项来完成（仅在
`Autoselect ancillary drivers` 被禁用时启用）：

#) `Device drivers` => `Multimedia support` => `Customize TV tuners`
#) `Device drivers` => `Multimedia support` => `Customize DVB frontends`

然后，在上述每个菜单中，请选择你板卡特定的前端与调谐器模块

### 加载模块


常规情况：如bttv 驱动检测到一个基bt8xx DVB 卡，所有前端与后端模块都会自动加载
例外情况有：

- 没有 EEPROM、共享一个通用 PCI 子系ID 的旧电视卡；
- 带有或不带有 CA 插槽、且不包Eeprom 的旧 TwinHan DST 卡或其克隆
在以下情况下，可能需要通过传modprobe 参数来覆bttv dvb-bt8xx 驱动PCI 类型检测
#### 运行 TwinHan 及其克隆

`Documentation/admin-guide/media/bttv-cardlist.rst` 所示，TwinHan 及其克隆卡使`card=113`
modprobe 参数。因此，为了正确
```

	$ modprobe bttv card=113
	$ modprobe dst

```
```

	verbose=0:		禁用消息
		1:		仅显示错误消		2:		显示通知
		3:		显示其他有用的消		4:		调试设置
	dst_addons=0:		卡仅为免费卫星（FTA）卡
		0x20:		卡带有用于加扰频道的条件接收插槽
	dst_algo=0:		（默认）软件调谐算法
	         1:		硬件调谐算法


```
自动检测的值由卡的“响应字符串”决定
在你的日志中可见，例如：dst_get_device_id: Recognize [DSTMCI]
对于缺陷报告，请发送一份激活了 verbose=4 的完整日志。另请参`Documentation/admin-guide/media/ci.rst`
#### 运行多张

关于ID 的完整列表，请参`Documentation/admin-guide/media/bttv-cardlist.rst`。一些示例：

	===========================	===
	Brand name			ID
	===========================	===
	Pinnacle PCTV Sat		 94
	Nebula Electronics Digi TV	104
	pcHDTV HD-2000 TV		112
	Twinhan DST and clones		113
	Avermedia AverTV DVB-T 77:	123
	Avermedia AverTV DVB-T 761	124
	DViCO FusionHDTV DVB-T Lite	128
	DViCO FusionHDTV 5 Lite		135
	===========================	===

   当你有多张卡时，ID 的顺序应与系统检测到它们的顺序一致。请注意，移插入其他 PCI 卡可能会
   改变检测顺序
```

	$ modprobe bttv card=113 card=135

```
如果还有进一步的问题，请订阅并向邮件列表发送问题：linux-media@vger.kernel.org
#### 探测 PCI 子系ID 损坏的卡


有一TwinHan 卡由于某种原因其 EEPROM 已损坏。这些卡没有正确PCI 子系ID```

	$ echo 109e 0878 $subvendor $subdevice > \
		/sys/bus/pci/drivers/bt878/new_id

```
```

	109e: PCI_VENDOR_ID_BROOKTREE
	0878: PCI_DEVICE_ID_BROOKTREE_878

```
