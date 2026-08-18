


## SysKonnect driver - SKFP


|copy| Copyright 1998-2000 SysKonnect,

skfp.txt created 11-May-2000

Readme File for skfp.o v2.06



   (1) OVERVIEW
   (2) SUPPORTED ADAPTERS
   (3) GENERAL INFORMATION
   (4) INSTALLATION
   (5) INCLUSION OF THE ADAPTER IN SYSTEM START
   (6) TROUBLESHOOTING
   (7) FUNCTION OF THE ADAPTER LEDS
   (8) HISTORY



## 1. Overview


鏈?README 浠嬬粛濡備綍鍦ㄤ綘鐨勭綉缁滈€傞厤鍣ㄤ笂浣跨敤 Linux 椹卞姩 'skfp'銆?
绗?2 绔狅細鍒楀嚭鏈┍鍔ㄦ敮鎸佺殑鎵€鏈夌綉缁滈€傞厤鍣ㄣ€?
绗?3 绔狅細
	   鎻愪緵涓€浜涢€氱敤淇℃伅銆?
绗?4 绔狅細鎻忚堪甯歌闂鍙婂叾瑙ｅ喅鏂规銆?
绗?5 绔狅細灞曠ず閫傞厤鍣?LED 鍔熻兘鐨勫彉鏇淬€?
绗?6 绔狅細寮€鍙戝巻鍙层€?

## 2. Supported adapters


缃戠粶椹卞姩 'skfp' 鏀寔浠ヤ笅缃戠粶閫傞厤鍣細
SysKonnect 閫傞厤鍣細

  - SK-5521 (SK-NET FDDI-UP)
  - SK-5522 (SK-NET FDDI-UP DAS)
  - SK-5541 (SK-NET FDDI-FP)
  - SK-5543 (SK-NET FDDI-LP)
  - SK-5544 (SK-NET FDDI-LP DAS)
  - SK-5821 (SK-NET FDDI-UP64)
  - SK-5822 (SK-NET FDDI-UP64 DAS)
  - SK-5841 (SK-NET FDDI-FP64)
  - SK-5843 (SK-NET FDDI-LP64)
  - SK-5844 (SK-NET FDDI-LP64 DAS)

Compaq 閫傞厤鍣紙鏈祴璇曪級锛?
  - Netelligent 100 FDDI DAS Fibre SC
  - Netelligent 100 FDDI SAS Fibre SC
  - Netelligent 100 FDDI DAS UTP
  - Netelligent 100 FDDI SAS UTP
  - Netelligent 100 FDDI SAS Fibre MIC


## 3. General Information


浠?v2.01 璧凤紝璇ラ┍鍔ㄥ凡闆嗘垚鍒?linux 鍐呮牳婧愮爜涓€傚洜姝わ紝瀹夎鏂瑰紡涓庡唴鏍告敮鎸佺殑浠讳綍鍏朵粬閫傞厤鍣ㄧ浉鍚屻€?
鍏充簬缃戠粶閫傞厤鍣ㄧ殑瀹夎锛岃鍙傞槄浣犲彂琛岀増鐨勮鏄庝功銆?
杩欒鎴戠殑宸ヤ綔杞绘澗澶氫簡 :-)

## 4. Troubleshooting


濡傛灉鍦ㄥ畨瑁呰繃绋嬩腑閬囧埌闂锛岃妫€鏌ヤ互涓嬪悇椤癸細

Problem:
	  椹卞姩鎵句笉鍒?FDDI 閫傞厤鍣ㄣ€?
Reason:
	  鍦?/proc/pci 涓煡鎵句互涓嬫潯鐩細

	     'FDDI network controller: SysKonnect SK-FDDI-PCI ...'

	  濡傛灉璇ユ潯鐩瓨鍦紝鍒?FDDI 閫傞厤鍣ㄥ凡琚郴缁熸壘鍒帮紝搴斿綋鍙互浣跨敤銆?
	  濡傛灉璇ユ潯鐩笉瀛樺湪锛屾垨鏂囦欢 '/proc/pci' 涓嶅瓨鍦紝鍒欎綘鍙兘鏈夌‖浠堕棶棰橈紝鎴栬€呭唴鏍告湭鍚敤 PCI 鏀寔銆?
	  鍙互浣跨敤 SysKonnect 缃戠珯涓婃彁渚涚殑璇婃柇绋嬪簭鏉ユ鏌ラ€傞厤鍣細

	      www.syskonnect.de

	  涓€浜?COMPAQ 鏈哄櫒鍦?Linux 涓嬪瓨鍦?PCI 鐩稿叧闂銆傝繖鍦?'PCI howto' 鏂囨。锛堝寘鍚湪鏌愪簺鍙戣鐗堜腑锛屾垨鍙粠 www 鑾峰彇锛屼緥濡?'www.linux.org'锛変腑鏈夋弿杩帮紝鐩墠娌℃湁瑙ｅ喅鍔炴硶銆?
Problem:
	  浣犳兂鎶婁綘鐨勭數鑴戠敤浣滃涓?IP 瀛愮綉锛堜娇鐢ㄥ涓€傞厤鍣級涔嬮棿鐨勮矾鐢卞櫒锛屼絾浣犳棤娉曡闂叾浠栧瓙缃戜腑鐨勮绠楁満銆?
Reason:
	  瑕佷箞鏄矾鐢卞櫒鐨勫唴鏍告湭閰嶇疆 IP 杞彂锛岃涔堟槸鑷冲皯涓€涓绠楁満涓婄殑璺敱琛ㄤ笌缃戝叧閰嶇疆鏈夐棶棰樸€?
濡傛灉浣犵殑闂鏈垪浜庢锛岃鑱旂郴鎴戜滑鐨勬妧鏈敮鎸佷互鑾峰彇甯姪銆?
浣犲彲浠ュ彂閫侀偖浠惰嚦锛歭inux@syskonnect.de

鑱旂郴鎴戜滑鐨勬妧鏈敮鎸佹椂锛岃纭繚鎻愪緵浠ヤ笅淇℃伅锛?
- System Manufacturer and Model
- Boards in your system
- Distribution
- Kernel version


## 5. Function of the Adapter LEDs


	FDDI 缃戠粶閫傞厤鍣ㄤ笂 LED 鐨勫姛鑳藉湪 SMT 鐗堟湰 v2.82 涓仛浜嗗彉鏇淬€傚湪杩欎釜鏂扮殑 SMT 鐗堟湰涓紝榛勮壊 LED 鐢ㄤ綔鐜繍琛屾寚绀恒€傞粍鑹?LED 鐐逛寒琛ㄧず鐜凡鏂紑銆傞€傞厤鍣ㄤ笂鐨勭豢鑹?LED 鐜板湪鐢ㄤ綔閾捐矾鎸囩ず锛岀豢鑹?LED 鐐逛寒琛ㄧず璇ョ鍙ｆ湁鐗╃悊杩炴帴銆?
	鍦?v2.82 涔嬪墠鐨?SMT 鐗堟湰涓紝榛勮壊 LED 鐔勭伃琛ㄧず鐜甯革紝鑰岀豢鑹?LED 鏄剧ず閫傞厤鍣ㄧ殑杩炴帴鐘舵€併€傜幆鏂紑鏃剁豢鑹?LED 鐔勭伃鑰岄粍鑹?LED 鐐逛寒銆?
	鎵€鏈夊疄鐜伴兘琛ㄦ槑锛屽鏋滄墍鏈?LED 閮界唲鐏紝鍒欒〃绀洪┍鍔ㄦ湭鍔犺浇銆?

## 6. History


v2.06 (20000511) (In-Kernel version)
    New features:

 - 64 bit support
 - new pci dma interface
 - in kernel 2.3.99

v2.05 (20000217) (In-Kernel version)
    New features:

 - Changes for 2.3.45 kernel

v2.04 (20000207) (Standalone version)
    New features:

 - Added rx/tx byte counter

v2.03 (20000111) (Standalone version)
    Problems fixed:

 - Fixed printk statements from v2.02

v2.02 (991215) (Standalone version)
    Problems fixed:

 - Removed unnecessary output
 - Fixed path for "printver.sh" in makefile

v2.01 (991122) (In-Kernel version)
    New features:

 - Integration in Linux kernel sources
 - Support for memory mapped I/O.

v2.00 (991112)
    New features:

 - Full source released under GPL

v1.05 (991023)
    Problems fixed:

 - Compilation with kernel version 2.2.13 failed

v1.04 (990427)
    Changes:

 - New SMT module included, changing LED functionality

    Problems fixed:

 - Synchronization on SMP machines was buggy

v1.03 (990325)
    Problems fixed:

 - Interrupt routing on SMP machines could be incorrect

v1.02 (990310)
    New features:

 - Support for kernel versions 2.2.x added
 - Kernel patch instead of private duplicate of kernel functions

v1.01 (980812)
    Problems fixed:

	Connection hangup with telnet
	Slow telnet connection

v1.00 beta 01 (980507)
    New features:

	None.

    Problems fixed:

	None.

    Known limitations:

 - tar archive instead of standard package format (rpm).
 - FDDI statistic is empty.
 - not tested with 2.1.xx kernels
 - integration in kernel not tested
 - not tested simultaneously with FDDI adapters from other vendors.
 - only X86 processors supported.
 - SBA (Synchronous Bandwidth Allocator) parameters can
	  not be configured.
 - does not work on some COMPAQ machines. See the PCI howto
	  document for details about this problem.
 - data corruption with kernel versions below 2.0.33.
