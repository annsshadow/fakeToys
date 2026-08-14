
## 为媒体设备构建支持


第一步是下载内核源代码，可以通过发行版特定的源文件，也可以通过内核的主 git 树\ [^1^]_。

但请注意，如果：

- 你是个勇敢者，想尝试新东西；
- 如果你想报告一个 bug；
- 如果你正在开发新补丁

你应该使用主媒体开发树的 `master` 分支：

    https://git.linuxtv.org/media.git/

在这种情况下，你可以在
`LinuxTv wiki 页面 <https://linuxtv.org/wiki>`_ 找到一些有用的信息：

    https://linuxtv.org/wiki/index.php/How_to_Obtain,_Build_and_Install_V4L-DVB_Device_Drivers


       https://git.kernel.org/pub/scm/li  nux/kernel/git/torvalds/linux.git/

## 配置 Linux 内核


```
    $ make menuconfig

```
然后，选择所有期望的选项并退出，保存配置。

修改后的配置将位于 `.config` 文件中。它大概会是
```

    ...
    # CONFIG_RC_CORE is not set
    # CONFIG_CEC_CORE is not set
    CONFIG_MEDIA_SUPPORT=m
    CONFIG_MEDIA_SUPPORT_FILTER=y
    ...

```
```
    Device Drivers --->
	<M> Remote Controller support  --->
	[ ] HDMI CEC RC integration
	[ ] Enable CEC error injection support
	[*] HDMI CEC drivers  --->
	<*> Multimedia support  --->

```
`Remote Controller support` 选项启用对遥控器的核心支持\ [^2^]_。

`HDMI CEC RC integration` 选项启用 HDMI CEC 与 Linux 的集成，允许像接收由直接连接到机器的遥控器产生的数据一样，通过 HDMI CEC 接收数据。

`HDMI CEC drivers` 选项允许选择通过 HDMI 接口接收和/或发送 CEC 码的平臺和 USB 驱动\ [^3^]_。

最后一个选项（`Multimedia support`）启用对摄像头、音视频采集卡和电视的支持。

媒体子系统支持既可以与主内核一起构建，也可以作为模块构建。在大多数用例中，更倾向于将其构建为模块。


   与其使用菜单，内核还提供了一个脚本，允许直接启用配置选项。要启用媒体支持
```

	$ scripts/config -m RC_CORE
	$ scripts/config -m MEDIA_SUPPORT

```
       想要使用某些可能依赖遥控器核心支持的电视卡驱动时。

       使用媒体 HDMI CEC 支持时。

       这些特定于 GPU 的驱动通过 `Device Drivers` 下的 `Graphics support`
       菜单选择。

       当某个 GPU 驱动支持 HDMI CEC 时，它会自动在媒体子系统启用 CEC 核心支持。

### 媒体依赖


应当注意，从一个干净的配置开始启用上述选项通常还不够。媒体子系统依赖于若干其他 Linux 核心支持才能工作。

例如，大多数媒体设备使用串行通信总线来与某些外设通信。这种总线称为 I²C
（Inter-Integrated Circuit，集成电路间总线）。为了能够构建对此类硬件的支持，应当启用 I²C 总线支持，可以通过
```

    ./scripts/config -m I2C

```
另一个例子：遥控器核心需要支持
```

    ./scripts/config -m INPUT

```
根据你想启用的具体驱动，可能还需要其他核心功能（如 PCI 和/或 USB 支持）。

### 启用遥控器支持


遥控器菜单允许选择特定设备的驱动。
```

         --- Remote Controller support
         <M>   Compile Remote Controller keymap modules
         [*]   LIRC user interface
         [*]     Support for eBPF programs attached to lirc devices
         [*]   Remote controller decoders  --->
         [*]   Remote Controller devices  --->

```
`Compile Remote Controller keymap modules` 选项为若干个流行的遥控器创建键映射。

`LIRC user interface` 选项通过启用一个允许用户空间接收来自遥控器的原始数据的 API，在使用 `lirc` 程序时增加增强功能。

`Support for eBPF programs attached to lirc devices` 选项允许使用特殊程序（称为 eBPF），让应用程序能够向 Linux 内核添加额外的遥控器解码功能。

`Remote controller decoders` 选项允许选择将被 Linux 内核识别的协议。除非你想禁用某个特定的解码器，否则建议保持所有子选项启用。

`Remote Controller devices` 允许你选择支持你的设备所需的驱动。

同样的配置也可以通过 `script/config` 脚本设置。例如，为了支持 ITE 遥控器
```

	$ scripts/config -e INPUT
	$ scripts/config -e ACPI
	$ scripts/config -e MODULES
	$ scripts/config -m RC_CORE
	$ scripts/config -e RC_DEVICES
	$ scripts/config -e RC_DECODERS
	$ scripts/config -m IR_RC5_DECODER
	$ scripts/config -m IR_ITE_CIR

```
### 启用 HDMI CEC 支持


当某个驱动需要 HDMI CEC 支持时，它会自动设置。因此，你只需要启用对需要它的显卡的支持，或者通过某个现有的 HDMI 驱动来启用。

特定于 HDMI 的驱动位于 `HDMI CEC drivers`
```

	--- HDMI CEC drivers
	< >   ChromeOS EC CEC driver
	< >   Amlogic Meson AO CEC driver
	< >   Amlogic Meson G12A AO CEC driver
	< >   Generic GPIO-based CEC driver
	< >   Samsung S5P CEC driver
	< >   STMicroelectronics STiH4xx HDMI CEC driver
	< >   STMicroelectronics STM32 HDMI CEC driver
	< >   Tegra HDMI CEC driver
	< >   SECO Boards HDMI CEC driver
	[ ]     SECO Boards IR RC5 support
	< >   Pulse Eight HDMI CEC
	< >   RainShadow Tech HDMI CEC

```
       HDMI 设备依赖于系统的架构，并且在新内核上可能有所不同。

### 启用媒体支持


媒体菜单比遥控器菜单有更多的选项。
```

	--- Media support
	[ ] Filter media drivers
	[*] Autoselect ancillary drivers
	    Media device types --->
	    Media core support --->
	    Video4Linux options --->
	    Media controller options --->
	    Digital TV options --->
	    HDMI CEC options --->
	    Media drivers --->
	    Media ancillary drivers --->

```
除非你确切知道自己在做什么，或者你想为 SoC 平台构建一个驱动，否则强烈建议保持 `Autoselect ancillary drivers` 选项开启，因为它会自动选择所需的 I²C 辅助驱动。

现在有如下所述的两种方式来选择媒体设备驱动。

##### ``Filter media drivers`` 菜单


此菜单旨在简化 PC 和笔记本电脑硬件的设置。它的工作方式是让用户指定需要哪类媒体驱动，
```

	[ ] Cameras and video grabbers
	[ ] Analog TV
	[ ] Digital TV
	[ ] AM/FM radio receivers/transmitters
	[ ] Software defined radio
	[ ] Platform-specific devices
	[ ] Test drivers

```
因此，如果你只想添加对摄像头或视频采集卡的支持，只选择第一个选项即可。允许多选。

一旦选择了此菜单上的选项，构建系统就会自动选择所需的核心驱动，以支持所选功能。


   大多数电视卡是混合的：它们同时支持模拟电视和数字电视。

   如果你有一张混合卡，可能需要在菜单中同时启用 `Analog TV`
   和 `Digital TV`。

使用此选项时，媒体支持核心功能的默认值通常足以提供驱动的基本功能。不过，你可以使用以下各项设置下的配置手动启用一些所需的额外（可选）功能
```

	    Media core support --->
	    Video4Linux options --->
	    Media controller options --->
	    Digital TV options --->
	    HDMI CEC options --->

```
一旦选择了所需的过滤器，符合过滤条件的驱动将在 `Media support->Media drivers` 子菜单中可用。

##### ``Media Core Support`` 菜单（不过滤）


如果禁用 `Filter media drivers` 菜单，所有依赖已满足、可用于你的系统的驱动都应该显示在 `Media drivers` 菜单中。

但请注意，你应当首先确保 `Media Core Support` 菜单具备你的驱动所需的所有核心功能，否则相应的设备驱动不会显示。

### 示例


为了对 [this table <cx231xx-cardlist>](this table <cx231xx-cardlist>) 中列出的某一块板卡启用模块化支持，并配合模块化的媒体核心模块，
```

    CONFIG_MODULES=y
    CONFIG_USB=y
    CONFIG_I2C=y
    CONFIG_INPUT=y
    CONFIG_RC_CORE=m
    CONFIG_MEDIA_SUPPORT=m
    CONFIG_MEDIA_SUPPORT_FILTER=y
    CONFIG_MEDIA_ANALOG_TV_SUPPORT=y
    CONFIG_MEDIA_DIGITAL_TV_SUPPORT=y
    CONFIG_MEDIA_USB_SUPPORT=y
    CONFIG_VIDEO_CX231XX=y
    CONFIG_VIDEO_CX231XX_DVB=y

```
## 构建并安装新内核


一旦 `.config` 文件具备了一切所需，构建所需的一切就是
```

    $ make

```
```

    $ sudo make modules_install
    $ sudo make install

```
## 仅构建新的媒体驱动和核心


从开发树运行一个新的开发内核通常是有风险的，因为它可能包含可能有 bug 的实验性改动。因此，有一些方法可以使用替代树来仅构建新的驱动。

有一个 `Linux Kernel backports 项目
<https://backports.wiki.kernel.org/index.php/Main_Page>`_，其中包含旨在针对稳定内核编译的较新驱动。

负责维护媒体子系统的 LinuxTV 开发者也维护了一个 backport 树，其中只包含每天从最新内核更新的媒体驱动。该树位于：

https://git.linuxtv.org/media_build.git/

应当注意，虽然将 `media_build` 树用于测试目的相对安全，但并不能保证它能在随机的内核上工作（甚至构建成功）。该树遵循“尽力而为”的原则维护，在我们时间允许时修复其中的问题。

如果你发现它有任何问题，欢迎向 Linux 媒体子系统的邮件列表提交补丁：media@vger.kernel.org。如果你为 media-build 提交新补丁，请在邮件主题中添加 `[PATCH media-build]`。

```

    $ ./build

```

    1) 如果 `media-build` 树被更新，你可能需要运行两次；
    2) 如果你过去曾为与你当前使用的不同内核版本构建过它，你可能需要执行一次 `make distclean`；
    3) 默认情况下，它会使用你正在运行的内核中为媒体定义的相同配置选项。

为了选择不同的驱动或不同的配置选项，
```

    $ make menuconfig

```
```

    $ make && sudo make install

```
这将覆盖你的内核之前正在使用的媒体驱动。
