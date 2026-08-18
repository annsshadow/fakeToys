## Linux 鍐呮牳 Makefiles


鏈枃妗ｆ弿杩颁簡 Linux 鍐呮牳鐨?Makefiles銆?

## Overview


```

	Makefile                    the top Makefile.
	.config                     the kernel configuration file.
	arch/$(SRCARCH)/Makefile    the arch Makefile.
	scripts/Makefile.*          common rules etc. for all kbuild Makefiles.
	kbuild Makefiles            exist in every subdirectory

```
椤跺眰 Makefile 璇诲彇鏉ヨ嚜鍐呮牳閰嶇疆杩囩▼鐢熸垚鐨?.config 鏂囦欢銆?

椤跺眰 Makefile 璐熻矗鏋勫缓涓や釜涓昏浜х墿锛歷mlinux锛堝父椹诲唴鏍告槧鍍忥級涓?modules锛堜换鎰忔ā鍧楁枃浠讹級銆傚畠閫氳繃閫掑綊杩涘叆鍐呮牳婧愮爜鏍戜腑鐨勫瓙鐩綍鏉ユ瀯寤鸿繖浜涚洰鏍囥€?

琚闂殑瀛愮洰褰曞垪琛ㄥ彇鍐充簬鍐呮牳閰嶇疆銆傞《灞?Makefile 浠ユ枃鏈柟寮忓寘鍚竴涓悕涓?arch/$(SRCARCH)/Makefile 鐨勬灦鏋?Makefile銆傝鏋舵瀯 Makefile 鍚戦《灞?Makefile 鎻愪緵鏋舵瀯鐩稿叧鐨勪俊鎭€?

姣忎釜瀛愮洰褰曢兘鏈変竴涓?kbuild Makefile锛岀敤浜庢墽琛屼粠涓婂眰浼犻€掍笅鏉ョ殑鍛戒护銆俴build Makefile 浣跨敤鏉ヨ嚜 .config 鏂囦欢鐨勪俊鎭紝鏋勯€?kbuild 鐢ㄤ簬鏋勫缓浠绘剰鍐呯疆鎴栨ā鍧楀寲鐩爣鎵€闇€鐨勫悇绉嶆枃浠跺垪琛ㄣ€?

scripts/Makefile.* 鍖呭惈浜嗗熀浜?kbuild makefiles 鏋勫缓鍐呮牳鎵€鐢ㄧ殑鍏ㄩ儴瀹氫箟/瑙勫垯绛夈€?

## Who does what


浜轰滑涓庡唴鏍?Makefiles 涔嬮棿瀛樺湪鍥涚涓嶅悓鐨勫叧绯汇€?

**Users锛堢敤鎴凤級** 鏄瀯寤哄唴鏍哥殑浜恒€傝繖浜涗汉杈撳叆璇稿 `make menuconfig` 鎴?`make` 杩欐牱鐨勫懡浠ゃ€備粬浠€氬父鏃笉闃呰涔熶笉缂栬緫浠讳綍鍐呮牳 Makefile锛堟垨浠讳綍鍏朵粬婧愭枃浠讹級銆?

**Normal developers锛堟櫘閫氬紑鍙戣€咃級** 鏄粠浜嬭澶囬┍鍔ㄣ€佹枃浠剁郴缁熷拰缃戠粶鍗忚绛夌壒鎬у紑鍙戠殑浜恒€傝繖浜涗汉闇€瑕佺淮鎶ゅ叾鎵€宸ヤ綔瀛愮郴缁熺殑 kbuild Makefile銆備负浜嗛珮鏁堝湴瀹屾垚杩欓」宸ヤ綔锛屼粬浠渶瑕佷竴浜涘叧浜庡唴鏍?Makefiles 鐨勬暣浣撶煡璇嗭紝浠ュ強瀵?kbuild 鍏叡鎺ュ彛鐨勮缁嗕簡瑙ｃ€?

**Arch developers锛堟灦鏋勫紑鍙戣€咃級** 鏄粠浜嬫暣涓灦鏋勶紙濡?sparc 鎴?x86锛夊紑鍙戠殑浜恒€傛灦鏋勫紑鍙戣€呴渶瑕佷簡瑙ｆ灦鏋?Makefile 浠ュ強 kbuild Makefile銆?

**Kbuild developers锛坘build 寮€鍙戣€咃級** 鏄粠浜嬪唴鏍告瀯寤虹郴缁熸湰韬紑鍙戠殑浜恒€傝繖浜涗汉闇€瑕佷簡瑙ｅ唴鏍?Makefiles 鐨勬柟鏂归潰闈€?

鏈枃妗ｉ潰鍚戞櫘閫氬紑鍙戣€呭拰鏋舵瀯寮€鍙戣€呫€?


## The kbuild files


鍐呮牳涓殑澶у鏁?Makefile 閮芥槸浣跨敤 kbuild 鍩虹璁炬柦鐨?kbuild Makefile銆傛湰绔犱粙缁?kbuild makefiles 涓墍浣跨敤鐨勮娉曘€?

kbuild 鏂囦欢鐨勫亸濂藉悕绉版槸 `Makefile`锛屼絾涔熷彲浠ヤ娇鐢?`Kbuild`锛涜嫢 `Makefile` 涓?`Kbuild` 鏂囦欢鍚屾椂瀛樺湪锛屽垯浼氫娇鐢?`Kbuild` 鏂囦欢銆?

绗?`Goal definitions`_ 鑺傛槸涓€涓揩閫熷叆闂紱鍚庣画绔犺妭閫氳繃鐪熷疄绀轰緥鎻愪緵浜嗘洿澶氱粏鑺傘€?

### Goal definitions


鐩爣瀹氫箟鏄?kbuild Makefile 鐨勪富瑕侀儴鍒嗭紙鏍稿績锛夈€傝繖浜涜瀹氫箟浜嗚鏋勫缓鐨勬枃浠躲€佷换浣曠壒娈婄殑缂栬瘧閫夐」锛屼互鍙婁换浣曢渶瑕侀€掑綊杩涘叆鐨勫瓙鐩綍銆?

鏈€绠€鍗曠殑 kbuild makefile 鍙寘鍚竴琛岋細

```

  obj-y += foo.o

```
杩欏憡璇?kbuild 璇ョ洰褰曚腑瀛樺湪涓€涓悕涓?foo.o 鐨勭洰鏍囨枃浠躲€俧oo.o 灏嗙敱 foo.c 鎴?foo.S 鏋勫缓銆?

濡傛灉 foo.o 瑕佷綔涓烘ā鍧楁瀯寤猴紝鍒欎娇鐢ㄥ彉閲?obj-m銆傚洜姝ょ粡甯镐娇鐢ㄥ涓嬫ā寮忥細

```

  obj-$(CONFIG_FOO) += foo.o

```
$(CONFIG_FOO) 姹傚€间负 y锛堝唴缃級鎴?m锛堟ā鍧楋級銆傚鏋?CONFIG_FOO 鏃笉鏄?y 涔熶笉鏄?m锛岄偅涔堣鏂囦欢鏃笉浼氳缂栬瘧涔熶笉浼氳閾炬帴銆?

### Built-in object goals - obj-y


kbuild Makefile 鍦?$(obj-y) 鍒楄〃涓负 vmlinux 鎸囧畾鐩爣鏂囦欢銆傝繖浜涘垪琛ㄥ彇鍐充簬鍐呮牳閰嶇疆銆?

Kbuild 缂栬瘧鎵€鏈夌殑 $(obj-y) 鏂囦欢銆傜劧鍚庡畠璋冪敤 `$(AR) rcSTP` 灏嗚繖浜涙枃浠跺悎骞朵负涓€涓?built-in.a 鏂囦欢銆傝繖鏄竴涓病鏈夌鍙疯〃鐨勭簿绠€褰掓。銆傚畠闅忓悗浼氳 scripts/link-vmlinux.sh 閾炬帴杩?vmlinux銆?

$(obj-y) 涓殑鏂囦欢椤哄簭鏄湁鎰忎箟鐨勩€傚垪琛ㄤ腑鍏佽閲嶅锛氱涓€涓疄渚嬩細琚摼鎺ヨ繘 built-in.a锛屽悗缁疄渚嬪皢琚拷鐣ャ€?

閾炬帴椤哄簭鏄湁鎰忎箟鐨勶紝鍥犱负鏌愪簺鍑芥暟锛坢odule_init() / __initcall锛変細鎸夊畠浠嚭鐜扮殑椤哄簭鍦ㄥ惎鍔ㄦ湡闂磋璋冪敤銆傚洜姝よ璁颁綇锛屾敼鍙橀摼鎺ラ『搴忓彲鑳戒細鏀瑰彉 SCSI 鎺у埗鍣ㄨ妫€娴嬬殑椤哄簭锛屼粠鑰屾敼鍙樹綘鐨勭鐩樼紪鍙枫€?

```

  #drivers/isdn/i4l/Makefile
  # Makefile for the kernel ISDN subsystem and device drivers.
  # Each configuration option enables a list of files.
  obj-$(CONFIG_ISDN_I4L)         += isdn.o
  obj-$(CONFIG_ISDN_PPP_BSDCOMP) += isdn_bsdcomp.o

```
### Loadable module goals - obj-m


$(obj-m) 鎸囧畾浣滀负鍙姞杞藉唴鏍告ā鍧楁瀯寤虹殑鐩爣鏂囦欢銆?

涓€涓ā鍧楀彲浠ョ敱涓€涓簮鏂囦欢鎴栬嫢骞叉簮鏂囦欢鏋勫缓銆傚浜庡崟涓簮鏂囦欢鐨勬儏鍐碉紝kbuild makefile 鍙渶灏嗚鏂囦欢鍔犲叆 $(obj-m)銆?

```

  #drivers/isdn/i4l/Makefile
  obj-$(CONFIG_ISDN_PPP_BSDCOMP) += isdn_bsdcomp.o

```
娉ㄦ剰锛氬湪鏈緥涓?$(CONFIG_ISDN_PPP_BSDCOMP) 姹傚€间负 "m"銆?

濡傛灉涓€涓唴鏍告ā鍧楃敱澶氫釜婧愭枃浠舵瀯寤猴紝浣犱互涓庝笂杩扮浉鍚岀殑鏂瑰紡鎸囧畾瑕佹瀯寤轰竴涓ā鍧楋紱鐒惰€岋紝kbuild 闇€瑕佺煡閬撲綘鎯崇敤鍝簺鐩爣鏂囦欢鏉ユ瀯寤轰綘鐨勬ā鍧楋紝鍥犳浣犲繀椤婚€氳繃璁剧疆 $(<module_name>-y) 鍙橀噺鏉ュ憡鐭ュ畠銆?

```

  #drivers/isdn/i4l/Makefile
  obj-$(CONFIG_ISDN_I4L) += isdn.o
  isdn-y := isdn_net_lib.o isdn_v110.o isdn_common.o

```
鍦ㄦ湰渚嬩腑锛屾ā鍧楀悕灏嗕负 isdn.o銆侹build 灏嗙紪璇?$(isdn-y) 涓垪鍑虹殑鐩爣鏂囦欢锛岀劧鍚庡杩欎簺鏂囦欢鍒楄〃杩愯 `$(LD) -r` 鏉ョ敓鎴?isdn.o銆?

鐢变簬 kbuild 浼氳瘑鍒敤浜庡鍚堢洰鏍囩殑 $(<module_name>-y)锛屼綘鍙互浣跨敤 `CONFIG_` 绗﹀彿鐨勫€兼湁閫夋嫨鍦板皢涓€涓洰鏍囨枃浠朵綔涓哄鍚堢洰鏍囩殑涓€閮ㄥ垎鍖呭惈杩涙潵銆?

```

  #fs/ext2/Makefile
  obj-$(CONFIG_EXT2_FS) += ext2.o
  ext2-y := balloc.o dir.o file.o ialloc.o inode.o ioctl.o \
    namei.o super.o symlink.o
  ext2-$(CONFIG_EXT2_FS_XATTR) += xattr.o xattr_user.o \
    xattr_trusted.o

```
鍦ㄦ湰渚嬩腑锛屽彧鏈夊綋 $(CONFIG_EXT2_FS_XATTR) 姹傚€间负 "y" 鏃讹紝xattr.o銆亁attr_user.o 鍜?xattr_trusted.o 鎵嶆槸澶嶅悎鐩爣 ext2.o 鐨勪竴閮ㄥ垎銆?

娉ㄦ剰锛氬綋鐒讹紝褰撲綘灏嗙洰鏍囨瀯寤鸿繘鍐呮牳鏃讹紝涓婅堪璇硶鍚屾牱閫傜敤銆傚洜姝わ紝濡傛灉浣犺缃簡 CONFIG_EXT2_FS=y锛宬build 浼氬儚浣犻鏈熺殑閭ｆ牱锛屼粠鍚勪釜閮ㄥ垎鏋勫缓鍑轰竴涓?ext2.o 鏂囦欢锛岀劧鍚庡皢鍏堕摼鎺ヨ繘 built-in.a銆?

### Library file goals - lib-y


鐢?obj-* 鍒楀嚭鐨勭洰鏍囩敤浜庢ā鍧楋紝鎴栬鍚堝苟杩涜鐗瑰畾鐩綍鐨?built-in.a銆傝繕鏈変竴绉嶅彲鑳斤紝鍗冲垪鍑哄皢琚寘鍚繘搴?lib.a 鐨勭洰鏍囥€傛墍鏈夌敤 lib-y 鍒楀嚭鐨勭洰鏍囬兘浼氳鍚堝苟杩涜鐩綍鐨勫崟涓簱涓€傚悓鏃跺垪鍦?obj-y 鍜?lib-y 涓殑鐩爣涓嶄細琚寘鍚繘搴擄紝鍥犱负瀹冧滑鏃犺濡備綍閮藉彲琚闂埌銆備负淇濇寔涓€鑷达紝鍒楀湪 lib-m 涓殑鐩爣浼氳鍖呭惈杩?lib.a銆?

娉ㄦ剰锛屽悓涓€涓?kbuild makefile 鍙兘鏃㈠垪鍑鸿鍐呯疆鐨勭洰鏍囷紝涔熷垪鍑鸿浣滀负搴撲竴閮ㄥ垎鐨勭洰鏍囥€傚洜姝ゅ悓涓€涓洰褰曚腑鍙兘鍚屾椂鍖呭惈 built-in.a 鍜?lib.a 鏂囦欢銆?

```

  #arch/x86/lib/Makefile
  lib-y    := delay.o

```
杩欏皢鍩轰簬 delay.o 鍒涘缓涓€涓簱 lib.a銆備负浜嗚 kbuild 鐪熸璇嗗埆鍑烘鍦ㄦ瀯寤轰竴涓?lib.a锛岃鐩綍蹇呴』鍒楀湪 libs-y 涓€?

鍙﹁ `List directories to visit when descending`_銆?

lib-y 鐨勪娇鐢ㄩ€氬父闄愬埗鍦?`lib/` 鍜?`arch/*/lib`銆?

### Descending down in directories


Makefile 鍙礋璐ｆ瀯寤哄叾鑷韩鐩綍涓殑鐩爣銆傚瓙鐩綍涓殑鏂囦欢搴旂敱杩欎簺瀛愮洰褰曚腑鐨?Makefile 璐熻矗銆傚彧瑕佷綘璁╂瀯寤虹郴缁熺煡閬撹繖浜涘瓙鐩綍锛屽畠浼氳嚜鍔ㄥ湪瀛愮洰褰曚腑閫掑綊璋冪敤 make銆?

涓烘锛屼娇鐢?obj-y 鍜?obj-m銆俥xt2 浣嶄簬涓€涓嫭绔嬬殑鐩綍涓紝fs/ 涓殑 Makefile 閫氳繃浠ヤ笅璧嬪€煎憡璇?kbuild 杩涜閫掑綊涓嬮檷銆?

```

  #fs/Makefile
  obj-$(CONFIG_EXT2_FS) += ext2/

```
濡傛灉 CONFIG_EXT2_FS 琚缃负 "y"锛堝唴缃級鎴?"m"锛堟ā鍧楀寲锛夛紝鐩稿簲鐨?obj- 鍙橀噺灏嗚璁剧疆锛宬build 灏嗛€掑綊涓嬮檷杩涘叆 ext2 鐩綍銆?

Kbuild 鍒╃敤杩欎簺淇℃伅涓嶄粎鍐冲畾鏄惁闇€瑕佽闂鐩綍锛岃繕鍐冲畾鏄惁闇€瑕佸皢璇ョ洰褰曚腑鐨勭洰鏍囬摼鎺ヨ繘 vmlinux銆?

褰?Kbuild 浠?"y" 涓嬮檷杩涘叆鐩綍鏃讹紝璇ョ洰褰曚腑鐨勬墍鏈夊唴缃洰鏍囦細琚悎骞惰繘 built-in.a锛屽苟鏈€缁堣閾炬帴杩?vmlinux銆?

鐩稿弽锛屽綋 Kbuild 浠?"m" 涓嬮檷杩涘叆鐩綍鏃讹紝璇ョ洰褰曚腑娌℃湁浠讳綍鍐呭浼氳閾炬帴杩?vmlinux銆傚鏋滆鐩綍涓殑 Makefile 鎸囧畾浜?obj-y锛岄偅浜涚洰鏍囧皢琚仐鐣欎负瀛ゅ効銆傝繖寰堝彲鑳芥槸 Makefile 鎴?Kconfig 涓緷璧栭」鐨?bug銆?

Kbuild 杩樻敮鎸佷笓鐢ㄨ娉?subdir-y 鍜?subdir-m 鐢ㄤ簬涓嬮檷鍒板瓙鐩綍銆傚綋浣犳槑纭煡閬撳畠浠牴鏈笉鍖呭惈鍐呮牳绌洪棿鐩爣鏃讹紝瀹冨緢鍚堥€傘€備竴涓吀鍨嬬敤閫旀槸璁?Kbuild 涓嬮檷杩涘叆瀛愮洰褰曟潵鏋勫缓宸ュ叿銆?

```

  # scripts/Makefile
  subdir-$(CONFIG_GCC_PLUGINS) += gcc-plugins
  subdir-$(CONFIG_MODVERSIONS) += genksyms
  subdir-$(CONFIG_SECURITY_SELINUX) += selinux

```
涓?obj-y/m 涓嶅悓锛宻ubdir-y/m 涓嶉渶瑕佸熬閮ㄦ枩鏉狅紝鍥犱负姝よ娉曞缁堢敤浜庣洰褰曘€?

鍦ㄨ祴鍊兼椂浣跨敤 `CONFIG_` 鍙橀噺鏄壇濂藉疄璺点€傝繖鏍凤紝濡傛灉鐩稿簲鐨?`CONFIG_` 閫夐」鏃笉鏄?"y" 涔熶笉鏄?"m"锛宬build 鍙互瀹屽叏璺宠繃璇ョ洰褰曘€?

### Non-builtin vmlinux targets - extra-y


extra-y 鎸囧畾鏋勫缓 vmlinux 鎵€闇€銆佷絾鏈鍚堝苟杩?built-in.a 鐨勭洰鏍囥€?

绀轰緥鍖呮嫭锛?

1) vmlinux 閾炬帴鑴氭湰

   vmlinux 鐨勯摼鎺ヨ剼鏈綅浜?arch/$(SRCARCH)/kernel/vmlinux.lds

```

  # arch/x86/kernel/Makefile
  extra-y	+= vmlinux.lds

```
extra-y 鐜板湪宸茶寮冪敤锛屽洜涓哄畠绛変环浜庯細

  always-$(KBUILD_BUILTIN) += vmlinux.lds

$(extra-y) 搴斿彧鍖呭惈鍦ㄦ瀯寤?vmlinux 鏃舵墍闇€鐨勭洰鏍囥€?

褰?vmlinux 鏄剧劧涓嶆槸涓€涓渶缁堢洰鏍囨椂锛孠build 浼氳烦杩?extra-y銆傦紙渚嬪 `make modules`锛屾垨鏋勫缓澶栭儴妯″潡锛?

濡傛灉浣犳墦绠楁棤鏉′欢鍦版瀯寤虹洰鏍囷紝always-y锛堜笅涓€鑺傝В閲婏級鎵嶆槸姝ｇ‘鐨勮娉曘€?

### Always built goals - always-y


always-y 鎸囧畾鍦?Kbuild 璁块棶璇?Makefile 鏃跺瓧闈笂鎬绘槸琚瀯寤虹殑鐩爣銆?

```

  # ./Kbuild
  offsets-file := include/generated/asm-offsets.h
  always-y += $(offsets-file)

```
### Compilation flags


ccflags-y, asflags-y and ldflags-y
  杩欎笁涓爣蹇椾粎搴旂敤浜庡畠浠璧嬪€肩殑 kbuild makefile銆傚畠浠敤浜庨€掑綊鏋勫缓鏈熼棿鎵€鏈夋甯哥殑 cc銆乤s 鍜?ld 璋冪敤銆?

  ccflags-y 鎸囧畾浣跨敤 $(CC) 缂栬瘧鏃剁殑閫夐」銆?

```

    # drivers/acpi/acpica/Makefile
    ccflags-y				:= -Os -D_LINUX -DBUILDING_ACPICA
    ccflags-$(CONFIG_ACPI_DEBUG)	+= -DACPI_DEBUG_OUTPUT

```
  璇ュ彉閲忔槸蹇呰鐨勶紝鍥犱负椤跺眰 Makefile 鎷ユ湁鍙橀噺 $(KBUILD_CFLAGS) 骞跺皢鍏剁敤浜庢暣涓簮鐮佹爲鐨勭紪璇戞爣蹇椼€?

  asflags-y 鎸囧畾姹囩紪鍣ㄩ€夐」銆?

  Example::

```

    #arch/sparc/kernel/Makefile
    asflags-y := -ansi

```
  ldflags-y 鎸囧畾浣跨敤 $(LD) 閾炬帴鏃剁殑閫夐」銆?

  Example::

```

    #arch/cris/boot/compressed/Makefile
    ldflags-y += -T $(src)/decompress_$(arch-y).lds

```
subdir-ccflags-y, subdir-asflags-y
  涓婇潰鍒楀嚭鐨勪袱涓爣蹇楃被浼间簬 ccflags-y 鍜?asflags-y銆備笉鍚屼箣澶勫湪浜?subdir- 鍙樹綋瀵瑰畠浠墍鍦ㄧ殑 kbuild 鏂囦欢浠ュ強鎵€鏈夊瓙鐩綍閮界敓鏁堛€備娇鐢?subdir-* 鎸囧畾鐨勯€夐」浼氳鍔犲湪闈?subdir 鍙樹綋鎸囧畾鐨勯€夐」涔嬪墠銆?

```

    subdir-ccflags-y := -Werror

```
ccflags-remove-y, asflags-remove-y
  杩欎簺鏍囧織鐢ㄤ簬绉婚櫎缂栬瘧鍣ㄣ€佹眹缂栧櫒璋冪敤涓殑鐗瑰畾鏍囧織銆?

```

    ccflags-remove-$(CONFIG_MCOUNT) += -pg

```
CFLAGS_$@, AFLAGS_$@
  CFLAGS_$@ 鍜?AFLAGS_$@ 浠呭簲鐢ㄤ簬褰撳墠 kbuild makefile 涓殑鍛戒护銆?

  $(CFLAGS_$@) 涓?$(CC) 鎸囧畾姣忔枃浠堕€夐」銆?@ 閮ㄥ垎鏄竴涓瓧闈㈤噺鍊硷紝鎸囧畾瀹冩墍閽堝鐨勬枃浠躲€?

  CFLAGS_$@ 鐨勪紭鍏堢骇楂樹簬 ccflags-remove-y锛汣FLAGS_$@ 鍙互閲嶆柊娣诲姞琚?ccflags-remove-y 绉婚櫎鐨勭紪璇戝櫒鏍囧織銆?

```

    # drivers/scsi/Makefile
    CFLAGS_aha152x.o =   -DAHA152X_STAT -DAUTOCONF

```
  姝よ鎸囧畾浜?aha152x.o 鐨勭紪璇戞爣蹇椼€?

  $(AFLAGS_$@) 鏄拡瀵规眹缂栬瑷€婧愭枃浠剁殑绫讳技鐗规€с€?

  AFLAGS_$@ 鐨勪紭鍏堢骇楂樹簬 asflags-remove-y锛汚FLAGS_$@ 鍙互閲嶆柊娣诲姞琚?asflags-remove-y 绉婚櫎鐨勬眹缂栧櫒鏍囧織銆?

  Example::

```

    # arch/arm/kernel/Makefile
    AFLAGS_head.o        := -DTEXT_OFFSET=$(TEXT_OFFSET)
    AFLAGS_crunch-bits.o := -Wa,-mcpu=ep9312
    AFLAGS_iwmmxt.o      := -Wa,-mcpu=iwmmxt

```
### Dependency tracking


Kbuild 璺熻釜浠ヤ笅鏂归潰鐨勪緷璧栵細

1) 鎵€鏈夊厛鍐虫潯浠舵枃浠讹紙鍖呮嫭 `**.c` 鍜?`**.h`锛?
2) 鎵€鏈夊厛鍐虫潯浠舵枃浠朵腑浣跨敤鐨?`CONFIG_` 閫夐」
3) 鐢ㄤ簬缂栬瘧鐩爣鐨勫懡浠よ

鍥犳锛屽鏋滀綘鏇存敼浜?$(CC) 鐨勬煇涓€夐」锛屾墍鏈夊彈褰卞搷鐨勬枃浠堕兘灏嗚閲嶆柊缂栬瘧銆?

### Custom Rules


褰?kbuild 鍩虹璁炬柦娌℃湁鎻愪緵鎵€闇€鏀寔鏃朵娇鐢ㄨ嚜瀹氫箟瑙勫垯銆備竴涓吀鍨嬬ず渚嬫槸鏋勫缓杩囩▼涓敓鎴愮殑澶存枃浠躲€傚彟涓€涓ず渚嬫槸闇€瑕佽嚜瀹氫箟瑙勫垯鏉ュ噯澶囧惎鍔ㄦ槧鍍忕瓑鐨勬灦鏋勭浉鍏?Makefile銆?

鑷畾涔夎鍒欏儚鏅€?Make 瑙勫垯涓€鏍风紪鍐欍€侹build 骞朵笉鏄湪 Makefile 鎵€鍦ㄧ洰褰曚腑鎵ц锛屽洜姝ゆ墍鏈夎嚜瀹氫箟瑙勫垯搴斾娇鐢ㄧ浉瀵硅矾寰勫紩鐢ㄥ厛鍐虫潯浠舵枃浠跺拰鐩爣鏂囦欢銆?

瀹氫箟鑷畾涔夎鍒欐椂浣跨敤涓や釜鍙橀噺锛?

$(src)
  $(src) 鏄?Makefile 鎵€鍦ㄧ洰褰曘€傚紩鐢ㄤ綅浜庢簮鐮佹爲涓殑鏂囦欢鏃讹紝搴斿缁堜娇鐢?$(src)銆?

$(obj)
  $(obj) 鏄繚瀛樼洰鏍囩殑鐩綍銆傚紩鐢ㄧ敓鎴愮殑鏂囦欢鏃讹紝搴斿缁堜娇鐢?$(obj)銆傚浜庨渶瑕佸悓鏃堕€傜敤浜庣敓鎴愭枃浠跺拰鐪熷疄婧愭枃浠剁殑妯″紡瑙勫垯锛屼娇鐢?$(obj)锛圴PATH 涓嶄粎浼氬湪瀵硅薄鏍戜腑锛屼篃浼氬湪婧愮爜鏍戜腑甯姪鏌ユ壘鍏堝喅鏉′欢锛夈€?

```

    #drivers/scsi/Makefile
    $(obj)/53c8xx_d.h: $(src)/53c7,8xx.scr $(src)/script_asm.pl
    $(CPP) -DCHIP=810 - < $< | ... $(src)/script_asm.pl

```
  杩欐槸涓€涓嚜瀹氫箟瑙勫垯锛岄伒寰?make 鎵€闇€鐨勬甯歌娉曘€?

  鐩爣鏂囦欢渚濊禆浜庝袱涓厛鍐虫潯浠舵枃浠躲€傚鐩爣鏂囦欢鐨勫紩鐢ㄤ互 $(obj) 涓哄墠缂€锛屽鍏堝喅鏉′欢鐨勫紩鐢ㄤ互 $(src) 涓哄墠缂€锛堝洜涓哄畠浠笉鏄敓鎴愮殑鏂囦欢锛夈€?


$(srcroot)
  $(srcroot) 鎸囦綘姝ｅ湪鏋勫缓鐨勬簮鐮佹牴鐩綍锛屽畠鍙互鏄唴鏍告簮鐮侊紝涔熷彲浠ユ槸澶栭儴妯″潡婧愮爜锛屽彇鍐充簬鏄惁璁剧疆浜?KBUILD_EXTMOD銆傚畠鍙互鏄浉瀵硅矾寰勬垨缁濆璺緞锛屼絾濡傛灉璁剧疆浜?KBUILD_ABS_SRCTREE=1锛屽畠濮嬬粓鏄粷瀵硅矾寰勩€?

$(srctree)
  $(srctree) 鎸囧唴鏍告簮鐮佹爲鐨勬牴鐩綍銆傛瀯寤哄唴鏍告椂锛屽畠涓?$(srcroot) 鐩稿悓銆?

$(objtree)
  $(objtree) 鎸囧唴鏍稿璞℃爲鐨勬牴鐩綍銆傛瀯寤哄唴鏍告椂瀹冩槸 `.`锛屼絾鏋勫缓澶栭儴妯″潡鏃跺垯涓嶅悓銆?

$(kecho)
  鍦ㄨ鍒欎腑鍚戠敤鎴峰洖鏄句俊鎭€氬父鏄竴绉嶈壇濂藉疄璺碉紝浣嗗湪鎵ц `make -s` 鏃讹紝闄や簡璀﹀憡/閿欒涔嬪锛屼笉搴旀湡鏈涚湅鍒颁换浣曡緭鍑恒€備负浜嗘敮鎸佽繖涓€鐐癸紝kbuild 瀹氫箟浜?$(kecho)锛屽畠浼氬皢 $(kecho) 鍚庨潰鐨勬枃鏈洖鏄惧埌 stdout锛岄櫎闈炰娇鐢ㄤ簡 `make -s`銆?

```

    # arch/arm/Makefile
    $(BOOT_TARGETS): vmlinux
            $(Q)$(MAKE) $(build)=$(boot) MACHINE=$(MACHINE) $(boot)/$@
            @$(kecho) '  Kernel: $(boot)/$@ is ready'

```
  褰?kbuild 鍦?KBUILD_VERBOSE 鏈缃殑鎯呭喌涓嬫墽琛屾椂锛岄€氬父鍙樉绀哄懡浠ょ殑绠€鍐欏舰寮忋€備负浜嗚鑷畾涔夊懡浠や篃鍏峰杩欑琛屼负锛宬build 瑕佹眰璁剧疆涓や釜鍙橀噺锛?

    quiet_cmd_<command> - 搴斿洖鏄剧殑鍐呭
          cmd_<command> - 瑕佹墽琛岀殑鍛戒护

  Example::

```

    # lib/Makefile
    quiet_cmd_crc32 = GEN     $@
          cmd_crc32 = $< > $@

    $(obj)/crc32table.h: $(obj)/gen_crc32table
            $(call cmd,crc32)

```
  鏇存柊 $(obj)/crc32table.h 鐩爣鏃讹紝浠ヤ笅琛岋細

```

    GEN     lib/crc32table.h

```
  浼氶殢 ``make KBUILD_VERBOSE=`` 涓€璧锋樉绀恒€?

### Command change detection


褰撹鍒欒姹傚€兼椂锛屼細姣旇緝鐩爣涓庡叾鍏堝喅鏉′欢鏂囦欢涔嬮棿鐨勬椂闂存埑銆侴NU Make 浼氬湪浠讳竴鍏堝喅鏉′欢姣旂洰鏍囨洿鏂版椂鏇存柊鐩爣銆?

褰撳懡浠よ鑷笂娆¤皟鐢ㄤ互鏉ュ彂鐢熷彉鍖栨椂锛岀洰鏍囦篃搴旇閲嶆柊鏋勫缓銆侻ake 鏈韩涓嶆敮鎸佽繖涓€鐐癸紝鍥犳 Kbuild 閫氳繃涓€绉嶅厓缂栫▼鏉ュ疄鐜般€?

```

  quiet_cmd_<command> = ...
        cmd_<command> = ...

  <target>: <source(s)> FORCE
          $(call if_changed,<command>)

```
浠讳綍浣跨敤 if_changed 鐨勭洰鏍囧繀椤诲垪鍦?$(targets) 涓紝鍚﹀垯鍛戒护琛屾鏌ュ皢澶辫触锛岃鐩爣灏嗘€绘槸琚瀯寤恒€?

濡傛灉鐩爣宸茬粡鍒楀湪宸茬煡鐨勮娉曚腑锛屽 obj-y/m銆乴ib-y/m銆乪xtra-y/m銆乤lways-y/m銆乭ostprogs銆乽serprogs锛孠build 浼氳嚜鍔ㄥ皢鍏跺姞鍏?$(targets)銆傚惁鍒欙紝鐩爣蹇呴』琚樉寮忓姞鍏?$(targets)銆?

瀵?$(targets) 鐨勮祴鍊间笉甯?$(obj)/ 鍓嶇紑銆俰f_changed 鍙笌 `Custom Rules`_ 涓畾涔夌殑鑷畾涔夎鍒欑粨鍚堜娇鐢ㄣ€?

娉ㄦ剰锛氬繕璁?FORCE 鍏堝喅鏉′欢鏄竴涓吀鍨嬮敊璇€傚彟涓€涓父瑙佺殑闄烽槺鏄┖鐧芥湁鏃舵槸鏈夋剰涔夌殑锛涘浜?

```

  target: source(s) FORCE

```
**WRONG!**	$(call if_changed, objcopy)

娉ㄦ剰锛?
  if_changed 涓嶅簲鍦ㄥ悓涓€鐩爣涓婁娇鐢ㄨ秴杩囦竴娆°€傚畠浼氬皢鎵ц鐨勫懡浠ゅ瓨鍌ㄥ湪鐩稿簲鐨?.cmd 鏂囦欢涓紝澶氭璋冪敤浼氬鑷磋鐩栵紝骞跺湪鐩爣鏄渶鏂扮殑銆佷笖鍙湁鍛戒护鍙樺寲鐨勬祴璇曡Е鍙戝懡浠ゆ墽琛屾椂浜х敓涓嶆湡鏈涚殑缁撴灉銆?

### $(CC) support functions


鍐呮牳鍙兘浣跨敤澶氫釜涓嶅悓鐗堟湰鐨?$(CC) 鏋勫缓锛屾瘡涓増鏈敮鎸佷竴缁勭嫭鐗圭殑鐗规€у拰閫夐」銆俴build 鎻愪緵鍩烘湰鏀寔鏉ユ鏌?$(CC) 鐨勬湁鏁堥€夐」銆?(CC) 閫氬父鏄?gcc 缂栬瘧鍣紝浣嗕篃鏈夊叾浠栨浛浠ｆ柟妗堝彲鐢ㄣ€?

as-option
  as-option 鐢ㄤ簬妫€鏌?$(CC) 鈥斺€?褰撶敤浜庣紪璇戞眹缂栧櫒锛坄*.S`锛夋枃浠舵椂 鈥斺€?鏄惁鏀寔缁欏畾閫夐」銆傚鏋滅涓€涓€夐」涓嶈鏀寔锛屽彲浠ユ寚瀹氬彲閫夌殑绗簩涓€夐」銆?

```

    #arch/sh/Makefile
    cflags-y += $(call as-option,-Wa$(comma)-isa=$(isa-y),)

```
  鍦ㄤ笂闈㈢殑绀轰緥涓紝濡傛灉 $(CC) 鏀寔璇ラ€夐」锛宑flags-y 灏嗚璧嬪€间负 -Wa$(comma)-isa=$(isa-y)銆傜浜屼釜鍙傛暟鏄彲閫夌殑锛屽鏋滄彁渚涳紝灏嗗湪绗竴涓弬鏁颁笉琚敮鎸佹椂浣跨敤銆?

as-instr
  as-instr 妫€鏌ユ眹缂栧櫒鏄惁鎶ュ憡鐗瑰畾鎸囦护锛岀劧鍚庤緭鍑?option1 鎴?option2銆傛祴璇曟寚浠や腑鏀寔 C 杞箟銆傛敞鎰忥細as-instr-option 浣跨敤 KBUILD_AFLAGS 浣滀负姹囩紪鍣ㄩ€夐」銆?

cc-option
  cc-option 鐢ㄤ簬妫€鏌?$(CC) 鏄惁鏀寔缁欏畾閫夐」锛岃嫢涓嶆敮鎸佸垯浣跨敤鍙€夌殑绗簩涓€夐」銆?

```

    #arch/x86/Makefile
    cflags-y += $(call cc-option,-march=pentium-mmx,-march=i586)

```
  鍦ㄤ笂闈㈢殑绀轰緥涓紝濡傛灉 $(CC) 鏀寔璇ラ€夐」锛宑flags-y 灏嗚璧嬪€间负 -march=pentium-mmx锛屽惁鍒欎负 -march=i586銆俢c-option 鐨勭浜屼釜鍙傛暟鏄彲閫夌殑锛屽鏋滅渷鐣ワ紝涓旂涓€涓€夐」涓嶈鏀寔锛宑flags-y 灏嗚璧嬩负绌哄€笺€傛敞鎰忥細cc-option 浣跨敤 KBUILD_CFLAGS 浣滀负 $(CC) 鐨勯€夐」銆?

cc-option-yn
  cc-option-yn 鐢ㄤ簬妫€鏌?$(CC) 鏄惁鏀寔缁欏畾閫夐」锛岃嫢鏀寔鍒欒繑鍥?"y"锛屽惁鍒欒繑鍥?"n"銆?

```

    #arch/ppc/Makefile
    biarch := $(call cc-option-yn, -m32)
    aflags-$(biarch) += -a32
    cflags-$(biarch) += -m32

```
  鍦ㄤ笂闈㈢殑绀轰緥涓紝濡傛灉 $(CC) 鏀寔 -m32 閫夐」锛屽垯 $(biarch) 琚涓?y銆傚綋 $(biarch) 绛変簬 "y" 鏃讹紝灞曞紑鍚庣殑鍙橀噺 $(aflags-y) 鍜?$(cflags-y) 灏嗗垎鍒璧嬪€间负 -a32 鍜?-m32銆?

  娉ㄦ剰锛歝c-option-yn 浣跨敤 KBUILD_CFLAGS 浣滀负 $(CC) 鐨勯€夐」銆?

cc-disable-warning
  cc-disable-warning 妫€鏌?$(CC) 鏄惁鏀寔缁欏畾鐨勮鍛婏紝骞惰繑鍥炵敤浜庣鐢ㄥ畠鐨勫懡浠よ寮€鍏炽€傝繖涓壒娈婂嚱鏁版槸蹇呴渶鐨勶紝鍥犱负 gcc 4.4 鍙婁互鍚庣増鏈帴鍙椾换浣曟湭鐭ョ殑 -Wno-* 閫夐」锛屽苟浠呭綋婧愭枃浠朵腑鏈夊叾浠栬鍛婃椂鎵嶅鍏跺彂鍑鸿鍛娿€?

```

    KBUILD_CFLAGS += $(call cc-disable-warning, unused-but-set-variable)

```
  鍦ㄤ笂闈㈢殑绀轰緥涓紝-Wno-unused-but-set-variable 鍙細鍦?$(CC) 鐪熸鎺ュ彈瀹冩椂琚姞鍏?KBUILD_CFLAGS銆?

gcc-min-version
  gcc-min-version 娴嬭瘯 $(CONFIG_GCC_VERSION) 鐨勫€兼槸鍚﹀ぇ浜庢垨绛変簬鎵€缁欏€硷紝鑻ユ槸鍒欐眰鍊间负 y銆?

```

    cflags-$(call gcc-min-version, 110100) := -foo

```
  鍦ㄦ绀轰緥涓紝濡傛灉 $(CC) 鏄?gcc 涓?$(CONFIG_GCC_VERSION) >= 11.1锛宑flags-y 灏嗚璧嬪€间负 -foo銆?

clang-min-version
  clang-min-version 娴嬭瘯 $(CONFIG_CLANG_VERSION) 鐨勫€兼槸鍚﹀ぇ浜庢垨绛変簬鎵€缁欏€硷紝鑻ユ槸鍒欐眰鍊间负 y銆?

```

    cflags-$(call clang-min-version, 110000) := -foo

```
  鍦ㄦ绀轰緥涓紝濡傛灉 $(CC) 鏄?clang 涓?$(CONFIG_CLANG_VERSION) >= 11.0.0锛宑flags-y 灏嗚璧嬪€间负 -foo銆?

cc-cross-prefix
  cc-cross-prefix 鐢ㄤ簬妫€鏌?PATH 涓槸鍚﹀瓨鍦ㄥ甫鏈夋墍鍒楀墠缂€涔嬩竴鐨?$(CC)銆傝繑鍥炴壘鍒?prefix$(CC) 鐨勭涓€涓墠缂€ 鈥斺€?濡傛灉鏈壘鍒颁换浣?prefix$(CC)锛屽垯杩斿洖绌恒€?

  鍦?cc-cross-prefix 鐨勮皟鐢ㄤ腑锛岄澶栫殑鍓嶇紑鐢ㄥ崟涓┖鏍煎垎闅斻€?

  杩欎釜鍔熻兘瀵逛簬璇曞浘灏?CROSS_COMPILE 璁剧疆涓哄凡鐭ュ€笺€佷絾鍙兘鏈夊涓€煎彲渚涢€夋嫨鐨勬灦鏋?Makefile 寰堟湁鐢ㄣ€?

  浠呭缓璁湪浜ゅ弶鏋勫缓锛堜富鏈烘灦鏋勪笌鐩爣鏋舵瀯涓嶅悓锛夋椂灏濊瘯璁剧疆 CROSS_COMPILE銆傚鏋?CROSS_COMPILE 宸茶璁剧疆锛屽垯淇濈暀鍏舵棫鍊笺€?

```

    #arch/m68k/Makefile
    ifneq ($(SUBARCH),$(ARCH))
            ifeq ($(CROSS_COMPILE),)
                    CROSS_COMPILE := $(call cc-cross-prefix, m68k-linux-gnu-)
            endif
    endif

```
### $(RUSTC) support functions


rustc-min-version
  rustc-min-version 娴嬭瘯 $(CONFIG_RUSTC_VERSION) 鐨勫€兼槸鍚﹀ぇ浜庢垨绛変簬鎵€缁欏€硷紝鑻ユ槸鍒欐眰鍊间负 y銆?

```

    rustflags-$(call rustc-min-version, 108500) := -Cfoo

```
  鍦ㄦ绀轰緥涓紝濡傛灉 $(CONFIG_RUSTC_VERSION) >= 1.85.0锛宺ustflags-y 灏嗚璧嬪€间负 -Cfoo銆?

### $(LD) support functions


ld-option
  ld-option 鐢ㄤ簬妫€鏌?$(LD) 鏄惁鏀寔鎵€鎻愪緵鐨勯€夐」銆俵d-option 浠ヤ袱涓€夐」浣滀负鍙傛暟銆?

  绗簩涓弬鏁版槸鍙€夌殑锛屽綋 $(LD) 涓嶆敮鎸佺涓€涓€夐」鏃跺彲浣跨敤瀹冦€?

```

    #Makefile
    LDFLAGS_vmlinux += $(call ld-option, -X)

```
### Script invocation


Make 瑙勫垯鍙互璋冪敤鑴氭湰鏉ユ瀯寤哄唴鏍搞€傝鍒欏簲濮嬬粓鎻愪緵閫傚綋鐨勮В閲婂櫒鏉ユ墽琛岃剼鏈€傚畠浠笉搴斾緷璧栨墽琛屼綅琚缃紝涔熶笉搴旂洿鎺ヨ皟鐢ㄨ剼鏈€備负渚夸簬鎵嬪姩璋冪敤鑴氭湰锛堜緥濡傝皟鐢?./scripts/checkpatch.pl锛夛紝浠嶅缓璁负鑴氭湰璁剧疆鎵ц浣嶃€?

Kbuild 鎻愪緵鍙橀噺 $(CONFIG_SHELL)銆?(AWK)銆?(PERL) 鍜?$(PYTHON3) 鏉ュ紩鐢ㄧ浉搴旇剼鏈殑瑙ｉ噴鍣ㄣ€?

```

  #Makefile
  cmd_depmod = $(CONFIG_SHELL) $(srctree)/scripts/depmod.sh $(DEPMOD) \
          $(KERNELRELEASE)

```
## Host Program support


Kbuild 鏀寔鍦ㄤ富鏈轰笂鏋勫缓鐢ㄤ簬缂栬瘧闃舵鐨勫彲鎵ц鏂囦欢銆?

浣跨敤涓绘満鍙墽琛屾枃浠堕渶瑕佷袱姝ャ€?

绗竴姝ユ槸鍛婅瘔 kbuild 瀛樺湪涓€涓富鏈虹▼搴忋€傝繖鏄€氳繃鍙橀噺 `hostprogs` 鏉ュ畬鎴愮殑銆?

绗簩姝ユ槸鍚戣鍙墽琛屾枃浠舵坊鍔犳樉寮忎緷璧栥€傝繖鍙互閫氳繃涓ょ鏂瑰紡瀹屾垚锛氬湪瑙勫垯涓坊鍔犱緷璧栵紝鎴栦娇鐢ㄥ彉閲?`always-y`銆備互涓嬪皢鎻忚堪杩欎袱绉嶅彲鑳姐€?

### Simple Host Program


鍦ㄦ煇浜涙儏鍐典笅锛岄渶瑕佸湪杩愯鏋勫缓鐨勮绠楁満涓婄紪璇戝苟杩愯涓€涓▼搴忋€?

浠ヤ笅琛屽憡璇?kbuild锛岀▼搴?bin2hex 搴斿湪鏋勫缓涓绘満涓婃瀯寤恒€?

```

  hostprogs := bin2hex

```
Kbuild 鍦ㄤ笂闈㈢殑绀轰緥涓亣璁?bin2hex 鐢变綅浜庝笌 Makefile 鐩稿悓鐩綍涓殑鍗曚釜 C 婧愭枃浠?bin2hex.c 鏋勬垚銆?

### Composite Host Programs


涓绘満绋嬪簭鍙互鍩轰簬澶嶅悎鐩爣鏋勬垚銆傜敤浜庡畾涔変富鏈虹▼搴忓鍚堢洰鏍囩殑璇硶涓庣敤浜庡唴鏍哥洰鏍囩殑璇硶绫讳技銆?(<executable>-objs) 鍒楀嚭鐢ㄤ簬閾炬帴鏈€缁堝彲鎵ц鏂囦欢鐨勬墍鏈夌洰鏍囥€?

```

  #scripts/lxdialog/Makefile
  hostprogs     := lxdialog
  lxdialog-objs := checklist.o lxdialog.o

```
鎵╁睍鍚嶄负 .o 鐨勭洰鏍囩敱鐩稿簲鐨?.c 鏂囦欢缂栬瘧鑰屾潵銆傚湪涓婇潰鐨勭ず渚嬩腑锛宑hecklist.c 琚紪璇戜负 checklist.o锛宭xdialog.c 琚紪璇戜负 lxdialog.o銆?

鏈€鍚庯紝杩欎袱涓?.o 鏂囦欢琚摼鎺ュ埌鍙墽琛屾枃浠?lxdialog銆傛敞鎰忥細璇硶 <executable>-y 涓嶅厑璁哥敤浜庝富鏈虹▼搴忋€?

### Using C++ for host programs


kbuild 鎻愪緵瀵圭敤 C++ 缂栧啓鐨勪富鏈虹▼搴忕殑鏀寔銆傝繖浠呮槸涓烘敮鎸?kconfig 鑰屽紩鍏ョ殑锛屼笉寤鸿鏅亶浣跨敤銆?

```

  #scripts/kconfig/Makefile
  hostprogs     := qconf
  qconf-cxxobjs := qconf.o

```
鍦ㄤ笂闈㈢殑绀轰緥涓紝鍙墽琛屾枃浠剁敱 C++ 鏂囦欢 qconf.cc 缁勬垚 鈥斺€?鐢?$(qconf-cxxobjs) 鏍囪瘑銆?

濡傛灉 qconf 鐢?.c 鍜?.cc 鏂囦欢鐨勬贩鍚堢粍鎴愶紝鍒欏彲浠ヤ娇鐢ㄩ澶栫殑涓€琛屾潵鏍囪瘑杩欎竴鐐广€?

```

  #scripts/kconfig/Makefile
  hostprogs     := qconf
  qconf-cxxobjs := qconf.o
  qconf-objs    := check.o

```
### Using Rust for host programs


Kbuild 鎻愪緵瀵圭敤 Rust 缂栧啓鐨勪富鏈虹▼搴忕殑鏀寔銆傜劧鑰岋紝鐢变簬 Rust 宸ュ叿閾惧苟闈炲唴鏍哥紪璇戞墍蹇呴渶锛屽畠鍙兘鐢ㄤ簬闇€瑕?Rust 鍙敤鐨勫満鏅紙渚嬪鍚敤浜?`CONFIG_RUST` 鏃讹級銆?

```

  hostprogs     := target
  target-rust   := y

```
Kbuild 灏嗕娇鐢ㄤ綅浜庝笌 `Makefile` 鐩稿悓鐩綍涓殑 `target.rs` 浣滀负 crate 鏍规潵缂栬瘧 `target`銆傝 crate 鍙兘鐢卞涓簮鏂囦欢缁勬垚锛堣 `samples/rust/hostprogs`锛夈€?

### Controlling compiler options for host programs


缂栬瘧涓绘満绋嬪簭鏃讹紝鍙互璁剧疆鐗瑰畾鏍囧織銆傜▼搴忓皢濮嬬粓浣跨敤 $(HOSTCC) 浼犲叆 $(KBUILD_HOSTCFLAGS) 涓寚瀹氱殑閫夐」杩涜缂栬瘧銆?

瑕佷负鍦ㄨ Makefile 涓垱寤虹殑鎵€鏈変富鏈虹▼搴忚缃敓鏁堢殑鏍囧織锛屼娇鐢ㄥ彉閲?HOST_EXTRACFLAGS銆?

```

  #scripts/lxdialog/Makefile
  HOST_EXTRACFLAGS += -I/usr/include/ncurses

```
瑕佷负鍗曚釜鏂囦欢璁剧疆鐗瑰畾鏍囧織锛屼娇鐢ㄤ互涓嬫瀯閫狅細

```

  #arch/ppc64/boot/Makefile
  HOSTCFLAGS_piggyback.o := -DKERNELBASE=$(KERNELBASE)

```
涔熷彲浠ヤ负閾炬帴鍣ㄦ寚瀹氶澶栭€夐」銆?

```

  #scripts/kconfig/Makefile
  HOSTLDLIBS_qconf := -L$(QTDIR)/lib

```
閾炬帴 qconf 鏃讹紝灏嗕紶鍏ラ澶栭€夐」 `-L$(QTDIR)/lib`銆?

### When host programs are actually built


Kbuild 浠呭湪涓绘満绋嬪簭琚紩鐢ㄤ负鍏堝喅鏉′欢鏃舵墠浼氭瀯寤哄畠銆?

杩欐湁涓ょ鏂瑰紡锛?

(1) 鍦ㄨ嚜瀹氫箟瑙勫垯涓樉寮忓垪鍑哄厛鍐虫潯浠躲€?

```

      #drivers/pci/Makefile
      hostprogs := gen-devlist
      $(obj)/devlist.h: $(src)/pci.ids $(obj)/gen-devlist
      ( cd $(obj); ./gen-devlist ) < $<

```
    鐩爣 $(obj)/devlist.h 鍦?$(obj)/gen-devlist 鏇存柊涔嬪墠涓嶄細琚瀯寤恒€傛敞鎰忥紝鑷畾涔夎鍒欎腑瀵逛富鏈虹▼搴忕殑寮曠敤蹇呴』浠?$(obj) 涓哄墠缂€銆?

(2) Use always-y

    褰撲笉瀛樺湪鍚堥€傜殑鑷畾涔夎鍒欙紝涓斾富鏈虹▼搴忓簲鍦ㄨ繘鍏ユ煇涓?makefile 鏃惰鏋勫缓鏃讹紝搴斾娇鐢?always-y 鍙橀噺銆?

```

      #scripts/lxdialog/Makefile
      hostprogs     := lxdialog
      always-y      := $(hostprogs)

```
    Kbuild 涓烘鎻愪緵浜嗕互涓嬬畝鍐欏舰寮忥細

      hostprogs-always-y := lxdialog

    杩欏皢鍛婅瘔 kbuild 鏋勫缓 lxdialog锛屽嵆浣垮畠鏈浠讳綍瑙勫垯寮曠敤銆?

## Userspace Program support


涓庝富鏈虹▼搴忎竴鏍凤紝Kbuild 涔熸敮鎸佷负鐩爣鏋舵瀯锛堝嵆浣犳鍦ㄤ负涔嬫瀯寤哄唴鏍哥殑鐩稿悓鏋舵瀯锛夋瀯寤虹敤鎴风┖闂村彲鎵ц鏂囦欢銆?

璇硶闈炲父鐩镐技銆備笉鍚屼箣澶勫湪浜庝娇鐢?`userprogs` 鑰岄潪 `hostprogs`銆?

### Simple Userspace Program


浠ヤ笅琛屽憡璇?kbuild锛岀▼搴?bpf-direct 搴旈拡瀵圭洰鏍囨灦鏋勬瀯寤恒€?

```

  userprogs := bpf-direct

```
Kbuild 鍦ㄤ笂闈㈢殑绀轰緥涓亣璁?bpf-direct 鐢变綅浜庝笌 Makefile 鐩稿悓鐩綍涓殑鍗曚釜 C 婧愭枃浠?bpf-direct.c 鏋勬垚銆?

### Composite Userspace Programs


鐢ㄦ埛绌洪棿绋嬪簭鍙互鍩轰簬澶嶅悎鐩爣鏋勬垚銆傜敤浜庡畾涔夌敤鎴风┖闂寸▼搴忓鍚堢洰鏍囩殑璇硶涓庣敤浜庡唴鏍哥洰鏍囩殑璇硶绫讳技銆?(<executable>-objs) 鍒楀嚭鐢ㄤ簬閾炬帴鏈€缁堝彲鎵ц鏂囦欢鐨勬墍鏈夌洰鏍囥€?

```

  #samples/seccomp/Makefile
  userprogs      := bpf-fancy
  bpf-fancy-objs := bpf-fancy.o bpf-helper.o

```
鎵╁睍鍚嶄负 .o 鐨勭洰鏍囩敱鐩稿簲鐨?.c 鏂囦欢缂栬瘧鑰屾潵銆傚湪涓婇潰鐨勭ず渚嬩腑锛宐pf-fancy.c 琚紪璇戜负 bpf-fancy.o锛宐pf-helper.c 琚紪璇戜负 bpf-helper.o銆?

鏈€鍚庯紝杩欎袱涓?.o 鏂囦欢琚摼鎺ュ埌鍙墽琛屾枃浠?bpf-fancy銆傛敞鎰忥細璇硶 <executable>-y 涓嶅厑璁哥敤浜庣敤鎴风┖闂寸▼搴忋€?

### Controlling compiler options for userspace programs


缂栬瘧鐢ㄦ埛绌洪棿绋嬪簭鏃讹紝鍙互璁剧疆鐗瑰畾鏍囧織銆傜▼搴忓皢濮嬬粓浣跨敤 $(CC) 浼犲叆 $(KBUILD_USERCFLAGS) 涓寚瀹氱殑閫夐」杩涜缂栬瘧銆?

瑕佷负鍦ㄨ Makefile 涓垱寤虹殑鎵€鏈夌敤鎴风┖闂寸▼搴忚缃敓鏁堢殑鏍囧織锛屼娇鐢ㄥ彉閲?userccflags銆?

```

  # samples/seccomp/Makefile
  userccflags += -I usr/include

```
瑕佷负鍗曚釜鏂囦欢璁剧疆鐗瑰畾鏍囧織锛屼娇鐢ㄤ互涓嬫瀯閫狅細

```

  bpf-helper-userccflags += -I user/include

```
涔熷彲浠ヤ负閾炬帴鍣ㄦ寚瀹氶澶栭€夐」銆?

```

  # net/bpfilter/Makefile
  bpfilter_umh-userldflags += -static

```
瑕佹寚瀹氶摼鎺ュ埌鐢ㄦ埛绌洪棿绋嬪簭鐨勫簱锛屽彲浠ヤ娇鐢?`<executable>-userldlibs`銆俙userldlibs` 璇硶鎸囧畾閾炬帴鍒板綋鍓?Makefile 涓垱寤虹殑鎵€鏈夌敤鎴风┖闂寸▼搴忕殑搴撱€?

閾炬帴 bpfilter_umh 鏃讹紝灏嗕紶鍏ラ澶栭€夐」 -static銆?

浠庡懡浠よ锛孶SERCFLAGS 鍜?USERLDFLAGS <userkbuildflags> 涔熶細琚娇鐢ㄣ€?

### When userspace programs are actually built


Kbuild 浠呭湪琚憡鐭ユ椂鎵嶆瀯寤虹敤鎴风┖闂寸▼搴忋€傛湁涓ょ鏂瑰紡鍙互鍋氬埌杩欎竴鐐广€?

(1) 灏嗗叾娣诲姞涓哄彟涓€鏂囦欢鐨勫厛鍐虫潯浠?

```

      #net/bpfilter/Makefile
      userprogs := bpfilter_umh
      $(obj)/bpfilter_umh_blob.o: $(obj)/bpfilter_umh

    $(obj)/bpfilter_umh 鍦?$(obj)/bpfilter_umh_blob.o 涔嬪墠琚瀯寤恒€?

```
(2) Use always-y

```

      userprogs := binderfs_example
      always-y := $(userprogs)

```
    Kbuild 涓烘鎻愪緵浜嗕互涓嬬畝鍐欏舰寮忥細

      userprogs-always-y := binderfs_example

    杩欏皢鍛婅瘔 Kbuild 鍦ㄨ闂 Makefile 鏃舵瀯寤?binderfs_example銆?

## Kbuild clean infrastructure


`make clean` 鍒犻櫎缂栬瘧鍐呮牳鐨勫璞℃爲涓ぇ澶氭暟鐢熸垚鐨勬枃浠躲€傝繖鍖呮嫭璇稿涓绘満绋嬪簭涔嬬被鐨勭敓鎴愭枃浠躲€侹build 鐭ラ亾鍒楀湪 $(hostprogs)銆?(always-y)銆?(always-m)銆?(always-)銆?(extra-y)銆?(extra-) 鍜?$(targets) 涓殑鐩爣銆傚畠浠湪鎵ц `make clean` 鏃跺叏閮ㄨ鍒犻櫎銆傚尮閰?`**.[oas]`銆乣**.ko` 妯″紡浠ュ強 kbuild 鐢熸垚鐨勪竴浜涢檮鍔犳枃浠讹紝浼氬湪鎵ц `make clean` 鏃朵簬鏁翠釜鍐呮牳婧愮爜鏍戜腑琚垹闄ゃ€?

棰濆鐨勬枃浠舵垨鐩綍鍙互閫氳繃鍦?kbuild makefile 涓娇鐢?$(clean-files) 鎸囧畾銆?

```

  #lib/Makefile
  clean-files := crc32table.h

```
鎵ц `make clean` 鏃讹紝鏂囦欢 `crc32table.h` 灏嗚鍒犻櫎銆侹build 浼氬亣璁炬枃浠朵笌 Makefile 澶勪簬鐩稿悓鐨勭浉瀵圭洰褰曚腑銆?

瑕佸皢鏌愪簺鏂囦欢鎴栫洰褰曟帓闄ゅ湪 make clean 涔嬪锛屼娇鐢?$(no-clean-files) 鍙橀噺銆?

閫氬父 kbuild 鐢变簬 `obj-* := dir/` 鑰屼笅闄嶈繘鍏ュ瓙鐩綍锛屼絾鍦?kbuild 鍩虹璁炬柦涓嶈冻鐨勬灦鏋?makefile 涓紝鏈夋椂闇€瑕佹樉寮忔寚瀹氥€?

```

  #arch/x86/boot/Makefile
  subdir- := compressed

```
涓婅堪璧嬪€兼寚绀?kbuild 鍦ㄦ墽琛?`make clean` 鏃朵笅闄嶈繘鍏?compressed/ 鐩綍銆?

娉ㄦ剰 1锛歛rch/$(SRCARCH)/Makefile 涓嶈兘浣跨敤 `subdir-`锛屽洜涓鸿鏂囦欢琚寘鍚湪椤跺眰 makefile 涓€傜浉鍙嶏紝arch/$(SRCARCH)/Kbuild 鍙互浣跨敤 `subdir-`銆?

娉ㄦ剰 2锛氬垪鍦?core-y銆乴ibs-y銆乨rivers-y 鍜?net-y 涓殑鎵€鏈夌洰褰曢兘浼氬湪 `make clean` 鏈熼棿琚闂€?

## Architecture Makefiles


椤跺眰 Makefile 鍦ㄨ繘琛屼笅闄嶈繘鍏ュ悇涓洰褰曚箣鍓嶏紝瀹屾垚鐜鎼缓涓庡噯澶囧伐浣溿€?

椤跺眰 makefile 鍖呭惈閫氱敤閮ㄥ垎锛岃€?arch/$(SRCARCH)/Makefile 鍖呭惈涓烘墍杩版灦鏋勬惌寤?kbuild 鎵€闇€鐨勫唴瀹广€?

涓烘锛宎rch/$(SRCARCH)/Makefile 璁剧疆鑻ュ共鍙橀噺骞跺畾涔夊皯閲忕洰鏍囥€?

褰?kbuild 鎵ц鏃讹紝閬靛惊浠ヤ笅姝ラ锛堝ぇ鑷达級锛?

1) 鍐呮牳閰嶇疆 => 鐢熸垚 .config

2) 灏嗗唴鏍哥増鏈瓨鍏?include/linux/version.h

3) 鏇存柊鐩爣 prepare 鐨勬墍鏈夊叾浠栧厛鍐虫潯浠讹細

   - 棰濆鐨勫厛鍐虫潯浠跺湪 arch/$(SRCARCH)/Makefile 涓寚瀹?

4) 閫掑綊涓嬮檷杩涘叆鍒楀湪 init-** core** drivers-** net-** libs-* 涓殑鎵€鏈夌洰褰曞苟鏋勫缓鎵€鏈夌洰鏍囥€?

   - 涓婅堪鍙橀噺鐨勫€煎湪 arch/$(SRCARCH)/Makefile 涓睍寮€銆?

5) 鎵€鏈夌洰鏍囨枃浠堕殢鍚庤閾炬帴锛岀敓鎴愮殑鏂囦欢 vmlinux 浣嶄簬瀵硅薄鏍戠殑鏍圭洰褰曘€傛渶鍏堣閾炬帴鐨勭洰鏍囧垪鍦?scripts/head-object-list.txt 涓€?

6) 鏈€鍚庯紝鏋舵瀯鐩稿叧閮ㄥ垎杩涜浠讳綍鎵€闇€鐨勫悗澶勭悊骞舵瀯寤烘渶缁堢殑鍚姩鏄犲儚銆?

   - 杩欏寘鎷瀯寤哄惎鍔ㄨ褰?
   - 鍑嗗 initrd 鏄犲儚绛?

### Set variables to tweak the build to the architecture


KBUILD_LDFLAGS
  閫氱敤鐨?$(LD) 閫夐」

  鐢ㄤ簬閾炬帴鍣ㄦ墍鏈夎皟鐢ㄧ殑鏍囧織銆傞€氬父鎸囧畾浠跨湡灏辫冻澶熶簡銆?

```

    #arch/s390/Makefile
    KBUILD_LDFLAGS         := -m elf_s390

```
  娉ㄦ剰锛歭dflags-y 鍙敤浜庤繘涓€姝ュ畾鍒舵墍浣跨敤鐨勬爣蹇椼€傝 `Non-builtin vmlinux targets - extra-y`_銆?

LDFLAGS_vmlinux
  閾炬帴 vmlinux 鏃剁敤浜?$(LD) 鐨勯€夐」

  LDFLAGS_vmlinux 鐢ㄤ簬鎸囧畾鍦ㄩ摼鎺ユ渶缁?vmlinux 鏄犲儚鏃朵紶閫掔粰閾炬帴鍣ㄧ殑棰濆鏍囧織銆?

  LDFLAGS_vmlinux 浣跨敤 LDFLAGS_$@ 鏀寔銆?

```

    #arch/x86/Makefile
    LDFLAGS_vmlinux := -e stext

```
OBJCOPYFLAGS
  objcopy 鏍囧織

  褰撲娇鐢?$(call if_changed,objcopy) 杞崲 .o 鏂囦欢鏃讹紝灏嗕娇鐢?OBJCOPYFLAGS 涓寚瀹氱殑鏍囧織銆?

  $(call if_changed,objcopy) 甯哥敤浜庡湪 vmlinux 涓婄敓鎴愬師濮嬩簩杩涘埗鏂囦欢銆?

```

    #arch/s390/Makefile
    OBJCOPYFLAGS := -O binary

    #arch/s390/boot/Makefile
    $(obj)/image: vmlinux FORCE
            $(call if_changed,objcopy)

```
  鍦ㄦ绀轰緥涓紝浜岃繘鍒?$(obj)/image 鏄?vmlinux 鐨勪簩杩涘埗鐗堟湰銆?(call if_changed,xxx) 鐨勭敤娉曞皢鍦ㄥ悗闈㈡弿杩般€?

KBUILD_AFLAGS
  姹囩紪鍣ㄦ爣蹇?

  榛樿鍊?鈥斺€?瑙侀《灞?Makefile銆?

  鏍规嵁鏋舵瀯闇€瑕佽拷鍔犳垨淇敼銆?

```

    #arch/sparc64/Makefile
    KBUILD_AFLAGS += -m64 -mcpu=ultrasparc

```
KBUILD_CFLAGS
  $(CC) 缂栬瘧鍣ㄦ爣蹇?

  榛樿鍊?鈥斺€?瑙侀《灞?Makefile銆?

  鏍规嵁鏋舵瀯闇€瑕佽拷鍔犳垨淇敼銆?

  閫氬父锛孠BUILD_CFLAGS 鍙橀噺鍙栧喅浜庨厤缃€?

```

    #arch/x86/boot/compressed/Makefile
    cflags-$(CONFIG_X86_32) := -march=i386
    cflags-$(CONFIG_X86_64) := -mcmodel=small
    KBUILD_CFLAGS += $(cflags-y)

```
  璁稿鏋舵瀯 Makefile 浼氬姩鎬佽繍琛岀洰鏍?C 缂栬瘧鍣ㄦ潵鎺㈡祴鍙楁敮鎸佺殑閫夐」锛?

```

    #arch/x86/Makefile

    ...
    cflags-$(CONFIG_MPENTIUMII)     += $(call cc-option,\
						-march=pentium2,-march=i686)
    ...
    # Disable unit-at-a-time mode ...
    KBUILD_CFLAGS += $(call cc-option,-fno-unit-at-a-time)
    ...


  绗竴涓ず渚嬪埄鐢ㄤ簡閰嶇疆閫夐」鍦ㄨ閫変腑鏃跺睍寮€涓?"y" 鐨勬妧宸с€?

```
KBUILD_RUSTFLAGS
  $(RUSTC) 缂栬瘧鍣ㄦ爣蹇?

  榛樿鍊?鈥斺€?瑙侀《灞?Makefile銆?

  鏍规嵁鏋舵瀯闇€瑕佽拷鍔犳垨淇敼銆?

  閫氬父锛孠BUILD_RUSTFLAGS 鍙橀噺鍙栧喅浜庨厤缃€?

  娉ㄦ剰锛岀洰鏍囪鑼冩枃浠剁殑鐢熸垚锛堢敤浜?`--target`锛夊湪 `scripts/generate_rust_target.rs` 涓鐞嗐€?

KBUILD_AFLAGS_KERNEL
  涓撶敤浜庡唴缃殑姹囩紪鍣ㄩ€夐」

  $(KBUILD_AFLAGS_KERNEL) 鍖呭惈鐢ㄤ簬缂栬瘧甯搁┗鍐呮牳浠ｇ爜鐨勯澶?C 缂栬瘧鍣ㄦ爣蹇椼€?

KBUILD_AFLAGS_MODULE
  涓撶敤浜庢ā鍧楃殑姹囩紪鍣ㄩ€夐」

  $(KBUILD_AFLAGS_MODULE) 鐢ㄤ簬娣诲姞鐢ㄤ簬姹囩紪鍣ㄧ殑鏋舵瀯鐩稿叧閫夐」銆?

  浠庡懡浠よ搴斾娇鐢?AFLAGS_MODULE锛堣 kbuild.rst锛夈€?

KBUILD_CFLAGS_KERNEL
  涓撶敤浜庡唴缃殑 $(CC) 閫夐」

  $(KBUILD_CFLAGS_KERNEL) 鍖呭惈鐢ㄤ簬缂栬瘧甯搁┗鍐呮牳浠ｇ爜鐨勯澶?C 缂栬瘧鍣ㄦ爣蹇椼€?

KBUILD_CFLAGS_MODULE
  鏋勫缓妯″潡鏃剁敤鍒扮殑 $(CC) 閫夐」

  $(KBUILD_CFLAGS_MODULE) 鐢ㄤ簬娣诲姞鐢ㄤ簬 $(CC) 鐨勬灦鏋勭浉鍏抽€夐」銆?

  浠庡懡浠よ搴斾娇鐢?CFLAGS_MODULE锛堣 kbuild.rst锛夈€?

KBUILD_RUSTFLAGS_KERNEL
  涓撶敤浜庡唴缃殑 $(RUSTC) 閫夐」

  $(KBUILD_RUSTFLAGS_KERNEL) 鍖呭惈鐢ㄤ簬缂栬瘧甯搁┗鍐呮牳浠ｇ爜鐨勯澶?Rust 缂栬瘧鍣ㄦ爣蹇椼€?

KBUILD_RUSTFLAGS_MODULE
  鏋勫缓妯″潡鏃剁敤鍒扮殑 $(RUSTC) 閫夐」

  $(KBUILD_RUSTFLAGS_MODULE) 鐢ㄤ簬娣诲姞鐢ㄤ簬 $(RUSTC) 鐨勬灦鏋勭浉鍏抽€夐」銆?

  浠庡懡浠よ搴斾娇鐢?RUSTFLAGS_MODULE锛堣 kbuild.rst锛夈€?

KBUILD_LDFLAGS_MODULE
  閾炬帴妯″潡鏃剁敤鍒扮殑 $(LD) 閫夐」

  $(KBUILD_LDFLAGS_MODULE) 鐢ㄤ簬娣诲姞鐢ㄤ簬閾炬帴妯″潡鏃剁殑鏋舵瀯鐩稿叧閫夐」銆傝繖閫氬父鏄竴涓摼鎺ュ櫒鑴氭湰銆?

  浠庡懡浠よ搴斾娇鐢?LDFLAGS_MODULE锛堣 kbuild.rst锛夈€?

KBUILD_LDS
  甯︽湁瀹屾暣璺緞鐨勯摼鎺ュ櫒鑴氭湰銆傜敱椤跺眰 Makefile 璧嬪€笺€?

KBUILD_VMLINUX_OBJS
  vmlinux 鐨勬墍鏈夌洰鏍囨枃浠躲€傚畠浠互 KBUILD_VMLINUX_OBJS 涓垪鍑虹殑鐩稿悓椤哄簭閾炬帴杩?vmlinux銆?

  scripts/head-object-list.txt 涓垪鍑虹殑鐩爣涓轰緥澶栵紱瀹冧滑琚斁缃湪鍏朵粬鐩爣涔嬪墠銆?

KBUILD_VMLINUX_LIBS
  vmlinux 鐨勬墍鏈?.a `lib` 鏂囦欢銆侹BUILD_VMLINUX_OBJS 鍜?KBUILD_VMLINUX_LIBS 鍏卞悓鎸囧畾浜嗙敤浜庨摼鎺?vmlinux 鐨勬墍鏈夌洰鏍囨枃浠躲€?

### Add prerequisites to archheaders


archheaders: 瑙勫垯鐢ㄤ簬鐢熸垚鍙兘鐢?`make headers_install` 瀹夎鍒扮敤鎴风┖闂寸殑澶存枃浠躲€?

褰撳湪鏋舵瀯鏈韩涓婅繍琛屾椂锛屽畠浼氬湪 `make archprepare` 涔嬪墠杩愯銆?

### Add prerequisites to archprepare


archprepare: 瑙勫垯鐢ㄤ簬鍒楀嚭鍦ㄥ紑濮嬩笅闄嶈繘鍏ュ瓙鐩綍涔嬪墠闇€瑕佹瀯寤虹殑鍏堝喅鏉′欢銆?

杩欓€氬父鐢ㄤ簬鍖呭惈姹囩紪甯搁噺鐨勫ご鏂囦欢銆?

```

  #arch/arm/Makefile
  archprepare: maketools

```
鍦ㄦ绀轰緥涓紝鏂囦欢鐩爣 maketools 灏嗗湪涓嬮檷杩涘叆瀛愮洰褰曚箣鍓嶈澶勭悊銆?

鍙﹁绔犺妭 XXX-TODO锛屽畠鎻忚堪浜?kbuild 濡備綍鏀寔鐢熸垚鍋忕Щ澶存枃浠躲€?

### List directories to visit when descending


鏋舵瀯 Makefile 涓庨《灞?Makefile 鍗忎綔锛屽畾涔夋寚瀹氬浣曟瀯寤?vmlinux 鏂囦欢鐨勫彉閲忋€傛敞鎰忥紝妯″潡娌℃湁鐩稿簲鐨勬灦鏋勭浉鍏崇珷鑺傦紱妯″潡鐨勬瀯寤烘満鍒跺畬鍏ㄤ笌鏋舵瀯鏃犲叧銆?

core-y, libs-y, drivers-y
  $(libs-y) 鍒楀嚭鍙畾浣?lib.a 褰掓。鐨勭洰褰曘€?

  鍏朵綑鍒楀嚭鍙畾浣?built-in.a 鐩爣鏂囦欢鐨勭洰褰曘€?

  鐒跺悗鍏朵綑鎸変互涓嬮『搴忥細

    $(core-y), $(libs-y), $(drivers-y)

  椤跺眰 Makefile 涓烘墍鏈夐€氱敤鐩綍瀹氫箟鍙栧€硷紝鑰?arch/$(SRCARCH)/Makefile 鍙坊鍔犳灦鏋勭浉鍏崇殑鐩綍銆?

```

    # arch/sparc/Makefile
    core-y                 += arch/sparc/

    libs-y                 += arch/sparc/prom/
    libs-y                 += arch/sparc/lib/

    drivers-$(CONFIG_PM) += arch/sparc/power/

```
### Architecture-specific boot images


鏋舵瀯 Makefile 鎸囧畾灏?vmlinux 鏂囦欢鍘嬬缉銆佺敤寮曞浠ｇ爜鍖呰９骞跺皢缁撴灉鏂囦欢澶嶅埗鍒版煇澶勭殑鐩爣銆傝繖鍖呮嫭鍚勭瀹夎鍛戒护銆傚疄闄呯洰鏍囧湪鍚勬灦鏋勯棿骞朵笉鏍囧噯鍖栥€?

閫氬父灏嗕换浣曢澶栧鐞嗘斁鍦?arch/$(SRCARCH)/ 涓嬬殑 boot/ 鐩綍涓€?

Kbuild 娌℃湁鎻愪緵浠讳綍鏅鸿兘鏂瑰紡鏉ユ敮鎸佹瀯寤?boot/ 涓寚瀹氱殑鐩爣銆傚洜姝?arch/$(SRCARCH)/Makefile 搴旀墜鍔ㄨ皟鐢?make 鏉ユ瀯寤?boot/ 涓殑鐩爣銆?

鎺ㄨ崘鐨勫仛娉曟槸鍦?arch/$(SRCARCH)/Makefile 涓寘鍚揩鎹锋柟寮忥紝骞跺湪鍚戜笅璋冪敤 arch/$(SRCARCH)/boot/Makefile 鏃朵娇鐢ㄥ畬鏁磋矾寰勩€?

```

  #arch/x86/Makefile
  boot := arch/x86/boot
  bzImage: vmlinux
          $(Q)$(MAKE) $(build)=$(boot) $(boot)/$@

```
`$(Q)$(MAKE) $(build)=<dir>` 鏄皟鐢ㄥ瓙鐩綍涓?make 鐨勬帹鑽愭柟寮忋€?

瀵逛簬鏋舵瀯鐩稿叧鐩爣鐨勫懡鍚嶆病鏈夎鍒欙紝浣嗘墽琛?`make help` 浼氬垪鍑烘墍鏈夌浉鍏崇洰鏍囥€備负浜嗘敮鎸佽繖涓€鐐癸紝蹇呴』瀹氫箟 $(archhelp)銆?

```

  #arch/x86/Makefile
  define archhelp
    echo  '* bzImage      - Compressed kernel image (arch/x86/boot/bzImage)'
  endif

```
褰?make 涓嶅甫鍙傛暟鎵ц鏃讹紝閬囧埌鐨勭涓€涓洰鏍囧皢琚瀯寤恒€傚湪椤跺眰 Makefile 涓紝瀛樺湪鐨勭涓€涓洰鏍囨槸 all:銆?

鏋舵瀯鍦ㄩ粯璁ゆ儏鍐典笅搴斿缁堟瀯寤轰竴涓彲鍚姩鏄犲儚銆傚湪 `make help` 涓紝榛樿鐩爣浠?`*` 楂樹寒鏄剧ず銆?

鍚?all: 娣诲姞涓€涓柊鐨勫厛鍐虫潯浠朵互閫夋嫨涓嶅悓浜?vmlinux 鐨勯粯璁ょ洰鏍囥€?

```

  #arch/x86/Makefile
  all: bzImage

```
褰撲笉甯﹀弬鏁版墽琛?`make` 鏃讹紝灏嗘瀯寤?bzImage銆?

### Commands useful for building a boot image


Kbuild 鎻愪緵浜嗕竴浜涘湪鏋勫缓鍚姩鏄犲儚鏃舵湁鐢ㄧ殑瀹忋€?

ld
  閾炬帴鐩爣銆傞€氬父锛孡DFLAGS_$@ 鐢ㄤ簬涓?ld 璁剧疆鐗瑰畾閫夐」銆?

```

    #arch/x86/boot/Makefile
    LDFLAGS_bootsect := -Ttext 0x0 -s --oformat binary
    LDFLAGS_setup    := -Ttext 0x0 -s --oformat binary -e begtext

    targets += setup setup.o bootsect bootsect.o
    $(obj)/setup $(obj)/bootsect: %: %.o FORCE
            $(call if_changed,ld)

```
  鍦ㄦ绀轰緥涓紝鏈変袱涓彲鑳界殑鐩爣锛岄渶瑕佷笉鍚岀殑閾炬帴鍣ㄩ€夐」銆傞摼鎺ュ櫒閫夐」浣跨敤 LDFLAGS_$@ 璇硶鎸囧畾 鈥斺€?姣忎釜娼滃湪鐩爣涓€涓€?

  $(targets) 琚祴鍊间负鎵€鏈夋綔鍦ㄧ洰鏍囷紝鐢辨 kbuild 鐭ラ亾杩欎簺鐩爣骞跺皢锛?

  1) 妫€鏌ュ懡浠よ鐨勫彉鍖?
  2) 鍦?make clean 鏈熼棿鍒犻櫎鐩爣

  ``: %: %.o`` 閮ㄥ垎鐨勫厛鍐虫潯浠舵槸涓€涓畝鍐欙紝浣挎垜浠笉蹇呭垪鍑?setup.o 鍜?bootsect.o 鏂囦欢銆?

  娉ㄦ剰锛?
  蹇樿 ``targets :=`` 璧嬪€兼槸涓€涓父瑙侀敊璇紝浼氬鑷寸洰鏍囨枃浠跺湪娌℃湁鏄庢樉鍘熷洜鐨勬儏鍐典笅琚噸鏂扮紪璇戙€?

objcopy
  澶嶅埗浜岃繘鍒舵枃浠躲€傞€氬父浣跨敤 arch/$(SRCARCH)/Makefile 涓寚瀹氱殑 OBJCOPYFLAGS銆?

  OBJCOPYFLAGS_$@ 鍙敤浜庤缃澶栭€夐」銆?

gzip
  鍘嬬缉鐩爣銆備娇鐢ㄦ渶澶у帇缂╂潵鍘嬬缉鐩爣銆?

```

    #arch/x86/boot/compressed/Makefile
    $(obj)/vmlinux.bin.gz: $(vmlinux.bin.all-y) FORCE
            $(call if_changed,gzip)

```
dtc
  鍒涘缓鎵佸钩璁惧鏍?blob 瀵硅薄锛岄€傚悎閾炬帴杩?vmlinux銆傞摼鎺ヨ繘 vmlinux 鐨勮澶囨爲 blob 琚斁缃湪鏄犲儚鐨勪竴涓?init 娈典腑銆傚钩鍙颁唬鐮?**蹇呴』** 鍦ㄨ皟鐢?unflatten_device_tree() 涔嬪墠灏嗚 blob 澶嶅埗鍒伴潪 init 鍐呭瓨銆?

  瑕佷娇鐢ㄦ鍛戒护锛屽彧闇€灏?`*.dtb` 鍔犲叆 obj-y 鎴?targets锛屾垨璁╁叾浠栨煇涓洰鏍囦緷璧栦簬 `%.dtb`銆?

  瀛樺湪涓€鏉′腑蹇冭鍒欑敤浜庝粠 `$(src)/%.dts` 鍒涘缓 `$(obj)/%.dtb`锛涙灦鏋?Makefile 鏃犻渶鏄惧紡鍐欏嚭璇ヨ鍒欍€?

```

    targets += $(dtb-y)
    DTC_FLAGS ?= -p 1024

```
### Preprocessing linker scripts


鏋勫缓 vmlinux 鏄犲儚鏃讹紝浣跨敤閾炬帴鑴氭湰 arch/$(SRCARCH)/kernel/vmlinux.lds銆?

璇ヨ剼鏈槸鍚岀洰褰曚笅鏂囦欢 vmlinux.lds.S 鐨勯澶勭悊鍙樹綋銆?

kbuild 璁よ瘑 .lds 鏂囦欢骞跺寘鍚竴鏉¤鍒?`**lds.S` -> `**lds`銆?

```

  #arch/x86/kernel/Makefile
  extra-y := vmlinux.lds

```
瀵?extra-y 鐨勮祴鍊肩敤浜庡憡璇?kbuild 鏋勫缓鐩爣 vmlinux.lds銆?

瀵?$(CPPFLAGS_vmlinux.lds) 鐨勮祴鍊煎憡璇?kbuild 鍦ㄦ瀯寤虹洰鏍?vmlinux.lds 鏃朵娇鐢ㄦ寚瀹氱殑閫夐」銆?

```

  KBUILD_CPPFLAGS      : Set in top-level Makefile
  cppflags-y           : May be set in the kbuild makefile
  CPPFLAGS_$(@F)       : Target-specific flags.
                         Note that the full filename is used in this
                         assignment.

```
`*lds` 鏂囦欢鐨?kbuild 鍩虹璁炬柦鍦ㄥ涓灦鏋勭浉鍏虫枃浠朵腑琚娇鐢ㄣ€?

### Generic header files


鐩綍 include/asm-generic 鍖呭惈鍙湪鍚勪釜鏋舵瀯涔嬮棿鍏变韩鐨勫ご鏂囦欢銆?

浣跨敤閫氱敤澶存枃浠剁殑鎺ㄨ崘鏂规硶鏄湪 Kbuild 鏂囦欢涓垪鍑鸿鏂囦欢銆?

鏈夊叧璇硶绛夌殑鏇村淇℃伅锛岃 `generic-y`_銆?

### Post-link pass


濡傛灉鏂囦欢 arch/xxx/Makefile.postlink 瀛樺湪锛岃 makefile 灏嗚璋冪敤浠ュ鍚庨摼鎺ュ璞★紙vmlinux 鍜?modules.ko锛夎繍琛屽悗閾炬帴澶勭悊锛屼緵鏋舵瀯浣跨敤銆傚畠杩樺繀椤诲鐞?clean 鐩爣銆?

姝よ繃绋嬪湪 kallsyms 鐢熸垚涔嬪悗杩愯銆傚鏋滄灦鏋勯渶瑕佷慨鏀圭鍙蜂綅缃紝鑰屼笉鏄搷浣?kallsyms锛岄偅涔堜负 .tmp_vmlinux? 鐩爣娣诲姞鍙︿竴涓?postlink 鐩爣銆佺敱 link-vmlinux.sh 璋冪敤鍙兘鏇存柟渚裤€?

渚嬪锛宲owerpc 鐢ㄥ畠鏉ユ鏌ラ摼鎺ュ悗鐨?vmlinux 鏂囦欢鐨勯噸瀹氫綅瀹屾暣鎬с€?

## Kbuild syntax for exported headers


鍐呮牳鍖呭惈涓€缁勫鍑哄埌鐢ㄦ埛绌洪棿鐨勫ご鏂囦欢銆傝澶氬ご鏂囦欢鍙互鍘熸牱瀵煎嚭锛屼絾鍏朵粬澶存枃浠跺湪鍙緵鐢ㄦ埛绌洪棿浣跨敤涔嬪墠闇€瑕佹渶灏戠殑棰勫鐞嗐€?

棰勫鐞嗕細锛?

- 涓㈠純鍐呮牳鐗瑰畾鐨勬敞瑙?
- 涓㈠純瀵?compiler.h 鐨勫寘鍚?
- 涓㈠純鎵€鏈夊唴鏍稿唴閮紙鐢?`ifdef __KERNEL__` 淇濇姢锛夌殑娈?

include/uapi/銆乮nclude/generated/uapi/銆乤rch/<arch>/include/uapi/ 鍜?arch/<arch>/include/generated/uapi/ 涓嬬殑鎵€鏈夊ご鏂囦欢閮戒細琚鍑恒€?

鍙互鍦?arch/<arch>/include/uapi/asm/ 鍜?arch/<arch>/include/asm/ 涓嬪畾涔?Kbuild 鏂囦欢锛屼互鍒楀嚭鏉ヨ嚜 asm-generic 鐨?asm 鏂囦欢銆?

鏈夊叧 Kbuild 鏂囦欢鐨勮娉曪紝瑙佸悗缁珷鑺傘€?

### no-export-headers


no-export-headers 鏈川涓婄敱 include/uapi/linux/Kbuild 浣跨敤锛屼互閬垮厤鍦ㄤ笉鏀寔鏌愪簺澶存枃浠讹紙渚嬪 kvm.h锛夌殑鏋舵瀯涓婂鍑哄畠浠€傚簲灏介噺閬垮厤浣跨敤瀹冦€?

### generic-y


濡傛灉鏌愪釜鏋舵瀯閫愬瓧浣跨敤鏉ヨ嚜 include/asm-generic 鐨勪竴涓ご鏂囦欢鍓湰锛屽垯鍦ㄦ枃浠?arch/$(SRCARCH)/include/asm/Kbuild 涓寜濡備笅鏂瑰紡鍒楀嚭锛?

```

  #arch/x86/include/asm/Kbuild
  generic-y += termios.h
  generic-y += rtc.h

```
鍦ㄦ瀯寤虹殑鍑嗗闃舵浼氱敓鎴愪竴涓寘瑁呭寘鍚?

```

  arch/$(SRCARCH)/include/generated/asm

```
褰撳鍑轰竴涓灦鏋勪娇鐢ㄩ€氱敤澶寸殑澶存枃浠舵椂锛屼細鐢熸垚绫讳技鐨勫寘瑁呬綔涓?

```

  usr/include/asm

```
鍦ㄨ繖涓ょ鎯呭喌涓嬶紝鐢熸垚鐨勫寘瑁呴兘濡備笅鎵€绀猴細

```

  #include <asm-generic/termios.h>

```
### generated-y


濡傛灉鏌愪釜鏋舵瀯鍦?generic-y 鍖呰涔嬪杩樼敓鎴愬叾浠栧ご鏂囦欢锛実enerated-y 鎸囧畾瀹冧滑銆?

杩欏彲浠ラ樆姝㈠畠浠褰撲綔杩囨椂鐨?asm-generic 鍖呰鑰岃鍒犻櫎銆?

```

  #arch/x86/include/asm/Kbuild
  generated-y += syscalls_32.h

```
### mandatory-y


mandatory-y 鏈川涓婄敱 include/(uapi/)asm-generic/Kbuild 浣跨敤锛岀敤浜庡畾涔夋墍鏈夋灦鏋勯兘蹇呴』鍏峰鐨勬渶灏?ASM 澶存枃浠堕泦鍚堛€?

瀹冪被浼间簬鍙€夌殑 generic-y銆傚鏋?arch/$(SRCARCH)/include/(uapi/)/asm 涓己灏戞煇涓己鍒跺ご鏂囦欢锛孠build 灏嗚嚜鍔ㄧ敓鎴愯 asm-generic 澶存枃浠剁殑鍖呰銆?

## Kbuild Variables


椤跺眰 Makefile 瀵煎嚭浠ヤ笅鍙橀噺锛?

VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION
  杩欎簺鍙橀噺瀹氫箟褰撳墠鐨勫唴鏍哥増鏈€傚皯鏁版灦鏋?Makefile 浼氱洿鎺ヤ娇鐢ㄨ繖浜涘€硷紱瀹冧滑搴斿綋鏀圭敤 $(KERNELRELEASE)銆?

  $(VERSION)銆?(PATCHLEVEL) 鍜?$(SUBLEVEL) 瀹氫箟鍩烘湰鐨勪笁娈电増鏈彿锛屼緥濡?"2"銆?4" 鍜?"0"銆傝繖涓変釜鍊煎缁堟槸鏁板瓧銆?

  $(EXTRAVERSION) 涓洪琛ヤ竵鎴栭檮鍔犺ˉ涓佸畾涔変竴涓洿灏忕殑瀛愮骇鍒€傚畠閫氬父鏄煇涓潪鏁板瓧瀛楃涓诧紝渚嬪 "-pre4"锛屽苟涓旂粡甯镐负绌恒€?

KERNELRELEASE
  $(KERNELRELEASE) 鏄竴涓崟瀛楃涓诧紝渚嬪 "2.4.0-pre4"锛岄€傚悎鐢ㄤ簬鏋勯€犲畨瑁呯洰褰曞悕鎴栨樉绀哄湪鐗堟湰瀛楃涓蹭腑銆備竴浜涙灦鏋?Makefile 灏嗗叾鐢ㄤ簬姝ょ洰鐨勩€?

ARCH
  姝ゅ彉閲忓畾涔夌洰鏍囨灦鏋勶紝渚嬪 "i386"銆?arm" 鎴?"sparc"銆備竴浜?kbuild Makefile 浼氭祴璇?$(ARCH) 浠ョ‘瀹氳缂栬瘧鍝簺鏂囦欢銆?

  榛樿鎯呭喌涓嬶紝椤跺眰 Makefile 灏?$(ARCH) 璁剧疆涓轰笌涓绘満绯荤粺鏋舵瀯鐩稿悓銆傚浜庝氦鍙夋瀯寤猴紝鐢ㄦ埛鍙互

```

    make ARCH=m68k ...

```
SRCARCH
  姝ゅ彉閲忔寚瀹?arch/ 涓鏋勫缓鐨勭洰褰曘€?

  ARCH 鍜?SRCARCH 涓嶄竴瀹氬尮閰嶃€傛湁鍑犱釜 arch 鐩綍鏄弻鏋舵瀯锛坆iarch锛夌殑锛屽嵆鍗曚釜 `arch/*/` 鐩綍鍚屾椂鏀寔 32 浣嶅拰 64 浣嶃€?

  渚嬪锛屼綘鍙互浼犲叆 ARCH=i386銆丄RCH=x86_64 鎴?ARCH=x86銆傚瀹冧滑鍏ㄩ儴鑰岃█锛孲RCARCH=x86锛屽洜涓?arch/x86/ 鍚屾椂鏀寔 i386 鍜?x86_64銆?

INSTALL_PATH
  姝ゅ彉閲忎负鏋舵瀯 Makefile 瀹氫箟瀹夎甯搁┗鍐呮牳鏄犲儚鍜?System.map 鏂囦欢鐨勪綅缃€傚皢鍏剁敤浜庢灦鏋勭浉鍏崇殑瀹夎鐩爣銆?

INSTALL_MOD_PATH, MODLIB
  $(INSTALL_MOD_PATH) 涓烘ā鍧楀畨瑁呯殑 $(MODLIB) 鎸囧畾鍓嶇紑銆傝鍙橀噺鏈湪 Makefile 涓畾涔夛紝浣嗗彲鎸夌敤鎴锋剰鎰夸粠鍛戒护琛屼紶鍏ャ€?

  $(MODLIB) 鎸囧畾妯″潡瀹夎鐩綍銆傞《灞?Makefile 灏?$(MODLIB) 瀹氫箟涓?$(INSTALL_MOD_PATH)/lib/modules/$(KERNELRELEASE)銆傜敤鎴峰彲鎸夐渶浠庡懡浠よ瑕嗙洊姝ゅ€笺€?

INSTALL_MOD_STRIP
  濡傛灉鎸囧畾浜嗘鍙橀噺锛屽畠灏嗗鑷存ā鍧楀湪瀹夎鍚庤 strip銆傚鏋?INSTALL_MOD_STRIP 涓?"1"锛屽垯浣跨敤榛樿閫夐」 --strip-debug銆傚惁鍒欙紝INSTALL_MOD_STRIP 鐨勫€煎皢浣滀负 strip 鍛戒护鐨勯€夐」浣跨敤銆?

INSTALL_DTBS_PATH
  姝ゅ彉閲忎负鏋勫缓鏍规墍闇€鐨勯噸瀹氫綅鎸囧畾鍓嶇紑銆傚畠瀹氫箟瀹夎璁惧鏍?blob 鐨勪綅缃€備笌 INSTALL_MOD_PATH 绫讳技锛屽畠鏈湪 Makefile 涓畾涔夛紝浣嗗彲鎸夌敤鎴锋剰鎰夸紶鍏ャ€傚惁鍒欓粯璁や娇鐢ㄥ唴鏍稿畨瑁呰矾寰勩€?

## Makefile language


鍐呮牳 Makefile 琚璁′负浣跨敤 GNU Make 杩愯銆侻akefile 鍙娇鐢?GNU Make 鐨勬枃妗ｅ寲鐗规€э紝浣嗗畠浠‘瀹炰娇鐢ㄤ簡璁稿 GNU 鎵╁睍銆?

GNU Make 鏀寔鍩烘湰鐨勫垪琛ㄥ鐞嗗嚱鏁般€傚唴鏍?Makefile 浣跨敤涓€绉嶆柊棰栫殑鍒楄〃鏋勫缓涓庢搷浣滈鏍硷紝鍑犱箮涓嶄娇鐢?`if` 璇彞銆?

GNU Make 鏈変袱涓祴鍊艰繍绠楃锛宍:=` 鍜?`=`銆俙:=` 瀵瑰彸渚ц繘琛岀珛鍗虫眰鍊硷紝骞跺皢涓€涓疄闄呯殑瀛楃涓插瓨鍏ュ乏渚с€俙=` 绫讳技浜庡叕寮忓畾涔夛紱瀹冨皢鍙充晶浠ユ湭姹傚€肩殑褰㈠紡瀛樺偍锛岀劧鍚庡湪姣忔浣跨敤宸︿晶鏃跺璇ュ舰寮忚繘琛屾眰鍊笺€?

鍦ㄦ煇浜涙儏鍐典笅 `=` 鏄悎閫傜殑銆備笉杩囷紝閫氬父 `:=` 鎵嶆槸姝ｇ‘鐨勯€夋嫨銆?

## Credits


- 鍘熷鐗堟湰鐢?Michael Elizabeth Chastain 鍒朵綔锛?mailto:mec@shout.net>
- 鐢?Kai Germaschewski <kai@tp1.ruhr-uni-bochum.de> 鏇存柊
- 鐢?Sam Ravnborg <sam@ravnborg.org> 鏇存柊
- 璇█璐ㄩ噺妫€鏌ョ敱 Jan Engelhardt <jengelh@gmx.de> 瀹屾垚

## TODO


- 鐢熸垚鍋忕Щ澶存枃浠躲€?
- 鍚戠 7 鎴栫 9 绔犳坊鍔犳洿澶氬彉閲忥紵
