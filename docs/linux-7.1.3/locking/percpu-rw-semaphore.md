## Percpu rw 淇″彿閲?


Percpu rw semaphores 鏄竴绉嶆柊鐨勮鍐欎俊鍙烽噺璁捐锛?
閽堝璇诲彇閿佸畾杩涜浜嗕紭鍖栥€?

浼犵粺璇诲啓淇″彿閲忕殑闂鍦ㄤ簬锛屽綋澶氫釜
鏍稿績鑾峰彇璇诲彇閿侊紝鍖呭惈淇″彿閲忕殑缂撳瓨琛?
鍦ㄥ唴鏍哥殑 L1 缂撳瓨涔嬮棿璺宠穬锛屽鑷存€ц兘涓嬮檷
闄嶈В銆?

璇诲彇閿佸畾闈炲父蹇紝瀹冧娇鐢?RCU 骞朵笖閬垮厤浠讳綍鍘熷瓙鎿嶄綔
閿佸畾鍜岃В閿佽矾寰勪腑鐨勬寚浠ゃ€傚彟涓€鏂归潰锛岄攣瀹?
鍐欏叆鏄潪甯告槀璐电殑锛屽畠璋冪敤synchronize_rcu()锛屽彲浠ラ噰鍙?
鏁扮櫨姣銆?

璇ラ攣浠モ€渟truct percpu_rw_semaphore鈥濈被鍨嬪０鏄庛€?
閿侀€氳繃 percpu_init_rwsem 鍒濆鍖栵紝鎴愬姛鏃惰繑鍥?0
鍜?-ENOMEM 鍒嗛厤澶辫触銆?
蹇呴』浣跨敤 percpu_free_rwsem 閲婃斁閿佷互閬垮厤鍐呭瓨娉勬紡銆?

璇ラ攣閫氳繃 percpu_down_read銆乸ercpu_up_read 鍜?percpu_down_read 閿佸畾浠ヨ繘琛岃鍙?
鐢ㄤ簬浣跨敤 percpu_down_write銆乸ercpu_up_write 杩涜鍐欏叆銆?

浣跨敤 RCU 鏉ヤ紭鍖?rw-lock 鐨勬兂娉曟槸鐢?
鍩冮噷鍏嬄锋潨椹鐗?eric.dumazet@gmail.com>銆?
浠ｇ爜鐢?Mikulas Patocka <mpatocka@redhat.com> 缂栧啓
