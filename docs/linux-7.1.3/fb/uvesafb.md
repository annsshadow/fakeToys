## uvesafb —兼容 VBE2+ 规范的通用显卡驱动


### 1. 需求（Requirements

uvesafb 适用于带有符VBE 2.0 标准 Video BIOS 的显卡
与其他驱动不同，uvesafb 借助一个名v86d 的用户态辅助程序工作。v86d 利用
CPU 在受控的模拟环境中运x86 Video BIOS 代码。这使得 uvesafb 能够x86
之外的架构上也能工作。关于当前受支持架构的列表，请查v86d 的文档
v86d 的源代码可从以下网站下载
https://github.com/mjanusz/v86d

有关详细的配置与安装说明，请参阅 v86d 的文档
请注意，为了uvesafb 正常工作，v86d 这个用户态辅助程序必须始终可用。如希望在启动早期就使用 uvesafb，必须把 v86d 包含initramfs 镜像，并以启initrd 支持的方式编译内核
### 2. 注意事项与限制（Caveats and limitations

uvesafb 是一个_通用_驱动，支持种类繁多的显卡，但其功能最终受限于 Video BIOS
接口。最重要的限制包括：

- 不支持任何类型的硬件加速- 支持的显示模式数量极为有限。通常，最优的原生分辨刷新率组合在 uvesafb
  下并不可用，原因仅仅Video BIOS 不支持你想要使用的显示模式。对于宽  面板而言这尤其令人困扰，因为其原生显示模式并4:3 宽高比，而大多数 BIOS
  都局限于此- 调整刷新率只有在 Video BIOS 兼容 VBE 3.0 时才有可能。注意，许多 nVidia   Video BIOS 声称兼容 VBE 3.0，但实际上会直接忽略刷新率设置
### 3. 配置（Configuration

uvesafb 既可编译为模块，也可直接编入内核。两种方式都支持相同的配置选项集，
例如
```

 video=uvesafb:1024x768-32,mtrr:3,ywrap (compiled into the kernel)

 # modprobe uvesafb mode_option=1024x768-32 mtrr=3 scroll=ywrap  (module)

```

可接受的选项如下
======= =========================================================
ypan 启用显示平移（display panning），使用 VESA 保护模式接口。可见屏幕只显存中的一个窗口，控制台滚动通过改变窗口起始位置实现。该选项x86 架构默认可用，也是该架构的默认选项ywrap ypan 类似，但假设显卡会在显存末端回绕读取（即读到显存末尾后从头部
重新开始）。比 ypan 更快。可用于 x86redraw 通过重绘屏幕受影响的部分来实现滚动，在非 x86 架构上为默认======= =========================================================

（如果你使用 uvesafb 模块，上述三个选项可作为名"scroll" 的模块参数使用，
例如 scroll=ypan。）

=========== ====================================================================
vgapal 使用标准 VGA 寄存器进行调色板（palette）更改pmipal 使用保护模式接口进行调色板更改。保护模式接口在可用时为默认。可用于 x86mtrr:n 为帧缓冲设置内存类型范围寄存器（MTRR），参数n- 0 - 禁用（相当于 nomtrr- 3 - 写入合并（write-combining，默认）
取0 3，超出范围会给出警告并视3 处理nomtrr 不使用内存类型范围寄存器vremap:n 重映'n' MiB 的显存。若指定0，则根据显示模式重映射显存vtotal:n Video BIOS 错误判断了显存总量时，用此选项覆盖 BIOS 的取值（单位
MiB）<模式> 要设置的显示模式，采用标modedb 格式。有关详细描述，请参modedb.txt。若 uvesafb 编译为模块，模式字符串通过 'mode_option' 选项提供vbemode:x 强制使用 VBE 模式 x。该模式应在 VBE 提供的支持模式列表中查找。注意：
模式编号 'x' 采用 VESA 模式编号表示法，而非 Linux 内核的表示法（例257 而非
769）。提示：若要将此选项与普通的 <模式> 参数一起用X 服务器，你可能还需设置 'nocrtc' 选项，以确保视频模式在控制台X 之间切换时能正确恢复nocrtc 使用 CRTC 时序来设置视频模式。此选项仅在 Video BIOS 兼容 VBE 3.0 有效。若以标准方式设置模式遇到问题，可使用此选项。注意：使用此选项意味着刷新
率调整将被忽略，刷新率保持为 BIOS 默认值（60 Hz）noedid 不尝试通过 EDID 获取显示模式noblank 禁用硬件消隐（blanking）v86d:path 设置 v86d 可执行文件的路径。该选项可作为模块参数使用，也可作为
video= 字符串的一部分。若 uvesafb 编入内核，则使用 uvesafb.v86d="path"=========== ====================================================================

此外，还提供以下参数，用于覆EDID 提供的值或 BIOS 默认值。关maxhfmaxvf maxclk 的正确取值，请参阅显示器规格
=========== ======================================
maxhf:n 最大水平频率（单位 kHz）maxvf:n 最大垂直频率（单位 Hz）maxclk:n 最大像素时钟（单位 MHz）=========== ======================================

### 4. sysfs 接口（The sysfs interface

uvesafb 提供若干 sysfs 节点，用于配置参数及提供额外信息
驱动属性：

/sys/bus/platform/drivers/uvesafb
v86d
（默认值：/sbin/v86dv86d 可执行文件的路径。若守护进程实例尚未运行，uvesafb 会启v86d
设备属性：

/sys/bus/platform/drivers/uvesafb/uvesafb.0
nocrtc
设为 1 时使用默认刷新率0 Hz）
oem_product_name、oem_product_rev、oem_string、oem_vendor
关于显卡制造商的信息
vbe_modes
Video BIOS 支持的显示模式列表，以及对应VBE 模式编号（十六进制）
vbe_version
指示所实现VBE 标准BCD 值
### 5. 杂项（Miscellaneous

Uvesafb Video BIOS 获取显示模式与默认刷新率时序，并fb_var_screeninfo
中将 pixclock 设为 0

Michal Januszewski <spock@gentoo.org>

Last updated: 2017-10-10

本文档关uvesafb 选项的说明大致基vesafb.txt