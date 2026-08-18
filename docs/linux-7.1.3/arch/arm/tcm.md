## Linux 涓殑 ARM TCM锛堢揣鑰﹀悎鍐呭瓨锛夊鐞?


Written by Linus Walleij <linus.walleij@stericsson.com>

涓€浜?ARM SoC 鍏锋湁鎵€璋撶殑 TCM锛圱ightly-Coupled Memory锛岀揣鑰﹀悎鍐呭瓨锛夈€?
杩欓€氬父鏄?ARM 澶勭悊鍣ㄥ唴閮ㄤ粎鍑狅紙4-64锛塊iB 鐨?RAM銆?

鐢变簬鍐呭祵浜?CPU 鍐呴儴锛孴CM 鍏锋湁鍝堜經锛圚arvard锛夋灦鏋勶紝鍥犳鏈変竴涓?ITCM锛堟寚浠?TCM锛?
鍜屼竴涓?DTCM锛堟暟鎹?TCM锛夈€侱TCM 涓嶈兘鍖呭惈浠讳綍鎸囦护锛屼絾 ITCM 瀹為檯涓婂彲浠ュ寘鍚暟鎹€?
DTCM 鎴?ITCM 鐨勬渶灏忓昂瀵镐负 4KiB锛屽洜姝ゅ吀鍨嬬殑閰嶇疆鏄?4KiB ITCM 鍜?4KiB DTCM銆?

ARM CPU 鏈変笓闂ㄧ殑瀵勫瓨鍣ㄦ潵璇诲嚭 TCM 鍐呭瓨鐨勭姸鎬併€佺墿鐞嗕綅缃拰澶у皬銆俛rch/arm/include/asm/cputype.h
瀹氫箟浜嗕竴涓?CPUID_TCM 瀵勫瓨鍣紝浣犲彲浠ヤ粠绯荤粺鎺у埗鍗忓鐞嗗櫒涓鍑恒€侫RM 鐨勬枃妗ｅ彲浠ュ湪 http://infocenter.arm.com,
鎵惧埌锛屾悳绱?"TCM Status Register" 鍙煡鐪嬫墍鏈?CPU 鐨勬枃妗ｃ€傝鍙栬瀵勫瓨鍣ㄤ綘鍙互纭畾鏈哄櫒涓?
鏄惁瀛樺湪 ITCM锛堜綅 1-0锛夊拰/鎴?DTCM锛堜綅 17-16锛夈€?

杩樻湁涓€涓?TCM 鍖哄煙瀵勫瓨鍣紙鍦?ARM 绔欑偣鎼滅储 "TCM Region Registers"锛夛紝鍙互鍦ㄨ繍琛屾椂鎶ュ憡骞?
淇敼 TCM 鍐呭瓨鐨勪綅缃拰澶у皬銆傝繖鐢ㄤ簬璇诲嚭鍜屼慨鏀?TCM 鐨勪綅缃笌澶у皬銆傛敞鎰忚繖涓嶆槸 MMU 椤佃〃锛氫綘
瀹為檯涓婃槸鎶?TCM 鐨勭墿鐞嗕綅缃Щ鍔ㄤ簡銆傚湪浣犳斁缃畠鐨勫湴鏂癸紝瀹冧細灞忚斀鎺?CPU 搴曞眰浠讳綍 RAM锛屽洜姝ら€氬父
鏈€濂戒笉瑕佽浠讳綍鐗╃悊 RAM 涓?TCM 閲嶅彔銆?

鐒跺悗鍙互浣跨敤 MMU 鎶?TCM 鍐呭瓨鍐嶆閲嶆槧灏勫埌鍙︿竴涓湴鍧€锛屼絾璇锋敞鎰?TCM 缁忓父鐢ㄤ簬 MMU 琚叧闂殑
鎯呭喌銆備负閬垮厤娣锋穯锛屽綋鍓?Linux 瀹炵幇浼氭妸 TCM 浠庣墿鐞嗗唴瀛樺埌铏氭嫙鍐呭瓨鎸夊唴鏍告寚瀹氱殑浣嶇疆鍋?1 瀵?1
鏄犲皠銆傜洰鍓?Linux 浼氭妸 ITCM 鏄犲皠鍒?0xfffe0000 鍙婁箣鍚庯紝鎶?DTCM 鏄犲皠鍒?0xfffe8000 鍙婁箣鍚庯紝
鏈€澶氭敮鎸?32KiB 鐨?ITCM 鍜?32KiB 鐨?DTCM銆?

鏇存柊鐗堟湰鐨勫尯鍩熷瘎瀛樺櫒杩樻敮鎸佹妸杩欎簺 TCM 鍒嗘垚涓や釜鐙珛鐨?bank锛屼緥濡備竴涓?8KiB 鐨?ITCM 琚垎鎴?
涓や釜 4KiB 鐨?bank锛屽悇鏈夎嚜宸辩殑鎺у埗瀵勫瓨鍣ㄣ€傚叾鎬濊矾鏄兘澶熼攣瀹氬苟闅愯棌鍏朵腑涓€涓?bank 渚涘畨鍏ㄤ笘鐣?
锛圱rustZone锛変娇鐢ㄣ€?

TCM 鐢ㄤ簬浠ヤ笅鍑犳柟闈細

- FIQ 浠ュ強鍏跺畠闇€瑕佺‘瀹氭€ф椂搴忎笖涓嶈兘绛夊緟缂撳瓨鏈懡涓殑涓柇澶勭悊绋嬪簭銆?

- 鎵€鏈夊閮?RAM 閮借繘鍏ヨ嚜鍒锋柊淇濇寔妯″紡鐨勭┖闂插惊鐜紝鍥犳 CPU 鍙兘璁块棶鐗囦笂 RAM锛岀劧鍚庢垜浠?
  鎸傝捣鍦?ITCM 鍐呯瓑寰呬腑鏂€?

- 鍏跺畠鎰忓懗鐫€鍏抽棴鎴栭噸鏂伴厤缃閮?RAM 鎺у埗鍣ㄧ殑鎿嶄綔銆?

鍦?<asm/tcm.h> 涓湁涓€涓敤浜?ARM 鏋舵瀯涓婁娇鐢?TCM 鐨勬帴鍙ｃ€備娇鐢ㄨ鎺ュ彛鍙互锛?

- 瀹氫箟 ITCM 鍜?DTCM 鐨勭墿鐞嗗湴鍧€鍜屽ぇ灏忋€?

- 鏍囪瑕佽缂栬瘧杩?ITCM 鐨勫嚱鏁般€?

- 鏍囪瑕佸垎閰嶅埌 DTCM 鍜?ITCM 鐨勬暟鎹拰甯搁噺銆?

- 鎶婂墿浣欑殑 TCM RAM 閫氳繃 gen_pool_create() 鍜?gen_pool_add() 娣诲姞鍒颁竴涓壒娈婄殑鍒嗛厤姹狅紝
  骞朵负姝ゅ唴瀛樻彁渚?tcm_alloc() 鍜?tcm_free()銆傝繖鏍风殑鍫嗛潪甯搁€傚悎鍦ㄥ叧闂澶囩數婧愬煙鏃朵繚瀛?
  璁惧鐘舵€佷箣绫荤殑浜嬫儏銆?

鎷ユ湁 TCM 鍐呭瓨鐨勬満鍣ㄥ簲褰撲负鑷繁浠?arch/arm/Kconfig 涓€夋嫨 HAVE_TCM銆傞渶瑕佷娇鐢?TCM 鐨勪唬鐮佸簲褰?
#include <asm/tcm.h>

瑕佽繘鍏?itcm 鐨勫嚱鏁板彲浠ヨ繖鏍锋爣璁帮細
int __tcmfunc foo(int bar);

鐢变簬杩欎簺琚爣璁颁负 long_calls锛岃€屼綘鍙兘甯屾湜 TCM 鍐呴儴浠ユ湰鍦版柟寮忚皟鐢ㄥ嚱鏁拌€屼笉娴垂绌洪棿锛屽洜姝?
杩樻湁 __tcmlocalfunc 鍓嶇紑锛屽畠浼氳璋冪敤鍙樹负鐩稿璋冪敤銆?

```

  int __tcmdata foo;

```
```

  int __tcmconst foo;

```
```

  .section ".tcm.text" or .section ".tcm.data"

```
respectively.

```

  #include <asm/tcm.h>

  /* Uninitialized data */
  static u32 __tcmdata tcmvar;
  /* Initialized data */
  static u32 __tcmdata tcmassigned = 0x2BADBABEU;
  /* Constant */
  static const u32 __tcmconst tcmconst = 0xCAFEBABEU;

  static void __tcmlocalfunc tcm_to_tcm(void)
  {
	int i;
	for (i = 0; i < 100; i++)
		tcmvar ++;
  }

  static void __tcmfunc hello_tcm(void)
  {
	/* Some abstract code that runs in ITCM */
	int i;
	for (i = 0; i < 100; i++) {
		tcmvar ++;
	}
	tcm_to_tcm();
  }

  static void __init test_tcm(void)
  {
	u32 *tcmem;
	int i;

	hello_tcm();
	printk("Hello TCM executed from ITCM RAM\n");

	printk("TCM variable from testrun: %u @ %p\n", tcmvar, &tcmvar);
	tcmvar = 0xDEADBEEFU;
	printk("TCM variable: 0x%x @ %p\n", tcmvar, &tcmvar);

	printk("TCM assigned variable: 0x%x @ %p\n", tcmassigned, &tcmassigned);

	printk("TCM constant: 0x%x @ %p\n", tcmconst, &tcmconst);

	/* Allocate some TCM memory from the pool */
	tcmem = tcm_alloc(20);
	if (tcmem) {
		printk("TCM Allocated 20 bytes of TCM @ %p\n", tcmem);
		tcmem[0] = 0xDEADBEEFU;
		tcmem[1] = 0x2BADBABEU;
		tcmem[2] = 0xCAFEBABEU;
		tcmem[3] = 0xDEADBEEFU;
		tcmem[4] = 0x2BADBABEU;
		for (i = 0; i < 5; i++)
			printk("TCM tcmem[%d] = %08x\n", i, tcmem[i]);
		tcm_free(tcmem, 20);
	}
  }

```