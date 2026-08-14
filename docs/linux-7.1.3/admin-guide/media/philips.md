
## Philips webcams (pwc driver)


本文档包含一些关于 Philips 及 OEM 网络摄像头（webcam）的附加信息。
E-mail: webcam@smcc.demon.nl                        Last updated: 2004-01-19
Site: http://www.smcc.demon.nl/webcam/

截至目前，支持以下摄像头：

 - Philips PCA645
 - Philips PCA646
 - Philips PCVC675
 - Philips PCVC680
 - Philips PCVC690
 - Philips PCVC720/40
 - Philips PCVC730
 - Philips PCVC740
 - Philips PCVC750
 - Askey VC010
 - Creative Labs Webcam 5
 - Creative Labs Webcam Pro Ex
 - Logitech QuickCam 3000 Pro
 - Logitech QuickCam 4000 Pro
 - Logitech QuickCam Notebook Pro
 - Logitech QuickCam Zoom
 - Logitech QuickCam Orbit
 - Logitech QuickCam Sphere
 - Samsung MPC-C10
 - Samsung MPC-C30
 - Sotec Afina Eye
 - AME CU-001
 - Visionite VCS-UM100
 - Visionite VCS-UC300

Philips 驱动的主网页位于上面的地址。它包含大量额外信息、一个 FAQ，以及二进制插件
'PWCX'。该插件包含解压缩例程，可让你使用更高的图像尺寸和帧率；此外网络摄像头在 USB
总线上占用更少的带宽（如果你想同时运行超过 1 个摄像头，这会很方便）。这些例程受 NDA
约束，因此不能以源代码形式分发；不过，它的使用是完全可选的。

你可以将此代码编译进内核，或编译为模块。我推荐后者，因为它使故障排查容易得多。内置的
麦克风通过 USB Audio class 支持。

加载模块时，你可以为摄像头设置一些默认设置；某些程序依赖于特定的图像尺寸或格式，并且
不知道如何在驱动中正确设置。选项如下：

size
   可以是 'sqcif'、'qsif'、'qcif'、'sif'、'cif' 或 'vga' 之一，对应的图像尺寸分别为
   128x96、160x120、176x144、320x240、352x288 和 640x480（当然，仅适用于支持这些
   分辨率的摄像头）。

fps
   指定期望的帧率。为整数，范围 4-30。

fbufs
   此参数指定用于存储来自摄像头帧的内部缓冲区数量。如果读取图像的程序稍慢或暂时繁忙，
   这会有所帮助。然而，在慢速机器上它只会引入延迟，因此要谨慎选择。默认为 3，这是合理
   的。可以设置在 2 到 5 之间。

mbufs
   这是一个 1 到 10 之间的整数。它告诉模块为 mmap()、VIDIOCCGMBUF、VIDIOCMCAPTURE 等
   保留的缓冲区数量。默认为 2，对大多数应用（双缓冲）而言是足够的。

   如果你在使用 mmap() 的工具抓取时遇到大量 'Dumping frame...' 消息，你可能会想增大它。
   不过，它并不真正缓冲图像，只是在你的程序落后时给你多一点点余量。但你需要一个多线程或
   已 fork 的程序才能真正利用这些缓冲区。

   绝对最大值是 10，但不要把设得太高！每个缓冲区占用 460 KB 的 RAM，所以除非你有大量
   内存，否则设成 4 以上纯粹是浪费。这块内存在 open() 期间才分配，因此摄像头不使用时不会
   浪费任何东西。

power_save
   当启用 power_save（设为 1）时，模块将在 close() 时尝试关闭摄像头，并在 open() 时重新
   激活。这会节省功耗并关闭 LED。不过并非所有摄像头都支持（645 和 646 根本不支持省电），
   而且某些型号也无法工作（它们会关闭，但再也不会唤醒）。请将其视为实验性特性。默认禁用
   此选项。

compression（仅在与插件一起使用时有效）
   通过此选项你可以控制摄像头用于通过 USB 总线压缩图像的压缩因子。你可以设置
```

     0 = prefer uncompressed images; if the requested mode is not available
	 in an uncompressed format, the driver will silently switch to low
	 compression.
     1 = low compression.
     2 = medium compression.
     3 = high compression.

   High compression takes less bandwidth of course, but it could also
   introduce some unwanted artefacts. The default is 2, medium compression.
   See the FAQ on the website for an overview of which modes require
   compression.

   The compression parameter does not apply to the 645 and 646 cameras
   and OEM models derived from those (only a few). Most cams honour this
   parameter.

```
leds
   此设置接受 2 个整数，定义 LED 的亮/灭时间（毫秒）。你可以用它做的一件有趣的事情
```

     leds=500,500

   will blink the LED once every second. But with::

     leds=0,0

   the LED never goes on, making it suitable for silent surveillance.

   By default the camera's LED is on solid while in use, and turned off
   when the camera is not used anymore.

   This parameter works only with the ToUCam range of cameras (720, 730, 740,
   750) and OEMs. For other cameras this command is silently ignored, and
   the LED cannot be controlled.

   Finally: this parameters does not take effect UNTIL the first time you
   open the camera device. Until then, the LED remains on.

```
dev_hint
   一个长期存在的问题是 USB 设备的动态特性：你永远不知道一个摄像头被分配了什么设备；它
   取决于模块加载顺序、hub 配置、设备插入的顺序，以及月亮的相位（即它可能是随机的）。通过
   此选项你可以给驱动一个提示，告诉它某个特定摄像头应使用哪个视频设备节点
   （/dev/videoX）。如果你有两台同型号的摄像头，这也很有用。

   一个摄像头由其类型（来自摄像头型号的数字，如 PCA645、PCVC750VC 等）以及可选的序列号
   （可在 /sys/kernel/debug/usb/devices 中看到）指定。提示由包含
```

      [type[.serialnumber]:]node

   The square brackets mean that both the type and the serialnumber are
   optional, but a serialnumber cannot be specified without a type (which
   would be rather pointless). The serialnumber is separated from the type
   by a '.'; the node number by a ':'.

   This somewhat cryptic syntax is best explained by a few examples::

     dev_hint=3,5              The first detected cam gets assigned
			       /dev/video3, the second /dev/video5. Any
			       other cameras will get the first free
			       available slot (see below).

     dev_hint=645:1,680:2      The PCA645 camera will get /dev/video1,
			       and a PCVC680 /dev/video2.

     dev_hint=645.0123:3,645.4567:0	The PCA645 camera with serialnumber
					0123 goes to /dev/video3, the same
					camera model with the 4567 serial
					gets /dev/video0.

     dev_hint=750:1,4,5,6       The PCVC750 camera will get /dev/video1, the
				next 3 Philips cams will use /dev/video4
				through /dev/video6.

   Some points worth knowing:

   - Serialnumbers are case sensitive and must be written full, including
     leading zeroes (it's treated as a string).
   - If a device node is already occupied, registration will fail and
     the webcam is not available.
   - You can have up to 64 video devices; be sure to make enough device
     nodes in /dev if you want to spread the numbers.
     After /dev/video9 comes /dev/video10 (not /dev/videoA).
   - If a camera does not match any dev_hint, it will simply get assigned
     the first available device node, just as it used to be.

```
trace
   为了更好地发现问题，现在可以开启模块所做的一些调用的“trace”；它会在 debug 级别将
   所有项记录到你的内核日志中。

   trace 变量是一个位掩码；每一位代表一个特定特性。如果你想跟踪某些内容，在下表中查找
   位值，将值相加后提供给 trace 变量。

   ====== ======= ================================================ =======
   Value  Value   Description					   Default
   (dec)  (hex)
   ====== ======= ================================================ =======
       1    0x1   模块初始化；将在加载和卸载模块时记录消息        On

       2    0x2   probe() 和 disconnect() 跟踪                     On

       4    0x4   跟踪 open() 和 close() 调用                      Off

       8    0x8   read()、mmap() 以及相关的 ioctl() 调用           Off

      16   0x10   缓冲区的内存分配等                               Off

      32   0x20   显示下溢、溢出以及 Dumping frame 消息            On

      64   0x40   显示视口和图像尺寸                               Off

     128   0x80   PWCX 调试                                        Off
   ====== ======= ================================================ =======

   例如，要跟踪 open() 和 read() 函数，求和 8 + 4 = 12，因此你应在 insmod 或 modprobe
   时提供 trace=12。如果你想关闭初始化和探测跟踪，设置 trace=0。trace 的默认值是 35
   （0x23）。



```

     # modprobe pwc size=cif fps=15 power_save=1

```
fbufs、mbufs 和 trace 参数是全局的，适用于所有已连接的摄像头。每个摄像头有自己的一组
缓冲区。

size 和 fps 仅在 open() 设备时指定默认值；这是为了兼容一些不设置尺寸的工具。你可以在
open() 之后用 Video4Linux 的 ioctl() 调用更改这些设置。默认中的默认是 10 fps 下的 QCIF
尺寸。

compression 参数是半全局的；它为所有摄像头设置初始压缩偏好，但此参数可以通过
VIDIOCPWCSCQUAL ioctl() 调用按每个摄像头设置。

所有参数都是可选的。

