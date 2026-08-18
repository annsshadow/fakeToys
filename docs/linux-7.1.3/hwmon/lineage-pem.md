## 鍐呮牳椹卞姩 lineage-pem


鏀寔鐨勮澶囷細

  - Lineage Compact Power Line 鐢垫簮鍏ュ彛妯″潡

    Prefix: 'lineage-pem'

    Addresses scanned: -

    Documentation:

	http://www.lineagepower.com/oem/pdf/CPLI2C.pdf

Author: Guenter Roeck <linux@roeck-us.net>


### 鎻忚堪


鏈┍鍔ㄦ敮鎸佸绉?Lineage Compact Power Line DC/DC 涓?AC/DC
杞崲鍣紝渚嬪 CP1800銆丆P2000AC銆丆P2000DC銆丆P2100DC 绛夈€?
Lineage CPL 鐢垫簮鍏ュ彛妯″潡鍚嶄箟涓婂吋瀹?PMBus銆傜劧鑰岋紝澶у鏁?鏍囧噯 PMBus 鍛戒护骞朵笉鍙楁敮鎸併€傚叿浣撹€岃█锛屾墍鏈夌‖浠剁洃鎺?涓庣姸鎬佷笂鎶ュ懡浠ら兘鏄潪鏍囧噯鐨勩€傚洜姝わ紝鏃犳硶浣跨敤鏍囧噯鐨?PMBus 椹卞姩銆?

### 浣跨敤娉ㄦ剰


鏈┍鍔ㄤ笉浼氭帰娴?Lineage CPL 璁惧锛屽洜涓烘病鏈夊彲渚涘畨鍏ㄧ敤浜?璇嗗埆鑺墖鐨勫瘎瀛樺櫒銆備綘蹇呴』鏄惧紡瀹炰緥鍖栬繖浜涜澶囥€?
绀轰緥锛氫互涓嬪懡浠ゅ皢涓哄湴鍧€ 0x40 澶勭殑 Lineage PEM 鍔犺浇椹卞姩

```

	$ modprobe lineage-pem
	$ echo lineage-pem 0x40 > /sys/bus/i2c/devices/i2c-1/new_device

```
鎵€鏈?Lineage CPL 鐢垫簮鍏ュ彛妯″潡閮藉唴缃簡涓€涓?I2C 鎬荤嚎涓婚€夋嫨鍣?锛圥CA9541锛夈€備负纭繚璁惧璁块棶锛屾湰椹卞姩鍙兘浣滀负
pca9541 I2C 涓婚€夋嫨鍣ㄩ┍鍔ㄧ殑瀹㈡埛绔┍鍔ㄤ娇鐢ㄣ€?

### Sysfs 鏉＄洰


鎵€鏈?Lineage CPL 璁惧閮戒細涓婃姤杈撳嚭鐢靛帇涓庤澶囨俯搴︼紝浠ュ強
杈撳嚭鐢靛帇銆佹俯搴︺€佽緭鍏ョ數鍘嬨€佽緭鍏ョ數娴併€佽緭鍏ュ姛鐜囧拰椋庢墖鐘舵€佺殑鍛婅銆?
杈撳叆鐢靛帇銆佽緭鍏ョ數娴併€佽緭鍏ュ姛鐜囧拰椋庢墖杞€熸祴閲忎粎鍦ㄦ柊娆捐澶囦笂
鍙楁敮鎸併€傞┍鍔ㄤ細妫€娴嬭繖浜涘睘鎬ф槸鍚﹀彈鏀寔锛屽苟浠呭湪鍙楁敮鎸佹椂
鍒涘缓鐩稿簲鐨?sysfs 鏉＄洰銆?
======================= ===============================
in1_input		杈撳嚭鐢靛帇锛坢V锛?in1_min_alarm		杈撳嚭娆犲帇鍛婅
in1_max_alarm		杈撳嚭杩囧帇鍛婅
in1_crit		杈撳嚭鐢靛帇涓ラ噸鍛婅

in2_input		杈撳叆鐢靛帇锛坢V锛屽彲閫夛級
in2_alarm		杈撳叆鐢靛帇鍛婅

curr1_input		杈撳叆鐢垫祦锛坢A锛屽彲閫夛級
curr1_alarm		杈撳叆杩囨祦鍛婅

power1_input		杈撳叆鍔熺巼锛坲W锛屽彲閫夛級
power1_alarm		杈撳叆鍔熺巼鍛婅

fan1_input		椋庢墖 1 杞€燂紙rpm锛屽彲閫夛級
fan2_input		椋庢墖 2 杞€燂紙rpm锛屽彲閫夛級
fan3_input		椋庢墖 3 杞€燂紙rpm锛屽彲閫夛級

temp1_input
temp1_max
temp1_crit
temp1_alarm
temp1_crit_alarm
temp1_fault
======================= ===============================
