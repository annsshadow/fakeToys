## 浣跨敤 ISA 鍜?LPC 璁惧杩涜 DMA


:Author: Pierre Ossman <drzeus@drzeus.cx>

鏈枃妗ｆ弿杩板浣曚娇鐢ㄦ棫鐨?ISA DMA 鎺у埗鍣ㄨ繘琛?DMA 浼犺緭銆傚敖绠?ISA 濡備粖鎴栧鎴栧皯宸茬粡娑堜骸锛?
浣?LPC 鎬荤嚎浣跨敤鐩稿悓鐨?DMA 绯荤粺锛屽洜姝ゅ畠杩樹細瀛樺湪鐩稿綋闀跨殑涓€娈垫椂闂淬€?

### 澶存枃浠朵笌渚濊禆


```

	#include <linux/dma-mapping.h>
	#include <asm/dma.h>

```
绗竴涓槸鐢ㄤ簬鎶婅櫄鎷熷湴鍧€杞崲涓烘€荤嚎鍦板潃鐨勯€氱敤 DMA API锛堣瑙?Documentation/core-api/dma-api.rst锛夈€?

绗簩涓寘鍚壒瀹氫簬 ISA DMA 浼犺緭鐨勪緥绋嬨€傜敱浜庡畠涓嶆槸鍦ㄦ墍鏈夊钩鍙颁笂閮藉瓨鍦紝璇风‘淇濅綘鐨?Kconfig
渚濊禆浜?ISA_DMA_API锛堣€岄潪 ISA锛夛紝杩欐牱灏变笉浼氭湁浜哄湪涓嶆敮鎸佺殑骞冲彴涓婂幓鏋勫缓浣犵殑椹卞姩銆?

### 缂撳啿鍖哄垎閰?


ISA DMA 鎺у埗鍣ㄥ瀹冭兘璁块棶鐨勫唴瀛樻湁闈炲父涓ユ牸鐨勮姹傦紝鍥犳鍦ㄥ垎閰嶇紦鍐插尯鏃跺繀椤绘牸澶栧皬蹇冦€?

锛堜綘閫氬父闇€瑕佷负 DMA 浼犺緭鍒嗛厤涓€涓壒娈婄殑缂撳啿鍖猴紝鑰屼笉鏄洿鎺ヤ粠浣犵殑鏅€氭暟鎹粨鏋勮繘琛屼紶杈撱€傦級

鍙繘琛?DMA 鐨勫湴鍧€绌洪棿鏄?_鐗╃悊_ 鍐呭瓨鏈€浣庣殑 16 MB銆傛澶栵紝浼犺緭鍧椾笉鑳借法瓒婇〉杈圭晫锛堟牴鎹墍鐢?
閫氶亾涓嶅悓锛岄〉澶у皬涓?64 鎴?128 KiB锛夈€?

涓轰簡鍒嗛厤涓€鍧楁弧瓒虫墍鏈夎繖浜涜姹傜殑鍐呭瓨锛屼綘鍚?kmalloc 浼犲叆 GFP_DMA 鏍囧織銆?

閬楁喚鐨勬槸鍙敤浜?ISA DMA 鐨勫唴瀛樺崄鍒嗙█缂猴紝鍥犳闄ら潪浣犲湪鍚姩鏃跺氨鍒嗛厤鍐呭瓨锛屽惁鍒欐渶濂藉悓鏃朵紶鍏?
__GFP_RETRY_MAYFAIL 鍜?__GFP_NOWARN锛岃鍒嗛厤鍣ㄦ洿鍔姏鍦板皾璇曘€?

锛堣繖绉嶇█缂烘€т篃鎰忓懗鐫€浣犲簲褰撳敖鏃╁垎閰嶇紦鍐插尯锛屽苟涓斿湪椹卞姩鍗歌浇涔嬪墠涓嶈閲婃斁瀹冦€傦級

### 鍦板潃杞崲


瑕佸皢铏氭嫙鍦板潃杞崲涓烘€荤嚎鍦板潃锛岃浣跨敤鏅€氱殑 DMA API銆俖涓嶈_ 浣跨敤 isa_virt_to_bus()锛屽嵆浣?
瀹冨仛鐨勬槸鍚屼竴浠朵簨銆傚師鍥犳槸鍑芥暟 isa_virt_to_bus() 浼氳姹?Kconfig 渚濊禆浜?ISA锛岃€屼笉鍙槸
鐪熸鎵€闇€鐨?ISA_DMA_API銆傝璁颁綇锛屽敖绠?DMA 鎺у埗鍣ㄨ捣婧愪簬 ISA锛屼絾瀹冧篃琚敤鍦ㄥ叾浠栧湴鏂广€?

娉ㄦ剰锛歺86_64 鍦?ISA 鏂归潰鐨?DMA API 鏇剧粡鏈夐棶棰橈紝浣嗗悗鏉ュ凡缁忎慨澶嶃€傚鏋滀綘鐨勬灦鏋勬湁闂锛岃
淇 DMA API锛岃€屼笉鏄洖閫€鍒?ISA 鍑芥暟銆?

### 閫氶亾


涓€涓櫘閫氱殑 ISA DMA 鎺у埗鍣ㄦ湁 8 涓€氶亾銆傝緝浣庣殑鍥涗釜鐢ㄤ簬 8 浣嶄紶杈擄紝杈冮珮鐨勫洓涓敤浜?16 浣嶄紶杈撱€?

锛堝疄闄呬笂 DMA 鎺у埗鍣ㄦ槸涓や釜鐙珛鐨勬帶鍒跺櫒锛屽叾涓€氶亾 4 鐢ㄤ簬璁╃浜屼釜鎺у埗鍣紙0-3锛夎幏寰?DMA
璁块棶銆傝繖鎰忓懗鐫€鍥涗釜 16 浣嶉€氶亾涓彧鏈変笁涓彲鐢ㄣ€傦級

浣犲垎閰嶅畠浠殑鏂瑰紡涓庢墍鏈夊熀鏈祫婧愮被浼硷細

extern int request_dma(unsigned int dmanr, const char * device_id);
extern void free_dma(unsigned int dmanr);

浣跨敤 16 浣嶈繕鏄?8 浣嶄紶杈撶殑鑳藉姏_涓峗鐢变綘浣滀负椹卞姩浣滆€呭喅瀹氾紝鑰屾槸鍙栧喅浜庣‖浠舵敮鎸佷粈涔堛€傝鏌ラ槄
浣犵殑瑙勬牸璇存槑鎴栨祴璇曚笉鍚岀殑閫氶亾銆?

### 浼犺緭鏁版嵁


鐜板湪鏄ソ涓滆タ锛屽疄闄呯殑 DMA 浼犺緭銆?)

鍦ㄤ娇鐢ㄤ换浣?ISA DMA 渚嬬▼涔嬪墠锛屼綘闇€瑕佷娇鐢?claim_dma_lock() 鑾峰彇 DMA 閿併€傚師鍥犳槸鏌愪簺 DMA
鎿嶄綔涓嶆槸鍘熷瓙鐨勶紝鍥犳鍚屼竴鏃堕棿鍙兘鏈変竴涓┍鍔ㄥ幓鎽嗗紕杩欎簺瀵勫瓨鍣ㄣ€?

浣犵涓€娆′娇鐢?DMA 鎺у埗鍣ㄦ椂搴斿綋璋冪敤 clear_dma_ff()銆傝繖浼氭竻闄?DMA 鎺у埗鍣ㄤ腑鐢ㄤ簬闈炲師瀛愭搷浣?
鐨勫唴閮ㄥ瘎瀛樺櫒銆傚彧瑕佷綘锛堜互鍙婂叾瀹冩墍鏈変汉锛夐兘浣跨敤閿佸嚱鏁帮紝灏卞彧闇€閲嶇疆涓€娆°€?

鎺ヤ笅鏉ワ紝浣跨敤 set_dma_mode() 鍛婅瘔鎺у埗鍣ㄤ綘鎵撶畻杩涜鍝釜鏂瑰悜鐨勪紶杈撱€傜洰鍓嶄綘鏈?DMA_MODE_READ
鍜?DMA_MODE_WRITE 涓や釜閫夐」銆?

璁剧疆浼犺緭搴斿紑濮嬬殑鍦板潃锛堝浜?16 浣嶄紶杈撻渶瑕?16 浣嶅榻愶級浠ュ強瑕佷紶杈撶殑瀛楄妭鏁般€傛敞鎰忛偅鏄痏瀛楄妭_銆?
DMA 渚嬬▼浼氬畬鎴愭墍鏈夊埌 DMA 鎺у埗鍣ㄦ墍鑳界悊瑙ｇ殑鏁板€兼墍闇€鐨勮浆鎹€?

鏈€鍚庝竴姝ユ槸浣胯兘 DMA 閫氶亾骞堕噴鏀?DMA 閿併€?

DMA 浼犺緭瀹屾垚锛堟垨瓒呮椂锛夊悗锛屼綘搴斿綋鍐嶆绂佺敤璇ラ€氶亾銆備綘杩樺簲褰撴鏌?get_dma_residue() 浠ョ‘淇?
鎵€鏈夋暟鎹兘宸蹭紶杈撱€?

```

	int flags, residue;

	flags = claim_dma_lock();

	clear_dma_ff();

	set_dma_mode(channel, DMA_MODE_WRITE);
	set_dma_addr(channel, phys_addr);
	set_dma_count(channel, num_bytes);

	dma_enable(channel);

	release_dma_lock(flags);

	while (!device_done());

	flags = claim_dma_lock();

	dma_disable(channel);

	residue = dma_get_residue(channel);
	if (residue != 0)
		printk(KERN_ERR "driver: Incomplete DMA transfer!"
			" %d bytes left!\n", residue);

	release_dma_lock(flags);

```
### 鎸傝捣/鎭㈠


椹卞姩鏈夎矗浠荤‘淇濆湪 DMA 浼犺緭杩涜鏈熼棿鏈哄櫒涓嶄細琚寕璧枫€傛澶栵紝褰撶郴缁熸寕璧锋椂鎵€鏈?DMA 璁剧疆閮戒細
涓㈠け锛屽洜姝ゅ鏋滀綘鐨勯┍鍔ㄤ緷璧?DMA 鎺у埗鍣ㄥ浜庢煇绉嶇姸鎬侊紝閭ｄ箞浣犲繀椤诲湪鎭㈠鏃舵仮澶嶈繖浜涘瘎瀛樺櫒銆?
