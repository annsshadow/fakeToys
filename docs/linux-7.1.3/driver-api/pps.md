
## PPS - Pulse Per Second


Copyright (C) 2007 Rodolfo Giometti <giometti@enneenne.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.



### 姒傝堪


LinuxPPS 鎻愪緵浜嗕竴涓紪绋嬫帴鍙ｏ紙API锛夛紝鐢ㄤ簬鍦ㄧ郴缁熶腑瀹氫箟澶氫釜 PPS 婧愩€?
PPS 鎰忎负鈥滄瘡绉掕剦鍐诧紙pulse per second锛夆€濓紝PPS 婧愬氨鏄竴涓瘡绉掓彁渚涗竴涓珮绮惧害淇″彿鐨?璁惧锛屽簲鐢ㄧ▼搴忓彲浠ュ埄鐢ㄥ畠鏉ヨ皟鏁寸郴缁熸椂閽熸椂闂淬€?
涓€涓?PPS 婧愬彲浠ヨ繛鎺ュ埌涓茶绔彛锛堥€氬父鏄帴鍒版暟鎹浇娉㈡娴?Data Carrier Detect 寮曡剼锛夈€?骞惰绔彛锛圓CK 寮曡剼锛夛紝鎴栬€呮煇涓笓鐢?CPU 鐨?GPIO锛堣繖鍦ㄥ祵鍏ュ紡绯荤粺涓緢甯歌锛夛紱浣嗗湪姣忕
鎯呭喌涓嬶紝褰撲竴涓柊鐨勮剦鍐插埌杈炬椂锛岀郴缁熼兘蹇呴』涓哄畠鎵撲笂鏃堕棿鎴筹紙timestamp锛夊苟璁板綍涓嬫潵渚涚敤鎴?绌洪棿浣跨敤銆?
甯歌鐨勭敤娉曟槸灏?NTPD 浣滀负鐢ㄦ埛绌洪棿绋嬪簭锛岄厤鍚堜竴涓?GPS 鎺ユ敹鍣ㄤ綔涓?PPS 婧愶紝浠庤€岃幏寰椾笌 UTC
淇濇寔浜氭绉掔骇鍚屾鐨勫涓婃椂閽熸椂闂达紙wallclock-time锛夈€?

### RFC 鐩稿叧鑰冮噺


鍦ㄤ负 PPS API 瀹炵幇 RFC 2783 鎵€瀹氫箟鐨勬帴鍙ｃ€佸苟浣跨敤宓屽叆寮?CPU 鐨?GPIO 寮曡剼浣滀负杩炴帴鍒颁俊鍙风殑
鐗╃悊閾捐矾鏃讹紝鎴戦亣鍒颁簡涓€涓洿娣卞眰鐨勯棶棰橈細

   At startup it needs a file descriptor as argument for the function
   time_pps_create().

杩欐剰鍛崇潃璇ユ簮蹇呴』鏈変竴涓?/dev/... 鏉＄洰銆傚浜庝覆琛岀鍙ｅ拰骞惰绔彛鏉ヨ杩欎釜鍋囪鏄彲浠ョ殑锛屽洜涓?闄や簡锛堬紒锛夐噰闆嗘椂闂存埑锛堣繖鏄?PPS API 鐨勬牳蹇冧换鍔★級涔嬪锛屼綘杩樺彲浠ュ湪杩欎簺绔彛涓婂仛涓€浜涙湁鐢ㄧ殑
浜嬫儏銆備絾瀵逛簬鍗曚竴鐢ㄩ€旂殑 GPIO 绾匡紝杩欎釜鍋囪灏变笉鎴愮珛浜嗐€傚湪杩欑鎯呭喌涓嬶紝鍗充究鍩烘湰鐨勬枃浠剁浉鍏?鍔熻兘锛堝 read() 涓?write()锛変篃姣棤鎰忎箟锛屼笉搴旀垚涓轰娇鐢?PPS API 鐨勫墠鎻愭潯浠躲€?
濡傛灉浣犺€冭檻鍒?PPS 婧愬苟涓嶆€绘槸涓?GPS 鏁版嵁婧愮浉杩烇紝杩欎釜闂灏卞彲浠ョ畝鍗曞湴瑙ｅ喅銆?
鍥犳浣犵殑绋嬪簭搴斿綋妫€鏌?GPS 鏁版嵁婧愶紙渚嬪涓茶绔彛锛夋槸鍚︿篃鏄竴涓?PPS 婧愶紱濡傛灉涓嶆槸锛屽畠浠?搴斿綋鎻愪緵鎵撳紑鍙︿竴涓澶囦綔涓?PPS 婧愮殑鍙兘鎬с€?
鍦?LinuxPPS 涓紝PPS 婧愬氨鏄櫘閫氱殑瀛楃璁惧锛岄€氬父鏄犲皠鍒?/dev/pps0銆?dev/pps1 绛夋枃浠躲€?

### 浣跨敤 USB 杞覆鍙ｈ澶囩殑 PPS


鍙互浠?USB 杞覆鍙ｈ澶囦笂鑾峰彇 PPS銆備笉杩囷紝浣犲簲璇ヨ€冭檻鍒?USB 鍗忚鏍堝紩鍏ョ殑寤惰繜涓庢姈鍔ㄣ€傜敤鎴锋姤鍛?閫氳繃 USB 涓?PPS 鍚屾鏃舵椂閽熶笉绋冲畾锛屽ぇ绾﹀湪 卤1ms銆備娇鐢?USB 2.0 鏃讹紝鎶栧姩鍙兘闄嶄綆鍒?125 寰
鐨勯噺绾с€?
杩欏浜庝娇鐢?NTP 杩涜鏃堕棿鏈嶅姟鍣ㄥ悓姝ュ彲鑳芥槸鍚堥€傜殑锛屽洜涓哄畠鏈変笅閲囨牱锛坲ndersampling锛夊拰绠楁硶銆?
濡傛灉浣犵殑璁惧娌℃湁鎶ュ憡 PPS锛屼綘鍙互妫€鏌ュ叾椹卞姩鏄惁鏀寔璇ュ姛鑳姐€傚ぇ澶氭暟鎯呭喌涓嬶紝浣犲彧闇€瑕佸湪妫€鏌?DCD 鐘舵€佷箣鍚庢坊鍔犲 usb_serial_handle_dcd_change 鐨勮皟鐢紙鍙傝 ch341 涓?pl2303 绀轰緥锛夈€?

### 缂栫爜绀轰緥


瑕佸皢涓€涓?PPS 婧愭敞鍐屽埌鍐呮牳涓紝浣犲簲璇ュ畾涔変竴涓?struct
```

    static struct pps_source_info pps_ktimer_info = {
	    .name         = "ktimer",
	    .path         = "",
	    .mode         = PPS_CAPTUREASSERT | PPS_OFFSETASSERT |
			    PPS_ECHOASSERT |
			    PPS_CANWAIT | PPS_TSFMT_TSPEC,
	    .echo         = pps_ktimer_echo,
	    .owner        = THIS_MODULE,
    };

```
鐒跺悗璋冪敤鍑芥暟 pps_register_source()锛屽湪浣犵殑
```

    source = pps_register_source(&pps_ktimer_info,
			PPS_CAPTUREASSERT | PPS_OFFSETASSERT);

```
```

  int pps_register_source(struct pps_source_info *info, int default_params)

```
鍏朵腑 鈥渋nfo鈥?鏄寚鍚戞弿杩版煇涓壒瀹?PPS 婧愮殑缁撴瀯鐨勬寚閽堬紝鈥渄efault_params鈥?鍛婅瘔绯荤粺璇ヨ澶囩殑鍒濆
榛樿鍙傛暟搴旇鏄粈涔堬紙鏄剧劧锛岃繖浜涘弬鏁板繀椤绘槸鎻忚堪椹卞姩鑳藉姏鐨?struct pps_source_info 涓墍瀹氫箟鐨?鍙傛暟鐨勪竴涓瓙闆嗭級銆?
涓€鏃︿綘灏嗕竴涓柊 PPS 婧愭敞鍐屽埌绯荤粺涓紝灏卞彲浠ュ彂鍑轰竴涓?assert 浜嬩欢锛堜緥濡傚湪涓柇澶勭悊渚嬬▼涓級
```

    pps_event(source, &ts, PPS_CAPTUREASSERT, ptr)

```
鍏朵腑 鈥渢s鈥?鏄簨浠剁殑鏃堕棿鎴炽€?
鍚屼竴涓嚱鏁拌繕鍙互杩愯鎵€瀹氫箟鐨?echo 鍑芥暟锛坧ps_ktimer_echo()锛屽悜瀹冧紶鍏?鈥減tr鈥?鎸囬拡锛夛紝濡傛灉鐢ㄦ埛
瑕佹眰杩欎箞鍋氱殑璇濃€︹€︾瓑绛夈€?
绀轰緥浠ｇ爜璇峰弬瑙?drivers/pps/clients/pps-ktimer.c 鏂囦欢銆?

### SYSFS 鏀寔


```

   $ ls /sys/class/pps/
   pps0/  pps1/  pps2/

```
姣忎釜鐩綍閮芥槸绯荤粺涓畾涔夌殑涓€涓?PPS 婧愮殑 ID锛屼互鍙?```

   $ ls -F /sys/class/pps/pps0/
   assert     dev        mode       path       subsystem@
   clear      echo       name       power/     uevent


```
鍦ㄦ瘡涓?鈥渁ssert鈥?涓?鈥渃lear鈥?鏂囦欢涓紝浣犲彲浠ユ壘鍒版椂闂存埑鍜屼竴涓?```

   $ cat /sys/class/pps/pps0/assert
   1170026870.983207967#8

```
鍏朵腑 鈥?鈥?涔嬪墠鐨勬槸浠ョ涓哄崟浣嶇殑鏃堕棿鎴筹紱涔嬪悗鐨勬槸搴忓垪鍙枫€傚叾瀹冩枃浠跺寘鎷細

 - echo锛氭姤鍛婅 PPS 婧愭槸鍚﹀叿鏈?echo 鍑芥暟锛?
 - mode锛氭姤鍛婂彲鐢ㄧ殑 PPS 宸ヤ綔妯″紡锛?
 - name锛氭姤鍛?PPS 婧愮殑鍚嶇О锛?
 - path锛氭姤鍛?PPS 婧愮殑璁惧璺緞锛屽嵆璇?PPS 婧愭墍杩炴帴鐨勮澶囷紙濡傛灉瀛樺湪锛夈€?

### 娴嬭瘯 PPS 鏀寔


鍗充究娌℃湁鐗瑰畾纭欢锛屼綘涔熷彲浠ヤ负浜嗘祴璇?PPS 鏀寔鑰屼娇鐢?pps-ktimer 椹卞姩锛堝弬瑙?PPS 閰嶇疆鑿滃崟涓殑
瀹㈡埛绔瓙灏忚妭锛変互鍙婁綘鐨勫彂琛岀増涓?pps-tools 杞欢鍖呫€乭ttp://linuxpps.org 鎴?https://github.com/redlab-i/pps-tools 涓彁渚涚殑鐢ㄦ埛绌洪棿宸ュ叿銆?
涓€鏃︿綘鍚敤浜?pps-ktimer 鐨勭紪璇戯紝鍙渶 modprobe 瀹冿紙濡傛灉
```

   # modprobe pps-ktimer

```
```

   $ ./ppstest /dev/pps1
   trying PPS source "/dev/pps1"
   found PPS source "/dev/pps1"
   ok, found 1 source(s), now start fetching data...
   source 0 - assert 1186592699.388832443, sequence: 364 - clear  0.000000000, sequence: 0
   source 0 - assert 1186592700.388931295, sequence: 365 - clear  0.000000000, sequence: 0
   source 0 - assert 1186592701.389032765, sequence: 366 - clear  0.000000000, sequence: 0

```
璇锋敞鎰忥紝瑕佺紪璇戠敤鎴风┖闂寸▼搴忥紝浣犻渶瑕?timepps.h 鏂囦欢銆傝鏂囦欢鍦ㄤ笂杩?pps-tools 浠撳簱涓彲浠ユ壘鍒般€?

### 鍙戠敓鍣紙Generators锛?

鏈夋椂涓嶄粎闇€瑕佹崟鑾?PPS 淇″彿锛岃繕闇€瑕佷骇鐢熷畠浠€備緥濡傦紝杩愯涓€涓垎甯冨紡浠跨湡锛屽畠瑕佹眰璁＄畻鏈虹殑鏃堕挓
琚潪甯哥揣瀵嗗湴鍚屾銆?
涓烘锛屽鍔犱簡 pps-gen 绫汇€傚彲浠ラ€氳繃瀹氫箟 struct pps_gen_source_info 鏉ュ悜鍐呮牳娉ㄥ唽 PPS 鍙戠敓鍣紝
濡備笅
```

    static const struct pps_gen_source_info pps_gen_dummy_info = {
            .use_system_clock       = true,
            .get_time               = pps_gen_dummy_get_time,
            .enable                 = pps_gen_dummy_enable,
    };

```
鍏朵腑 use_system_clock 琛ㄦ槑璇ュ彂鐢熷櫒鏄惁浣跨敤绯荤粺鏃堕挓鏉ヤ骇鐢熻剦鍐诧紝杩樻槸浣跨敤鏉ヨ嚜澶栬璁惧鏃堕挓鐨?鑴夊啿銆傛柟娉?get_time() 鐢ㄤ簬鏌ヨ瀛樺偍鍦ㄥ彂鐢熷櫒鏃堕挓涓殑鏃堕棿锛岃€屾柟娉?enable() 鐢ㄤ簬鍚敤鎴栫鐢?PPS 鑴夊啿鐨勪骇鐢熴€?
鐒跺悗鍦ㄤ綘鐨勫垵濮嬪寲渚嬬▼涓皟鐢ㄥ嚱鏁?pps_gen_register_source()锛屽涓嬫墍绀猴紝浼氬垱寤轰竴涓柊鐨勫彂鐢熷櫒
```

    pps_gen = pps_gen_register_source(&pps_gen_dummy_info);

```
### 鍙戠敓鍣?SYSFS 鏀寔


```

    $ ls /sys/class/pps-gen/
    pps-gen0/  pps-gen1/  pps-gen2/

```
姣忎釜鐩綍閮芥槸绯荤粺涓畾涔夌殑涓€涓?PPS 鍙戠敓鍣ㄧ殑 ID锛屼互鍙?```

    $ ls -F /sys/class/pps-gen/pps-gen0/
    dev  enable  name  power/  subsystem@  system  time  uevent

```
```

    $ echo 1 > /sys/class/pps-gen/pps-gen0/enable

```
### 骞惰绔彛鍙戠敓鍣?

涓€绉嶅仛娉曟槸鍙戞槑鏌愪簺澶嶆潅鐨勭‖浠舵柟妗堬紝浣嗚繖鏃㈡病蹇呰涔熸湭蹇呭垝绠椼€備究瀹滅殑鍋氭硶鏄湪鍏朵腑涓€鍙拌绠楁満
锛堜富鑺傜偣锛宮aster锛変笂鍔犺浇涓€涓?PPS 鍙戠敓鍣紝鍦ㄥ叾瀹冭绠楁満锛堜粠鑺傜偣锛宻lave锛変笂鍔犺浇 PPS 瀹㈡埛绔紝
骞朵娇鐢ㄩ潪甯哥畝鍗曠殑绾跨紗锛屼緥濡傞€氳繃骞惰绔彛鏉ヤ紶閫佷俊鍙枫€?
```

	pin	name	master      slave
	1	STROBE	  *------     *
	2	D0	  *     |     *
	3	D1	  *     |     *
	4	D2	  *     |     *
	5	D3	  *     |     *
	6	D4	  *     |     *
	7	D5	  *     |     *
	8	D6	  *     |     *
	9	D7	  *     |     *
	10	ACK	  *     ------*
	11	BUSY	  *           *
	12	PE	  *           *
	13	SEL	  *           *
	14	AUTOFD	  *           *
	15	ERROR	  *           *
	16	INIT	  *           *
	17	SELIN	  *           *
	18-25	GND	  *-----------*

```
璇锋敞鎰忥紝骞惰绔彛涓柇鍙湪鐢遍珮鍒颁綆鐨勮烦鍙樻椂瑙﹀彂锛屽洜姝ゅ畠琚敤浜?PPS 鐨?assert 杈规部銆侾PS 鐨?clear
杈规部鍙兘閫氳繃鍦ㄤ腑鏂鐞嗙▼搴忎腑浣跨敤杞锛坧olling锛夋潵纭畾锛岃繖瀹為檯涓婂彲浠ュ仛寰楁洿绮剧‘锛屽洜涓轰腑鏂?澶勭悊鐨勫欢杩熷彲鑳界浉褰撳ぇ涓旈殢鏈恒€傚洜姝ゅ綋鍓嶇殑 parport PPS 鍙戠敓鍣ㄥ疄鐜帮紙pps_gen_parport 妯″潡锛夊€惧悜浜?浣跨敤 clear 杈规部鏉ヨ繘琛屾椂闂村悓姝ャ€?
clear 杈规部鐨勮疆璇㈡槸鍦ㄥ叧闂腑鏂殑鎯呭喌涓嬭繘琛岀殑锛屽洜姝ゆ渶濂藉皢 assert 涓?clear 杈规部涔嬮棿鐨勫欢杩熼€夊緱
灏藉彲鑳藉皬锛屼互闄嶄綆绯荤粺寤惰繜銆備絾濡傛灉澶皬锛屼粠鑺傜偣灏嗘棤娉曟崟鑾?clear 杈规部鐨勮烦鍙樸€?0 寰鐨勯粯璁ゅ€煎湪
澶у鏁版儏鍐典笅搴旇瓒冲濂姐€傝寤惰繜鍙互浣跨敤 'delay' pps_gen_parport 妯″潡鍙傛暟鏉ラ€夋嫨銆?

### Intel Timed I/O PPS 淇″彿鍙戠敓鍣?

Intel Timed I/O 鏄竴涓珮绮惧害璁惧锛屽嚭鐜板湪 2019 骞村強鏇存柊鐨?Intel CPU 涓婏紝鍙互浜х敓 PPS 淇″彿銆?
Timed I/O 涓庣郴缁熸椂闂撮兘鐢卞悓涓€涓‖浠舵椂閽熼┍鍔ㄣ€備俊鍙风殑鐢熸垚绮惧害绾︿负 20 绾崇銆傜敓鎴愮殑 PPS 淇″彿鐢ㄤ簬
灏嗗閮ㄨ澶囦笌绯荤粺鏃堕挓鍚屾銆備緥濡傦紝瀹冨彲浠ョ敤鏉ヤ笌鎺ユ敹鐢?Timed I/O 璁惧鐢熸垚鐨?PPS 淇″彿鐨勮澶囧叡浜?浣犵殑鏃堕挓銆傛湁涓撶敤鐨?Timed I/O 寮曡剼鐢ㄤ簬灏?PPS 淇″彿浼犻€佸埌澶栭儴璁惧銆?
灏?Intel Timed I/O 鐢ㄤ綔 PPS 鍙戠敓鍣細

```

        $echo 1 > /sys/class/pps-gen/pps-genx/enable

```
```

        $echo 0 > /sys/class/pps-gen/pps-genx/enable

```
