## Linux UVC Gadget 驱动

### Overview（概述）

UVC Gadget 驱动是一个用于 USB 连接中 **设备（device）** 侧硬件的驱动。它意在运行在具备 USB 设备侧硬件（例如带有 OTG 端口的开发板）的 Linux 系统上。

在设备系统上，一旦驱动被绑定，它就会表现为一个具有输出能力的 V4L2 设备。

在主机侧（一旦通过 USB 线缆连接），运行 UVC Gadget 驱动 **并由恰当的用户空间程序控制** 的设备应当表现为一个符合 UVC 规范的摄像头，并能与任何为处理这类设备而设计的程序正常配合工作。运行在设备系统上的用户空间程序可以从各种来源排队图像缓冲区，以便通过 USB 连接传输。通常这意味着从摄像头传感器外设转发缓冲区，但缓冲区的来源完全取决于用户空间的配套程序。

### Configuring the device kernel（配置设备内核）

必须选中 Kconfig 选项 USB_CONFIGFS、USB_LIBCOMPOSITE、USB_CONFIGFS_F_UVC 和 USB_F_UVC 以启用对 UVC gadget 的支持。

### Configuring the gadget through configfs（通过 configfs 配置 gadget）

UVC Gadget 期望通过 configfs 使用 UVC 函数来配置。这提供了相当程度的灵活性，因为 UVC 设备的许多设置都可以通过这种方式来控制。

此处并未描述所有可用属性。完整的枚举请见 Documentation/ABI/testing/configfs-usb-gadget-uvc

#### Assumptions（前提假设）

本节假设您已将 configfs 挂载到 `/sys/kernel/config`，并已将某个 gadget 创建为 `/sys/kernel/config/usb_gadget/g1`。

#### The UVC Function（UVC 函数）

第一步是创建 UVC 函数：


	# These variables will be assumed throughout the rest of the document
	CONFIGFS="/sys/kernel/config"
	GADGET="$CONFIGFS/usb_gadget/g1"
	FUNCTION="$GADGET/functions/uvc.0"

	mkdir -p $FUNCTION

#### Formats and Frames（格式与帧）

您必须通过告知 gadget 您所支持的格式，以及每种格式所支持的帧大小与帧间隔，来配置 gadget。在当前实现中，gadget 没有办法拒绝主机指令它设置的某个格式，因此本步骤 **准确地** 完成非常重要，以确保主机永远不会请求一个无法提供的格式。

格式创建于 streaming/uncompressed 和 streaming/mjpeg 这两个 configfs 组之下，帧大小则创建于格式之下，其结构如下：

```

	uvc.0 +
	      |
	      + streaming +
			  |
			  + mjpeg +
			  |       |
			  |       + mjpeg +
			  |	       |
			  |	       + 720p
			  |	       |
			  |	       + 1080p
			  |
			  + uncompressed +
					 |
					 + yuyv +
						|
						+ 720p
						|
						+ 1080p

```

每个帧随后可以配置宽度和高度，加上存储单帧所需的最大缓冲区大小，最后是相应格式和帧大小所支持的帧间隔。宽度和高度以像素为单位枚举，帧间隔以 100ns 为单位。例如，要为上面对每个帧大小创建含 2、15 和 100 fps 帧间隔的结构，您可以这样做：


	create_frame() {
		# Example usage:
		# create_frame <width> <height> <group> <format name>

		WIDTH=$1
		HEIGHT=$2
		FORMAT=$3
		NAME=$4

		wdir=$FUNCTION/streaming/$FORMAT/$NAME/${HEIGHT}p

		mkdir -p $wdir
		echo $WIDTH > $wdir/wWidth
		echo $HEIGHT > $wdir/wHeight
		echo $(( $WIDTH ** $HEIGHT ** 2 )) > $wdir/dwMaxVideoFrameBufferSize
		cat <<EOF > $wdir/dwFrameInterval
	666666
	100000
	5000000
	EOF
	}

	create_frame 1280 720 mjpeg mjpeg
	create_frame 1920 1080 mjpeg mjpeg
	create_frame 1280 720 uncompressed yuyv
	create_frame 1920 1080 uncompressed yuyv

当前唯一支持的非压缩格式是 YUYV，其细节见 Documentation/userspace-api/media/v4l/pixfmt-packed-yuv.rst。

#### Color Matching Descriptors（色彩匹配描述符）

可以为您创建的每个格式指定一些色度（colorimetry）信息。这一步是可选的，如果跳过，将包含默认信息；这些默认值遵循 UVC 规范中 “色彩匹配描述符”（Color Matching Descriptor）一节的定义。

要创建一个色彩匹配描述符，需创建一个 configfs 项并将其三个属性设为期望的设置，然后从您希望它关联到的格式处建立指向它的链接：


	# Create a new Color Matching Descriptor

	mkdir $FUNCTION/streaming/color_matching/yuyv
	pushd $FUNCTION/streaming/color_matching/yuyv

	echo 1 > bColorPrimaries
	echo 1 > bTransferCharacteristics
	echo 4 > bMatrixCoefficients

	popd

	# Create a symlink to the Color Matching Descriptor from the format's config item
	ln -s $FUNCTION/streaming/color_matching/yuyv $FUNCTION/streaming/uncompressed/yuyv

有关有效取值的详细说明，请查阅 UVC 规范。注意，存在一个默认的色彩匹配描述符，并被任何没有链接到其他色彩匹配描述符的格式所使用。可以更改默认描述符的属性设置，因此请记住，如果您这样做，就是在更改任何未链接到其他描述符的格式的默认值。


#### Header linking（头部链接）

UVC 规范要求 Format 和 Frame 描述符之前要有 Header，用于描述诸如下文不同 Format 描述符的数量与累计大小等信息。这一步以及类似的操作，在 configfs 中通过链接代表 header 的 configfs 项与代表那些其他描述符的 config 项来实现，方式如下：


	mkdir $FUNCTION/streaming/header/h

	# This section links the format descriptors and their associated frames
	# to the header
	cd $FUNCTION/streaming/header/h
	ln -s ../../uncompressed/yuyv
	ln -s ../../mjpeg/mjpeg

	# This section ensures that the header will be transmitted for each
	# speed's set of descriptors. If support for a particular speed is not
	# needed then it can be skipped here.
	cd ../../class/fs
	ln -s ../../header/h
	cd ../../class/hs
	ln -s ../../header/h
	cd ../../class/ss
	ln -s ../../header/h
	cd ../../../control
	mkdir header/h
	ln -s header/h class/fs
	ln -s header/h class/ss


#### Extension Unit Support（扩展单元支持）

一个 UVC 扩展单元（XU）本质上提供了一个独立的单元，控制 set 和 get 请求可以寻址到它。这些控制请求的含义完全取决于实现，但可用于控制在 UVC 规范之外的设置（例如启用或禁用视频特效）。一个 XU 可以插入到 UVC 单元链中，也可以保持游离。

配置扩展单元涉及在相应的目录中创建一个条目并恰当地设置其属性，如下所示：


	mkdir $FUNCTION/control/extensions/xu.0
	pushd $FUNCTION/control/extensions/xu.0

	# Set the bUnitID of the Processing Unit as the source for this
	# Extension Unit
	echo 2 > baSourceID

	# Set this XU as the source of the default output terminal. This inserts
	# the XU into the UVC chain between the PU and OT such that the final
	# chain is IT > PU > XU.0 > OT
	cat bUnitID > ../../terminal/output/default/baSourceID

	# Flag some controls as being available for use. The bmControl field is
	# a bitmap with each bit denoting the availability of a particular
	# control. For example to flag the 0th, 2nd and 3rd controls available:
	echo 0x0d > bmControls

	# Set the GUID; this is a vendor-specific code identifying the XU.
	echo -e -n "\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10" > guidExtensionCode

	popd

bmControls 属性和 baSourceID 属性是多值属性。这意味着您可以向它们写入多个以换行分隔的值。例如要将第 1、2、9、10 个控制标记为可用，您需要向 bmControls 写入两个值，如下所示：


	cat << EOF > bmControls
	0x03
	0x03
	EOF

baSourceID 属性的多值特性掩盖了 XU 可以是多输入这一事实，不过请注意，目前这并没有什么显著影响。

bControlSize 属性反映了 bmControls 属性的大小，类似地，bNrInPins 反映了 baSourceID 属性的大小。当您设置 bmControls 和 baSourceID 时，这两个属性都会自动增大/减小。也可以手动增大或减小 bControlSize，其效果是将条目截断到新大小，或用 0x00 填充条目，例如：

```

	$ cat bmControls
	0x03
	0x05

	$ cat bControlSize
	2

	$ echo 1 > bControlSize
	$ cat bmControls
	0x03

	$ echo 2 > bControlSize
	$ cat bmControls
	0x03
	0x00

```

bNrInPins 和 baSourceID 以相同方式工作。

#### Configuring Supported Controls for Camera Terminal and Processing Unit（为 Camera Terminal 和 Processing Unit 配置受支持的控制）

UVC 链中的 Camera Terminal 和 Processing Unit 也拥有 bmControls 属性，其作用类似于扩展单元中的同名字段。不过与 XU 不同的是，这些单元的位标志含义在 UVC 规范中有定义；您应当查阅 “Camera Terminal Descriptor” 和 “Processing Unit Descriptor” 两节以获取这些标志的枚举。


        # Set the Processing Unit's bmControls, flagging Brightness, Contrast
        # and Hue as available controls:
        echo 0x05 > $FUNCTION/control/processing/default/bmControls

        # Set the Camera Terminal's bmControls, flagging Focus Absolute and
        # Focus Relative as available controls:
        echo 0x60 > $FUNCTION/control/terminal/camera/default/bmControls

如果您不设置这些字段，默认情况下 Camera Terminal 的 Auto-Exposure Mode 控制和 Processing Unit 的 Brightness 控制会被标记为可用；如果它们不被支持，您应当将该字段设为 0x00。

注意，Camera Terminal 或 Processing Unit 的 bmControls 字段的大小由 UVC 规范固定，因此这里的 bControlSize 属性是只读的。

#### Custom Strings Support（自定义字符串支持）

为 USB 设备各部分提供文字描述的字符串描述符，可以在 USB configfs 中通常的位置定义，然后可以从 UVC 函数根目录或扩展单元目录链接过去，以将这些字符串指派为描述符：


	# Create a string descriptor in us-EN and link to it from the function
	# root. The name of the link is significant here, as it declares this
	# descriptor to be intended for the Interface Association Descriptor.
	# Other significant link names at function root are vs0_desc and vs1_desc
	# For the VideoStreaming Interface 0/1 Descriptors.

	mkdir -p $GADGET/strings/0x409/iad_desc
	echo -n "Interface Associaton Descriptor" > $GADGET/strings/0x409/iad_desc/s
	ln -s $GADGET/strings/0x409/iad_desc $FUNCTION/iad_desc

	# Because the link to a String Descriptor from an Extension Unit clearly
	# associates the two, the name of this link is not significant and may
	# be set freely.

	mkdir -p $GADGET/strings/0x409/xu.0
	echo -n "A Very Useful Extension Unit" > $GADGET/strings/0x409/xu.0/s
	ln -s $GADGET/strings/0x409/xu.0 $FUNCTION/control/extensions/xu.0

#### The interrupt endpoint（中断端点）

VideoControl 接口有一个可选的中断端点，默认是禁用的。它旨在支持 UVC 的延迟响应控制 set 请求（应当通过该中断端点而非占用端点 0 来响应）。目前尚不支持通过该端点发送数据，因此将其保持禁用以免混淆。如果您希望启用它，可以通过 configfs 属性来做到：


	echo 1 > $FUNCTION/control/enable_interrupt_ep

#### Bandwidth configuration（带宽配置）

有三个属性控制 USB 连接的带宽。它们位于函数根目录，可以在限制范围内设置：


	# streaming_interval sets bInterval. Values range from 1..255
	echo 1 > $FUNCTION/streaming_interval

	# streaming_maxpacket sets wMaxPacketSize. Valid values are 1024/2048/3072
	echo 3072 > $FUNCTION/streaming_maxpacket

	# streaming_maxburst sets bMaxBurst. Valid values are 1..15
	echo 1 > $FUNCTION/streaming_maxburst


这里传入的值会根据 UVC 规范（取决于 USB 连接的速度）被钳制到有效值。要理解这些设置如何影响带宽，您应当查阅 UVC 规范，但一条经验法则是：增大 streaming_maxpacket 设置会提升带宽（从而提升最大可能的帧率），在 USB 连接运行于 SuperSpeed 时，streaming_maxburst 同理。增大 streaming_interval 会降低带宽和帧率。

### The userspace application（用户空间应用程序）

单凭 UVC Gadget 驱动本身无法做任何特别有趣的事。它必须与一个响应用 UVC 控制请求、并填充缓冲区以便排队到驱动所创建的 V4L2 设备的用户空间程序配合使用。这些事情如何达成取决于具体实现，超出了本文档的范围，但可以在 https://gitlab.freedesktop.org/camera/uvc-gadget 找到一个参考应用程序
