
## Sparse


Sparse 鏄?C 绋嬪簭鐨勮涔夋鏌ュ櫒锛涘畠鍙互鐢ㄦ潵鍙戠幇鍐呮牳浠ｇ爜涓殑鑻ュ共娼滃湪闂銆傚叧浜?sparse 鐨勬杩拌鍙傝 https://lwn.net/Articles/689907/锛涙湰鏂囨。鍖呭惈涓€浜涘唴鏍哥浉鍏崇殑
sparse 淇℃伅銆傛洿澶氬叧浜?sparse 鐨勪俊鎭紙涓昏鍏充簬鍏跺唴閮ㄥ疄鐜帮級鍙互鍦ㄥ叾瀹樻柟椤甸潰
https://sparse.docs.kernel.org 鎵惧埌銆?

### 浣跨敤 sparse 杩涜绫诲瀷妫€鏌?

```

        typedef int __bitwise pm_request_t;

        enum pm_request {
                PM_SUSPEND = (__force pm_request_t) 1,
                PM_RESUME = (__force pm_request_t) 2
        };

```
杩欎娇寰?PM_SUSPEND 涓?PM_RESUME 鎴愪负鈥渂itwise鈥濇暣鏁帮紙杩欓噷鐨?"__force" 鏄洜涓?sparse
浼氭姳鎬ㄥ悜/浠?bitwise 绫诲瀷杩涜寮哄埗杞崲锛屼絾鍦ㄦ渚嬩腑鎴戜滑纭疄_鎯宠_寮哄埗杞崲锛夈€傚苟涓斿洜涓?鏋氫妇鍊奸兘鏄悓涓€绫诲瀷锛岀幇鍦?"enum pm_request" 涔熶細鏄偅涓被鍨嬨€?
鑰屽浜?gcc锛屾墍鏈夌殑 "__bitwise"/"__force 閭ｄ簺涓滆タ" 閮戒細娑堝け锛屽 gcc 鏉ヨ瀹冧滑鏈€缁?鐪嬭捣鏉ュ氨鍙槸鏅€氱殑鏁存暟銆?
鍧︾櫧璇达紝浣犲苟涓嶉渶瑕侀偅閲岀殑鏋氫妇銆備笂闈㈢殑鍐呭瀹為檯涓婇兘鍙互褰掔粨涓轰竴绉嶇壒娈婄殑
"int __bitwise" 绫诲瀷銆?
```

        typedef int __bitwise pm_request_t;

        #define PM_SUSPEND ((__force pm_request_t) 1)
        #define PM_RESUME ((__force pm_request_t) 2)

```
鐜板湪浣犲氨鎷ユ湁浜嗚繘琛屼弗鏍肩被鍨嬫鏌ユ墍闇€鐨勫叏閮ㄥ熀纭€璁炬柦銆?
涓€涓皬鎻愮ず锛氬父閲忔暣鏁?"0" 鏄壒娈婄殑銆備綘鍙互鍦ㄤ笉寮曞彂 sparse 浠讳綍鎶辨€ㄧ殑鎯呭喌涓嬶紝灏?甯搁噺 0 鐢ㄤ綔 bitwise 鏁存暟绫诲瀷銆傝繖鏄洜涓?"bitwise"锛堥【鍚嶆€濅箟锛夎璁捐鐢ㄦ潵纭繚 bitwise
绫诲瀷涓嶄細琚贩娣嗭紙灏忕 vs 澶х vs cpu 绔?vs 鍏跺畠锛夛紝鑰屽湪閭ｉ噷甯搁噺 "0" 纭疄_鏄痏鐗规畩鐨勩€?
### 鑾峰彇 sparse


浣犲彲浠ヤ粠浠ヤ笅鍦板潃鑾峰彇鏈€鏂板彂甯冪増鏈殑 tarball锛?https://www.kernel.org/pub/software/devel/sparse/dist/

鍙﹀锛屼綘鍙互鑾峰彇鏈€鏂板紑鍙戠増鏈殑蹇収

```

        git://git.kernel.org/pub/scm/devel/sparse/sparse.git

```
```

        make
        make install

```
浣滀负鏅€氱敤鎴凤紝瀹冧細灏?sparse 瀹夎鍒颁綘鐨?~/bin 鐩綍涓嬨€?
### 浣跨敤 sparse


鎵ц "make C=1" 鐨勫唴鏍哥紪璇戯紝鍙閲嶆柊缂栬瘧鐨勬墍鏈?C 鏂囦欢杩愯 sparse锛涙垨鑰呬娇鐢?"make C=2" 瀵规枃浠惰繍琛?sparse锛屾棤璁哄畠浠槸鍚﹂渶瑕侀噸鏂扮紪璇戙€傚鏋滀綘宸茬粡鏋勫缓杩囨暣涓?浠ｇ爜鏍戯紝鍚庤€呮槸妫€鏌ユ暣妫垫爲鐨勪竴绉嶅揩閫熸柟寮忋€?
鍙€夌殑 make 鍙橀噺 CF 鍙敤浜庡悜 sparse 浼犻€掑弬鏁般€傛瀯寤虹郴缁熶細鑷姩鍚?sparse 浼犻€?-Wbitwise銆?
娉ㄦ剰锛宻parse 瀹氫箟浜?__CHECKER__ 棰勫鐞嗗畯銆?