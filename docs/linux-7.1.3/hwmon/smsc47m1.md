## 鍐呮牳椹卞姩 smsc47m1


鏀寔鐨勮姱鐗囷細

  - SMSC LPC47B27x, LPC47M112, LPC47M10x, LPC47M13x, LPC47M14x,

    LPC47M15x 涓?LPC47M192

    鎵弿鍦板潃锛氭棤锛屽湴鍧€浠?Super I/O 閰嶇疆绌洪棿璇诲彇

    Prefix: 'smsc47m1'

    Datasheets:

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47b272.pdf

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47m10x.pdf

	http://www.smsc.com/media/Downloads_Public/Data_Sheets/47m112.pdf

	http://www.smsc.com/

  - SMSC LPC47M292

    鎵弿鍦板潃锛氭棤锛屽湴鍧€浠?Super I/O 閰嶇疆绌洪棿璇诲彇

    Prefix: 'smsc47m2'

    Datasheet: 涓嶅叕寮€

  - SMSC LPC47M997

    鎵弿鍦板潃锛氭棤锛屽湴鍧€浠?Super I/O 閰嶇疆绌洪棿璇诲彇

    Prefix: 'smsc47m1'

    Datasheet: 鏃?

Authors:

     - Mark D. Studebaker <mdsxyz123@yahoo.com>,
     - 鍦?Bruce Allen <ballen@uwm.edu> 鍙婂叾 fan.c 绋嬪簭鐨勫崗鍔╀笅锛?
       - http://www.lsc-group.phys.uwm.edu/%7Eballen/driver/

     - Gabriele Gorla <gorlik@yahoo.com>,
     - Jean Delvare <jdelvare@suse.de>

### 鎻忚堪


鏍囧噯寰郴缁熷叕鍙革紙SMSC锛夌殑 47M1xx Super I/O 鑺墖鍖呭惈鐢ㄤ簬涓や釜椋庢墖鐨勭洃鎺т笌 PWM 鎺у埗
鐢佃矾銆?
LPC47M15x銆丩PC47M192 涓?LPC47M292 鑺墖闄や簡椋庢墖鐩戞帶涓庢帶鍒跺锛岃繕鍖呭惈涓€涓畬鏁寸殑
鈥滅‖浠剁洃鎺у潡鈥濄€傝纭欢鐩戞帶鍧椾笉鍙楁湰椹卞姩鏀寔锛屽姝よ浣跨敤 smsc47m192 椹卞姩銆?
娌℃湁 47M997 鐨勬枃妗ｅ彲鐢紝浣嗗畠涓?47M15x 鍜?47M192 鑺墖鍏锋湁鐩稿悓鐨勮澶?ID锛屽苟涓斾技涔?鍏煎銆?
椋庢墖杞€熶互 RPM锛堟瘡鍒嗛挓杞暟锛夋姤鍛娿€傚鏋滆浆閫熼檷鍒板彲缂栫▼闄愬埗浠ヤ笅锛屼細瑙﹀彂鍛婅銆傞鎵?璇绘暟鍙互琚竴涓彲缂栫▼鐨勫垎棰戝櫒锛?銆?銆? 鎴?8锛夐櫎锛屼互缁欎簣璇绘暟鏇村ぇ鐨勮寖鍥存垨绮惧害銆傚苟闈?鎵€鏈?RPM 鍊奸兘鑳界簿纭〃绀猴紝鍥犳浼氳繘琛屼竴浜涜垗鍏ャ€備娇鐢ㄥ垎棰戝櫒 2 鏃讹紝鍙〃绀虹殑鏈€浣庡€肩害涓?2600 RPM銆?
PWM 鍊艰寖鍥翠负 0 鍒?255銆?
濡傛灉鍛婅瑙﹀彂锛屽畠灏嗕竴鐩翠繚鎸佽Е鍙戠姸鎬侊紝鐩村埌纭欢瀵勫瓨鍣ㄨ嚦灏戣璇诲彇涓€娆°€傝繖鎰忓懗鐫€鍛婅鐨?鍘熷洜鍙兘宸茬粡娑堝け浜嗭紒娉ㄦ剰锛屽湪褰撳墠瀹炵幇涓紝鍙璇诲彇浠讳綍鏁版嵁锛屽氨浼氳鍙栨墍鏈夌‖浠跺瘎瀛樺櫒
锛堥櫎闈炶窛绂讳笂娆℃洿鏂颁笉鍒?1.5 绉掞級銆傝繖鎰忓懗鐫€浣犲彲鑳戒細杞绘槗閿欒繃浠呰Е鍙戜竴娆＄殑鍛婅銆?
------------------------------------------------------------------

lm_sensors 椤圭洰琛峰績鎰熻阿 Intel 鍦ㄦ湰椹卞姩寮€鍙戣繃绋嬩腑鎻愪緵鐨勬敮鎸併€?