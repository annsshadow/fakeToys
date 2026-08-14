## 绾挎€ф椂搴忛€昏緫锛圠inear temporal logic锛?

### 绠€浠?

杩愯鏃堕獙璇佺洃瑙嗗櫒锛圧untime verification monitor锛夋槸涓€绉嶉獙璇佹妧鏈紝鐢ㄤ簬妫€鏌ュ唴鏍告槸鍚﹂伒寰煇瑙勬牸璇存槑锛坰pecification锛夈€傚畠閫氳繃浣跨敤 tracepoint 鐩戣鍐呮牳鐨勬墽琛岃建杩癸紝骞堕獙璇佽鎵ц杞ㄨ抗婊¤冻瑙勬牸璇存槑鏉ュ疄鐜般€?
鏈€鍒濓紝瑙勬牸璇存槑鍙兘浠ョ‘瀹氭€ц嚜鍔ㄦ満锛圖A锛夌殑褰㈠紡缂栧啓銆傜劧鑰岋紝鍦ㄥ皾璇曚负涓€浜涘鏉傝鏍艰鏄庡疄鐜?DA 鐩戣鍣ㄦ椂锛屼汉浠彂鐜扮‘瀹氭€ц嚜鍔ㄦ満浣滀负瑙勬牸璇存槑璇█骞朵笉鍚堥€傘€傝鑷姩鏈哄鏉傘€侀毦浠ョ悊瑙ｄ笖瀹规槗鍑洪敊銆?
鍥犳锛屽紩鍏ヤ簡鍩轰簬绾挎€ф椂搴忛€昏緫锛圠TL锛夌殑 RV 鐩戣鍣ㄣ€傝繖绫荤洃瑙嗗櫒浣跨敤 LTL 鑰岄潪 DA 浣滀负瑙勬牸璇存槑銆傚湪鏌愪簺鎯呭喌涓嬶紝灏嗚鏍艰鏄庡啓鎴?LTL 鏇翠负绠€娲佸拰鐩磋銆?```

  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking, The MIT
  Press, 2008.

```
### 璇硶锛圙rammar锛?

涓庢煇浜涚幇鏈夎娉曚笉鍚岋紝鍐呮牳鐨?LTL 瀹炵幇鏇翠负鍐楅暱銆傝繖鏄€冭檻鍒伴槄璇?LTL 瑙勬牸璇存槑鐨勪汉鍙兘骞朵笉绮鹃€?LTL銆?
璇硶锛?    ltl ::= opd | ( ltl ) | ltl binop ltl | unop ltl

鎿嶄綔鏁帮紙opd锛夛細
    true銆乫alse銆佺敱澶у啓瀛楁瘝銆佹暟瀛楀拰涓嬪垝绾跨粍鎴愮殑鐢ㄦ埛瀹氫箟鍚嶇О銆?
涓€鍏冭繍绠楃锛坲nop锛夛細
    always锛堟€绘槸锛?    eventually锛堟渶缁堬級
    next锛堜笅涓€鏃跺埢锛?    not锛堥潪锛?
浜屽厓杩愮畻绗︼紙binop锛夛細
    until锛堢洿鍒帮級
    and锛堜笌锛?    or锛堟垨锛?    imply锛堣暣鍚級
    equivalent锛堢瓑浠凤級

璇ヨ娉曟槸姝т箟鐨勶細鏈畾涔夎繍绠楃浼樺厛绾с€傚繀椤讳娇鐢ㄦ嫭鍙枫€?
### 绾挎€ф椂搴忛€昏緫绀轰緥


   RAIN imply (GO_OUTSIDE imply HAVE_UMBRELLA)

鍚箟锛氬鏋滄鍦ㄤ笅闆紝閭ｄ箞澶栧嚭鎰忓懗鐫€甯︿簡浼炪€?

   RAIN imply (WET until not RAIN)

鍚箟锛氬鏋滄鍦ㄤ笅闆紝閭ｄ箞鍦ㄤ笅闆ㄥ仠姝箣鍓嶉兘浼氭槸婀跨殑銆?

   RAIN imply eventually not RAIN

鍚箟锛氬鏋滄鍦ㄤ笅闆紝闆ㄦ渶缁堜細鍋溿€?
涓婅堪绀轰緥浠呮寚褰撳墠鏃堕棿瀹炰緥銆傚浜庡唴鏍搁獙璇侊紝閫氬父甯屾湜浣跨敤 `always` 杩愮畻绗︽潵鎸囧畾
```

    always (RAIN imply eventually not RAIN)

```
鍚箟锛?*鎵€鏈?*闆ㄦ渶缁堥兘浼氬仠銆?
鍦ㄤ笂杩扮ず渚嬩腑锛宍RAIN`銆乣GO_OUTSIDE`銆乣HAVE_UMBRELLA` 鍜?`WET` 鏄€滃師瀛愬懡棰橈紙atomic propositions锛夆€濄€?
### 鐩戣鍣ㄧ患鍚?

瑕佸皢 LTL 缁煎悎涓哄唴鏍哥洃瑙嗗櫒锛屽彲浠ヤ娇鐢?`rvgen` 宸ュ叿锛歚tools/verification/rvgen`銆傝鏍艰鏄庨渶瑕佷互鏂囦欢褰㈠紡鎻愪緵锛?```

    RULE = always (ACQUIRE imply ((not KILLED and not CRASHED) until RELEASE))

```
鍏跺惈涔夋槸锛氬鏋滃彂鐢?`ACQUIRE`锛屽垯蹇呴』鍦?`KILLED` 鎴?`CRASHED` 涔嬪墠鍙戠敓 `RELEASE`銆?
鍙互浣跨敤瀛愯〃杈惧紡灏?LTL 鎷嗗垎銆備笂杩扮瓑浠蜂簬锛?
```

    RULE = always (ACQUIRE imply (ALIVE until RELEASE))
    ALIVE = not KILLED and not CRASHED

```
鏍规嵁璇ヨ鏍艰鏄庯紝`rvgen` 浼氱敓鎴愪竴涓?B眉chi 鑷姩鏈虹殑 C 瀹炵幇鈥斺€斾竴涓敤浜庢鏌?LTL 鍙弧瓒虫€х殑闈炵‘瀹氭€х姸鎬佹満銆傚叧浜庝娇鐢?`rvgen` 鐨勭粏鑺傦紝璇峰弬瑙?Documentation/trace/rv/monitor_synthesis.rst銆?
### 鍙傝€冩枃鐚?

```

  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking, The MIT
  Press, 2008.

```
```

  Ruijie Meng, Zhen Dong, Jialin Li, Ivan Beschastnikh, and Abhik Roychoudhury.
  2022. Linear-time temporal logic guided greybox fuzzing. In Proceedings of the
  44th International Conference on Software Engineering (ICSE '22).  Association
  for Computing Machinery, New York, NY, USA, 1343鈥?355.
  https://doi.org/10.1145/3510003.3510082

```
```

  Gerth, R., Peled, D., Vardi, M.Y., Wolper, P. (1996). Simple On-the-fly
  Automatic Verification of Linear Temporal Logic. In: Dembi艅ski, P., 艢redniawa,
  M. (eds) Protocol Specification, Testing and Verification XV. PSTV 1995. IFIP
  Advances in Information and Communication Technology. Springer, Boston, MA.
  https://doi.org/10.1007/978-0-387-34892-6_1

```
