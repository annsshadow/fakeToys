
##  USB4 and Thunderbolt锛圲SB4 涓?Thunderbolt锛?

USB4 鏄熀浜?Thunderbolt 3 鍗忚鐨勫叕寮€瑙勮寖锛屼絾鍦ㄥ瘎瀛樺櫒绾у埆绛夋柟闈㈠瓨鍦ㄤ竴浜涘樊寮傘€?杩炴帴绠＄悊鍣紙connection manager锛夋槸涓€涓繍琛屽湪涓绘満璺敱鍣紙涓绘満鎺у埗鍣級涓婄殑瀹炰綋锛?璐熻矗鏋氫妇璺敱鍣ㄥ苟寤虹珛闅ч亾锛坱unnel锛夈€傝繛鎺ョ鐞嗗櫒鏃㈠彲浠ョ敤鍥轰欢瀹炵幇锛屼篃鍙互鐢ㄨ蒋浠?瀹炵幇銆傞€氬父锛孭C 閰嶆湁鐢ㄤ簬 Thunderbolt 3 涓庢棭鏈?USB4 绯荤粺鐨勫浐浠惰繛鎺ョ鐞嗗櫒銆傝€?Apple 绯荤粺鍒欎娇鐢ㄨ蒋浠惰繛鎺ョ鐞嗗櫒锛屽悗鏉ョ殑 USB4 鍚堣璁惧涔熸部鐢ㄦ鏂瑰紡銆?
Linux 鐨?Thunderbolt 椹卞姩鍚屾椂鏀寔涓よ€咃紝骞惰兘鍦ㄨ繍琛屾椂妫€娴嬪簲褰撲娇鐢ㄥ摢绉嶈繛鎺ョ鐞嗗櫒
瀹炵幇銆備负浜嗗畨鍏ㄨ捣瑙侊紝Linux 涓殑杞欢杩炴帴绠＄悊鍣ㄨ繕浼氶€氬憡瀹夊叏绾у埆 `user`锛岃繖鎰忓懗鐫€
榛樿绂佺敤 PCIe 闅ч亾銆備笅闈㈢殑鏂囨。閫傜敤浜庤繖涓ょ瀹炵幇锛屽敮涓€鐨勪緥澶栨槸杞欢杩炴帴绠＄悊鍣ㄥ彧
鏀寔 `user` 瀹夊叏绾у埆锛屽苟涓斿簲褰撻厤鍚堝熀浜?IOMMU 鐨?DMA 淇濇姢涓€璧蜂娇鐢ㄣ€?
### Security levels and how to use them锛堝畨鍏ㄧ骇鍒強鍏朵娇鐢ㄦ柟娉曪級


姝ゅ鍛堢幇鐨勬帴鍙ｅ苟闈為潰鍚戞渶缁堢敤鎴枫€傜浉鍙嶏紝搴斿綋鏈変竴涓敤鎴风┖闂村伐鍏锋潵澶勭悊鎵€鏈夊簳灞?缁嗚妭銆佺淮鎶ゅ凡鎺堟潈璁惧鐨勬暟鎹簱锛屽苟鍦ㄦ湁鏂拌繛鎺ユ椂鎻愮ず鐢ㄦ埛銆?
鍏充簬 Thunderbolt 璁惧 sysfs 鎺ュ彛鐨勬洿澶氱粏鑺傦紝鍙互鍦?Documentation/ABI/testing/sysfs-bus-thunderbolt 涓壘鍒般€?
閭ｄ簺鍙兂杩炴帴浠讳綍璁惧鑰屾棤闇€浠讳綍鎵嬪姩鎿嶄綔鐨勭敤鎴凤紝鍙互灏嗕笅闈㈣繖涓€琛屾坊鍔犲埌
```
锛坲dev 瑙勫垯涓級锛?
  ACTION=="add", SUBSYSTEM=="thunderbolt", ATTR{authorized}=="0", ATTR{authorized}="1"

```
杩欎細鍦ㄨ澶囧嚭鐜版椂鑷姩鎺堟潈鎵€鏈夎澶囥€傜劧鑰岋紝璇疯浣忚繖鏍峰仛浼氱粫杩囧畨鍏ㄧ骇鍒紝骞朵娇绯荤粺
瀹规槗鍙楀埌 DMA 鏀诲嚮銆?
鑷?Intel Falcon Ridge Thunderbolt 鎺у埗鍣ㄨ捣锛屾湁 4 涓彲鐢ㄧ殑瀹夊叏绾у埆銆侷ntel Titan
Ridge 鍙堝鍔犱簡涓€涓畨鍏ㄧ骇鍒紙usbonly锛夈€備箣鎵€浠ラ渶瑕佽繖浜涳紝鏄洜涓烘墍杩炴帴鐨勮澶囧彲浠?鎴愪负 DMA 涓昏澶囷紝浠庤€屽湪娌℃湁 CPU 鍜屾搷浣滅郴缁熺煡鏅撶殑鎯呭喌涓嬭鍙栦富鏈哄唴瀛樼殑鍐呭銆傝櫧鐒?鍙互閫氳繃璁剧疆 IOMMU 鏉ラ槻姝㈣繖绉嶆儏鍐碉紝浣嗙敱浜庡悇绉嶅師鍥犲畠骞朵笉鎬绘槸鍙敤銆?
涓€浜?USB4 绯荤粺鏈変竴涓敤浜庣鐢?PCIe 闅ч亾鐨?BIOS 璁剧疆銆傝繖琚涓哄彟涓€涓畨鍏ㄧ骇鍒?锛坣opcie锛夈€?
瀹夊叏绾у埆濡備笅锛?
  none锛堟棤锛?    鎵€鏈夎澶囩敱鍥轰欢鑷姩杩炴帴銆傛棤闇€鐢ㄦ埛鎵瑰噯銆傚湪 BIOS 璁剧疆涓繖閫氬父绉颁负
    **Legacy mode锛堜紶缁熸ā寮忥級**銆?
  user锛堢敤鎴凤級
    浼氳闂敤鎴锋槸鍚﹀厑璁歌繛鎺ヨ璁惧銆傚熀浜庨€氳繃 `/sys/bus/thunderbolt/devices`
    鍙敤鐨勮澶囨爣璇嗕俊鎭紝鐢ㄦ埛闅忓悗鍙互鍋氬嚭鍐冲畾銆傚湪 BIOS 璁剧疆涓繖閫氬父绉颁负
    **Unique ID锛堝敮涓€ ID锛?*銆?
  secure锛堝畨鍏級
    浼氳闂敤鎴锋槸鍚﹀厑璁歌繛鎺ヨ璁惧銆傞櫎浜?UUID 涔嬪锛岃澶囷紙濡傛灉鏀寔瀹夊叏杩炴帴锛夎繕浼?    鏀跺埌涓€涓寫鎴樺€硷紝璇ュ€煎簲涓庡熀浜庡啓鍏?`key` sysfs 灞炴€х殑闅忔満瀵嗛挜鎵€鏈熸湜鐨勫€煎尮閰嶃€?    鍦?BIOS 璁剧疆涓繖閫氬父绉颁负 **One time saved key锛堜竴娆℃€т繚瀛樺瘑閽ワ級**銆?
  dponly锛堜粎鏄剧ず绔彛锛?    鍥轰欢鑷姩涓?Display Port 涓?USB 鍒涘缓闅ч亾銆備笉杩涜 PCIe 闅ч亾銆傚湪 BIOS 璁剧疆涓?    杩欓€氬父绉颁负 **Display Port Only锛堜粎鏄剧ず绔彛锛?*銆?
  usbonly锛堜粎 USB锛?    鍥轰欢鑷姩涓烘墿灞曞潪涓殑 USB 鎺у埗鍣ㄤ笌 Display Port 鍒涘缓闅ч亾銆傛墿灞曞潪涓嬫父鐨勬墍鏈?    PCIe 閾捐矾琚Щ闄ゃ€?
  nopcie锛堟棤 PCIe锛?    PCIe 闅ч亾琚?BIOS 绂佺敤/绂佹銆傚湪涓€浜?USB4 绯荤粺涓彲鐢ㄣ€?
褰撳墠鐨勫畨鍏ㄧ骇鍒彲浠ヤ粠 `/sys/bus/thunderbolt/devices/domainX/security` 璇诲彇锛屽叾涓?`domainX` 鏄富鏈烘帶鍒跺櫒绠＄悊鐨?Thunderbolt 鍩熴€傞€氬父姣忎釜 Thunderbolt 涓绘満鎺у埗鍣?瀵瑰簲涓€涓煙銆?
濡傛灉瀹夊叏绾у埆涓?`user` 鎴?`secure`锛屽垯鍦ㄥ垱寤?PCIe 闅ч亾锛堜緥濡?PCIe 璁惧鍑虹幇锛?涔嬪墠锛屽繀椤荤敱鐢ㄦ埛鎺堟潈鎵€杩炴帴鐨勮澶囥€?
姣忎釜鎻掑叆鐨?Thunderbolt 璁惧閮戒細鍑虹幇鍦?sysfs 鐨?`/sys/bus/thunderbolt/devices`
涓嬨€傝璁惧鐩綍鎼哄甫鍙敤浜庤瘑鍒壒瀹氳澶囩殑淇℃伅锛屽寘鎷叾鍚嶇О涓?UUID銆?
### Authorizing devices when security level is ``user`` or ``secure``锛堝湪瀹夊叏绾у埆涓?``user`` 鎴?``secure`` 鏃舵巿鏉冭澶囷級


```
  /sys/bus/thunderbolt/devices/0-1/authorized	- 0
  /sys/bus/thunderbolt/devices/0-1/device	- 0x8004
  /sys/bus/thunderbolt/devices/0-1/device_name	- Thunderbolt to FireWire Adapter
  /sys/bus/thunderbolt/devices/0-1/vendor	- 0x1
  /sys/bus/thunderbolt/devices/0-1/vendor_name	- Apple, Inc.
  /sys/bus/thunderbolt/devices/0-1/unique_id	- e0376f00-0300-0100-ffff-ffffffffffff

```
`authorized` 灞炴€ц鍙栦负 0锛屾剰鍛崇潃灏氭湭鍒涘缓 PCIe 闅ч亾銆傛巿鏉冭璁惧锛?```
锛堝悜 authorized 鍐欏叆 1锛氾級

  # echo 1 > /sys/bus/thunderbolt/devices/0-1/authorized

```
杩欏皢鍒涘缓 PCIe 闅ч亾锛岃澶囩幇宸茶繛鎺ャ€?
濡傛灉璁惧鏀寔瀹夊叏杩炴帴锛屼笖鍩熷畨鍏ㄧ骇鍒涓?`secure`锛屽畠浼氭湁涓€涓澶栫殑 `key` 灞炴€э紝
鍙繚瀛樹竴涓殢鏈虹殑 32 瀛楄妭鍊硷紝鐢ㄤ簬鎺堟潈涓庢寫鎴樿璁惧锛?```
锛堜緥濡傦細锛?
  /sys/bus/thunderbolt/devices/0-3/authorized	- 0
  /sys/bus/thunderbolt/devices/0-3/device	- 0x305
  /sys/bus/thunderbolt/devices/0-3/device_name	- AKiTiO Thunder3 PCIe Box
  /sys/bus/thunderbolt/devices/0-3/key		-
  /sys/bus/thunderbolt/devices/0-3/vendor	- 0x41
  /sys/bus/thunderbolt/devices/0-3/vendor_name	- inXtron
  /sys/bus/thunderbolt/devices/0-3/unique_id	- dc010000-0000-8508-a22d-32ca6421cb16

```
娉ㄦ剰锛岄粯璁ゆ儏鍐典笅 key 涓虹┖銆?
濡傛灉鐢ㄦ埛涓嶆兂浣跨敤瀹夊叏杩炴帴锛屼粬浠彧闇€ `echo 1` 鍒?`authorized` 灞炴€э紝PCIe 闅ч亾灏变細
浠ヤ笌 `user` 瀹夊叏绾у埆鐩稿悓鐨勬柟寮忚鍒涘缓銆?
濡傛灉鐢ㄦ埛鎯充娇鐢ㄥ畨鍏ㄨ繛鎺ワ紝鍦ㄨ澶囬娆℃彃鍏ユ椂锛?```
锛堢敓鎴愬苟鍐欏叆瀵嗛挜锛岀劧鍚庢巿鏉冿細锛?
  # key=$(openssl rand -hex 32)
  # echo $key > /sys/bus/thunderbolt/devices/0-3/key
  # echo 1 > /sys/bus/thunderbolt/devices/0-3/authorized

```
鐜板湪璁惧宸茶繛鎺ワ紙PCIe 闅ч亾琚垱寤猴級锛屽苟涓斿瘑閽ヨ瀛樺偍鍦ㄨ澶囩殑 NVM 涓娿€?
涓嬩竴娆℃彃鍏ヨ澶囨椂锛岀敤鎴峰彲浠ュ璁惧杩涜楠岃瘉锛堟寫鎴橈級锛?```
锛堝啓鍏ュ瘑閽ュ苟浠ユ寫鎴樻ā寮忔巿鏉冿細锛?
  # echo $key > /sys/bus/thunderbolt/devices/0-3/key
  # echo 2 > /sys/bus/thunderbolt/devices/0-3/authorized

```
濡傛灉璁惧杩斿洖鐨勬寫鎴樺€间笌鍩轰簬瀵嗛挜鎵€鏈熸湜鐨勫€煎尮閰嶏紝璁惧灏辫杩炴帴骞朵笖 PCIe 闅ч亾琚垱寤恒€?鐒惰€岋紝濡傛灉鎸戞垬澶辫触锛屽垯涓嶄細鍒涘缓浠讳綍闅ч亾锛屽苟鍚戠敤鎴疯繑鍥為敊璇€?
濡傛灉鐢ㄦ埛浠嶆兂杩炴帴璇ヨ澶囷紝浠栦滑鍙互涓嶇敤瀵嗛挜鐩存帴鎵瑰噯璇ヨ澶囷紝鎴栬€呭啓鍏ヤ竴涓柊瀵嗛挜骞跺悜
`authorized` 鏂囦欢鍐欏叆 1锛屼粠鑰屽皢鏂板瘑閽ュ瓨鍌ㄥ湪璁惧鐨?NVM 涓娿€?
### De-authorizing devices锛堝彇娑堟巿鏉冭澶囷級


鍙互閫氳繃灏?`0` 鍐欏叆鍏?`authorized` 灞炴€ф潵鍙栨秷瀵硅澶囩殑鎺堟潈銆傝繖闇€瑕佽繛鎺ョ鐞嗗櫒
瀹炵幇鐨勬敮鎸侊紝鍙互閫氳繃璇诲彇鍩熺殑 `deauthorization` 灞炴€ф潵妫€鏌ャ€傚鏋滃畠璇讳负 `1`锛屽垯
璇ュ姛鑳藉彈鏀寔銆?
褰撲竴涓澶囪鍙栨秷鎺堟潈鏃讹紝浠庣埗璁惧鐨?PCIe 涓嬫父锛堟垨鏍癸級绔彛鍒拌澶?PCIe 涓婃父绔彛鐨?PCIe 闅ч亾浼氳鎷嗛櫎銆傝繖鏈川涓婁笌 PCIe 鐑Щ闄ょ浉鍚岋紝鎵€娑夊強鐨?PCIe 鎷撴墤灏嗕笉鍐嶅彲璁块棶锛?鐩村埌璁惧琚啀娆℃巿鏉冦€傚鏋滄秹鍙?NVMe 鎴栫被浼肩殑瀛樺偍璁惧锛岃嫢鍏朵笂鐨勬枃浠剁郴缁熸湭姝ｇ‘
鍏抽棴锛屽氨鏈夋暟鎹涪澶辩殑椋庨櫓銆傜壒姝よ鍛婏紒

### DMA protection utilizing IOMMU锛堝埄鐢?IOMMU 鐨?DMA 淇濇姢锛?

2018 骞村強涔嬪悗甯︽湁 Thunderbolt 绔彛鐨勬柊绯荤粺鍙兘鍘熺敓鏀寔 IOMMU銆傝繖鎰忓懗鐫€ Thunderbolt
瀹夊叏鎬х敱 IOMMU 澶勭悊锛屽洜姝ゆ墍杩炴帴鐨勮澶囨棤娉曡闂┍鍔ㄤ负鍏跺垎閰嶄箣澶栫殑鍐呭瓨鍖哄煙銆傚綋 Linux
杩愯鍦ㄨ繖鏍风殑绯荤粺涓婃椂锛屽鏋滅敤鎴峰皻鏈惎鐢紝瀹冧細鑷姩鍚敤 IOMMU銆傝繖浜涚郴缁熷彲浠ラ€氳繃浠?`/sys/bus/thunderbolt/devices/domainX/iommu_dma_protection` 灞炴€ц鍙?`1` 鏉ヨ瘑鍒€?
鍦ㄨ繖绉嶆儏鍐典笅锛岄┍鍔ㄥ苟鏈仛浠讳綍鐗规畩鎿嶄綔锛屼絾鐢变簬 DMA 淇濇姢鐢?IOMMU 澶勭悊锛屽畨鍏ㄧ骇鍒?锛堝鏋滆缃簡锛夊氨鍙樺緱澶氫綑銆傚嚭浜庤繖涓師鍥狅紝涓€浜涚郴缁熷嚭鍘傛椂灏嗗畨鍏ㄧ骇鍒涓?`none`銆?鍏朵粬绯荤粺灏嗗畨鍏ㄧ骇鍒涓?`user` 浠ユ敮鎸侀檷绾у埌杈冩棫鐨勬搷浣滅郴缁燂紝鍥犳甯屾湜鍦?IOMMU DMA
淇濇姢鍚敤鏃惰嚜鍔ㄦ巿鏉冭澶囩殑鐢ㄦ埛鍙互浣跨敤锛?```
锛堜互涓?udev 瑙勫垯锛氾級

  ACTION=="add", SUBSYSTEM=="thunderbolt", ATTRS{iommu_dma_protection}=="1", ATTR{authorized}=="0", ATTR{authorized}="1"

```
### Upgrading NVM on Thunderbolt device, host or retimer锛堝崌绾?Thunderbolt 璁惧銆佷富鏈烘垨閲嶅畾鏃跺櫒鐨?NVM锛?

鐢变簬澶ч儴鍒嗗姛鑳界敱杩愯鍦ㄤ富鏈烘帶鍒跺櫒鎴栬澶囦笂鐨勫浐浠跺鐞嗭紝鍥犳鍥轰欢鑳藉琚崌绾у埌鏈€鏂?鐗堟湰锛堝叾涓彲鑳界殑缂洪櫡宸茶淇锛夋槸寰堥噸瑕佺殑銆傞€氬父 OEM 浼氫粠鍏舵敮鎸佺珯鐐规彁渚涜鍥轰欢銆?
鐩墠锛屾帹鑽愰€氳繃 鈥渇wupd鈥?宸ュ叿鏇存柊鍥轰欢銆傞粯璁ゆ儏鍐典笅瀹冧娇鐢?LVFS锛圠inux Vendor Firmware
Service锛孡inux 渚涘簲鍟嗗浐浠舵湇鍔★級闂ㄦ埛浠庣‖浠朵緵搴斿晢鑾峰彇鏈€鏂板浐浠讹紝骞跺湪鍙戠幇鍏煎鏃舵洿鏂?鎵€杩炴帴鐨勮澶囥€傝鎯呭弬瑙侊細https://github.com/fwupd/fwupd銆?
鍦ㄤ负璁惧銆佷富鏈烘垨閲嶅畾鏃跺櫒鍗囩骇鍥轰欢涔嬪墠锛岃纭繚杩欐槸涓€娆″悎閫傜殑鍗囩骇銆傚鏋滄湭鑳藉仛鍒帮紝
鍙兘浼氫娇璁惧杩涘叆涓€绉嶆病鏈夌壒娈婂伐鍏峰氨鏃犳硶姝ｅ父浣跨敤鐨勭姸鎬侊紒

Apple Mac 涓婄殑涓绘満 NVM 鍗囩骇涓嶅彈鏀寔銆?
fwupd 榛樿宸插畨瑁呫€傚鏋滀綘鐨勭郴缁熶笂娌℃湁瀹冿紝鍙渶浣跨敤浣犵殑鍙戣鐗堝寘绠＄悊鍣ㄦ潵鑾峰彇瀹冦€?
瑕侀€氳繃 fwupd 鏌ョ湅鍙兘鐨勬洿鏂帮紝浣犻渶瑕佹彃鍏ヤ竴涓?Thunderbolt 璁惧锛屼互渚夸富鏈烘帶鍒跺櫒鍑虹幇銆?杩炴帴鍝釜璁惧骞朵笉閲嶈锛堥櫎闈炰綘鏄湪鍗囩骇鏌愪釜璁惧鐨?NVM鈥斺€旀鏃朵綘闇€瑕佽繛鎺ラ偅涓壒瀹氱殑
璁惧锛夈€?
娉ㄦ剰锛屼綘鐨勭郴缁熷彲鑳芥彁渚?OEM 鐗瑰畾鐨勬柟娉曟潵涓轰笂鐢垫帶鍒跺櫒锛堚€滃己鍒朵笂鐢碘€濓紝force power锛夛紝
鍦ㄨ繖绉嶆儏鍐典笅灏辨棤闇€鎻掑叆 Thunderbolt 璁惧銆?
浣跨敤 fwupd 鏇存柊鍥轰欢寰堢畝鍗曗€斺€旇鍙傞槄 fwupd github 涓婄殑瀹樻柟 readme銆?
濡傛灉鍥轰欢鏄犲儚鍐欏叆鎴愬姛锛岃澶囦細鐭殏娑堝け銆備竴鏃﹀畠閲嶆柊鍑虹幇锛岄┍鍔ㄤ細娉ㄦ剰鍒板畠骞跺彂璧蜂竴娆?瀹屾暣鐨勫姞鐢靛惊鐜€傝繃浜嗕竴浼氬効璁惧浼氬啀娆″嚭鐜帮紝姝ゆ椂瀹冨簲褰撳畬鍏ㄥ彲鐢ㄣ€?
鐩爣璁惧搴斿湪 fwupd 鐣岄潰涓樉绀?鈥淐urrent version锛堝綋鍓嶇増鏈級鈥?涓嬬殑鏂扮増鏈紝浠ュ強
鈥淯pdate State: Success锛堟洿鏂扮姸鎬侊細鎴愬姛锛夆€濄€?
### Upgrading firmware manually锛堟墜鍔ㄥ崌绾у浐浠讹級


濡傛灉鍙兘锛岃浣跨敤 fwupd 鏉ユ洿鏂板浐浠躲€備絾鏄紝濡傛灉浣犵殑璁惧 OEM 灏氭湭灏嗗浐浠朵笂浼犲埌 LVFS锛?鑰屽畠鍙粠浠栦滑涓€渚т笅杞斤紝浣犲彲浠ヤ娇鐢ㄤ笅闈㈢殑鏂规硶鐩存帴鍗囩骇鍥轰欢銆?
鎵嬪姩鍥轰欢鏇存柊鍙互浣跨敤 'dd' 宸ュ叿瀹屾垚銆傝浣跨敤璇ユ柟娉曟洿鏂板浐浠讹紝浣犻渶瑕佸皢鍏跺啓鍏ヤ富鏈烘垨
璁惧 NVM 鐨勯潪娲昏穬閮ㄥ垎銆備互涓嬫槸鍦?Intel NUC6i7KYK 涓婃洿鏂扮殑绀轰緥锛?```
锛堝皢鍥轰欢鏄犲儚鍐欏叆闈炴椿璺?NVM锛氾級

  # dd if=KYK_TBT_FW_0018.bin of=/sys/bus/thunderbolt/devices/0-0/nvm_non_active0/nvmem

```
涓€鏃︽搷浣滃畬鎴愶紝鎴戜滑鍙互瑙﹀彂 NVM 璁よ瘉锛?```
锛堝啓鍏?1 瑙﹀彂璁よ瘉锛氾級

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/nvm_authenticate

```
濡傛灉娌℃湁杩斿洖閿欒锛岃澶囩殑琛屼负搴斾笌涓婁竴鑺傛墍杩颁竴鑷淬€?
鎴戜滑鍙互閫氳繃杩愯浠ヤ笅鍛戒护鏉ラ獙璇佹柊鐨?NVM 鍥轰欢宸叉縺娲伙細
```
锛堟鏌ヨ璇佺姸鎬佷笌鐗堟湰锛氾級

  # cat /sys/bus/thunderbolt/devices/0-0/nvm_authenticate
  0x0
  # cat /sys/bus/thunderbolt/devices/0-0/nvm_version
  18.0

```
濡傛灉 `nvm_authenticate` 鍖呭惈闄?0x0 涔嬪鐨勪换浣曞€硷紝瀹冨氨鏄笂涓€娆¤璇佸懆鏈熺殑閿欒鐮侊紝
杩欐剰鍛崇潃 NVM 鏄犲儚鐨勮璇佸け璐ャ€?
娉ㄦ剰锛孨VMem 璁惧鐨勫悕绉?`nvm_activeN` 涓?`nvm_non_activeN` 鍙栧喅浜庡畠浠湪 NVMem
瀛愮郴缁熶腑娉ㄥ唽鐨勯『搴忋€傚悕绉颁腑鐨?N 鏄?NVMem 瀛愮郴缁熸坊鍔犵殑鏍囪瘑绗︺€?
### Upgrading on-board retimer NVM when there is no cable connected锛堝湪娌℃湁绾跨紗杩炴帴鏃跺崌绾ф澘杞介噸瀹氭椂鍣ㄧ殑 NVM锛?

濡傛灉骞冲彴鏀寔锛屽嵆浣?USB4 绔彛涓婃病鏈夎繛鎺ヤ换浣曚笢瑗匡紝涔熷彲鑳藉崌绾ч噸瀹氭椂鍣?NVM 鍥轰欢銆?鍦ㄨ繖绉嶆儏鍐典笅锛宍usb4_portX` 璁惧鏈変袱涓壒娈婂睘鎬э細`offline`锛堢绾匡級涓?`rescan`
锛堥噸鏂版壂鎻忥級銆傚崌绾у浐浠剁殑鏂瑰紡鏄細
```
锛堝厛灏嗙鍙ｇ疆涓虹绾匡細锛?
  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/offline

```
杩欎竴姝ョ‘淇濈鍙ｄ笉鍝嶅簲浠讳綍鐑彃鎷斾簨浠讹紝鍚屾椂涔熺‘淇濋噸瀹氭椂鍣ㄨ涓婄數銆備笅涓€姝ユ槸鎵弿锛?```
锛堣Е鍙戦噸鏂版壂鎻忎互鏋氫妇鏉胯浇閲嶅畾鏃跺櫒锛氾級

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/rescan

```
杩欎細鏋氫妇骞舵坊鍔犳澘杞介噸瀹氭椂鍣ㄣ€傜幇鍦ㄥ彲浠ュ儚鏈夌嚎缂嗚繛鎺ユ椂涓€鏍峰崌绾ч噸瀹氭椂鍣?NVM锛堝弬瑙?涓婁竴鑺傦級銆傜劧鑰岋紝鐢变簬澶勪簬绂荤嚎妯″紡锛岄噸瀹氭椂鍣ㄥ苟鏈柇寮€杩炴帴锛屽洜姝ゅ湪鍚?`nvm_authenticate`
鍐欏叆 `1` 涔嬪悗锛屽簲褰撶瓑寰咃細
```
锛堝啀娆℃壂鎻忎娇閲嶅畾鏃跺櫒閲嶆柊灏辩华锛氾級

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/rescan

```
濡傛灉涓€鍒囬『鍒╋紝姝ゆ椂鍙互灏嗙鍙ｆ仮澶嶄负锛?```
锛堥€€鍑虹绾挎ā寮忥細锛?
  # echo 0 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/offline

```
### Upgrading NVM when host controller is in safe mode锛堝湪涓绘満鎺у埗鍣ㄥ浜庡畨鍏ㄦā寮忔椂鍗囩骇 NVM锛?

濡傛灉鐜版湁 NVM 鏈姝ｇ‘璁よ瘉锛堟垨缂哄け锛夛紝涓绘満鎺у埗鍣ㄤ細杩涘叆瀹夊叏妯″紡锛岃繖鎰忓懗鐫€鍞竴
鍙敤鐨勫姛鑳芥槸鍒峰啓涓€涓柊鐨?NVM 鏄犲儚銆傚湪姝ゆā寮忎笅锛岃鍙?`nvm_version` 浼氬洜
`ENODATA` 鑰屽け璐ワ紝骞朵笖璁惧鏍囪瘑淇℃伅缂哄け銆?
瑕佷粠璇ユā寮忔仮澶嶏紝闇€瑕佷互涓庝笂涓€绔犵浉鍚岀殑鏂瑰紡鍚戜富鏈烘帶鍒跺櫒鍒峰啓涓€涓湁鏁堢殑 NVM 鏄犲儚銆?
### Tunneling events锛堥毀閬撲簨浠讹級


褰?`thunderbolt_domain` 涓彂鐢熼毀閬撳彉鍖栨椂锛岄┍鍔ㄤ細鍚戠敤鎴风┖闂村彂閫?`KOBJ_CHANGE`
浜嬩欢銆傝閫氱煡鎼哄甫锛?```
锛堜互涓嬬幆澧冨彉閲忥細锛?
  TUNNEL_EVENT=<EVENT>
  TUNNEL_DETAILS=0:12 <-> 1:20 (USB3)

```
`<EVENT>` 鐨勫彲鑳藉彇鍊间负锛?
  activated锛堝凡婵€娲伙級
    闅ч亾琚縺娲伙紙鍒涘缓锛夈€?
  changed锛堝凡鏀瑰彉锛?    姝ら毀閬撳彂鐢熶簡鍙樺寲銆備緥濡傚甫瀹藉垎閰嶈鏀瑰彉銆?
  deactivated锛堝凡鍋滅敤锛?    闅ч亾琚媶闄ゃ€?
  low bandwidth锛堜綆甯﹀锛?    闅ч亾鏈幏寰楁渶浣冲甫瀹姐€?
  insufficient bandwidth锛堝甫瀹戒笉瓒筹級
    褰撳墠闅ч亾闇€姹傛病鏈夎冻澶熺殑甯﹀銆?
`TUNNEL_DETAILS` 浠呭湪闅ч亾宸茬煡鏃舵墠鎻愪緵銆備緥濡傦紝鍦ㄥ浐浠惰繛鎺ョ鐞嗗櫒鐨勬儏鍐典笅锛岃繖浼?缂哄け鎴栦笉鎻愪緵瀹屾暣鐨勯毀閬撲俊鎭€傚湪杞欢杩炴帴绠＄悊鍣ㄧ殑鎯呭喌涓嬶紝杩欎細鍖呭惈瀹屾暣鐨勯毀閬撹鎯呫€?鐩墠鐨勬牸寮忎笌椹卞姩璁板綍鏃ュ織鏃朵娇鐢ㄧ殑鏍煎紡涓€鑷淬€傝繖鍙兘浼氶殢鏃堕棿鏀瑰彉銆?
### Networking over Thunderbolt cable锛堥€氳繃 Thunderbolt 绾跨紗鑱旂綉锛?

Thunderbolt 鎶€鏈厑璁搁€氳繃 Thunderbolt 绾跨紗杩炴帴鐨勪袱鍙颁富鏈轰箣闂磋繘琛岃蒋浠堕€氫俊銆?
鍙互鍦?Thunderbolt 閾捐矾涓婇毀閬撲紶杈撲换浣曠被鍨嬬殑娴侀噺锛屼絾鐩墠鎴戜滑鍙敮鎸?Apple
ThunderboltIP 鍗忚銆?
濡傛灉鍙︿竴鍙颁富鏈鸿繍琛岀殑鏄?Windows 鎴?macOS锛屼綘鍞竴闇€瑕佸仛鐨勬槸鍦ㄤ袱鍙颁富鏈轰箣闂磋繛鎺?涓€鏍?Thunderbolt 绾跨紗锛沗thunderbolt-net` 椹卞姩浼氳嚜鍔ㄥ姞杞姐€傚鏋滃彟涓€鍙颁富鏈轰篃鏄?Linux锛屼綘搴斿綋鍦ㄤ竴鍙颁富鏈轰笂鎵嬪姩鍔犺浇 `thunderbolt-net`锛堝畠
```
浼氳嚜鍔ㄨЕ鍙戝彟涓€鍙颁富鏈轰笂鐨勬ā鍧楀姞杞斤細锛?
  # modprobe thunderbolt-net

```
濡傛灉椹卞姩鍐呭缓鍒板唴鏍告槧鍍忎腑锛屽垯鏃犻渶鍋氫换浣曚簨鎯呫€?
椹卞姩浼氫负姣忎釜 Thunderbolt 绔彛鍒涘缓涓€涓櫄鎷熶互澶綉鎺ュ彛锛屽叾鍚嶇О绫讳技 `thunderbolt0`
绛夌瓑銆備粠杩欎竴鐐硅捣锛屼綘鍙互浣跨敤 `ip` 绛夋爣鍑嗙敤鎴风┖闂村伐鍏锋潵閰嶇疆鎺ュ彛锛屾垨璁╀綘鐨?GUI
鑷姩澶勭悊瀹冦€?
### Forcing power锛堝己鍒朵笂鐢碉級


璁稿 OEM 鍖呭惈涓€涓柟娉曪紝鍙敤浜庡皢 Thunderbolt 鎺у埗鍣ㄧ殑鐢垫簮寮哄埗缃簬鈥滃紑鈥濈姸鎬侊紝鍗充娇
娌℃湁杩炴帴浠讳綍涓滆タ銆傚鏋滀綘鐨勬満鍣ㄦ敮鎸侊紝杩欎細鐢?WMI 鎬荤嚎閫氳繃涓€涓悕涓?鈥渇orce_power鈥?鐨?sysfs 灞炴€ф毚闇插嚭鏉ワ紝璇﹁
Documentation/ABI/testing/sysfs-platform-intel-wmi-thunderbolt銆?
娉ㄦ剰锛氱洰鍓嶆棤娉曟煡璇㈠钩鍙扮殑寮哄埗涓婄數鐘舵€併€?