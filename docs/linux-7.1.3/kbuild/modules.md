## Building External Modules

鏈枃妗ｄ粙缁嶅浣曟瀯寤轰竴涓爲澶栵紙out-of-tree锛夊唴鏍告ā鍧椼€?
## 绠€浠?
"kbuild" 鏄?Linux 鍐呮牳浣跨敤鐨勬瀯寤虹郴缁熴€傛ā鍧楀繀椤讳娇鐢?kbuild锛屾墠鑳戒笌鏋勫缓鍩虹璁炬柦鐨勫彉鍖栦繚鎸佸吋瀹癸紝骞惰幏寰椾紶缁欑紪璇戝櫒鐨勬纭爣蹇椼€傛棤璁烘槸鏍戝唴锛坕n-tree锛夎繕鏄爲澶栵紙out-of-tree锛夌殑妯″潡锛屾瀯寤哄姛鑳介兘宸叉彁渚涖€備袱鑰呯殑鏋勫缓鏂规硶鐩镐技锛岃€屼笖鎵€鏈夋ā鍧楁渶鍒濋兘鏄湪鏍戝寮€鍙戝拰鏋勫缓鐨勩€?
鏈枃妗ｉ潰鍚戦偅浜涙湁鍏磋叮鏋勫缓鏍戝锛堟垨绉?"external"锛屽閮級妯″潡鐨勫紑鍙戣€呫€傚閮ㄦā鍧楃殑浣滆€呭簲褰撴彁渚涗竴涓?makefile锛屾妸澶ч儴鍒嗗鏉傛€ч殣钘忚捣鏉ワ紝杩欐牱鍙渶杈撳叆 "make" 鍗冲彲鏋勫缓妯″潡銆傝繖寰堝鏄撳仛鍒帮紝瀹屾暣鐨勭ず渚嬪皢鍦?`Creating a Kbuild File for an External Module`_ 涓€鑺備腑缁欏嚭銆?
## 濡備綍鏋勫缓澶栭儴妯″潡

瑕佹瀯寤哄閮ㄦā鍧楋紝浣犲繀椤诲噯澶囧ソ涓€涓凡缁忛鍏堟瀯寤哄ソ鐨勫唴鏍革紝鍏朵腑鍖呭惈鏋勫缓鏃舵墍鐢ㄥ埌鐨勯厤缃笌澶存枃浠躲€傛澶栵紝璇ュ唴鏍稿繀椤绘槸鍦ㄥ惎鐢ㄦā鍧楃殑鎯呭喌涓嬫瀯寤虹殑銆傚鏋滀綘浣跨敤鐨勬槸鍙戣鐗堝唴鏍革紝浣犵殑鍙戣鐗堥€氬父浼氭彁渚涗笌浣犳鍦ㄨ繍琛岀殑鍐呮牳鐩稿搴旂殑杞欢鍖呫€?
鍙︿竴绉嶅仛娉曟槸浣跨敤 "make" 鐨?"modules_prepare" 鐩爣銆傝繖浼氱‘淇濆唴鏍稿寘鍚墍闇€鐨勪俊鎭€傝鐩爣瀛樺湪鐨勫敮涓€鐩殑锛屽氨鏄负鏋勫缓澶栭儴妯″潡鍑嗗鍐呮牳婧愮爜鏍戞彁渚涗竴绉嶇畝鍗曟柟寮忋€?
娉ㄦ剰锛?modules_prepare" 鍗充娇璁剧疆浜?CONFIG_MODVERSIONS锛屼篃涓嶄細鏋勫缓 Module.symvers锛涘洜姝わ紝瑕佷娇妯″潡鐗堟湰鎺у埗鐢熸晥锛岄渶瑕佹墽琛屼竴娆″畬鏁寸殑鍐呮牳鏋勫缓銆?
### 鍛戒护璇硶

```

		$ make -C <path_to_kernel_dir> M=$PWD

	The kbuild system knows that an external module is being built
	due to the "M=<dir>" option given in the command.

	To build against the running kernel use::

		$ make -C /lib/modules/`uname -r`/build M=$PWD

	Then to install the module(s) just built, add the target
	"modules_install" to the command::

		$ make -C /lib/modules/`uname -r`/build M=$PWD modules_install

	Starting from Linux 6.13, you can use the -f option instead of -C. This
	will avoid unnecessary change of the working directory. The external
	module will be output to the directory where you invoke make.

		$ make -f /lib/modules/`uname -r`/build/Makefile M=$PWD

```

### 閫夐」

	锛?KDIR 鎸囧唴鏍告簮鐮佺洰褰曠殑璺緞锛岃嫢鍐呮牳鏄湪鍗曠嫭鐨勬瀯寤虹洰褰曚腑鏋勫缓鐨勶紝鍒欐寚鍐呮牳杈撳嚭鐩綍鐨勮矾寰勩€傦級

	濡傛灉浣犳兂鍦ㄥ崟鐙殑鐩綍涓瀯寤烘ā鍧楋紝鍙互閫夋嫨浼犲叆 MO= 閫夐」銆?
	make -C $KDIR M=$PWD [MO=$BUILD_DIR]

	-C $KDIR
		鍖呭惈鐢ㄤ簬鏋勫缓澶栭儴妯″潡鐨勫唴鏍稿強鐩稿叧鏋勫缓浜х墿鐨勭洰褰曘€?		"make" 鍦ㄦ墽琛屾椂瀹為檯涓婁細鍒囨崲鍒版寚瀹氱殑鐩綍锛屽苟鍦ㄧ粨鏉熷悗鍒囧洖銆?
	M=$PWD
		鍛婄煡 kbuild 姝ｅ湪鏋勫缓涓€涓閮ㄦā鍧椼€?		浼犵粰 "M" 鐨勫€兼槸澶栭儴妯″潡锛坘build 鏂囦欢锛夋墍鍦ㄧ洰褰曠殑缁濆璺緞銆?
	MO=$BUILD_DIR
		涓哄閮ㄦā鍧楁寚瀹氫竴涓嫭绔嬬殑杈撳嚭鐩綍銆?
### 鐩爣

	鏋勫缓澶栭儴妯″潡鏃讹紝鍙湁 "make" 鐩爣鐨勪竴涓瓙闆嗗彲鐢ㄣ€?
	make -C $KDIR M=$PWD [target]

	榛樿浼氭瀯寤轰綅浜庡綋鍓嶇洰褰曚腑鐨勬ā鍧楋紝鍥犳鏃犻渶鎸囧畾鐩爣銆傛墍鏈夎緭鍑烘枃浠朵篃浼氬湪璇ョ洰褰曚腑鐢熸垚銆備笉浼氬皾璇曟洿鏂板唴鏍告簮鐮侊紝骞朵笖鍓嶆彁鏄唴鏍稿凡缁忔垚鍔熸墽琛岃繃涓€娆?"make"銆?
	modules
		澶栭儴妯″潡鐨勯粯璁ょ洰鏍囥€傚叾鍔熻兘鍜屾湭鎸囧畾鐩爣鏃剁浉鍚屻€傚弬瑙佷笂闈㈢殑璇存槑銆?
	modules_install
		瀹夎澶栭儴妯″潡銆傞粯璁や綅缃槸
		/lib/modules/<kernel_release>/updates/锛屼絾鍙互閫氳繃 INSTALL_MOD_PATH 娣诲姞鍓嶇紑锛堝湪 `Module Installation`_ 涓€鑺備腑璁ㄨ锛夈€?
	clean
		浠呭垹闄ゆā鍧楃洰褰曚腑鐢熸垚鐨勬墍鏈夋枃浠躲€?
	help
		鍒楀嚭澶栭儴妯″潡鍙敤鐨勭洰鏍囥€?
### 鏋勫缓鍗曠嫭鐨勬枃浠?
	鍙互鏋勫缓灞炰簬鏌愪釜妯″潡鐨勫崟涓枃浠躲€?	杩欏鍐呮牳銆佹ā鍧椼€佺敋鑷冲閮ㄦā鍧楀悓鏍烽€傜敤銆?
```

		make -C $KDIR M=$PWD bar.lst
		make -C $KDIR M=$PWD baz.o
		make -C $KDIR M=$PWD foo.ko
		make -C $KDIR M=$PWD ./

```

## 涓哄閮ㄦā鍧楀垱寤?Kbuild 鏂囦欢

鍦ㄤ笂涓€鑺備腑锛屾垜浠湅鍒颁簡涓烘鍦ㄨ繍琛岀殑鍐呮牳鏋勫缓妯″潡鐨勫懡浠ゃ€備笉杩囨ā鍧楀疄闄呬笂骞舵湭琚瀯寤猴紝鍥犱负杩橀渶瑕佷竴涓瀯寤烘枃浠躲€傝鏂囦欢涓皢鍖呭惈琚瀯寤烘ā鍧楃殑鍚嶇О锛屼互鍙婃墍闇€鐨勬簮鏂囦欢鍒楄〃

```

	obj-m := <module_name>.o

```

kbuild 绯荤粺浼氫粠 <module_name>.c 鏋勫缓鍑?<module_name>.o锛屽苟鍦ㄩ摼鎺ヤ箣鍚庣敓鎴愬唴鏍告ā鍧?<module_name>.ko銆備笂闈㈣繖琛屽彲浠ユ斁鍦?"Kbuild" 鏂囦欢鎴?"Makefile" 涓€傚綋妯″潡鐢卞涓簮鏂囦欢鏋勫缓鏃讹紝杩橀渶瑕侀澶栫殑涓€琛?
```

	<module_name>-y := <src1>.o <src2>.o ...

```

娉ㄦ剰锛氭弿杩?kbuild 鎵€鐢ㄨ娉曠殑杩涗竴姝ユ枃妗ｄ綅浜?Documentation/kbuild/makefiles.rst銆?
涓嬮潰鐨勭ず渚嬫紨绀哄浣曚负浠ヤ笅鏂囦欢鍒涘缓鏋勫缓鏂囦欢

```

	8123_if.c
	8123_if.h
	8123_pci.c

```

### 鍏变韩 Makefile

	澶栭儴妯″潡濮嬬粓鍖呭惈涓€涓寘瑁呯敤鐨?makefile锛屽畠鏀寔涓嶅甫鍙傛暟浣跨敤 "make" 鏉ユ瀯寤烘ā鍧椼€?	杩欎釜鐩爣骞堕潪鐢?kbuild 浣跨敤锛屼粎涓烘柟渚胯€岃銆備篃鍙互鍔犲叆棰濆鐨勫姛鑳斤紙渚嬪娴嬭瘯鐩爣锛夛紝浣嗙敱浜庡彲鑳藉瓨鍦ㄥ悕绉板啿绐侊紝搴斿綋灏嗗叾浠?kbuild 涓繃婊ゆ帀銆?
```

		--> filename: Makefile
		ifneq ($(KERNELRELEASE),)
		# kbuild part of makefile
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		else
		# normal makefile
		KDIR ?= /lib/modules/`uname -r`/build

		default:
			$(MAKE) -C $(KDIR) M=$$PWD

		endif

	The check for KERNELRELEASE is used to separate the two parts
	of the makefile. In the example, kbuild will only see the two
	assignments, whereas "make" will see everything except these
	two assignments. This is due to two passes made on the file:
	the first pass is by the "make" instance run on the command
	line; the second pass is by the kbuild system, which is
	initiated by the parameterized "make" in the default target.

```

### 鐙珛鐨?Kbuild 鏂囦欢涓?Makefile

	Kbuild 浼氶鍏堟煡鎵惧悕涓?"Kbuild" 鐨勬枃浠讹紝鑻ユ湭鎵惧埌锛屽垯鍐嶅幓鏌ユ壘 "Makefile"銆傚埄鐢?"Kbuild" 鏂囦欢锛屾垜浠彲浠ュ皢绀轰緥 1 涓殑 "Makefile" 鎷嗗垎涓轰袱涓枃浠讹細

```

		--> filename: Kbuild
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		--> filename: Makefile
		KDIR ?= /lib/modules/`uname -r`/build

		default:
			$(MAKE) -C $(KDIR) M=$$PWD

```

	绀轰緥 2 涓殑鎷嗗垎鐢变簬姣忎釜鏂囦欢閮藉緢绠€鍗曡€屾樉寰楀浣欙紱涓嶈繃锛屾湁浜涘閮ㄦā鍧椾娇鐢ㄧ殑 makefile 闀胯揪鏁扮櫨琛岋紝鍦ㄨ繖绉嶆儏鍐典笅锛屽皢 kbuild 閮ㄥ垎涓庡叾浣欓儴鍒嗗垎绂荤‘瀹炲ぇ鏈夎（鐩娿€?
	Linux 6.13 鍙婃洿楂樼増鏈敮鎸佸彟涓€绉嶆柟寮忋€傚閮ㄦā鍧楃殑 Makefile 鍙互鐩存帴鍖呭惈鍐呮牳 Makefile锛岃€屼笉鏄皟鐢ㄥ瓙 Make銆?
	Example 3::

```

		--> filename: Kbuild
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		--> filename: Makefile
		KDIR ?= /lib/modules/$(shell uname -r)/build
		export KBUILD_EXTMOD := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))
		include $(KDIR)/Makefile


```

### 鏋勫缓澶氫釜妯″潡

	kbuild 鏀寔鐢ㄥ崟涓瀯寤烘枃浠舵瀯寤哄涓ā鍧椼€備緥濡傦紝濡傛灉浣犳兂鏋勫缓涓や釜妯″潡 foo.ko

```

		obj-m := foo.o bar.o
		foo-y := <foo_srcs>
		bar-y := <bar_srcs>

	It is that simple!


```

## 鍖呭惈鏂囦欢

鍦ㄥ唴鏍镐腑锛屽ご鏂囦欢鎸夌収浠ヤ笅瑙勫垯鏀剧疆鍦ㄦ爣鍑嗕綅缃細

 - 濡傛灉澶存枃浠跺彧鎻忚堪鏌愪釜妯″潡鐨勫唴閮ㄦ帴鍙ｏ紝鍒欒鏂囦欢鏀惧湪涓庢簮鏂囦欢鐩稿悓鐨勭洰褰曚腑銆? - 濡傛灉澶存枃浠舵弿杩板唴鏍稿叾浠栭儴鍒嗭紙浣嶄簬涓嶅悓鐩綍涓級鎵€浣跨敤鐨勬帴鍙ｏ紝鍒欒鏂囦欢鏀惧湪 include/linux/ 涓€?
	  NOTE:
	      璇ヨ鍒欐湁涓や釜鏄捐憲鐨勪緥澶栵細杈冨ぇ鐨勫瓙绯荤粺鍦?include/ 涓嬫湁鑷繁鐙珛鐨勭洰褰曪紝渚嬪 include/scsi锛涜€岀壒瀹氫簬鏋舵瀯鐨勫ご鏂囦欢浣嶄簬 arch/$(SRCARCH)/include/ 涓嬨€?
### Kernel Includes

	瑕佸寘鍚綅浜?include/linux/ 涓嬬殑澶存枃浠讹紝鍙渶

```

		#include <linux/module.h>

	kbuild will add options to the compiler so the relevant directories
	are searched.

```

### Single Subdirectory

	澶栭儴妯″潡鍊惧悜浜庢妸澶存枃浠舵斁鍦ㄥ叾婧愮爜鎵€鍦ㄤ綅缃笅涓€涓嫭绔嬬殑
	include/ 鐩綍涓紝灏界杩欏苟闈為€氬父鐨勫唴鏍搁鏍笺€傝鍛婄煡 kbuild 璇ョ洰褰曪紝鍙娇鐢?ccflags-y 鎴?CFLAGS_<filename>.o銆?
	浣跨敤绗?3 鑺傜殑绀轰緥锛屽鏋滄垜浠皢 8123_if.h 绉诲姩鍒颁竴涓悕涓?include 鐨勫瓙鐩綍锛屽垯寰楀埌鐨?kbuild 鏂囦欢灏嗘槸

```

		--> filename: Kbuild
		obj-m := 8123.o

		ccflags-y := -I $(src)/include
		8123-y := 8123_if.o 8123_pci.o

```

### Several Subdirectories

	kbuild 鍙互澶勭悊鍒嗘暎鍦ㄥ涓洰褰曚腑鐨勬枃浠躲€?
```

		.
		|__ src
		|   |__ complex_main.c
		|   |__ hal
		|	|__ hardwareif.c
		|	|__ include
		|	    |__ hardwareif.h
		|__ include
			|__ complex.h

	To build the module complex.ko, we then need the following
	kbuild file::

		--> filename: Kbuild
		obj-m := complex.o
		complex-y := src/complex_main.o
		complex-y += src/hal/hardwareif.o

		ccflags-y := -I$(src)/include
		ccflags-y += -I$(src)/src/hal/include

	As you can see, kbuild knows how to handle object files located
	in other directories. The trick is to specify the directory
	relative to the kbuild file's location. That being said, this
	is NOT recommended practice.

	For the header files, kbuild must be explicitly told where to
	look. When kbuild executes, the current directory is always the
	root of the kernel tree (the argument to "-C") and therefore an
	absolute path is needed. $(src) provides the absolute path by
	pointing to the directory where the currently executing kbuild
	file is located.


```

## 妯″潡瀹夎

鍖呭惈鍦ㄥ唴鏍镐腑鐨勬ā鍧椾細琚畨瑁呭埌浠ヤ笅鐩綍锛?
	/lib/modules/$(KERNELRELEASE)/kernel/

鑰屽閮ㄦā鍧椾細琚畨瑁呭埌锛?
	/lib/modules/$(KERNELRELEASE)/updates/

### INSTALL_MOD_PATH

	涓婇潰鏄粯璁ょ洰褰曪紝浣嗗拰閫氬父涓€鏍凤紝涓€瀹氱▼搴︾殑瀹氬埗鏄彲鑳界殑銆傚彲浠ユ坊鍔犱竴涓墠缂€鍒?
```

		$ make INSTALL_MOD_PATH=/frodo modules_install
		=> Install dir: /frodo/lib/modules/$(KERNELRELEASE)/kernel/

	INSTALL_MOD_PATH may be set as an ordinary shell variable or,
	as shown above, can be specified on the command line when
	calling "make." This has effect when installing both in-tree
	and out-of-tree modules.

```

### INSTALL_MOD_DIR

	榛樿鎯呭喌涓嬶紝澶栭儴妯″潡琚畨瑁呭埌 /lib/modules/$(KERNELRELEASE)/updates/ 涓嬬殑鏌愪釜鐩綍涓紝浣嗕綘鍙兘甯屾湜灏嗙壒瀹氬姛鑳界殑妯″潡鏀惧湪涓€涓嫭绔嬬殑鐩綍涓€備负姝わ紝鍙娇鐢?INSTALL_MOD_DIR 鏉ユ寚瀹氫竴涓?
```

		$ make INSTALL_MOD_DIR=gandalf -C $KDIR \
		       M=$PWD modules_install
		=> Install dir: /lib/modules/$(KERNELRELEASE)/gandalf/


```

## 妯″潡鐗堟湰鎺у埗

妯″潡鐗堟湰鎺у埗鐢?CONFIG_MODVERSIONS 鏍囪鍚敤锛岀敤浣滀竴绉嶇畝鍗曠殑 ABI 涓€鑷存€ф鏌ャ€備細涓哄鍑虹殑绗﹀彿鐨勫畬鏁村師鍨嬪垱寤轰竴涓?CRC 鍊笺€傚綋妯″潡琚姞杞?浣跨敤鏃讹紝鍐呮牳涓寘鍚殑 CRC 鍊间細涓庢ā鍧椾腑鐨勭被浼煎€艰繘琛屾瘮杈冿紱鑻ヤ笉鐩哥瓑锛屽唴鏍稿皢鎷掔粷鍔犺浇璇ユā鍧椼€?
Module.symvers 鍖呭惈涓€涓唴鏍告瀯寤轰腑鎵€鏈夊凡瀵煎嚭绗﹀彿鐨勫垪琛ㄣ€?
### 鏉ヨ嚜鍐呮牳鐨勭鍙凤紙vmlinux + 妯″潡锛?
	鍦ㄥ唴鏍告瀯寤烘湡闂达紝浼氱敓鎴愪竴涓悕涓?Module.symvers 鐨勬枃浠躲€侻odule.symvers 鍖呭惈鍐呮牳涓庡凡缂栬瘧妯″潡涓墍鏈夊鍑虹殑绗﹀彿銆傚浜庢瘡涓鍙凤紝鍏跺搴旂殑 CRC 鍊间篃浼氳瀛樺偍銆?
```

		<CRC>       <Symbol>         <Module>                         <Export Type>     <Namespace>

		0xe1cc2a05  usb_stor_suspend drivers/usb/storage/usb-storage  EXPORT_SYMBOL_GPL USB_STORAGE

	The fields are separated by tabs and values may be empty (e.g.
	if no namespace is defined for an exported symbol).

	For a kernel build without CONFIG_MODVERSIONS enabled, the CRC
	would read 0x00000000.

	Module.symvers serves two purposes:

	1) It lists all exported symbols from vmlinux and all modules.
	2) It lists the CRC if CONFIG_MODVERSIONS is enabled.

```

### 鐗堟湰淇℃伅鏍煎紡

	瀵煎嚭鐨勭鍙峰皢鍏朵俊鎭瓨鍌ㄥ湪 __ksymtab 涓?__kflagstab 娈典腑銆傜鍙峰悕涓庡懡鍚嶇┖闂村瓨鍌ㄥ湪 __ksymtab_strings 娈典腑锛屼娇鐢ㄧ殑鏍煎紡绫讳技浜?ELF 鎵€鐢ㄧ殑瀛楃涓茶〃銆傝嫢鍚敤浜?CONFIG_MODVERSIONS锛屼笌瀵煎嚭绗﹀彿瀵瑰簲鐨?CRC 浼氳娣诲姞鍒?__kcrctab 娈典腑銆?
	鑻ュ惎鐢ㄤ簡 CONFIG_BASIC_MODVERSIONS锛圕ONFIG_MODVERSIONS 榛樿寮€鍚椤癸級锛屽鍏ョ鍙风殑绗﹀彿鍚嶄笌 CRC 浼氬瓨鍌ㄥ湪瀵煎叆妯″潡鐨?__versions 娈典腑銆傝妯″紡浠呮敮鎸侀暱搴︿笉瓒呰繃 64 瀛楄妭鐨勭鍙枫€?
	鑻ュ惎鐢ㄤ簡 CONFIG_EXTENDED_MODVERSIONS锛堥渶鍚屾椂鍚敤 CONFIG_MODVERSIONS 涓?CONFIG_RUST锛夛紝瀵煎叆绗﹀彿鐨勭鍙峰悕浼氫互涓€绯诲垪鎷兼帴璧锋潵鐨勩€佷互绌哄瓧绗︾粨灏剧殑瀛楃涓插舰寮忚褰曞湪 __version_ext_names 娈典腑銆傝繖浜涚鍙风殑 CRC 浼氳褰曞湪 __version_ext_crcs 娈典腑銆?
### 绗﹀彿涓庡閮ㄦā鍧?
	鏋勫缓澶栭儴妯″潡鏃讹紝鏋勫缓绯荤粺闇€瑕佽闂唴鏍镐腑鐨勭鍙凤紝浠ユ鏌ユ墍鏈夊閮ㄧ鍙锋槸鍚﹂兘宸插畾涔夈€傝繖涓€姝ュ湪 MODPOST 闃舵瀹屾垚銆俶odpost 閫氳繃璇诲彇鍐呮牳婧愮爜鏍戜腑鐨?Module.symvers 鏉ヨ幏鍙栫鍙枫€傚湪 MODPOST 闃舵锛屼細鍐欏叆涓€涓柊鐨?Module.symvers 鏂囦欢锛屽叾涓寘鍚澶栭儴妯″潡瀵煎嚭鐨勬墍鏈夌鍙枫€?
### 鏉ヨ嚜鍙︿竴涓閮ㄦā鍧楃殑绗﹀彿

	鏈夋椂锛屼竴涓閮ㄦā鍧椾細浣跨敤鍙︿竴涓閮ㄦā鍧楀鍑虹殑绗﹀彿銆侹build 闇€瑕佸畬鍏ㄦ帉鎻℃墍鏈夌鍙凤紝浠ラ伩鍏嶅彂鍑哄叧浜庢湭瀹氫箟绗﹀彿鐨勮鍛娿€傞拡瀵硅繖绉嶆儏鍐垫湁涓ょ瑙ｅ喅鏂规硶銆?
	娉ㄦ剰锛氭帹鑽愪娇鐢ㄥ甫椤跺眰 kbuild 鏂囦欢鐨勬柟娉曪紝浣嗗湪鏌愪簺鎯呭喌涓嬪彲鑳藉苟涓嶅疄鐢ㄣ€?
	Use a top-level kbuild file
		濡傛灉浣犳湁涓や釜妯″潡 foo.ko 涓?bar.ko锛屽叾涓?foo.ko 闇€瑕佹潵鑷?bar.ko 鐨勭鍙凤紝浣犲彲浠ヤ娇鐢ㄤ竴涓叡鐢ㄧ殑椤跺眰 kbuild 鏂囦欢锛屼娇涓や釜妯″潡鍦ㄥ悓涓€涓瀯寤轰腑缂栬瘧銆傚弬鑰冧互涓?
```

			./foo/ <= contains foo.ko
			./bar/ <= contains bar.ko

		The top-level kbuild file would then look like::

			#./Kbuild (or ./Makefile):
				obj-m := foo/ bar/

		And executing::

			$ make -C $KDIR M=$PWD

		will then do the expected and compile both modules with
		full knowledge of symbols from either module.

	Use "make" variable KBUILD_EXTRA_SYMBOLS
		If it is impractical to add a top-level kbuild file,
		you can assign a space separated list
		of files to KBUILD_EXTRA_SYMBOLS in your build file.
		These files will be loaded by modpost during the
		initialization of its symbol tables.


```

## 鎶€宸т笌璇€绐?
### 娴嬭瘯 CONFIG_FOO_BAR

	妯″潡甯稿父闇€瑕佹鏌ユ煇浜?`CONFIG_` 閫夐」锛屼互鍐冲畾妯″潡鏄惁鍖呭惈鏌愰」鐗瑰畾鍔熻兘銆傚湪 kbuild 涓紝杩欏彲浠ラ€氳繃寮曠敤 `CONFIG_` 鍙橀噺鏉ュ疄鐜?
```

		#fs/ext2/Makefile
		obj-$(CONFIG_EXT2_FS) += ext2.o

		ext2-y := balloc.o bitmap.o dir.o
		ext2-$(CONFIG_EXT2_FS_XATTR) += xattr.o

```
