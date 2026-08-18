## Linux Gadget 涓茶椹卞姩 v2.0


11/20/2004

锛?008-05-08 鏇存柊鑷?v2.3锛?

### 璁稿彲璇佷笌鍏嶈矗澹版槑

鏈▼搴忔槸鑷敱杞欢锛涗綘鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇佹潯娆句笅閲嶆柊鍒嗗彂鍜?鎴栦慨鏀瑰畠锛涘彲浠ユ槸璁稿彲璇佺殑绗?2 鐗堬紝鎴栬€咃紙鐢变綘閫夋嫨锛変换浣曟洿鏅氱殑鐗堟湰銆?
鏈▼搴忓垎鍙戠殑鐩殑鏄笇鏈涘畠鏈夌敤锛屼絾娌℃湁浠讳綍鎷呬繚锛涚敋鑷虫病鏈夊閫傞攢鎬ф垨鐗瑰畾鐢ㄩ€旈€傜敤鎬х殑榛樼ず鎷呬繚銆傝瑙?GNU 閫氱敤鍏叡璁稿彲璇併€?
浣犲簲璇ュ凡缁忛殢鏈▼搴忔敹鍒颁簡涓€浠?GNU 閫氱敤鍏叡璁稿彲璇佺殑鍓湰锛涘鏋滄病鏈夛紝璇峰啓淇＄粰鑷敱杞欢鍩洪噾浼氾紝鍦板潃锛?9 Temple Place, Suite 330, Boston, MA 02111-1307 USA銆?
鏈枃妗ｄ互鍙?gadget 涓茶椹卞姩鏈韩鐗堟潈褰?(C) 2004 Al Borchers (alborchers@steinerpoint.com) 鎵€鏈夈€?
濡傛灉浣犲鏈┍鍔ㄦ湁鐤戦棶銆侀棶棰樻垨寤鸿锛岃鑱旂郴 Al Borchers锛歛lborchers@steinerpoint.com銆?

### 鍏堝喅鏉′欢

gadget 涓茶椹卞姩鏈夐€傜敤浜?2.4 Linux 鍐呮牳鐨勭増鏈紝浣嗘湰鏂囨。鍋囧畾浣犳鍦?2.6 Linux 鍐呮牳涓娇鐢?2.3 鎴栨洿楂樼増鏈殑 gadget 涓茶椹卞姩銆?
鏈枃妗ｅ亣瀹氫綘鐔熸倝 Linux 鍜?Windows锛屽苟涓旂煡閬撳浣曢厤缃拰鏋勫缓 Linux 鍐呮牳銆佽繍琛屾爣鍑嗗伐鍏枫€佷娇鐢?minicom 鍜?HyperTerminal锛屼互鍙婁娇鐢?USB 鍜屼覆琛岃澶囥€傚畠杩樺亣瀹氫綘灏?Linux gadget 鍜?usb 椹卞姩閰嶇疆涓烘ā鍧椼€?
鍦ㄩ┍鍔ㄧ殑 2.3 鐗堟湰涓紝涓昏澶囧彿鍜屾璁惧鍙蜂笉鍐嶉潤鎬佸畾涔夈€備綘鐨?Linux 绯荤粺搴斿綋鎶?sysfs 鎸傝浇鍦?/sys锛屽苟浣跨敤 鈥渕dev鈥濓紙Busybox 涓級鎴?鈥渦dev鈥?鏉ュ垱寤轰笌 sysfs /sys/class/tty 鏂囦欢鍖归厤鐨?/dev 鑺傜偣銆?


### 姒傝堪

gadget 涓茶椹卞姩鏄竴涓?Linux USB gadget 椹卞姩锛屽嵆涓€涓?USB 璁惧渚ч┍鍔ㄣ€傚畠杩愯鍦ㄥ叿澶?USB 璁惧渚х‖浠剁殑 Linux 绯荤粺涓婏紱渚嬪 PDA銆佸祵鍏ュ紡 Linux 绯荤粺锛屾垨甯︽湁 USB 寮€鍙戞澘鐨?PC銆?
gadget 涓茶椹卞姩閫氳繃 USB 涓庝竴涓?CDC ACM 椹卞姩閫氫俊
```

   Host
   --------------------------------------
  | Host-Side   CDC ACM       USB Host   |
  | Operating |   or        | Controller |   USB
  | System    | Generic USB | Driver     |--------
  | (Linux or | Serial      | and        |        |
  | Windows)    Driver        USB Stack  |        |
   --------------------------------------         |
                                                  |
                                                  |
                                                  |
   Gadget                                         |
   --------------------------------------         |
  | Gadget                   USB Periph. |        |
  | Device-Side |  Gadget  | Controller  |        |
  | Linux       |  Serial  | Driver      |--------
  | Operating   |  Driver  | and         |
  | System                   USB Stack   |
   --------------------------------------

```
鍦ㄨ澶囦晶 Linux 绯荤粺涓婏紝gadget 涓茶椹卞姩鐪嬭捣鏉ュ儚涓€涓覆琛岃澶囥€?
鍦ㄤ富鏈轰晶绯荤粺涓婏紝gadget 涓茶璁惧鐪嬭捣鏉ュ儚鏄竴涓鍚?CDC ACM 鏍囧噯鐨勭被璁惧锛屾垨鑰呮槸涓€涓甫鏈?bulk in 鍜?bulk out 绔偣鐨勭畝鍗曞巶鍟嗙壒瀹氳澶囷紝骞朵笖瀹冪殑澶勭悊鏂瑰紡涓庡叾浠栦覆琛岃澶囩被浼笺€?
涓绘満渚ч┍鍔ㄥ彲浠ユ槸浠绘剰绗﹀悎 ACM 鏍囧噯鐨勯┍鍔紝鎴栬€呬换浣曡兘澶熶笌甯︽湁绠€鍗?bulk in/out 鎺ュ彛鐨勮澶囬€氫俊鐨勯┍鍔ㄣ€侴adget 涓茶宸茬粡鐢?Linux ACM 椹卞姩銆乄indows usbser.sys ACM 椹卞姩浠ュ強 Linux USB 閫氱敤涓茶椹卞姩娴嬭瘯杩囥€?
鍦?gadget 涓茶椹卞姩鍜屼富鏈轰晶 ACM 鎴栭€氱敤涓茶椹卞姩杩愯鐨勬儏鍐典笅锛屼綘搴旇鑳藉鍦ㄤ富鏈哄拰 gadget 渚х郴缁熶箣闂撮€氫俊锛屽氨鍍忓畠浠€氳繃涓茶鐢电紗杩炴帴涓€鏍枫€?
gadget 涓茶椹卞姩鍙彁渚涚畝鍗曠殑涓嶅彲闈犳暟鎹€氫俊銆傚畠杩樻湭鑳藉鐞嗘祦鎺ф垨鏅€氫覆琛岃澶囩殑璁稿鍏朵粬鐗规€с€?

### 瀹夎 Gadget 涓茶椹卞姩

瑕佷娇鐢?gadget 涓茶椹卞姩锛屼綘蹇呴』灏?Linux gadget 渚у唴鏍搁厤缃负 鈥淪upport for USB Gadgets鈥濄€佷竴涓?鈥淯SB Peripheral Controller鈥濓紙渚嬪 net2280锛夛紝浠ュ強 鈥淪erial Gadget鈥?椹卞姩銆傞厤缃唴鏍告椂杩欎簺閮藉垪鍦?鈥淯SB Gadget Support鈥?涓嬨€傜劧鍚庨噸鏂版瀯寤哄苟瀹夎鍐呮牳鎴栨ā鍧椼€?
鐒跺悗浣犲繀椤诲姞杞?gadget 涓茶椹卞姩銆傝灏嗗叾浣滀负
```
  modprobe g_serial

```
```
  modprobe g_serial use_acm=0

```
鍔犺浇锛岃繖涔熶細鑷姩鍔犺浇搴曞眰鐨?gadget 澶栬鎺у埗鍣ㄩ┍鍔ㄣ€傛瘡娆￠噸鍚?gadget 渚?Linux 绯荤粺鏃堕兘蹇呴』杩欐牱鍋氥€傚鏋滈渶瑕侊紝浣犲彲浠ユ妸瀹冨姞鍏ュ惎鍔ㄨ剼鏈€?
浣犵殑绯荤粺搴斿綋浣跨敤 mdev锛堟潵鑷?busybox锛夋垨 udev 鏉ュ垱寤鸿澶囪妭鐐广€傚湪杩欎釜 gadget 椹卞姩璁剧疆濂戒箣鍚庯紝浣犲簲璇ョ湅鍒?```

  # ls -l /dev/ttyGS0 | cat
  crw-rw----    1 root     root     253,   0 May  8 14:10 /dev/ttyGS0
  #

```
娉ㄦ剰锛屼富璁惧鍙凤紙涓婇潰鐨?253锛夋槸绯荤粺鐗瑰畾鐨勩€傚鏋滀綘闇€瑕佹墜鍔ㄥ垱寤?/dev 鑺傜偣锛岃浣跨敤鐨勬纭彿鐮佷細鍦?/sys/class/tty/ttyGS0/dev 鏂囦欢涓€?
濡傛灉浣犺緝鏃╅摼鎺ヨ繖涓?gadget 椹卞姩锛堢敋鑷冲彲鑳介潤鎬侀摼鎺ワ級锛屼綘鍙兘鎯宠缃竴涓?/etc/inittab 鏉＄洰鏉ュ湪涓婇潰杩愯 鈥済etty鈥濄€?dev/ttyGS0 杩欎竴琛屽簲褰撳儚澶у鏁板叾浠栦覆琛岀鍙ｄ竴鏍峰伐浣溿€?

濡傛灉 gadget 涓茶浣滀负 ACM 璁惧鍔犺浇锛屼綘浼氬湪涓绘満渚т娇鐢?Windows 鎴?Linux ACM 椹卞姩銆傚鏋?gadget 涓茶浣滀负 bulk in/out 璁惧鍔犺浇锛屼綘浼氬湪涓绘満渚т娇鐢?Linux 閫氱敤涓茶椹卞姩銆傝鎸夌収涓嬮潰鐩稿簲鐨勮鏄庢潵瀹夎涓绘満渚ч┍鍔ㄣ€?

### 瀹夎 Windows 涓绘満 ACM 椹卞姩

瑕佷娇鐢?Windows ACM 椹卞姩锛屼綘蹇呴』鎷ユ湁 鈥渓inux-cdc-acm.inf鈥?鏂囦欢锛堥殢鏈枃妗ｄ竴璧锋彁渚涳級锛屽畠鏀寔鎵€鏈夎繎鏈熺増鏈殑 Windows銆?
褰?gadget 涓茶椹卞姩宸插姞杞姐€佸苟涓?USB 璁惧閫氳繃 USB 鐢电紗杩炴帴鍒?Windows 涓绘満鏃讹紝Windows 搴斿綋璇嗗埆 gadget 涓茶璁惧骞惰姹傞┍鍔ㄣ€傚憡璇?Windows 鍦ㄥ寘鍚?鈥渓inux-cdc-acm.inf鈥?鏂囦欢鐨勬枃浠跺す涓煡鎵鹃┍鍔ㄣ€?
渚嬪锛屽湪 Windows XP 涓婏紝褰?gadget 涓茶璁惧棣栨鎻掑叆鏃讹紝鈥淔ound New Hardware Wizard鈥?浼氬惎鍔ㄣ€傞€夋嫨 鈥淚nstall from a list or specific location (Advanced)鈥濓紝鐒跺悗鍦ㄤ笅涓€涓睆骞曚笂閫夋嫨 鈥淚nclude this location in the search鈥?骞惰緭鍏ヨ矾寰勶紝鎴栨祻瑙堝埌鍖呭惈 鈥渓inux-cdc-acm.inf鈥?鏂囦欢鐨勬枃浠跺す銆俉indows 浼氭姳鎬?Gadget Serial 椹卞姩娌℃湁閫氳繃 Windows Logo 娴嬭瘯锛屼絾閫夋嫨 鈥淐ontinue anyway鈥?骞跺畬鎴愰┍鍔ㄥ畨瑁呫€?
鍦?Windows XP 涓婏紝鍦?鈥淒evice Manager鈥濓紙浣嶄簬 鈥淐ontrol Panel鈥濄€佲€淪ystem鈥濄€佲€淗ardware鈥?涓嬶級涓睍寮€ 鈥淧orts (COM & LPT)鈥?鏉＄洰锛屼綘搴旇浼氱湅鍒?鈥淕adget Serial鈥?琚垪涓哄叾涓竴涓?COM 绔彛鐨勯┍鍔ㄣ€?
瑕佸嵏杞?Windows XP 涓婄殑 鈥淕adget Serial鈥?椹卞姩锛岃鍦?鈥淒evice Manager鈥?涓彸閿崟鍑?鈥淕adget Serial鈥?鏉＄洰骞堕€夋嫨 鈥淯ninstall鈥濄€?

### 瀹夎 Linux 涓绘満 ACM 椹卞姩

瑕佷娇鐢?Linux ACM 椹卞姩锛屼綘蹇呴』灏?Linux 涓绘満渚у唴鏍搁厤缃负 鈥淪upport for Host-side USB鈥?鍜?鈥淯SB Modem (CDC ACM) support鈥濄€?
涓€鏃?gadget 涓茶椹卞姩宸插姞杞姐€佸苟涓?USB 璁惧閫氳繃 USB 鐢电紗杩炴帴鍒?Linux 涓绘満锛屼富鏈虹郴缁熷簲褰撹瘑鍒?```

  cat /sys/kernel/debug/usb/devices

```
```

  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=02 Dev#=  5 Spd=480 MxCh= 0
  D:  Ver= 2.00 Cls=02(comm.) Sub=00 Prot=00 MxPS=64 #Cfgs=  1
  P:  Vendor=0525 ProdID=a4a7 Rev= 2.01
  S:  Manufacturer=Linux 2.6.8.1 with net2280
  S:  Product=Gadget Serial
  S:  SerialNumber=0
  C:* #Ifs= 2 Cfg#= 2 Atr=c0 MxPwr=  2mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  E:  Ad=83(I) Atr=03(Int.) MxPS=   8 Ivl=32ms
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
  E:  Ad=81(I) Atr=02(Bulk) MxPS= 512 Ivl=0ms
  E:  Ad=02(O) Atr=02(Bulk) MxPS= 512 Ivl=0ms

```
濡傛灉涓绘満渚?Linux 绯荤粺閰嶇疆姝ｇ‘锛孉CM 椹卞姩搴斿綋鑷姩鍔犺浇銆傚懡浠?鈥渓smod鈥?搴斿綋鏄剧ず 鈥渁cm鈥?妯″潡宸插姞杞姐€?

### 瀹夎 Linux 涓绘満閫氱敤 USB 涓茶椹卞姩

瑕佷娇鐢?Linux 閫氱敤 USB 涓茶椹卞姩锛屼綘蹇呴』灏?Linux 涓绘満渚у唴鏍搁厤缃负 鈥淪upport for Host-side USB鈥濄€佲€淯SB Serial Converter support鈥?浠ュ強 鈥淯SB Generic Serial Driver鈥濄€?
涓€鏃?gadget 涓茶椹卞姩宸插姞杞姐€佸苟涓?USB 璁惧閫氳繃 USB 鐢电紗杩炴帴鍒?Linux 涓绘満锛屼富鏈虹郴缁熷簲褰撹瘑鍒?```

  cat /sys/kernel/debug/usb/devices

```
```

  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=02 Dev#=  6 Spd=480 MxCh= 0
  D:  Ver= 2.00 Cls=ff(vend.) Sub=00 Prot=00 MxPS=64 #Cfgs=  1
  P:  Vendor=0525 ProdID=a4a6 Rev= 2.01
  S:  Manufacturer=Linux 2.6.8.1 with net2280
  S:  Product=Gadget Serial
  S:  SerialNumber=0
  C:* #Ifs= 1 Cfg#= 1 Atr=c0 MxPwr=  2mA
  I:  If#= 0 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=serial
  E:  Ad=81(I) Atr=02(Bulk) MxPS= 512 Ivl=0ms
  E:  Ad=02(O) Atr=02(Bulk) MxPS= 512 Ivl=0ms

```
浣犲繀椤诲姞杞?usbserial 椹卞姩骞舵樉寮忚缃叾鍙傛暟
```

  echo 0x0525 0xA4A6 >/sys/bus/usb-serial/drivers/generic/new_id

```
```

  modprobe usbserial vendor=0x0525 product=0xA4A6

```
濡傛灉涓€鍒囨甯革紝usbserial 浼氬湪绯荤粺鏃ュ織涓墦鍗颁竴鏉＄被浼?鈥淕adget Serial converter now attached to ttyUSB0鈥?鐨勬秷鎭€?

### 浣跨敤 Minicom 鎴?HyperTerminal 娴嬭瘯

涓€鏃?gadget 涓茶椹卞姩鍜屼富鏈洪┍鍔ㄩ兘瀹夎濂斤紝骞朵笖 USB 鐢电紗灏?gadget 璁惧杩炴帴鍒颁富鏈猴紝浣犲氨搴旇鑳藉鍦?gadget 鍜屼富鏈虹郴缁熶箣闂撮€氳繃 USB 閫氫俊銆備綘鍙互浣跨敤 minicom 鎴?HyperTerminal 鏉ュ皾璇曘€?
鍦?gadget 渚ц繍琛?鈥渕inicom -s鈥?鏉ラ厤缃竴涓柊鐨?minicom 浼氳瘽銆傚湪 鈥淪erial port setup鈥?涓嬪皢 鈥?dev/ttygserial鈥?璁句负 鈥淪erial Device鈥濄€傚皢娉㈢壒鐜囥€佹暟鎹綅銆佹牎楠屼綅鍜屽仠姝綅璁句负 9600銆?銆乶one 鍜?1鈥斺€旇繖浜涜缃熀鏈棤鍏崇揣瑕併€傚湪 鈥淢odem and dialing鈥?涓嬫竻闄ゆ墍鏈夎皟鍒惰В璋冨櫒鍜屾嫧鍙峰瓧绗︿覆銆?
鍦ㄨ繍琛?ACM 椹卞姩鐨?Linux 涓绘満涓婏紝绫讳技鍦伴厤缃?minicom锛屼絾浣跨敤 鈥?dev/ttyACM0鈥?浣滀负 鈥淪erial Device鈥濄€傦紙濡傛灉浣犺繛鎺ヤ簡鍏朵粬 ACM 璁惧锛岃鐩稿簲鍦版洿鏀硅澶囧悕銆傦級

鍦ㄨ繍琛?USB 閫氱敤涓茶椹卞姩鐨?Linux 涓绘満涓婏紝绫讳技鍦伴厤缃?minicom锛屼絾浣跨敤 鈥?dev/ttyUSB0鈥?浣滀负 鈥淪erial Device鈥濄€傦紙濡傛灉浣犺繛鎺ヤ簡鍏朵粬 USB 涓茶璁惧锛岃鐩稿簲鍦版洿鏀硅澶囧悕銆傦級

鍦?Windows 涓绘満涓婏紝閰嶇疆涓€涓柊鐨?HyperTerminal 浼氳瘽浠ヤ娇鐢ㄥ垎閰嶇粰 Gadget Serial 鐨?COM 绔彛銆傗€淧ort Settings鈥?浼氬湪 HyperTerminal 杩炴帴鍒?gadget 涓茶璁惧鏃惰嚜鍔ㄨ缃紝鍥犳浣犲彲浠ュ皢鍏朵繚鐣欎负榛樿鍊尖€斺€旇繖浜涜缃熀鏈棤鍏崇揣瑕併€?
鍦?gadget 渚ч厤缃苟杩愯 minicom锛屽苟涓斿湪涓绘満渚ч厤缃苟杩愯 minicom 鎴?HyperTerminal 涔嬪悗锛屼綘搴旇鑳藉鍦?gadget 渚у拰涓绘満渚х郴缁熶箣闂存潵鍥炲彂閫佹暟鎹€備綘鍦?gadget 渚х粓绔獥鍙ｄ腑閿叆鐨勪换浣曞唴瀹归兘搴旇鍑虹幇鍦ㄤ富鏈轰晶鐨勭粓绔獥鍙ｄ腑锛屽弽涔嬩害鐒躲€?