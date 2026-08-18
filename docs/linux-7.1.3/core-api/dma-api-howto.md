## 鍔ㄦ€?DMA 鏄犲皠鎸囧崡


:Author: David S. Miller <davem@redhat.com>
:Author: Richard Henderson <rth@cygnus.com>
:Author: Jakub Jelinek <jakub@redhat.com>

鏈寚鍗楅潰鍚戣澶囬┍鍔ㄥ紑鍙戣€咃紝浠嬬粛濡備綍浣跨敤 DMA API锛屽苟闄勬湁浼唬鐮佺ず渚嬨€傚叧浜?API 鐨?绠€鏄庤鏄庯紝璇峰弬瑙?Documentation/core-api/dma-api.rst銆?
## CPU 鍦板潃涓?DMA 鍦板潃


DMA API 涓秹鍙婂嚑绉嶄笉鍚岀殑鍦板潃锛岀悊瑙ｅ畠浠殑鍖哄埆闈炲父閲嶈銆?
鍐呮牳閫氬父浣跨敤铏氭嫙鍦板潃銆俴malloc()銆乿malloc() 浠ュ強绫讳技鎺ュ彛杩斿洖鐨勪换浣曞湴鍧€閮芥槸铏氭嫙鍦板潃锛?鍙互淇濆瓨鍦?`void *` 涓€?
铏氭嫙鍐呭瓨绯荤粺锛圱LB銆侀〉琛ㄧ瓑锛夊皢铏氭嫙鍦板潃杞崲涓?CPU 鐗╃悊鍦板潃锛岀墿鐞嗗湴鍧€浠?"phys_addr_t" 鎴?"resource_size_t" 褰㈠紡瀛樺偍銆傚唴鏍稿皢瀵勫瓨鍣ㄧ瓑璁惧璧勬簮褰撲綔鐗╃悊鍦板潃
绠＄悊銆傝繖浜涘氨鏄?/proc/iomem 涓殑鍦板潃銆傜墿鐞嗗湴鍧€瀵归┍鍔ㄥ苟涓嶇洿鎺ユ湁鐢紱椹卞姩蹇呴』浣跨敤
ioremap() 鏉ユ槧灏勮绌洪棿骞跺緱鍒颁竴涓櫄鎷熷湴鍧€銆?
I/O 璁惧浣跨敤绗笁绉嶅湴鍧€锛氣€滄€荤嚎鍦板潃鈥濓紙bus address锛夈€傚鏋滆澶囧湪鏌愪釜 MMIO 鍦板潃涓婃嫢鏈?瀵勫瓨鍣紝鎴栬€呭畠鎵ц DMA 鏉ヨ鍐欑郴缁熷唴瀛橈紝璁惧鎵€浣跨敤鐨勫湴鍧€灏辨槸鎬荤嚎鍦板潃銆傚湪鏌愪簺绯荤粺涓紝
鎬荤嚎鍦板潃涓?CPU 鐗╃悊鍦板潃瀹屽叏鐩稿悓锛屼絾涓€鑸儏鍐典笅骞堕潪濡傛銆侷OMMU 鍜屼富鏈烘ˉ锛坔ost bridge锛?鍙互鍦ㄧ墿鐞嗗湴鍧€鍜屾€荤嚎鍦板潃涔嬮棿寤虹珛浠绘剰鏄犲皠銆?
浠庤澶囩殑瑙掑害鐪嬶紝DMA 浣跨敤鐨勬槸鎬荤嚎鍦板潃绌洪棿锛屼絾鍙兘浠呴檺浜庤绌洪棿鐨勪竴涓瓙闆嗐€備緥濡傦紝鍗充娇
涓€涓郴缁熸敮鎸?64 浣嶇殑涓诲瓨鍦板潃鍜?PCI BAR锛屽畠涔熷彲鑳戒娇鐢?IOMMU锛屼娇寰楄澶囧彧闇€浣跨敤 32 浣?DMA 鍦板潃銆?
```

               CPU                  CPU                  Bus
             Virtual              Physical             Address
             Address              Address               Space
              Space                Space

            +-------+             +------+             +------+
            |       |             |MMIO  |   Offset    |      |
            |       |  Virtual    |Space |   applied   |      |
          C +-------+ --------> B +------+ ----------> +------+ A
            |       |  mapping    |      |   by host   |      |
  +-----+   |       |             |      |   bridge    |      |   +--------+
  |     |   |       |             +------+             |      |   |        |
  | CPU |   |       |             | RAM  |             |      |   | Device |
  |     |   |       |             |      |             |      |   |        |
  +-----+   +-------+             +------+             +------+   +--------+
            |       |  Virtual    |Buffer|   Mapping   |      |
          X +-------+ --------> Y +------+ <---------- +------+ Z
            |       |  mapping    | RAM  |   by IOMMU
            |       |             |      |
            |       |             |      |
            +-------+             +------+

```
鍦ㄦ灇涓捐繃绋嬩腑锛屽唴鏍镐細浜嗚В鍒?I/O 璁惧鍙婂叾 MMIO 绌洪棿锛屼互鍙婂皢璁惧杩炴帴鍒扮郴缁熺殑涓绘満妗ャ€?渚嬪锛屽鏋滀竴涓?PCI 璁惧鏈変竴涓?BAR锛屽唴鏍镐細浠?BAR 涓鍙栨€荤嚎鍦板潃锛圓锛夊苟灏嗗叾杞崲涓?CPU
鐗╃悊鍦板潃锛圔锛夈€傚湴鍧€ B 淇濆瓨鍦ㄤ竴涓?struct resource 涓紝閫氬父閫氳繃 /proc/iomem 鏆撮湶銆傚綋
椹卞姩璁ら涓€涓澶囨椂锛屽畠閫氬父浣跨敤 ioremap() 灏嗙墿鐞嗗湴鍧€ B 鏄犲皠鍒版煇涓櫄鎷熷湴鍧€锛圕锛夈€傜劧鍚?瀹冨氨鍙互浣跨敤渚嬪 ioread32(C) 鏉ヨ闂€荤嚎鍦板潃 A 澶勭殑璁惧瀵勫瓨鍣ㄣ€?
濡傛灉璁惧鏀寔 DMA锛岄┍鍔ㄤ娇鐢?kmalloc() 鎴栫被浼兼帴鍙ｅ缓绔嬩竴鍧楃紦鍐插尯锛岃鎺ュ彛杩斿洖涓€涓櫄鎷?鍦板潃锛圶锛夈€傝櫄鎷熷唴瀛樼郴缁熷皢 X 鏄犲皠鍒扮郴缁?RAM 涓殑鏌愪釜鐗╃悊鍦板潃锛圷锛夈€傞┍鍔ㄥ彲浠ヤ娇鐢ㄨ櫄鎷?鍦板潃 X 鏉ヨ闂缂撳啿鍖猴紝浣嗚澶囨湰韬笉鑳斤紝鍥犱负 DMA 涓嶇粡杩?CPU 鐨勮櫄鎷熷唴瀛樼郴缁熴€?
鍦ㄦ煇浜涚畝鍗曠郴缁熶腑锛岃澶囧彲浠ョ洿鎺ュ鐗╃悊鍦板潃 Y 鍋?DMA銆備絾鍦ㄨ澶氬叾浠栫郴缁熶腑锛屾湁 IOMMU 纭欢
灏?DMA 鍦板潃杞崲涓虹墿鐞嗗湴鍧€锛屼緥濡傚皢 Z 杞崲涓?Y銆傝繖姝ｆ槸闇€瑕?DMA API 鐨勯儴鍒嗗師鍥狅細椹卞姩鍙互
鎶婁竴涓櫄鎷熷湴鍧€ X 浜ょ粰鍍?dma_map_single() 杩欐牱鐨勬帴鍙ｏ紝鐢卞畠寤虹珛浠讳綍蹇呴渶鐨?IOMMU 鏄犲皠骞?杩斿洖 DMA 鍦板潃 Z銆傜劧鍚庨┍鍔ㄥ憡鐭ヨ澶囧 Z 鍋?DMA锛孖OMMU 鍐嶅皢鍏舵槧灏勫埌绯荤粺 RAM 涓湴鍧€ Y 澶勭殑
缂撳啿鍖恒€?
涓轰簡璁?Linux 鑳藉浣跨敤鍔ㄦ€?DMA 鏄犲皠锛屽畠闇€瑕侀┍鍔ㄦ彁渚涗竴浜涘府鍔╋紝鍗冲繀椤昏€冭檻鍒?DMA 鍦板潃
鍙簲鍦ㄥ疄闄呬娇鐢ㄦ湡闂磋鏄犲皠锛屽苟鍦?DMA 浼犺緭瀹屾垚鍚庤鍙栨秷鏄犲皠銆?
褰撶劧锛屽嵆浣垮湪涓嶅瓨鍦ㄦ绫荤‖浠剁殑骞冲彴涓婏紝涓嬮潰鐨?API 涔熻兘宸ヤ綔銆?
娉ㄦ剰锛孌MA API 閫傜敤浜庝换浣曟€荤嚎锛岃€屼笌搴曞眰鐨勫井澶勭悊鍣ㄦ灦鏋勬棤鍏炽€備綘搴旇浣跨敤 DMA API锛岃€屼笉鏄?鎬荤嚎鐗瑰畾鐨?DMA API锛屼篃灏辨槸璇达紝浣跨敤 dma_map_*() 鎺ュ彛锛岃€屼笉鏄?pci_map_*() 鎺ュ彛銆?
```

	#include <linux/dma-mapping.h>

```
鍑虹幇鍦ㄤ綘鐨勯┍鍔ㄤ腑锛屽畠鎻愪緵浜?dma_addr_t 鐨勫畾涔夈€傝绫诲瀷鍙互淇濆瓨骞冲彴涓婁换浣曟湁鏁堢殑 DMA
鍦板潃锛屽嚒鏄淇濆瓨浠?DMA 鏄犲皠鍑芥暟杩斿洖鐨?DMA 鍦板潃鏃讹紝閮藉簲浣跨敤姝ょ被鍨嬨€?
## 鍝簺鍐呭瓨鍙仛 DMA


浣犲繀椤荤煡閬撶殑绗竴浠朵簨鏄紝鍝簺鍐呮牳鍐呭瓨鍙互涓?DMA 鏄犲皠璁炬柦涓€璧蜂娇鐢ㄣ€傚叧浜庤繖涓€鐐逛竴鐩存湁涓€濂?涓嶆垚鏂囩殑瑙勫垯锛屾湰鏂囪瘯鍥炬渶缁堟妸瀹冧滑鍐欎笅鏉ャ€?
濡傛灉浣犻€氳繃椤靛垎閰嶅櫒锛堝嵆 __get_free_page*()锛夋垨閫氱敤鍐呭瓨鍒嗛厤鍣紙鍗?kmalloc() 鎴?kmem_cache_alloc()锛夎幏寰楀唴瀛橈紝閭ｄ箞浣犲氨鍙互浣跨敤杩欎簺渚嬬▼杩斿洖鐨勫湴鍧€瀵硅鍐呭瓨杩涜 DMA
璇诲啓銆?
杩欏叿浣撴剰鍛崇潃浣燺涓嶈兘_浣跨敤 vmalloc() 杩斿洖鐨勫唴瀛?鍦板潃鏉ュ仛 DMA銆傚彲浠ュ鏄犲皠鍒?vmalloc() 鍖哄煙
鐨刜搴曞眰_鍐呭瓨鍋?DMA锛屼絾杩欓渶瑕侀亶鍘嗛〉琛ㄤ互鑾峰緱鐗╃悊鍦板潃锛岀劧鍚庣敤 __va() 涔嬬被鐨勫嚱鏁版妸姣忎釜
椤靛啀杞崲鍥炲唴鏍稿湴鍧€銆俒娉細寰呮垜浠泦鎴?Gerd Knorr 瀹炵幇姝ゅ姛鑳界殑閫氱敤浠ｇ爜鏃跺啀鏇存柊姝ゅ銆俔

杩欐潯瑙勫垯杩樻剰鍛崇潃锛屼綘鏃笉鑳戒娇鐢ㄥ唴鏍告槧鍍忓湴鍧€锛坉ata/text/bss 娈典腑鐨勯」锛夛紝涔熶笉鑳戒娇鐢ㄦā鍧?鏄犲儚鍦板潃鎴栨爤鍦板潃鏉ュ仛 DMA銆傝繖浜涢兘鍙兘琚槧灏勫埌涓庣墿鐞嗗唴瀛樺叾浣欓儴鍒嗗畬鍏ㄤ笉鍚岀殑鍦版柟銆傚嵆浣?杩欎簺绫诲埆鐨勫唴瀛樺湪鐗╃悊涓婂彲浠ヤ笌 DMA 閰嶅悎宸ヤ綔锛屼綘涔熼渶瑕佺‘淇?I/O 缂撳啿鍖烘槸鎸夌紦瀛樿瀵归綈鐨勩€?鍚﹀垯锛屽湪鍏锋湁 DMA 涓嶄竴鑷达紙DMA-incoherent锛夌紦瀛樼殑 CPU 涓婏紝浣犱細閬囧埌缂撳瓨琛屽叡浜棶棰橈紙鏁版嵁
鎹熷潖锛夈€傦紙CPU 鍙兘鍐欎竴涓瓧锛孌MA 鍐欏悓涓€涓紦瀛樿涓殑鍙︿竴涓瓧锛屽叾涓竴涓彲鑳借瑕嗙洊銆傦級

鍚屾牱锛岃繖鎰忓懗鐫€浣犱笉鑳芥嬁 kmap() 璋冪敤鐨勮繑鍥炲€煎幓鍋?DMA 璇诲啓銆傝繖涓?vmalloc() 绫讳技銆?
鍧?I/O 鍜岀綉缁滅紦鍐插尯鍛紵鍧?I/O 鍜岀綉缁滃瓙绯荤粺浼氱‘淇濆畠浠娇鐢ㄧ殑缂撳啿鍖哄 DMA 璇诲啓鏄湁鏁堢殑銆?
## __dma_from_device_group_begin/end 娉ㄨВ


濡傚墠鎵€杩帮紝褰撲竴涓粨鏋勪綋鍖呭惈涓€涓?DMA_FROM_DEVICE / DMA_BIDIRECTIONAL 缂撳啿鍖猴紙璁惧鍐欏叆
鍐呭瓨锛変互鍙?CPU 鍐欏叆鐨勫瓧娈垫椂锛孌MA 缂撳啿鍖轰笌 CPU 鍐欏叆瀛楁涔嬮棿鐨勭紦瀛樿鍏变韩锛屼細鍦ㄥ叿鏈?DMA 涓嶄竴鑷寸紦瀛樼殑 CPU 涓婂鑷存暟鎹崯鍧忋€?
`__dma_from_device_group_begin(GROUP)/__dma_from_device_group_end(GROUP)`
```

	struct my_device {
		spinlock_t lock1;
		__dma_from_device_group_begin();
		char dma_buffer1[16];
		char dma_buffer2[16];
		__dma_from_device_group_end();
		spinlock_t lock2;
	};

```
涓轰簡灏?DMA 缂撳啿鍖轰笌鐩搁偦瀛楁闅旂寮€鏉ワ紝璇峰湪绗竴涓?DMA 缂撳啿鍖哄瓧娈典箣鍓嶄娇鐢?`__dma_from_device_group_begin(GROUP)`锛屽湪鏈€鍚庝竴涓?DMA 缂撳啿鍖哄瓧娈典箣鍚庝娇鐢?`__dma_from_device_group_end(GROUP)`锛堜娇鐢ㄧ浉鍚岀殑 GROUP 鍚嶇О锛夈€傝繖浼氫繚鎶ょ紦鍐插尯鐨勫ご閮ㄥ拰
灏鹃儴閮戒笉鍙楃紦瀛樿鍏变韩鐨勫奖鍝嶃€?
GROUP 鍙傛暟鏄竴涓彲閫夌殑鏍囪瘑绗︼紝鐢ㄤ簬鍛藉悕 DMA 缂撳啿鍖虹粍
```

	struct my_device {
		spinlock_t lock1;
		__dma_from_device_group_begin(buffer1);
		char dma_buffer1[16];
		__dma_from_device_group_end(buffer1);
		spinlock_t lock2;
		__dma_from_device_group_begin(buffer2);
		char dma_buffer2[16];
		__dma_from_device_group_end(buffer2);
	};

```
鍦ㄧ紦瀛樹竴鑷达紙cache-coherent锛夌殑骞冲彴涓婏紝杩欎簺瀹忎細灞曞紑涓洪浂闀垮害鏁扮粍鏍囪銆傚湪闈炰竴鑷村钩鍙颁笂锛?瀹冧滑杩樹細纭繚鏈€灏忕殑 DMA 瀵归綈锛屾渶澶у彲鑳借揪鍒?128 瀛楄妭銆?

        鍏佽锛堝敖绠℃湁浜涜剢寮憋級鍦ㄧ粍鍐呭寘鍚笉鎵撶畻渚涜澶囧仛 DMA 鐨勯澶栧瓧娈碉紙浠ヤ究璁╃粨鏋勪綋
        绱у噾鎺掑垪锛夆€斺€斾絾鍓嶆彁鏄紝鍙缁勫唴鐨勪换浣曞瓧娈佃鏄犲皠涓?DMA_FROM_DEVICE 鎴?        DMA_BIDIRECTIONAL锛孋PU 灏变笉寰楀啓鍏ヨ繖浜涘瓧娈点€?
## DMA 瀵诲潃鑳藉姏


榛樿鎯呭喌涓嬶紝鍐呮牳鍋囪浣犵殑璁惧鑳藉杩涜 32 浣嶇殑 DMA 瀵诲潃銆傚浜庢敮鎸?64 浣嶇殑璁惧锛岄渶瑕?鎻愰珮杩欎釜鍊硷紱瀵逛簬鏈夐檺鍒剁殑璁惧锛岄渶瑕侀檷浣庤繖涓€笺€?
鍏充簬 PCI 鐨勭壒鍒鏄庯細PCI-X 瑙勮寖瑙勫畾 PCI-X 璁惧蹇呴』鏀寔瀵规墍鏈変簨鍔＄殑 64 浣嶅鍧€锛圖AC锛夈€?骞朵笖鑷冲皯鏈変竴涓钩鍙帮紙SGI SN2锛夎姹傚湪 IO 鎬荤嚎澶勪簬 PCI-X 妯″紡鏃讹紝浣跨敤 64 浣嶄竴鑷存€у垎閰嶆墠鑳?姝ｅ父宸ヤ綔銆?
涓轰簡姝ｇ‘杩愯锛屼綘蹇呴』璁剧疆 DMA 鎺╃爜锛坢ask锛夋潵鍛婄煡鍐呮牳浣犵殑璁惧鐨?DMA 瀵诲潃鑳藉姏銆?
```

	int dma_set_mask_and_coherent(struct device *dev, u64 mask);

```
璇ュ嚱鏁颁細鍚屾椂涓烘祦寮忥紙streaming锛夊拰涓€鑷存€э紙coherent锛堿PI 璁剧疆鎺╃爜銆傚鏋滀綘鏈変竴浜涚壒娈?闇€姹傦紝鍙互鏀圭敤涓嬮潰涓や釜鐙珛鐨勮皟鐢細

	娴佸紡鏄犲皠鐨勮缃€氳繃涓€涓濡備笅鍑芥暟鐨勮皟鐢ㄦ潵瀹屾垚
```

		int dma_set_mask(struct device *dev, u64 mask);

	涓€鑷存€у垎閰嶇殑璁剧疆閫氳繃璋冪敤 dma_set_coherent_mask() 鏉ュ畬鎴?:

		int dma_set_coherent_mask(struct device *dev, u64 mask);

```
杩欓噷锛宒ev 鏄寚鍚戜綘鐨勮澶囩殑 device 缁撴瀯浣撶殑鎸囬拡锛宮ask 鏄竴涓綅鎺╃爜锛屾弿杩颁綘鐨勮澶囨敮鎸?鍦板潃鐨勫摢浜涗綅銆傞€氬父锛屼綘鐨勮澶囩殑 device 缁撴瀯浣撳祵鍏ュ湪瀹冪殑鎬荤嚎鐗瑰畾鐨?device 缁撴瀯浣撲腑銆?渚嬪锛?pdev->dev 鏄寚鍚?PCI 璁惧鐨?device 缁撴瀯浣撶殑鎸囬拡锛坧dev 鏄寚鍚戜綘鐨勮澶囩殑 PCI
device 缁撴瀯浣撶殑鎸囬拡锛夈€?
杩欎簺璋冪敤閫氬父杩斿洖闆讹紝琛ㄧず鍦ㄧ粰瀹氱殑鍦板潃鎺╃爜涓嬶紝浣犵殑璁惧鍙互鍦ㄨ鏈哄櫒涓婃甯告墽琛?DMA锛涗絾
濡傛灉鎺╃爜澶皬浠ヨ嚦浜庤绯荤粺鏃犳硶鏀寔锛屽畠浠篃鍙兘杩斿洖閿欒銆傚鏋滆繑鍥為潪闆讹紝璇存槑浣犵殑璁惧鍦ㄨ
骞冲彴涓婃棤娉曟纭墽琛?DMA锛屽皾璇曡繖鏍峰仛灏嗗鑷存湭瀹氫箟鐨勮涓恒€傞櫎闈?dma_set_mask 绯诲垪鍑芥暟
杩斿洖鎴愬姛锛屽惁鍒欎綘涓嶅緱鍦ㄨ璁惧涓婁娇鐢?DMA銆?
杩欐剰鍛崇潃鍦ㄥけ璐ョ殑鎯呭喌涓嬶紝浣犳湁涓や釜閫夋嫨锛?
1) 濡傛灉鍙兘锛屼娇鐢ㄦ煇绉嶉潪 DMA 妯″紡杩涜鏁版嵁浼犺緭銆?2) 蹇界暐璇ヨ澶囷紝涓嶈鍒濆鍖栧畠銆?
寤鸿浣犵殑椹卞姩鍦ㄨ缃?DMA 鎺╃爜澶辫触鏃舵墦鍗颁竴鏉″唴鏍?KERN_WARNING 娑堟伅銆傝繖鏍凤紝濡傛灉浣犵殑椹卞姩鐨?鐢ㄦ埛鎶ュ憡鎬ц兘寰堝樊鎴栬€呰澶囩敋鑷虫湭琚娴嬪埌锛屼綘鍙互鍚戜粬浠鍐呮牳娑堟伅鏉ユ煡鏄庣‘鍒囩殑鍘熷洜銆?
```

	if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(24))) {
		dev_warn(dev, "mydev: No suitable DMA available\n");
		goto ignore_this_device;
	}

```
```

	dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64))

```
褰撲负 DMA_BIT_MASK(64) 鏃讹紝dma_set_mask_and_coherent() 姘歌繙涓嶄細杩斿洖澶辫触銆傚吀鍨?```

	/* 閿欒鐨勪唬鐮?*/
	if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64)))
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32))

```
褰撳ぇ浜?32 浣嶆椂锛宒ma_set_mask_and_coherent() 姘歌繙涓嶄細杩斿洖澶辫触銆?```

	/* 鎺ㄨ崘鐨勪唬鐮?*/
	if (support_64bit)
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
	else
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));

```
濡傛灉璁惧浠呭涓€鑷存€у垎閰嶄腑鐨勬弿杩扮鏀寔 32 浣嶅鍧€锛屼絾瀵规祦寮忔槧灏勬敮鎸佸畬鏁寸殑 64 浣?```

	if (dma_set_mask(dev, DMA_BIT_MASK(64))) {
		dev_warn(dev, "mydev: No suitable DMA available\n");
		goto ignore_this_device;
	}

```
涓€鑷存€ф帺鐮佹€绘槸鑳藉璁剧疆涓轰笌娴佸紡鎺╃爜鐩稿悓鎴栨洿灏忕殑鎺╃爜銆備絾瀵逛簬璁惧椹卞姩浠呬娇鐢ㄤ竴鑷存€у垎閰?杩欑缃曡鎯呭喌锛屽氨蹇呴』妫€鏌?dma_set_coherent_mask() 鐨勮繑鍥炲€笺€?
鏈€鍚庯紝濡傛灉浣犵殑璁惧鍙兘椹卞姩浣?24 浣?```

	if (dma_set_mask(dev, DMA_BIT_MASK(24))) {
		dev_warn(dev, "mydev: 24-bit DMA addressing not available\n");
		goto ignore_this_device;
	}

```
褰?dma_set_mask() 鎴?dma_set_mask_and_coherent() 鎴愬姛骞惰繑鍥為浂鏃讹紝鍐呮牳浼氫繚瀛樹綘鎻愪緵鐨?杩欎釜鎺╃爜銆備箣鍚庡湪浣犺繘琛?DMA 鏄犲皠鏃讹紝鍐呮牳浼氫娇鐢ㄨ繖浜涗俊鎭€?
鐩墠鎴戜滑浜嗚В鍒颁竴绉嶅€煎緱涓€鎻愮殑鎯呭喌锛屽€煎緱鍦ㄦ湰鏂囨。涓鏄庛€傚鏋滀綘鐨勮澶囨敮鎸佸涓姛鑳?锛堜緥濡備竴鍧楀０鍗℃彁渚涙挱鏀惧拰褰曢煶鍔熻兘锛夛紝骞朵笖鍚勪釜涓嶅悓鍔熻兘鍏锋湁_涓嶅悓鐨刜 DMA 瀵诲潃闄愬埗锛屼綘
鍙兘甯屾湜鎺㈡祴姣忎釜鎺╃爜锛屽彧鎻愪緵璇ユ満鍣ㄨ兘澶熷鐞嗙殑鍔熻兘銆傞噸瑕佺殑鏄紝瀵?dma_set_mask() 鐨勬渶鍚?涓€娆¤皟鐢ㄥ簲褰撴槸閽堝鏈€鍏蜂綋鐨勬帺鐮併€?
```

	#define PLAYBACK_ADDRESS_BITS	DMA_BIT_MASK(32)
	#define RECORD_ADDRESS_BITS	DMA_BIT_MASK(24)

	struct my_sound_card *card;
	struct device *dev;

	...
	if (!dma_set_mask(dev, PLAYBACK_ADDRESS_BITS)) {
		card->playback_enabled = 1;
	} else {
		card->playback_enabled = 0;
		dev_warn(dev, "%s: Playback disabled due to DMA limitations\n",
		       card->name);
	}
	if (!dma_set_mask(dev, RECORD_ADDRESS_BITS)) {
		card->record_enabled = 1;
	} else {
		card->record_enabled = 0;
		dev_warn(dev, "%s: Record disabled due to DMA limitations\n",
		       card->name);
	}

```
杩欓噷浠ュ０鍗′负渚嬶紝鏄洜涓鸿繖绫?PCI 璁惧浼间箮鍏呮枼鐫€甯︽湁 PCI 鍓嶇鐨?ISA 鑺墖锛屽洜鑰屼繚鐣欎簡
ISA 鐨?16MB DMA 瀵诲潃闄愬埗銆?
## DMA 鏄犲皠鐨勭被鍨?

鏈変袱绉嶇被鍨嬬殑 DMA 鏄犲皠锛?
- 涓€鑷存€э紙Coherent锛塂MA 鏄犲皠锛岄€氬父鍦ㄩ┍鍔ㄥ垵濮嬪寲鏃舵槧灏勩€佸湪缁撴潫鏃跺彇娑堟槧灏勶紝纭欢搴斿綋
  淇濊瘉璁惧鍜?CPU 鍙互骞惰璁块棶鏁版嵁锛屽苟涓旀棤闇€浠讳綍鏄惧紡鐨勮蒋浠跺埛鏂板嵆鍙湅鍒板鏂规墍鍋氱殑鏇存柊銆?
  鍙互鎶娾€滀竴鑷存€р€濈悊瑙ｄ负鈥滃悓姝モ€濄€?
  褰撳墠鐨勯粯璁よ涓烘槸鍦?DMA 绌洪棿鐨勪綆 32 浣嶄腑杩斿洖涓€鑷存€у唴瀛樸€備絾鏄紝涓轰簡灏嗘潵鐨勫吋瀹规€э紝鍗充娇
  杩欎釜榛樿鍊煎浣犵殑椹卞姩鏉ヨ娌￠棶棰橈紝浣犱篃搴旇璁剧疆涓€鑷存€ф帺鐮併€?
  閫傚悎浣跨敤涓€鑷存€ф槧灏勭殑濂戒緥瀛愭湁锛?
 - 缃戝崱 DMA 鐜舰鎻忚堪绗︺€? - SCSI 閫傞厤鍣ㄩ偖绠卞懡浠ゆ暟鎹粨鏋勩€? - 浠庝富瀛樹腑鎵ц鐨勮澶囧浐浠跺井鐮併€?
  杩欎簺渚嬪瓙鐨勫叡鍚屼笉鍙樺紡鏄細浠讳綍 CPU 瀵瑰唴瀛樼殑瀛樺偍閮界珛鍗冲璁惧鍙锛屽弽涔嬩害鐒躲€備竴鑷存€?  鏄犲皠淇濊瘉浜嗚繖涓€鐐广€?
```

	     Coherent DMA memory does not preclude the usage of
	     proper memory barriers.  The CPU may reorder stores to
	     coherent memory just as it may normal memory.  Example:
	     if it is important for the device to see the first word
	     of a descriptor updated before the second, you must do
	     something like::

		desc->word0 = address;
		wmb();
		desc->word1 = DESC_VALID;

             in order to get correct behavior on all platforms.

	     Also, on some platforms your driver may need to flush CPU write
	     buffers in much the same way as it needs to flush write buffers
	     found in PCI bridges (such as by reading a register's value
	     after writing it).

```
- 娴佸紡锛圫treaming锛塂MA 鏄犲皠锛岄€氬父鏄犲皠鐢ㄤ簬涓€娆?DMA 浼犺緭锛屼紶杈撳悗绔嬪嵆鍙栨秷鏄犲皠锛堥櫎闈炰綘
  鍦ㄤ笅闈娇鐢?dma_sync_*锛夛紝纭欢鍙互閽堝椤哄簭璁块棶杩涜浼樺寲銆?
  鍙互鎶娾€滄祦寮忊€濈悊瑙ｄ负鈥滃紓姝モ€濇垨鈥滃湪涓€鑷存€у煙涔嬪鈥濄€?
  閫傚悎浣跨敤娴佸紡鏄犲皠鐨勫ソ渚嬪瓙鏈夛細

 - 璁惧鍙戦€?鎺ユ敹鐨勭綉缁滅紦鍐插尯銆? - SCSI 璁惧鍐欏叆/璇诲彇鐨勬枃浠剁郴缁熺紦鍐插尯銆?
  杩欑被鏄犲皠鐨勪娇鐢ㄦ帴鍙ｅ湪璁捐鏃跺氨鑰冭檻鍒颁簡瀹炵幇鍙互鍋氱‖浠跺厑璁哥殑浠讳綍鎬ц兘浼樺寲銆備负姝わ紝浣跨敤
  杩欑被鏄犲皠鏃讹紝浣犲繀椤绘槑纭鏄庝綘甯屾湜鍙戠敓浠€涔堛€?
涓ょ DMA 鏄犲皠閮芥病鏈夋潵鑷簳灞傛€荤嚎鐨勫榻愰檺鍒讹紝灏界鏌愪簺璁惧鍙兘鏈夎繖鏍风殑闄愬埗銆傛澶栵紝鍦?缂撳瓨涓嶆槸 DMA 涓€鑷寸殑绯荤粺涓紝褰撳簳灞傜紦鍐插尯涓嶄笌鍏朵粬鏁版嵁鍏变韩缂撳瓨琛屾椂锛屽伐浣滄儏鍐典細鏇村ソ銆?
## 浣跨敤涓€鑷存€?DMA 鏄犲皠


瑕佸垎閰嶅苟鏄犲皠杈冨ぇ鐨勶紙澶х害 PAGE_SIZE 澶у皬锛変竴鑷存€?DMA 鍖哄煙锛?```

	dma_addr_t dma_handle;

	cpu_addr = dma_alloc_coherent(dev, size, &dma_handle, gfp);

```
鍏朵腑 device 鏄?`struct device *`銆傝繖鍙互鍦ㄤ腑鏂笂涓嬫枃涓互 GFP_ATOMIC 鏍囧織璋冪敤銆?
size 鏄綘鎯冲垎閰嶇殑鍖哄煙闀垮害锛屼互瀛楄妭涓哄崟浣嶃€?
璇ヤ緥绋嬩細涓洪偅涓尯鍩熷垎閰?RAM锛屾墍浠ュ畠绫讳技浜?__get_free_pages()锛堜絾鎺ュ彈 size 鑰屼笉鏄〉
order锛夈€傚鏋滀綘鐨勯┍鍔ㄩ渶瑕佸皬浜庝竴椤电殑鍖哄煙锛屼綘鍙兘鏇村€惧悜浜庝娇鐢ㄤ笅闈㈡弿杩扮殑 dma_pool 鎺ュ彛銆?
涓€鑷存€?DMA 鏄犲皠鎺ュ彛榛樿杩斿洖涓€涓?32 浣嶅彲瀵诲潃鐨?DMA 鍦板潃銆傚嵆浣胯澶囷紙閫氳繃 DMA 鎺╃爜锛夎〃鏄?瀹冨彲浠ュ鍧€楂?32 浣嶏紝涓€鑷存€у垎閰嶄篃鍙細鍦ㄩ€氳繃 dma_set_coherent_mask() 鏄惧紡鏇存敼浜嗕竴鑷存€?DMA 鎺╃爜鐨勬儏鍐典笅锛屾墠浼氫负 DMA 杩斿洖 > 32 浣嶇殑鍦板潃銆俤ma_pool 鎺ュ彛涔熸槸濡傛銆?
dma_alloc_coherent() 杩斿洖涓や釜鍊硷細浣犲彲浠ヤ粠 CPU 鐢ㄦ潵璁块棶瀹冪殑铏氭嫙鍦板潃锛屼互鍙婁綘浼犵粰
缃戝崱鐨?dma_handle銆?
CPU 铏氭嫙鍦板潃鍜?DMA 鍦板潃閮戒繚璇佸榻愬埌澶т簬绛変簬璇锋眰澶у皬鐨勬渶灏?PAGE_SIZE 鐨?order銆傚瓨鍦ㄨ繖涓?涓嶅彉寮忥紙渚嬪锛夋槸涓轰簡淇濊瘉锛氬鏋滀綘鍒嗛厤鐨勫潡灏忎簬绛変簬 64 鍗冨瓧鑺傦紝浣犳敹鍒扮殑缂撳啿鍖虹殑鑼冨洿涓嶄細
璺ㄨ秺 64K 杈圭晫銆?
```

	dma_free_coherent(dev, size, cpu_addr, dma_handle);

```
鍏朵腑 dev銆乻ize 涓庝笂闈㈣皟鐢ㄤ腑鐨勭浉鍚岋紝cpu_addr 鍜?dma_handle 鏄?dma_alloc_coherent() 杩斿洖
缁欎綘鐨勫€笺€傝鍑芥暟涓嶈兘鍦ㄤ腑鏂笂涓嬫枃涓皟鐢ㄣ€?
濡傛灉浣犵殑椹卞姩闇€瑕佸ぇ閲忚緝灏忕殑鍐呭瓨鍖哄煙锛屼綘鍙互缂栧啓鑷畾涔変唬鐮佹潵缁嗗垎 dma_alloc_coherent()
杩斿洖鐨勯〉锛屾垨鑰呬娇鐢?dma_pool API 鏉ュ仛杩欎欢浜嬨€俤ma_pool 绫讳技浜?kmem_cache锛屼絾瀹冧娇鐢?dma_alloc_coherent()锛岃€屼笉鏄?__get_free_pages()銆傛澶栵紝瀹冪悊瑙ｅ父瑙佺殑纭欢瀵归綈绾︽潫锛屼緥濡?闃熷垪澶撮渶瑕佸榻愬埌 N 瀛楄妭杈圭晫銆?
```

	struct dma_pool *pool;

	pool = dma_pool_create(name, dev, size, align, boundary);

```
鈥渘ame鈥濈敤浜庤瘖鏂紙绫讳技 kmem_cache 鐨勫悕绉帮級锛沝ev 鍜?size 鍚屼笂銆傝绫诲瀷鏁版嵁鐨勮澶囩殑纭欢
瀵归綈瑕佹眰鏄€渁lign鈥濓紙浠ュ瓧鑺傝〃绀猴紝涓斿繀椤绘槸 2 鐨勫箓锛夈€傚鏋滀綘鐨勮澶囨病鏈夎法瓒婅竟鐣岀殑闄愬埗锛?涓?boundary 浼?0锛涗紶 4096 琛ㄧず浠庤繖涓睜涓垎閰嶇殑鍐呭瓨涓嶅緱璺ㄨ秺 4K 瀛楄妭杈圭晫锛堜絾鍦ㄩ偅绉嶆儏鍐典笅锛?鎴栬鏈€濂界洿鎺ヤ娇鐢?dma_alloc_coherent()锛夈€?
```

	cpu_addr = dma_pool_alloc(pool, flags, &dma_handle);

```
濡傛灉鍏佽闃诲锛堜笉鍦?in_interrupt 涓紝涔熸病鏈夋寔鏈?SMP 閿侊級锛宖lags 涓?GFP_KERNEL锛屽惁鍒欎负
GFP_ATOMIC銆備笌 dma_alloc_coherent() 涓€鏍凤紝杩欎篃杩斿洖涓や釜鍊硷細cpu_addr 鍜?dma_handle銆?
```

	dma_pool_free(pool, cpu_addr, dma_handle);

```
鍏朵腑 pool 鏄綘浼犵粰 dma_pool_alloc() 鐨勫€硷紝cpu_addr 鍜?dma_handle 鏄?dma_pool_alloc()
杩斿洖鐨勫€笺€傝鍑芥暟鍙互鍦ㄤ腑鏂笂涓嬫枃涓皟鐢ㄣ€?
```

	dma_pool_destroy(pool);

```
鍦ㄩ攢姣佹睜涔嬪墠锛岃纭繚浣犲凡缁忓浠庤姹犲垎閰嶇殑鎵€鏈夊唴瀛樿皟鐢ㄤ簡 dma_pool_free()銆傝鍑芥暟涓嶈兘鍦?涓柇涓婁笅鏂囦腑璋冪敤銆?
## DMA 鏂瑰悜


鏈枃妗ｅ悗缁儴鍒嗘弿杩扮殑鎺ュ彛鎺ュ彈涓€涓?DMA 鏂瑰悜鍙傛暟锛屽畠鏄竴涓暣鏁帮紝鍙栧€间负
```

 DMA_BIDIRECTIONAL
 DMA_TO_DEVICE
 DMA_FROM_DEVICE
 DMA_NONE

```
濡傛灉浣犵煡閬撴柟鍚戯紝灏卞簲褰撴彁渚涚‘鍒囩殑 DMA 鏂瑰悜銆?
DMA_TO_DEVICE 琛ㄧず鈥滀粠涓诲瓨鍒拌澶団€?DMA_FROM_DEVICE 琛ㄧず鈥滀粠璁惧鍒颁富瀛樷€?瀹冩槸 DMA 浼犺緭杩囩▼涓暟鎹Щ鍔ㄧ殑鏂瑰悜銆?
浣犺_寮虹儓_榧撳姳灏藉彲鑳界簿纭湴鎸囧畾瀹冦€?
濡傛灉浣犵粷瀵规棤娉曠煡閬?DMA 浼犺緭鐨勬柟鍚戯紝璇锋寚瀹?DMA_BIDIRECTIONAL銆傚畠琛ㄧず DMA 鍙互鍚戜换涓€鏂瑰悜
杩涜銆傚钩鍙颁繚璇佷綘鍙互鍚堟硶鍦版寚瀹氬畠锛屽苟涓斿畠浼氭甯稿伐浣滐紝浣嗚繖鍙兘鏄互鎬ц兘涓轰唬浠风殑銆?
DMA_NONE 杩欎釜鍊肩敤浜庤皟璇曘€備綘鍙互鍦ㄧ‘鍒囨柟鍚戠‘瀹氫箣鍓嶅皢瀹冧繚瀛樺湪涓€涓暟鎹粨鏋勪腑锛岃繖鏈夊姪浜?鎹曡幏浣犵殑鏂瑰悜璺熻釜閫昏緫鏈兘姝ｇ‘璁剧疆鐨勬儏鍐点€?
绮剧‘鎸囧畾杩欎釜鍊硷紙闄や簡娼滃湪鐨勫钩鍙扮壒瀹氫紭鍖栦箣澶栵級鐨勫彟涓€涓ソ澶勬槸渚夸簬璋冭瘯銆傛煇浜涘钩鍙板疄闄呬笂
鏈変竴涓啓鏉冮檺甯冨皵鍊硷紝DMA 鏄犲皠鍙互琚爣璁颁笂瀹冿紝灏卞儚鐢ㄦ埛绋嬪簭鍦板潃绌洪棿涓殑椤典繚鎶や竴鏍枫€傚綋
DMA 鎺у埗鍣ㄧ‖浠舵娴嬪埌杩濆弽浜嗚鏉冮檺璁剧疆鏃讹紝杩欑被骞冲彴鍙互骞朵笖纭疄浼氬湪鍐呮牳鏃ュ織涓姤鍛婇敊璇€?
鍙湁娴佸紡鏄犲皠鎵嶆寚瀹氭柟鍚戯紝涓€鑷存€ф槧灏勯殣寮忓湴灏嗘柟鍚戝睘鎬ц涓?DMA_BIDIRECTIONAL銆?
SCSI 瀛愮郴缁熶細鍦ㄤ綘鐨勯┍鍔ㄦ鍦ㄥ鐞嗙殑 SCSI 鍛戒护鐨?'sc_data_direction' 鎴愬憳涓憡璇変綘瑕佷娇鐢?鐨勬柟鍚戙€?
瀵逛簬缃戠粶椹卞姩锛岃繖鏄竴浠剁浉褰撶畝鍗曠殑浜嬫儏銆傚浜庡彂閫佹暟鎹寘锛屼娇鐢?DMA_TO_DEVICE 鏂瑰悜璇存槑绗?鏉ユ槧灏?鍙栨秷鏄犲皠瀹冧滑銆傚浜庢帴鏀舵暟鎹寘锛屽垯姝ｅソ鐩稿弽锛屼娇鐢?DMA_FROM_DEVICE 鏂瑰悜璇存槑绗︽潵
鏄犲皠/鍙栨秷鏄犲皠瀹冧滑銆?
## 浣跨敤娴佸紡 DMA 鏄犲皠


娴佸紡 DMA 鏄犲皠渚嬬▼鍙互鍦ㄤ腑鏂笂涓嬫枃涓皟鐢ㄣ€傛瘡涓槧灏?鍙栨秷鏄犲皠閮芥湁涓や釜鐗堟湰锛屼竴涓槧灏?
鍙栨秷鏄犲皠鍗曚釜鍐呭瓨鍖哄煙锛屽彟涓€涓槧灏?鍙栨秷鏄犲皠涓€涓?scatterlist銆?
```

	struct device *dev = &my_dev->dev;
	dma_addr_t dma_handle;
	void *addr = buffer->ptr;
	size_t size = buffer->len;

	dma_handle = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

```
```

	dma_unmap_single(dev, dma_handle, size, direction);

```
浣犲簲璇ヨ皟鐢?dma_mapping_error()锛屽洜涓?dma_map_single() 鍙兘浼氬け璐ュ苟杩斿洖閿欒銆傝繖鏍峰仛鍙互
纭繚鏄犲皠浠ｇ爜鍦ㄦ墍鏈?DMA 瀹炵幇涓婇兘鑳芥纭伐浣滐紝鑰屼笉渚濊禆浜庡簳灞傚疄鐜扮殑缁嗚妭銆備笉缁忛敊璇鏌ュ氨
浣跨敤杩斿洖鐨勫湴鍧€锛屽彲鑳藉鑷翠粠鍐呮牳宕╂簝鍒伴潤榛樻暟鎹崯鍧忕瓑鍚勭鏁呴殰銆傝繖鍚屾牱閫傜敤浜?dma_map_page()銆?
浣犲簲璇ュ湪 DMA 娲诲姩缁撴潫鏃讹紙渚嬪锛屼粠鍛婅瘔浣?DMA 浼犺緭宸插畬鎴愮殑涓柇涓級璋冪敤 dma_unmap_single()銆?
鍍忚繖鏍峰鍗曚釜鏄犲皠浣跨敤 CPU 鎸囬拡鏈変竴涓己鐐癸細浣犳棤娉曚互杩欑鏂瑰紡寮曠敤 HIGHMEM 鍐呭瓨銆傚洜姝わ紝瀛樺湪
涓€瀵圭被浼间簬 dma_{map,unmap}_single() 鐨勬槧灏?鍙栨秷鏄犲皠鎺ュ彛銆傝繖浜涙帴鍙ｅ鐞嗙殑鏄〉/鍋忕Щ瀵癸紝
鑰屼笉鏄?CPU 鎸囬拡銆?```

	struct device *dev = &my_dev->dev;
	dma_addr_t dma_handle;
	struct page *page = buffer->page;
	unsigned long offset = buffer->offset;
	size_t size = buffer->len;

	dma_handle = dma_map_page(dev, page, offset, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

	...

	dma_unmap_page(dev, dma_handle, size, direction);

```
杩欓噷锛屸€渙ffset鈥濊〃绀虹粰瀹氶〉鍐呯殑瀛楄妭鍋忕Щ銆?
浣犲簲璇ヨ皟鐢?dma_mapping_error()锛屽洜涓哄鍓嶅湪 dma_map_single() 璁ㄨ涓墍姒傝堪鐨勶紝dma_map_page()
鍙兘浼氬け璐ュ苟杩斿洖閿欒銆?
浣犲簲璇ュ湪 DMA 娲诲姩缁撴潫鏃讹紙渚嬪锛屼粠鍛婅瘔浣?DMA 浼犺緭宸插畬鎴愮殑涓柇涓級璋冪敤 dma_unmap_page()銆?
```

	int i, count = dma_map_sg(dev, sglist, nents, direction);
	struct scatterlist *sg;

	for_each_sg(sglist, sg, count, i) {
		hw_address[i] = sg_dma_address(sg);
		hw_len[i] = sg_dma_len(sg);
	}

```
鍏朵腑 nents 鏄?sglist 涓殑鏉＄洰鏁般€?
瀹炵幇鍙互鑷敱鍦板皢鍑犱釜杩炵画鐨?sglist 鏉＄洰鍚堝苟涓轰竴涓紙渚嬪锛屽鏋?DMA 鏄犲皠浠?PAGE_SIZE 涓?绮掑害杩涜锛岄偅涔堜换鎰忚繛缁殑 sglist 鏉＄洰閮藉彲浠ュ悎骞朵负涓€涓紝鍙绗竴涓湪椤佃竟鐣岀粨鏉熴€佺浜屼釜
鍦ㄩ〉杈圭晫寮€濮嬧€斺€斾簨瀹炰笂锛屽浜庝笉鑳藉仛鍒嗘暎/鑱氶泦锛坰catter-gather锛夋垨鍒嗘暎/鑱氶泦鏉＄洰鏁伴噺闈炲父
鏈夐檺鐨勭綉鍗℃潵璇达紝杩欐槸涓€涓法澶х殑浼樺娍锛夛紝骞惰繑鍥炲畠鏄犲皠鍒扮殑瀹為檯 sg 鏉＄洰鏁般€傚け璐ユ椂杩斿洖 0銆?
鐒跺悗浣犲簲璇ュ惊鐜?count 娆★紙娉ㄦ剰锛氳繖鍙兘灏戜簬 nents 娆★級锛屽苟鍦ㄤ綘鍘熷厛璁块棶 sg->address 鍜?sg->length 鐨勫湴鏂逛娇鐢?sg_dma_address() 鍜?sg_dma_len() 瀹忥紝濡備笂鎵€绀恒€?
```

	dma_unmap_sg(dev, sglist, nents, direction);

```
鍐嶆寮鸿皟锛岃纭繚 DMA 娲诲姩宸茬粡缁撴潫銆?

	浼犵粰 dma_unmap_sg 璋冪敤鐨?'nents' 鍙傛暟蹇呴』鏄綘浼犵粰 dma_map_sg 璋冪敤鐨?	_鍚屼竴涓猒锛屽畠_涓嶅簲_鏄?dma_map_sg 璋冪敤_杩斿洖_鐨?'count' 鍊笺€?
姣忎釜 dma_map_{single,sg}() 璋冪敤閮藉簲璇ユ湁瀵瑰簲鐨?dma_unmap_{single,sg}() 璋冪敤锛屽洜涓?DMA
鍦板潃绌洪棿鏄竴涓叡浜祫婧愶紝濡傛灉浣犺€楀敖浜嗘墍鏈?DMA 鍦板潃锛屽彲鑳戒細浣挎満鍣ㄦ棤娉曚娇鐢ㄣ€?
濡傛灉浣犻渶瑕佸娆′娇鐢ㄥ悓涓€涓祦寮?DMA 鍖哄煙锛屽苟涓斿湪 DMA 浼犺緭涔嬮棿浼氳Е纰版暟鎹紝閭ｄ箞璇ョ紦鍐插尯
闇€瑕佽姝ｇ‘鍚屾锛屼互渚?CPU 鍜岃澶囬兘鑳界湅鍒版渶鏂颁笖姝ｇ‘鐨?DMA 缂撳啿鍖哄壇鏈€?
鎵€浠ワ紝棣栧厛锛屽彧鐢?dma_map_{single,sg}() 鏄犲皠瀹冿紝鐒跺悗鍦ㄦ瘡娆?DMA 涔嬪悗
```

	dma_sync_single_for_cpu(dev, dma_handle, size, direction);

```
```

	dma_sync_sg_for_cpu(dev, sglist, nents, direction);

```
瑙嗘儏鍐典娇鐢ㄣ€?
鐒跺悗锛屽鏋滀綘鎯宠璁惧鍐嶆璁块棶 DMA 鍖哄煙锛屽湪 CPU 瀹屾垚瀵规暟鎹殑璁块棶涔嬪悗锛屽苟涓斿湪鐪熸
```

	dma_sync_single_for_device(dev, dma_handle, size, direction);

```
```

	dma_sync_sg_for_device(dev, sglist, nents, direction);

```
瑙嗘儏鍐典娇鐢ㄣ€?

	      dma_sync_sg_for_cpu() 鍜?dma_sync_sg_for_device() 鐨?'nents'
	      鍙傛暟蹇呴』涓庝紶缁?dma_map_sg() 鐨勭浉鍚屻€傚畠_涓嶆槸_ dma_map_sg()
	      杩斿洖鐨?count銆?
鍦ㄦ渶鍚庝竴娆?DMA 浼犺緭涔嬪悗锛岃皟鐢?dma_unmap_{single,sg}() 涔嬩竴銆傚鏋滀綘浠庣涓€娆?dma_map_**()
璋冪敤鍒?dma_unmap_**() 閮芥病鏈夎Е纰版暟鎹紝閭ｄ箞浣犳牴鏈笉闇€瑕佽皟鐢?dma_sync_*() 渚嬬▼銆?
涓嬮潰鏄竴涓吉浠ｇ爜锛屽睍绀轰簡涓€涓綘闇€瑕佷娇鐢ㄥ悓姝ョ殑鎯呭喌
```

	my_card_setup_receive_buffer(struct my_card *cp, char *buffer, int len)
	{
		dma_addr_t mapping;

		mapping = dma_map_single(cp->dev, buffer, len, DMA_FROM_DEVICE);
		if (dma_mapping_error(cp->dev, mapping)) {
			/*
			 * reduce current DMA mapping usage,
			 * delay and try again later or
			 * reset driver.
			 */
			goto map_error_handling;
		}

		cp->rx_buf = buffer;
		cp->rx_len = len;
		cp->rx_dma = mapping;

		give_rx_buf_to_card(cp);
	}

	...

	my_card_interrupt_handler(int irq, void *devid, struct pt_regs *regs)
	{
		struct my_card *cp = devid;

		...
		if (read_card_status(cp) == RX_BUF_TRANSFERRED) {
			struct my_card_header *hp;

			/* Examine the header to see if we wish
			 * to accept the data.  But synchronize
			 * the DMA transfer with the CPU first
			 * so that we see updated contents.
			 */
			dma_sync_single_for_cpu(&cp->dev, cp->rx_dma,
						cp->rx_len,
						DMA_FROM_DEVICE);

			/* Now it is safe to examine the buffer. */
			hp = (struct my_card_header *) cp->rx_buf;
			if (header_is_ok(hp)) {
				dma_unmap_single(&cp->dev, cp->rx_dma, cp->rx_len,
						 DMA_FROM_DEVICE);
				pass_to_upper_layers(cp->rx_buf);
				make_and_setup_new_rx_buf(cp);
			} else {
				/* CPU should not write to
				 * DMA_FROM_DEVICE-mapped area,
				 * so dma_sync_single_for_device() is
				 * not needed here. It would be required
				 * for DMA_BIDIRECTIONAL mapping if
				 * the memory was modified.
				 */
				give_rx_buf_to_card(cp);
			}
		}
	}

```
## 閿欒澶勭悊


鍦ㄦ煇浜涙灦鏋勪笂 DMA 鍦板潃绌洪棿鏄湁闄愮殑锛屽垎閰嶅け璐ュ彲浠ラ€氳繃浠ヤ笅鏂瑰紡鍒ゅ畾锛?
- 妫€鏌?dma_alloc_coherent() 鏄惁杩斿洖 NULL锛屾垨 dma_map_sg 鏄惁杩斿洖 0

- 妫€鏌?dma_map_single() 鍜?dma_map_page() 杩斿洖鐨?dma_addr_t
```

	dma_addr_t dma_handle;

	dma_handle = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

```
- 褰撳湪澶氶〉鏄犲皠灏濊瘯涓€斿彂鐢熸槧灏勯敊璇椂锛屽彇娑堟槧灏勫凡缁忔槧灏勭殑椤点€傝繖浜涗緥瀛愬悓鏍烽€傜敤浜?  dma_map_page()銆?
```

	dma_addr_t dma_handle1;
	dma_addr_t dma_handle2;

	dma_handle1 = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle1)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling1;
	}
	dma_handle2 = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle2)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling2;
	}

	...

	map_error_handling2:
		dma_unmap_single(dma_handle1);
	map_error_handling1:

```
```

	/*
	 * if buffers are allocated in a loop, unmap all mapped buffers when
	 * mapping error is detected in the middle
	 */

	dma_addr_t dma_addr;
	dma_addr_t array[DMA_BUFFERS];
	int save_index = 0;

	for (i = 0; i < DMA_BUFFERS; i++) {

		...

		dma_addr = dma_map_single(dev, addr, size, direction);
		if (dma_mapping_error(dev, dma_addr)) {
			/*
			 * reduce current DMA mapping usage,
			 * delay and try again later or
			 * reset driver.
			 */
			goto map_error_handling;
		}
		array[i].dma_addr = dma_addr;
		save_index++;
	}

	...

	map_error_handling:

	for (i = 0; i < save_index; i++) {

		...

		dma_unmap_single(array[i].dma_addr);
	}

```
缃戠粶椹卞姩鍦?DMA 鏄犲皠鍦ㄥ彂閫侀挬瀛愶紙ndo_start_xmit锛変腑澶辫触鏃讹紝蹇呴』璋冪敤 dev_kfree_skb() 鏉?閲婃斁濂楁帴瀛楃紦鍐插尯骞惰繑鍥?NETDEV_TX_OK銆傝繖鎰忓懗鐫€濂楁帴瀛楃紦鍐插尯鍦ㄥけ璐ユ儏鍐典笅琚洿鎺ヤ涪寮冦€?
SCSI 椹卞姩鍦?queuecommand 閽╁瓙涓?DMA 鏄犲皠澶辫触鏃讹紝蹇呴』杩斿洖 SCSI_MLQUEUE_HOST_BUSY銆傝繖鎰忓懗鐫€
SCSI 瀛愮郴缁熺◢鍚庝細鍐嶆灏嗚鍛戒护浜ょ粰椹卞姩銆?
## 浼樺寲鍙栨秷鏄犲皠鐘舵€佺┖闂村崰鐢?

鍦ㄨ澶氬钩鍙颁笂锛宒ma_unmap_{single,page}() 鍏跺疄灏辨槸涓€涓┖鎿嶄綔锛坣op锛夈€傚洜姝わ紝璁板綍鏄犲皠鍦板潃鍜?闀垮害鏄湪娴垂绌洪棿銆備笅闈㈢殑璁炬柦鎻愪緵浜嗗姙娉曪紝鑰屼笉鏄敤 ifdef 涔嬬被鐨勪笢瑗挎妸浣犵殑椹卞姩濉弧鏉?鈥滅粫寮€鈥濊繖涓棶棰橈紙閭ｆ牱浼氳繚鑳屽彲绉绘 API 鐨勬暣涓垵琛凤級銆?
瀹為檯涓婏紝鎴戜滑涓嶆潵閫愪釜鎻忚堪杩欎簺瀹忥紝鑰屾槸杞崲涓€浜涚ず渚嬩唬鐮併€?
1) 鍦ㄤ繚瀛樼姸鎬佺殑缁撴瀯浣撲腑浣跨敤 DEFINE_DMA_UNMAP_{ADDR,LEN}銆?```

	struct ring_state {
		struct sk_buff *skb;
		dma_addr_t mapping;
		__u32 len;
	};

   after::

	struct ring_state {
		struct sk_buff *skb;
		DEFINE_DMA_UNMAP_ADDR(mapping);
		DEFINE_DMA_UNMAP_LEN(len);
	};

```
2) 浣跨敤 dma_unmap_{addr,len}_set() 鏉ヨ缃繖浜涘€笺€?```

	ringp->mapping = FOO;
	ringp->len = BAR;

   after::

	dma_unmap_addr_set(ringp, mapping, FOO);
	dma_unmap_len_set(ringp, len, BAR);

```
3) 浣跨敤 dma_unmap_{addr,len}() 鏉ヨ闂繖浜涘€笺€?```

	dma_unmap_single(dev, ringp->mapping, ringp->len,
			 DMA_FROM_DEVICE);

   after::

	dma_unmap_single(dev,
			 dma_unmap_addr(ringp, mapping),
			 dma_unmap_len(ringp, len),
			 DMA_FROM_DEVICE);

```
杩欏簲璇ユ槸涓嶈█鑷槑鐨勩€傛垜浠皢 ADDR 鍜?LEN 鍒嗗紑澶勭悊锛屽洜涓哄疄鐜版湁鍙兘鍙渶瑕佸湴鍧€灏辫兘鎵ц
鍙栨秷鏄犲皠鎿嶄綔銆?
## 骞冲彴鐩稿叧闂


濡傛灉浣犲彧鏄负 Linux 缂栧啓椹卞姩锛岃€屼笉缁存姢鍐呮牳鐨勬煇涓灦鏋勭Щ妞嶏紝浣犲彲浠ュ畨鍏ㄥ湴璺冲埌鈥滅粨鏉熻鈥濄€?
1) struct scatterlist 鐨勮姹傘€?
   濡傛灉鏋舵瀯鏀寔 IOMMU锛堝寘鎷蒋浠?IOMMU锛夛紝浣犻渶瑕佸惎鐢?CONFIG_NEED_SG_DMA_LENGTH銆?
2) ARCH_DMA_MINALIGN

   鏋舵瀯蹇呴』纭繚 kmalloc 鍒嗛厤鍑虹殑缂撳啿鍖烘槸 DMA 瀹夊叏鐨勩€傞┍鍔ㄥ拰瀛愮郴缁熼兘渚濊禆瀹冦€傚鏋滀竴涓?   鏋舵瀯涓嶆槸瀹屽叏 DMA 涓€鑷寸殑锛堝嵆纭欢涓嶈兘纭繚 CPU 缂撳瓨涓殑鏁版嵁涓庝富瀛樹腑鐨勬暟鎹浉鍚岋級锛?   ARCH_DMA_MINALIGN 蹇呴』琚缃紝浠ヤ究鍐呭瓨鍒嗛厤鍣ㄧ‘淇?kmalloc 鍒嗛厤鐨勭紦鍐插尯涓嶄細涓庡叾浠?   缂撳啿鍖哄叡浜紦瀛樿銆傚弬瑙?arch/arm/include/asm/cache.h 浣滀负渚嬪瓙銆?
   娉ㄦ剰锛孉RCH_DMA_MINALIGN 鏄叧浜?DMA 鍐呭瓨瀵归綈绾︽潫鐨勩€備綘涓嶉渶瑕佹媴蹇冩灦鏋勭殑鏁版嵁瀵归綈绾︽潫
   锛堜緥濡傚叧浜?64 浣嶅璞＄殑瀵归綈绾︽潫锛夈€?
## 缁撴潫璇?

濡傛灉娌℃湁浼楀涓汉鐨勫弽棣堝拰寤鸿锛屾湰鏂囨。浠ュ強 API 鏈韩涓嶄細鏄幇鍦ㄨ繖涓牱瀛愩€?鎴戜滑鎯崇壒鍒彁鍙婁互涓嬩汉澹紙鎺掑悕涓嶅垎鍏堝悗锛夛細
```

	Russell King <rmk@arm.linux.org.uk>
	Leo Dagum <dagum@barrel.engr.sgi.com>
	Ralf Baechle <ralf@oss.sgi.com>
	Grant Grundler <grundler@cup.hp.com>
	Jay Estabrook <Jay.Estabrook@compaq.com>
	Thomas Sailer <sailer@ife.ee.ethz.ch>
	Andrea Arcangeli <andrea@suse.de>
	Jens Axboe <jens.axboe@oracle.com>
	David Mosberger-Tang <davidm@hpl.hp.com>

```
