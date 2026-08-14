### Verity 鏂囦欢


ext4 鏀寔 fs-verity锛岃繖鏄竴绉嶆枃浠剁郴缁熺壒鎬э紝涓哄崟涓彧璇绘枃浠舵彁渚涘熀浜?Merkle 鏍戠殑
鍝堝笇銆俧s-verity 鐨勫ぇ閮ㄥ垎鍐呭瀵规墍鏈夋敮鎸佸畠鐨勬枃浠剁郴缁熸槸閫氱敤鐨勶紱鏈夊叧 fs-verity
鏂囨。璇峰弬闃?Documentation/filesystems/fsverity.rst <fsverity>銆備絾鏄紝verity
鍏冩暟鎹殑纾佺洏甯冨眬鏄枃浠剁郴缁熺壒瀹氱殑銆傚湪 ext4 涓婏紝verity 鍏冩暟鎹瓨鍌ㄥ湪鏂囦欢鏁版嵁
鏈韩鏈熬涔嬪悗锛屾牸寮忓涓嬶細

- 闆跺～鍏呭埌涓嬩竴涓?65536 瀛楄妭杈圭晫銆傛濉厖瀹為檯涓婃棤闇€鍦ㄧ鐩樹笂鍒嗛厤锛屽嵆瀹冨彲浠ユ槸涓€涓?  绌烘礊銆?
- Merkle 鏍戯紝濡?:ref:`Documentation/filesystems/fsverity.rst
  <fsverity_merkle_tree>` 涓墍杩帮紝鏍戠殑灞傜骇鎸変粠鏍瑰埌鍙剁殑椤哄簭瀛樺偍锛屾瘡涓眰绾у唴鐨?  鏍戝潡鎸夎嚜鐒堕『搴忓瓨鍌ㄣ€?
- 闆跺～鍏呭埌涓嬩竴涓枃浠剁郴缁熷潡杈圭晫銆?
- verity 鎻忚堪绗︼紝濡?Documentation/filesystems/fsverity.rst <fsverity_descriptor>
  涓墍杩帮紝鍙€夋嫨鎬у湴闄勫姞绛惧悕 blob銆?
- 闆跺～鍏呭埌鏂囦欢绯荤粺鍧楄竟鐣屽墠 4 瀛楄妭鐨勪笅涓€涓亸绉诲銆?
- verity 鎻忚堪绗︾殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級锛屼负涓€涓?4 瀛楄妭灏忕鏁存暟銆?
Verity inode 璁剧疆浜?EXT4_VERITY_FL锛屽苟涓斿畠浠繀椤讳娇鐢?extent锛屽嵆蹇呴』璁剧疆
EXT4_EXTENTS_FL 涓斿繀椤绘竻闄?EXT4_INLINE_DATA_FL銆傚畠浠彲浠ヨ缃?EXT4_ENCRYPT_FL锛?姝ゆ椂 verity 鍏冩暟鎹笌鏁版嵁鏈韩涓€璧疯鍔犲瘑銆?
Verity 鏂囦欢涓嶈兘鍦?verity 鍏冩暟鎹湯灏句箣鍚庡垎閰嶅潡銆?
Verity 涓?DAX 涓嶅吋瀹癸紝璇曞浘鍦ㄦ枃浠朵笂鍚屾椂璁剧疆杩欎袱涓爣蹇楀皢澶辫触銆?