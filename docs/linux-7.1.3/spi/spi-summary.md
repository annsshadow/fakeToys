## Linux 鍐呮牳 SPI 鏀寔姒傝堪


02-Feb-2012

### 浠€涔堟槸 SPI锛?
鈥淪erial Peripheral Interface鈥濓紙SPI锛屼覆琛屽璁炬帴鍙ｏ級鏄竴绉嶅悓姝ョ殑鍥涚嚎涓茶
閾捐矾锛岀敤浜庡皢寰帶鍒跺櫒杩炴帴鍒颁紶鎰熷櫒銆佸瓨鍌ㄥ櫒鍜屽璁俱€傚畠鏄竴涓畝鍗曠殑鈥滀簨瀹?鏍囧噯鈥濓紝杩樻病鏈夊鏉傚埌闇€瑕佹垚绔嬩竴涓爣鍑嗗寲缁勭粐銆係PI 浣跨敤涓绘満/鐩爣锛坔ost/target锛?閰嶇疆銆?
涓夋潯淇″彿绾垮寘鍚竴鏉℃椂閽熺嚎锛圫CK锛岄€氬父绾︿负 10 MHz锛夛紝浠ュ強骞惰鐨勬暟鎹嚎锛?鍒嗗埆鎵胯浇鈥滀富鏈鸿緭鍑恒€佷粠鏈鸿緭鍏モ€濓紙MOSI锛夋垨鈥滀富鏈鸿緭鍏ャ€佷粠鏈鸿緭鍑衡€濓紙MISO锛?淇″彿銆傦紙涔熸湁浣跨敤鍏朵粬鍚嶇О鐨勬儏鍐点€傦級鍏辨湁鍥涚鏃堕挓妯″紡鐢ㄤ簬浜ゆ崲鏁版嵁锛涘叾涓?mode-0 鍜?mode-3 鏈€涓哄父鐢ㄣ€傛瘡涓椂閽熷懆鏈熺Щ鍑哄拰绉诲叆鏁版嵁锛涙椂閽熷彧鍦ㄦ湁鏁版嵁
浣嶉渶瑕佺Щ浣嶆椂鎵嶄細璺冲姩銆備笉杩囧苟闈炴墍鏈夋暟鎹綅閮戒細琚娇鐢紱骞堕潪姣忎釜鍗忚閮戒細
鐢ㄥ埌杩欎簺鍏ㄥ弻宸ヨ兘鍔涖€?
SPI 涓绘満浣跨敤绗洓鏉♀€滅墖閫夆€濓紙chip select锛夌嚎鏉ユ縺娲绘煇涓粰瀹氱殑 SPI 鐩爣
璁惧锛屽洜姝ら偅涓夋潯淇″彿绾垮彲浠ュ苟琛岃繛鎺ュ埌澶氫釜鑺墖銆傛墍鏈?SPI 鐩爣閮芥敮鎸佺墖閫夛紱
瀹冧滑閫氬父鏄綆鐢靛钩鏈夋晥鐨勪俊鍙凤紝瀵圭洰鏍?'x' 鏍囪涓?nCSx锛堜緥濡?nCS0锛夈€傛湁浜?璁惧杩樻湁鍏跺畠淇″彿锛岄€氬父鍖呭惈涓€涓彂寰€涓绘満鐨勪腑鏂€?
涓?USB 鎴?SMBus 涔嬬被鐨勪覆琛屾€荤嚎涓嶅悓锛屽嵆浣挎槸 SPI 鐩爣鍔熻兘鐨勫簳灞傚崗璁紝
閫氬父涔熷湪涓嶅悓鍘傚晢涔嬮棿浜掍笉鍏煎锛堝晢鍝佸寲鐨?SPI 瀛樺偍鍣ㄨ姱鐗囬櫎澶栵級銆?
  - SPI 鍙敤浜庤姹?鍝嶅簲寮忕殑璁惧鍗忚锛屼緥濡傝Е鎽稿睆浼犳劅鍣ㄥ拰瀛樺偍鍣ㄨ姱鐗囥€?
  - 瀹冧篃鍙互鐢ㄤ簬浠绘剰鏂瑰悜鐨勬暟鎹祦浼犺緭锛堝崐鍙屽伐锛夛紝鎴栦袱涓柟鍚戝悓鏃惰繘琛?    锛堝叏鍙屽伐锛夈€?
  - 鏈変簺璁惧鍙兘浣跨敤 8 浣嶅瓧銆傚叾瀹冭澶囧彲鑳戒娇鐢ㄤ笉鍚岀殑瀛楅暱锛屼緥濡?12 浣嶆垨
    20 浣嶆暟瀛楅噰鏍峰€肩殑娴併€?
  - 瀛楅€氬父浠ュ叾鏈€楂樻湁鏁堜綅锛圡SB锛夊厛鍙戦€侊紝浣嗘湁鏃舵渶浣庢湁鏁堜綅锛圠SB锛変細
    鍏堝彂閫併€?
  - 鏈夋椂 SPI 鐢ㄤ簬鎶婅澶囧儚绉讳綅瀵勫瓨鍣ㄩ偅鏍风骇鑱旓紙daisy-chain锛夈€?
鍚屾牱鍦帮紝SPI 鐩爣鏋佸皯鏀寔浠讳綍绫诲瀷鐨勮嚜鍔ㄥ彂鐜?鏋氫妇鍗忚銆備粠缁欏畾 SPI 涓绘満
鎺у埗鍣ㄥ彲璁块棶鐨勭洰鏍囪澶囨爲閫氬父瑕侀€氳繃閰嶇疆琛ㄦ墜鍔ㄥ缓绔嬨€?
SPI 鍙槸姝ょ被鍥涚嚎鍗忚浣跨敤鐨勪竴涓悕绉帮紝澶у鏁版帶鍒跺櫒澶勭悊鈥淢icroWire鈥?锛堝彲瑙嗕负鍗婂弻宸ョ殑 SPI锛岀敤浜庤姹?鍝嶅簲鍗忚锛夈€丼SP锛堚€淪ynchronous Serial
Protocol鈥濓級锛孭SP锛堚€淧rogrammable Serial Protocol鈥濓級浠ュ強鍏跺畠鐩稿叧鍗忚閮?娌℃湁闂銆?
鏈変簺鑺墖閫氳繃鍚堝苟 MOSI 鍜?MISO銆佸苟鍦ㄧ‖浠跺眰闈㈠皢鑷繁闄愬埗涓哄崐鍙屽伐鏉ュ噺灏?涓€鏉′俊鍙风嚎銆備簨瀹炰笂鏈変簺 SPI 鑺墖灏辨妸杩欑淇″彿妯″紡浣滀负涓€涓?strapping 閫夐」銆?杩欎簺鑺墖鍙互浣跨敤涓?SPI 鐩稿悓鐨勭紪绋嬫帴鍙ｈ闂紝浣嗗綋鐒跺畠浠棤娉曞鐞嗗叏鍙屽伐
浼犺緭銆備綘鍙兘浼氬彂鐜拌繖绉嶈姱鐗囪鎻忚堪涓轰娇鐢ㄢ€滀笁绾库€濅俊鍙凤細SCK銆乨ata銆乶CSx銆?锛堥偅鏉℃暟鎹嚎鏈夋椂琚О涓?MOMI 鎴?SISO銆傦級

寰帶鍒跺櫒閫氬父鍚屾椂鏀寔 SPI 鍗忚鐨勪富鏈虹鍜岀洰鏍囩銆傛湰鏂囨。锛堜互鍙?Linux锛夊悓鏃?鏀寔 SPI 浜や簰鐨勪富鏈虹涓庣洰鏍囩銆?

### 璋佸湪浣跨敤瀹冿紵鍦ㄥ摢浜涚郴缁熶笂锛?
浣跨敤 SPI 鐨?Linux 寮€鍙戣€呭ぇ姒傛槸鍦ㄤ负宓屽叆寮忕郴缁熸澘缂栧啓璁惧椹卞姩銆係PI 鐢ㄤ簬
鎺у埗澶栭儴鑺墖锛屽畠涔熸槸姣忓紶 MMC 鎴?SD 瀛樺偍鍗￠兘鏀寔鐨勫崗璁€傦紙杈冩棭鐨勨€淒ataFlash鈥?鍗℃棭浜?MMC 鍗★紝浣嗕娇鐢ㄧ浉鍚岀殑杩炴帴鍣ㄥ拰鍗″舰锛屽彧鏀寔 SPI銆傦級鏈変簺 PC 纭欢浣跨敤
SPI flash 瀛樻斁 BIOS 浠ｇ爜銆?
SPI 鐩爣鑺墖绉嶇被绻佸锛屼粠鐢ㄤ簬妯℃嫙浼犳劅鍣ㄥ拰缂栬В鐮佸櫒鐨勬暟/妯¤浆鎹㈠櫒锛屽埌瀛樺偍鍣紝
鍐嶅埌鍍?USB 鎺у埗鍣ㄦ垨浠ュお缃戦€傞厤鍣ㄨ繖鏍风殑澶栬锛岀瓑绛夈€?
澶у鏁颁娇鐢?SPI 鐨勭郴缁熶細鍦ㄤ富鏉夸笂闆嗘垚灏戦噺璁惧銆傛湁浜涢€氳繃鎵╁睍杩炴帴鍣ㄦ彁渚?SPI
閾捐矾锛涘湪娌℃湁涓撶敤 SPI 鎺у埗鍣ㄧ殑鎯呭喌涓嬶紝鍙互浣跨敤 GPIO 寮曡剼鍒涘缓涓€涓綆閫熺殑
鈥渂itbanging鈥濋€傞厤鍣ㄣ€傛瀬灏戞湁绯荤粺浼氬 SPI 鎺у埗鍣ㄨ繘琛屸€滅儹鎻掓嫈鈥濓紱浣跨敤 SPI 鐨?鐞嗙敱鍦ㄤ簬浣庢垚鏈拰绠€鍗曟搷浣滐紝鑰屽鏋滃姩鎬侀噸閰嶇疆寰堥噸瑕侊紝USB 閫氬父鏄洿鍚堥€傜殑
浣庡紩鑴氭暟澶栬鎬荤嚎銆?
璁稿鍙互杩愯 Linux 鐨勫井鎺у埗鍣ㄩ兘闆嗘垚浜嗕竴涓垨澶氫釜甯︽湁 SPI 妯″紡鐨?I/O
鎺ュ彛銆傛湁浜?SPI 鏀寔锛屽畠浠氨鍙互浣跨敤 MMC 鎴?SD 鍗★紝鑰屾棤闇€涓撶敤鐨?MMC/SD/SDIO
鎺у埗鍣ㄣ€?

### 鎴戞湁鐐瑰洶鎯戙€傝繖鍥涚 SPI鈥滄椂閽熸ā寮忊€濇槸浠€涔堬紵

杩欓噷寰堝鏄撴贩娣嗭紝鑰屼笖浣犺兘鎵惧埌鐨勫巶鍟嗘枃妗ｆ湭蹇呮湁甯姪銆傝繖鍥涚妯″紡缁勫悎浜?涓や釜妯″紡浣嶏細

 - CPOL 琛ㄧず鍒濆鏃堕挓鏋佹€с€侰POL=0 琛ㄧず鏃堕挓璧峰涓轰綆鐢靛钩锛屽洜姝ょ涓€涓?   锛堝墠娌匡級杈规部鏄笂鍗囨部锛岀浜屼釜锛堝悗娌匡級杈规部鏄笅闄嶆部銆侰POL=1 琛ㄧず鏃堕挓
   璧峰涓洪珮鐢靛钩锛屽洜姝ょ涓€涓紙鍓嶆部锛夎竟娌挎槸涓嬮檷娌裤€?
 - CPHA 琛ㄧず鐢ㄤ簬閲囨牱鏁版嵁鐨勬椂閽熺浉浣嶏紱CPHA=0 琛ㄧず鍦ㄥ墠娌块噰鏍凤紝CPHA=1
   琛ㄧず鍦ㄥ悗娌块噰鏍枫€?
   鐢变簬淇″彿闇€瑕佸湪閲囨牱鍓嶇ǔ瀹氾紝CPHA=0 鎰忓懗鐫€鍏舵暟鎹湪绗竴涓椂閽熻竟娌夸箣鍓?   鍗婁釜鏃堕挓鍛ㄦ湡灏辫鍐欏叆銆傜墖閫夊彲鑳戒娇鍏跺彉寰楀彲鐢ㄣ€?
鑺墖瑙勬牸骞朵笉鎬绘槸浼氱敤鍚屾牱澶氱殑璇濊鈥滀娇鐢?SPI 妯″紡 X鈥濓紝浣嗗畠浠殑鏃跺簭鍥句細璁?CPOL 鍜?CPHA 妯″紡涓€鐩簡鐒躲€?
鍦?SPI 妯″紡缂栧彿涓紝CPOL 鏄珮浣嶏紝CPHA 鏄綆浣嶃€傚洜姝わ紝褰撴煇涓姱鐗囩殑鏃跺簭鍥?鏄剧ず鏃堕挓璧峰涓轰綆鐢靛钩锛圕POL=0锛夛紝涓旀暟鎹湪鍚庢部鏃堕挓杈规部绋冲畾浠ヤ究閲囨牱
锛圕PHA=1锛夋椂锛岄偅灏辨槸 SPI 妯″紡 1銆?
娉ㄦ剰锛屾椂閽熸ā寮忓湪鐗囬€夊彉涓烘湁鏁堢殑涓€鍒诲氨鐩稿叧浜嗐€傚洜姝や富鏈哄繀椤诲湪閫夋嫨鐩爣涔嬪墠
灏嗘椂閽熻涓烘棤鏁堬紝鑰岀洰鏍囧彲浠ラ€氳繃鍦ㄥ叾閫夐€氱嚎鍙樹负鏈夋晥鏃堕噰鏍锋椂閽熺數骞虫潵鍒ゆ柇鎵€閫夌殑
鏋佹€с€傝繖灏辨槸涓轰粈涔堣澶氳澶囧悓鏃舵敮鎸佷緥濡傛ā寮?0 鍜屾ā寮?3锛氬畠浠笉鍏冲績鏋佹€э紝
骞朵笖鎬绘槸鍦ㄤ笂鍗囨椂閽熻竟娌挎敹鍙戞暟鎹€?

### 杩欎簺椹卞姩缂栫▼鎺ュ彛鏄浣曞伐浣滅殑锛?
<linux/spi/spi.h> 澶存枃浠跺寘鍚?kerneldoc锛屼富婧愪唬鐮佷篃鏄姝わ紝浣犲綋鐒跺簲璇?闃呰鍐呮牳 API 鏂囨。涓殑閭ｄ竴绔犮€傝繖閲屽彧鏄竴涓杩帮紝浠ヤ究浣犲湪浜嗚В缁嗚妭涔嬪墠
鍏堝缓绔嬭捣鏁翠綋璁ょ煡銆?
SPI 璇锋眰鎬绘槸杩涘叆 I/O 闃熷垪銆傚缁欏畾 SPI 璁惧鐨勮姹傛€绘槸鎸?FIFO 椤哄簭鎵ц锛?骞堕€氳繃瀹屾垚鍥炶皟锛坈ompletion callbacks锛夊紓姝ュ畬鎴愩€備篃鎻愪緵浜嗕竴浜涚畝鍗曠殑鍚屾
灏佽鏉ヨ皟鐢ㄨ繖浜涙帴鍙ｏ紝鍖呮嫭鐢ㄤ簬甯歌浜嬪姟绫诲瀷鐨勫皝瑁咃紝渚嬪鍏堝啓涓€鏉″懡浠ゅ啀璇诲彇
鍏跺搷搴斻€?
SPI 椹卞姩鏈変袱绉嶇被鍨嬶紝杩欓噷绉颁负锛?
  Controller drivers锛堟帶鍒跺櫒椹卞姩锛?...
        鎺у埗鍣ㄥ彲鑳藉唴缃簬 System-On-Chip锛堢墖涓婄郴缁燂級
	澶勭悊鍣ㄤ腑锛屽苟涓旈€氬父鍚屾椂鏀寔 Controller 鍜岀洰鏍囪鑹层€?	杩欎簺椹卞姩浼氳闂‖浠跺瘎瀛樺櫒骞跺彲鑳戒娇鐢?DMA銆?	鎴栬€呭畠浠彲浠ユ槸 PIO bitbanger锛屽彧闇€瑕?GPIO 寮曡剼銆?
  Protocol drivers锛堝崗璁┍鍔級 ...
        杩欎簺椹卞姩閫氳繃鎺у埗鍣ㄩ┍鍔ㄤ紶閫掓秷鎭紝
	浠ヤ笌浣嶄簬 SPI 閾捐矾鍙︿竴绔殑鏌愪釜鐩爣鎴?Controller 璁惧閫氫俊銆?
鍥犳锛屼緥濡傛煇涓崗璁┍鍔ㄥ彲鑳戒笌 MTD 灞傚璇濓紝鎶婃暟鎹鍑哄埌瀛樺偍鍦?SPI flash
锛堝 DataFlash锛変笂鐨勬枃浠剁郴缁燂紱鍏跺畠鍗忚椹卞姩鍙兘鎺у埗闊抽鎺ュ彛銆佸皢瑙︽懜灞忎紶鎰熷櫒
鍛堢幇涓鸿緭鍏ユ帴鍙ｏ紝鎴栧湪宸ヤ笟澶勭悊杩囩▼涓洃瑙嗘俯搴﹀拰鐢靛帇姘村钩銆傝€岃繖浜涘彲鑳介兘鍦?鍏变韩鍚屼竴涓帶鍒跺櫒椹卞姩銆?
鈥渟truct spi_device鈥濆皝瑁呬簡杩欎袱绫婚┍鍔ㄤ箣闂寸殑鎺у埗鍣ㄧ鎺ュ彛銆?
SPI 缂栫▼鎺ュ彛鏈変竴涓渶灏忔牳蹇冿紝渚ч噸浜庝娇鐢ㄩ┍鍔ㄦā鍨嬨€佸€熷姪鏉跨骇鐗瑰畾鍒濆鍖栦唬鐮?鎻愪緵鐨勮澶囪〃鏉ヨ繛鎺ユ帶鍒跺櫒椹卞姩鍜屽崗璁┍鍔ㄣ€係PI
```

   /sys/devices/.../CTLR ... physical node for a given SPI controller

   /sys/devices/.../CTLR/spiB.C ... spi_device on bus "B",
	chipselect C, accessed through CTLR.

   /sys/bus/spi/devices/spiB.C ... symlink to that physical
	.../CTLR/spiB.C device

   /sys/devices/.../CTLR/spiB.C/modalias ... identifies the driver
	that should be used with this device (for hotplug/coldplug)

   /sys/bus/spi/drivers/D ... driver for one or more spi*.* devices

   /sys/class/spi_master/spiB ... symlink to a logical node which could hold
	class related state for the SPI host controller managing bus "B".
	All spiB.* devices share one physical SPI bus segment, with SCLK,
	MOSI, and MISO.

   /sys/devices/.../CTLR/slave ... virtual file for (un)registering the
	target device for an SPI target controller.
	Writing the driver name of an SPI target handler to this file
	registers the target device; writing "(null)" unregisters the target
	device.
	Reading from this file shows the name of the target device ("(null)"
	if not registered).

   /sys/class/spi_slave/spiB ... symlink to a logical node which could hold
	class related state for the SPI target controller on bus "B".  When
	registered, a single spiB.* device is present here, possible sharing
	the physical SPI bus segment with other SPI target devices.

```
鐩墠锛屽敮涓€鐨勭被鐗瑰畾鐘舵€佸氨鏄€荤嚎鍙凤紙鈥渟piB鈥濅腑鐨勨€淏鈥濓級锛屽洜姝ら偅浜?/sys/class 鏉＄洰浠呯敤浜庡揩閫熻瘑鍒€荤嚎銆?

### 鏉跨骇鐗瑰畾鐨勫垵濮嬪寲浠ｇ爜濡備綍澹版槑 SPI 璁惧锛?
Linux 闇€瑕佽嫢骞茬被淇℃伅鎵嶈兘姝ｇ‘閰嶇疆 SPI 璁惧銆傚嵆渚垮浜庢敮鎸侀儴鍒嗚嚜鍔?鍙戠幇/鏋氫妇鐨勮姱鐗囷紝杩欎簺淇℃伅閫氬父涔熺敱鏉跨骇鐗瑰畾浠ｇ爜鎻愪緵銆?
##### 澹版槑鎺у埗鍣?

绗竴绫讳俊鎭槸涓€涓垪琛紝鍒楀嚭瀛樺湪鍝簺 SPI 鎺у埗鍣ㄣ€傚浜庡熀浜?System-on-Chip
锛圫OC锛夌殑鏉匡紝杩欎簺閫氬父鏄?platform 璁惧锛屽苟涓旀帶鍒跺櫒鍙兘闇€瑕佷竴浜?platform_data
鎵嶈兘姝ｅ父杩愪綔銆傗€渟truct platform_device鈥濅細鍖呭惈璇稿鎺у埗鍣ㄧ涓€涓瘎瀛樺櫒鐨勭墿鐞?鍦板潃鍙婂叾 IRQ 绛夎祫婧愩€?
骞冲彴閫氬父浼氭娊璞″嚭鈥滄敞鍐?SPI 鎺у埗鍣ㄢ€濊繖涓€鎿嶄綔锛屼篃璁稿皢鍏朵笌鍒濆鍖栧紩鑴氶厤缃殑
浠ｇ爜鑰﹀悎鍦ㄤ竴璧凤紝浠ヤ究澶氫釜鏉跨殑 arch/.../mach-**/board-**.c 鏂囦欢閮借兘鍏变韩
鐩稿悓鐨勫熀鏈帶鍒跺櫒璁剧疆浠ｇ爜銆傝繖鏄洜涓哄ぇ澶氭暟 SOC 閮芥湁澶氫釜鏀寔 SPI 鐨勬帶鍒跺櫒锛?鑰岄€氬父鍙簲璁剧疆骞舵敞鍐屽湪鏌愪釜缁欏畾鏉夸笂鐪熸鍙敤鐨勯偅浜涖€?```

	#include <mach/spi.h>	/* for mysoc_spi_data */

	/* if your mach-* infrastructure doesn't support kernels that can
	 * run on multiple boards, pdata wouldn't benefit from "__init".
	 */
	static struct mysoc_spi_data pdata __initdata = { ... };

	static __init board_init(void)
	{
		...
		/* this board only uses SPI controller #2 */
		mysoc_register_spi(2, &pdata);
		...
	}

```
```

	#include <mach/spi.h>

	static struct platform_device spi2 = { ... };

	void mysoc_register_spi(unsigned n, struct mysoc_spi_data *pdata)
	{
		struct mysoc_spi_data *pdata2;

		pdata2 = kmalloc(sizeof *pdata2, GFP_KERNEL);
		*pdata2 = pdata;
		...
		if (n == 2) {
			spi2->dev.platform_data = pdata2;
			register_platform_device(&spi2);

			/* also: set up pin modes so the spi2 signals are
			 * visible on the relevant pins ... bootloaders on
			 * production boards may already have done this, but
			 * developer boards will often need Linux to do it.
			 */
		}
		...
	}

```
娉ㄦ剰锛屽嵆浣夸娇鐢ㄧ浉鍚岀殑 SOC 鎺у埗鍣紝涓嶅悓鏉跨殑 platform_data 涔熷彲鑳戒笉鍚屻€備緥濡傦紝
鍦ㄤ竴鍧楁澘涓?SPI 鍙兘浣跨敤澶栭儴鏃堕挓锛岃€屽彟涓€鍧楁澘鍒欎粠鏌愪釜涓绘椂閽熺殑褰撳墠璁剧疆鎺ㄥ鍑?SPI 鏃堕挓銆?
##### 澹版槑鐩爣璁惧


绗簩绫讳俊鎭槸涓€涓垪琛紝鍒楀嚭鐩爣鏉夸笂瀛樺湪鍝簺 SPI 鐩爣璁惧锛岄€氬父杩樺甫鏈夐┍鍔?姝ｇ‘宸ヤ綔鎵€闇€鐨勬煇浜涙澘绾х壒瀹氭暟鎹€?
閫氬父浣犵殑 arch/.../mach-**/board-**.c 鏂囦欢浼氭彁渚涗竴涓皬琛ㄦ牸锛屽垪鍑烘瘡鍧楁澘涓婄殑
SPI 璁惧銆傦紙杩欓€氬父鍙?```

	static struct ads7846_platform_data ads_info = {
		.vref_delay_usecs	= 100,
		.x_plate_ohms		= 580,
		.y_plate_ohms		= 410,
	};

	static struct spi_board_info spi_board_info[] __initdata = {
	{
		.modalias	= "ads7846",
		.platform_data	= &ads_info,
		.mode		= SPI_MODE_0,
		.irq		= GPIO_IRQ(31),
		.max_speed_hz	= 120000 /* max sample rate at 3V */ * 16,
		.bus_num	= 1,
		.chip_select	= 0,
	},
	};

```
鍚屾牱锛屾敞鎰忔澘绾х壒瀹氫俊鎭槸濡備綍鎻愪緵鐨勶紱姣忎釜鑺墖鍙兘闇€瑕佽嫢骞茬绫诲瀷銆傝繖涓緥瀛?灞曠ず浜嗛€氱敤绾︽潫锛屼緥濡傚厑璁哥殑鏈€蹇?SPI 鏃堕挓锛堝湪鏈緥涓槸鏉跨數鍘嬬殑鍑芥暟锛夛紝鎴栬€?IRQ 寮曡剼鏄浣曟帴绾跨殑锛屼互鍙婅姱鐗囩壒瀹氱殑绾︽潫锛屼緥濡傛煇涓紩鑴氱殑鐢靛鎵€瀵艰嚧鐨勯噸瑕?寤惰繜鍙樺寲銆?
锛堣繕鏈夆€渃ontroller_data鈥濓紝鍗冲鎺у埗鍣ㄩ┍鍔ㄥ彲鑳芥湁鐢ㄧ殑淇℃伅銆備緥濡傜壒瀹氫簬澶栬鐨?DMA 璋冧紭鏁版嵁鎴栫墖閫夊洖璋冦€傚畠涔嬪悗浼氬瓨鍌ㄥ湪 spi_device 涓€傦級

board_info 搴旀彁渚涜冻澶熺殑淇℃伅锛屼互渚跨郴缁熷湪璇ヨ姱鐗囩殑椹卞姩鍔犺浇涔嬪墠灏辫兘宸ヤ綔銆傚叾
涓渶楹荤儲鐨勬柟闈㈠彲鑳芥槸 spi_device.mode 瀛楁涓殑 SPI_CS_HIGH 浣嶏紝鍥犱负鍦ㄤ竴涓?鎶婄墖閫夆€滃弽鍚戔€濊В閲婄殑璁惧鍏变韩鍚屼竴鏉℃€荤嚎涔嬪墠锛屽熀纭€璁炬柦鏃犳硶鐭ラ亾濡備綍鍙栨秷閫変腑瀹冦€?
鐒跺悗锛屼綘鐨勬澘鍒濆鍖栦唬鐮佷細鍚?SPI 鍩虹璁炬柦娉ㄥ唽璇ヨ〃锛岃繖鏍风◢鍚?SPI 涓绘満鎺у埗鍣?```

	spi_register_board_info(spi_board_info, ARRAY_SIZE(spi_board_info));

```
鍍忓叾瀹冮潤鎬佹澘绾х壒瀹氳缃竴鏍凤紝浣犱笉浼氭敞閿€瀹冧滑銆?
骞挎硾浣跨敤鐨勨€渃ard鈥濆紡璁＄畻鏈轰細鎶婂瓨鍌ㄥ櫒銆乧pu 浠ュ強灏戦噺鍏跺畠鍏冧欢闆嗘垚鍒颁竴鍧楀彲鑳?鍙湁涓夊崄骞虫柟鍘樼背鐨勫崱涓娿€傚湪杩欐牱鐨勭郴缁熶笂锛屼綘鐨?`arch/.../mach-.../board-*.c`
鏂囦欢涓昏鎻愪緵鍏充簬姝ょ被鍗℃墍鎻掑叆鐨勪富鏉夸笂鐨勮澶囦俊鎭€傝繖褰撶劧鍖呮嫭閫氳繃鍗¤繛鎺ュ櫒
鎺ヤ笂鐨?SPI 璁惧锛?

##### 闈為潤鎬侀厤缃?

褰?Linux 鍖呭惈閫氳繃 SPI 瀵?MMC/SD/SDIO/DataFlash 鍗＄殑鏀寔鏃讹紝杩欎簺閰嶇疆涔熷皢鏄?鍔ㄦ€佺殑銆傚垢杩愮殑鏄紝杩欑被璁惧閮芥敮鎸佸熀鏈殑璁惧璇嗗埆鎺㈡祴锛屽洜姝ゅ畠浠簲褰撹兘姝ｅ父
鐑彃鎷斻€?

### 濡備綍缂栧啓鈥淪PI 鍗忚椹卞姩鈥濓紵

鐩墠澶у鏁?SPI 椹卞姩閮芥槸鍐呮牳椹卞姩锛屼絾涔熸敮鎸佺敤鎴风┖闂撮┍鍔ㄣ€傝繖閲屾垜浠彧璁ㄨ
鍐呮牳椹卞姩銆?```

	static struct spi_driver CHIP_driver = {
		.driver = {
			.name		= "CHIP",
			.pm		= &CHIP_pm_ops,
		},

		.probe		= CHIP_probe,
		.remove		= CHIP_remove,
	};

```
椹卞姩鏍稿績浼氳嚜鍔ㄥ皾璇曞皢姝ら┍鍔ㄧ粦瀹氬埌浠讳綍 board_info 缁欏嚭 modalias 涓衡€淐HIP鈥濈殑
SPI 璁惧銆傞櫎闈炰綘姝ｅ湪鍒涘缓涓€涓鐞嗘€荤嚎鐨勮澶囷紙鍑虹幇鍦?/sys/class/spi_master
涓嬶級锛屽惁鍒欎綘鐨?probe() 浠ｇ爜鍙兘鍍忚繖鏍凤細
```

	static int CHIP_probe(struct spi_device *spi)
	{
		struct CHIP			*chip;
		struct CHIP_platform_data	*pdata;

		/* assuming the driver requires board-specific data: */
		pdata = &spi->dev.platform_data;
		if (!pdata)
			return -ENODEV;

		/* get memory for driver's per-chip state */
		chip = kzalloc(sizeof *chip, GFP_KERNEL);
		if (!chip)
			return -ENOMEM;
		spi_set_drvdata(spi, chip);

		... etc
		return 0;
	}

```
涓€鏃﹁繘鍏?probe()锛岄┍鍔ㄥ氨鍙互浣跨敤鈥渟truct spi_message鈥濆悜 SPI 璁惧鍙戣捣 I/O
璇锋眰銆傚綋 remove() 杩斿洖锛屾垨 probe() 澶辫触涔嬪悗锛岄┍鍔ㄤ繚璇佷笉浼氬啀鎻愪氦浠讳綍姝ょ被
娑堟伅銆?
  - 涓€涓?spi_message 鏄竴涓插崗璁搷浣滃簭鍒楋紝浣滀负涓€涓師瀛愬簭鍒楁墽琛屻€係PI 椹卞姩
    鎺у埗鍖呮嫭锛?
      - 鍙屽悜璇诲啓浣曟椂寮€濮嬧€︹€︾敱鍏?spi_transfer 璇锋眰搴忓垪鐨勬帓鍒楁柟寮忓喅瀹氾紱

      - 浣跨敤鍝簺 I/O 缂撳啿鍖衡€︹€︽瘡涓?spi_transfer 涓烘瘡涓紶杈撴柟鍚戝寘瑁呬竴涓?        缂撳啿鍖猴紝鏀寔鍏ㄥ弻宸ワ紙涓や釜鎸囬拡锛屼袱绉嶆儏鍐靛彲鑳界浉鍚岋級鍜屽崐鍙屽伐
        锛堜竴涓寚閽堜负 NULL锛変紶杈擄紱

      - 鍙€夊湴鍦ㄤ紶杈撲箣鍚庡畾涔夌煭寤舵椂鈥︹€︿娇鐢?spi_transfer.delay.value 璁剧疆
        锛堝鏋滅紦鍐插尯闀垮害涓洪浂锛岃寤舵椂鍙互鏄敮涓€鐨勫崗璁晥鏋滐級鈥︹€︽寚瀹氭寤舵椂鏃?        榛樿鐨?spi_transfer.delay.unit 鏄井绉掞紝浣嗗鏈夐渶瑕佸彲浠ヨ皟鏁翠负鏃堕挓
        鍛ㄦ湡鎴栫撼绉掞紱

      - 浼犺緭涔嬪悗鐗囬€夋槸鍚﹀彉涓烘棤鏁堜互鍙婃槸鍚﹀甫寤舵椂鈥︹€︿娇鐢?spi_transfer.cs_change
        鏍囧織锛?
      - 鎻愮ず涓嬩竴鏉℃秷鎭槸鍚﹀彲鑳藉彂寰€鍚屼竴璁惧鈥︹€︿娇鐢ㄩ偅涓師瀛愮粍涓渶鍚庝竴娆?        浼犺緭涓婄殑 spi_transfer.cs_change 鏍囧織锛屽苟鍙兘鑺傜渷鐗囬€夊彇娑堥€変腑鍜?        閫変腑鐨勫紑閿€銆?
  - 閬靛惊鏍囧噯鍐呮牳瑙勫垯锛屽苟鍦ㄤ綘鐨勬秷鎭腑鎻愪緵 DMA 瀹夊叏鐨勭紦鍐插尯銆傝繖鏍蜂娇鐢?DMA 鐨?    鎺у埗鍣ㄩ┍鍔ㄥ氨涓嶅繀鍋氶澶栫殑鎷疯礉锛岄櫎闈炵‖浠舵湁姝よ姹傦紙渚嬪缁曞紑寮哄埗浣跨敤
    鍙嶅脊缂撳啿锛坆ounce buffering锛夌殑纭欢 errata锛夈€?
  - 鍩烘湰鐨?I/O 鍘熻鏄?spi_async()銆傚紓姝ヨ姹傚彲浠ュ湪浠讳綍涓婁笅鏂囷紙irq 澶勭悊绋嬪簭銆?    浠诲姟绛夛級涓彂璧凤紝瀹屾垚閫氳繃娑堟伅闄勫甫鐨勫洖璋冩姤鍛娿€傚湪妫€娴嬪埌浠讳綍閿欒涔嬪悗锛岃姱鐗?    琚彇娑堥€変腑锛屽苟涓旇 spi_message 鐨勫鐞嗚涓銆?
  - 杩樻湁鍍?spi_sync() 杩欐牱鐨勫悓姝ュ皝瑁咃紝浠ュ強鍍?spi_read()銆乻pi_write() 鍜?    spi_write_then_read() 杩欐牱鐨勫皝瑁呫€傝繖浜涘彧鑳藉湪鍙兘鐫＄湢鐨勪笂涓嬫枃涓彂璧凤紝
    鑰屼笖瀹冧滑閮芥槸浣嶄簬 spi_async() 涔嬩笂鐨勫共鍑€锛堜笖灏忓阀銆佲€滃彲閫夆€濓級灞傘€?
  - spi_write_then_read() 璋冪敤浠ュ強鍥寸粫瀹冪殑渚挎嵎灏佽锛屽彧搴斿湪鏁版嵁閲忚緝灏忋€佸彲浠?    蹇界暐涓€娆￠澶栨嫹璐濆紑閿€鐨勬儏鍐典笅浣跨敤銆傚畠鏃ㄥ湪鏀寔甯歌鐨?RPC 寮忚姹傦紝渚嬪
    鍐欎竴涓?8 浣嶅懡浠ゅ苟璇讳竴涓?16 浣嶅搷搴斺€斺€攕pi_w8r16() 灏辨槸鍏跺皝瑁呬箣涓€锛屽仛鐨?    姝ｆ槸杩欎欢浜嬨€?
鏈変簺椹卞姩鍙兘闇€瑕佷慨鏀?spi_device 鐨勭壒鎬э紝渚嬪浼犺緭妯″紡銆佸瓧闀挎垨鏃堕挓閫熺巼銆傝繖
閫氳繃 spi_setup() 瀹屾垚锛屽畠閫氬父搴斿湪绗竴娆″璁惧鍋?I/O 涔嬪墠浠?probe() 璋冪敤銆?涓嶈繃锛屼篃鍙互鍦ㄨ璁惧娌℃湁浠讳綍娑堟伅鎸傝捣鏃剁殑浠讳綍鏃跺埢璋冪敤銆?
铏界劧鈥渟pi_device鈥濇槸椹卞姩鐨勪笅杈圭晫锛屼笂杈圭晫鍙兘鍖呮嫭 sysfs锛堝挨鍏舵槸浼犳劅鍣ㄨ鏁帮級銆?杈撳叆灞傘€丄LSA銆佺綉缁溿€丮TD銆佸瓧绗﹁澶囨鏋讹紝鎴栧叾瀹?Linux 瀛愮郴缁熴€?
娉ㄦ剰锛屼綔涓轰笌 SPI 璁惧浜や簰鐨勪竴閮ㄥ垎锛屼綘鐨勯┍鍔ㄥ繀椤荤鐞嗕袱绫诲唴瀛樸€?
  - I/O 缂撳啿鍖轰娇鐢ㄩ€氬父鐨?Linux 瑙勫垯锛屽苟涓斿繀椤绘槸 DMA 瀹夊叏鐨勩€備綘閫氬父搴斾粠
    鍫嗘垨绌洪棽椤垫睜涓垎閰嶅畠浠€備笉瑕佷娇鐢ㄦ爤锛屾垨浠讳綍琚０鏄庝负鈥渟tatic鈥濈殑涓滆タ銆?
  - 鐢ㄤ簬灏嗛偅浜?I/O 缂撳啿鍖虹矘鍚堜负涓€缁勫崗璁簨鍔＄殑 spi_message 鍜?spi_transfer
    鍏冩暟鎹€傝繖浜涘彲浠ュ湪鏂逛究鐨勪换浣曞湴鏂瑰垎閰嶏紝鍖呮嫭浣滀负鍏跺畠涓€娆℃€у垎閰嶇殑椹卞姩
    鏁版嵁缁撴瀯鐨勪竴閮ㄥ垎銆傚皢杩欎簺娓呴浂鍒濆鍖栥€?
濡傛灉浣犳効鎰忥紝鍙互浣跨敤 spi_message_alloc() 鍜?spi_message_free() 渚挎嵎渚嬬▼鏉?鍒嗛厤骞堕浂鍒濆鍖栦竴涓甫鏈夊涓紶杈撶殑 spi_message銆?

### 濡備綍缂栧啓鈥淪PI 鎺у埗鍣ㄩ┍鍔ㄢ€濓紵

涓€涓?SPI 鎺у埗鍣ㄥぇ姒備細娉ㄥ唽鍦?platform_bus 涓婏紱缂栧啓涓€涓┍鍔ㄦ潵缁戝畾鍒拌璁惧锛?鏃犺娑夊強鐨勬槸鍝潯鎬荤嚎銆?
杩欑被椹卞姩鐨勪富瑕佷换鍔℃槸鎻愪緵涓€涓€渟pi_controller鈥濄€備娇鐢?spi_alloc_host() 鍒嗛厤
涓绘満鎺у埗鍣紝浣跨敤 spi_controller_get_devdata() 鑾峰彇涓鸿璁惧鍒嗛厤鐨勯┍鍔ㄧ鏈?鏁版嵁銆?```

	struct spi_controller	*ctlr;
	struct CONTROLLER	*c;

	ctlr = spi_alloc_host(dev, sizeof *c);
	if (!ctlr)
		return -ENODEV;

	c = spi_controller_get_devdata(ctlr);

```
椹卞姩灏嗗垵濮嬪寲璇?spi_controller 鐨勫瓧娈碉紝鍖呮嫭鎬荤嚎鍙凤紙涔熻涓?platform 璁惧 ID
鐩稿悓锛変互鍙婁笌 SPI 鏍稿績鍜?SPI 鍗忚椹卞姩浜や簰鐨勪笁涓柟娉曘€傚畠杩樹細鍒濆鍖栬嚜宸辩殑鍐呴儴
鐘舵€併€傦紙鍏充簬鎬荤嚎缂栧彿鍜岄偅浜涙柟娉曪紝瑙佷笅鏂囥€傦級

鍦ㄤ綘鍒濆鍖?spi_controller 涔嬪悗锛屼娇鐢?spi_register_controller() 灏嗗叾鍙戝竷鍒?绯荤粺鐨勫叾浣欓儴鍒嗐€傛鏃讹紝鎺у埗鍣ㄥ拰浠讳綍棰勫厛澹版槑鐨?spi 璁惧鐨勮澶囪妭鐐归兘灏嗗彲鐢紝
椹卞姩妯″瀷鏍稿績浼氳礋璐ｅ皢瀹冧滑缁戝畾鍒伴┍鍔ㄣ€?
濡傛灉浣犻渶瑕佺Щ闄や綘鐨?SPI 鎺у埗鍣ㄩ┍鍔紝spi_unregister_controller() 灏嗛€嗚浆
spi_register_controller() 鐨勬晥鏋溿€?

##### 鎬荤嚎缂栧彿


鎬荤嚎缂栧彿寰堥噸瑕侊紝鍥犱负 Linux 姝ｆ槸鐢ㄥ畠鏉ヨ瘑鍒粰瀹?SPI 鎬荤嚎锛堝叡浜?SCK銆丮OSI銆?MISO锛夌殑銆傛湁鏁堢殑鎬荤嚎鍙蜂粠闆跺紑濮嬨€傚湪 SOC 绯荤粺涓婏紝鎬荤嚎鍙峰簲涓庤姱鐗囧埗閫犲晢瀹氫箟鐨?缂栧彿鐩稿尮閰嶃€備緥濡傦紝纭欢鎺у埗鍣?SPI2 灏嗘槸鎬荤嚎鍙?2锛岃繛鎺ュ埌瀹冪殑璁惧鐨?spi_board_info 灏嗕娇鐢ㄨ缂栧彿銆?
濡傛灉浣犳病鏈夎繖鏍风殑纭欢鍒嗛厤鐨勬€荤嚎鍙凤紝鍙堝洜涓烘煇绉嶅師鍥犳棤娉曡嚜琛屽垎閰嶏紝閭ｄ箞鎻愪緵涓€涓?璐熺殑鎬荤嚎鍙枫€傞殢鍚庡畠灏嗚涓€涓姩鎬佸垎閰嶇殑缂栧彿鏇挎崲銆傝繖鏃朵綘闇€瑕佸皢鍏惰涓洪潪闈欐€?閰嶇疆锛堣涓婃枃锛夈€?

##### SPI 涓绘満鎺у埗鍣ㄦ柟娉?

`ctlr->setup(struct spi_device *spi)`
	杩欎細璁剧疆璁惧鏃堕挓閫熺巼銆丼PI 妯″紡鍜屽瓧闀裤€傞┍鍔ㄥ彲浠ヤ慨鏀?board_info 鎻愪緵鐨?	榛樿鍊硷紝鐒跺悗璋冪敤 spi_setup(spi) 鏉ヨ皟鐢ㄦ渚嬬▼銆傚畠鍙兘浼氱潯鐪犮€?
	闄ら潪姣忎釜 SPI 鐩爣閮芥湁鑷繁鐨勯厤缃瘎瀛樺櫒锛屽惁鍒欎笉瑕佺珛鍗充慨鏀瑰畠浠€︹€﹀惁鍒?	椹卞姩鍙兘浼氱牬鍧忔鍦ㄤ负鍏跺畠 SPI 璁惧杩涜鐨?I/O銆?```

		BUG ALERT:  for some reason the first version of
		many spi_controller drivers seems to get this wrong.
		When you code setup(), ASSUME that the controller
		is actively processing transfers for another device.

```
`ctlr->cleanup(struct spi_device *spi)`
	浣犵殑鎺у埗鍣ㄩ┍鍔ㄥ彲浠ヤ娇鐢?spi_device.controller_state 鏉ヤ繚瀛樺畠鍔ㄦ€佸叧鑱斿埌
	璇ヨ澶囩殑鐘舵€併€傚鏋滀綘杩欐牱鍋氾紝鍔″繀鎻愪緵 cleanup() 鏂规硶鏉ラ噴鏀捐鐘舵€併€?
`ctlr->prepare_transfer_hardware(struct spi_controller *ctlr)`
	闃熷垪鏈哄埗浼氳皟鐢ㄥ畠锛屽悜椹卞姩鍙戝嚭淇″彿琛ㄧず涓€鏉℃秷鎭嵆灏嗗埌鏉ワ紝浜庢槸瀛愮郴缁熻姹?	椹卞姩閫氳繃鍙戣捣姝よ皟鐢ㄦ潵鍑嗗浼犺緭纭欢銆傚畠鍙兘浼氱潯鐪犮€?
`ctlr->unprepare_transfer_hardware(struct spi_controller *ctlr)`
	闃熷垪鏈哄埗浼氳皟鐢ㄥ畠锛屽悜椹卞姩鍙戝嚭淇″彿琛ㄧず闃熷垪涓病鏈夋洿澶氭寕璧风殑娑堟伅锛屽畠鍙互
	鏀炬澗纭欢锛堜緥濡傞€氳繃鐢垫簮绠＄悊璋冪敤锛夈€傚畠鍙兘浼氱潯鐪犮€?
`ctlr->transfer_one_message(struct spi_controller **ctlr, struct spi_message **mesg)`
	瀛愮郴缁熻皟鐢ㄩ┍鍔ㄦ潵浼犺緭鍗曟潯娑堟伅锛屽悓鏃舵妸鏈熼棿鍒拌揪鐨勪紶杈撴帓鍏ラ槦鍒椼€傚綋椹卞姩
	瀹屾垚姝ゆ秷鎭椂锛屽畠蹇呴』璋冪敤 spi_finalize_current_message()锛屼互渚垮瓙绯荤粺
	鍙互鍙戝嚭涓嬩竴鏉℃秷鎭€傚畠鍙兘浼氱潯鐪犮€?
`ctrl->transfer_one(struct spi_controller **ctlr, struct spi_device **spi, struct spi_transfer *transfer)`
	瀛愮郴缁熻皟鐢ㄩ┍鍔ㄦ潵浼犺緭鍗曚釜浼犺緭锛屽悓鏃舵妸鏈熼棿鍒拌揪鐨勪紶杈撴帓鍏ラ槦鍒椼€傚綋椹卞姩
	瀹屾垚姝や紶杈撴椂锛屽畠蹇呴』璋冪敤 spi_finalize_current_transfer()锛屼互渚垮瓙绯荤粺
	鍙互鍙戝嚭涓嬩竴涓紶杈撱€傚畠鍙兘浼氱潯鐪犮€傛敞鎰忥細transfer_one 鍜?transfer_one_message
	鏄簰鏂ョ殑锛涘綋涓よ€呴兘琚缃椂锛岄€氱敤瀛愮郴缁熶笉浼氳皟鐢ㄤ綘鐨?transfer_one 鍥炶皟銆?
	杩斿洖鍊硷細

 - 璐熺殑 errno锛氶敊璇? - 0锛氫紶杈撳凡瀹屾垚
 - 1锛氫紶杈撲粛鍦ㄨ繘琛屼腑

`ctrl->set_cs_timing(struct spi_device *spi, u8 setup_clk_cycles, u8 hold_clk_cycles, u8 inactive_clk_cycles)`
	姝ゆ柟娉曞厑璁?SPI 瀹㈡埛绔┍鍔ㄨ姹?SPI 涓绘満鎺у埗鍣ㄩ厤缃澶囩壒瀹氱殑 CS 寤虹珛銆?	淇濇寔鍜屾棤鏁堟椂搴忚姹傘€?
##### 宸插純鐢ㄧ殑鏂规硶


`ctrl->transfer(struct spi_device **spi, struct spi_message **message)`
	杩欑粷涓嶈兘鐫＄湢銆傚畠鐨勮亴璐ｆ槸瀹夋帓浼犺緭鍙戠敓锛屽苟鍙戝嚭鍏?complete() 鍥炶皟銆傝繖涓や欢浜?	閫氬父绋嶅悗鍙戠敓锛屽湪鍏跺畠浼犺緭瀹屾垚涔嬪悗锛岃€屽鏋滄帶鍒跺櫒绌洪棽锛屽垯闇€瑕佽 kickstart銆?	姝ゆ柟娉曚笉鐢ㄤ簬鎺掗槦寮忔帶鍒跺櫒锛屽苟涓斿湪瀹炵幇浜?transfer_one_message() 鍜?	(un)prepare_transfer_hardware() 鏃跺繀椤讳负绌恒€?

##### SPI 娑堟伅闃熷垪


濡傛灉浣犲 SPI 瀛愮郴缁熸彁渚涚殑鏍囧噯鎺掗槦鏈哄埗鎰熷埌婊℃剰锛屽彧闇€瀹炵幇涓婇潰鎸囧畾鐨勬帓闃熸柟娉?鍗冲彲銆備娇鐢ㄦ秷鎭槦鍒楃殑濂藉鏄彲浠ラ泦涓ぇ閲忎唬鐮侊紝骞舵彁渚涙柟娉曠殑绾繘绋嬩笂涓嬫枃鎵ц銆?鍦ㄩ珮浼樺厛绾?SPI 娴侀噺涓嬶紝娑堟伅闃熷垪涔熷彲浠ユ彁鍗囧埌瀹炴椂浼樺厛绾с€?
闄ら潪閫夋嫨浜?SPI 瀛愮郴缁熷唴鐨勬帓闃熸満鍒讹紝鍚﹀垯椹卞姩鐨勫ぇ閮ㄥ垎宸ヤ綔灏嗘槸绠＄悊鐢辩幇宸插純鐢?鐨?transfer() 鍑芥暟鎵€鍠傚叆鐨?I/O 闃熷垪銆?
閭ｄ釜闃熷垪鍙互鏄函姒傚康涓婄殑銆備緥濡傦紝涓€涓粎鐢ㄤ簬浣庨浼犳劅鍣ㄨ闂殑椹卞姩锛屼娇鐢ㄥ悓姝?PIO 鍙兘灏辫冻澶熶簡銆?
浣嗛偅涓槦鍒楀緢鍙兘鏄潪甯哥湡瀹炵殑锛屼娇鐢?message->queue銆丳IO锛岀粡甯?DMA锛堢壒鍒槸
濡傛灉鏍规枃浠剁郴缁熶綅浜?SPI flash 涓級锛屼互鍙婂儚 IRQ 澶勭悊绋嬪簭銆乼asklet 鎴栧伐浣滈槦鍒?锛堝 keventd锛夎繖鏍风殑鎵ц涓婁笅鏂囥€備綘鐨勯┍鍔ㄥ彲浠ュ浣犻渶瑕侀偅鑸姳鍝紝鎴栭偅鑸畝鍗曘€?杩欐牱鐨?transfer() 鏂规硶閫氬父鍙槸鎶婃秷鎭姞鍏ラ槦鍒楋紝鐒跺悗鍚姩鏌愪釜寮傛浼犺緭寮曟搸
锛堥櫎闈炲畠宸茬粡鍦ㄨ繍琛岋級銆?

### SPI 鍗忚鐨勬墿灞?
SPI 娌℃湁姝ｅ紡鐨勮鑼冩垨鏍囧噯锛岃繖涓€浜嬪疄浣垮緱鑺墖鍒堕€犲晢鍙互浠ョ暐鏈変笉鍚岀殑鏂瑰紡瀹炵幇
SPI 鍗忚銆傚湪澶у鏁版儏鍐典笅锛屾潵鑷笉鍚屽巶鍟嗙殑 SPI 鍗忚瀹炵幇褰兼鍏煎銆備緥濡傦紝鍦?SPI 妯″紡 0锛圕POL=0锛孋PHA=0锛変笅锛屾€荤嚎淇″彿绾垮彲鑳借〃鐜板涓嬶細
```

  nCSx ___                                                                   ___
          \_________________________________________________________________/
          鈥?                                                                鈥?          鈥?                                                                鈥?  SCLK         ___     ___     ___     ___     ___     ___     ___     ___
       _______/   \___/   \___/   \___/   \___/   \___/   \___/   \___/   \_____
          鈥?  :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; 鈥?          鈥?  :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; 鈥?  MOSI XXX__________         _______                 _______         ________XXX
  0xA5 XXX__/ 1     \_0_____/ 1     \_0_______0_____/ 1     \_0_____/ 1    \_XXX
          鈥?      ;       ;       ;       ;       ;       ;       ;       ; 鈥?          鈥?      ;       ;       ;       ;       ;       ;       ;       ; 鈥?  MISO XXX__________         _______________________          _______        XXX
  0xBA XXX__/     1 \_____0_/     1       1       1 \_____0__/    1  \____0__XXX

```
```

  鈥?marks the start/end of transmission;
  : marks when data is clocked into the peripheral;
  ; marks when data is clocked into the controller;
  X marks when line states are not specified.

```
鍦ㄥ皯鏁版儏鍐典笅锛岃姱鐗囬€氳繃鎸囧畾鍏跺畠 SPI 鍗忚涓嶄娇鐢ㄧ殑淇″彿绾胯涓猴紙渚嬪 CS 鏈柇瑷€鏃?鐨勬暟鎹嚎鐘舵€侊級鏉ユ墿灞?SPI 鍗忚銆傞偅浜涗笉鍚岀殑 SPI 鍗忚銆佹ā寮忓拰閰嶇疆鐢变笉鍚岀殑 SPI
妯″紡鏍囧織鏀寔銆?
##### MOSI 绌洪棽鐘舵€侀厤缃?

甯歌鐨?SPI 鍗忚瀹炵幇娌℃湁涓烘帶鍒跺櫒鏈椂閽熻緭鍑烘暟鎹椂鐨?MOSI 绾挎寚瀹氫换浣曠姸鎬佹垨
琛屼负銆傜劧鑰岋紝纭疄瀛樺湪涓€浜涘璁撅紝瑕佹眰鍦ㄦ湭鏃堕挓杈撳嚭鏁版嵁鏃?MOSI 绾垮浜庣壒瀹氱姸鎬併€?渚嬪锛屽鏋滃璁炬湡鏈涘湪鎺у埗鍣ㄦ湭鏃堕挓杈撳嚭鏁版嵁鏃?MOSI 绾夸负楂樼數骞筹紙`SPI_MOSI_IDLE_HIGH`锛夛紝
閭ｄ箞 SPI 妯″紡 0 涓嬬殑浼犺緭鐪嬭捣鏉ュ涓嬶細
```

  nCSx ___                                                                   ___
          \_________________________________________________________________/
          鈥?                                                                鈥?          鈥?                                                                鈥?  SCLK         ___     ___     ___     ___     ___     ___     ___     ___
       _______/   \___/   \___/   \___/   \___/   \___/   \___/   \___/   \_____
          鈥?  :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; 鈥?          鈥?  :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; 鈥?  MOSI _____         _______         _______         _______________         ___
  0x56      \_0_____/ 1     \_0_____/ 1     \_0_____/ 1       1     \_0_____/
          鈥?      ;       ;       ;       ;       ;       ;       ;       ; 鈥?          鈥?      ;       ;       ;       ;       ;       ;       ;       ; 鈥?  MISO XXX__________         _______________________          _______        XXX
  0xBA XXX__/     1 \_____0_/     1       1       1 \_____0__/    1  \____0__XXX

```
```

  鈥?marks the start/end of transmission;
  : marks when data is clocked into the peripheral;
  ; marks when data is clocked into the controller;
  X marks when line states are not specified.

```
鍦ㄥ閫氬父 SPI 鍗忚鐨勮繖涓墿灞曚腑锛孧OSI 绾跨姸鎬佽鎸囧畾涓猴細鍦?CS 鏂█浣嗘帶鍒跺櫒鏈?鏃堕挓杈撳嚭鏁版嵁缁欏璁炬椂锛屼互鍙婂湪 CS 鏈柇瑷€鏃讹紝閮戒繚鎸佷负楂樼數骞炽€?
闇€瑕佹鎵╁睍鐨勫璁惧繀椤婚€氳繃鍦ㄥ叾 ``struct spi_device`` 鐨?mode 灞炴€т腑璁剧疆
`SPI_MOSI_IDLE_HIGH` 浣嶅苟璋冪敤 spi_setup() 鏉ヨ姹傚畠銆傛敮鎸佹鎵╁睍鐨勬帶鍒跺櫒搴?閫氳繃鍦ㄥ叾 `struct spi_controller` 鐨?mode_bits 灞炴€т腑璁剧疆 `SPI_MOSI_IDLE_HIGH`
鏉ヨ〃鏄庤繖涓€鐐广€傚皢 MOSI 绌洪棽涓轰綆鐢靛钩鐨勯厤缃笌涔嬬被浼硷紝浣嗕娇鐢?`SPI_MOSI_IDLE_LOW`
妯″紡浣嶃€?

### 鑷磋阿

瀵?Linux-SPI 璁ㄨ鍋氬嚭璐＄尞鐨勪汉鍖呮嫭锛堟寜濮撴皬瀛楁瘝椤哄簭锛夛細

- Mark Brown
- David Brownell
- Russell King
- Grant Likely
- Dmitry Pervushin
- Stephen Street
- Mark Underwood
- Andrew Victor
- Linus Walleij
- Vitaly Wool
