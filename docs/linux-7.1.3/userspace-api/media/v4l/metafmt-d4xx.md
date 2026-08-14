
######## V4L2_META_FMT_D4XX ('D4XX')


Intel D4xx UVC 鎽勫儚澶村厓鏁版嵁


## 鎻忚堪


Intel D4xx锛圖435銆丏455 鍙婂叾浠栵級鎽勫儚澶村湪鍏?UVC 鏈夋晥璐熻浇澶撮儴涓寘鍚瘡甯у厓鏁版嵁锛?閬靛惊 Microsoft(R) UVC 鎵╁睍鎻愭 [1_]銆傝繖鎰忓懗鐫€锛岄伒寰爣鍑?UVC 澶撮儴鐨勭鏈?D4XX
鍏冩暟鎹寜鍧楃粍缁囥€侱4xx 鎽勫儚澶村疄鐜颁簡 Microsoft 鎻愬嚭鐨勮嫢骞叉爣鍑嗗潡绫诲瀷锛屼互鍙婅嫢骞?涓撴湁鍧楃被鍨嬨€傛敮鎸佺殑鏍囧噯鍏冩暟鎹被鍨嬩负 MetadataId_CaptureStats锛圛D 3锛夈€?MetadataId_CameraExtrinsics锛圛D 4锛夊拰 MetadataId_CameraIntrinsics锛圛D 5锛夈€?鍏惰鏄庤 [1_]銆傛湰鏂囨。鎻忚堪 D4xx 鎽勫儚澶翠娇鐢ㄧ殑涓撴湁鍏冩暟鎹被鍨嬨€?
V4L2_META_FMT_D4XX 缂撳啿鍖洪伒寰?V4L2_META_FMT_UVC 鐨勫厓鏁版嵁缂撳啿鍖哄竷灞€锛屽敮涓€鐨?鍖哄埆鍦ㄤ簬瀹冭繕鍖呭惈涓撴湁鏈夋晥璐熻浇澶撮儴鏁版嵁銆侱4xx 鎽勫儚澶翠娇鐢ㄦ壒閲忎紶杈擄紝姣忓抚浠呭彂閫?涓€涓湁鏁堣礋杞斤紝鍥犳鍏跺ご閮ㄤ笉鑳借秴杩?255 瀛楄妭銆?
鏈枃妗ｅ疄鐜?Intel 閰嶇疆鐗堟湰 3 [9_]銆?
浠ヤ笅鏄?D4xx 鎽勫儚澶翠娇鐢ㄧ殑涓撴湁 Microsoft 椋庢牸鍏冩暟鎹被鍨嬶紝鎵€鏈夊瓧娈靛潎閲囩敤灏忕搴忥細



    :widths: 1 2
    :header-rows:  1
    :stub-columns: 0

    - - **瀛楁**
      - **鎻忚堪**
    - - `1` **娣卞害鎺у埗**
    - - __u32 ID
      - 0x80000000
    - - __u32 Size
      - 瀛楄妭鏁帮紝鍖呭惈 ID锛堟墍鏈夊崗璁増鏈細60锛?    - - __u32 Version
      - 鏈粨鏋勪綋鐨勭増鏈€傛湰鏂囨。娑电洊鐗堟湰 1銆? 鍜?3銆傛柊澧炲瓧娈垫椂鐗堟湰鍙蜂細閫掑銆?    - - __u32 Flags
      - 鏍囧織浣嶆帺鐮侊細瑙佷笅鏂?[2_]
    - - __u32 Gain
      - 浠ュ唴閮ㄥ崟浣嶈〃绀虹殑澧炵泭鍊硷紝涓庣敤浜庨噰闆嗚甯х殑 V4L2_CID_GAIN 鎺т欢鐩稿悓
    - - __u32 Exposure
      - 閲囬泦璇ュ抚鎵€鐢ㄧ殑鏇濆厜鏃堕棿锛堝井绉掞級
    - - __u32 Laser power
      - 婵€鍏?LED 鐨勫姛鐜?0-360锛岀敤浜庢繁搴︽祴閲?    - - __u32 AE mode
      - 0锛氭墜鍔紱1锛氳嚜鍔ㄦ洕鍏?    - - __u32 Exposure priority
      - 鏇濆厜浼樺厛绾у€硷細0 - 鎭掑畾甯х巼
    - - __u32 AE ROI left
      - AE 鎰熷叴瓒ｅ尯鍩燂紙ROI锛夌殑宸﹁竟鐣岋紙鎵€鏈?ROI 鍊煎潎浠ュ儚绱犱负鍗曚綅锛屼笖鍒嗗埆浠嬩簬 0 涓庢渶澶у搴︽垨鏈€澶ч珮搴︿箣闂达級
    - - __u32 AE ROI right
      - AE ROI 鐨勫彸杈圭晫
    - - __u32 AE ROI top
      - AE ROI 鐨勪笂杈圭晫
    - - __u32 AE ROI bottom
      - AE ROI 鐨勪笅杈圭晫
    - - __u32 Preset
      - 棰勮閫夋嫨鍣ㄥ€硷紝榛樿鍊硷細0锛岄櫎闈炵敤鎴蜂慨鏀?    - - __u8 Emitter mode (v3 only) (__u32 Laser mode for v1) [8_]
      - 0锛氬叧闂紝1锛氬紑鍚紝涓?v1 鐨?__u32 Laser mode 鐩稿悓
    - - __u8 RFU byte (v3 only)
      - 棰勭暀瀛楄妭锛屼緵灏嗘潵浣跨敤
    - - __u16 LED Power (v3 only)
      - LED 鍔熺巼鍊?0-360锛團416 SKU锛?
    - - `1` **閲囬泦鏃跺簭**
    - - __u32 ID
      - 0x80000001
    - - __u32 Size
      - 瀛楄妭鏁帮紝鍖呭惈 ID锛堟墍鏈夊崗璁増鏈細40锛?    - - __u32 Version
      - 鏈粨鏋勪綋鐨勭増鏈€傛湰鏂囨。瀵瑰簲鐗堟湰 xxx銆傛柊澧炲瓧娈垫椂鐗堟湰鍙蜂細閫掑銆?    - - __u32 Flags
      - 鏍囧織浣嶆帺鐮侊細瑙佷笅鏂?[3_]
    - - __u32 Frame counter
      - 鍗曡皟閫掑璁℃暟鍣?    - - __u32 Optical time
      - 浠庡抚寮€濮嬪埌甯т腑闂寸殑鏃堕棿锛堝井绉掞級
    - - __u32 Readout time
      - 璇诲嚭涓€甯ф墍鐢ㄧ殑鏃堕棿锛堝井绉掞級
    - - __u32 Exposure time
      - 甯ф洕鍏夋椂闂达紙寰锛?    - - __u32 Frame interval
      - 鍗曚綅寰 = 1000000 / 甯х巼
    - - __u32 Pipe latency
      - 浠庡抚寮€濮嬪埌鏁版嵁杩涘叆 USB 缂撳啿鍖虹殑鏃堕棿锛堝井绉掞級

    - - `1` **閰嶇疆**
    - - __u32 ID
      - 0x80000002
    - - __u32 Size
      - 瀛楄妭鏁帮紝鍖呭惈 ID锛坴1:36锛寁3:40锛?    - - __u32 Version
      - 鏈粨鏋勪綋鐨勭増鏈€傛湰鏂囨。瀵瑰簲鐗堟湰 xxx銆傛柊澧炲瓧娈垫椂鐗堟湰鍙蜂細閫掑銆?    - - __u32 Flags
      - 鏍囧織浣嶆帺鐮侊細瑙佷笅鏂?[4_]
    - - __u8 Hardware type
      - 鎽勫儚澶寸‖浠剁増鏈?[5_]
    - - __u8 SKU ID
      - 鎽勫儚澶寸‖浠堕厤缃?[6_]
    - - __u32 Cookie
      - 鍐呴儴鍚屾
    - - __u16 Format
      - 鍥惧儚鏍煎紡浠ｇ爜 [7_]
    - - __u16 Width
      - 瀹藉害锛堝儚绱狅級
    - - __u16 Height
      - 楂樺害锛堝儚绱狅級
    - - __u16 Framerate
      - 璇锋眰鐨勬瘡绉掑抚鐜?    - - __u16 Trigger
      - 瀛楄妭 0锛歜it 0锛氭繁搴︿笌 RGB 宸插悓姝ワ紝bit 1锛氬閮ㄨЕ鍙?    - - __u16 Calibration count (v3 only)
      - 鏍″噯璁℃暟鍣紝瑙佷笅鏂?[4_]
    - - __u8 GPIO input data (v3 only)
      - GPIO 璇诲嚭锛岃涓嬫柟 [4_]锛堣嚜鍥轰欢 5.12.7.0 璧锋敮鎸侊級
    - - __u32 Sub-preset info (v3 only)
      - 瀛愰璁鹃€夋嫨淇℃伅锛岃涓嬫柟 [4_]
    - - __u8 reserved (v3 only)
      - RFU 瀛楄妭銆?

[^1^] https://docs.microsoft.com/en-us/windows-hardware/drivers/stream/uvc-extensions-1-5


```
0x00000001 Gain
0x00000002 Exposure
0x00000004 Laser power
0x00000008 AE mode
0x00000010 Exposure priority
0x00000020 AE ROI
0x00000040 Preset
0x00000080 Emitter mode
0x00000100 LED Power
```

```
0x00000001 Frame counter
0x00000002 Optical time
0x00000004 Readout time
0x00000008 Exposure time
0x00000010 Frame interval
0x00000020 Pipe latency
```

```
0x00000001 Hardware type
0x00000002 SKU ID
0x00000004 Cookie
0x00000008 Format
0x00000010 Width
0x00000020 Height
0x00000040 Framerate
0x00000080 Trigger
0x00000100 Cal count
0x00000200 GPIO Input Data
0x00000400 Sub-preset Info
```

```
0 DS5
1 IVCAM2
```

```
  [1:0] depthCamera
	00: no depth
	01: standard depth
	10: wide depth
	11: reserved
  [2]   depthIsActive - has a laser projector
  [3]   RGB presence
  [4]   Inertial Measurement Unit (IMU) presence
  [5]   projectorType
	0: HPTG
	1: Princeton
  [6]   0: a projector, 1: an LED
  [7]   reserved
```

[^7^] 鍚勮棰戞祦鎺ュ彛鐨勫浘鍍忔牸寮忎唬鐮侊細

```
1 Z16
2 Z
```

```
1 Y8
2 UYVY
3 R8L8
4 Calibration
5 W10
```

```
1 RAW8
```

[^8^] "Laser mode" 鍦ㄧ増鏈?3 涓凡琚笁涓笉鍚岀殑瀛楁鍙栦唬銆?鐢变簬鎽勫儚澶存姇褰变华鏈夊绉嶆妧鏈紝"Laser" 宸查噸鍛藉悕涓?"Emitter"銆傜敱浜庡彟鏈?"Laser Power" 瀛楁锛屾垜浠负棰濆鐨勫彂灏勫櫒寮曞叆浜?"LED Power"銆?
```
   1 __u8 Emitter mode
   2 __u8 RFU byte
   3 __u16 LED Power
```
杩欐槸鐗堟湰 1 涓?3 涔嬮棿鐨勫彉鏇淬€傜増鏈?1銆?銆? 鍧囦笌鐩稿悓鐨勬暟鎹牸寮忓悜鍚庡吋瀹癸紝
涓斿潎鍙楁敮鎸併€傚摢浜涘睘鎬ф湁鏁堣瑙?[2_]銆?

[^9^] LibRealSense SDK 鍏冩暟鎹潵婧愶細
https://github.com/IntelRealSense/librealsense/blob/master/src/metadata.h
