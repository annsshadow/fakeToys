## ARECA 固件规范（FIRMWARE SPEC

本文档规Areca 基于 IOP331 RAID 控制器固件接口规范，涵盖消息机制、Doorbell/RS-232 仿真、PostQ 队列及对应的命令码与数据结构，供 RAID 驱动开发者实现主机与控制器通信时参考

本文档描Areca RAID 控制器（基于 IOP331）的固件接口规范，涵盖消息机制、RS-232 仿真、PostQ 队列，以及用RAID 管理RS-232 命令码与数据结构

## IOP331 适配器的使用


（所有输输出均从 IOP331 的视角出发）

### 1. Message 0


- InitThread 消息与返回码

### 2. Doorbell 用于 RS-232 仿真


inDoorBell
    bit0
	数据输入就绪
	（驱动数据写入完成）
    bit1
	数据输出已读
	（驱动数据读取完成）

outDooeBell:
    bit0
	数据输出就绪
	（IOP331 数据写入完成
    bit1
	数据输入已读
	（IOP331 数据读取完成

### 3. 索引内存使用


============   ==========================================
offset 0xf00   用于 RS232 输出（请求缓冲区
offset 0xe00   用于 RS232 输入（临时缓冲区
offset 0xa00   用于入站消息message_rwbuffer
	       （驱动发送给 IOP331
offset 0xa00   用于出站消息message_rwbuffer
	       （IOP331 发送给驱动
============   ==========================================

### 4. RS-232 仿真


当前使用 128 字节缓冲区：

============   =====================
1st uint32_t   数据长度--124
Byte 4--127    最124 字节数据
============   =====================

### 5. PostQ


所SCSI 命令都必须通过 postQ 发送：

    （入站队列端口）
	请求帧必32 字节对齐

	    #bit27--bit31
		用于 post ccb 的标
	    #bit0--bit26
		post arcmsr_cdb 的真实地址（bit27--bit31

		=====   ===================
		bit31   ==  ===============
			0   256 瀛楄妭甯。
			1   512 瀛楄妭甯。
			==  ===============
		bit30   ==  ==============
			0   普通请
			1   BIOS 请求
			==  ==============
		bit29   保留
		bit28   保留
		bit27   保留
		=====   ===================

    （出站队列端口）
	请求回复

	    #bit27--bit31
		回复标志
	    #bit0--bit26
		reply arcmsr_cdb 的真实地址（bit27--bit31

		    =====   =======================================================
		    bit31   必须0（对于此类回复）
		    bit30   BIOS 握手保留
		    bit29   保留
		    bit28   ==  ===================================================
			    0   无错误，忽略 AdapStatus/DevStatus/SenseData
			    1   错误，错误码位于 AdapStatus/DevStatus/SenseData
			    ==  ===================================================
		    bit27   保留
		    =====   =======================================================

### 6. BIOS 请求


所BIOS 请求与来PostQ 的请求相

例外

请求帧从配置空间发送：

	============   ==========================
	offset: 0x78   请求帧（bit30 == 1
	offset: 0x18   只写以生
		       鍚?IOP331 鐨?IRQ
	============   ==========================

```

	(bit30 == 0, bit28==err flag)

```
### 7. SGL 条目（结构体）的定义


### 8. Message1 输出 - 诊断状态码


### 9. Message0 消息


======  =================================================================
0x00    NOP
0x01    获取配置（Get Config
	->offset 0xa00 :用于出站的消息码 message_rwbuffer
	（IOP331 发送给驱动

	===================== ==========================================
	Signature             0x87974060(4)
	请求长度              0x00000200(4)
	队列数量              0x00000100(4)
	SDRAM 大小            0x00000100(4)-->256 MB
	IDE 通道              0x00000008(4)
	厂商                  40 字节字符
	型号                  8 字节字符
	固件版本              16 字节字符
	设备映射              16 字节字符
	FirmwareVersion       DWORD

         - 新增用于检
			新的固件能力
	===================== ==========================================
0x02    设置配置（Set Config
	->offset 0xa00 :用于入站的消息码 message_rwbuffer
	（驱动发送给 IOP331

	========================= ==================
	Signature                 0x87974063(4)
	请求帧的 UPPER32->仅驱
	========================= ==================
0x03    重置（中止所有已排队的命令）
0x04    停止后台活动
0x05    刷新缓存
0x06    启动后台活动
	（如果后台已停止则重新启动）
0x07    检查是否有主机命令挂起
	（Novell 可能需要此功能
0x08    设置控制器时
	->offset 0xa00   用于入站的消息码 message_rwbuffer
	（驱动到 IOP331

	======   ==================
	byte 0   0xaa <-- 签名
	byte 1   0x55 <-- 签名
	byte 2   年（04
	byte 3   月（1..12
	byte 4   日（1..31
	byte 5   时（0..23
	byte 6   分（0..59
	byte 7   秒（0..59
	======   ==================
======  =================================================================


## 用于 Areca RAID 控制器的 RS-232 接口


       底层命令接口VT100 终端互斥

### 1. 命令执行顺序


	(A) 澶。
		3 字节序列x5E, 0x01, 0x61

	(B) 鍛戒护鍧。
		包含长度
		命令码、数据和校验字节的可变长度数

	(C) 返回数据
		可变长度的数

### 2. 鍛戒护鍧。


	(A) 绗?1 瀛楄妭
		命令块长度（低字节）

	(B) 绗?2 瀛楄妭
		命令块长度（高字节）

		.. 注意:: 命令块长度不应超2040 字节
			  长度不包含这两个字节

	(C) 绗?3 瀛楄妭
		鍛戒护鐮。

	(D) 4 及后续字
		可变长度数据字节

	    取决于命令码

	(E) 最1 字节
	    校验字节（从1 字节到最后一个数据字节的和）

### 3. 命令码及相关数据


以下RAID 控制器中定义的命令码
命令0x10--0x1 用于系统级管理，
无需密码检查，并且应在独立
受控工具中实现，不供最终用户访问
命令0x20--0x 始终检查密码，
```

	enum
	{
		GUI_SET_SERIAL=0x10,
		GUI_SET_VENDOR,
		GUI_SET_MODEL,
		GUI_IDENTIFY,
		GUI_CHECK_PASSWORD,
		GUI_LOGOUT,
		GUI_HTTP,
		GUI_SET_ETHERNET_ADDR,
		GUI_SET_LOGO,
		GUI_POLL_EVENT,
		GUI_GET_EVENT,
		GUI_GET_HW_MONITOR,
		//    GUI_QUICK_CREATE=0x20, (function removed)
		GUI_GET_INFO_R=0x20,
		GUI_GET_INFO_V,
		GUI_GET_INFO_P,
		GUI_GET_INFO_S,
		GUI_CLEAR_EVENT,
		GUI_MUTE_BEEPER=0x30,
		GUI_BEEPER_SETTING,
		GUI_SET_PASSWORD,
		GUI_HOST_INTERFACE_MODE,
		GUI_REBUILD_PRIORITY,
		GUI_MAX_ATA_MODE,
		GUI_RESET_CONTROLLER,
		GUI_COM_PORT_SETTING,
		GUI_NO_OPERATION,
		GUI_DHCP_IP,
		GUI_CREATE_PASS_THROUGH=0x40,
		GUI_MODIFY_PASS_THROUGH,
		GUI_DELETE_PASS_THROUGH,
		GUI_IDENTIFY_DEVICE,
		GUI_CREATE_RAIDSET=0x50,
		GUI_DELETE_RAIDSET,
		GUI_EXPAND_RAIDSET,
		GUI_ACTIVATE_RAIDSET,
		GUI_CREATE_HOT_SPARE,
		GUI_DELETE_HOT_SPARE,
		GUI_CREATE_VOLUME=0x60,
		GUI_MODIFY_VOLUME,
		GUI_DELETE_VOLUME,
		GUI_START_CHECK_VOLUME,
		GUI_STOP_CHECK_VOLUME
	};

```
##### 命令描述


GUI_SET_SERIAL
	设置控制器序列号

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x10
	byte 3            password length (should be 0x0f)
	byte 4-0x13       should be "ArEcATecHnoLogY"
	byte 0x14--0x23   序列号字符串（必须为 16 字节
	================  =============================================

GUI_SET_VENDOR
	设置控制器的厂商字符

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x11
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x3B   厂商字符串（必须40 字节
	================  =============================================

GUI_SET_MODEL
	设置控制器的型号名称

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x12
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x1B   型号字符串（必须8 字节
	================  =============================================

GUI_IDENTIFY
	识别设备

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x13
			    return "Areca RAID Subsystem "
	================  =============================================

GUI_CHECK_PASSWORD
	验证密码

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x14
	byte 3            password length
	byte 4-0x??       user password to be checked
	================  =============================================

GUI_LOGOUT
	注销 GUI（在下一个命令时强制进行密码检查）

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x15
	================  =============================================

GUI_HTTP
	HTTP 接口（保留用HTTP 代理服务）（0x16

GUI_SET_ETHERNET_ADDR
	设置以太MAC 地址

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x17
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x19   以太MAC 地址（必须为 6 字节
	================  =============================================

GUI_SET_LOGO
	HTTP 中设置徽

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x18
	byte 3            页号/1/2/3）（0xff --> 清除 OEM 徽标
	byte 4/5/6/7      0x55/0xaa/0xa5/0x5a
	byte 8            TITLE.JPG 数据（每页必须为 2000 字节

			  .. 注意:: page0 的前 2 字节必须
				    JPG 文件的实际长
	================  =============================================

GUI_POLL_EVENT
	轮询事件日志是否更改

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x19
	================  =============================================

GUI_GET_EVENT
	读取事件

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x1a
	byte 3            事件页（0：第 1 / 1/2/3：最后一页）
	================  =============================================

GUI_GET_HW_MONITOR
	获取硬件监视器数

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x1b
	byte 3 		  风扇数量（示2
	byte 4 		  电压传感器数量（示例 3
	byte 5 		  温度传感器数量（示例 2
	byte 6 		  电源数量
	byte 7/8          风扇#0（RPM
	byte 9/10         风扇#1
	byte 11/12 	  Voltage#0 原始值（单位 `*1000`
	byte 13/14 	  Voltage#0 鍊。
	byte 15/16 	  Voltage#1 原始
	byte 17/18 	  Voltage#1
	byte 19/20 	  Voltage#2 原始
	byte 21/22 	  Voltage#2
	byte 23 	  温度#0
	byte 24 	  温度#1
	byte 25 	  电源指示 (bit0   power#0,
			  bit1   power#1)
	byte 26 	  UPS 指示
	================  =============================================

GUI_QUICK_CREATE
	快速创RAID/卷集

	================  ==============================================
	byte 0,1       	  length
	byte 2         	  command code 0x20
	byte 3/4/5/6   	  raw capacity
	byte 7 		  raid level
	byte 8 		  stripe size
	byte 9 		  spare
	byte 10/11/12/13  设备掩码（用于创raid/volume 的设备）
	================  ==============================================

    此功能已移除，应用程序若
    瑕佸疄鐜板揩閫熷垱寤哄姛鑳。

    需要使GUI_CREATE_RAIDSET GUI_CREATE_VOLUMESET 功能

GUI_GET_INFO_R
	获取 RAID 集信

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x20
	byte 3            raidset#
	================  =============================================

```

    typedef struct sGUI_RAIDSET
    {
	    BYTE grsRaidSetName[16];
	    DWORD grsCapacity;
	    DWORD grsCapacityX;
	    DWORD grsFailMask;
	    BYTE grsDevArray[32];
	    BYTE grsMemberDevices;
	    BYTE grsNewMemberDevices;
	    BYTE grsRaidState;
	    BYTE grsVolumes;
	    BYTE grsVolumeList[16];
	    BYTE grsRes1;
	    BYTE grsRes2;
	    BYTE grsRes3;
	    BYTE grsFreeSegments;
	    DWORD grsRawStripes[8];
	    DWORD grsRes4;
	    DWORD grsRes5; //     Total to 128 bytes
	    DWORD grsRes6; //     Total to 128 bytes
    } sGUI_RAIDSET, *pGUI_RAIDSET;

```
GUI_GET_INFO_V
	获取卷集信息

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x21
	byte 3            volumeset#
	================  =============================================

```

    typedef struct sGUI_VOLUMESET
    {
	    BYTE gvsVolumeName[16]; //     16
	    DWORD gvsCapacity;
	    DWORD gvsCapacityX;
	    DWORD gvsFailMask;
	    DWORD gvsStripeSize;
	    DWORD gvsNewFailMask;
	    DWORD gvsNewStripeSize;
	    DWORD gvsVolumeStatus;
	    DWORD gvsProgress; //     32
	    sSCSI_ATTR gvsScsi;
	    BYTE gvsMemberDisks;
	    BYTE gvsRaidLevel; //     8
	    BYTE gvsNewMemberDisks;
	    BYTE gvsNewRaidLevel;
	    BYTE gvsRaidSetNumber;
	    BYTE gvsRes0; //     4
	    BYTE gvsRes1[4]; //     64 bytes
    } sGUI_VOLUMESET, *pGUI_VOLUMESET;

```
GUI_GET_INFO_P
	获取物理驱动器信

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x22
	byte 3            驱动器编号（0 max-channels - 1
	================  =============================================

```

    typedef struct sGUI_PHY_DRV
    {
	    BYTE gpdModelName[40];
	    BYTE gpdSerialNumber[20];
	    BYTE gpdFirmRev[8];
	    DWORD gpdCapacity;
	    DWORD gpdCapacityX; //     Reserved for expansion
	    BYTE gpdDeviceState;
	    BYTE gpdPioMode;
	    BYTE gpdCurrentUdmaMode;
	    BYTE gpdUdmaMode;
	    BYTE gpdDriveSelect;
	    BYTE gpdRaidNumber; //     0xff if not belongs to a raid set
	    sSCSI_ATTR gpdScsi;
	    BYTE gpdReserved[40]; //     Total to 128 bytes
    } sGUI_PHY_DRV, *pGUI_PHY_DRV;

```
GUI_GET_INFO_S
	获取系统信息

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x23
	================  =============================================

```

    typedef struct sCOM_ATTR
    {
	    BYTE comBaudRate;
	    BYTE comDataBits;
	    BYTE comStopBits;
	    BYTE comParity;
	    BYTE comFlowControl;
    } sCOM_ATTR, *pCOM_ATTR;
    typedef struct sSYSTEM_INFO
    {
	    BYTE gsiVendorName[40];
	    BYTE gsiSerialNumber[16];
	    BYTE gsiFirmVersion[16];
	    BYTE gsiBootVersion[16];
	    BYTE gsiMbVersion[16];
	    BYTE gsiModelName[8];
	    BYTE gsiLocalIp[4];
	    BYTE gsiCurrentIp[4];
	    DWORD gsiTimeTick;
	    DWORD gsiCpuSpeed;
	    DWORD gsiICache;
	    DWORD gsiDCache;
	    DWORD gsiScache;
	    DWORD gsiMemorySize;
	    DWORD gsiMemorySpeed;
	    DWORD gsiEvents;
	    BYTE gsiMacAddress[6];
	    BYTE gsiDhcp;
	    BYTE gsiBeeper;
	    BYTE gsiChannelUsage;
	    BYTE gsiMaxAtaMode;
	    BYTE gsiSdramEcc; //     1:if ECC enabled
	    BYTE gsiRebuildPriority;
	    sCOM_ATTR gsiComA; //     5 bytes
	    sCOM_ATTR gsiComB; //     5 bytes
	    BYTE gsiIdeChannels;
	    BYTE gsiScsiHostChannels;
	    BYTE gsiIdeHostChannels;
	    BYTE gsiMaxVolumeSet;
	    BYTE gsiMaxRaidSet;
	    BYTE gsiEtherPort; //     1:if ether net port supported
	    BYTE gsiRaid6Engine; //     1:Raid6 engine supported
	    BYTE gsiRes[75];
    } sSYSTEM_INFO, *pSYSTEM_INFO;

```
GUI_CLEAR_EVENT
	清除系统事件

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x24
	================  =============================================

GUI_MUTE_BEEPER
	闈欓煶褰撳墠铚傞福鍣。

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x30
	================  =============================================

GUI_BEEPER_SETTING
	绂佺敤铚傞福鍣。

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x31
	byte 3            0->禁用, 1->启用
	================  =============================================

GUI_SET_PASSWORD
	更改密码

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x32
	byte 3 		  密码长度（必<= 15
	byte 4 		  密码（必须为字母数字
	================  =============================================

GUI_HOST_INTERFACE_MODE
	设置主机接口模式

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x33
	byte 3 		  0->独立模式, 1->集群模式
	================  =============================================

GUI_REBUILD_PRIORITY
	设置重建优先

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x34
	byte 3 		  0/1/2/3（低->高）
	================  =============================================

GUI_MAX_ATA_MODE
	设置要使用的最ATA 模式

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x35
	byte 3 		  0/1/2/3锛?33/100/66/33锛。
	================  =============================================

GUI_RESET_CONTROLLER
	閲嶇疆鎺у埗鍣。

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x36
     - VT100 屏幕响应（丢弃它
	================  =============================================

GUI_COM_PORT_SETTING
	COM 端口设置

	================  =================================================
	byte 0,1	  length
	byte 2 		  command code 0x37
	byte 3 		  0->COMA（终端端口）,
			  1->COMB（调试端口）
	byte 4 		  0/1/2/3/4/5/6/7
			  (1200/2400/4800/9600/19200/38400/57600/115200)
	byte 5 		  数据
			  (0:7 bit, 1:8 bit   must be 8 bit)
	byte 6 		  停止位（0:1, 1:2 停止位）
	byte 7 		  校验位（0: 1: 2:偶校验）
	byte 8 		  flow control
			  (0: 1:xon/xoff, 2:硬件 => 必须使用
	================  =================================================

GUI_NO_OPERATION
	无操

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x38
	================  =============================================

GUI_DHCP_IP
	设置 DHCP 选项和本IP 地址

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x39
	byte 3            0:dhcp 禁用, 1:dhcp 启用
	byte 4/5/6/7      IP 地址
	================  =============================================

GUI_CREATE_PASS_THROUGH
	创建直通磁

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x40
	byte 3 		  device #
	byte 4 		  scsi 通道/1
	byte 5 		  scsi id锛?-->15锛。
	byte 6 		  scsi lun锛?-->7锛。
	byte 7 		  标记队列   启用
	byte 8 		  缓存模式   启用
	byte 9 		  最大速度/1/2/3/4
			  scsi 下为 async/20/40/80/160
			  （ide 下为 0/1/2/3/43/66/100/133/150  
	================  =============================================

GUI_MODIFY_PASS_THROUGH
	修改直通磁

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x41
	byte 3 		  device #
	byte 4 		  scsi 通道/1
	byte 5 		  scsi id锛?-->15锛。
	byte 6 		  scsi lun锛?-->7锛。
	byte 7 		  标记队列   启用
	byte 8 		  缓存模式   启用
	byte 9 		  最大速度/1/2/3/4
			  scsi 下为 async/20/40/80/160
			  （ide 下为 0/1/2/3/43/66/100/133/150  
	================  =============================================

GUI_DELETE_PASS_THROUGH
	删除直通磁

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x42
	byte 3            待删除的设备编号
	================  =============================================

GUI_IDENTIFY_DEVICE
	识别设备

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x43
	byte 3            Flash 方式
			  :选择 flash, 1:未选择 flash
	byte 4/5/6/7      flash IDE 设备掩码
			  .. 注意:: 无可用响应数
	================  =============================================

GUI_CREATE_RAIDSET
	创建 RAID 

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x50
	byte 3/4/5/6      device mask
	byte 7-22         raidset 名称（若 byte 7 == 0：使用默认值）
	================  =============================================

GUI_DELETE_RAIDSET
	删除 RAID 

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x51
	byte 3            raidset#
	================  =============================================

GUI_EXPAND_RAIDSET
	鎵╁睍 RAID 闆。

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x52
	byte 3            raidset#
	byte 4/5/6/7      device mask for expansion
	byte 8/9/10       (8:0 无变 1 变化, 0xff:终止,
			  9:raid 级别,
			  10:新条带大
			  0/1/2/3/4/5->4/8/16/32/64/128K )
	byte 11/12/13     raidset 中的每个 volume 重复
	================  =============================================

GUI_ACTIVATE_RAIDSET
	激活不完整RAID 

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x53
	byte 3            raidset#
	================  =============================================

GUI_CREATE_HOT_SPARE
	创建热备

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x54
	byte 3/4/5/6      用于创建热备盘的设备掩码
	================  =============================================

GUI_DELETE_HOT_SPARE
	删除热备

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x55
	byte 3/4/5/6      用于删除热备盘的设备掩码
	================  =============================================

GUI_CREATE_VOLUME
	创建卷集

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x60
	byte 3            raidset#
	byte 4-19         卷集名称
			  (if byte4 == 0, use default)
	byte 20-27        volume capacity (blocks)
	byte 28 	  raid level
	byte 29 	  stripe size
			  (0/1/2/3/4/5->4/8/16/32/64/128K)
	byte 30 	  channel
	byte 31 	  ID
	byte 32 	  LUN
	byte 33 	  1 启用标记
	byte 34 	  1 启用缓存
	byte 35 	  speed
			  (0/1/2/3/4->async/20/40/80/160 for scsi)
			  (0/1/2/3/4->33/66/100/133/150 for IDE  )
	byte 36 	  1 to select quick init
	================  =============================================

GUI_MODIFY_VOLUME
	修改卷集

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x61
	byte 3            volumeset#
	byte 4-19         新卷集名
			  (if byte4 == 0, not change)
	byte 20-27        新卷容量（保留）
	byte 28 	  new raid level
	byte 29 	  new stripe size
			  (0/1/2/3/4/5->4/8/16/32/64/128K)
	byte 30 	  new channel
	byte 31 	  new ID
	byte 32 	  new LUN
	byte 33 	  1 启用标记
	byte 34 	  1 启用缓存
	byte 35 	  speed
			  (0/1/2/3/4->async/20/40/80/160 for scsi)
			  (0/1/2/3/4->33/66/100/133/150 for IDE  )
	================  =============================================

GUI_DELETE_VOLUME
	删除卷集

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x62
	byte 3            volumeset#
	================  =============================================

GUI_START_CHECK_VOLUME
	启动卷一致性检

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x63
	byte 3            volumeset#
	================  =============================================

GUI_STOP_CHECK_VOLUME
	停止卷一致性检

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x64
	================  =============================================

### 4. 返回的数


(A) Header
    3 字节序列x5E, 0x01, 0x61
(B) 长度
    2 字节
    （低字节在前，不包含长度和校验字节）
(C)
    状态或数据

```

		#define GUI_OK                    0x41
		#define GUI_RAIDSET_NOT_NORMAL    0x42
		#define GUI_VOLUMESET_NOT_NORMAL  0x43
		#define GUI_NO_RAIDSET            0x44
		#define GUI_NO_VOLUMESET          0x45
		#define GUI_NO_PHYSICAL_DRIVE     0x46
		#define GUI_PARAMETER_ERROR       0x47
		#define GUI_UNSUPPORTED_COMMAND   0x48
		#define GUI_DISK_CONFIG_CHANGED   0x49
		#define GUI_INVALID_PASSWORD      0x4a
		#define GUI_NO_DISK_SPACE         0x4b
		#define GUI_CHECKSUM_ERROR        0x4c
		#define GUI_PASSWORD_REQUIRED     0x4d

	2) 如果长度 > 1

		从控制器返回的数据块
		鍏跺唴瀹瑰彇鍐充簬鍛戒护鐮?

```
(E) 鏍￠獙鍜。
    长度和状态或数据字节的校验和
