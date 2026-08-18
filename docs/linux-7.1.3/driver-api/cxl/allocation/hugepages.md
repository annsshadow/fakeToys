## 澶ч〉锛圚uge Pages锛?


## 杩炵画鍐呭瓨鍒嗛厤鍣紙Contiguous Memory Allocator锛?


鍦ㄦ棭鏈熷惎鍔ㄩ樁娈典綔涓?SystemRAM 涓婄嚎鐨?CXL 鍐呭瓨鍙敤浜?CMA锛屽洜涓哄湪 CMA 鍒掑垎鍑鸿繛缁閲忔椂锛屾壙杞借瀹归噺鐨?NUMA 鑺傜偣澶勪簬 `Online` 鐘舵€併€?

寤惰繜鍒?CXL 椹卞姩杩涜閰嶇疆鐨?CXL 鍐呭瓨锛屽叾瀹归噺鏃犳硶鐢?CMA 鍒嗛厤鈥斺€斿洜涓哄湪 CMA 鍒掑垎鍑鸿繛缁閲忔椂锛堝嵆 `__init` 鏃跺埢锛夛紝鎵胯浇璇ュ閲忕殑 NUMA 鑺傜偣澶勪簬 `Offline` 鐘舵€併€?

## HugeTLB


涓嶅悓鐨勫ぇ椤靛昂瀵稿厑璁镐笉鍚岀殑鍐呭瓨閰嶇疆銆?

### 2MB 澶ч〉

鏃犺閰嶇疆鏃堕棿鎴栧唴瀛樺尯鍩燂紙zone锛夊浣曪紝鎵€鏈?CXL 瀹归噺閮藉彲鐢ㄤ簬 2MB 澶ч〉銆?

### 1GB 澶ч〉

鍦?`ZONE_NORMAL` 涓笂绾跨殑 CXL 瀹归噺鍙敤浜?1GB 宸ㄥ瀷椤碉紙Gigantic Page锛夊垎閰嶃€?

鍦?`ZONE_MOVABLE` 涓笂绾跨殑 CXL 瀹归噺涓嶈兘鐢ㄤ簬 1GB 宸ㄥ瀷椤靛垎閰嶃€?
