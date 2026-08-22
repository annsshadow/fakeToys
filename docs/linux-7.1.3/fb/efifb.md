## efifb - 通用 EFI 平台驱动


这是一款面向使UEFI 固件系统的通用 EFI 平台驱动。系统必须通过 EFI stub 启动才能使用它。efifb 既支持带Graphics Output Protocol（GOP，图形输出协议）显示的系统，也支持仅带有 Universal Graphics Adapter（UGA，通用图形适配器）显示的旧系统
## 支持的硬

- iMac 17"/20"
- Macbook
- Macbook Pro 15"/17"
- MacMini
- 带有 UEFI 固件ARM/ARM64/X86 系统

## 如何使用

对于 UGA 显示，efifb 不会以任何方式自动检测你的机器
```

	Macbook :
		video=efifb:macbook
	MacMini :
		video=efifb:mini
	Macbook Pro 15", iMac 17" :
		video=efifb:i17
	Macbook Pro 17", iMac 20" :
		video=efifb:i20

```
对于 GOP 显示，efifb 可以自动检测显示的分辨率与帧缓冲地址，因此这些应该开箱即用，无需任何特殊参数
可接受的选项
======= ===========================================================
nowc	不要将帧缓冲映射为写组合（write combined）。当写入大量控制台数据时	可用于规避其CPU 核心上的副作用与性能下降======= ===========================================================

GOP 显示的选项
mode=n
        EFI stub 会尽可能将显示的模式设置为模式编n
<xres>x<yres>[-(rgb|bgr|<bpp>)]
        EFI stub 会搜索与指定的水平和垂直分辨率（以及可选的位深）相匹配的显示模式，
        若找到则将其设置为显示的模式。位深可以是“rgb”或“bgr”以专门匹配那些像素格式        也可以是一个数字以匹配每像素位数相同的模式
auto
        EFI stub 会选择分辨率最高（水平与垂直分辨率的乘积）的模式。若存在多个最高分        率的模式，则选择颜色深度最高的一个
list
        EFI stub 会列出所有可用的显示模式。随后可使用上述某个选项为下一次启动选择特定模式
Edgar Hucek <gimli@dark-green.com>
