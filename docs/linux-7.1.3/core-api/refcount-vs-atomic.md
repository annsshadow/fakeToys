## refcount_t API 涓?atomic_t 鐨勫姣?

## 绠€浠?

refcount_t API 鐨勭洰鏍囨槸鎻愪緵涓€涓渶灏忕殑 API锛岀敤浜庡疄鐜板璞＄殑寮曠敤璁℃暟銆傝櫧鐒?lib/refcount.c 涓€氱敤鐨勩€佷笌鏋舵瀯鏃犲叧鐨勫簳灞傚疄鐜颁娇鐢ㄤ簡鍘熷瓙鎿嶄綔锛屼絾鏌愪簺
`refcount_*()` 涓?`atomic_*()` 鍑芥暟鍦ㄥ唴瀛橀『搴忥紙memory ordering锛変繚璇佹柟闈?瀛樺湪鑻ュ共宸紓銆傛湰鏂囨。姒傝堪杩欎簺宸紓骞剁粰鍑虹浉搴旂ず渚嬶紝浠ュ府鍔╃淮鎶よ€呴拡瀵硅繖浜涘唴瀛?椤哄簭淇濊瘉鐨勫彉鍖栨牎楠屼粬浠殑浠ｇ爜銆?
鏈枃妗ｄ腑浣跨敤鐨勬湳璇瘯鍥鹃伒寰?tools/memory-model/Documentation/explanation.txt
涓畾涔夌殑姝ｅ紡 LKMM銆?
memory-barriers.txt 鍜?atomic_t.txt 鎻愪緵浜嗗叧浜庡唴瀛橀『搴忥紙鎬讳綋锛変互鍙婂師瀛愭搷浣?锛堝叿浣擄級鐨勬洿澶氳儗鏅俊鎭€?
## 鐩稿叧鐨勫唴瀛橀『搴忕被鍨?

鏈枃妗ｄ腑涓庤鎻愬強鐨勫師瀛愭搷浣滃拰寮曠敤璁℃暟鐩稿叧銆佸苟璐┛浣跨敤鐨勫唴瀛橀『搴忕被鍨嬨€傛洿
鍏ㄩ潰鐨勫浘鏅鍙傝€?memory-barriers.txt 鏂囨。銆?
鍦ㄦ病鏈変换浣曞唴瀛橀『搴忎繚璇侊紙鍗冲畬鍏ㄦ棤搴忥級鐨勬儏鍐典笅锛宎tomics 鍜?refcounters 鍙?鎻愪緵鍘熷瓙鎬у拰绋嬪簭椤哄簭锛坧o锛夊叧绯伙紙鍦ㄥ悓涓€ CPU 涓婏級銆傚畠淇濊瘉姣忎釜 `atomic_**()`
鍜?`refcount_**()` 鎿嶄綔閮芥槸鍘熷瓙鐨勶紝涓旀寚浠ゅ湪鍚屼竴 CPU 涓婃寜绋嬪簭椤哄簭鎵ц銆傝繖
鏄娇鐢?READ_ONCE()/WRITE_ONCE() 浠ュ強姣旇緝骞朵氦鎹紙compare-and-swap锛夊師璇?瀹炵幇鐨勩€?
寮猴紙瀹屾暣锛夊唴瀛橀『搴忎繚璇侊細鍚屼竴 CPU 涓婃墍鏈夊厛鍓嶇殑鍔犺浇鍜屽瓨鍌紙鎵€鏈?po 鏇存棭鐨?鎸囦护锛夐兘鍦ㄤ换浣?po 鏇存櫄鐨勬寚浠や簬鍚屼竴 CPU 涓婃墽琛屼箣鍓嶅畬鎴愩€傚畠杩樹繚璇佸悓涓€ CPU 涓?鎵€鏈?po 鏇存棭鐨勫瓨鍌ㄤ互鍙婃潵鑷叾浠?CPU 鐨勬墍鏈夊凡浼犳挱瀛樺偍锛岄兘蹇呴』鍦ㄤ换浣?po 鏇存櫄
鐨勬寚浠や簬鍘熷 CPU 涓婃墽琛屼箣鍓嶄紶鎾埌鎵€鏈夊叾浠?CPU锛圓-cumulative 灞炴€э級銆傝繖鏄?浣跨敤 smp_mb() 瀹炵幇鐨勩€?
RELEASE 鍐呭瓨椤哄簭淇濊瘉锛氬悓涓€ CPU 涓婃墍鏈夊厛鍓嶇殑鍔犺浇鍜屽瓨鍌紙鎵€鏈?po 鏇存棭鐨勬寚浠わ級
閮藉湪璇ユ搷浣滀箣鍓嶅畬鎴愩€傚畠杩樹繚璇佸悓涓€ CPU 涓婃墍鏈?po 鏇存棭鐨勫瓨鍌ㄤ互鍙婃潵鑷叾浠?CPU
鐨勬墍鏈夊凡浼犳挱瀛樺偍锛岄兘蹇呴』鍦?release 鎿嶄綔涔嬪墠浼犳挱鍒版墍鏈夊叾浠?CPU锛圓-cumulative
灞炴€э級銆傝繖鏄娇鐢?smp_store_release() 瀹炵幇鐨勩€?
ACQUIRE 鍐呭瓨椤哄簭淇濊瘉锛氬悓涓€ CPU 涓婃墍鏈夊悗缁殑鍔犺浇鍜屽瓨鍌紙鎵€鏈?po 鏇存櫄鐨勬寚浠わ級
閮藉湪 acquire 鎿嶄綔涔嬪悗瀹屾垚銆傚畠杩樹繚璇佸悓涓€ CPU 涓婃墍鏈?po 鏇存櫄鐨勫瓨鍌ㄩ兘蹇呴』鍦?acquire 鎿嶄綔鎵ц涔嬪悗浼犳挱鍒版墍鏈夊叾浠?CPU銆傝繖鏄娇鐢?smp_acquire__after_ctrl_dep()
瀹炵幇鐨勩€?
寮曠敤璁℃暟鐨勬帶鍒朵緷璧栵紙鎴愬姛鏃讹級淇濊瘉锛氬鏋滄垚鍔熻幏鍙栦簡瀵硅薄鐨勫紩鐢紙寮曠敤璁℃暟鍙戠敓
浜嗛€掑鎴栧姞娉曪紝鍑芥暟杩斿洖 true锛夛紝鍒欏悗缁殑瀛樺偍閮戒笌姝ゆ搷浣滄湁搴忋€傚瓨鍌ㄤ笂鐨勬帶鍒?渚濊禆涓嶄娇鐢ㄤ换浣曟樉寮忓睆闅滃疄鐜帮紝鑰屾槸渚濊禆 CPU 涓嶄細瀵瑰瓨鍌ㄨ繘琛屾帹娴嬫墽琛屻€傝繖浠呮槸涓€
涓崟 CPU 鍏崇郴锛屽鍏朵粬 CPU 涓嶆彁渚涗换浣曚繚璇併€?

## 鍑芥暟瀵规瘮


### 鎯呭舰 1) - 闈炩€滆/淇敼/鍐欌€濓紙RMW锛夋搷浣?

鍑芥暟鍙樺寲锛?
 - atomic_set() --> refcount_set()
 - atomic_read() --> refcount_read()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 鏃狅紙涓よ€呴兘瀹屽叏鏃犲簭锛?

### 鎯呭舰 2) - 甯?release 椤哄簭鐨勯潪鈥滆/淇敼/鍐欌€濓紙RMW锛夋搷浣?

鍑芥暟鍙樺寲锛?
 - atomic_set_release() --> refcount_set_release()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 鏃狅紙涓よ€呴兘鎻愪緵 RELEASE 椤哄簭锛?

### 鎯呭舰 3) - 涓嶈繑鍥炲€肩殑鍩轰簬閫掑鐨勬搷浣?

鍑芥暟鍙樺寲锛?
 - atomic_inc() --> refcount_inc()
 - atomic_add() --> refcount_add()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 鏃狅紙涓よ€呴兘瀹屽叏鏃犲簭锛?
### 鎯呭舰 4) - 涓嶈繑鍥炲€肩殑鍩轰簬閫掑噺鐨?RMW 鎿嶄綔


鍑芥暟鍙樺寲锛?
 - atomic_dec() --> refcount_dec()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏃犲簭 --> RELEASE 椤哄簭


### 鎯呭舰 5) - 杩斿洖鍊笺€佸熀浜庨€掑鐨?RMW 鎿嶄綔


鍑芥暟鍙樺寲锛?
 - atomic_inc_not_zero() --> refcount_inc_not_zero()
 - 鏃犲搴旂殑 atomic 鍑芥暟 --> refcount_add_not_zero()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏈夊簭 --> 鎴愬姛鏃跺瀛樺偍鐨勬帶鍒朵緷璧?

   鍙栧緱瀵硅薄鎸囬拡鐨勭粨鏋滐紒

### 鎯呭舰 6) - 杩斿洖鍊笺€佸甫 acquire 椤哄簭銆佸熀浜庨€掑鐨?RMW 鎿嶄綔


鍑芥暟鍙樺寲锛?
 - atomic_inc_not_zero() --> refcount_inc_not_zero_acquire()
 - 鏃犲搴旂殑 atomic 鍑芥暟 --> refcount_add_not_zero_acquire()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏈夊簭 --> 鎴愬姛鏃剁殑 ACQUIRE 椤哄簭


### 鎯呭舰 7) - 杩斿洖鍊笺€侀€氱敤鐨勫熀浜?dec/sub 閫掑噺鐨?RMW 鎿嶄綔


鍑芥暟鍙樺寲锛?
 - atomic_dec_and_test() --> refcount_dec_and_test()
 - atomic_sub_and_test() --> refcount_sub_and_test()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏈夊簭 --> RELEASE 椤哄簭 + 鎴愬姛鏃剁殑 ACQUIRE 椤哄簭


### 鎯呭舰 8) - 鍏朵粬杩斿洖鍊笺€佸熀浜庨€掑噺鐨?RMW 鎿嶄綔


鍑芥暟鍙樺寲锛?
 - 鏃犲搴旂殑 atomic 鍑芥暟 --> refcount_dec_if_one()
 - `atomic_add_unless(&var, -1, 1)` --> `refcount_dec_not_one(&var)`

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏈夊簭 --> RELEASE 椤哄簭 + 鎺у埗渚濊禆


### 鎯呭舰 9) - 鍩轰簬閿佺殑 RMW


鍑芥暟鍙樺寲锛?
 - atomic_dec_and_lock() --> refcount_dec_and_lock()
 - atomic_dec_and_mutex_lock() --> refcount_dec_and_mutex_lock()

鍐呭瓨椤哄簭淇濊瘉鍙樺寲锛?
 - 瀹屽叏鏈夊簭 --> RELEASE 椤哄簭 + 鎺у埗渚濊禆 + 鎴愬姛鏃舵寔鏈?spin_lock()
