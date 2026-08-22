## Gadget 测试


本文件总结了关于对 gadget 所提供USB 功能进行基本测试的信息

   1. ACM 功能
   2. ECM 功能
   3. ECM subset 功能
   4. EEM 功能
   5. FFS 功能
   6. HID 功能
   7. LOOPBACK 功能
   8. MASS STORAGE 功能
   9. MIDI 功能
   10. NCM 功能
   11. OBEX 功能
   12. PHONET 功能
   13. RNDIS 功能
   14. SERIAL 功能
   15. SOURCESINK 功能
   16. UAC1 功能（旧实现   17. UAC2 功能
   18. UVC 功能
   19. PRINTER 功能
   20. UAC1 功能（新 API   21. MIDI2 功能


## 1. ACM 功能


该功能由 usb_f_acm.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"acm"。ACM 功能在其功能目录中只提供一个属性：

	port_num

该属性是只读的
系统中最多可以有 4 ACM/通用串行/OBEX 端口

### 测试 ACM 功能


```
	cat > /dev/ttyACM<X>
```
```
	cat /dev/ttyGS<Y>
```
鐒跺悗鍙嶈繃鏉。
```
	cat > /dev/ttyGS<Y>
```
```
	cat /dev/ttyACM<X>
```
## 2. ECM 功能


该功能由 usb_f_ecm.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"ecm"。ECM 功能在其功能目录中提供以下属性：

	=============== ==================================================
	ifname		与本功能实例关联的网络设备接口名
	qmult		高速与超速下的队列长度乘	host_addr	USB 以太链路上主机一侧的 MAC 地址
	dev_addr		USB 以太链路上设备一侧的 MAC 地址
	=============== ==================================================

在创functions/ecm.<实例 之后，它们包含默认值：qmult 5，dev_addr host_addr 为随机选择。如果功能未绑定，ifname 可被写入。写入内容必须是一个接口模式，
例如 "usb%d"，这将导致网络核心选择下一个空闲的 usbX 接口。默认情况下它被设为 "usb%d"
### 测试 ECM 功能


配置设备与主机的 IP 地址。然后：

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 3. ECM subset 功能


该功能由 usb_f_ecm_subset.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"geth"。ECM subset 功能在其功能目录中提供以下属性：

	=============== ==================================================
	ifname		与本功能实例关联的网络设备接口名
	qmult		高速与超速下的队列长度乘	host_addr	USB 以太链路上主机一侧的 MAC 地址
	dev_addr		USB 以太链路上设备一侧的 MAC 地址
	=============== ==================================================

在创functions/ecm.<实例 之后，它们包含默认值：qmult 5，dev_addr host_addr 为随机选择。如果功能未绑定，ifname 可被写入。写入内容必须是一个接口模式，
例如 "usb%d"，这将导致网络核心选择下一个空闲的 usbX 接口。默认情况下它被设为 "usb%d"
### 测试 ECM subset 功能


配置设备与主机的 IP 地址。然后：

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 4. EEM 功能


该功能由 usb_f_eem.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"eem"。EEM 功能在其功能目录中提供以下属性：

	=============== ==================================================
	ifname		与本功能实例关联的网络设备接口名
	qmult		高速与超速下的队列长度乘	host_addr	USB 以太链路上主机一侧的 MAC 地址
	dev_addr		USB 以太链路上设备一侧的 MAC 地址
	=============== ==================================================

在创functions/eem.<实例 之后，它们包含默认值：qmult 5，dev_addr host_addr 为随机选择。如果功能未绑定，ifname 可被写入。写入内容必须是一个接口模式，
例如 "usb%d"，这将导致网络核心选择下一个空闲的 usbX 接口。默认情况下它被设为 "usb%d"
### 测试 EEM 功能


配置设备与主机的 IP 地址。然后：

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 5. FFS 功能


该功能由 usb_f_fs.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"ffs"。该功能目录被有意留空且不可修改
创建目录之后，系统中会出FunctionFS 的一个新实例（一"device"）。一"device"
可用，用户应遵循使用 FunctionFS 的标准流程（挂载它、运行实现该功能本身的用户空间进程）gadget 应通过usb_gadget/<gadget>/UDC 写入合适的字符串来启用
FFS 功能在其功能目录中只提供一个属性：

	ready

该属性是只读的，用于指示功能是否已就绪（1）可供使用，例如用户空间是否已向 ep0 写入
描述符与字符串，从而可以启gadget
### 测试 FFS 功能


设备端：启动该功能的用户空间守护进程，启gadget

主机端：使用设备提供USB 功能

## 6. HID 功能


该功能由 usb_f_hid.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"hid"。HID 功能在其功能目录中提供以下属性：

	=============== ===========================================
	protocol	要使用的 HID 协议
	report_desc	用于 HID report 的数据，经由 /dev/hidg<X>
			传入的数据除	report_length	HID report 长度
	subclass	要使用的 HID 子类
	=============== ===========================================

对于键盘，protocol subclass 1，report_length 8```
  $ hd my_report_desc
  00000000  05 01 09 06 a1 01 05 07  19 e0 29 e7 15 00 25 01  |..........)...%.|
  00000010  75 01 95 08 81 02 95 01  75 08 81 03 95 05 75 01  |u.......u.....u.|
  00000020  05 08 19 01 29 05 91 02  95 01 75 03 91 03 95 06  |....).....u.....|
  00000030  75 08 15 00 25 65 05 07  19 00 29 65 81 00 c0     |u...%e....)e...|
  0000003f
```
```
  $ echo -ne \\x05\\x01\\x09\\x06\\xa1.....
```
### 测试 HID 功能


设备端：

- 创建 gadget
- gadget 连接到一个主机，最好不是用于控gadget 的那- 运行一个向 /dev/hidg<N> 写入的程序，例如
```
	$ ./hid_gadget_test /dev/hidg0 keyboard
```
主机端：

- 观察来自 gadget 的按
## 7. LOOPBACK 功能


该功能由 usb_f_ss_lb.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"Loopback"。LOOPBACK 功能在其功能目录中提供以下属性：

	=============== =======================
	qlen		回环队列的深	bulk_buflen	缓冲区长	=============== =======================

### 测试 LOOPBACK 功能


设备端：运行 gadget

主机端：test-usb（tools/usb/testusb.c
## 8. MASS STORAGE 功能


该功能由 usb_f_mass_storage.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"mass_storage"。MASS STORAGE 功能在其目录中提供以属性，文件
	=============== ==============================================
	stall		设为允许功能暂停批量端点			在某些已知无法正常工作的 USB 设备上会被禁用			你应该将其设true	num_buffers	流水线缓冲区的数量。有效数值为
			2..4。仅当设置了 CONFIG_USB_GADGET_DEBUG_FILES
			时可用	=============== ==============================================

以及一个对应于 SCSI LUN #0 的默lun.0 目录
```
	$ mkdir functions/mass_storage.0/partition.5
```
LUN 编号不必连续，除了默认创建的 lun #0 之外。最多可指定 8 lun，且都必须遵<name>.<number> 的命名方式。编号可以是 0..8。一个不错的约定是将 lun 命名"lun.<number>"，尽管这并非强制
在每lun 目录中有以下属性文件：

	=============== ==============================================
	file		LUN 后端文件的路径。如LUN 未被标记为可移除			则为必需	ro		指定对该 LUN 的访问应为只读的标志。当启用			CD-ROM 模拟，以及当无法R/W 模式打开 "filename"
			时，隐含此标志	removable	指定LUN 应被指示为可移除的标志	cdrom		指定LUN 应被报告CD-ROM 的标志	nofua		指定 SCSI WRITE(10,12) FUA 标志的标	forced_eject	这个只写文件仅在功能处于活动状态时才有用。它会导			后端文件被强制从 LUN 分离，无论主机是否允许			写入任意非零字节数都将导致弹出	=============== ==============================================

### 测试 MASS STORAGE 功能


设备端：连接 gadget，启用它
主机端：dmesg，观USB 驱动器出现（如果系统配置为自动挂载）

## 9. MIDI 功能


该功能由 usb_f_midi.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"midi"。MIDI 功能在其功能目录中提供以下属性：

	================ ====================================
	buflen		 MIDI 缓冲区长	id		 USB MIDI 适配器的 ID 字符	in_ports	  MIDI 输入端口	index		 USB MIDI 适配器的索引	out_ports	 MIDI 输出端口	qlen		 USB 读请求队列长	interface_string USB AudioControl 接口字符	================ ====================================

### 测试 MIDI 功能


有两种情形：gadget 向主机播mid，以及从主机gadget 播放 mid
1) gadget 向主机播mid
```
  $ arecordmidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1
  $ arecordmidi -p 24:0 from_gadget.mid
```
```
  $ aplaymidi -l
   Port    Client name                      Port name
   20:0    f_midi                           f_midi

  $ aplaymidi -p 20:0 to_host.mid
```
2) 从主机向 gadget 播放 mid

```
  $ arecordmidi -l
   Port    Client name                      Port name
   20:0    f_midi                           f_midi

  $ arecordmidi -p 20:0 from_host.mid
```
```
  $ aplaymidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1

  $ aplaymidi -p24:0 to_gadget.mid
```
from_gadget.mid 听起来应to_host.mid 完全相同
from_host.id 听起来应to_gadget.mid 完全相同
```
  $ aplaymidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1
  128:0    TiMidity                         TiMidity port 0
  128:1    TiMidity                         TiMidity port 1
  128:2    TiMidity                         TiMidity port 2
  128:3    TiMidity                         TiMidity port 3

  $ aplaymidi -p 128:0 file.mid
```
```
  $ aconnect 24:0 128:0 # try it on the host
```
gadget MIDI 端口连接timidity MIDI 端口后，gadget 端用 aplaymidi -l
播放的任何内容都可以在主机的扬声耳机中听到
## 10. NCM 功能


该功能由 usb_f_ncm.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"ncm"。NCM 功能在其功能目录中提供以下属性：

	======================= ==================================================
	ifname			与本功能实例关联的网络设备接口名
	qmult			高速与超速下的队列长度乘	host_addr		USB 以太链路上主机一侧的 MAC 地址
	dev_addr		USB 以太链路上设备一侧的 MAC 地址
	max_segment_size	P2P 连接所需的段大小。这将把 MTU 设为 14 字节
	======================= ==================================================

在创functions/ncm.<实例 之后，它们包含默认值：qmult 5，dev_addr host_addr 为随机选择。如果功能未绑定，ifname 可被写入。写入内容必须是一个接口模式，
例如 "usb%d"，这将导致网络核心选择下一个空闲的 usbX 接口。默认情况下它被设为 "usb%d"
### 测试 NCM 功能


配置设备与主机的 IP 地址。然后：

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 11. OBEX 功能


该功能由 usb_f_obex.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"obex"。OBEX 功能在其功能目录中只提供一个属性：

	port_num

该属性是只读的
系统中最多可以有 4 ACM/通用串行/OBEX 端口
### 测试 OBEX 功能


```
	seriald -f /dev/ttyGS<Y> -s 1024
```
```
	serialc -v <vendorID> -p <productID> -i<interface#> -a1 -s1024 \
                -t<out endpoint addr> -r<in endpoint addr>
```
其中 seriald serialc Felipe 的工具，可在以下位置找到
	https://github.com/felipebalbi/usb-tools.git master

## 12. PHONET 功能


该功能由 usb_f_phonet.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"phonet"。PHONET 功能在其功能目录中只提供一个属性：

	=============== ==================================================
	ifname		与本功能实例关联的网络设备接口名
	=============== ==================================================

### 测试 PHONET 功能


没有特定的硬件无法测SOCK_STREAM 协议，因此只测试SOCK_DGRAM。要使后者工作，
过去我不得不应用这里提到的补丁：

http://www.spinics.net/lists/linux-usb/msg85689.html

需要这些工具：

git://git.gitorious.org/meego-cellular/phonet-utils.git

```
	$ ./phonet -a 0x10 -i usbpn0
	$ ./pnroute add 0x6c usbpn0
	$./pnroute add 0x10 usbpn0
	$ ifconfig usbpn0 up
```
```
	$ ./phonet -a 0x6c -i upnlink0
	$ ./pnroute add 0x10 upnlink0
	$ ifconfig upnlink0 up
```
```
	http://www.spinics.net/lists/linux-usb/msg85690.html
```
```
	$ ./pnxmit -a 0x6c -r
```
```
	$ ./pnxmit -a 0x10 -s 0x6c
```
结果应有一些数据从主机发送到设备。然后反过来
```
	$ ./pnxmit -a 0x10 -r
```
```
	$ ./pnxmit -a 0x6c -s 0x10
```
## 13. RNDIS 功能


该功能由 usb_f_rndis.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"rndis"。RNDIS 功能在其功能目录中提供以下属性：

	=============== ==================================================
	ifname		与本功能实例关联的网络设备接口名
	qmult		高速与超速下的队列长度乘	host_addr	USB 以太链路上主机一侧的 MAC 地址
	dev_addr		USB 以太链路上设备一侧的 MAC 地址
	=============== ==================================================

在创functions/rndis.<实例 之后，它们包含默认值：qmult 5，dev_addr host_addr 为随机选择。如果功能未绑定，ifname 可被写入。写入内容必须是一个接口模式，
例如 "usb%d"，这将导致网络核心选择下一个空闲的 usbX 接口。默认情况下它被设为 "usb%d"
### 测试 RNDIS 功能


配置设备与主机的 IP 地址。然后：

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 14. SERIAL 功能


该功能由 usb_f_gser.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"gser"。SERIAL 功能在其功能目录中只提供一个属性：

	port_num

该属性是只读的
系统中最多可以有 4 ACM/通用串行/OBEX 端口
### 测试 SERIAL 功能


```
	insmod usbserial
	echo VID PID >/sys/bus/usb-serial/drivers/generic/new_id
```
```
	cat > /dev/ttyUSB<X>
```
```
	cat /dev/ttyGS<Y>
```
鐒跺悗鍙嶈繃鏉?
```
	cat > /dev/ttyGS<Y>
```
```
	cat /dev/ttyUSB<X>
```
## 15. SOURCESINK 功能


该功能由 usb_f_ss_lb.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"SourceSink"。SOURCESINK 功能在其功能目录中提供以属性：

	=============== ==================================
	pattern		0（全零）（mod63）（无	isoc_interval	1..16
	isoc_maxpacket	0 - 1023（fs） - 1024（hs/ss	isoc_mult	0..2（仅 hs/ss	isoc_maxburst	0..15（仅 ss	bulk_buflen	缓冲区长	bulk_maxburst	0..15（仅 ss	bulk_qlen	批量队列深度
	iso_qlen	等时队列深度
	=============== ==================================

### 测试 SOURCESINK 功能


设备端：运行 gadget

主机端：test-usb（tools/usb/testusb.c

## 16. UAC1 功能（旧实现

该功能由 usb_f_uac1_legacy.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"uac1_legacy"。uac1 功能在其功能目录中提供以下属性：

	=============== ====================================
	audio_buf_size	音频缓冲区大	fn_cap		采集 pcm 设备文件	fn_cntl		控制设备文件	fn_play		播放 pcm 设备文件	req_buf_size	ISO OUT 端点请求缓冲区大	req_count	ISO OUT 端点请求计数
	=============== ====================================

这些属性都有合理的默认值
### 测试 UAC1 功能


设备端：运行 gadget

```
	aplay -l # 应列出我们的 USB Audio Gadget
```
## 17. UAC2 功能


该功能由 usb_f_uac2.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"uac2"。uac2 功能在其功能目录中提供以下属性：

	================ ====================================================
	c_chmask         采集通道掩码
	c_srate          采集采样率列表（逗号分隔	c_ssize          采集采样大小（字节）
	c_sync           采集同步类型（async/adaptive	c_mute_present   采集静音控制使能
	c_volume_present 采集音量控制使能
	c_volume_min     采集音量控制最小值（单位 1/256 dB	c_volume_max     采集音量控制最大值（单位 1/256 dB	c_volume_res     采集音量控制分辨率（单位 1/256 dB	c_hs_bint        采集 HS/SS bInterval-4：固定，0：自动）
	fb_max           异步模式下的最大额外带	p_chmask         播放通道掩码
	p_srate          播放采样率列表（逗号分隔	p_ssize          播放采样大小（字节）
	p_mute_present   播放静音控制使能
	p_volume_present 播放音量控制使能
	p_volume_min     播放音量控制最小值（单位 1/256 dB	p_volume_max     播放音量控制最大值（单位 1/256 dB	p_volume_res     播放音量控制分辨率（单位 1/256 dB	p_hs_bint        播放 HS/SS bInterval-4：固定，0：自动）
	req_number       为采集与播放预分配的请求数量
	function_name    接口名称
	if_ctrl_name     拓扑控制名称
	clksrc_in_name   输入时钟名称
	clksrc_out_name  输出时钟名称
	p_it_name        播放输入终端名称
	p_it_ch_name     播放输入首通道名称
	p_ot_name        播放输出终端名称
	p_fu_vol_name    播放功能单元名称
	c_it_name        采集输入终端名称
	c_it_ch_name     采集输入首通道名称
	c_ot_name        采集输出终端名称
	c_fu_vol_name    采集功能单元名称
	c_terminal_type  采集终端类型代码
	p_terminal_type  播放终端类型代码
	================ ====================================================

这些属性都有合理的默认值
### 测试 UAC2 功能


设备端：运行 gadget
主机端：aplay -l # 应列出我们的 USB Audio Gadget

该功能不需要真实的硬件支持，它只是向主机发从主机接收一段音频数据流。为了真正在
设备端听到声音，可以使用类似如下的命```
	$ arecord -f dat -t wav -D hw:2,0 | aplay -D hw:0,0 &
```
```
	$ arecord -f dat -t wav -D hw:CARD=UAC2Gadget,DEV=0 | \
	  aplay -D default:CARD=OdroidU3
```
## 18. UVC 功能


该功能由 usb_f_uvc.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"uvc"。uvc 功能在其功能目录中提供以下属性：

	=================== ================================================
	streaming_interval  轮询端点以进行数据传输的间隔
	streaming_maxburst  超速伴随描述符中的 bMaxBurst
	streaming_maxpacket 选择此配置时该端点能够发送或接收的最大包大小
	function_name       接口名称
	=================== ================================================

还有 "control" "streaming" 两个子目录，每个都包含一定数量的子目录。提供了一合理的默认值，但用户必须提供以下内容：

	================== ====================================================
	control header     control/header 中创建，control/class/fs
			   control/class/ss 链接
	streaming header   streaming/header 中创建，			   streaming/class/fs streaming/class/hs 			   streaming/class/ss 链接
	format description streaming/mjpeg 			   streaming/uncompressed 中创	frame description  streaming/mjpeg/<format> 			   streaming/uncompressed/<format> 中创	================== ====================================================

每个帧描述都包含帧间隔规范，而每个这样的规范由若干带间隔值的行组```
  # mkdir functions/uvc.usb0/control/header/h
  # cd functions/uvc.usb0/control/
  # ln -s header/h class/fs
  # ln -s header/h class/ss
  # mkdir -p functions/uvc.usb0/streaming/uncompressed/u/360p
  # cat <<EOF > functions/uvc.usb0/streaming/uncompressed/u/360p/dwFrameInterval
  666666
  1000000
  5000000
  EOF
  # cd $GADGET_CONFIGFS_ROOT
  # mkdir functions/uvc.usb0/streaming/header/h
  # cd functions/uvc.usb0/streaming/header/h
  # ln -s ../../uncompressed/u
  # cd ../../class/fs
  # ln -s ../../header/h
  # cd ../../class/hs
  # ln -s ../../header/h
  # cd ../../class/ss
  # ln -s ../../header/h
```
### 测试 UVC 功能


```
  # uvc-gadget -u /dev/video<uvc video node #> -v /dev/video<vivid video node #>
```
其中 uvc-gadget 是这个程序：
	http://git.ideasonboard.org/uvc-gadget.git

应用这些补丁
	https://lore.kernel.org/r/1386675637-18243-1-git-send-email-r.baldyga@samsung.com/

```
	luvcview -f yuv
```
## 19. PRINTER 功能


该功能由 usb_f_printer.ko 模块提供
### 功能特定configfs 接口


创建功能目录时要使用的功能名"printer"。printer 功能在其功能目录中提供以下属性：

	==========	===========================================
	pnp_string	pnp 字符串中传递给主机的数	q_len		每个端点的请求数
	==========	===========================================

### 测试 PRINTER 功能


最基本的测试：

```
	# ls -l /devices/virtual/usb_printer_gadget/
```
应显g_printer<number>
如果 udev 处于活动状态，/dev/g_printer<number> 应自动出现
主机端：

如果 udev 处于活动状态，则例/dev/usb/lp0 应出现
主机到设备传输：

```
	# cat /dev/g_printer<number>
```
```
	# cat > /dev/usb/lp0
```
```
	# cat > /dev/g_printer<number>
```
```
	# cat /dev/usb/lp0
```
更高级的测试可以使用 Documentation/usb/gadget_printer.rst 中描述的 prn_example 进行

## 20. UAC1 功能（虚ALSA 声卡，使u_audio API

该功能由 usb_f_uac1.ko 模块提供它将创建一个虚ALSA 声卡，音频流简单地汇入/源自该声卡
### 功能特定configfs 接口


创建功能目录时要使用的功能名"uac1"。uac1 功能在其功能目录中提供以下属性：

	================ ====================================================
	c_chmask         采集通道掩码
	c_srate          采集采样率列表（逗号分隔	c_ssize          采集采样大小（字节）
	c_mute_present   采集静音控制使能
	c_volume_present 采集音量控制使能
	c_volume_min     采集音量控制最小值（单位 1/256 dB	c_volume_max     采集音量控制最大值（单位 1/256 dB	c_volume_res     采集音量控制分辨率（单位 1/256 dB	p_chmask         播放通道掩码
	p_srate          播放采样率列表（逗号分隔	p_ssize          播放采样大小（字节）
	p_mute_present   播放静音控制使能
	p_volume_present 播放音量控制使能
	p_volume_min     播放音量控制最小值（单位 1/256 dB	p_volume_max     播放音量控制最大值（单位 1/256 dB	p_volume_res     播放音量控制分辨率（单位 1/256 dB	req_number       为采集与播放预分配的请求数量
	function_name    接口名称
	p_it_name        播放输入终端名称
	p_it_ch_name     播放通道名称
	p_ot_name        播放输出终端名称
	p_fu_vol_name    播放静音/音量功能单元名称
	c_it_name        采集输入终端名称
	c_it_ch_name     采集通道名称
	c_ot_name        采集输出终端名称
	c_fu_vol_name    采集静音/音量功能单元名称
	================ ====================================================

这些属性都有合理的默认值
### 测试 UAC1 功能


设备端：运行 gadget
主机端：aplay -l # 应列出我们的 USB Audio Gadget

该功能不需要真实的硬件支持，它只是向主机发从主机接收一段音频数据流。为了真正在
设备端听到声音，可以使用类似如下的命```
	$ arecord -f dat -t wav -D hw:2,0 | aplay -D hw:0,0 &
```
```
	$ arecord -f dat -t wav -D hw:CARD=UAC1Gadget,DEV=0 | \
	  aplay -D default:CARD=OdroidU3
```
## 21. MIDI2 功能


该功能由 usb_f_midi2.ko 模块提供它将创建一个包UMP rawmidi 设备的虚ALSA 声卡，其UMP 包被回环。此外，还会
创建一个传统的 rawmidi 设备。UMP rawmidi 也与 ALSA sequencer 客户端绑定
### 功能特定configfs 接口


创建功能目录时要使用的功能名"midi2"。midi2 功能在其功能目录中提供以下作为声顶层信息的属性：

	=============	=================================================
	process_ump	用于处理 UMP Stream 消息的布尔标志（0 1	static_block	用于静态块的布尔标志（0 1	iface_name	可选的接口名称字符	=============	=================================================

该目录包含一"ep.0" 子目录，它提UMP Endpoint（一USB MIDI 端点）的属性：

	=============	=================================================
	protocol_caps	MIDI 协议能力			1：MIDI 1.0：MIDI 2.0，或 3：两种协	protocol	默认 MIDI 协议 2	ep_name		UMP Endpoint 名称字符	product_id	产品 ID 字符	manufacturer	制造商 ID 号（24 位）
	family		设备系列 ID 号（16 位）
	model		设备型号 ID 号（16 位）
	sw_revision	软件版本2 位）
	=============	=================================================

每个 Endpoint 子目录包含一"block.0" 子目录，它代Block 0 信息Function Block其属性为
	=================	===============================================
	name			Function Block 名称字符	direction		FB 的方				1：输入，2：输出，3：双	first_group		首个 UMP Group 编号-15	num_groups		FB 中的 group 数量-16	midi1_first_group	MIDI 1.0 的首UMP Group 编号-15	midi1_num_groups	MIDI 1.0 group 数量-16	ui_hint			FB UI 提示
				0：未知，1：接收方：发送方：两	midi_ci_version		支持MIDI-CI 版本号（8 位）
	is_midi1		传统 MIDI 1.0 设备-2				0：MIDI 2.0 设备				1：无限制MIDI 1.0，或
				2：低速的 MIDI 1.0
	sysex8_streams		SysEx8 流的最大数量（8 位）
	active			指示 FB 活动状态的布尔标志 1	=================	===============================================

如果需要多Function Block，可以通过创建带相Function Block 编号、……）"block.<num>" 子目录来添加更多 Function Block。FB 子目录也可以动态移除。注Function
Block 编号必须是连续的
类似地，如果需要多UMP Endpoint，可以通过创建 "ep.<num>" 子目录来添加更多 Endpoint编号必须是连续的
为了模拟不支UMP v1.1 的旧 MIDI 2.0 设备，将 0 传给 `process_ump` 标志。这样整UMP v1.1 请求都会被忽略
### 测试 MIDI2 功能


```
  $ cat /proc/asound/cards
```
将显示一个包MIDI2 设备的新声卡
```
  $ cat /proc/asound/cards
```
将显示一个包MIDI1 MIDI2 设备的新声卡，取决于 USB 音频驱动的配置
在两者上，当主机启用ALSA sequencer 时，你可以找到诸"MIDI 2.0 Gadget" 这样UMP MIDI 客户端
由于驱动只是回环数据，测试时不需要真实设备
为了测试gadget 到主机的 MIDI 输入（例如模MIDI 键盘），你可以发送如下的 MIDI
流
```
  $ aconnect -o
  ....
  client 20: 'MIDI 2.0 Gadget' [type=kernel,card=1]
      0 'MIDI 2.0        '
      1 'Group 1 (MIDI 2.0 Gadget I/O)'
  $ aplaymidi -p 20:1 to_host.mid
```
```
  $ aconnect -i
  ....
  client 24: 'MIDI 2.0 Gadget' [type=kernel,card=2]
      0 'MIDI 2.0        '
      1 'Group 1 (MIDI 2.0 Gadget I/O)'
  $ arecordmidi -p 24:1 from_gadget.mid
```
如果你有支持 UMP 的应用程序，也可以使UMP 端口来发接收原始 UMP 包。例aseqdump
程序
```
  $ aseqdump -u 2 -p 24:1
  Waiting for data. Press Ctrl+C to end.
  Source  Group    Event                  Ch  Data
   24:1   Group  0, Program change          0, program 0, Bank select 0:0
   24:1   Group  0, Channel pressure        0, value 0x80000000
```
为了测试gadget MIDI 输出（例如模MIDI 合成器），只需反过来即可
```
  $ arecordmidi -p 20:1 from_host.mid
```
```
  $ aplaymidi -p 24:1 to_gadget.mid
```
主机端对 altset 0 MIDI 1.0 的访问受支持，并且它会在 gadget 上被转换UMP 包。它
只绑定到 Function Block 0
当前的操作模式可以在 ALSA 控制元素中观察到
```
  $ amixer -c1 contents
  numid=1,iface=RAWMIDI,name='Operation Mode'
    ; type=INTEGER,access=r--v----,values=1,min=0,max=2,step=0
    : values=2
```
其中 0 = 未使用，1 = MIDI 1.0（altset 0），2 = MIDI 2.0（altset 1）。上面的例子显示
它运行在 2，即 MIDI 2.0