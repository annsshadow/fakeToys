
## Kbuild 鐨?Bash 琛ュ叏


鍐呮牳鏋勫缓绯荤粺鏄敤 Makefile 缂栧啓鐨勶紝鑰?`make` 鍛戒护鐨?Bash 琛ュ叏鍙€氳繃
`bash-completion`_ 椤圭洰鑾峰緱銆?
鐒惰€岋紝鍐呮牳鏋勫缓鐨?Makefile 闈炲父澶嶆潅銆俙make` 鍛戒护鐨勯€氱敤琛ュ叏瑙勫垯闄?`make`
鍛戒护鑷韩鐨勯€夐」澶栵紝鏃犳硶涓哄唴鏍告瀯寤虹郴缁熸彁渚涙湁鎰忎箟鐨勫缓璁€?
涓轰簡澧炲己瀵瑰悇绉嶅彉閲忎笌鐩爣鐨勮ˉ鍏紝鍐呮牳婧愮爜鍦ㄥ叾 `scripts/bash-completion/make`
涓寘鍚簡涓€涓嚜宸辩殑琛ュ叏鑴氭湰銆?
璇ヨ剼鏈湪鍐呮牳鏍戝唴宸ヤ綔鏃舵彁渚涢澶栫殑琛ュ叏銆傚湪鍐呮牳鏍戜箣澶栵紝瀹冮粯璁ゅ洖閫€鍒?`make`
鍛戒护鐨勯€氱敤琛ュ叏瑙勫垯銆?
## 鍏堝喅鏉′欢


璇ヨ剼鏈緷璧栦簬 `bash-completion`_ 椤圭洰鎻愪緵鐨勮緟鍔╁嚱鏁般€傝纭繚瀹冨凡瀹夎鍦ㄤ綘鐨?绯荤粺涓娿€傚湪澶у鏁板彂琛岀増涓紝浣犲彲浠ラ€氳繃鏍囧噯鍖呯鐞嗗櫒瀹夎 `bash-completion`
杞欢鍖呫€?
## 濡備綍浣跨敤


```

  $ source scripts/bash-completion/make

```
鎴栬€咃紝浣犲彲浠ュ皢瀹冨鍒跺埌 Bash 琛ュ叏鑴氭湰鐨勬悳绱㈣矾寰勪腑銆?```

  $ mkdir -p ~/.local/share/bash-completion/completions
  $ cp scripts/bash-completion/make ~/.local/share/bash-completion/completions/

```
## 缁嗚妭


鍦ㄤ互涓嬫儏鍐典笅浼氬惎鐢ㄩ拡瀵?Kbuild 鐨勯澶栬ˉ鍏細

 - 浣犲浜庡唴鏍告簮鐮佺殑鏍圭洰褰曘€? - 浣犲浜庣敱 O= 閫夐」鍒涘缓鐨勯《灞傛瀯寤虹洰褰?   锛堥€氳繃鎸囧悜鍐呮牳婧愮爜鐨?`source` 绗﹀彿閾炬帴妫€鏌ワ級銆? - -C make 閫夐」鎸囧畾浜嗗唴鏍告簮鐮佹垨鏋勫缓鐩綍銆? - -f make 閫夐」鎸囧畾浜嗗唴鏍告簮鐮佹垨鏋勫缓鐩綍涓殑鏌愪釜鏂囦欢銆?
濡傛灉浠ヤ笂閮戒笉婊¤冻锛屽垯鍥為€€鍒伴€氱敤琛ュ叏瑙勫垯銆?
琛ュ叏鏀寔锛?
  - 甯哥敤鐩爣锛屼緥濡?`all`銆乣menuconfig`銆乣dtbs` 绛夈€?  - Make锛堟垨鐜锛夊彉閲忥紝渚嬪 `ARCH`銆乣LLVM` 绛夈€?  - 鍗曠洰鏍囨瀯寤猴紙`foo/bar/baz.o`锛?  - 閰嶇疆鏂囦欢锛坄**_defconfig` 涓?`**.config`锛?
涓€浜涘彉閲忔彁渚涙櫤鑳借涓恒€備緥濡傦紝`CROSS_COMPILE=` 鍚庤窡涓€涓?TAB 浼氭樉绀哄凡瀹夎鐨?宸ュ叿閾俱€傛墍鏄剧ず鐨?defconfig 鏂囦欢鍒楄〃鍙栧喅浜?`ARCH=` 鍙橀噺鐨勫€笺€?