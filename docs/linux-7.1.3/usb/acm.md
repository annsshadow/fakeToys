## Linux ACM 椹卞姩 v0.16


Copyright (c) 1999 Vojtech Pavlik <vojtech@suse.cz>

鐢?SuSE 璧炲姪

#### 0. 鍏嶈矗澹版槑

鏈▼搴忔槸鑷敱杞欢锛涙偍鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇佹潯娆句笅
閲嶆柊鍙戝竷鍜?鎴栦慨鏀瑰畠锛涜鍙瘉鐗堟湰涓虹 2 鐗堬紝鎴栵紙鏍规嵁鎮ㄧ殑閫夋嫨锛変换浣曟洿楂樼増鏈€?
鏈▼搴忕殑鍒嗗彂甯屾湜瀹冩湁鐢紝浣嗘病鏈変换浣曟媴淇濓紱鐢氳嚦娌℃湁瀵归€傞攢鎬ф垨鐗瑰畾鐢ㄩ€旈€傜敤鎬х殑
闅愬惈鎷呬繚銆傛湁鍏虫洿澶氱粏鑺傦紝璇峰弬闃?GNU 閫氱敤鍏叡璁稿彲璇併€?
鎮ㄥ簲璇ュ凡缁忛殢鏈▼搴忔敹鍒颁簡涓€浠?GNU 閫氱敤鍏叡璁稿彲璇侊紱濡傛灉娌℃湁锛岃鍐欎俊缁欒嚜鐢辫蒋浠?鍩洪噾浼氾紝鍦板潃锛欼nc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA

濡傛灉鎮ㄩ渶瑕佽仈绯绘垜锛堜綔鑰咃級锛屽彲浠ラ€氳繃鐢靛瓙閭欢鈥斺€斿皢鎮ㄧ殑娑堟伅鍙戦€佸埌
<vojtech@suse.cz>鈥斺€旀垨鑰呴€氳繃绾歌川閭欢锛歏ojtech Pavlik锛孶citelska 1576,
Prague 8, 182 00 Czech Republic

涓烘柟渚胯捣瑙侊紝GNU 閫氱敤鍏叡璁稿彲璇佺 2 鐗堢殑鏂囨湰宸插寘鍚湪杞欢鍖呬腑锛氳鍙傞槄
COPYING 鏂囦欢銆?
#### 1. 鐢ㄦ硶

drivers/usb/class/cdc-acm.c 椹卞姩閫傜敤浜庣鍚堥€氱敤涓茶鎬荤嚎閫氫俊璁惧绫绘娊璞℃帶鍒舵ā鍨?锛圲SB CDC ACM锛夎鑼冪殑 USB 璋冨埗瑙ｈ皟鍣ㄥ拰 USB ISDN 缁堢閫傞厤鍣ㄣ€?
璁稿璋冨埗瑙ｈ皟鍣ㄩ兘绗﹀悎锛屼互涓嬫槸鎴戞墍鐭ラ亾鐨勫瀷鍙峰垪琛細

 - 3Com OfficeConnect 56k
 - 3Com Voice FaxModem Pro
 - 3Com Sportster
 - MultiTech MultiModem 56k
 - Zoom 2986L FaxModem
 - Compaq 56k FaxModem
 - ELSA Microlink 56k

鎴戠煡閬撴湁涓€娆?ISDN TA 鍙互涓?acm 椹卞姩閰嶅悎浣跨敤锛?
 - 3Com USR ISDN Pro TA

涓€浜涙墜鏈轰篃閫氳繃 USB 杩炴帴銆傛垜鐭ラ亾浠ヤ笅鎵嬫満鍙互宸ヤ綔锛?
 - SonyEricsson K800i

閬楁喚鐨勬槸锛岃澶氳皟鍒惰В璋冨櫒鍜屽ぇ澶氭暟 ISDN TA 浣跨敤涓撴湁鎺ュ彛锛屽洜姝ゆ棤娉曚笌璇ラ┍鍔ㄩ厤鍚?宸ヤ綔銆傝喘涔板墠璇风‘璁ゆ槸鍚︾鍚?ACM 瑙勮寖銆?
```
	usbcore.ko
	uhci-hcd.ko ohci-hcd.ko or ehci-hcd.ko
	cdc-acm.ko
```

涔嬪悗锛岃皟鍒惰В璋冨櫒搴斿綋鍙璁块棶銆傛偍搴斿綋鑳藉浣跨敤 minicom銆乸pp 鍜?mgetty 鏉ユ搷浣滃畠浠€?
#### 2. 楠岃瘉鏄惁宸ヤ綔


绗竴姝ュ簲褰撴鏌?/sys/kernel/debug/usb/devices锛屽叾鍐呭搴斿綋绫讳技濡備笅

```
  T:  Bus=01 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12  MxCh= 2
  B:  Alloc=  0/900 us ( 0%), #Int=  0, #Iso=  0
  D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
  P:  Vendor=0000 ProdID=0000 Rev= 0.00
  S:  Product=USB UHCI Root Hub
  S:  SerialNumber=6800
  C:* #Ifs= 1 Cfg#= 1 Atr=40 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
  E:  Ad=81(I) Atr=03(Int.) MxPS=   8 Ivl=255ms
  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=01 Dev#=  2 Spd=12  MxCh= 0
  D:  Ver= 1.00 Cls=02(comm.) Sub=00 Prot=00 MxPS= 8 #Cfgs=  2
  P:  Vendor=04c1 ProdID=008f Rev= 2.07
  S:  Manufacturer=3Com Inc.
  S:  Product=3Com U.S. Robotics Pro ISDN TA
  S:  SerialNumber=UFT53A49BVT7
  C:  #Ifs= 1 Cfg#= 1 Atr=60 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 3 Cls=ff(vend.) Sub=ff Prot=ff Driver=acm
  E:  Ad=85(I) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=04(O) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=81(I) Atr=03(Int.) MxPS=  16 Ivl=128ms
  C:* #Ifs= 2 Cfg#= 2 Atr=60 MxPwr=  0mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  E:  Ad=81(I) Atr=03(Int.) MxPS=  16 Ivl=128ms
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
  E:  Ad=85(I) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
  E:  Ad=04(O) Atr=02(Bulk) MxPS=  64 Ivl=  0ms
```

杩欎笁琛岋紙浠ュ強 Cls= 'comm' 鍜?'data' 绫伙級鐨勫瓨鍦ㄥ緢閲嶈锛屽畠琛ㄧず杩欐槸涓€涓?ACM 璁惧銆?Driver=acm 琛ㄧず acm 椹卞姩姝ｈ鐢ㄤ簬璇ヨ澶囥€傚鏋滄偍鍙湅鍒?Cls=ff(vend.)锛岄偅涔堟偍灏?
```
  D:  Ver= 1.00 Cls=02(comm.) Sub=00 Prot=00 MxPS= 8 #Cfgs=  2
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
```

```
  usb.c: USB new device connect, assigned device number 2
  usb.c: kmalloc IF c7691fa0, numif 1
  usb.c: kmalloc IF c7b5f3e0, numif 2
  usb.c: skipped 4 class/vendor specific interface descriptors
  usb.c: new device strings: Mfr=1, Product=2, SerialNumber=3
  usb.c: USB device number 2 default language ID 0x409
  Manufacturer: 3Com Inc.
  Product: 3Com U.S. Robotics Pro ISDN TA
  SerialNumber: UFT53A49BVT7
  acm.c: probing config 1
  acm.c: probing config 2
  ttyACM0: USB ACM device
  acm.c: acm_control_msg: rq: 0x22 val: 0x0 len: 0x0 result: 0
  acm.c: acm_control_msg: rq: 0x20 val: 0x0 len: 0x7 result: 7
  usb.c: acm driver claimed interface c7b5f3e0
  usb.c: acm driver claimed interface c7b5f3f8
  usb.c: acm driver claimed interface c7691fa0
```

濡傛灉涓€鍒囩湅璧锋潵姝ｅ父锛屽惎鍔?minicom 骞舵妸瀹冭缃负涓?ttyACM 璁惧閫氫俊锛岀劧鍚庤瘯鐫€杈撳叆
'at'銆傚鏋滃畠杩斿洖 'OK'锛岄偅涔堜竴鍒囬兘鍦ㄦ甯稿伐浣溿€?