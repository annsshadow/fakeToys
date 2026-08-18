
## IPsec


姝ゅ璁板綍宸茬煡鐨?IPsec 杈圭晫鎯呭喌锛屽湪鐪熷疄鐢熶骇鐜涓儴缃插悇绉?IPsec 閰嶇疆鏃堕渶瑕佺墷璁般€?
1. IPcomp:
	   杈冨皬鐨?IP 鎶ユ枃鍦ㄥ彂閫佺涓嶄細琚帇缂╋紝骞跺湪鎺ユ敹绔殑绛栫暐妫€鏌ヤ腑澶辫触銆?
```

  2.2. 闈炴墿灞曠瓥鐣?
   If the total size of a compressed payload and the IPComp header, as
   defined in section 3, is not smaller than the size of the original
   payload, the IP datagram MUST be sent in the original non-compressed
   form.  To clarify: If an IP datagram is sent non-compressed, no

   IPComp header is added to the datagram.  This policy ensures saving
   the decompression processing cycles and avoiding incurring IP
   datagram fragmentation when the expanded datagram is larger than the
   MTU.

   Small IP datagrams are likely to expand as a result of compression.
   Therefore, a numeric threshold should be applied before compression,
   where IP datagrams of size smaller than the threshold are sent in the
   original form without attempting compression.  The numeric threshold
   is implementation dependent.

```
褰撳墠鐨勫疄鐜扮‘瀹炰弗鏍奸伒寰鑼冿紝浣嗗湪瀹為檯涓紝褰撳悜瀵圭鍙戦€佹湭鍘嬬缉鎶ユ枃鏃讹紙鏃犺鎶ユ枃闀垮害鏄惁灏忎簬闃堝€硷紝鎴栧帇缂╁悗闀垮害澶т簬鍘熷鎶ユ枃闀垮害锛夛紝璇ユ姤鏂囧湪绛栫暐妫€鏌ュ琚涪寮冿紝鍥犱负瀹冨尮閰嶉€夋嫨鍣ㄤ絾骞堕潪鏉ヨ嚜浠讳綍 XFRM 灞傦紝鍗虫病鏈夊畨鍏ㄨ矾寰勩€傝繖绉嶈８鎶ユ枃鏈€缁堟棤娉曢€佽揪涓婂眰銆?褰撶敤鎴蜂娇鐢ㄤ笉鍚岃浇鑽烽暱搴?ping 瀵圭鏃讹紝缁撴灉瀵圭敤鎴锋潵璇存洿鍔犺寮傘€?
涓€绉嶅彉閫氭柟娉曟槸鍦ㄨ瀵熷埌涓婅堪鍦烘櫙鏃讹紝涓烘瘡涓瓥鐣ュ皾璇曡缃€渓evel use鈥濄€傝繖鏍峰仛鐨勭粨鏋滄槸灏忔姤鏂囷紙鏈帇缂╋級灏嗗湪鎺ユ敹绔烦杩囩瓥鐣ユ鏌ャ€?