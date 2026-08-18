
## 鍒嗘暎鍒楄〃鍔犲瘑 API


## 绠€浠?

Scatterlist Crypto API 浠ラ〉鍚戦噺锛坰catterlists锛変綔涓哄弬鏁帮紝骞剁洿鎺ヤ綔鐢ㄤ簬
椤点€傚湪鏌愪簺鎯呭喌涓嬶紙渚嬪 ECB 妯″紡瀵嗙爜锛夛紝杩欏皢鍏佽椤靛氨鍦板姞瀵嗚€屾棤闇€浠讳綍
鎷疯礉銆?
璇ヨ璁℃渶鍒濈殑鐩爣涔嬩竴鏄究浜庢敮鎸?IPsec锛屼粠鑰屽彲浠ュ鍒嗛〉鐨?skb 鐩存帴鏂藉姞
澶勭悊鑰屾棤闇€绾挎€у寲銆?

## 缁嗚妭


鏈€搴曞眰鏄畻娉曪紝瀹冧滑浼氬悜璇?API 鍔ㄦ€佹敞鍐屻€?
鈥滆浆鎹⑩€濓紙Transforms锛夋槸鐢ㄦ埛瀹炰緥鍖栫殑瀵硅薄锛屽畠浠淮鎶ょ姸鎬併€佸鐞嗘墍鏈夊疄鐜?閫昏緫锛堜緥濡傛搷浣滈〉鍚戦噺锛夛紝骞朵负搴曞眰绠楁硶鎻愪緵涓€灞傛娊璞°€備絾鍦ㄧ敤鎴峰眰闈紝瀹冧滑
闈炲父绠€娲併€?
```

  [transform api]  (鐢ㄦ埛鎺ュ彛)
  [transform ops]  (鎸夌被鍨嬬殑閫昏緫绮樺悎锛屼緥濡?cipher.c銆乧ompress.c)
  [algorithm api]  (鐢ㄤ簬娉ㄥ唽绠楁硶)

```
鍏舵€濇兂鏄鐢ㄦ埛鐣岄潰鍜岀畻娉曟敞鍐?API 閮藉敖閲忕畝鍗曪紝鍚屾椂灏嗘牳蹇冮€昏緫瀵逛簩鑰?闅愯棌銆傛潵鑷幇鏈?API锛堝 Cryptoapi 鍜?Nettle锛夌殑璁稿浼樼璁捐鎬濇兂宸茶
鍊熼壌浜庢銆?
璇?API 鐩墠鏀寔浜旂涓昏鐨勮浆鎹㈢被鍨嬶細AEAD锛堝甫鍏宠仈鏁版嵁鐨勮璇佸姞瀵嗭級銆?鍒嗙粍瀵嗙爜锛圔lock Ciphers锛夈€佸瘑鐮侊紙Ciphers锛夈€佸帇缂╁櫒锛圕ompressors锛夊拰
鍝堝笇锛圚ashes锛夈€?
璇锋敞鎰忥紝鈥滃垎缁勫瘑鐮佲€濆灏戞湁浜涚敤璇嶄笉褰撱€傚畠瀹為檯涓婃棬鍦ㄦ敮鎸佸寘鎷祦瀵嗙爜鍦ㄥ唴
鐨勬墍鏈夊瘑鐮併€傚垎缁勫瘑鐮佷笌瀵嗙爜鐨勫尯鍒湪浜庯細鍚庤€呮伆濂芥搷浣滀竴涓潡锛岃€屽墠鑰呭彲浠?鎿嶄綔浠绘剰鏁伴噺鐨勬暟鎹紝浣嗛渶婊¤冻鍧楀ぇ灏忚姹傦紙鍗抽潪娴佸瘑鐮佸彧鑳藉鐞嗗潡澶у皬鐨勬暣鏁?鍊嶏級銆?
```

	#include <crypto/hash.h>
	#include <linux/err.h>
	#include <linux/scatterlist.h>

	struct scatterlist sg[2];
	char result[128];
	struct crypto_ahash *tfm;
	struct ahash_request *req;

	tfm = crypto_alloc_ahash("md5", 0, CRYPTO_ALG_ASYNC);
	if (IS_ERR(tfm))
		fail();

	/* ... 璁剧疆 scatterlists ... */

	req = ahash_request_alloc(tfm, GFP_ATOMIC);
	if (!req)
		fail();

	ahash_request_set_callback(req, 0, NULL, NULL);
	ahash_request_set_crypt(req, sg, result, 2);

	if (crypto_ahash_digest(req))
		fail();

	ahash_request_free(req);
	crypto_free_ahash(tfm);


```
璁稿鐪熷疄绀轰緥鍙湪鍥炲綊娴嬭瘯妯″潡锛坱crypt.c锛変腑鎵惧埌銆?

## 寮€鍙戣€呴』鐭?

杞崲鍙兘鍦ㄧ敤鎴蜂笂涓嬫枃锛坲ser context锛変腑鍒嗛厤锛屽姞瀵嗘柟娉曞彧鑳戒粠 softirq 鍜?鐢ㄦ埛涓婁笅鏂囦腑璋冪敤銆傚浜庡甫鏈?setkey 鏂规硶鐨勮浆鎹紝setkey 涔熷簲鍙湪鐢ㄦ埛涓婁笅鏂?涓皟鐢ㄣ€?
浣跨敤璇?API 澶勭悊瀵嗙爜鏃讹紝鑻ユ瘡涓?scatterlist 鍖呭惈鐨勬暟鎹噺鏄瘑鐮佸潡澶у皬
锛堥€氬父涓?8 瀛楄妭锛夌殑鏁存暟鍊嶏紝鍒欐€ц兘鏈€浣炽€傝繖鍙互閬垮厤鍦ㄩ潪瀵归綈鐨勯〉鐗囨杈圭晫
涓婅繘琛屼换浣曟嫹璐濄€?

## 鏂板绠楁硶


鎻愪氦鏂扮畻娉曚互渚涘悎鍏ユ椂锛屼竴椤瑰己鍒舵€ц姹傛槸鑷冲皯鍖呭惈鏉ヨ嚜宸茬煡鏉ユ簮锛堟渶濂芥槸
鏍囧噯锛夌殑鍑犱釜娴嬭瘯鍚戦噺銆?
浼樺厛閲囩敤杞崲鐜版湁鐭ュ悕浠ｇ爜鐨勬柟寮忥紝鍥犱负瀹冩洿鍙兘宸茶瀹￠槄骞剁粡骞挎硾娴嬭瘯銆傝嫢
鎻愪氦鏉ヨ嚜 LGPL 鏉ユ簮鐨勪唬鐮侊紝璇疯€冭檻灏嗚鍙瘉鏀逛负 GPL锛堣 LGPL 绗?3 鑺傦級銆?
鎻愪氦鐨勭畻娉曡繕蹇呴』澶т綋涓婃棤涓撳埄闂锛堜緥濡?IDEA 鍦?2011 骞村墠鍚庝箣鍓嶄笉浼氳
鍚堝叆涓荤嚎锛夛紝骞朵笖搴斿熀浜庡叕璁ょ殑鏍囧噯鍜?鎴栫粡杩囬€傚綋鐨勫悓琛岃瘎瀹°€?
鍚屾椂璇锋煡闃呭彲鑳界浉鍏崇殑 RFC锛屼互鍙婇€氱敤鐨勫簲鐢ㄨ鏄庯紝渚嬪 RFC2451
锛堚€淭he ESP CBC-Mode Cipher Algorithms鈥濓級銆?
鏈€濂介伩鍏嶄娇鐢ㄥぇ閲忓畯锛屾敼鐢ㄥ唴鑱斿嚱鏁帮紝鍥犱负 gcc 瀵瑰唴鑱斿鐞嗗緱寰堝ソ锛岃€岃繃搴?浣跨敤瀹忓彲鑳戒細鍦ㄦ煇浜涘钩鍙颁笂瀵艰嚧缂栬瘧闂銆?
涔熻鏌ョ湅涓嬫柟缃戠珯涓婄殑 TODO 鍒楄〃锛屼簡瑙ｅ埆浜哄彲鑳藉凡缁忓湪鍋氱殑宸ヤ綔銆?

## Bug 鎶ュ憡


璇峰皢 bug 鎶ュ憡鍙戦€佽嚦锛?    linux-crypto@vger.kernel.org

鎶勯€侊細
    Herbert Xu <herbert@gondor.apana.org.au>,
    David S. Miller <davem@redhat.com>


## 鏇村淇℃伅


鏈夊叧鍚庣画琛ヤ竵鍜屽悇绉嶆洿鏂帮紙鍖呮嫭褰撳墠鐨?TODO 鍒楄〃锛夛紝璇疯锛?http://gondor.apana.org.au/~herbert/crypto/


## 浣滆€?

- James Morris
- David S. Miller
- Herbert Xu


## 鑷磋阿


浠ヤ笅浜哄憳鍦?API 鐨勫紑鍙戣繃绋嬩腑鎻愪緵浜嗗疂璐电殑鍙嶉锛?
  - Alexey Kuznetzov
  - Rusty Russell
  - Herbert Valerio Riedel
  - Jeff Garzik
  - Michael Richardson
  - Andrew Morton
  - Ingo Oeser
  - Christoph Hellwig

鏈?API 鐨勯儴鍒嗗唴瀹规簮鑷互涓嬮」鐩細

  Kerneli Cryptoapi (http://www.kerneli.org/)
   - Alexander Kjeldaas
   - Herbert Valerio Riedel
   - Kyle McMartin
   - Jean-Luc Cooke
   - David Bryson
   - Clemens Fruhwirth
   - Tobias Ringstrom
   - Harald Welte

浠ュ強锛?
  Nettle (https://www.lysator.liu.se/~nisse/nettle/)
   - Niels M枚ller

鍔犲瘑绠楁硶鐨勫師濮嬪紑鍙戣€咃細

  - Dana L. How (DES)
  - Andrew Tridgell and Steve French (MD4)
  - Colin Plumb (MD5)
  - Steve Reid (SHA1)
  - Jean-Luc Cooke (SHA256, SHA384, SHA512)
  - Kazunori Miyazawa / USAGI (HMAC)
  - Matthew Skala (Twofish)
  - Dag Arne Osvik (Serpent)
  - Brian Gladman (AES)
  - Kartikey Mahendra Bhatt (CAST6)
  - Jon Oberheide (ARC4)
  - Jouni Malinen (Michael MIC)
  - NTT(Nippon Telegraph and Telephone Corporation) (Camellia)

SHA1 绠楁硶璐＄尞鑰咃細
  - Jean-Francois Dive

DES 绠楁硶璐＄尞鑰咃細
  - Raimar Falke
  - Gisle S忙lensminde
  - Niels M枚ller

Blowfish 绠楁硶璐＄尞鑰咃細
  - Herbert Valerio Riedel
  - Kyle McMartin

Twofish 绠楁硶璐＄尞鑰咃細
  - Werner Koch
  - Marc Mutz

SHA256/384/512 绠楁硶璐＄尞鑰咃細
  - Andrew McDonald
  - Kyle McMartin
  - Herbert Valerio Riedel

AES 绠楁硶璐＄尞鑰咃細
  - Alexander Kjeldaas
  - Herbert Valerio Riedel
  - Kyle McMartin
  - Adam J. Richter
  - Fruhwirth Clemens (i586)
  - Linus Torvalds (i586)

CAST5 绠楁硶璐＄尞鑰咃細
  - Kartikey Mahendra Bhatt (鍘熷寮€鍙戣€呮湭鐭ワ紝FSF 鐗堟潈)銆?
TEA/XTEA 绠楁硶璐＄尞鑰咃細
  - Aaron Grothe
  - Michael Ringe

Khazad 绠楁硶璐＄尞鑰咃細
  - Aaron Grothe

Whirlpool 绠楁硶璐＄尞鑰咃細
  - Aaron Grothe
  - Jean-Luc Cooke

Anubis 绠楁硶璐＄尞鑰咃細
  - Aaron Grothe

Tiger 绠楁硶璐＄尞鑰咃細
  - Aaron Grothe

VIA PadLock 璐＄尞鑰咃細
  - Michal Ludvig

Camellia 绠楁硶璐＄尞鑰咃細
  - NTT(Nippon Telegraph and Telephone Corporation) (Camellia)

閫氱敤 scatterwalk 浠ｇ爜鐢?Adam J. Richter <adam@yggdrasil.com> 缂栧啓

璇峰皢浠讳綍鑷磋阿鏇存柊鎴栨洿姝ｅ彂閫佽嚦锛?Herbert Xu <herbert@gondor.apana.org.au>
