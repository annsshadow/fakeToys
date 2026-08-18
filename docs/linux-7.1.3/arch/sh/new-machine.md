
## Adding a new board to LinuxSH


               Paul Mundt <lethal@linux-sh.org>

鏈枃妗ｈ瘯鍥炬杩板湪鏂扮殑 2.5 鍜?2.6 鍐呮牳涓嬶紝涓?LinuxSH 绉绘鐗堟坊鍔犳柊鏉垮崱鏀寔鎵€闇€
鐨勬楠ゃ€傚悓鏃朵篃璇曞浘璇存槑 2.4 涓?2.5/2.6 SH 鍚庣涔嬮棿涓€浜涙樉钁楃殑鍙樺寲銆?

## 1. New Directory Structure


棣栧厛瑕佹敞鎰忕殑鏄柊鐨勭洰褰曠粨鏋勩€傚湪 2.4 涓嬶紝缁濆ぇ澶氭暟鏉垮崱鐩稿叧鐨勪唬鐮侊紙stboards 闄ゅ锛?
鏈€缁堥兘鐩存帴鏀惧湪 arch/sh/kernel/ 涓紝鏉垮崱鐩稿叧鐨勫ご鏂囦欢鍒欐斁鍦?include/asm-sh/ 涓€?
鍦ㄦ柊鍐呮牳涓紝浠ｇ爜鎸夋澘鍗＄被鍨嬨€侀厤濂楄姱鐗囩被鍨嬩互鍙?CPU 绫诲瀷鎷嗗垎銆備粠璇ョ洰褰曞眰绾х殑鏍戠姸
瑙嗗浘鏉ョ湅锛屽ぇ鑷村涓嬶細

```

    .
    |-- arch
    |   `-- sh
    |       `-- boards
    |           |-- adx
    |           |   `-- board-specific files
    |           |-- bigsur
    |           |   `-- board-specific files
    |           |
    |           ... more boards here ...
    |
    `-- include
	`-- asm-sh
	    |-- adx
	    |   `-- board-specific headers
	    |-- bigsur
	    |   `-- board-specific headers
	    |
	    .. more boards here ...

```
```

    .
    `-- arch
	`-- sh
	    `-- cchips
		`-- hd6446x
		    `-- hd64461
			`-- cchip-specific files

```
鈥︹€︿互姝ょ被鎺ㄣ€傞厤濂楄姱鐗囩殑澶存枃浠朵笌鏉垮崱鐗瑰畾鐨勫ご鏂囦欢澶勭悊鏂瑰紡鐩稿悓銆傚洜姝わ紝include/asm-sh/hd64461
瀛樻斁浜嗘墍鏈?hd64461 鐗瑰畾鐨勫ご鏂囦欢銆?

```

    .
    |-- arch
    |   `-- sh
    |       |-- kernel
    |       |   `-- cpu
    |       |       |-- sh2
    |       |       |   `-- SH-2 generic files
    |       |       |-- sh3
    |       |       |   `-- SH-3 generic files
    |       |       `-- sh4
    |       |           `-- SH-4 generic files
    |       `-- mm
    |           `-- This is also broken out per CPU family, so each family can
    |               have their own set of cache/tlb functions.
    |
    `-- include
	`-- asm-sh
	    |-- cpu-sh2
	    |   `-- SH-2 specific headers
	    |-- cpu-sh3
	    |   `-- SH-3 specific headers
	    `-- cpu-sh4
		`-- SH-4 specific headers

```
搴斿綋娉ㄦ剰锛孋PU 瀛愮被鍨嬪苟_涓峗鍋氭娊璞°€傚洜姝わ紝杩欎簺浠嶉渶鐢?CPU 绯诲垪鐩稿叧鐨勪唬鐮佹潵澶勭悊銆?

## 2. Adding a New Board


棣栧厛瑕佺‘瀹氱殑鏄紝浣犳鍦ㄦ坊鍔犵殑鏉垮崱鏄嫭绔嬬殑锛岃繕鏄睘浜庝竴涓澘鍗″鏃忊€斺€旇瀹舵棌涓?
鍚勬垚鍛樺樊鍒緢灏忥紝澶у鍙互鍏变韩鐩稿悓鐨勬澘鍗＄壒瀹氫唬鐮併€?

鍦ㄧ涓€绉嶆儏鍐典笅锛屽彧闇€鍦?arch/sh/boards/ 涓嬩负浣犵殑鏉垮崱寤轰竴涓洰褰曪紝骞舵坊鍔犺鍒欏皢浣犵殑
鏉垮崱鎸傛帴鍒版瀯寤虹郴缁燂紙涓嬩竴鑺傝杩帮級銆備絾瀵逛簬鏉垮崱瀹舵棌锛屾洿鍚堢悊鐨勫仛娉曟槸鍦?arch/sh/boards/
涓嬪缓绔嬩竴涓叕鍏辩殑椤跺眰鐩綍锛岀劧鍚庡湪璇ョ洰褰曚笅涓烘瘡涓鏃忔垚鍛樺缓绔嬪瓙鐩綍銆係olution Engine
鍜?hp6xx 鏉垮崱閮芥槸杩欑渚嬪瓙銆?

璁剧疆濂芥柊鐨?arch/sh/boards/ 鐩綍鍚庯紝璇疯浣忎綘杩樺簲鍦?include/asm-sh 涓嬫坊鍔犱竴涓笓灞炰簬
璇ユ澘鍗＄殑鐩綍锛堝鏋滀細鏈夊涓殑璇濓級銆備负浜嗚兘涓庢瀯寤虹郴缁熸棤缂濆崗浣滐紝鏈€濂借璇ョ洰褰曞悕涓?
arch/sh/boards/ 鐨勭洰褰曞悕鐩稿悓锛涗笉杩囧鏋滀綘鐨勬澘鍗″張灞炰簬鏌愪釜瀹舵棌锛屾瀯寤虹郴缁熸湁鍔炴硶澶勭悊
杩欑鎯呭喌锛堥€氳繃 incdir-y 閲嶈浇锛夛紝浣犱篃鍙互鑷敱鍦版寜瀹舵棌鎴愬憳鏈韩鏉ュ懡鍚嶇洰褰曘€?

姣忎釜鏉垮崱鍦?arch/sh/boards 鍜?include/asm-sh/ 灞傜骇涓嬮兘闇€瑕佸叿澶囦竴浜涜绱犮€備负浜嗘洿濂藉湴
璇存槑锛屾垜浠互娣诲姞涓€涓櫄鎷熸澘鍗′负渚嬨€傚浜庡垵濮嬪寲浠ｇ爜锛屾垜浠嚦灏戝繀椤绘彁渚?get_system_type()
鍜?platform_setup() 鐨勫畾涔夈€傚浜庢垜浠殑铏氭嫙鏉垮崱锛岃繖
```

    /*
    * arch/sh/boards/vapor/setup.c - Setup code for imaginary board
    */
    #include <linux/init.h>

    const char *get_system_type(void)
    {
	    return "FooTech Vaporboard";
    }

    int __init platform_setup(void)
    {
	    /*
	    * If our hardware actually existed, we would do real
	    * setup here. Though it's also sane to leave this empty
	    * if there's no real init work that has to be done for
	    * this board.
	    */

	    /* Start-up imaginary PCI ... */

	    /* And whatever else ... */

	    return 0;
    }

```
鎴戜滑鏂扮殑铏氭嫙鏉垮崱杩樺繀椤诲湪 machvec 涓寕鎺ワ紝鎵嶈兘鍙戞尌浣滅敤銆?

machvec 鍑芥暟鍒嗕负鑻ュ共绫伙細

 - 璁块棶 IO 鍐呭瓨锛坕nb 绛夛級鍜?PCI/涓诲唴瀛橈紙readb 绛夛級鐨?I/O 鍑芥暟銆?
 - I/O 鏄犲皠鍑芥暟锛坕oport_map銆乮oport_unmap 绛夛級銆?
 - 涓€涓€滃績璺斥€濓紙heartbeat锛夊嚱鏁般€?
 - PCI 鍜?IRQ 鍒濆鍖栦緥绋嬨€?
 - 涓€鑷存€у垎閰嶅櫒锛坈onsistent allocator锛岄拡瀵归渶瑕佺壒娈婂垎閰嶅櫒銆佸挨鍏舵槸瑕佷粠鏌愪簺鏉垮崱
   鐗瑰畾鐨?SRAM 涓负 DMA handle 鍒嗛厤鍐呭瓨鐨勬澘鍗★級銆?

machvec 鍑芥暟浼氶殢鏃堕棿涓嶆柇澧炲噺锛屽洜姝よ鍔″繀鏌ラ槄 include/asm-sh/machvec.h 浠ヤ簡瑙?
machvec 鐨勫綋鍓嶇姸鎬併€?

鍐呮牳浼氬湪鍚姩鏃惰嚜鍔ㄤ负 machvec 涓湭瀹氫箟鐨勫嚱鏁版寚閽堝鐢ㄩ€氱敤渚嬬▼锛屽洜涓?machvec 鍑芥暟鍦?
鍐呮牳鏍戠殑澶ч儴鍒嗗湴鏂归兘鏄棤鏉′欢寮曠敤鐨勩€傛湁浜涙澘鍗＄殑 machvec 鏋佷负绮剧畝锛堝 dreamcast 鍜?
sh03锛夛紝鑰屽彟涓€浜涘垯蹇呴』瀹氫箟鍑犱箮鍏ㄩ儴锛坮ts7751r2d锛夈€?

娣诲姞涓€涓柊鏈哄櫒鐩稿綋绠€鍗曪紙浠?vapor 涓轰緥锛夛細

濡傛灉鏉垮崱鐗瑰畾鐨勫畾涔夐潪甯哥簿绠€锛堢粷澶у鏁版澘鍗￠兘鏄繖绉嶆儏鍐碉級锛岄偅涔堝彧闇€涓€涓崟鐙殑鏉垮崱
鐗瑰畾澶存枃浠跺氨瓒冲浜嗐€?

 - 娣诲姞涓€涓柊鏂囦欢 include/asm-sh/vapor.h锛屽叾涓寘鍚互鏈哄櫒鍚嶄綔涓哄墠缂€鐨勩€佹墍鏈夋満鍣?
   鐗瑰畾 IO 鍑芥暟鐨勫師鍨嬶紝渚嬪 vapor_inb銆傚湪濉啓鏈哄櫒鍚戦噺锛坢achine vector锛夋椂浼氱敤鍒板畠浠€?

   娉ㄦ剰锛岃繖浜涘師鍨嬮€氳繃璁剧疆
```

	#define __IO_PREFIX vapor
	#include <asm/io_generic.h>

   somewhere in the board-specific header. Any boards being ported that still
   have a legacy io.h should remove it entirely and switch to the new model.

 - Add machine vector definitions to the board's setup.c. At a bare minimum,
   this must be defined as something like::

	struct sh_machine_vector mv_vapor __initmv = {
		.mv_name = "vapor",
	};
	ALIAS_MV(vapor)

 - finally add a file arch/sh/boards/vapor/io.c, which contains definitions of
   the machine specific io functions (if there are enough to warrant it).

```
## 3. Hooking into the Build System


鐜板湪鐩綍閮藉凡寤虹珛锛屾墍鏈夋澘鍗＄壒瀹氫唬鐮佷篃宸插氨浣嶏紝鏄椂鍊欑湅鐪嬪浣曡杩欎竴鍥笢瑗胯瀺鍏?
鏋勫缓绯荤粺浜嗐€?

鏋勫缓绯荤粺鐨勫緢澶ч儴鍒嗙幇鍦ㄦ槸瀹屽叏鍔ㄦ€佺殑锛屽彧闇€瑕佸湪鍚勫鍔犲叆鍚堥€傜殑鏉＄洰鍗冲彲瀹屾垚宸ヤ綔銆?

棣栧厛瑕佸仛鐨勬槸鍦?arch/sh/Kconfig 鐨?
```

    config SH_VAPOR
	    bool "Vapor"
	    help
	    select Vapor if configuring for a FooTech Vaporboard.

```
鎺ヤ笅鏉ワ紝蹇呴』鎶婂畠鍔犲叆 arch/sh/Makefile銆傛墍鏈夋澘鍗￠兘闇€瑕佷竴涓?machdir-y 鏉＄洰鎵嶈兘琚?
鏋勫缓銆傝鏉＄洰蹇呴』鏄澘鍗＄洰褰曞湪 arch/sh/boards 涓嚭鐜扮殑鍚嶇О锛屽嵆浣垮畠浣嶄簬瀛愮洰褰曚腑锛堝湪
閭ｇ鎯呭喌涓嬶紝arch/sh/boards/ 浠ヤ笅鐨勬墍鏈夌埗鐩綍
```

    machdir-$(CONFIG_SH_VAPOR)	+= vapor

```
鍓嶆彁鏄垜浠凡缁忔妸鎵€鏈夊唴瀹规斁鍦?arch/sh/boards/vapor/ 鐩綍涓€?

鎺ヤ笅鏉ワ紝鏋勫缓绯荤粺鍋囧畾浣犵殑 include/asm-sh 鐩綍涔熶細浣跨敤鐩稿悓鐨勫悕瀛椼€傚鏋滀笉鏄紙渚嬪
灞炰簬鍚屼竴鍏叡瀹舵棌鐨勫涓澘鍗★級锛屽垯闇€瑕佸皢璇ョ洰褰曞悕闅愬紡杩藉姞鍒?incdir-y銆傜幇鏈変唬鐮佸凡缁?
涓?Solution Engine 鍜?hp6xx 鏉垮崱澶勭悊浜嗚繖涓€鐐癸紝鍙弬鑰冭繖浜涗緥瀛愩€?

澶勭悊濂戒箣鍚庯紝灏卞埌浜嗕负 mach 绫诲瀷娣诲姞鏉＄洰鐨勭幆鑺傘€傝繖閫氳繃鍚?arch/sh/tools/mach-types
鍒楄〃鏈熬娣诲姞鏉＄洰鏉ュ畬鎴愩€傚仛娉曚笉瑷€鑷槑锛屾澶勪笉鍐嶈禈杩般€傚畬鎴愬悗锛屽鏋滀綘鍦ㄦ暣涓?
```

	/* Make sure we're on the FooTech Vaporboard */
	if (!mach_is_vapor())
		return -ENODEV;

```
杩樿娉ㄦ剰锛宮ach_is_boardname() 妫€鏌ヤ細琚殣寮忓己鍒惰浆涓哄皬鍐欙紝灏界 mach-types 鏉＄洰鍏ㄩ儴
鏄ぇ鍐欍€傚鏋滀綘鐪熺殑寰堝湪鎰忓彲浠ヨ閭ｄ釜鑴氭湰锛屼絾瀹冪浉褰撲笐闄嬶紝鎵€浠ヤ綘鍙兘骞朵笉鎯宠繖涔堝仛銆?

鐜板湪鍓╀笅鐨勫氨鏄负浣犵殑鏂版澘鍗℃彁渚涗竴涓?defconfig銆傝繖鏍凤紝鏈€缁堟嬁鍒拌繖鍧楁澘鍗＄殑鍏朵粬浜?
灏卞彲浠ョ洿鎺ュ弬鑰冭閰嶇疆锛岃€屼笉蹇呭幓鐚滄祴搴斿綋浣跨敤鍝簺璁剧疆銆?

鍙﹀锛屼竴鏃︿綘涓烘柊鏉垮崱澶嶅埗浜嗕竴涓ず渚?.config锛堝亣璁句负 arch/sh/configs/vapor_defconfig锛夛紝
浣犱篃鍙互鐩存帴灏嗗畠浣滀负涓€涓瀯寤虹洰鏍囦娇鐢紝瀹冧細琚殣寮忓湴鍒楀湪 help 鏂囨湰涓€?

鏌ョ湅 'make help' 鐨勮緭鍑猴紝浣犵幇鍦ㄥ簲璇ヤ細鐪嬪埌绫讳技濡備笅鍐呭锛?

Architecture specific targets (sh)锛?

  =======================   =============================================
  zImage                    Compressed kernel image (arch/sh/boot/zImage)
  adx_defconfig             Build for adx
  cqreek_defconfig          Build for cqreek
  dreamcast_defconfig       Build for dreamcast
  ...
  vapor_defconfig           Build for vapor
  =======================   =============================================

```

    $ make ARCH=sh CROSS_COMPILE=sh4-linux- vapor_defconfig vmlinux

```
瀹冧細杩涜€屽鍒惰鏉垮崱鐨?defconfig锛岀敤 oldconfig 璺戜竴閬嶏紙鑷垱寤轰互鏉ヨ嫢鏈夋柊閫夐」浼氭彁绀轰綘
纭锛夛紝鐒跺悗甯︿綘韪忎笂涓烘柊鏉垮崱鏋勫缓涓€涓彲鐢ㄥ唴鏍哥殑寰佺▼銆?
