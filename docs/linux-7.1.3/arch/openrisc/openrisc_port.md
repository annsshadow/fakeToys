## OpenRISC Linux

杩欐槸灏?Linux 绉绘鍒?OpenRISC 绯诲垪寰鐞嗗櫒鐨勬垚鏋滐紱鍏蜂綋鑰岃█锛屾渶鍒濈殑鐩爣鏋舵瀯鏄?32 浣嶇殑 OpenRISC 1000 瀹舵棌锛坥r1k锛夈€?
鏈夊叧 OpenRISC 澶勭悊鍣ㄥ拰鎸佺画寮€鍙戠殑淇℃伅锛?
	=======		==============================
	website		https://openrisc.io
	email		linux-openrisc@vger.kernel.org
	=======		==============================

---------------------------------------------------------------------

## OpenRISC 宸ュ叿閾句笌 Linux 鐨勬瀯寤鸿鏄?
涓轰簡鏋勫缓骞惰繍琛?OpenRISC 涓婄殑 Linux锛屼綘鑷冲皯闇€瑕佷竴涓熀鏈殑宸ュ叿閾撅紝鍙兘杩橀渶瑕?鏋舵瀯妯℃嫙鍣ㄣ€傛澶勬杩颁簡璁╄繖浜涚粍浠跺氨浣嶆墍闇€鐨勬楠ゃ€?
1) 宸ュ叿閾?
宸ュ叿閾句簩杩涘埗鏂囦欢鍙互浠?openrisc.io 鎴栨垜浠殑 github releases 椤甸潰鑾峰彇銆傛瀯寤轰笉鍚?宸ュ叿閾剧殑璇存槑鍙互鍦?openrisc.io 鎴?Stafford 鐨勫伐鍏烽摼鏋勫缓涓庡彂甯冭剼鏈腑鎵惧埌銆?
	==========	==========================================================
	binaries	https://github.com/stffrdhrn/or1k-toolchain-build/releases
	toolchains	https://openrisc.io/software
	building	https://github.com/stffrdhrn/or1k-toolchain-build
	==========	==========================================================

2) 鏋勫缓

```

	make ARCH=openrisc CROSS_COMPILE="or1k-linux-" defconfig
	make ARCH=openrisc CROSS_COMPILE="or1k-linux-"

```
```

	make ARCH=openrisc CROSS_COMPILE="or1k-linux-" CONFIG_INITRAMFS_SOURCE="path/to/rootfs path/to/devnodes"

```
鍏充簬姝ゅ鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/filesystems/ramfs-rootfs-initramfs.rst銆?
3) 鍦?FPGA 涓婅繍琛岋紙鍙€夛級

OpenRISC 绀惧尯閫氬父浣跨敤 FuseSoC 鏉ョ鐞嗗皢 SoC 鏋勫缓骞剁儳褰曞埌 FPGA 涓€備笅闈㈡槸灏?OpenRISC SoC 鐑у綍鍒?De0 Nano 寮€鍙戞澘鐨勭ず渚嬨€傚湪鏋勫缓杩囩▼涓紝FPGA RTL 浠ｇ爜浼氫粠
FuseSoC IP 鏍镐粨搴撲笅杞斤紝骞朵娇鐢?FPGA 鍘傚晢宸ュ叿鏋勫缓銆備簩杩涘埗鏂囦欢閫氳繃 openocd 鍔犺浇鍒?鏉垮崱涓娿€?
```

	git clone https://github.com/olofk/fusesoc
	cd fusesoc
	sudo pip install -e .

	fusesoc init
	fusesoc build de0_nano
	fusesoc pgm de0_nano

	openocd -f interface/altera-usb-blaster.cfg \
		-f board/or1k_generic.cfg

	telnet localhost 4444
	> init
	> halt; load_image vmlinux ; reset

```
4) 鍦ㄦā鎷熷櫒涓婅繍琛岋紙鍙€夛級

QEMU 鏄竴涓鐞嗗櫒妯℃嫙鍣紝鎴戜滑鎺ㄨ崘鐢ㄥ畠鏉ユā鎷?OpenRISC 骞冲彴銆傝鎸夌収 QEMU 缃戠珯涓婄殑
OpenRISC 璇存槑鏉ュ湪 QEMU 涓婅繍琛?Linux銆備綘鍙互鑷繁鏋勫缓 QEMU锛屼絾浣犵殑 Linux 鍙戣鐗堝緢鍙兘
鎻愪緵浜嗘敮鎸?OpenRISC 鐨勪簩杩涘埗鍖呫€?
	=============	======================================================
	qemu openrisc	https://wiki.qemu.org/Documentation/Platforms/OpenRISC
	=============	======================================================

---------------------------------------------------------------------

## 鏈

鍦ㄤ唬鐮佷腑锛岀鍙蜂笂浣跨敤浠ヤ笅鈥滅矑瀛愶紙particle锛夆€濇潵灏嗚寖鍥撮檺瀹氫负鎴栧鎴栧皯鐗瑰畾鐨勫鐞嗗櫒
瀹炵幇锛?
========= =======================================
openrisc:  OpenRISC 绯诲垪澶勭悊鍣?or1k:      OpenRISC 1000 瀹舵棌澶勭悊鍣?or1200:    OpenRISC 1200 澶勭悊鍣?========= =======================================

---------------------------------------------------------------------

## 鍘嗗彶

18-11-2003	Matjaz Breskvar (phoenix@bsemi.com)
	灏?linux 鍒濇绉绘鍒?OpenRISC/or32 鏋舵瀯銆?        鎵€鏈夋牳蹇冮儴鍒嗛兘宸插疄鐜帮紝鐪嬭捣鏉ュ彲鐢ㄣ€?
08-12-2003	Matjaz Breskvar (phoenix@bsemi.com)
	瀹屽叏鏀瑰彉浜?TLB miss 鐨勫鐞嗘柟寮忋€?	閲嶅啓浜嗗紓甯稿鐞嗐€?	榛樿 initrd 涓叿澶囧畬鏁村彲鐢ㄧ殑 sash-3.6銆?	涓€涓悇鏂归潰閮芥湁寰堝ぇ鏀硅繘鐨勭増鏈€?
10-04-2004	Matjaz Breskvar (phoenix@bsemi.com)
	澶ч噺鐨?bug 淇銆?	浠ュお缃戞敮鎸侊紝鍙敤鐨?http 鍜?telnet 鏈嶅姟鍣ㄣ€?	杩愯璁稿鏍囧噯 linux 搴旂敤銆?
26-06-2004	Matjaz Breskvar (phoenix@bsemi.com)
	绉绘鍒?2.6.x銆?
30-11-2004	Matjaz Breskvar (phoenix@bsemi.com)
	澶ч噺 bug 淇涓庡寮恒€?	娣诲姞浜?opencores 甯х紦鍐查┍鍔ㄣ€?
09-10-2010    Jonas Bonn (jonas@southpole.se)
	閲嶅ぇ閲嶅啓锛屼互涓庝笂娓?Linux 2.6.36 鐪嬮綈
