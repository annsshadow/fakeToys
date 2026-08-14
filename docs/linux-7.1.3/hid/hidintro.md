
## HID 鎶ュ憡鎻忚堪绗︾畝浠?


鏈珷鏃ㄥ湪骞挎硾姒傝堪 HID 鎶ュ憡鎻忚堪绗︽槸浠€涔堬紝浠ュ強涓€涓櫘閫氱殑锛堥潪鍐呮牳锛夌▼搴忓憳濡備綍澶勭悊鍦?Linux
涓嬪伐浣滀笉姝ｅ父鐨?HID 璁惧銆?

    :local:
    :depth: 2

- [hidreport-parsing](hidreport-parsing)

## 绠€浠?


HID 浠ｈ〃 Human Interface Device锛堜汉鏈烘帴鍙ｈ澶囷級锛屽彲浠ユ槸浣犵敤鏉ヤ笌璁＄畻鏈轰氦浜掔殑浠讳綍璁惧锛?
鏃犺鏄紶鏍囥€佽Е鎽告澘銆佹暟浣嶆澘杩樻槸楹﹀厠椋庛€?

璁稿 HID 璁惧寮€绠卞嵆鐢紙out of the box锛夛紝鍗充娇瀹冧滑鐨勭‖浠跺悇涓嶇浉鍚屻€備緥濡傦紝榧犳爣鍙互鏈変换鎰?
鏁伴噺鐨勬寜閽紱瀹冧滑鍙兘鏈変竴涓粴杞紱涓嶅悓鍨嬪彿涔嬮棿鐨勭Щ鍔ㄧ伒鏁忓害涓嶅悓锛岀瓑绛夈€傚敖绠″姝わ紝澶у鏁?
鏃跺€欎竴鍒囬兘鑳芥甯稿伐浣滐紝鑰屾棤闇€涓鸿嚜 1970 骞翠互鏉ュ紑鍙戠殑姣忎釜榧犳爣鍨嬪彿鍦ㄥ唴鏍镐腑缂栧啓涓撻棬鐨勪唬鐮併€?

杩欐槸鍥犱负鐜颁唬 HID 璁惧纭疄閫氳繃鍏?**HID 鎶ュ憡鎻忚堪绗?*锛圚ID report descriptor锛夆€斺€斾竴缁?
鍥哄畾鐨勫瓧鑺傦紝绮剧‘鎻忚堪璁惧涓庝富鏈轰箣闂村彲浠ュ彂閫佸摢浜?**HID 鎶ュ憡**锛圚ID reports锛変互鍙婅繖浜?
鎶ュ憡涓瘡涓崟鐙綅鐨勫惈涔夆€斺€旀潵澹版槑鍏惰兘鍔涖€備緥濡傦紝涓€涓?HID 鎶ュ憡鎻忚堪绗﹀彲浠ユ寚瀹?鈥滃湪 ID 涓?3
鐨勬姤鍛婁腑锛岀 8 鍒?15 浣嶆槸榧犳爣鐨?X 澧為噺鍧愭爣鈥濄€?

HID 鎶ュ憡鏈韩闅忓悗浠呮惡甯﹀疄闄呮暟鎹€硷紝娌℃湁浠讳綍棰濆鐨勫厓淇℃伅銆傝娉ㄦ剰锛孒ID 鎶ュ憡鍙互浠庤澶?
鍙戝嚭锛?Input Reports"锛屽嵆杈撳叆浜嬩欢锛夈€佸彂寰€璁惧锛?Output Reports"锛屼緥濡傜敤浜庢敼鍙?LED锛夋垨
鐢ㄤ簬璁惧閰嶇疆锛?Feature reports"锛夈€備竴涓澶囧彲浠ユ敮鎸佷竴涓垨澶氫釜 HID 鎶ュ憡銆?

HID 瀛愮郴缁熻礋璐ｈВ鏋?HID 鎶ュ憡鎻忚堪绗︼紝骞跺皢 HID 浜嬩欢杞崲涓烘甯哥殑杈撳叆璁惧鎺ュ彛锛堝弬瑙?
Documentation/hid/hid-transport.rst锛夈€傝澶囧彲鑳借涓哄紓甯革紝鍘熷洜鍖呮嫭璁惧鎻愪緵鐨?HID 鎶ュ憡
鎻忚堪绗︽湁璇€侀渶瑕佷互鐗规畩鏂瑰紡澶勭悊锛屾垨榛樿浠ｇ爜鏈鐞嗘煇浜涚壒娈婅澶囨垨浜や簰妯″紡銆?

HID 鎶ュ憡鎻忚堪绗︾殑鏍煎紡鐢变袱涓枃妗ｆ弿杩帮紝鍙粠 `USB Implementers Forum
<https://www.usb.org/>`_ `HID web page <https://www.usb.org/hid>`_ 鍦板潃鑾峰彇锛?

 - the `HID USB Device Class Definition
   <https://www.usb.org/document-library/device-class-definition-hid-111>`_ (HID Spec from now on)
 - the `HID Usage Tables <https://usb.org/document-library/hid-usage-tables-14>`_ (HUT from now on)

HID 瀛愮郴缁熷彲浠ュ鐞嗕笉鍚岀殑浼犺緭锛坱ransport锛夐┍鍔紙USB銆両2C銆丅luetooth 绛夛級銆傚弬瑙?
Documentation/hid/hid-transport.rst銆?

## 瑙ｆ瀽 HID 鎶ュ憡鎻忚堪绗?


褰撳墠 HID 璁惧鐨勫垪琛ㄥ彲鍦?`/sys/bus/hid/devices/` 鎵惧埌銆傚浜庢瘡涓澶囷紝渚嬪
`/sys/bus/hid/devices/0003\:093A\:2510.0002/`锛?
```
  $ hexdump -C /sys/bus/hid/devices/0003\:093A\:2510.0002/report_descriptor
  00000000  05 01 09 02 a1 01 09 01  a1 00 05 09 19 01 29 03  |..............).|
  00000010  15 00 25 01 75 01 95 03  81 02 75 05 95 01 81 01  |..%.u.....u.....|
  00000020  05 01 09 30 09 31 09 38  15 81 25 7f 75 08 95 03  |...0.1.8..%.u...|
  00000030  81 06 c0 c0                                       |....|
  00000034
```

鍙€夛細HID 鎶ュ憡鎻忚堪绗︿篃鍙互閫氳繃鐩存帴璁块棶 hidraw 椹卞姩 [#hidraw]_ 鏉ヨ鍙栥€?

HID 鎶ュ憡鎻忚堪绗︾殑鍩烘湰缁撴瀯鍦?HID 瑙勮寖涓畾涔夛紝鑰?HUT 鈥滃畾涔変簡涓€缁勫父閲忥紝鍙緵搴旂敤绋嬪簭瑙ｉ噴浠?
璇嗗埆 HID 鎶ュ憡涓暟鎹瓧娈电殑鐢ㄩ€斿拰鍚箟鈥濄€傛瘡涓潯鐩嚦灏戠敱涓ゅ瓧鑺傚畾涔夛紝鍏朵腑绗竴涓瓧鑺傚畾涔?
鍚庨潰璺熼殢鐨勫€肩殑绫诲瀷锛屽苟鍦?HID 瑙勮寖涓弿杩帮紱绗簩涓瓧鑺傛惡甯﹀疄闄呭€硷紝骞跺湪 HUT 涓弿杩般€?

鍘熷垯涓婏紝HID 鎶ュ憡鎻忚堪绗﹀彲浠ラ€愬瓧鑺傚湴銆佽垂鍔涘湴鎵嬪伐瑙ｆ瀽銆?

鍏充簬濡備綍鍋氬埌杩欎竴鐐圭殑绠€鐭粙缁嶆杩颁簬 Documentation/hid/hidreport-parsing.rst锛涘彧鏈夊綋浣?
闇€瑕佷慨琛ワ紙patch锛塇ID 鎶ュ憡鎻忚堪绗︽椂鎵嶉渶瑕佺悊瑙ｅ畠銆?

鍦ㄥ疄璺典腑锛屼綘涓嶅簲鎵嬪伐瑙ｆ瀽 HID 鎶ュ憡鎻忚堪绗︼紱鐩稿弽锛屼綘搴斿綋浣跨敤鐜版湁鐨勮В鏋愬櫒銆傚湪鎵€鏈夊彲鐢ㄧ殑
瑙ｆ瀽鍣ㄤ腑锛?

  - 鍦ㄧ嚎鐨?`USB Descriptor and Request Parser
    <http://eleccelerator.com/usbdescreqparser/>`_锛?
  - `hidrdd <https://github.com/abend0c1/hidrdd>`_锛?
    瀹冩彁渚涢潪甯歌缁嗕笖鏈変簺鍐楅暱鐨勬弿杩帮紙濡傛灉浣犱笉鐔熸倝 HID 鎶ュ憡鎻忚堪绗︼紝杩欑鍐楅暱鍙兘寰堟湁鐢級锛?
  - `hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_锛?
    涓€濂楀畬鏁寸殑瀹炵敤宸ュ叿闆嗭紝闄ゅ叾瀹冨姛鑳藉锛屽厑璁歌褰曞拰鍥炴斁鍘熷鐨?HID 鎶ュ憡锛屼互鍙婅皟璇曞拰鍥炴斁
    HID 璁惧銆傚畠姝ｇ敱 Linux HID 瀛愮郴缁熺淮鎶よ€呯Н鏋佸紑鍙戙€?

鐢?`hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_ 瑙ｆ瀽榧犳爣鐨?HID 鎶ュ憡
鎻忚堪绗﹀緱鍒帮細
```
    $ ./hid-decode /sys/bus/hid/devices/0003\:093A\:2510.0002/report_descriptor
    # device 0:0
    # 0x05, 0x01,		     // Usage Page (Generic Desktop)	    0
    # 0x09, 0x02,		     // Usage (Mouse)			    2
    # 0xa1, 0x01,		     // Collection (Application)	    4
    # 0x09, 0x01,		     // Usage (Pointer)		    	    6
    # 0xa1, 0x00,		     // Collection (Physical)  	    	    8
    # 0x05, 0x09, 		     //	Usage Page (Button)		   10
```
```
    # 0x19, 0x01, 		     //	Usage Minimum (1)		   12
    # 0x29, 0x03, 		     //	Usage Maximum (3)		   14
```
```
    # 0x15, 0x00, 		     //	Logical Minimum (0)		   16
    # 0x25, 0x01, 		     //	Logical Maximum (1)		   18
```
姣忎釜鎸夐挳鍙互鍙戦€佷粠 0 鍒板寘鎷?1 鐨勫€?
```
    # 0x75, 0x01, 		     //	Report Size (1) 		   20
```
```
    # 0x95, 0x03, 		     //	Report Count (3)		   22
```
```
    # 0x81, 0x02, 		     //	Input (Data,Var,Abs)		   24
```
瀹冩槸瀹為檯鐨勬暟鎹紙Data锛岄潪甯搁噺濉厖锛夛紝琛ㄧず鍗曚釜鍙橀噺锛圴ar锛夛紝鍏跺€间负缁濆锛圓bsolute锛岃€岄潪
鐩稿锛夛紱
```
    # 0x75, 0x05, 		     //	Report Size (5) 		   26
```
```
    # 0x95, 0x01, 		     //	Report Count (1)		   28
```
```
    # 0x81, 0x01, 		     //	Input (Cnst,Arr,Abs)		   30
```
```
    # 0x05, 0x01,		     // Usage Page (Generic Desktop)       32
    # 0x09, 0x30,		     // Usage (X)			   34
    # 0x09, 0x31,		     // Usage (Y)			   36
    # 0x09, 0x38,		     // Usage (Wheel) 		    	   38
```
璇ラ紶鏍囪繕鏈変袱涓墿鐞嗕綅缃紙Usage (X)銆乁sage (Y)锛?
```
    # 0x15, 0x81, 		     //	Logical Minimum (-127)  	   40
    # 0x25, 0x7f, 		     //	Logical Maximum (127)		   42
```
```
    # 0x75, 0x08, 		     //	Report Size (8) 		   44
```
```
    # 0x95, 0x03, 		     //	Report Count (3)		   46
```
```
    # 0x81, 0x06,		     // Input (Data,Var,Rel)  	    	   48
```
杩欐鏁版嵁鍊兼槸鐩稿鐨勶紙Relative锛孯el锛夛紝鍗冲畠浠〃绀?
```
    # 0xc0,			     // End Collection 		    	   50
    # 0xc0,			     // End Collection  		   51
    #
    R: 52 05 01 09 02 a1 01 09 01 a1 00 05 09 19 01 29 03 15 00 25 01 75 01 95 03 81 02 75 05 95 01 81 01 05 01 09 30 09 31 09 38 15 81 25 7f 75 08 95 03 81 06 c0 c0
    N: device 0:0
    I: 3 0001 0001
```
杩欎釜鎶ュ憡鎻忚堪绗﹀憡璇夋垜浠紝榧犳爣杈撳叆灏嗕娇鐢ㄥ洓涓瓧鑺備紶杈擄細绗竴涓瓧鑺傜敤浜庢寜閽紙浣跨敤涓変綅锛屼簲浣?
鐢ㄤ簬濉厖锛夛紝鏈€鍚庝笁涓瓧鑺傚垎鍒敤浜庨紶鏍囩殑 X銆乊 鍜屾粴杞彉鍖栥€?

瀹為檯涓婏紝瀵逛簬浠讳綍浜嬩欢锛岄紶鏍囬兘浼氬彂閫佷竴涓洓瀛楄妭鐨?**report**銆傛垜浠彲浠ラ€氳繃渚嬪鍊熷姪鏉ヨ嚜
`hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_ 鐨?`hid-recorder` 宸ュ叿
鏉ユ鏌ュ彂閫佺殑鍊硷細
```
  $ sudo ./hid-recorder /dev/hidraw1

  ....
  output of hid-decode
  ....

  #  Button: 1  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000000.000000 4 01 00 00 00
  #  Button: 0  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000000.183949 4 00 00 00 00
  #  Button: 0  1  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000001.959698 4 02 00 00 00
  #  Button: 0  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000002.103899 4 00 00 00 00
  #  Button: 0  0  1 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000004.855799 4 04 00 00 00
  #  Button: 0  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000005.103864 4 00 00 00 00
```
杩欎釜渚嬪瓙琛ㄦ槑锛屽綋鐐瑰嚮鎸夐挳 2 鏃讹紝浼氬彂閫佸瓧鑺?`02 00 00 00`锛岃€岀揣闅忓叾鍚庣殑浜嬩欢
锛坄00 00 00 00`锛夋槸鎸夐挳 2 鐨勯噴鏀撅紙娌℃湁鎸夐挳琚寜涓嬶紝璇疯浣忔暟鎹€兼槸 **缁濆**锛坅bsolute锛?
鐨勶級銆?

濡傛灉鏀逛负鍏堢偣鍑诲苟鎸変綇鎸夐挳 1锛岀劧鍚庣偣鍑诲苟鎸変綇鎸夐挳
```
  #  Button: 1  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000044.175830 4 01 00 00 00
  #  Button: 1  1  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000045.975997 4 03 00 00 00
  #  Button: 0  1  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000047.407930 4 02 00 00 00
  #  Button: 0  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000049.199919 4 00 00 00 00
```
鍏朵腑浣跨敤 `03 00 00 00` 琛ㄧず涓や釜鎸夐挳閮借鎸変笅锛岃€岄殢鍚庣殑 `02 00 00 00` 琛ㄧず鎸夐挳 1 琚噴鏀?
鑰屾寜閽?2 浠嶅浜庢縺娲荤姸鎬併€?

### Output銆両nput 涓?Feature 鎶ュ憡


HID 璁惧鍙互鍏锋湁 Input 鎶ュ憡锛堝榧犳爣绀轰緥锛夈€丱utput 鎶ュ憡鍜?Feature 鎶ュ憡銆傗€淥utput鈥?鎰忓懗鐫€
淇℃伅琚彂寰€璁惧銆備緥濡傦紝甯︽湁鍔涘弽棣堬紙force feedback锛夌殑鎿嶇旱鏉嗕細鏈夋煇浜涜緭鍑猴紱閿洏鐨?LED 涔?
闇€瑕佽緭鍑恒€傗€淚nput鈥?鎰忓懗鐫€鏁版嵁鏉ヨ嚜璁惧銆?

鈥淔eature鈥?骞堕潪渚涙渶缁堢敤鎴锋秷璐癸紝鑰屾槸瀹氫箟璁惧鐨勯厤缃€夐」銆傚畠浠彲浠ヤ粠涓绘満鏌ヨ锛涘綋澹版槑涓?
**Volatile**锛堟槗鍙橈級鏃讹紝瀹冧滑搴旂敱涓绘満鏇存敼銆?


## 闆嗗悎锛圕ollections锛夈€佹姤鍛?ID 涓?Evdev 浜嬩欢


鍗曚釜璁惧鍙互鍦ㄩ€昏緫涓婂皢鏁版嵁鍒嗙粍鍒颁笉鍚岀殑鐙珛闆嗗悎涓紝绉颁负 **Collection**锛堥泦鍚堬級銆傞泦鍚堝彲浠?
宓屽锛屽苟涓斿瓨鍦ㄤ笉鍚岀被鍨嬬殑闆嗗悎锛堣瑙?HID 瑙勮寖 6.2.2.6 鈥淐ollection, End Collection
Items鈥濓級銆?

涓嶅悓鐨勬姤鍛婇€氳繃涓嶅悓鐨?**Report ID**锛堟姤鍛?ID锛夊瓧娈垫潵鏍囪瘑锛屽嵆涓€涓敤浜庢爣璇嗙揣闅忓叾鍚庣殑鎶ュ憡
缁撴瀯缂栧彿銆傛瘡褰撻渶瑕?Report ID 鏃讹紝瀹冮兘浣滀负浠讳綍鎶ュ憡鐨勭涓€涓瓧鑺備紶杈撱€備竴涓彧鏀寔鍗曚釜 HID
鎶ュ憡鐨勮澶囷紙濡備笂闈㈢殑榧犳爣绀轰緥锛夊彲浠ョ渷鐣ユ姤鍛?ID銆?

```
  05 01 09 02 A1 01 85 01 05 09 19 01 29 05 15 00
  25 01 95 05 75 01 81 02 95 01 75 03 81 01 05 01
  09 30 09 31 16 00 F8 26 FF 07 75 0C 95 02 81 06
  09 38 15 80 25 7F 75 08 95 01 81 06 05 0C 0A 38
  02 15 80 25 7F 75 08 95 01 81 06 C0 05 01 09 02
  A1 01 85 02 05 09 19 01 29 05 15 00 25 01 95 05
  75 01 81 02 95 01 75 03 81 01 05 01 09 30 09 31
  16 00 F8 26 FF 07 75 0C 95 02 81 06 09 38 15 80
  25 7F 75 08 95 01 81 06 05 0C 0A 38 02 15 80 25
  7F 75 08 95 01 81 06 C0 05 01 09 07 A1 01 85 05
  05 07 15 00 25 01 09 29 09 3E 09 4B 09 4E 09 E3
  09 E8 09 E8 09 E8 75 01 95 08 81 02 95 00 81 01
  C0 05 0C 09 01 A1 01 85 06 15 00 25 01 75 01 95
  01 09 3F 81 06 09 3F 81 06 09 3F 81 06 09 3F 81
  06 09 3F 81 06 09 3F 81 06 09 3F 81 06 09 3F 81
  06 C0 05 0C 09 01 A1 01 85 03 09 05 15 00 26 FF
  00 75 08 95 02 B1 02 C0
```
鍦ㄨВ鏋愬畠涔嬪悗锛堣瘯鐫€鐢ㄥ缓璁殑宸ュ叿鑷繁瑙ｆ瀽锛侊級鍙互鐪嬪埌锛岃璁惧鍛堢幇浜嗕袱涓?`Mouse` 搴旂敤闆嗗悎
锛堝垎鍒敱鎶ュ憡 ID 1 鍜?2 鏍囪瘑锛夈€佷竴涓?`Keypad` 搴旂敤闆嗗悎锛堝叾鎶ュ憡鐢辨姤鍛?ID 5 鏍囪瘑锛変互鍙婁袱涓?
`Consumer Controls` 搴旂敤闆嗗悎锛堝垎鍒敱鎶ュ憡 ID 6 鍜?3 鏍囪瘑锛夈€備絾璇锋敞鎰忥紝涓€涓澶囧彲浠ラ拡瀵?
鍚屼竴涓簲鐢ㄩ泦鍚堜娇鐢ㄤ笉鍚岀殑鎶ュ憡 ID銆?

鍙戦€佺殑鏁版嵁灏嗕互鎶ュ憡 ID 瀛楄妭寮€澶达紝闅忓悗鏄浉搴旂殑淇℃伅銆備緥濡傦紝涓轰互涓嬮儴鍒嗕紶杈撶殑鏁版嵁锛?
```
  0x05, 0x0C,        // Usage Page (Consumer)
  0x09, 0x01,        // Usage (Consumer Control)
  0xA1, 0x01,        // Collection (Application)
  0x85, 0x03,        //   Report ID (3)
  0x09, 0x05,        //   Usage (Headphone)
  0x15, 0x00,        //   Logical Minimum (0)
  0x26, 0xFF, 0x00,  //   Logical Maximum (255)
  0x75, 0x08,        //   Report Size (8)
  0x95, 0x02,        //   Report Count (2)
  0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
  0xC0,              // End Collection
```
灏嗘槸涓変釜瀛楄妭锛氱涓€涓瓧鑺傛槸鎶ュ憡 ID锛?锛夛紝鎺ヤ笅鏉ョ殑涓や釜瀛楄妭鐢ㄤ簬鑰虫満锛屽悇涓轰袱涓?
锛坄Report Count (2)`锛夊瓧鑺傦紙`Report Size (8)`锛夛紝姣忎釜瀛楄妭鐨勮寖鍥翠粠 0锛坄Logical Minimum
(0)`锛夊埌 255锛坄Logical Maximum (255)`锛夈€?

璁惧鍙戦€佺殑鎵€鏈?Input 鏁版嵁閮藉簲琚浆鎹负鐩稿簲鐨?Evdev 浜嬩欢锛屼互渚垮崗璁爤鐨勫叾浣欓儴鍒嗚兘澶熺煡閬?
鍙戠敓浜嗕粈涔堬紝渚嬪绗竴涓寜閽殑浣嶈浆鎹负 `EV_KEY/BTN_LEFT` evdev 浜嬩欢锛岀浉瀵圭殑 X 绉诲姩杞崲涓?
`EV_REL/REL_X` evdev 浜嬩欢銆?

## 浜嬩欢


鍦?Linux 涓紝浼氫负姣忎釜 ``Application Collection`` 鍒涘缓涓€涓?`/dev/input/event*`銆傚洖鍒伴紶鏍囩殑
渚嬪瓙锛屽苟閲嶅鍏堢偣鍑诲苟鎸変綇鎸夐挳 1銆佺劧鍚庣偣鍑诲苟鎸変綇鐨勫簭鍒楋細
```
  $ sudo libinput record /dev/input/event1
  # libinput record
  version: 1
  ndevices: 1
  libinput:
    version: "1.23.0"
    git: "unknown"
  system:
    os: "opensuse-tumbleweed:20230619"
    kernel: "6.3.7-1-default"
    dmi: "dmi:bvnHP:bvrU77Ver.01.05.00:bd03/24/2022:br5.0:efr20.29:svnHP:pnHPEliteBook64514inchG9NotebookPC:pvr:rvnHP:rn89D2:rvrKBCVersion14.1D.00:cvnHP:ct10:cvr:sku5Y3J1EA#ABZ:"
  devices:
  - node: /dev/input/event1
    evdev:
      # Name: PixArt HP USB Optical Mouse
      # ID: bus 0x3 vendor 0x3f0 product 0x94a version 0x111
      # Supported Events:
      # Event type 0 (EV_SYN)
      # Event type 1 (EV_KEY)
      #   Event code 272 (BTN_LEFT)
      #   Event code 273 (BTN_RIGHT)
      #   Event code 274 (BTN_MIDDLE)
      # Event type 2 (EV_REL)
      #   Event code 0 (REL_X)
      #   Event code 1 (REL_Y)
      #   Event code 8 (REL_WHEEL)
      #   Event code 11 (REL_WHEEL_HI_RES)
      # Event type 4 (EV_MSC)
      #   Event code 4 (MSC_SCAN)
      # Properties:
      name: "PixArt HP USB Optical Mouse"
      id: [3, 1008, 2378, 273]
      codes:
  	0: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] # EV_SYN
  	1: [272, 273, 274] # EV_KEY
  	2: [0, 1, 8, 11] # EV_REL
  	4: [4] # EV_MSC
      properties: []
    hid: [
      0x05, 0x01, 0x09, 0x02, 0xa1, 0x01, 0x09, 0x01, 0xa1, 0x00, 0x05, 0x09, 0x19, 0x01, 0x29, 0x03,
      0x15, 0x00, 0x25, 0x01, 0x95, 0x08, 0x75, 0x01, 0x81, 0x02, 0x05, 0x01, 0x09, 0x30, 0x09, 0x31,
      0x09, 0x38, 0x15, 0x81, 0x25, 0x7f, 0x75, 0x08, 0x95, 0x03, 0x81, 0x06, 0xc0, 0xc0
    ]
    udev:
      properties:
      - ID_INPUT=1
      - ID_INPUT_MOUSE=1
      - LIBINPUT_DEVICE_GROUP=3/3f0/94a:usb-0000:05:00.3-2
    quirks:
    events:
    # Current time is 12:31:56
    - evdev:
      - [  0,	   0,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  0,	   0,	1, 272,       1] # EV_KEY / BTN_LEFT		      1
      - [  0,	   0,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +0ms
    - evdev:
      - [  1, 207892,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  1, 207892,	1, 273,       1] # EV_KEY / BTN_RIGHT		      1
      - [  1, 207892,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +1207ms
    - evdev:
      - [  2, 367823,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  2, 367823,	1, 272,       0] # EV_KEY / BTN_LEFT		      0
      - [  2, 367823,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +1160ms
    # Current time is 12:32:00
    - evdev:
      - [  3, 247617,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  3, 247617,	1, 273,       0] # EV_KEY / BTN_RIGHT		      0
      - [  3, 247617,   0,   0,       0] # ------------ SYN_REPORT (0) ---------- +880ms
```
娉ㄦ剰锛氬鏋滀綘鐨勭郴缁熶笂娌℃湁 `libinput record`锛岃灏濊瘯浣跨敤 `evemu-record`銆?

## 褰撴煇浜涘姛鑳戒笉宸ヤ綔鏃?


璁惧琛屼负涓嶆纭殑鍘熷洜鍙兘鏈夊緢澶氥€備緥濡傦細

- 璁惧鎻愪緵鐨?HID 鎶ュ憡鎻忚堪绗﹀彲鑳芥槸閿欒鐨勶紝渚嬪

  - 瀹冧笉閬靛惊鏍囧噯锛屽洜姝ゅ唴鏍稿皢鏃犳硶鐞嗚В璇?HID 鎶ュ憡鎻忚堪绗︼紱
  - HID 鎶ュ憡鎻忚堪绗?**涓庡疄闄?* 璁惧鍙戦€佺殑鍐呭涓嶅尮閰嶏紙杩欏彲浠ラ€氳繃璇诲彇鍘熷 HID 鏁版嵁鏉ラ獙璇侊級锛?
- HID 鎶ュ憡鎻忚堪绗﹀彲鑳介渶瑕佷竴浜?鈥渜uirks鈥濓紙鎬櫀锛岃鍚庢枃锛夈€?

鍥犳锛屽彲鑳戒笉浼氫负姣忎釜搴旂敤闆嗗悎鍒涘缓 `/dev/input/event*`锛屽苟涓?鎴栬€呭叾涓殑浜嬩欢鍙兘涓嶇鍚?
浣犵殑棰勬湡銆?


### Quirks锛堟€櫀锛?


鍐呮牳鐭ラ亾濡備綍淇鐨?HID 璁惧鏈変竴浜涘凡鐭ョ殑鐗规€р€斺€旇繖浜涜绉颁负 HID quirks锛屽叾鍒楄〃鍙湪
`include/linux/hid.h` 涓壘鍒般€?

濡傛灉鏄繖绉嶆儏鍐碉紝瀵逛簬鎵嬪ご鐨?HID 璁惧锛屽彧闇€鍦ㄥ唴鏍镐腑娣诲姞鎵€闇€鐨?quirk 鍗冲彲銆傝繖鍙互鍦?
`drivers/hid/hid-quirks.c` 鏂囦欢涓畬鎴愩€傚湪鏌ョ湅璇ユ枃浠跺悗锛屽浣曞仛搴旇鐩稿鐩磋銆?

褰撳墠瀹氫箟鐨?quirks 鍒楄〃锛堟潵鑷?`include/linux/hid.h`锛夋槸

   :doc: HID quirks

USB 璁惧鐨?quirks 鍙互鍦ㄥ姞杞?usbhid 妯″潡鏃舵寚瀹氾紝鍙傝 `modinfo usbhid`锛屼絾姝ｇ‘鐨勪慨澶?
搴斿綋杩涘叆 hid-quirks.c 骞?**鎻愪氦鍒颁笂娓革紙be submitted upstream锛?*銆傚叧浜庡浣曟彁浜よˉ涓佺殑鎸囧崡锛?
璇峰弬瑙?Documentation/process/submitting-patches.rst銆傚叾瀹冩€荤嚎鐨?quirks 闇€瑕佽繘鍏?
hid-quirks.c銆?

### 淇ˉ HID 鎶ュ憡鎻忚堪绗?


濡傛灉浣犻渶瑕佷慨琛?HID 鎶ュ憡鎻忚堪绗︼紝鏈€绠€鍗曠殑鏂规硶鏄眰鍔╀簬 eBPF锛屽 Documentation/hid/hid-bpf.rst
涓墍杩般€?

鍩烘湰涓婏紝浣犲彲浠ユ洿鏀瑰師濮?HID 鎶ュ憡鎻忚堪绗︾殑浠讳綍瀛楄妭銆俿amples/hid 涓殑绀轰緥搴旇鏄竴涓緢濂界殑
璧风偣锛?
```
  SEC("fmod_ret/hid_bpf_rdesc_fixup")
  int BPF_PROG(hid_rdesc_fixup, struct hid_bpf_ctx *hctx)
  {
    ....
       data[39] = 0x31;
       data[41] = 0x30;
    return 0;
  }
```
褰撶劧锛岃繖涔熷彲浠ュ湪鍐呮牳婧愮爜涓畬鎴愶紝渚嬪鍙傝€?`drivers/hid/hid-aureal.c` 鎴?
`drivers/hid/hid-samsung.c` 浠ヨ幏寰楃◢寰鏉備竴浜涚殑鏂囦欢銆?

濡傛灉浣犲湪鏌ラ槄 HID 鎵嬪唽鍜岀悊瑙?HID 鎶ュ憡鎻忚堪绗﹀崄鍏繘鍒舵暟瀛楃殑纭垏鍚箟鏂归潰闇€瑕佷换浣曞府鍔╋紝璇?
鏌ラ槄 Documentation/hid/hidreport-parsing.rst銆?

鏃犺浣犳兂鍑轰粈涔堣В鍐虫柟妗堬紝璇疯浣?**灏嗕慨澶嶆彁浜ょ粰 HID 缁存姢鑰?*锛屼互渚垮畠鑳界洿鎺ユ暣鍚堣繘鍐呮牳锛屼娇
閭ｄ釜鐗瑰畾鐨?HID 璁惧鑳藉鎵€鏈夊叾浠栦汉姝ｅ父宸ヤ綔銆傚叧浜庡浣曞仛鍒拌繖涓€鐐圭殑鎸囧崡锛岃鍙傝
Documentation/process/submitting-patches.rst銆?


### 鍔ㄦ€佷慨鏀逛紶杈撶殑鏁版嵁


浣跨敤 eBPF 杩樺彲浠ヤ慨鏀逛笌璁惧浜ゆ崲鐨勬暟鎹€傚啀娆″弬瑙?samples/hid 涓殑绀轰緥銆?

鍚屾牱鍦帮紝**璇峰彂甯冧綘鐨勪慨澶?*锛屼互渚垮畠鑳芥暣鍚堣繘鍐呮牳锛?

### 缂栧啓涓撻棬鐨勯┍鍔?


杩欑湡鐨勫簲璇ユ槸浣犵殑鏈€鍚庢墜娈点€?

渚嬪鍙弬鑰?`samples/hidraw/hid-example.c` 鏂囦欢銆?
```
    $ sudo ./hid-example
    Report Descriptor Size: 52
    Report Descriptor:
    5 1 9 2 a1 1 9 1 a1 0 5 9 19 1 29 3 15 0 25 1 75 1 95 3 81 2 75 5 95 1 81 1 5 1 9 30 9 31 9 38 15 81 25 7f 75 8 95 3 81 6 c0 c0

    Raw Name: PixArt USB Optical Mouse
    Raw Phys: usb-0000:05:00.4-2.3/input0
    Raw Info:
            bustype: 3 (USB)
            vendor: 0x093a
            product: 0x2510
    ...
```
