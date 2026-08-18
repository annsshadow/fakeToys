
## 鍑芥暟閲嶅畾鍚?API


## 姒傝堪


缂栧啓鍗曞厓娴嬭瘯鏃讹紝鑳藉灏嗗緟娴嬩唬鐮佷笌鍐呮牳鍏跺畠閮ㄥ垎闅旂寮€鏉ユ槸寰堥噸瑕佺殑銆傝繖淇濊瘉浜嗘祴璇曠殑鍙潬鎬?
锛堜笉浼氬彈澶栭儴鍥犵礌褰卞搷锛夈€佸噺灏戝鐗瑰畾纭欢鎴栭厤缃€夐」鐨勪緷璧栵紙浣挎祴璇曟洿瀹规槗杩愯锛夛紝骞朵繚鎶?
绯荤粺鍏朵綑閮ㄥ垎鐨勭ǔ瀹氭€э紙闄嶄綆娴嬭瘯鐗瑰畾鐘舵€佸共鎵扮郴缁熷叾浣欓儴鍒嗙殑鍙兘鎬э級銆?

瀵逛簬鏌愪簺浠ｇ爜锛堥€氬父鏄€氱敤鏁版嵁缁撴瀯銆佽緟鍔╁嚱鏁颁互鍙婂叾瀹?绾嚱鏁?锛夋潵璇磋繖寰堝鏄擄紝浣嗗浜?
鍏跺畠浠ｇ爜锛堝璁惧椹卞姩銆佹枃浠剁郴缁熴€佹牳蹇冨瓙绯荤粺锛夋潵璇达紝浠ｇ爜涓庡唴鏍稿叾瀹冮儴鍒嗛珮搴﹁€﹀悎銆?

杩欑鑰﹀悎寰€寰€浠ユ煇绉嶆柟寮忔簮浜庡叏灞€鐘舵€侊細鏃犺鏄澶囩殑鍏ㄥ眬鍒楄〃銆佹枃浠剁郴缁燂紝杩樻槸鏌愪簺纭欢鐘舵€併€?
娴嬭瘯闇€瑕佸皬蹇冨湴绠＄悊銆侀殧绂诲拰鎭㈠鐘舵€侊紝鎴栬€呬篃鍙互閫氳繃鐢?鍋?锛坒ake锛夋垨"妯℃嫙"锛坢ock锛?
鍙樹綋鏇挎崲瀵硅鐘舵€佺殑璁块棶鍜屼慨鏀规潵瀹屽叏閬垮紑瀹冦€?

閫氳繃閲嶆瀯瀵规绫荤姸鎬佺殑璁块棶锛堜緥濡傚紩鍏ヤ竴灞傞棿鎺ュ眰锛岃灞傚彲浠ヤ娇鐢ㄦ垨妯℃嫙涓€缁勭嫭绔嬬殑娴嬭瘯鐘舵€侊級
涔熻兘鍋氬埌銆傜劧鑰岋紝杩欐牱鐨勯噸鏋勬湰韬湁浠ｄ环锛堝湪鑳藉啓娴嬭瘯涔嬪墠杩涜閲嶅ぇ閲嶆瀯骞朵笉鐞嗘兂锛夈€?

涓€绉嶆洿绠€鍗曠殑鎷︽埅鍜屾浛鎹㈡煇浜涘嚱鏁拌皟鐢ㄧ殑鏂瑰紡鏄娇鐢ㄥ熀浜庨潤鎬佹々锛坰tatic stub锛夌殑鍑芥暟閲嶅畾鍚戙€?


## 闈欐€佹々


闈欐€佹々鏄竴绉嶅皢涓€涓嚱鏁帮紙"鐪熷疄"鍑芥暟锛夌殑璋冪敤閲嶅畾鍚戝埌鍙︿竴涓嚱鏁帮紙"鏇挎崲"鍑芥暟锛夌殑鏂规硶銆?

瀹冪殑宸ヤ綔鍘熺悊鏄悜"鐪熷疄"鍑芥暟涓坊鍔犱竴涓畯锛岃瀹忔鏌ユ槸鍚︽湁娴嬭瘯姝ｅ湪杩愯锛屼互鍙婃槸鍚﹀瓨鍦ㄥ彲鐢ㄧ殑
鏇挎崲鍑芥暟銆傚鏋滃瓨鍦紝灏变細璋冪敤璇ュ嚱鏁颁互浠ｆ浛鍘熷嚱鏁般€?

浣跨敤闈欐€佹々鐩稿綋鐩存帴锛?

1. 灏?KUNIT_STATIC_STUB_REDIRECT() 瀹忔坊鍔犲埌"鐪熷疄"鍑芥暟鐨勫紑澶淬€?

   杩欏簲璇ユ槸鍑芥暟涓揣闅忎换浣曞彉閲忓０鏄庝箣鍚庣殑绗竴鏉¤鍙ャ€侹UNIT_STATIC_STUB_REDIRECT() 鎺ュ彈
   鍑芥暟鍚嶏紝鍚庤窡浼犻€掔粰鐪熷疄鍑芥暟鐨勬墍鏈夊弬鏁般€?

   渚嬪锛?

   .. code-block:: c

   void send_data_to_hardware(const char *str)
   {
   	KUNIT_STATIC_STUB_REDIRECT(send_data_to_hardware, str);
   	/** 鐪熷疄瀹炵幇 **/
   }

2. 缂栧啓涓€涓垨澶氫釜鏇挎崲鍑芥暟銆?

   杩欎簺鍑芥暟搴斿綋鍏锋湁涓庣湡瀹炲嚱鏁扮浉鍚岀殑鍑芥暟绛惧悕銆傚鏋滃畠浠渶瑕佽闂垨淇敼娴嬭瘯鐗瑰畾鐘舵€侊紝鍙互
   浣跨敤 kunit_get_current_test() 鑾峰彇涓€涓?struct kunit 鎸囬拡銆傜劧鍚庡彲浠ュ皢鍏朵紶缁欐湡鏈?鏂█
   瀹忥紝鎴栫敤浜庢煡鎵?KUnit 璧勬簮銆?

   渚嬪锛?

   .. code-block:: c

   void fake_send_data_to_hardware(const char *str)
   {
   	struct kunit *test = kunit_get_current_test();
   	KUNIT_EXPECT_STREQ(test, str, "Hello World!");
   }

3. 浠庝綘鐨勬祴璇曚腑婵€娲婚潤鎬佹々銆?

   鍦ㄦ祴璇曞唴閮紝鍙互浣跨敤 kunit_activate_static_stub() 鍚敤閲嶅畾鍚戯紝瀹冩帴鍙椾竴涓?struct kunit
   鎸囬拡銆佺湡瀹炲嚱鏁板拰鏇挎崲鍑芥暟銆備綘鍙互鐢ㄤ笉鍚岀殑鏇挎崲鍑芥暟澶氭璋冪敤瀹冿紝浠ヤ氦鎹㈣鍑芥暟鐨勫疄鐜般€?

   鍦ㄦ垜浠殑渚嬪瓙涓紝杩欏皢鏄?

   .. code-block:: c

   kunit_activate_static_stub(test,
   			   send_data_to_hardware,
   			   fake_send_data_to_hardware);

4. 璋冪敤锛堝彲鑳芥槸闂存帴鍦帮級鐪熷疄鍑芥暟銆?

   涓€鏃︽縺娲讳簡閲嶅畾鍚戯紝浠讳綍瀵圭湡瀹炲嚱鏁扮殑璋冪敤閮戒細鏀逛负璋冪敤鏇挎崲鍑芥暟銆傛绫昏皟鐢ㄥ彲鑳芥繁鍩嬪湪鍙︿竴涓?
   鍑芥暟鐨勫疄鐜颁腑锛屼絾蹇呴』鏉ヨ嚜娴嬭瘯鐨?kthread銆?

   渚嬪锛?

   .. code-block:: c

   send_data_to_hardware("Hello World!"); /** 鎴愬姛 **/
   send_data_to_hardware("Something else"); /** 娴嬭瘯澶辫触銆?**/

5. 锛堝彲閫夛級绂佺敤璇ユ々銆?

   褰撲笉鍐嶉渶瑕佸畠鏃讹紝浣跨敤 kunit_deactivate_static_stub() 绂佺敤閲嶅畾鍚戯紙浠庤€屾仮澶?鐪熷疄"鍑芥暟鐨?
   鍘熷琛屼负锛夈€傚惁鍒欙紝瀹冧細鍦ㄦ祴璇曢€€鍑烘椂鑷姩绂佺敤銆?

   渚嬪锛?

   .. code-block:: c

   kunit_deactivate_static_stub(test, send_data_to_hardware);

涔熷彲浠ュ埄鐢ㄨ繖浜涙浛鎹㈠嚱鏁版潵娴嬭瘯鏌愪釜鍑芥暟鏄惁琚皟鐢ㄨ繃锛屼緥濡傦細

   void send_data_to_hardware(const char *str)
   {
   	KUNIT_STATIC_STUB_REDIRECT(send_data_to_hardware, str);
   	/** 鐪熷疄瀹炵幇 **/
   }

   /** 鍦ㄦ祴璇曟枃浠朵腑 **/
   int times_called = 0;
   void fake_send_data_to_hardware(const char *str)
   {
   	times_called++;
   }
   ...
   /** 鍦ㄦ祴璇曠敤渚嬩腑锛屽湪娴嬭瘯鏈熼棿閲嶅畾鍚戣皟鐢?**/
   kunit_activate_static_stub(test, send_data_to_hardware, fake_send_data_to_hardware);

   send_data_to_hardware("hello");
   KUNIT_EXPECT_EQ(test, times_called, 1);

   /** 濡傛灉闇€瑕侊紝涔熷彲浠ユ彁鍓嶅仠鐢ㄨ妗?**/
   kunit_deactivate_static_stub(test, send_data_to_hardware);


   send_data_to_hardware("hello again");
   KUNIT_EXPECT_EQ(test, times_called, 1);


## API 鍙傝€?

   :internal:

	send_data_to_hardware("hello");
	KUNIT_EXPECT_EQ(test, times_called, 1);

	/** Can also deactivate the stub early, if wanted **/
	kunit_deactivate_static_stub(test, send_data_to_hardware);

	send_data_to_hardware("hello again");
	KUNIT_EXPECT_EQ(test, times_called, 1);



## API Reference


   :internal:
