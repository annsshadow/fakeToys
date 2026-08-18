## Intel IXP4xx 缃戠粶澶勭悊鍣ㄤ笂鐨?Linux 鍙戣璇存槑


### 缁存姢鑰咃細Deepak Saxena <dsaxena@plexity.net>


1. 姒傝堪

Intel 鐨?IXP4xx 缃戠粶澶勭悊鍣ㄦ槸涓€娆鹃珮搴﹂泦鎴愮殑 SoC锛岄潰鍚戠綉缁滃簲鐢紝浣嗙敱浜庡叾浣庢垚鏈笌浣?鍔熻€楋紝鍦ㄥ伐涓氭帶鍒跺強鍏朵粬棰嗗煙涔熼鍙楁杩庛€侷XP4xx 绯诲垪鐩墠鍖呭惈鑻ュ共鏀寔涓嶅悓缃戠粶鍗歌浇鍔熻兘
锛堝鍔犲瘑銆佽矾鐢便€侀槻鐏绛夛級鐨勫鐞嗗櫒銆侷XP46x 绯诲垪鏄竴涓洿鏂扮増鏈紝鏀寔鏇撮珮鐨勯€熷害銆佹柊鐨?鍐呭瓨涓庨棯瀛橀厤缃紝浠ュ強鏇撮珮鐨勯泦鎴愬害锛堜緥濡傜墖鍐?I2C 鎺у埗鍣級銆?
鍏充簬璇?CPU 鍚勪釜鐗堟湰鐨勬洿澶氫俊鎭紝璇峰弬闃咃細

   http://developer.intel.com/design/network/products/npfamily/ixp4xx.htm

Intel 杩樻浘鐢熶骇杩?IXCP1100 CPU锛屽畠鏄幓闄や簡澶ч噺缃戠粶鏅鸿兘鐨?IXP4xx銆?
2. Linux 鏀寔

Linux 鐩墠鍦?IXP4xx 鑺墖涓婃敮鎸佷互涓嬬壒鎬э細

- 鍙屼覆鍙?- PCI 鎺ュ彛
- 闂瓨璁块棶锛圡TD/JFFS锛?- IXP42x 涓婇€氳繃 GPIO 瀹炵幇鐨?I2C
- 鐢ㄤ簬杈撳叆/杈撳嚭/涓柇鐨?GPIO
  璁块棶鍑芥暟鍙傝 arch/arm/mach-ixp4xx/include/mach/platform.h銆?- 瀹氭椂鍣紙鐪嬮棬鐙椼€佹搷浣滅郴缁燂級

浠ヤ笅鑺墖缁勪欢涓嶅彈 Linux 鏀寔锛岄渶瑕佷娇鐢?Intel 涓撴湁鐨?CSR 杞欢锛?
- USB 璁惧鎺ュ彛
- 缃戠粶鎺ュ彛锛圚SS銆乁topia銆丯PE 绛夛級
- 缃戠粶鍗歌浇鍔熻兘

濡傛灉浣犻渶瑕佷娇鐢ㄤ笂杩颁换浣曞姛鑳斤紝闇€瑕佷粠浠ヤ笅鍦板潃涓嬭浇 Intel 鐨勮蒋浠讹細

   http://developer.intel.com/design/network/products/npfamily/ixp425.htm

璇峰嬁灏变笓鏈夎蒋浠跺悜 Linux 閭欢鍒楄〃鎻愰棶銆?
鏈夊涓綉绔欐彁渚涗娇鐢?Intel 杞欢鐨勬寚寮?绾跨储锛?
   - http://sourceforge.net/projects/ixp4xx-osdg/
    浣跨敤 uClinux 涓?Intel 搴撶殑寮€婧愬紑鍙戣€呮寚鍗?
   - http://gatewaymaker.sourceforge.net/
    浣跨敤 IXP425 涓?Linux 鏋勫缓缃戝叧鐨勭畝鍗曞崟椤垫憳瑕?
   - http://ixp425.sourceforge.net/
    渚濊禆 Intel 搴撶殑 IXP425 ATM 璁惧椹卞姩

3. 宸茬煡闂/闄愬埗

3a. 鍏ョ珯 PCI 绐楀彛鏈夐檺

IXP4xx 绯诲垪鏈€澶氭敮鎸?256MB 鍐呭瓨锛屼絾 PCI 鎺ュ彛鍙兘灏嗗叾涓?64MB 鏆撮湶缁?PCI 鎬荤嚎銆傝繖鎰忓懗鐫€
濡傛灉浣犺繍琛岀殑鍐呭瓨澶т簬 64MB锛屾墍鏈夎秴鍑哄彲璁块棶鑼冨洿鐨?PCI 缂撳啿鍖洪兘灏嗛€氳繃
arch/arm/common/dmabounce.c 涓殑渚嬬▼杩涜鍙嶅脊锛坆ounce锛夈€?
3b. 鍑虹珯 PCI 绐楀彛鏈夐檺

IXP4xx 鎻愪緵涓ょ璁块棶 PCI 鍐呭瓨绌洪棿鐨勬柟娉曪細

1) 浠?0x48000000 鍒?0x4bffffff 鐨勭洿鎺ユ槧灏勭獥鍙ｏ紙64MB锛夈€?   瑕侀€氳繃姝ょ┖闂磋闂?PCI锛屾垜浠彧闇€灏?BAR 浣跨敤 ioremap() 鏄犲皠鍒板唴鏍镐腑锛屽嵆鍙娇鐢?   鏍囧噯鐨?read[bwl]/write[bwl] 瀹忋€傜敱浜庨€熷害鍘熷洜杩欐槸棣栭€夋柟娉曪紝浣嗗畠灏嗙郴缁熼檺鍒朵负浠?   64MB 鐨?PCI 鍐呭瓨銆傚湪浣跨敤鏄惧崱鍙婂叾浠栭珮鍐呭瓨鍗犵敤璁惧鏃讹紝杩欏彲鑳戒細鎴愪负闂銆?
2) 濡傛灉闇€瑕佸ぇ浜?64MB 鐨勫唴瀛樼┖闂达紝鍙皢 IXP4xx 閰嶇疆涓轰娇鐢ㄩ棿鎺ュ瘎瀛樺櫒鏉ヨ闂?PCI銆傝繖鍏佽
   鎬荤嚎涓婃渶澶?128MB锛?x48000000 鍒?0x4fffffff锛夌殑鍐呭瓨銆傚叾缂虹偣鍦ㄤ簬姣忔 PCI 璁块棶閮介渶瑕?   涓夋鏈湴瀵勫瓨鍣ㄨ闂鍔犱竴鎶婅嚜鏃嬮攣锛屼絾鍦ㄦ煇浜涙儏鍐典笅鎬ц兘鎹熷け鏄彲浠ユ帴鍙楃殑銆傛澶栵紝鐢变簬
   PCI 绐楀彛鐨勯棿鎺ョ壒鎬э紝杩欑鎯呭喌涓嬫棤娉曞 PCI 璁惧杩涜 mmap()銆?
榛樿鎯呭喌涓嬶紝鍑轰簬鎬ц兘鑰冭檻浣跨敤鐩存帴鏂规硶銆傚鏋滀綘闇€瑕佹洿澶?PCI 鍐呭瓨锛岃鍚敤
IXP4XX_INDIRECT_PCI 閰嶇疆閫夐」銆?
3c. GPIO 浣滀负涓柇

鐩墠浠ｇ爜浠呭鐞嗙數骞虫晱鎰燂紙level-sensitive锛夌殑 GPIO 涓柇銆?
4. 鏀寔鐨勫钩鍙?
ADI Engineering Coyote 缃戝叧鍙傝€冨钩鍙?http://www.adiengineering.com/productsCoyote.html

   ADI Coyote 骞冲彴鏄负鏋勫缓灏忓瀷浣忓畢/鍔炲叕缃戝叧鑰呮彁渚涚殑鍙傝€冭璁°€備竴涓?NPE 杩炴帴鍒?   10/100 鎺ュ彛锛屼竴涓繛鎺ュ埌 4 绔彛 10/100 浜ゆ崲鏈猴紝绗笁涓繛鎺ュ埌 ADSL 鎺ュ彛銆傛澶栵紝
   瀹冭繕鏀寔閫氳繃 SLIC 杩炴帴鐨?POTs 鎺ュ彛銆傝娉ㄦ剰杩欎簺涓嶅彈 Linux ATM 鏀寔銆傛渶鍚庯紝璇ュ钩鍙?   鏈変袱涓敤浜?802.11[bga] 鍗＄殑 mini-PCI 鎻掓Ы銆傛澶栵紝鎵╁睍鎬荤嚎涓婃寕鏈変竴涓?IDE 绔彛銆?
Gateworks Avila 缃戠粶骞冲彴
http://www.gateworks.com/support/overview.php

   Avila 骞冲彴鍩烘湰涓婂氨鏄?IXDP425锛屽彧鏄皢 4 涓?PCI 鎻掓Ы鏇挎崲涓?mini-PCI 鎻掓Ы锛屽苟鍦?   鎵╁睍鎬荤嚎涓婃寕浜嗕竴涓?CF IDE 鎺ュ彛銆?
Intel IXDP425 寮€鍙戝钩鍙?http://www.intel.com/design/network/products/npfamily/ixdpg425.htm

   杩欐槸 Intel 閽堝 IXDP425 鐨勬爣鍑嗗弬鑰冨钩鍙帮紝涔熻绉颁负 Richfield 鏉裤€傚畠鍖呭惈 4 涓?   PCI 鎻掓Ы銆?6MB 闂瓨銆佷袱涓?10/100 绔彛浠ュ強涓€涓?ADSL 绔彛銆?
Intel IXDP465 寮€鍙戝钩鍙?http://www.intel.com/design/network/products/npfamily/ixdp465.htm

   杩欏熀鏈笂鏄甫鏈?IXP465 浠ュ強 32MB 闂瓨锛堣€岄潪浠?16MB锛夌殑 IXDP425銆?
Intel IXDPG425 寮€鍙戝钩鍙?
   杩欏熀鏈笂鏄甫鏈夋柊澧?NEC EHCI 鎺у埗鍣ㄧ殑 ADI Coyote 鏉裤€傝鏉跨殑涓€涓棶棰樻槸 mini-PCI
   鎻掓Ы浠呰繛鎺ヤ簡 3.3v 渚涚數绾匡紝鍥犳浣犳棤娉曚娇鐢ㄥ甫鏈?E100 鍗＄殑 PCI 杞?mini-PCI 閫傞厤鍣ㄣ€?   鍥犳锛屼负浜嗕互 NFS 浣滀负鏍规枃浠剁郴缁燂紝浣犻渶瑕佷娇鐢?CSR 鎴栦竴寮?WiFi 鍗★紝浠ュ強涓€涓墽琛?   BOOTP 鐒跺悗 pivot_root 鍒?NFS 鐨?ramdisk銆?
Motorola PrPMC1100 澶勭悊鍣ㄥす灞傚崱
http://www.fountainsys.com

   PrPMC1100 鍩轰簬 IXCP1100锛岀敤浜庢彃鍏?IXP2400/2800 绯荤粺浠ュ厖褰撶郴缁熸帶鍒跺櫒銆傚畠鏉夸笂浠?   鍖呭惈涓€涓?CPU 鍜?16MB 闂瓨锛岄渶瑕佹彃鍏ヨ浇鏉挎墠鑳藉伐浣溿€傜洰鍓?Linux 浠呮敮鎸佽骞冲彴鐨?   Motorola PrPMC 杞芥澘銆?
5. 寰呭姙鍒楄〃

- 娣诲姞瀵?Coyote IDE 鐨勬敮鎸?- 娣诲姞瀵硅竟娌胯Е鍙戯紙edge-based锛塆PIO 涓柇鐨勬敮鎸?- 娣诲姞瀵规墿灞曟€荤嚎涓?CF IDE 鐨勬敮鎸?
6. 鑷磋阿

IXP4xx 鐨勫伐浣滅敱 Intel Corp. 涓?MontaVista Software, Inc. 璧勫姪銆?
浠ヤ笅浜哄＋鎻愪緵浜嗚ˉ涓?璇勮绛夛細

- Lennerty Buytenhek
- Lutz Jaenicke
- Justin Mayfield
- Robert E. Ranslam

[鎴戠煡閬撴垜閬楁紡浜嗗叾浠栦汉锛岃鍙戦偖浠剁粰鎴戜互渚胯ˉ鍏匽

-------------------------------------------------------------------------

鏈€杩戞洿鏂帮細01/04/2005
