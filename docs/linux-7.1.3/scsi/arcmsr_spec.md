## ARECA 鍥轰欢瑙勮寖锛團IRMWARE SPEC锛?

鏈枃妗ｈ瀹?Areca 鍩轰簬 IOP331 鐨?RAID 鎺у埗鍣ㄥ浐浠舵帴鍙ｈ鑼冿紝娑电洊娑堟伅鏈哄埗銆丏oorbell/RS-232 浠跨湡銆丳ostQ 闃熷垪鍙婂搴旂殑鍛戒护鐮佷笌鏁版嵁缁撴瀯锛屼緵 RAID 椹卞姩寮€鍙戣€呭疄鐜颁富鏈轰笌鎺у埗鍣ㄩ€氫俊鏃跺弬鑰冦€?

鏈枃妗ｆ弿杩?Areca RAID 鎺у埗鍣紙鍩轰簬 IOP331锛夌殑鍥轰欢鎺ュ彛瑙勮寖锛屾兜鐩栨秷鎭満鍒躲€丷S-232 浠跨湡銆丳ostQ 闃熷垪锛屼互鍙婄敤浜?RAID 绠＄悊鐨?RS-232 鍛戒护鐮佷笌鏁版嵁缁撴瀯銆?

## IOP331 閫傞厤鍣ㄧ殑浣跨敤


锛堟墍鏈夎緭鍏?杈撳嚭鍧囦粠 IOP331 鐨勮瑙掑嚭鍙戯級

### 1. Message 0


- InitThread 娑堟伅涓庤繑鍥炵爜

### 2. Doorbell 鐢ㄤ簬 RS-232 浠跨湡


inDoorBell
    bit0
	鏁版嵁杈撳叆灏辩华
	锛堥┍鍔ㄦ暟鎹啓鍏ュ畬鎴愶級
    bit1
	鏁版嵁杈撳嚭宸茶
	锛堥┍鍔ㄦ暟鎹鍙栧畬鎴愶級

outDooeBell:
    bit0
	鏁版嵁杈撳嚭灏辩华
	锛圛OP331 鏁版嵁鍐欏叆瀹屾垚锛?
    bit1
	鏁版嵁杈撳叆宸茶
	锛圛OP331 鏁版嵁璇诲彇瀹屾垚锛?

### 3. 绱㈠紩鍐呭瓨浣跨敤


============   ==========================================
offset 0xf00   鐢ㄤ簬 RS232 杈撳嚭锛堣姹傜紦鍐插尯锛?
offset 0xe00   鐢ㄤ簬 RS232 杈撳叆锛堜复鏃剁紦鍐插尯锛?
offset 0xa00   鐢ㄤ簬鍏ョ珯娑堟伅鐮?message_rwbuffer
	       锛堥┍鍔ㄥ彂閫佺粰 IOP331锛?
offset 0xa00   鐢ㄤ簬鍑虹珯娑堟伅鐮?message_rwbuffer
	       锛圛OP331 鍙戦€佺粰椹卞姩锛?
============   ==========================================

### 4. RS-232 浠跨湡


褰撳墠浣跨敤 128 瀛楄妭缂撳啿鍖猴細

============   =====================
1st uint32_t   鏁版嵁闀垮害锛?--124锛?
Byte 4--127    鏈€澶?124 瀛楄妭鏁版嵁
============   =====================

### 5. PostQ


鎵€鏈?SCSI 鍛戒护閮藉繀椤婚€氳繃 postQ 鍙戦€侊細

    锛堝叆绔欓槦鍒楃鍙ｏ級
	璇锋眰甯у繀椤?32 瀛楄妭瀵归綈锛?

	    #bit27--bit31
		鐢ㄤ簬 post ccb 鐨勬爣蹇?
	    #bit0--bit26
		post arcmsr_cdb 鐨勭湡瀹炲湴鍧€锛坆it27--bit31锛?

		=====   ===================
		bit31   ==  ===============
			0   256 瀛楄妭甯?
			1   512 瀛楄妭甯?
			==  ===============
		bit30   ==  ==============
			0   鏅€氳姹?
			1   BIOS 璇锋眰
			==  ==============
		bit29   淇濈暀
		bit28   淇濈暀
		bit27   淇濈暀
		=====   ===================

    锛堝嚭绔欓槦鍒楃鍙ｏ級
	璇锋眰鍥炲锛?

	    #bit27--bit31
		鍥炲鏍囧織
	    #bit0--bit26
		reply arcmsr_cdb 鐨勭湡瀹炲湴鍧€锛坆it27--bit31锛?

		    =====   =======================================================
		    bit31   蹇呴』涓?0锛堝浜庢绫诲洖澶嶏級
		    bit30   涓?BIOS 鎻℃墜淇濈暀
		    bit29   淇濈暀
		    bit28   ==  ===================================================
			    0   鏃犻敊璇紝蹇界暐 AdapStatus/DevStatus/SenseData
			    1   閿欒锛岄敊璇爜浣嶄簬 AdapStatus/DevStatus/SenseData
			    ==  ===================================================
		    bit27   淇濈暀
		    =====   =======================================================

### 6. BIOS 璇锋眰


鎵€鏈?BIOS 璇锋眰涓庢潵鑷?PostQ 鐨勮姹傜浉鍚?

渚嬪锛?

璇锋眰甯т粠閰嶇疆绌洪棿鍙戦€侊細

	============   ==========================
	offset: 0x78   璇锋眰甯э紙bit30 == 1锛?
	offset: 0x18   鍙啓浠ョ敓鎴?
		       鍚?IOP331 鐨?IRQ
	============   ==========================

```

	(bit30 == 0, bit28==err flag)

```
### 7. SGL 鏉＄洰锛堢粨鏋勪綋锛夌殑瀹氫箟


### 8. Message1 杈撳嚭 - 璇婃柇鐘舵€佺爜锛????锛?


### 9. Message0 娑堟伅鐮?


======  =================================================================
0x00    NOP
0x01    鑾峰彇閰嶇疆锛圙et Config锛?
	->offset 0xa00 :鐢ㄤ簬鍑虹珯鐨勬秷鎭爜 message_rwbuffer
	锛圛OP331 鍙戦€佺粰椹卞姩锛?

	===================== ==========================================
	Signature             0x87974060(4)
	璇锋眰闀垮害              0x00000200(4)
	闃熷垪鏁伴噺              0x00000100(4)
	SDRAM 澶у皬            0x00000100(4)-->256 MB
	IDE 閫氶亾              0x00000008(4)
	鍘傚晢                  40 瀛楄妭瀛楃
	鍨嬪彿                  8 瀛楄妭瀛楃
	鍥轰欢鐗堟湰              16 瀛楄妭瀛楃
	璁惧鏄犲皠              16 瀛楄妭瀛楃
	FirmwareVersion       DWORD

         - 鏂板鐢ㄤ簬妫€鏌?
			鏂扮殑鍥轰欢鑳藉姏
	===================== ==========================================
0x02    璁剧疆閰嶇疆锛圫et Config锛?
	->offset 0xa00 :鐢ㄤ簬鍏ョ珯鐨勬秷鎭爜 message_rwbuffer
	锛堥┍鍔ㄥ彂閫佺粰 IOP331锛?

	========================= ==================
	Signature                 0x87974063(4)
	璇锋眰甯х殑 UPPER32锛?锛?->浠呴┍鍔?
	========================= ==================
0x03    閲嶇疆锛堜腑姝㈡墍鏈夊凡鎺掗槦鐨勫懡浠わ級
0x04    鍋滄鍚庡彴娲诲姩
0x05    鍒锋柊缂撳瓨
0x06    鍚姩鍚庡彴娲诲姩
	锛堝鏋滃悗鍙板凡鍋滄鍒欓噸鏂板惎鍔級
0x07    妫€鏌ユ槸鍚︽湁涓绘満鍛戒护鎸傝捣
	锛圢ovell 鍙兘闇€瑕佹鍔熻兘锛?
0x08    璁剧疆鎺у埗鍣ㄦ椂闂?
	->offset 0xa00   鐢ㄤ簬鍏ョ珯鐨勬秷鎭爜 message_rwbuffer
	锛堥┍鍔ㄥ埌 IOP331锛?

	======   ==================
	byte 0   0xaa <-- 绛惧悕
	byte 1   0x55 <-- 绛惧悕
	byte 2   骞达紙04锛?
	byte 3   鏈堬紙1..12锛?
	byte 4   鏃ワ紙1..31锛?
	byte 5   鏃讹紙0..23锛?
	byte 6   鍒嗭紙0..59锛?
	byte 7   绉掞紙0..59锛?
	======   ==================
======  =================================================================


## 鐢ㄤ簬 Areca RAID 鎺у埗鍣ㄧ殑 RS-232 鎺ュ彛


       搴曞眰鍛戒护鎺ュ彛涓?VT100 缁堢浜掓枼

### 1. 鍛戒护鎵ц椤哄簭


	(A) 澶?
		3 瀛楄妭搴忓垪锛?x5E, 0x01, 0x61锛?

	(B) 鍛戒护鍧?
		鍖呭惈闀垮害銆?
		鍛戒护鐮併€佹暟鎹拰鏍￠獙瀛楄妭鐨勫彲鍙橀暱搴︽暟鎹?

	(C) 杩斿洖鏁版嵁
		鍙彉闀垮害鐨勬暟鎹?

### 2. 鍛戒护鍧?


	(A) 绗?1 瀛楄妭
		鍛戒护鍧楅暱搴︼紙浣庡瓧鑺傦級

	(B) 绗?2 瀛楄妭
		鍛戒护鍧楅暱搴︼紙楂樺瓧鑺傦級

		.. 娉ㄦ剰:: 鍛戒护鍧楅暱搴︿笉搴旇秴杩?2040 瀛楄妭锛?
			  闀垮害涓嶅寘鍚繖涓や釜瀛楄妭

	(C) 绗?3 瀛楄妭
		鍛戒护鐮?

	(D) 绗?4 鍙婂悗缁瓧鑺?
		鍙彉闀垮害鏁版嵁瀛楄妭

	    鍙栧喅浜庡懡浠ょ爜

	(E) 鏈€鍚?1 瀛楄妭
	    鏍￠獙瀛楄妭锛堜粠绗?1 瀛楄妭鍒版渶鍚庝竴涓暟鎹瓧鑺傜殑鍜岋級

### 3. 鍛戒护鐮佸強鐩稿叧鏁版嵁


浠ヤ笅鏄?RAID 鎺у埗鍣ㄤ腑瀹氫箟鐨勫懡浠ょ爜
鍛戒护鐮?0x10--0x1? 鐢ㄤ簬绯荤粺绾х鐞嗭紝
鏃犻渶瀵嗙爜妫€鏌ワ紝骞朵笖搴斿湪鐙珛鐨?
鍙楁帶宸ュ叿涓疄鐜帮紝涓嶄緵鏈€缁堢敤鎴疯闂€?
鍛戒护鐮?0x20--0x?? 濮嬬粓妫€鏌ュ瘑鐮侊紝
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
##### 鍛戒护鎻忚堪


GUI_SET_SERIAL
	璁剧疆鎺у埗鍣ㄥ簭鍒楀彿

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x10
	byte 3            password length (should be 0x0f)
	byte 4-0x13       should be "ArEcATecHnoLogY"
	byte 0x14--0x23   搴忓垪鍙峰瓧绗︿覆锛堝繀椤讳负 16 瀛楄妭锛?
	================  =============================================

GUI_SET_VENDOR
	璁剧疆鎺у埗鍣ㄧ殑鍘傚晢瀛楃涓?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x11
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x3B   鍘傚晢瀛楃涓诧紙蹇呴』涓?40 瀛楄妭锛?
	================  =============================================

GUI_SET_MODEL
	璁剧疆鎺у埗鍣ㄧ殑鍨嬪彿鍚嶇О

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x12
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x1B   鍨嬪彿瀛楃涓诧紙蹇呴』涓?8 瀛楄妭锛?
	================  =============================================

GUI_IDENTIFY
	璇嗗埆璁惧

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x13
			    return "Areca RAID Subsystem "
	================  =============================================

GUI_CHECK_PASSWORD
	楠岃瘉瀵嗙爜

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x14
	byte 3            password length
	byte 4-0x??       user password to be checked
	================  =============================================

GUI_LOGOUT
	娉ㄩ攢 GUI锛堝湪涓嬩竴涓懡浠ゆ椂寮哄埗杩涜瀵嗙爜妫€鏌ワ級

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x15
	================  =============================================

GUI_HTTP
	HTTP 鎺ュ彛锛堜繚鐣欑敤浜?HTTP 浠ｇ悊鏈嶅姟锛夛紙0x16锛?

GUI_SET_ETHERNET_ADDR
	璁剧疆浠ュお缃?MAC 鍦板潃

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x17
	byte 3            password length (should be 0x08)
	byte 4-0x13       should be "ArEcAvAr"
	byte 0x14--0x19   浠ュお缃?MAC 鍦板潃锛堝繀椤讳负 6 瀛楄妭锛?
	================  =============================================

GUI_SET_LOGO
	鍦?HTTP 涓缃窘鏍?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x18
	byte 3            椤靛彿锛?/1/2/3锛夛紙0xff --> 娓呴櫎 OEM 寰芥爣锛?
	byte 4/5/6/7      0x55/0xaa/0xa5/0x5a
	byte 8            TITLE.JPG 鏁版嵁锛堟瘡椤靛繀椤讳负 2000 瀛楄妭锛?

			  .. 娉ㄦ剰:: page0 鐨勫墠 2 瀛楄妭蹇呴』涓?
				    JPG 鏂囦欢鐨勫疄闄呴暱搴?
	================  =============================================

GUI_POLL_EVENT
	杞浜嬩欢鏃ュ織鏄惁鏇存敼

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x19
	================  =============================================

GUI_GET_EVENT
	璇诲彇浜嬩欢

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x1a
	byte 3            浜嬩欢椤碉紙0锛氱 1 椤?/ 1/2/3锛氭渶鍚庝竴椤碉級
	================  =============================================

GUI_GET_HW_MONITOR
	鑾峰彇纭欢鐩戣鍣ㄦ暟鎹?

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x1b
	byte 3 		  椋庢墖鏁伴噺锛堢ず渚?2锛?
	byte 4 		  鐢靛帇浼犳劅鍣ㄦ暟閲忥紙绀轰緥 3锛?
	byte 5 		  娓╁害浼犳劅鍣ㄦ暟閲忥紙绀轰緥 2锛?
	byte 6 		  鐢垫簮鏁伴噺
	byte 7/8          椋庢墖#0锛圧PM锛?
	byte 9/10         椋庢墖#1
	byte 11/12 	  Voltage#0 鍘熷鍊硷紙鍗曚綅 `*1000`
	byte 13/14 	  Voltage#0 鍊?
	byte 15/16 	  Voltage#1 鍘熷鍊?
	byte 17/18 	  Voltage#1
	byte 19/20 	  Voltage#2 鍘熷鍊?
	byte 21/22 	  Voltage#2
	byte 23 	  娓╁害#0
	byte 24 	  娓╁害#1
	byte 25 	  鐢垫簮鎸囩ず (bit0   power#0,
			  bit1   power#1)
	byte 26 	  UPS 鎸囩ず
	================  =============================================

GUI_QUICK_CREATE
	蹇€熷垱寤?RAID/鍗烽泦

	================  ==============================================
	byte 0,1       	  length
	byte 2         	  command code 0x20
	byte 3/4/5/6   	  raw capacity
	byte 7 		  raid level
	byte 8 		  stripe size
	byte 9 		  spare
	byte 10/11/12/13  璁惧鎺╃爜锛堢敤浜庡垱寤?raid/volume 鐨勮澶囷級
	================  ==============================================

    姝ゅ姛鑳藉凡绉婚櫎锛屽簲鐢ㄧ▼搴忚嫢
    瑕佸疄鐜板揩閫熷垱寤哄姛鑳?

    闇€瑕佷娇鐢?GUI_CREATE_RAIDSET 鍜?GUI_CREATE_VOLUMESET 鍔熻兘銆?

GUI_GET_INFO_R
	鑾峰彇 RAID 闆嗕俊鎭?

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
	鑾峰彇鍗烽泦淇℃伅

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
	鑾峰彇鐗╃悊椹卞姩鍣ㄤ俊鎭?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x22
	byte 3            椹卞姩鍣ㄧ紪鍙凤紙浠?0 鍒?max-channels - 1锛?
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
	鑾峰彇绯荤粺淇℃伅

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
	娓呴櫎绯荤粺浜嬩欢

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x24
	================  =============================================

GUI_MUTE_BEEPER
	闈欓煶褰撳墠铚傞福鍣?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x30
	================  =============================================

GUI_BEEPER_SETTING
	绂佺敤铚傞福鍣?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x31
	byte 3            0->绂佺敤, 1->鍚敤
	================  =============================================

GUI_SET_PASSWORD
	鏇存敼瀵嗙爜

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x32
	byte 3 		  瀵嗙爜闀垮害锛堝繀椤?<= 15锛?
	byte 4 		  瀵嗙爜锛堝繀椤讳负瀛楁瘝鏁板瓧锛?
	================  =============================================

GUI_HOST_INTERFACE_MODE
	璁剧疆涓绘満鎺ュ彛妯″紡

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x33
	byte 3 		  0->鐙珛妯″紡, 1->闆嗙兢妯″紡
	================  =============================================

GUI_REBUILD_PRIORITY
	璁剧疆閲嶅缓浼樺厛绾?

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x34
	byte 3 		  0/1/2/3锛堜綆->楂橈級
	================  =============================================

GUI_MAX_ATA_MODE
	璁剧疆瑕佷娇鐢ㄧ殑鏈€澶?ATA 妯″紡

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x35
	byte 3 		  0/1/2/3锛?33/100/66/33锛?
	================  =============================================

GUI_RESET_CONTROLLER
	閲嶇疆鎺у埗鍣?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x36
     - 浠?VT100 灞忓箷鍝嶅簲锛堜涪寮冨畠锛?
	================  =============================================

GUI_COM_PORT_SETTING
	COM 绔彛璁剧疆

	================  =================================================
	byte 0,1	  length
	byte 2 		  command code 0x37
	byte 3 		  0->COMA锛堢粓绔鍙ｏ級,
			  1->COMB锛堣皟璇曠鍙ｏ級
	byte 4 		  0/1/2/3/4/5/6/7
			  (1200/2400/4800/9600/19200/38400/57600/115200)
	byte 5 		  鏁版嵁浣?
			  (0:7 bit, 1:8 bit   must be 8 bit)
	byte 6 		  鍋滄浣嶏紙0:1, 1:2 鍋滄浣嶏級
	byte 7 		  鏍￠獙浣嶏紙0:鏃? 1:鍏? 2:鍋舵牎楠岋級
	byte 8 		  flow control
			  (0:鏃? 1:xon/xoff, 2:纭欢 => 蹇呴』浣跨敤鏃?
	================  =================================================

GUI_NO_OPERATION
	鏃犳搷浣?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x38
	================  =============================================

GUI_DHCP_IP
	璁剧疆 DHCP 閫夐」鍜屾湰鍦?IP 鍦板潃

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x39
	byte 3            0:dhcp 绂佺敤, 1:dhcp 鍚敤
	byte 4/5/6/7      IP 鍦板潃
	================  =============================================

GUI_CREATE_PASS_THROUGH
	鍒涘缓鐩撮€氱鐩?

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x40
	byte 3 		  device #
	byte 4 		  scsi 閫氶亾锛?/1锛?
	byte 5 		  scsi id锛?-->15锛?
	byte 6 		  scsi lun锛?-->7锛?
	byte 7 		  鏍囪闃熷垪锛?   鍚敤锛?
	byte 8 		  缂撳瓨妯″紡锛?   鍚敤锛?
	byte 9 		  鏈€澶ч€熷害锛?/1/2/3/4锛?
			  scsi 涓嬩负 async/20/40/80/160锛?
			  锛坕de 涓嬩负 0/1/2/3/4锛?3/66/100/133/150  锛?
	================  =============================================

GUI_MODIFY_PASS_THROUGH
	淇敼鐩撮€氱鐩?

	================  =============================================
	byte 0,1          length
	byte 2 		  command code 0x41
	byte 3 		  device #
	byte 4 		  scsi 閫氶亾锛?/1锛?
	byte 5 		  scsi id锛?-->15锛?
	byte 6 		  scsi lun锛?-->7锛?
	byte 7 		  鏍囪闃熷垪锛?   鍚敤锛?
	byte 8 		  缂撳瓨妯″紡锛?   鍚敤锛?
	byte 9 		  鏈€澶ч€熷害锛?/1/2/3/4锛?
			  scsi 涓嬩负 async/20/40/80/160锛?
			  锛坕de 涓嬩负 0/1/2/3/4锛?3/66/100/133/150  锛?
	================  =============================================

GUI_DELETE_PASS_THROUGH
	鍒犻櫎鐩撮€氱鐩?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x42
	byte 3            寰呭垹闄ょ殑璁惧缂栧彿
	================  =============================================

GUI_IDENTIFY_DEVICE
	璇嗗埆璁惧

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x43
	byte 3            Flash 鏂瑰紡
			  锛?:閫夋嫨 flash, 1:鏈€夋嫨 flash锛?
	byte 4/5/6/7      寰?flash 鐨?IDE 璁惧鎺╃爜
			  .. 娉ㄦ剰:: 鏃犲彲鐢ㄥ搷搴旀暟鎹?
	================  =============================================

GUI_CREATE_RAIDSET
	鍒涘缓 RAID 闆?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x50
	byte 3/4/5/6      device mask
	byte 7-22         raidset 鍚嶇О锛堣嫢 byte 7 == 0锛氫娇鐢ㄩ粯璁ゅ€硷級
	================  =============================================

GUI_DELETE_RAIDSET
	鍒犻櫎 RAID 闆?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x51
	byte 3            raidset#
	================  =============================================

GUI_EXPAND_RAIDSET
	鎵╁睍 RAID 闆?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x52
	byte 3            raidset#
	byte 4/5/6/7      device mask for expansion
	byte 8/9/10       (8:0 鏃犲彉鍖? 1 鍙樺寲, 0xff:缁堟,
			  9:鏂?raid 绾у埆,
			  10:鏂版潯甯﹀ぇ灏?
			  0/1/2/3/4/5->4/8/16/32/64/128K )
	byte 11/12/13     瀵?raidset 涓殑姣忎釜 volume 閲嶅
	================  =============================================

GUI_ACTIVATE_RAIDSET
	婵€娲讳笉瀹屾暣鐨?RAID 闆?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x53
	byte 3            raidset#
	================  =============================================

GUI_CREATE_HOT_SPARE
	鍒涘缓鐑鐩?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x54
	byte 3/4/5/6      鐢ㄤ簬鍒涘缓鐑鐩樼殑璁惧鎺╃爜
	================  =============================================

GUI_DELETE_HOT_SPARE
	鍒犻櫎鐑鐩?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x55
	byte 3/4/5/6      鐢ㄤ簬鍒犻櫎鐑鐩樼殑璁惧鎺╃爜
	================  =============================================

GUI_CREATE_VOLUME
	鍒涘缓鍗烽泦

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x60
	byte 3            raidset#
	byte 4-19         鍗烽泦鍚嶇О
			  (if byte4 == 0, use default)
	byte 20-27        volume capacity (blocks)
	byte 28 	  raid level
	byte 29 	  stripe size
			  (0/1/2/3/4/5->4/8/16/32/64/128K)
	byte 30 	  channel
	byte 31 	  ID
	byte 32 	  LUN
	byte 33 	  1 鍚敤鏍囪
	byte 34 	  1 鍚敤缂撳瓨
	byte 35 	  speed
			  (0/1/2/3/4->async/20/40/80/160 for scsi)
			  (0/1/2/3/4->33/66/100/133/150 for IDE  )
	byte 36 	  1 to select quick init
	================  =============================================

GUI_MODIFY_VOLUME
	淇敼鍗烽泦

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x61
	byte 3            volumeset#
	byte 4-19         鏂板嵎闆嗗悕绉?
			  (if byte4 == 0, not change)
	byte 20-27        鏂板嵎瀹归噺锛堜繚鐣欙級
	byte 28 	  new raid level
	byte 29 	  new stripe size
			  (0/1/2/3/4/5->4/8/16/32/64/128K)
	byte 30 	  new channel
	byte 31 	  new ID
	byte 32 	  new LUN
	byte 33 	  1 鍚敤鏍囪
	byte 34 	  1 鍚敤缂撳瓨
	byte 35 	  speed
			  (0/1/2/3/4->async/20/40/80/160 for scsi)
			  (0/1/2/3/4->33/66/100/133/150 for IDE  )
	================  =============================================

GUI_DELETE_VOLUME
	鍒犻櫎鍗烽泦

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x62
	byte 3            volumeset#
	================  =============================================

GUI_START_CHECK_VOLUME
	鍚姩鍗蜂竴鑷存€ф鏌?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x63
	byte 3            volumeset#
	================  =============================================

GUI_STOP_CHECK_VOLUME
	鍋滄鍗蜂竴鑷存€ф鏌?

	================  =============================================
	byte 0,1          length
	byte 2            command code 0x64
	================  =============================================

### 4. 杩斿洖鐨勬暟鎹?


(A) Header
    3 瀛楄妭搴忓垪锛?x5E, 0x01, 0x61锛?
(B) 闀垮害
    2 瀛楄妭
    锛堜綆瀛楄妭鍦ㄥ墠锛屼笉鍖呭惈闀垮害鍜屾牎楠屽瓧鑺傦級
(C)
    鐘舵€佹垨鏁版嵁锛?

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

	2) 濡傛灉闀垮害 > 1锛?

		浠庢帶鍒跺櫒杩斿洖鐨勬暟鎹潡
		鍏跺唴瀹瑰彇鍐充簬鍛戒护鐮?

```
(E) 鏍￠獙鍜?
    闀垮害鍜岀姸鎬佹垨鏁版嵁瀛楄妭鐨勬牎楠屽拰
