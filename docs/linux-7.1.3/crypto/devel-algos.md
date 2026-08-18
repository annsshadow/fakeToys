## 寮€鍙戝瘑鐮佺畻娉?

### 娉ㄥ唽涓庢敞閿€鍙樻崲


鍦?Crypto API 涓湁涓夌涓嶅悓绫诲瀷鐨勬敞鍐屽嚱鏁般€備竴绉嶇敤浜庢敞鍐岄€氱敤鐨勫瘑鐮佸鍙樻崲锛坈ryptographic transformation锛夛紝鍙﹀涓ょ鍒欎笓闂ㄧ敤浜?HASH 鍙樻崲鍜屽帇缂╋紙COMPRESSion锛夊彉鎹€傛垜浠皢鍦ㄥ崟鐙殑绔犺妭璁ㄨ鍚庝袱绉嶏紝姝ゅ浠呭叧娉ㄩ€氱敤閭ｄ竴绉嶃€?
鍦ㄨ璁烘敞鍐屽嚱鏁颁箣鍓嶏紝蹇呴』鍏堜簡瑙ｆ瘡涓嚱鏁伴渶瑕佸～鍏呯殑鏁版嵁缁撴瀯 struct crypto_alg 鈥斺€?璇ユ暟鎹粨鏋勭殑鎻忚堪瑙佷笅鏂囥€?
閫氱敤娉ㄥ唽鍑芥暟鍙湪 include/linux/crypto.h 涓壘鍒帮紝鍏跺畾涔夎涓嬫枃銆?鍓嶈€呮敞鍐屽崟涓彉鎹紝鍚庤€呬綔鐢ㄤ簬涓€缁勫彉鎹㈡弿杩扮鏁扮粍銆傚悗鑰呭湪鎵归噺娉ㄥ唽鍙樻崲鏃跺緢鏈夌敤锛?渚嬪褰撲竴涓┍鍔ㄥ疄鐜板涓彉鎹㈡椂銆?
```
       int crypto_register_alg(struct crypto_alg *alg);
       int crypto_register_algs(struct crypto_alg *algs, int count);
```
杩欎簺鍑芥暟鐨勫搴旀敞閿€鍑芥暟鍒椾妇濡備笅銆?
```
       void crypto_unregister_alg(struct crypto_alg *alg);
       void crypto_unregister_algs(struct crypto_alg *algs, int count);
```
娉ㄥ唽鍑芥暟鍦ㄦ垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖璐熺殑 errno 鍊笺€俢rypto_register_algs() 鍙湁鍦ㄦ垚鍔熸敞鍐屼簡鎵€鏈夌粰瀹氱畻娉曟椂鎵嶄細鎴愬姛锛涘鏋滀腑閫斿け璐ワ紝鍒欎换浣曞凡鍋氱殑鏇存敼閮戒細琚洖婊氥€?
娉ㄩ攢鍑芥暟鎬绘槸鎴愬姛锛屽洜姝ゅ畠浠病鏈夎繑鍥炲€笺€備笉瑕佸皾璇曟敞閿€褰撳墠鏈敞鍐岀殑绠楁硶銆?
### 鍗曞潡瀵圭О瀵嗙爜 [CIPHER]


鍙樻崲绀轰緥锛歛es銆乻erpent 绛?
鏈妭鎻忚堪鎵€鏈夊彉鎹㈠疄鐜颁腑鏈€绠€鍗曠殑涓€绉嶏紝鍗崇敤浜庡绉板瘑鐮佺殑 CIPHER 绫诲瀷銆侰IPHER 绫诲瀷鐢ㄤ簬姣忔鎭板ソ鎿嶄綔涓€涓潡銆佷笖鍧椾箣闂村畬鍏ㄦ病鏈変换浣曚緷璧栧叧绯荤殑鍙樻崲銆?
#### 娉ㄥ唽缁嗚妭


[CIPHER] 绠楁硶鐨勬敞鍐岀壒娈婁箣澶勫湪浜庡叾 struct crypto_alg 瀛楁 .cra_type 涓虹┖銆傚繀椤诲～鍏?.cra_u.cipher锛屽苟閰嶄互瀹炵幇姝ゅ彉鎹㈢殑閫傚綋鍥炶皟銆?
鍙傝涓嬫枃鐨?struct cipher_alg銆?
#### 浣跨敤 struct cipher_alg 瀹氫箟瀵嗙爜


Struct cipher_alg 瀹氫箟鍗曞潡瀵嗙爜銆?
浠ヤ笅鏄綋杩欎簺鍑芥暟浠庡唴鏍稿叾浠栭儴鍒嗚璋冪敤鏃剁殑绀烘剰銆傛敞鎰?.cia_setkey() 璋冪敤鍙兘鍙戠敓鍦ㄨ繖浜涚ず鎰忎箣鍓嶆垨涔嬪悗锛屼絾涓嶅緱鍦ㄨ繖浜涚ず鎰忚繘琛屾湡闂村彂鐢熴€?
```
             KEY ---.    PLAINTEXT ---.
                    v                 v
              .cia_setkey() -> .cia_encrypt()
                                      |
                                      '-----> CIPHERTEXT
```
璇锋敞鎰忥紝澶氭璋冪敤 .cia_setkey() 鐨勬ā寮忓悓鏍锋槸鍚堟硶鐨勶細

```
      KEY1 --.    PLAINTEXT1 --.         KEY2 --.    PLAINTEXT2 --.
             v                 v                v                 v
       .cia_setkey() -> .cia_encrypt() -> .cia_setkey() -> .cia_encrypt()
                               |                                  |
                               '---> CIPHERTEXT1                  '---> CIPHERTEXT2
```
### 澶氬潡瀵嗙爜


鍙樻崲绀轰緥锛歝bc(aes)銆乧hacha20 绛?
鏈妭鎻忚堪澶氬潡瀵嗙爜鍙樻崲鐨勫疄鐜般€傚鍧楀瘑鐮佺敤浜庢搷浣滄彁渚涚粰鍙樻崲鍑芥暟鐨勫垎鏁ｅ垪琛紙scatterlist锛夋暟鎹€傚畠浠篃灏嗙粨鏋滆緭鍑哄埌鏁版嵁鍒嗘暎鍒楄〃涓€?
#### 娉ㄥ唽缁嗚妭


澶氬潡瀵嗙爜绠楁硶鐨勬敞鍐屾槸鏁翠釜 crypto API 涓渶鏍囧噯鐨勬祦绋嬩箣涓€銆?
娉ㄦ剰锛屽鏋滃瘑鐮佸疄鐜拌姹傛暟鎹€傚綋瀵归綈锛岃皟鐢ㄨ€呭簲浣跨敤 crypto_skcipher_alignmask() 鍑芥暟鏉ヨ瘑鍒唴瀛樺榻愭帺鐮併€傚唴鏍?crypto API 鑳藉澶勭悊鏈榻愮殑璇锋眰銆備絾杩欎篃鎰忓懗鐫€浼氬甫鏉ラ澶栫殑寮€閿€锛屽洜涓哄唴鏍?crypto API 闇€瑕侀噸鏂板榻愭暟鎹紝杩欏彲鑳芥秹鍙婃暟鎹殑绉诲姩銆?
#### 浣跨敤 struct skcipher_alg 瀹氫箟瀵嗙爜


Struct skcipher_alg 瀹氫箟涓€涓鍧楀瘑鐮侊紝鎴栨洿涓€鑸湴璇达紝涓€涓繚鎸侀暱搴︾殑瀵圭О瀵嗙爜绠楁硶銆?
#### 鍒嗘暎鍒楄〃澶勭悊


鏌愪簺椹卞姩甯屾湜浣跨敤 Generic ScatterWalk锛堥€氱敤鍒嗘暎娓歌蛋锛夛紝浠ラ槻纭欢闇€瑕佽鍠傚叆鍒嗘暎鍒楄〃涓寘鍚槑鏂囥€佸苟灏嗗寘鍚瘑鏂囩殑鐙珛鍧椼€傝鍙傝€?Linux 鍐呮牳鍒嗘暎/鑱氶泦锛坰catter / gather锛夊垪琛ㄥ疄鐜版彁渚涚殑 ScatterWalk 鎺ュ彛銆?
### 鍝堝笇 [HASH]


鍙樻崲绀轰緥锛歝rc32銆乵d5銆乻ha1銆乻ha256 绛?
#### 娉ㄥ唽涓庢敞閿€鍙樻崲


鏍规嵁鍙樻崲鏄悓姝ョ殑 [SHASH] 杩樻槸寮傛鐨?[AHASH]锛屼互鍙婃垜浠娉ㄥ唽澶氬皯 HASH 鍙樻崲锛屾湁澶氱鏂瑰紡娉ㄥ唽 HASH 鍙樻崲銆備綘鍙互鍦?include/crypto/internal/hash.h 涓壘鍒板師鍨嬪畾涔夛細

```
       int crypto_register_ahash(struct ahash_alg *alg);

       int crypto_register_shash(struct shash_alg *alg);
       int crypto_register_shashes(struct shash_alg *algs, int count);
```
娉ㄩ攢 HASH 鍙樻崲鐨勫搴斿嚱鏁板涓嬶細

```
       void crypto_unregister_ahash(struct ahash_alg *alg);

       void crypto_unregister_shash(struct shash_alg *alg);
       void crypto_unregister_shashes(struct shash_alg *algs, int count);
```
#### 浣跨敤 struct shash_alg 鍜?ahash_alg 瀹氫箟瀵嗙爜


浠ヤ笅鏄綋杩欎簺鍑芥暟浠庡唴鏍稿叾浠栭儴鍒嗚璋冪敤鏃剁殑绀烘剰銆傛敞鎰?.setkey() 璋冪敤鍙兘鍙戠敓鍦ㄨ繖浜涚ず鎰忎箣鍓嶆垨涔嬪悗锛屼絾涓嶅緱鍦ㄨ繖浜涚ず鎰忚繘琛屾湡闂村彂鐢熴€傝娉ㄦ剰锛屽厛璋冪敤 .init() 鐒跺悗绱ф帴鐫€璋冪敤 .final() 鍚屾牱鏄竴娆″畬鍏ㄥ悎娉曠殑鍙樻崲銆?
```
       I)   DATA -----------.
                            v
             .init() -> .update() -> .final()      ! .update() 鍦ㄦ鍦烘櫙涓?                         ^    |         |            鍙兘鏍规湰涓嶄細琚皟鐢ㄣ€?                         '----'         '---> HASH

       II)  DATA -----------.-----------.
                            v           v
             .init() -> .update() -> .finup()      ! .update() 鍦ㄦ鍦烘櫙涓?                         ^    |         |            鍙兘鏍规湰涓嶄細琚皟鐢ㄣ€?                         '----'         '---> HASH

       III) DATA -----------.
                            v
                        .digest()                  ! 鏁翠釜杩囩▼鐢?                            |                        .digest() 璋冪敤澶勭悊銆?                            '---------------> HASH
```
浠ヤ笅鏄?.export()/.import() 鍑芥暟浠庡唴鏍稿叾浠栭儴鍒嗚璋冪敤鏃剁殑绀烘剰銆?
```
       KEY--.                 DATA--.
            v                       v                  ! .update() 鍦ㄦ鍦烘櫙涓?        .setkey() -> .init() -> .update() -> .export()   鍙兘鏍规湰涓嶄細琚皟鐢ㄣ€?                                 ^     |         |
                                 '-----'         '--> PARTIAL_HASH

       ----------- 姝ゅ鍙戠敓鍏朵粬鍙樻崲 -----------

       PARTIAL_HASH--.   DATA1--.
                     v          v
                 .import -> .update() -> .final()     ! .update() 鍦ㄦ鍦烘櫙涓?                             ^    |         |           鍙兘鏍规湰涓嶄細琚皟鐢ㄣ€?                             '----'         '--> HASH1

       PARTIAL_HASH--.   DATA2-.
                     v         v
                 .import -> .finup()
                               |
                               '---------------> HASH2
```
璇锋敞鎰忥紝鈥滄斁寮冣€濅竴涓姹傚璞℃槸瀹屽叏鍚堟硶鐨勶細
- 璋冪敤 .init()锛岀劧鍚庯紙澶氭锛夎皟鐢?.update()
- 鍦ㄥ皢鏉ヤ换浣曟椂鍊欓兘**涓?*璋冪敤 .final()銆?finup() 鎴?.export() 涓殑浠讳綍涓€涓?
鎹㈣█涔嬶紝瀹炵幇搴旈【鍙婅祫婧愬垎閰嶄笌娓呯悊銆?涓庤姹傚璞＄浉鍏崇殑璧勬簮涓嶅簲鍦ㄨ皟鐢?.init() 鎴?.update() 涔嬪悗浠嶄繚鎸佸垎閰嶇姸鎬侊紝鍥犱负鍙兘鍐嶄篃娌℃湁鏈轰細閲婃斁瀹冧滑銆?
#### 寮傛 HASH 鍙樻崲鐨勭粏鑺?

鏌愪簺椹卞姩甯屾湜浣跨敤 Generic ScatterWalk锛屼互闃插疄鐜伴渶瑕佽鍠傚叆鍖呭惈杈撳叆鏁版嵁鐨勫垎鏁ｅ垪琛ㄧ殑鐙珛鍧椼€?