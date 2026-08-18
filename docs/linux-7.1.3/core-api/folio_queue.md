
## Folio Queue


:Author: David Howells <dhowells@redhat.com>


 - 姒傝堪锛圤verview锛? - 鍒濆鍖栵紙Initialisation锛? - 娣诲姞涓庣Щ闄?folio锛圓dding and removing folios锛? - 鏌ヨ folio 鐨勪俊鎭紙Querying information about a folio锛? - 鏌ヨ folio_queue 鐨勪俊鎭紙Querying information about a folio_queue锛? - folio 闃熷垪杩唬锛團olio queue iteration锛? - folio 鏍囪锛團olio marks锛? - 鏃犻攣鐨勫悓姝ョ敓浜?娑堣垂闂锛圠ockless simultaneous production/consumption issues锛?

## 姒傝堪


folio_queue 缁撴瀯浣撴瀯鎴愪簡 folio 鍒嗘閾捐〃涓殑涓€娈碉紝璇ラ摼琛ㄥ彲鐢ㄤ簬鏋勬垚涓€涓?I/O 缂撳啿鍖恒€?鍥犳锛岃閾捐〃鍙互浣跨敤 ITER_FOLIOQ 绫诲瀷鐨?iov_iter 杩涜杩唬銆?
```

	struct folio_queue {
		struct folio_queue *next;
		struct folio_queue *prev;
		...
	};

```
鎻愪緵浜嗕竴瀵规寚閽?`next` 涓?`prev`锛屽垎鍒寚鍚戣璁块棶娈典袱渚х殑娈点€傝櫧鐒惰繖鏄竴涓弻鍚戦摼琛紝浣嗗畠
鏁呮剰涓嶆槸涓€涓幆褰㈤摼琛紱鏈娈典腑鍚戝鐨勫厔寮熸寚閽堝簲涓?NULL銆?
閾捐〃涓殑姣忎釜娈佃繕瀛樺偍锛?
 - 涓€缁勬湁搴忕殑 folio 鎸囬拡搴忓垪锛? - 姣忎釜 folio 鐨勫ぇ灏忥紝浠ュ強
 - 姣忎釜 folio 涓変釜 1 浣嶇殑鏍囪锛?
浣嗚繖浜涗笉搴旇鐩存帴璁块棶锛屽洜涓哄簳灞傛暟鎹粨鏋勫彲鑳戒細鍙樺寲锛岃€屽簲浣跨敤涓嬮潰鍒楀嚭鐨勮闂嚱鏁般€?
```

	#include <linux/folio_queue.h>

```
```

	#include <linux/uio.h>


```
## 鍒濆鍖?

```

	void folioq_init(struct folio_queue *folioq);

```
骞朵紶鍏ユ寚鍚戝緟鍒濆鍖栨鐨勬寚閽堛€傛敞鎰忥紝杩欎笉涓€瀹氫細鍒濆鍖栨墍鏈夌殑 folio 鎸囬拡锛屽洜姝ゅ繀椤诲皬蹇冩鏌?鎵€娣诲姞鐨?folio 鏁伴噺銆?

## 娣诲姞涓庣Щ闄?folio


鍙互閫氳繃璋冪敤浠ヤ笅鍑芥暟鍦ㄦ缁撴瀯浣撶殑涓嬩竴涓湭浣跨敤妲戒綅涓缃?folio锛?
```

	unsigned int folioq_append(struct folio_queue *folioq,
				   struct folio *folio);

	unsigned int folioq_append_mark(struct folio_queue *folioq,
					struct folio *folio);

```
杩欎袱涓嚱鏁伴兘浼氭洿鏂版墍瀛樺偍鐨?folio 璁℃暟銆佸瓨鍌ㄨ folio 骞惰褰曞叾澶у皬銆傜浜屼釜鍑芥暟杩樹細涓烘墍娣诲姞
鐨?folio 璁剧疆绗竴涓爣璁般€備袱涓嚱鏁伴兘杩斿洖鎵€鐢ㄦЫ浣嶇殑缂栧彿銆俒!] 娉ㄦ剰锛屼笉浼氬皾璇曟鏌ュ閲忔槸鍚?琚孩鍑猴紝閾捐〃涔熶笉浼氳嚜鍔ㄦ墿灞曘€?
```

	void folioq_clear(struct folio_queue *folioq, unsigned int slot);

```
杩欎細娓呯┖鏁扮粍涓殑璇ユЫ浣嶏紝骞舵竻绌鸿 folio 鐨勬墍鏈夋爣璁帮紝浣嗕笉浼氭敼鍙?folio 璁℃暟鈥斺€斿洜姝ゆ湭鏉?璁块棶璇ユЫ浣嶆椂蹇呴』妫€鏌ヨ妲戒綅鏄惁琚崰鐢ㄣ€?

## 鏌ヨ folio 鐨勪俊鎭?

鍙互浣跨敤浠ヤ笅鍑芥暟鏌ヨ鐗瑰畾妲戒綅涓?folio 鐨勪俊鎭細

```

	struct folio *folioq_folio(const struct folio_queue *folioq,
				   unsigned int slot);

```
濡傛灉鏌愪釜妲戒綅涓皻鏈缃?folio锛岃繖鍙兘浜х敓鏈畾涔夌殑缁撴灉锛?
```

	unsigned int folioq_folio_order(const struct folio_queue *folioq,
					unsigned int slot);

	size_t folioq_folio_size(const struct folio_queue *folioq,
				 unsigned int slot);

```
绗竴涓嚱鏁颁互 order 褰㈠紡杩斿洖澶у皬锛岀浜屼釜鍑芥暟浠ュ瓧鑺傛暟褰㈠紡杩斿洖澶у皬銆?

## 鏌ヨ folio_queue 鐨勪俊鎭?

鍙互浣跨敤浠ヤ笅鍑芥暟妫€绱㈢壒瀹氭鐨勪俊鎭細

```

	unsigned int folioq_nr_slots(const struct folio_queue *folioq);

	unsigned int folioq_count(struct folio_queue *folioq);

	bool folioq_full(struct folio_queue *folioq);

```
绗竴涓嚱鏁拌繑鍥炴鐨勬渶澶у閲忋€備笉寰楀亣璁惧畠鍦ㄤ笉鍚屾涔嬮棿涓嶄細鍙樺寲銆傜浜屼釜鍑芥暟杩斿洖娣诲姞鍒版涓殑
folio 鏁伴噺锛岀涓変釜鍑芥暟鏄竴涓畝鍐欙紝鐢ㄤ簬鎸囩ず璇ユ鏄惁宸茶濉厖鍒板閲忎笂闄愩€?
娉ㄦ剰锛岃鏁颁笌濉厖鐘舵€佷笉鍙椾粠娈典腑娓呯┖ folio 鐨勫奖鍝嶃€傚畠浠洿澶氬湴鐢ㄤ簬琛ㄧず鏁扮粍涓湁澶氬皯涓Ы浣?宸茶鍒濆鍖栵紝骞朵笖鍋囧畾妲戒綅涓嶄細琚鐢紝鑰屾槸褰撻槦鍒楄娑堣垂鏃惰娈典細琚涪寮冦€?

## folio 鏍囪


闃熷垪涓殑 folio 涔熷彲浠ヨ璧嬩簣鏍囪銆傝繖浜涙爣璁板彲鐢ㄤ簬璁板綍璇稿鏌愪釜 folio 鏄惁闇€瑕佸鍏惰皟鐢?folio_put() 涔嬬被鐨勪俊鎭€傛瘡涓?folio 鍙缃笁涓爣璁般€?
```

	void folioq_mark(struct folio_queue *folioq, unsigned int slot);
	void folioq_mark2(struct folio_queue *folioq, unsigned int slot);

```
```

	void folioq_unmark(struct folio_queue *folioq, unsigned int slot);
	void folioq_unmark2(struct folio_queue *folioq, unsigned int slot);

```
```

	bool folioq_is_marked(const struct folio_queue *folioq, unsigned int slot);
	bool folioq_is_marked2(const struct folio_queue *folioq, unsigned int slot);

```
杩欎簺鏍囪鍙敤浜庝换浣曠敤閫旓紝鏈?API 涓嶄細瀵瑰叾浣滃嚭瑙ｉ噴銆?

## folio 闃熷垪杩唬


鍙互浣跨敤 I/O 杩唬鍣ㄨ鏂斤紝閫氳繃涓€涓?`ITER_FOLIOQ` 绫诲瀷鐨?`iov_iter` 杩唬鍣ㄦ潵杩唬娈甸摼琛ㄣ€?璇ヨ凯浠ｅ櫒鍙互

```

	void iov_iter_folio_queue(struct iov_iter *i, unsigned int direction,
				  const struct folio_queue *folioq,
				  unsigned int first_slot, unsigned int offset,
				  size_t count);

```
鍙互鍛婄煡瀹冧粠闃熷垪涓壒瀹氱殑娈点€佹Ы浣嶄笌鍋忕Щ澶勫紑濮嬨€俰ov 杩唬鍣ㄥ嚱鏁板湪鍓嶈繘鏃朵細璺熼殢 next 鎸囬拡锛?鍦ㄥ洖閫€鏃朵細璺熼殢 prev 鎸囬拡锛堝湪闇€瑕佹椂锛夈€?

## 鏃犻攣鐨勫悓姝ョ敓浜?娑堣垂闂


濡傛灉绠＄悊寰楀綋锛岄摼琛ㄥ彲浠ョ敱鐢熶骇鑰呭湪澶撮儴绔墿灞曪紝鍚屾椂鐢辨秷璐硅€呭湪灏鹃儴绔缉鐭紝鑰屾棤闇€鍔犻攣銆?ITER_FOLIOQ 杩唬鍣ㄤ細鎻掑叆閫傚綋鐨勫睆闅滄潵杈呭姪杩欎竴鐐广€?
鍚屾椂鐢熶骇涓庢秷璐逛竴涓摼琛ㄦ椂蹇呴』灏忓績銆傚鏋滃埌杈炬渶鍚庝竴涓锛屽苟涓?IOV 杩唬鍣ㄥ凡瀹屽叏娑堣垂瀹冩墍
寮曠敤鐨?folio锛岄偅涔?iov_iter 缁撴瀯浣撳皢鎸囧悜鏈€鍚庝竴涓锛屽叾妲戒綅缂栧彿绛変簬璇ユ鐨勫閲忋€傚綋璇?杩唬鍣ㄥ啀娆¤浣跨敤鏃讹紝瀹冧細灏濊瘯浠庢澶勭户缁紙濡傛灉鏈夊彟涓€涓彲鐢ㄦ锛夛紝浣嗗繀椤诲皬蹇冿紝浠ュ厤璇ユ鍦?杩唬鍣ㄥ墠杩涗箣鍓嶅凡琚秷璐硅€呯Щ闄ゅ苟閲婃斁銆?
寤鸿闃熷垪濮嬬粓鑷冲皯鍖呭惈涓€涓锛屽嵆浣胯娈典粠鏈濉厖鎴栧凡琚畬鍏ㄨ€楀敖銆傝繖鍙互闃叉澶存寚閽堜笌灏?鎸囬拡鍙戠敓閲嶅彔銆?

## API 鍑芥暟鍙傝€?