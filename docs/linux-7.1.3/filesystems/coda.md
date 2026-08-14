
## Coda 鍐呮牳-Venus 鎺ュ彛


   杩欐槸鎻忚堪 Coda 缁勪欢鐨勬妧鏈枃妗ｄ箣涓€鈥斺€旀湰鏂囨。鎻忚堪鐨勬槸瀹㈡埛绔唴鏍?Venus 鎺ュ彛銆?

鏇村淇℃伅璇峰弬瑙侊細

  http://www.coda.cs.cmu.edu

杩愯 Coda 鎵€闇€鐨勭敤鎴风骇杞欢锛?

  ftp://ftp.coda.cs.cmu.edu

  瑕佽繍琛?Coda锛岄渶瑕佷负瀹㈡埛绔幏鍙栦竴涓敤鎴风骇缂撳瓨绠＄悊鍣紝鍚嶄负 Venus锛?
  浠ュ強鐢ㄤ簬鎿嶄綔 ACL銆佺櫥褰曠瓑鐨勫伐鍏枫€傚鎴风鍦ㄥ唴鏍搁厤缃腑闇€瑕侀€変腑 Coda
  鏂囦欢绯荤粺銆?

  鏈嶅姟鍣ㄩ渶瑕佷竴涓敤鎴风骇鏈嶅姟鍣紝鐩墠涓嶄緷璧栧唴鏍告敮鎸併€?

  Venus 鍐呮牳鎺ュ彛

  Peter J. Braam

  v1.0锛?997骞?1鏈?鏃?

  鏈枃妗ｆ弿杩颁簡 Venus 涓庡唴鏍哥骇鏂囦欢绯荤粺浠ｇ爜涔嬮棿鐨勯€氫俊锛岃繖鏄?Coda
  鏂囦欢绯荤粺杩愯鎵€蹇呴渶鐨勩€傛湰鏂囨。鐗堟湰鏃ㄥ湪鎻忚堪褰撳墠鎺ュ彛
  锛堢増鏈?1.0锛変互鍙婃垜浠鎯崇殑鏀硅繘銆?


  1. 绠€浠?

  2. 涓?Coda 鏂囦欢绯荤粺璋冪敤鎻愪緵鏈嶅姟

  3. 娑堟伅灞?

     3.1 瀹炵幇缁嗚妭

  4. 璋冪敤灞傛帴鍙?

     4.1 鍐呮牳涓?Venus 鍏变韩鐨勬暟鎹粨鏋?
     4.2 pioctl 鎺ュ彛
     4.3 root
     4.4 lookup
     4.5 getattr
     4.6 setattr
     4.7 access
     4.8 create
     4.9 mkdir
     4.10 link
     4.11 symlink
     4.12 remove
     4.13 rmdir
     4.14 readlink
     4.15 open
     4.16 close
     4.17 ioctl
     4.18 rename
     4.19 readdir
     4.20 vget
     4.21 fsync
     4.22 inactive
     4.23 rdwr
     4.24 odymount
     4.25 ody_lookup
     4.26 ody_expand
     4.27 prefetch
     4.28 signal

  5. 杩蜂綘缂撳瓨涓庝笅琛岃皟鐢紙downcall锛?

     5.1 INVALIDATE
     5.2 FLUSH
     5.3 PURGEUSER
     5.4 ZAPFILE
     5.5 ZAPDIR
     5.6 ZAPVNODE
     5.7 PURGEFID
     5.8 REPLACE

  6. 鍒濆鍖栦笌娓呯悊

     6.1 闇€姹?

## 1. 绠€浠?


  Coda 鍒嗗竷寮忔枃浠剁郴缁熺殑鍏抽敭缁勪欢鏄紦瀛樼鐞嗗櫒 Venus銆?

  褰撳惎鐢ㄤ簡 Coda 鐨勭郴缁熶笂鐨勮繘绋嬭闂?Coda 鏂囦欢绯荤粺涓殑鏂囦欢鏃讹紝璇锋眰浼氳
  瀵煎悜鎿嶄綔绯荤粺涓殑鏂囦欢绯荤粺灞傘€傛搷浣滅郴缁熷皢涓?Venus 閫氫俊浠ュ鐞嗚杩涚▼鐨?
  璇锋眰銆俈enus 绠＄悊涓€涓寔涔呯殑瀹㈡埛绔紦瀛橈紝骞跺悜 Coda 鏂囦欢鏈嶅姟鍣ㄥ強鐩稿叧
  鏈嶅姟鍣紙濡傝璇佹湇鍔″櫒锛夊彂璧疯繙绋嬭繃绋嬭皟鐢紝浠ュ鐞嗕粠鎿嶄綔绯荤粺鏀跺埌鐨勮繖浜?
  璇锋眰銆傚綋 Venus 澶勭悊瀹屼竴涓姹傚悗锛屼細鐢ㄩ€傚綋鐨勮繑鍥炵爜鍙婁笌璇ヨ姹傜浉鍏崇殑
  鍏朵粬鏁版嵁鍥炲鎿嶄綔绯荤粺銆傚彲閫夊湴锛孋oda 鐨勫唴鏍告敮鎸佸彲浠ョ淮鎶や竴涓渶杩戝鐞?
  璇锋眰鐨勮糠浣犵紦瀛橈紝浠ラ檺鍒朵笌 Venus 鐨勪氦浜掓鏁般€俈enus 鍏峰鍦ㄨ糠浣犵紦瀛樹腑鐨?
  鍏冪礌涓嶅啀鏈夋晥鏃堕€氱煡鍐呮牳鐨勮兘鍔涖€?

  鏈枃妗ｇ簿纭弿杩板唴鏍镐笌 Venus 涔嬮棿鐨勮繖绉嶉€氫俊銆傚皢缁欏嚭鎵€璋撲笂琛岃皟鐢?
  锛坲pcall锛夊拰涓嬭璋冪敤锛坉owncall锛夌殑瀹氫箟锛屼互鍙婂畠浠墍澶勭悊鏁版嵁鐨勬牸寮忋€?
  鎴戜滑杩樺皢鎻忚堪鐢辫繖浜涜皟鐢ㄤ骇鐢熺殑璇箟涓嶅彉閲忋€?

  鍘嗗彶涓婏紝Coda 鍦?Mach 2.6 鐨?BSD 鏂囦欢绯荤粺涓疄鐜般€傚唴鏍镐笌 Venus 涔嬮棿鐨?
  鎺ュ彛涓?BSD VFS 鎺ュ彛闈炲父鐩镐技銆傚畠鎻愪緵浜嗙被浼肩殑鍔熻兘锛屽弬鏁板拰杩斿洖鏁版嵁鐨?
  鏍煎紡涔熶笌 BSD VFS 闈炲父鐩镐技銆傝繖浣垮緱鍦?BSD 绯荤粺涓负 Coda 瀹炵幇涓€涓?
  鍐呮牳绾ф枃浠剁郴缁熼┍鍔ㄥ嚑涔庢槸涓€涓嚜鐒剁殑鐜銆傜劧鑰岋紝鍏朵粬鎿嶄綔绯荤粺濡?Linux銆?
  Windows 95 鍜?NT 鎷ユ湁涓嶅悓鎺ュ彛鐨勮櫄鎷熸枃浠剁郴缁熴€?

  瑕佸湪杩欎簺绯荤粺涓婂疄鐜?Coda锛岄渶瑕佸 Venus/鍐呮牳鍗忚杩涜涓€浜涢€嗗悜宸ョ▼銆?
  鍚屾椂浜轰滑涔熷彂鐜帮紝鍏朵粬绯荤粺鍙互浠庡崗璁殑鏌愪簺灏忎紭鍖栧拰淇敼涓樉钁楀彈鐩娿€?
  涓轰簡渚夸簬杩欓」宸ヤ綔锛屽苟浣挎湭鏉ョ殑绉绘鏇村鏄擄紝Venus 涓庡唴鏍镐箣闂寸殑閫氫俊搴?
  璇﹀敖鍦拌褰曘€傝繖灏辨槸鏈枃妗ｇ殑鐩爣銆?

## 2. 涓?Coda 鏂囦欢绯荤粺璋冪敤鎻愪緵鏈嶅姟


  Coda 鏂囦欢绯荤粺鏈嶅姟鐨勮姹傚鐞嗭紝璧锋簮浜庤闂?Coda 鏂囦欢銆佸彂鍑洪櫡鍏ワ紙trap锛?
  鍒?OS 鍐呮牳鐨勮繘绋?P銆傚湪 Unix 鐜涓紝杩欑被闄峰叆鍐呮牳鐨勮皟鐢ㄧず渚嬪寘鎷?
  `read`銆乣write`銆乣open`銆乣close`銆乣create`銆乣mkdir`銆?
  `rmdir`銆乣chmod`銆傚湪 Win32 鐜涓篃瀛樺湪绫讳技鐨勮皟鐢紝鍚嶄负 `CreateFile`銆?

  閫氬父鎿嶄綔绯荤粺鍦ㄨ櫄鎷熸枃浠剁郴缁燂紙VFS锛夊眰涓鐞嗚璇锋眰锛屽湪 NT 涓О涓?
  I/O Manager锛屽湪 Windows 95 涓О涓?IFS manager銆俈FS 璐熻矗璇锋眰鐨勯儴鍒?
  澶勭悊锛屽苟瀹氫綅灏嗕负璇锋眰鍚勯儴鍒嗘彁渚涙湇鍔＄殑鐗瑰畾鏂囦欢绯荤粺銆傞€氬父璺緞涓殑淇℃伅
  鏈夊姪浜庡畾浣嶆纭殑 FS 椹卞姩銆傛湁鏃跺湪澶ч噺棰勫鐞嗕箣鍚庯紝VFS 寮€濮嬭皟鐢?FS
  椹卞姩涓鍑虹殑渚嬬▼銆傝繖灏辨槸璇锋眰鐨?FS 鐗瑰畾澶勭悊寮€濮嬩箣澶勶紝涔熸槸 Coda 鐗瑰畾鐨?
  鍐呮牳浠ｇ爜鍙戞尌浣滅敤涔嬪銆?

  Coda 鐨?FS 灞傚繀椤绘毚闇插苟瀹炵幇澶氫釜鎺ュ彛銆傞鍏堜篃鏄渶閲嶈鐨勶紝VFS 蹇呴』
  鑳藉鍚?Coda FS 灞傚彂鍑烘墍鏈夊繀瑕佺殑璋冪敤锛屽洜姝?Coda FS 椹卞姩蹇呴』鏆撮湶
  鎿嶄綔绯荤粺涓€傜敤鐨?VFS 鎺ュ彛銆傝繖浜涙帴鍙ｅ湪涓嶅悓鎿嶄綔绯荤粺涔嬮棿宸紓寰堝ぇ锛屼絾
  鍏变韩璇稿璇?鍐欎互鍙婂垱寤哄拰鍒犻櫎瀵硅薄绛夊姛鑳姐€侰oda FS 灞傞€氳繃璋冪敤缂撳瓨绠＄悊鍣?
  Venus 鎻愪緵鐨勪竴涓垨澶氫釜瀹氫箟鏄庣‘鐨勬湇鍔℃潵澶勭悊姝ょ被 VFS 璇锋眰銆傚綋鏉ヨ嚜
  Venus 鐨勫洖澶嶈繑鍥炲埌 FS 椹卞姩鍚庯紝VFS 璋冪敤鐨勫鐞嗙户缁紝骞朵互瀵瑰唴鏍?VFS 鐨?
  鍥炲缁撴潫銆傛渶鍚?VFS 灞傝繑鍥炲埌杩涚▼銆?

  鐢变簬杩欑璁捐鐨勯渶瑕侊紝FS 椹卞姩蹇呴』鏆撮湶鐨勪竴涓熀鏈帴鍙ｈ鍏佽 Venus 绠＄悊
  娑堟伅娴侀噺銆傜壒鍒槸 Venus 蹇呴』鑳藉妫€绱㈠拰鏀剧疆娑堟伅锛屽苟鍦ㄦ柊娑堟伅鍒拌揪鏃跺緱鍒?
  閫氱煡銆傜敱浜?Venus 鍗充娇鍦ㄦ病鏈夋秷鎭瓑寰呮垨姝ｅ湪澶勭悊鏃朵篃蹇呴』澶勭悊鍏朵粬浠诲姟锛?
  鍥犳閫氱煡蹇呴』閫氳繃涓嶄細闃诲 Venus 鐨勬満鍒惰繘琛屻€?

  **Coda FS 椹卞姩鐨勬帴鍙?*

  姝ゅ锛孎S 灞傛彁渚涚敤鎴疯繘绋嬩笌 Venus 涔嬮棿鐨勪竴鏉＄壒娈婇€氫俊璺緞锛岀О涓?pioctl
  鎺ュ彛銆俻ioctl 鎺ュ彛鐢ㄤ簬 Coda 鐗瑰畾鐨勬湇鍔★紝渚嬪璇锋眰鍏充簬 Venus 绠＄悊鐨?
  鎸佷箙缂撳瓨鐨勮缁嗕俊鎭€傝繖閲屽唴鏍哥殑鍙備笌鏄渶灏忕殑銆傚畠璇嗗埆璋冪敤杩涚▼骞跺皢
  淇℃伅浼犻€掔粰 Venus銆傚綋 Venus 鍥炲鏃讹紝鍝嶅簲浠ユ湭淇敼鐨勫舰寮忎紶鍥炶皟鐢ㄨ€呫€?

  鏈€鍚庯紝Venus 鍏佽鍐呮牳 FS 椹卞姩缂撳瓨鏌愪簺鏈嶅姟鐨勭粨鏋溿€傝繖鏍峰仛鏄负浜嗛伩鍏?
  杩囧鐨勪笂涓嬫枃鍒囨崲锛屼粠鑰屽緱鍒颁竴涓珮鏁堢殑绯荤粺銆傜劧鑰岋紝Venus 鍙兘鑾峰彇鍒?
  淇℃伅锛堜緥濡傛潵鑷綉缁滐級锛岃繖鎰忓懗鐫€缂撳瓨鐨勪俊鎭繀椤昏鍒锋柊鎴栨浛鎹€俈enus 浜庢槸
  鍚?Coda FS 灞傚彂鍑轰竴涓笅琛岃皟鐢紙downcall锛夛紝浠ヨ姹傚埛鏂版垨鏇存柊缂撳瓨銆?
  鍐呮牳 FS 椹卞姩鍚屾鍦板鐞嗘绫昏姹傘€?

  鍦ㄨ繖浜涙帴鍙ｄ腑锛孷FS 鎺ュ彛浠ュ強鏀剧疆銆佹帴鏀舵秷鎭苟鑾风煡娑堟伅鍒拌揪鐨勬満鍒舵槸
  骞冲彴鐩稿叧鐨勩€傛垜浠笉浼氭繁鍏ヨ璁哄鍑哄埌 VFS 灞傜殑璋冪敤锛屼絾鎴戜滑灏嗚鏄?
  娑堟伅浜ゆ崲鏈哄埗鐨勯渶姹傘€?


## 3. 娑堟伅灞?


  鍦ㄦ渶搴曞眰锛孷enus 涓?FS 椹卞姩涔嬮棿鐨勯€氫俊閫氳繃娑堟伅杩涜銆傝姹?Coda 鏂囦欢鏈嶅姟
  鐨勮繘绋嬩笌 Venus 涔嬮棿鐨勫悓姝ヤ緷璧栦簬杩涚▼鐨勯樆濉炰笌鍞ら啋銆侰oda FS 椹卞姩浠ｈ〃
  杩涚▼ P 澶勭悊 VFS 鍜?pioctl 璇锋眰锛屼负 Venus 鍒涘缓娑堟伅锛岀瓑寰呭洖澶嶏紝鏈€鍚?
  杩斿洖璋冪敤鑰呫€傛秷鎭氦鎹㈢殑瀹炵幇鏄钩鍙扮浉鍏崇殑锛屼絾鍏惰涔夛紙鍒扮洰鍓嶄负姝級鐪嬫潵
  鏅亶閫傜敤銆傛暟鎹紦鍐插尯鐢?FS 椹卞姩鍦ㄥ唴鏍稿唴瀛樹腑浠ｈ〃 P 鍒涘缓锛屽苟澶嶅埗鍒?
  Venus 鐨勭敤鎴峰唴瀛樹腑銆?

  FS 椹卞姩鍦ㄤ负 P 鏈嶅姟鏃惰繘琛屼笂琛岃皟鐢紙upcall锛夊埌 Venus銆傝繖鏍蜂竴涓笂琛岃皟鐢?
  閫氳繃鍒涘缓娑堟伅缁撴瀯鍒嗘淳缁?Venus銆傝缁撴瀯鍖呭惈 P 鐨勬爣璇嗐€佹秷鎭簭鍙枫€佽姹?
  鐨勫ぇ灏忥紝浠ュ強鎸囧悜鍐呮牳鍐呭瓨涓姹傛暟鎹殑鎸囬拡銆傜敱浜庢暟鎹紦鍐插尯琚鐢ㄤ互
  淇濆瓨鏉ヨ嚜 Venus 鐨勫洖澶嶏紝鍥犳鏈変竴涓瓧娈电敤浜庡洖澶嶇殑澶у皬銆傛秷鎭腑浣跨敤涓€涓?
  flags 瀛楁鏉ョ簿纭褰曟秷鎭殑鐘舵€併€傚叾浠栧钩鍙扮浉鍏崇粨鏋勬秹鍙婄敤浜庣‘瀹氭秷鎭湪
  闃熷垪涓綅缃殑鎸囬拡锛屼互鍙婃寚鍚戝悓姝ュ璞＄殑鎸囬拡銆傚湪 upcall 渚嬬▼涓紝娑堟伅缁撴瀯
  琚～鍏咃紝flags 缃负 0锛屽苟琚斁鍏?**pending**锛堟寕璧凤級闃熷垪銆傝皟鐢?upcall
  鐨勪緥绋嬭礋璐ｅ垎閰嶆暟鎹紦鍐插尯锛涘叾缁撴瀯灏嗗湪涓嬩竴鑺傛弿杩般€?

  蹇呴』鎻愪緵涓€绉嶆満鍒舵潵閫氱煡 Venus 娑堟伅宸插垱寤猴紝骞朵娇鐢?OS 涓彲鐢ㄧ殑鍚屾瀵硅薄
  瀹炵幇銆傛閫氱煡鍦ㄨ繘绋?P 鐨?upcall 涓婁笅鏂囦腑瀹屾垚銆傚綋娑堟伅鍦ㄦ寕璧烽槦鍒椾笂鏃讹紝
  杩涚▼ P 鏃犳硶鍦?upcall 涓户缁€侾 鍦ㄦ枃浠剁郴缁熻姹備緥绋嬩腑鐨勶紙鍐呮牳妯″紡锛?
  澶勭悊蹇呴』鎸傝捣锛岀洿鍒?Venus 鍥炲銆傚洜姝よ皟鐢ㄧ嚎绋嬪湪 P 鐨?upcall 涓闃诲銆?
  娑堟伅缁撴瀯涓殑涓€涓寚閽堝皢瀹氫綅 P 姝ｅ湪鍏朵笂鐫＄湢鐨勫悓姝ュ璞°€?

  Venus 妫€娴嬪埌娑堟伅宸插埌杈剧殑閫氱煡锛孎S 椹卞姩鍏佽 Venus 閫氳繃 getmsg_from_kernel
  璋冪敤妫€绱㈣娑堟伅銆傝鍔ㄤ綔鍦ㄥ唴鏍镐腑鎵ц瀹屾瘯锛屽嵆灏嗘秷鎭斁鍏ュ鐞嗕腑娑堟伅闃熷垪
  骞跺皢 flags 缃负 READ銆傛秷鎭紦鍐插尯鐨勫唴瀹硅浼犻€掔粰 Venus銆俫etmsg_from_kernel
  璋冪敤鐜板湪杩斿洖锛孷enus 澶勭悊璇ヨ姹傘€?

  绋嶅悗鏌愪釜鏃跺埢锛孎S 椹卞姩浠?Venus 鏀跺埌涓€鏉℃秷鎭紝鍗?Venus 璋冪敤
  sendmsg_to_kernel 鏃躲€傛鏃?Coda FS 椹卞姩鏌ョ湅娑堟伅鍐呭骞跺喅瀹氾細

  - 娑堟伅鏄寕璧风嚎绋?P 鐨勫洖澶嶃€傝嫢鏄紝瀹冧粠澶勭悊闃熷垪涓Щ闄よ娑堟伅骞跺皢娑堟伅
    鏍囪涓?WRITTEN銆傛渶鍚庯紝FS 椹卞姩瑙ｉ櫎 P 鐨勯樆濉烇紙浠嶅湪 Venus 鐨勫唴鏍告ā寮?
    涓婁笅鏂囦腑锛夛紝sendmsg_to_kernel 璋冪敤杩斿洖缁?Venus銆傝繘绋?P 灏嗗湪鏌愪釜鏃跺埢
    琚皟搴︼紝骞剁户缁鐞嗗叾 upcall锛屾鏃舵暟鎹紦鍐插尯宸茶 Venus 鐨勫洖澶嶆浛鎹€?

  - 娑堟伅鏄竴涓?`downcall`锛堜笅琛岃皟鐢級銆備笅琛岃皟鐢ㄦ槸 Venus 瀵?FS 椹卞姩鐨?
    璇锋眰銆侳S 椹卞姩绔嬪嵆澶勭悊璇ヨ姹傦紙閫氬父鏄竴娆＄紦瀛橀┍閫愭垨鏇挎崲锛夛紝瀹屾垚鍚?
    sendmsg_to_kernel 杩斿洖銆?

  鐜板湪 P 琚敜閱掑苟缁х画澶勭悊 upcall銆傛湁涓€浜涘井濡欎箣澶勯渶瑕佽€冭檻銆傞鍏堬紝P 灏?
  纭畾瀹冩槸鍚︾敱鍏朵粬鏉ユ簮鐨?signal 鍞ら啋锛堜緥濡傝瘯鍥剧粓姝?P 鐨勫皾璇曪級锛岃繕鏄儚
  閫氬父鎯呭喌閭ｆ牱鐢?Venus 鍦ㄥ叾 sendmsg_to_kernel 璋冪敤涓敜閱掋€傚湪姝ｅ父鎯呭喌
  涓嬶紝upcall 渚嬬▼灏嗛噴鏀炬秷鎭粨鏋勫苟杩斿洖銆侳S 渚嬬▼鍙互缁х画鍏跺鐞嗐€?


  **鐫＄湢涓?IPC 瀹夋帓**

  濡傛灉 P 鏄敱 signal 鑰岄潪 Venus 鍞ら啋锛屽畠灏嗛鍏堟煡鐪?flags 瀛楁銆傚鏋?
  娑堟伅灏氭湭琚?READ锛岃繘绋?P 鍙互鍦ㄤ笉閫氱煡 Venus 鐨勬儏鍐典笅澶勭悊鍏?signal銆?
  濡傛灉 Venus 宸茬粡 READ锛屼笖璇锋眰涓嶅簲琚鐞嗭紝P 鍙互鍚?Venus 鍙戦€佷竴鏉′俊鍙?
  娑堟伅锛岃〃鏄庡畠搴斿拷鐣ュ厛鍓嶇殑娑堟伅銆傛绫讳俊鍙疯鏀惧湪闃熷垪澶撮儴锛岀敱 Venus 棣栧厛
  璇诲彇銆傚鏋滄秷鎭凡琚爣璁颁负 WRITTEN锛屽垯鍋滄澶勭悊涓烘椂宸叉櫄銆俈FS 渚嬬▼鐜板湪
  灏嗙户缁€傦紙-- 濡傛灉涓€涓?VFS 璇锋眰娑夊強澶氫釜 upcall锛岃繖浼氬鑷村鏉傜殑鐘舵€侊紝
  鍙互鍦ㄦ秷鎭粨鏋勪腑娣诲姞涓€涓澶栧瓧娈?"handle_signals" 浠ユ爣璇嗗凡瓒婅繃
  涓嶅彲杩斿洖鐐广€?-锛?



### 3.1. 瀹炵幇缁嗚妭


  璇ユ満鍒剁殑 Unix 瀹炵幇鏄€氳繃瀹炵幇涓€涓笌 Coda 鍏宠仈鐨勫瓧绗﹁澶囨潵瀹屾垚鐨勩€?
  Venus 閫氳繃瀵硅澶囨墽琛?read 鏉ユ绱㈡秷鎭紝鍥炲閫氳繃 write 鍙戦€侊紝閫氱煡閫氳繃
  瀵硅澶囨枃浠舵弿杩扮鎵ц select 绯荤粺璋冪敤鏉ュ疄鐜般€傝繘绋?P 琚繚鎸佸湪鍙腑鏂?
  鐨勭瓑寰呴槦鍒楀璞′笂绛夊緟銆?

  鍦?Windows NT 鍜?DPMI Windows 95 瀹炵幇涓紝浣跨敤浜?DeviceIoControl 璋冪敤銆?
  DeviceIoControl 璋冪敤鏃ㄥ湪閫氳繃 OPCODES 灏嗙紦鍐插尯浠庣敤鎴峰唴瀛樺鍒跺埌鍐呮牳鍐呭瓨銆?
  sendmsg_to_kernel 浣滀负鍚屾璋冪敤鍙戝嚭锛岃€?getmsg_from_kernel 璋冪敤鏄?
  寮傛鐨勩€俉indows EventObjects 鐢ㄤ簬閫氱煡娑堟伅鍒拌揪銆傝繘绋?P 鍦?NT 涓淇濇寔
  鍦?KernelEvent 瀵硅薄涓婄瓑寰咃紝鍦?Windows 95 涓淇濇寔鍦ㄤ俊鍙烽噺涓婄瓑寰呫€?


## 4. 璋冪敤灞傛帴鍙?


  鏈妭鎻忚堪 Coda FS 椹卞姩鍙互瀵?Venus 杩涜鐨勪笂琛岃皟鐢紙upcall锛夈€傝繖浜?
  涓婅璋冪敤涓殑姣忎竴涓兘浣跨敤涓や釜缁撴瀯锛歩nputArgs 鍜?outputArgs銆備互浼?BNF
  褰㈠紡锛岃繖浜涚粨鏋勫涓嬶細

```
	struct inputArgs {
	    u_long opcode;
	    u_long unique;     /* Keep multiple outstanding msgs distinct */
	    u_short pid;                 /* Common to all */
	    u_short pgid;                /* Common to all */
	    struct CodaCred cred;        /* Common to all */

	    <union "in" of call dependent parts of inputArgs>
	};

	struct outputArgs {
	    u_long opcode;
	    u_long unique;       /* Keep multiple outstanding msgs distinct */
	    u_long result;

	    <union "out" of call dependent parts of inputArgs>
	};
```


  鍦ㄧ户缁箣鍓嶏紝璁╂垜浠槓鏄庡悇涓瓧娈电殑浣滅敤銆俰nputArgs 浠ュ畾涔夋墍璇锋眰 Venus
  鏈嶅姟绫诲瀷鐨?opcode 寮€澶淬€傜洰鍓嶅ぇ绾︽湁 30 涓笂琛岃皟鐢紝鎴戜滑灏嗛€愪竴璁ㄨ銆?
  unique 瀛楁鐢ㄥ敮涓€缂栧彿鏍囪 inputArg锛岃缂栧彿灏嗗敮涓€鏍囪瘑璇ユ秷鎭€傝繘绋嬪拰
  杩涚▼缁?id 琚紶閫掋€傛渶鍚庡寘鍚皟鐢ㄨ€呯殑鍑瘉锛坈redentials锛夈€?

  鍦ㄦ繁鍏ュ叿浣撶殑璋冪敤涔嬪墠锛屾垜浠渶瑕佽璁哄唴鏍镐笌 Venus 鍏变韩鐨勫悇绉嶆暟鎹粨鏋勩€?


### 4.1. 鍐呮牳涓?Venus 鍏变韩鐨勬暟鎹粨鏋?


  CodaCred 缁撴瀯瀹氫箟浜嗚皟鐢ㄨ繘绋嬫墍璁剧疆鐨勫绉嶇敤鎴峰拰缁?id銆倂uid_t 鍜?
  vgid_t 鏄?32 浣嶆棤绗﹀彿鏁存暟銆傚畠杩樺湪鏁扮粍涓畾涔夌粍鎴愬憳鍏崇郴銆傚湪 Unix 涓婏紝
  CodaCred 宸茶璇佹槑瓒充互瀹炵幇 Coda 鐨勮壇濂藉畨鍏ㄨ涔夛紝浣嗚缁撴瀯鍙兘蹇呴』
  淇敼

```
	struct CodaCred {
	    vuid_t cr_uid, cr_euid, cr_suid, cr_fsuid; /* Real, effective, set, fs uid */
	    vgid_t cr_gid, cr_egid, cr_sgid, cr_fsgid; /* same for groups */
	    vgid_t cr_groups[NGROUPS];        /* Group membership for caller */
	};
```


  .. Note::

     鏄惁闇€瑕佸湪 Venus 涓繚鐣?CodaCreds 鍊煎緱鎬€鐤戙€傛渶鍚?Venus 骞朵笉浜嗚В
     缁勶紝灏界瀹冪‘瀹炰娇鐢ㄩ粯璁ょ殑 uid/gid 鍒涘缓鏂囦欢銆備篃璁哥粍鎴愬憳鍏崇郴鍒楄〃鏄?
     澶氫綑鐨勩€?


  涓嬩竴椤规槸鐢ㄤ簬鏍囪瘑 Coda 鏂囦欢鐨勫熀鏈爣璇嗙锛屽嵆 ViceFid銆傛枃浠剁殑 fid 鍦?
  涓€涓?cell [1]_ 鍐呭敮涓€鍦板畾涔?Coda 鏂囦欢绯荤粺涓殑鏂囦欢鎴栫洰褰曪細

```
	typedef struct ViceFid {
	    VolumeId Volume;
	    VnodeId Vnode;
	    Unique_t Unique;
	} ViceFid;
```

  .. [1] cell 鏄湪鍗曚竴绯荤粺鎺у埗鏈猴紙SCM锛夌殑搴囨姢涓嬭繍琛岀殑涓€缁?Coda 鏈嶅姟鍣ㄣ€?
	 鏈夊叧 SCM 瑙掕壊鐨勮缁嗚鏄庯紝璇峰弬闃?Coda 绠＄悊鎵嬪唽銆?

  VolumeId銆乂nodeId 鍜?Unique_t 杩欎笁涓粍鎴愬瓧娈甸兘鏄棤绗﹀彿 32 浣嶆暣鏁般€?
  鎴戜滑璁炬兂闇€瑕佸湪鍓嶉潰鍐嶅姞涓€涓瓧娈典互鏍囪瘑 Coda cell锛涜繖鍙兘閲囧彇閫氳繃 DNS
  鍛藉悕 Coda cell 鐨?Ipv6 澶у皬 IP 鍦板潃鐨勫舰寮忋€?

  Venus 涓庡唴鏍镐箣闂村叡浜殑涓嬩竴涓噸瑕佺粨鏋勬槸鏂囦欢鐨勫睘鎬с€備娇鐢ㄤ互涓嬬粨鏋勬潵
  浜ゆ崲淇℃伅銆傚畠涓烘湭鏉ョ殑鎵╁睍鐣欐湁绌洪棿锛屼緥濡傚璁惧鏂囦欢锛堝綋鍓?Coda 涓?
  涓嶅瓨鍦級鐨勬敮鎸侊細

```
	struct coda_timespec {
		int64_t         tv_sec;         /* seconds */
		long            tv_nsec;        /* nanoseconds */
	};

	struct coda_vattr {
		enum coda_vtype va_type;        /* vnode type (for create) */
		u_short         va_mode;        /* files access mode and type */
		short           va_nlink;       /* number of references to file */
		vuid_t          va_uid;         /* owner user id */
		vgid_t          va_gid;         /* owner group id */
		long            va_fsid;        /* file system id (dev for now) */
		long            va_fileid;      /* file id */
		u_quad_t        va_size;        /* file size in bytes */
		long            va_blocksize;   /* blocksize preferred for i/o */
		struct coda_timespec va_atime;  /* time of last access */
		struct coda_timespec va_mtime;  /* time of last modification */
		struct coda_timespec va_ctime;  /* time file changed */
		u_long          va_gen;         /* generation number of file */
		u_long          va_flags;       /* flags defined for file */
		dev_t           va_rdev;        /* device special file represents */
		u_quad_t        va_bytes;       /* bytes of disk space held by file */
		u_quad_t        va_filerev;     /* file modification number */
		u_int           va_vaflags;     /* operations flags, see below */
		long            va_spare;       /* remain quad aligned */
	};
```


### 4.2. pioctl 鎺ュ彛


  Coda 鐗瑰畾鐨勮姹傚彲浠ョ敱搴旂敤绋嬪簭閫氳繃 pioctl 鎺ュ彛鍙戝嚭銆俻ioctl 瀹炵幇涓?
  瀵硅櫄鏋勬枃浠?/coda/.CONTROL 鐨勬櫘閫?ioctl銆俻ioctl 璋冪敤鎵撳紑璇ユ枃浠讹紝鑾峰彇
  鏂囦欢鍙ユ焺骞惰繘琛?ioctl 璋冪敤銆傛渶鍚庡畠鍏抽棴鏂囦欢銆?

  鍐呮牳鍦ㄦ澶勭殑鍙備笌浠呴檺浜庢彁渚涙墦寮€鍜屽叧闂互鍙婁紶閫?ioctl 娑堟伅鐨勮兘鍔涳紝骞?
  楠岃瘉 pioctl 鏁版嵁缂撳啿鍖轰腑鐨勮矾寰勬槸 Coda 鏂囦欢绯荤粺涓殑鏂囦欢銆?

```
	struct {
	    const char *path;
	    struct ViceIoctl vidata;
	    int follow;
	} data;
```

  鍏朵腑锛?

```
	struct ViceIoctl {
		caddr_t in, out;        /* Data to be transferred in, or out */
		short in_size;          /* Size of input buffer <= 2K */
		short out_size;         /* Maximum size of output buffer, <= 2K */
	};
```

  璺緞蹇呴』鏄?Coda 鏂囦欢锛屽惁鍒欏皢涓嶄細杩涜 ioctl 涓婅璋冪敤銆?

  .. Note:: 鏁版嵁缁撴瀯鍜屼唬鐮佷竴鍥㈢碂銆傛垜浠渶瑕佹竻鐞嗗畠銆?


**鎴戜滑鐜板湪鐫€鎵嬭褰曞悇涓皟鐢?*锛?


### 4.3. root


  鍙傛暟
     in

	empty

```
		struct cfs_root_out {
		    ViceFid VFid;
		} cfs_root;
```

  鎻忚堪
    姝よ皟鐢ㄥ湪 Coda 鏂囦欢绯荤粺鍒濆鍖栨湡闂磋鍙戝線 Venus銆傚鏋滅粨鏋滀负闆讹紝
    cfs_root 缁撴瀯鍖呭惈 Coda 鏂囦欢绯荤粺鏍圭殑 ViceFid銆傚鏋滀骇鐢熼潪闆剁粨鏋滐紝
    鍏跺€间负骞冲彴鐩稿叧鐨勯敊璇爜锛屾寚绀?Venus 鍦ㄥ畾浣?Coda 鏂囦欢绯荤粺鏍规椂閬囧埌鐨?
    鍥伴毦銆?


### 4.4. lookup


  鎽樿
    濡傛灉瀵硅薄瀛樺湪锛屾煡鎵剧洰褰曚腑瀵硅薄鐨?ViceFid 鍜岀被鍨嬨€?

  鍙傛暟

```
		struct  cfs_lookup_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_lookup;
```

     out锛?

```
		struct cfs_lookup_out {
		    ViceFid VFid;
		    int vtype;
		} cfs_lookup;
```

  鎻忚堪
    姝よ皟鐢ㄧ敤浜庣‘瀹氱洰褰曢」鐨?ViceFid 鍜屾枃浠剁被鍨嬨€傛墍璇锋眰鐨勭洰褰曢」
    鍚嶄负 'name'锛孷enus 灏嗘悳绱㈢敱 cfs_lookup_in.VFid 鏍囪瘑鐨勭洰褰曘€傜粨鏋?
    鍙兘鎸囩ず璇ュ悕绉颁笉瀛樺湪锛屾垨鍦ㄦ煡鎵炬椂閬囧埌鍥伴毦锛堜緥濡傜敱浜庢柇寮€杩炴帴锛夈€?
    濡傛灉缁撴灉涓洪浂锛屽瓧娈?cfs_lookup_out.VFid 鍖呭惈鐩爣鐨?ViceFid锛?
    cfs_lookup_out.vtype 鍖呭惈鏍囪瘑璇ュ悕绉版墍鎸囧璞＄被鍨嬬殑 coda_vtype銆?

  璇ュ璞＄殑鍚嶇О鏄渶澶ч暱搴︿负 CFS_MAXNAMLEN 鐨?8 浣嶅瓧绗︿覆锛屽綋鍓嶈涓?
  256锛堝寘鎷竴涓?0 缁撳熬绗︼級銆?

  鏋佸叾閲嶈鐨勬槸瑕佽璇嗗埌锛孷enus 灏嗗瓧娈?cfs_lookup.vtype 鎸変綅鎴栦笂
  CFS_NOCACHE锛屼互鎸囩ず璇ュ璞′笉搴旇鏀惧叆鍐呮牳鍚嶇О缂撳瓨銆?

  .. Note::

     褰撳墠 vtype 鐨勭被鍨嬫槸閿欒鐨勩€傚畠搴旇鏄?coda_vtype銆侺inux 娌℃湁
     娉ㄦ剰 CFS_NOCACHE銆傚畠搴旇杩欐牱鍋氥€?


### 4.5. getattr


  鎽樿 鑾峰彇鏂囦欢鐨勫睘鎬с€?

  鍙傛暟

```
		struct cfs_getattr_in {
		    ViceFid VFid;
		    struct coda_vattr attr; /* XXXXX */
		} cfs_getattr;
```

     out锛?

```
		struct cfs_getattr_out {
		    struct coda_vattr attr;
		} cfs_getattr;
```

  鎻忚堪
    姝よ皟鐢ㄨ繑鍥炵敱 fid 鏍囪瘑鐨勬枃浠剁殑灞炴€с€?

  閿欒
    濡傛灉甯︽湁璇?fid 鐨勫璞′笉瀛樺湪銆佷笉鍙闂紝鎴栬皟鐢ㄨ€呮病鏈夎幏鍙栧睘鎬х殑
    鏉冮檺锛屽氨鍙兘鍙戠敓閿欒銆?

  .. Note::

     璁稿鍐呮牳 FS 椹卞姩锛圠inux銆丯T 鍜?Windows 95锛夐渶瑕佽幏鍙栧睘鎬т互鍙?
     鐢ㄤ簬瀹炰緥鍖栧唴閮?"inode" 鎴?"FileHandle" 鐨?Fid銆傚湪姝ょ被绯荤粺涓婏紝閫氳繃
     鍦?Venus/鍐呮牳浜や簰灞備互鍙?RPC 灞傚皢 lookup 鍜?getattr 璋冪敤鍚堝苟锛屽彲浠?
     鏄捐憲鏀瑰杽鎬ц兘銆?

  杈撳叆鍙傛暟涓寘鍚殑 vattr 缁撴瀯鏄浣欑殑锛屽簲璇ュ垹闄ゃ€?


### 4.6. setattr


  鎽樿
    璁剧疆鏂囦欢鐨勫睘鎬с€?

  鍙傛暟

```
		struct cfs_setattr_in {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_setattr;
```

     out

	empty

  鎻忚堪
    缁撴瀯 attr 浠?BSD 椋庢牸濉厖瑕佹洿鏀圭殑灞炴€с€備笉鏇存敼鐨勫睘鎬ц涓?-1锛?
    闄?vtype 璁句负 VNON 澶栥€傚叾浠栧睘鎬ц涓鸿璧嬩簣鐨勫€笺€侳S 椹卞姩鍙兘璇锋眰
    鏇存敼鐨勫敮涓€灞炴€ф槸 mode銆乷wner銆乬roupid銆乤time銆乵time 鍜?ctime銆?
    杩斿洖鍊兼寚绀烘垚鍔熸垨澶辫触銆?

  閿欒
    鍙兘鍙戠敓鍚勭閿欒銆傚璞″彲鑳戒笉瀛樺湪銆佸彲鑳戒笉鍙闂紝鎴?Venus 鍙兘
    涓嶆巿浜堟潈闄愩€?


### 4.7. access


  鍙傛暟

```
		struct cfs_access_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_access;
```

     out

	empty

  鎻忚堪
    楠岃瘉瀵圭敱 VFid 鏍囪瘑鐨勩€佺敱 flags 鎻忚堪鐨勬搷浣滅殑瀵硅薄璁块棶鏄惁琚?
    鍏佽銆傜粨鏋滄寚绀烘槸鍚﹀皢鎺堜簣璁块棶鏉冮檺銆傚姟蹇呰浣忥紝Coda 浣跨敤 ACL 鏉ュ疄鏂?
    淇濇姢锛屾渶缁堢敱鏈嶅姟鍣ㄨ€岄潪瀹㈡埛绔潵寮哄埗绯荤粺鐨勫畨鍏ㄦ€с€傛璋冪敤鐨勭粨鏋滃皢
    鍙栧喅浜庣敤鎴锋槸鍚︽寔鏈?token銆?

  閿欒
    瀵硅薄鍙兘涓嶅瓨鍦紝鎴栨弿杩颁繚鎶ょ殑 ACL 鍙兘涓嶅彲璁块棶銆?


### 4.8. create


  鎽樿
    璋冪敤浠ュ垱寤烘枃浠躲€?

  鍙傛暟

```
		struct cfs_create_in {
		    ViceFid VFid;
		    struct coda_vattr attr;
		    int excl;
		    int mode;
		    char        *name;          /* Place holder for data. */
		} cfs_create;
```

     out锛?

```
		struct cfs_create_out {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_create;
```

  鎻忚堪
    姝や笂琛岃皟鐢ㄨ璋冪敤浠ヨ姹傚垱寤烘枃浠躲€傝鏂囦欢灏嗗湪鐢?VFid 鏍囪瘑鐨?
    鐩綍涓垱寤猴紝鍏跺悕绉颁负 name锛宮ode 涓?mode銆傚鏋滆缃簡 excl锛屼笖鏂囦欢
    宸插瓨鍦紝鍒欒繑鍥為敊璇€傚鏋?attr 涓殑 size 瀛楁璁句负闆讹紝鏂囦欢灏嗚鎴柇銆?
    鏂囦欢鐨?uid 鍜?gid 閫氳繃浣跨敤瀹?CRTOUID锛堟瀹忔槸骞冲彴鐩稿叧鐨勶級灏?CodaCred
    杞崲涓?uid 鏉ヨ缃€傛垚鍔熷悗杩斿洖鏂囦欢鐨?VFid 鍜屽睘鎬с€侰oda FS 椹卞姩閫氬父
    浼氬湪鍐呮牳灞備负鏂板璞″疄渚嬪寲涓€涓?vnode銆乮node 鎴栨枃浠跺彞鏌勩€?

  閿欒
    鍙兘鍙戠敓鍚勭閿欒銆傛潈闄愬彲鑳戒笉瓒炽€傚鏋滃璞″瓨鍦ㄤ笖涓嶆槸鏂囦欢锛屽湪
    Unix 涓嬭繑鍥為敊璇?EISDIR銆?

  .. Note::

     鍙傛暟鐨勬墦鍖呮晥鐜囧緢浣庯紝浼间箮琛ㄦ槑绯荤粺璋冪敤 creat 涓?VFS 鎿嶄綔 create
     涔嬮棿鐨勬贩娣嗐€俈FS 鎿嶄綔 create 浠呭湪鍒涘缓鏂板璞℃椂琚皟鐢ㄣ€傛 create 璋冪敤
     涓?Unix 鐗堟湰鐨勪笉鍚屼箣澶勫湪浜庯紝瀹冧笉琚皟鐢ㄦ潵杩斿洖鏂囦欢鎻忚堪绗︺€倀runcate
     鍜?exclusive 閫夐」杩炲悓 mode锛屽彲浠ョ畝鍗曞湴鍍?Unix 涓嬮偅鏍锋垚涓?mode 鐨?
     涓€閮ㄥ垎銆備笉搴旀湁 flags 鍙傛暟锛沠lags 鐢ㄤ簬 open(2) 涓互 READ 鎴?WRITE
     妯″紡杩斿洖鏂囦欢鎻忚堪绗︺€?

  鐢变簬澶у皬鍜?mtime 鍙戠敓浜嗗彉鍖栵紝鐩綍鐨勫睘鎬т篃搴旇繑鍥炪€?


### 4.9. mkdir


  鎽樿
    鍒涘缓鏂扮洰褰曘€?

  鍙傛暟

```
		struct cfs_mkdir_in {
		    ViceFid     VFid;
		    struct coda_vattr attr;
		    char        *name;          /* Place holder for data. */
		} cfs_mkdir;
```

     out锛?

```
		struct cfs_mkdir_out {
		    ViceFid VFid;
		    struct coda_vattr attr;
		} cfs_mkdir;
```

  鎻忚堪
    姝よ皟鐢ㄧ被浼间簬 create锛屼絾鍒涘缓涓€涓洰褰曘€傝緭鍏ュ弬鏁颁腑浠呬娇鐢?mode
    瀛楁杩涜鍒涘缓銆傛垚鍔熷垱寤哄悗锛岃繑鍥炵殑 attr 鍖呭惈鏂扮洰褰曠殑灞炴€с€?

  閿欒
    鍚?create銆?

  .. Note::

     杈撳叆鍙傛暟搴旀敼涓?mode 鑰岄潪灞炴€с€?

  鐖剁洰褰曠殑灞炴€у簲杩斿洖锛屽洜涓哄ぇ灏忓拰 mtime 鍙戠敓浜嗗彉鍖栥€?


### 4.10. link


  鎽樿
    鍒涘缓鍒扮幇鏈夋枃浠剁殑閾炬帴銆?

  鍙傛暟

```
		struct cfs_link_in {
		    ViceFid sourceFid;          /* cnode to link *to* */
		    ViceFid destFid;            /* Directory in which to place link */
		    char        *tname;         /* Place holder for data. */
		} cfs_link;
```

     out

	empty

  鎻忚堪
    姝よ皟鐢ㄥ湪鐢?destFid 鏍囪瘑鐨勭洰褰曚腑锛屼互鍚嶇О tname 鍒涘缓鍒?
    sourceFid 鐨勯摼鎺ャ€傛簮蹇呴』椹荤暀鍦ㄧ洰鏍囩殑鐖剁洰褰曚腑锛屽嵆婧愬繀椤诲叿鏈夌埗鐩綍
    destFid锛屼篃灏辨槸璇?Coda 涓嶆敮鎸佽法鐩綍纭摼鎺ャ€傚彧鏈夎繑鍥炲€肩浉鍏炽€傚畠鎸囩ず
    鎴愬姛鎴栧け璐ョ被鍨嬨€?

  閿欒
    甯歌鐨勯敊璇兘鍙兘鍙戠敓銆?


### 4.11. symlink


  鎽樿
    鍒涘缓绗﹀彿閾炬帴

  鍙傛暟

```
		struct cfs_symlink_in {
		    ViceFid     VFid;          /* Directory to put symlink in */
		    char        *srcname;
		    struct coda_vattr attr;
		    char        *tname;
		} cfs_symlink;
```

     out

	none

  鎻忚堪
    鍒涘缓绗﹀彿閾炬帴銆傝閾炬帴灏嗘斁缃湪鐢?VFid 鏍囪瘑鐨勭洰褰曚腑锛屽苟鍛藉悕涓?
    tname銆傚畠搴旀寚鍚戣矾寰勫悕 srcname銆傛柊鍒涘缓瀵硅薄鐨勫睘鎬у皢璁句负 attr銆?

  .. Note::

     鐢变簬鐩爣鐩綍鐨勫ぇ灏忓彂鐢熷彉鍖栵紝搴旇繑鍥炲叾灞炴€с€?


### 4.12. remove


  鎽樿
    鍒犻櫎鏂囦欢

  鍙傛暟

```
		struct cfs_remove_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_remove;
```

     out

	none

  鎻忚堪
    鍒犻櫎鐢?VFid 鏍囪瘑鐨勭洰褰曚腑鍚嶄负 cfs_remove_in.name 鐨勬枃浠躲€?

  .. Note::

     鐢变簬鐩綍鐨?mtime 鍜屽ぇ灏忓彲鑳藉彂鐢熷彉鍖栵紝搴旇繑鍥炲叾灞炴€с€?


### 4.13. rmdir


  鎽樿
    鍒犻櫎鐩綍

  鍙傛暟

```
		struct cfs_rmdir_in {
		    ViceFid     VFid;
		    char        *name;          /* Place holder for data. */
		} cfs_rmdir;
```

     out

	none

  鎻忚堪
    浠庣敱 VFid 鏍囪瘑鐨勭洰褰曚腑鍒犻櫎鍚嶄负 'name' 鐨勭洰褰曘€?

  .. Note:: 鐢变簬鐖剁洰褰曠殑 mtime 鍜屽ぇ灏忓彲鑳藉彂鐢熷彉鍖栵紝搴旇繑鍥炲叾鐖剁洰褰曠殑灞炴€с€?


### 4.14. readlink


  鎽樿
    璇诲彇绗﹀彿閾炬帴鐨勫€笺€?

  鍙傛暟

```
		struct cfs_readlink_in {
		    ViceFid VFid;
		} cfs_readlink;
```

     out锛?

```
		struct cfs_readlink_out {
		    int count;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_readlink;
```

  鎻忚堪
    姝や緥绋嬪皢鐢?VFid 鏍囪瘑鐨勭鍙烽摼鎺ョ殑鍐呭璇诲叆缂撳啿鍖?data銆俤ata
    缂撳啿鍖哄繀椤昏兘澶熷绾充换浣曢暱杈?CFS_MAXNAMLEN 鐨勫悕绉帮紙PATH 杩樻槸 NAM锛燂級銆?

  閿欒
    鏃犲紓甯搁敊璇€?


### 4.15. open


  鎽樿
    鎵撳紑鏂囦欢銆?

  鍙傛暟

```
		struct cfs_open_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_open;
```

     out锛?

```
		struct cfs_open_out {
		    dev_t       dev;
		    ino_t       inode;
		} cfs_open;
```

  鎻忚堪
    姝よ姹傝姹?Venus 灏?VFid 鏍囪瘑鐨勬枃浠舵斁鍏ュ叾缂撳瓨锛屽苟璁颁笅璋冪敤
    杩涚▼甯屾湜浠?open(2) 涓殑 flags 鎵撳紑瀹冦€傝繑鍥炵粰鍐呮牳鐨勫€煎湪 Unix 鍜?
    Windows 绯荤粺涔嬮棿鏈夋墍涓嶅悓銆傚浜?Unix 绯荤粺锛孋oda FS 椹卞姩琚憡鐭ュ鍣?
    鏂囦欢鍦?dev 鍜?inode 瀛楁涓殑璁惧鍜?inode 鍙枫€傚浜?Windows锛岃繑鍥?
    瀹瑰櫒鏂囦欢鐨勮矾寰勭粰鍐呮牳銆?

  .. Note::

     褰撳墠 cfs_open_out 缁撴瀯娌℃湁姝ｇ‘閫傞厤浠ュ鐞?Windows 鎯呭喌銆傛渶濂藉疄鐜?
     涓や釜涓婅璋冪敤锛屼竴涓互瀹瑰櫒鏂囦欢鍚嶇О涓虹洰鏍囷紝鍙︿竴涓互瀹瑰櫒鏂囦欢 inode
     涓虹洰鏍囥€?


### 4.16. close


  鎽樿
    鍏抽棴鏂囦欢锛屽湪鏈嶅姟鍣ㄤ笂鏇存柊瀹冦€?

  鍙傛暟

```
		struct cfs_close_in {
		    ViceFid     VFid;
		    int flags;
		} cfs_close;
```

     out

	none

  鎻忚堪
    鍏抽棴鐢?VFid 鏍囪瘑鐨勬枃浠躲€?

  .. Note::

     flags 鍙傛暟鏄吉閫犵殑涓旀湭琚娇鐢ㄣ€傜劧鑰岋紝Venus 鐨勪唬鐮佺暀鏈夊鐞?execp
     杈撳叆瀛楁鐨勪綑鍦帮紝鍙兘搴斾娇鐢ㄦ瀛楁鏉ュ憡鐭?Venus 鏂囦欢宸插叧闂絾浠嶈
     鍐呭瓨鏄犲皠浠ユ墽琛屻€俈enus 鐨?vproc_vfscalls 涓湁鍏充簬鑾峰彇涓庝笉鑾峰彇鏁版嵁鐨?
     娉ㄩ噴銆傝繖鐪嬭捣鏉ュ緢鍌汇€傚鏋滄枃浠舵鍦ㄥ叧闂紝瀹瑰櫒鏂囦欢涓殑鏁版嵁灏嗘垚涓?
     鏂版暟鎹€傝繖閲?execp 鏍囧織鍙兘鍙堜細鍙備笌鍒堕€犳贩涔憋細褰撳墠 Venus 鍙兘璁や负
     鏂囦欢鍦ㄤ粛琚唴瀛樻槧灏勬椂灏卞彲浠ヤ粠缂撳瓨涓埛鏂般€傝繖闇€瑕佽鐞嗚В銆?


### 4.17. ioctl


  鎽樿
    瀵规枃浠舵墽琛?ioctl銆傝繖鍖呮嫭 pioctl 鎺ュ彛銆?

  鍙傛暟

```
		struct cfs_ioctl_in {
		    ViceFid VFid;
		    int cmd;
		    int len;
		    int rwflag;
		    char *data;                 /* Place holder for data. */
		} cfs_ioctl;
```

     out锛?

```
		struct cfs_ioctl_out {
		    int len;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_ioctl;
```

  鎻忚堪
    瀵规枃浠舵墽琛?ioctl 鎿嶄綔銆俢ommand銆乴en 鍜?data 鍙傛暟鐓у父濉厖銆?
    flags 涓嶈 Venus 浣跨敤銆?

  .. Note::

     鍙︿竴涓吉閫犵殑鍙傛暟銆俧lags 鏈浣跨敤銆俈enus 浠ｇ爜涓叧浜?PREFETCHING
     鐨勪簨鏄粈涔堬紵


### 4.18. rename


  鎽樿
    閲嶅懡鍚嶄竴涓?fid銆?

  鍙傛暟

```
		struct cfs_rename_in {
		    ViceFid     sourceFid;
		    char        *srcname;
		    ViceFid destFid;
		    char        *destname;
		} cfs_rename;
```

     out

	none

  鎻忚堪
    灏?sourceFid 鐩綍涓悕涓?srcname 鐨勫璞￠噸鍛藉悕涓?destFid 涓?
    鐨?destname銆傞噸瑕佺殑鏄悕绉?srcname 鍜?destname 鏄互 0 缁撳熬鐨勫瓧绗︿覆銆?
    Unix 鍐呮牳涓殑瀛楃涓插苟涓嶆€绘槸浠?null 缁撳熬銆?


### 4.19. readdir


  鎽樿
    璇诲彇鐩綍椤广€?

  鍙傛暟

```
		struct cfs_readdir_in {
		    ViceFid     VFid;
		    int count;
		    int offset;
		} cfs_readdir;
```

     out锛?

```
		struct cfs_readdir_out {
		    int size;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_readdir;
```

  鎻忚堪
    浠?VFid 寮€濮嬶紝浠?offset 鍋忕Щ澶勮鍙栫洰褰曢」锛屾渶澶氳鍙?count
    瀛楄妭銆傚皢鏁版嵁杩斿洖鍒?data 涓紝骞跺皢澶у皬杩斿洖鍒?size 涓€?

  .. Note::

     姝よ皟鐢ㄦ湭琚娇鐢ㄣ€俁eaddir 鎿嶄綔鍒╃敤瀹瑰櫒鏂囦欢銆傛垜浠皢鍦ㄨ繘琛屼腑鐨?
     鐩綍鏀归€犳湡闂撮噸鏂拌瘎浼拌繖涓€鐐广€?


### 4.20. vget


  鎽樿
    鎸囩ず Venus 鎵ц FSDB->Get銆?

  鍙傛暟

```
		struct cfs_vget_in {
		    ViceFid VFid;
		} cfs_vget;
```

     out锛?

```
		struct cfs_vget_out {
		    ViceFid VFid;
		    int vtype;
		} cfs_vget;
```

  鎻忚堪
    姝や笂琛岃皟鐢ㄨ姹?Venus 瀵圭敱 VFid 鏍囪鐨?fsobj 鎵ц get 鎿嶄綔銆?

  .. Note::

     姝ゆ搷浣滄湭琚娇鐢ㄣ€傜劧鑰岋紝瀹冩瀬鍏舵湁鐢紝鍥犱负瀹冨彲鐢ㄤ簬澶勭悊璇?鍐欏唴瀛?
     鏄犲皠鏂囦欢銆傝繖浜涙枃浠跺彲浠ヤ娇鐢?vget 鍦?Venus 缂撳瓨涓?鍥哄畾"锛屽苟浣跨敤
     inactive 閲婃斁銆?


### 4.21. fsync


  鎽樿
    鍛婅瘔 Venus 鏇存柊鏂囦欢鐨?RVM 灞炴€с€?

  鍙傛暟

```
		struct cfs_fsync_in {
		    ViceFid VFid;
		} cfs_fsync;
```

     out

	none

  鎻忚堪
    瑕佹眰 Venus 鏇存柊瀵硅薄 VFid 鐨?RVM 灞炴€с€傝繖搴斾綔涓哄唴鏍哥骇 fsync
    绫诲瀷璋冪敤鐨勪竴閮ㄥ垎琚皟鐢ㄣ€傜粨鏋滄寚绀哄悓姝ユ槸鍚︽垚鍔熴€?

  .. Note:: Linux 鏈疄鐜版璋冪敤銆傚畠搴旇瀹炵幇銆?


### 4.22. inactive


  鎽樿
    鍛婅瘔 Venus 涓€涓?vnode 涓嶅啀琚娇鐢ㄣ€?

  鍙傛暟

```
		struct cfs_inactive_in {
		    ViceFid VFid;
		} cfs_inactive;
```

     out

	none

  鎻忚堪
    姝ゆ搷浣滆繑鍥?EOPNOTSUPP銆?

  .. Note:: 杩欎篃璁稿簲璇ヨ鍒犻櫎銆?


### 4.23. rdwr


  鎽樿
    浠庢枃浠惰鎴栧啓

  鍙傛暟

```
		struct cfs_rdwr_in {
		    ViceFid     VFid;
		    int rwflag;
		    int count;
		    int offset;
		    int ioflag;
		    caddr_t     data;           /* Place holder for data. */
		} cfs_rdwr;
```

     out锛?

```
		struct cfs_rdwr_out {
		    int rwflag;
		    int count;
		    caddr_t     data;   /* Place holder for data. */
		} cfs_rdwr;
```

  鎻忚堪
    姝や笂琛岃皟鐢ㄨ姹?Venus 浠庢枃浠惰鎴栧啓銆?

  .. Note::

    瀹冨簲璇ヨ鍒犻櫎锛屽洜涓鸿/鍐欐搷浣滄案杩滀笉浼氬埌杈?Venus锛岃繖杩濊儗浜?Coda
    鐨勭悊蹇点€傛湁浜哄憡璇夋垜璇ユ搷浣滀笉璧蜂綔鐢ㄣ€傚畠褰撳墠鏈浣跨敤銆?


### 4.24. odymount


  鎽樿
    鍏佽鍦ㄤ竴涓?Unix 鎸傝浇鐐逛笂鎸傝浇澶氫釜 Coda "鏂囦欢绯荤粺"銆?

  鍙傛暟

```
		struct ody_mount_in {
		    char        *name;          /* Place holder for data. */
		} ody_mount;
```

     out锛?

```
		struct ody_mount_out {
		    ViceFid VFid;
		} ody_mount;
```

  鎻忚堪
    瑕佹眰 Venus 杩斿洖鍚嶄负 name 鐨?Coda 绯荤粺鐨?rootfid銆俧id 鍦?
    VFid 涓繑鍥炪€?

  .. Note::

     姝よ皟鐢ㄦ浘琚?David 鐢ㄤ簬鍔ㄦ€侀泦鍚堬紙dynamic sets锛夈€傚畠搴旇琚垹闄わ紝
     鍥犱负瀹冨湪 VFS 鎸傝浇鍖哄煙閫犳垚鎸囬拡涓涙灄銆侰oda 鏈韩涓嶄娇鐢ㄥ畠銆俈enus 鏈?
     瀹炵幇姝よ皟鐢ㄣ€?


### 4.25. ody_lookup


  鎽樿
    鏌ユ壘鏌愮墿銆?

  鍙傛暟
     in

	irrelevant

     out

	irrelevant

  .. Note:: 鍘绘帀瀹冦€俈enus 鏈疄鐜版璋冪敤銆?


### 4.26. ody_expand


  鎽樿
    灞曞紑鍔ㄦ€侀泦鍚堜腑鐨勬煇鐗┿€?

  鍙傛暟
     in

	irrelevant

     out

	irrelevant

  .. Note:: 鍘绘帀瀹冦€俈enus 鏈疄鐜版璋冪敤銆?


### 4.27. prefetch


  鎽樿
    棰勫彇鍔ㄦ€侀泦鍚堛€?

  鍙傛暟
     in

	Not documented.

     out

	Not documented.

  鎻忚堪
    Venus 鐨?worker.cc 鏀寔姝よ皟鐢紝灏界娉ㄦ剰鍒板畠涓嶈捣浣滅敤銆傛涓嶅鎬紝
    鍥犱负鍐呮牳涓嶆敮鎸佸畠銆傦紙ODY_PREFETCH 涓嶆槸宸插畾涔夌殑鎿嶄綔锛夈€?

  .. Note:: 鍘绘帀瀹冦€傚畠涓嶈捣浣滅敤锛屼笖 Coda 鏈娇鐢ㄥ畠銆?


### 4.28. signal


  鎽樿
    鍚?Venus 鍙戦€佸叧浜庝笂琛岃皟鐢ㄧ殑淇″彿銆?

  鍙傛暟
     in

	none

     out

	not applicable.

  鎻忚堪
    杩欐槸涓€涓彂缁?Venus 鐨勫甫澶栵紙out-of-band锛変笂琛岃皟鐢紝鐢ㄤ簬閫氱煡
    Venus 璋冪敤杩涚▼鍦?Venus 浠庤緭鍏ラ槦鍒楄鍙栨秷鎭悗鏀跺埌浜嗕竴涓?signal銆?
    Venus 搴旀竻鐞嗚鎿嶄綔銆?

  閿欒
    涓嶇粰鍑哄洖澶嶃€?

     鎴戜滑闇€瑕佹洿濂藉湴鐞嗚В Venus 闇€瑕佹竻鐞嗕粈涔堬紝浠ュ強瀹冩槸鍚︽纭湴鎵ц浜?
     娓呯悊銆傛垜浠繕闇€瑕佹纭鐞嗘瘡涓郴缁熻皟鐢ㄥ涓?upcall 鐨勬儏鍐点€備簡瑙ｅ湪
     upcall 涔嬪悗 Venus 涓彂鐢熶簡鍝簺鐘舵€佸彉鍖栧緢閲嶈锛屽唴鏍歌礋璐ｉ€氱煡 Venus
     娓呯悊杩欎簺鍙樺寲锛堜緥濡?open 鑲畾鏄繖鏍风殑鐘舵€佸彉鍖栵紝浣嗚澶氬叾浠栫殑涔熻
     涓嶆槸锛夈€?


## 5. 杩蜂綘缂撳瓨涓庝笅琛岃皟鐢紙downcall锛?


  Coda FS 椹卞姩鍙互缂撳瓨 lookup 鍜?access 涓婅璋冪敤鐨勭粨鏋滐紝浠ラ檺鍒朵笂琛岃皟鐢?
  鐨勯鐜囥€備笂琛岃皟鐢ㄦ槸鏈変唬浠风殑锛屽洜涓洪渶瑕佽繘琛岃繘绋嬩笂涓嬫枃鍒囨崲銆傜紦瀛樹俊鎭殑
  瀵瑰簲闈㈡槸锛孷enus 灏嗛€氱煡 FS 椹卞姩缂撳瓨鏉＄洰蹇呴』琚埛鏂版垨閲嶅懡鍚嶃€?

  鍐呮牳浠ｇ爜閫氬父蹇呴』缁存姢涓€涓粨鏋勶紝灏嗗唴閮ㄦ枃浠跺彞鏌勶紙鍦?BSD 涓О涓?vnodes锛?
  鍦?Linux 涓О涓?inodes锛屽湪 Windows 涓О涓?FileHandles锛変笌 Venus 缁存姢鐨?
  ViceFid 鍏宠仈璧锋潵銆傚師鍥犳槸锛屼负浜嗚繘琛屼笂琛岃皟鐢ㄥ苟浣跨敤涓婅璋冪敤鐨勭粨鏋滐紝
  闇€瑕侀绻佸湴鏉ュ洖杞崲銆傛绫婚摼鎺ュ璞＄О涓?cnodes銆?

  褰撳墠鐨勮糠浣犵紦瀛樺疄鐜版嫢鏈夌殑缂撳瓨鏉＄洰璁板綍濡備笅锛?

  1. 鏂囦欢鐨勫悕绉?

  2. 鍖呭惈璇ュ璞＄殑鐩綍鐨?cnode

  3. 鍏佽杩涜 lookup 鐨?CodaCred 鍒楄〃

  4. 璇ュ璞＄殑 cnode

  Coda FS 椹卞姩涓殑 lookup 璋冪敤鍙互閫氳繃浼犻€掑叾鍚嶇О銆佺洰褰曞拰璋冪敤鑰呯殑
  CodaCred锛屼粠缂撳瓨璇锋眰鎵€闇€瀵硅薄鐨?cnode銆傜紦瀛樺皢杩斿洖 cnode锛屾垨鎸囩ず鎵句笉鍒般€?
  Coda FS 椹卞姩鍦ㄤ慨鏀规垨鍒犻櫎瀵硅薄鏃跺繀椤诲皬蹇冨湴浣跨紦瀛樻潯鐩け鏁堛€?

  褰?Venus 鑾峰緱鎸囩ず缂撳瓨鏉＄洰涓嶅啀鏈夋晥鐨勪俊鎭椂锛屽畠灏嗗悜鍐呮牳鍙戝嚭涓嬭璋冪敤
  锛坉owncall锛夈€備笅琛岃皟鐢ㄨ Coda FS 椹卞姩鎷︽埅锛屽苟瀵艰嚧濡備笅鎵€杩扮殑缂撳瓨澶辨晥銆?
  Coda FS 椹卞姩涓嶈繑鍥為敊璇紝闄ら潪涓嬭璋冪敤鏁版嵁鏃犳硶璇诲叆鍐呮牳鍐呭瓨銆?


### 5.1. INVALIDATE


  鍏充簬姝よ皟鐢ㄦ病鏈夊彲鐢ㄤ俊鎭€?


### 5.2. FLUSH


  鍙傛暟
    None

  鎽樿
    瀹屽叏鍒锋柊鍚嶇О缂撳瓨銆?

  鎻忚堪
    Venus 鍦ㄥ惎鍔ㄥ拰閫€鍑烘椂鍙戝嚭姝よ皟鐢ㄣ€傝繖鏄负浜嗛槻姝繚鐣欓檲鏃х殑缂撳瓨
    淇℃伅銆傛煇浜涙搷浣滅郴缁熷厑璁稿姩鎬佸叧闂唴鏍稿悕绉扮紦瀛樸€傚綋杩欐牱鍋氭椂锛屼細杩涜
    姝や笅琛岃皟鐢ㄣ€?


### 5.3. PURGEUSER


  鍙傛暟

```
	  struct cfs_purgeuser_out {/* CFS_PURGEUSER is a venus->kernel call */
	      struct CodaCred cred;
	  } cfs_purgeuser;
```

  鎻忚堪
    绉婚櫎缂撳瓨涓墍鏈夋惡甯﹁ Cred 鐨勬潯鐩€傚綋鐢ㄦ埛鐨?token 杩囨湡鎴栬
    鍒锋柊鏃跺彂鍑烘璋冪敤銆?


### 5.4. ZAPFILE


  鍙傛暟

```
	  struct cfs_zapfile_out {  /* CFS_ZAPFILE is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_zapfile;
```

  鎻忚堪
    绉婚櫎鎵€鏈夊叿鏈夛紙dir vnode, name锛夊鐨勬潯鐩€傝繖鏄敱浜?vnode 鐨?
    缂撳瓨灞炴€уけ鏁堣€屽彂鍑虹殑銆?

  .. Note::

     鍦?NetBSD 鍜?Mach 涓璋冪敤鍛藉悕涓嶆纭€傝糠浣犵紦瀛?zapfile 渚嬬▼
     閲囩敤涓嶅悓鐨勫弬鏁般€侺inux 鏈纭疄鐜板睘鎬х殑澶辨晥銆?


### 5.5. ZAPDIR


  鍙傛暟

```
	  struct cfs_zapdir_out {   /* CFS_ZAPDIR is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_zapdir;
```

  鎻忚堪
    绉婚櫎缂撳瓨涓綅浜庣洰褰?CodaFid 涓殑鎵€鏈夋潯鐩紝浠ュ強璇ョ洰褰曠殑鎵€鏈?
    瀛愰」銆傚綋 Venus 鏀跺埌璇ョ洰褰曠殑鍥炶皟鏃跺彂鍑烘璋冪敤銆?


### 5.6. ZAPVNODE


  鍙傛暟

```
	  struct cfs_zapvnode_out { /* CFS_ZAPVNODE is a venus->kernel call */
	      struct CodaCred cred;
	      ViceFid VFid;
	  } cfs_zapvnode;
```

  鎻忚堪
    绉婚櫎缂撳瓨涓墍鏈夋惡甯﹀弬鏁颁腑 cred 鍜?VFid 鐨勬潯鐩€傛涓嬭璋冪敤
    鍙兘浠庢湭琚彂鍑恒€?


### 5.7. PURGEFID


  鍙傛暟

```
	  struct cfs_purgefid_out { /* CFS_PURGEFID is a venus->kernel call */
	      ViceFid CodaFid;
	  } cfs_purgefid;
```

  鎻忚堪
    鍒锋柊鏂囦欢鐨勫睘鎬с€傚鏋滃畠鏄洰褰曪紙濂囨暟 vnode锛夛紝鍒欎粠鍚嶇О缂撳瓨
    涓竻闄ゅ叾瀛愰」锛屽苟浠庡悕绉扮紦瀛樹腑绉婚櫎璇ユ枃浠躲€?


### 5.8. REPLACE


  鎽樿
    鏇挎崲涓€缁勫悕绉扮殑 Fid銆?

  鍙傛暟

```
	  struct cfs_replace_out { /* cfs_replace is a venus->kernel call */
	      ViceFid NewFid;
	      ViceFid OldFid;
	  } cfs_replace;
```

  鎻忚堪
    姝や緥绋嬪皢鍚嶇О缂撳瓨涓殑 ViceFid 鏇挎崲涓哄彟涓€涓€傛坊鍔犲畠鏄负浜嗗厑璁?
    Venus 鍦ㄩ噸鏂伴泦鎴愶紙reintegration锛夋湡闂达紝鍗充娇杩欎簺 fid 鐨勫紩鐢ㄨ鏁颁笉涓洪浂锛?
    涔熻兘鐢ㄥ叏灞€ fid 鏇挎崲鏂紑杩炴帴鏃舵湰鍦板垎閰嶇殑涓存椂 fid銆?


## 6. 鍒濆鍖栦笌娓呯悊


  鏈妭绠€瑕佹彁绀?Coda FS 椹卞姩鍦ㄥ惎鍔ㄣ€佸叧闂垨 Venus 鏁呴殰鏃跺簲鍏峰鐨勫彲鍙?
  鐗规€с€傚湪璁ㄨ涔嬪墠锛岄噸鐢充竴涓?Coda FS 椹卞姩缁存姢浠ヤ笅鏁版嵁鏄湁鐢ㄧ殑锛?


  1. 娑堟伅闃熷垪

  2. cnodes

  3. 鍚嶇О缂撳瓨鏉＄洰

     鍚嶇О缂撳瓨鏉＄洰瀹屽叏鐢遍┍鍔ㄧ鏈夛紝鍥犳鍙互杞绘澗鎿嶄綔銆傛秷鎭槦鍒楅€氬父
     鏈夋槑纭殑鍒濆鍖栧拰閿€姣佺偣銆俢nodes 鍒欒寰寰楀銆傜敤鎴疯繘绋嬪湪 Coda
     鏂囦欢绯荤粺涓寔鏈夊紩鐢ㄨ鏁帮紝娓呯悊 cnodes 鍙兘寰堝洶闅俱€?

  瀹冨彲浠ラ€氳繃濡備笅鏂瑰紡鏀跺埌璇锋眰锛?

  1. 娑堟伅瀛愮郴缁?

  2. VFS 灞?

  3. pioctl 鎺ュ彛

     褰撳墠 pioctl 閫氳繃 Coda 鐨?VFS 浼犻€掞紝鍥犳鎴戜滑鍙互绫讳技鍦板鐞嗚繖浜涖€?


### 6.1. 闇€姹?


  搴旀弧瓒充互涓嬮渶姹傦細

  1. 娑堟伅闃熷垪搴旀湁鎵撳紑鍜屽叧闂緥绋嬨€傚湪 Unix 涓婏紝瀛楃璁惧鐨勬墦寮€灏辨槸
     姝ょ被渚嬬▼銆?

    - 鎵撳紑涔嬪墠锛屼笉鑳芥斁缃换浣曟秷鎭€?

    - 鎵撳紑灏嗙Щ闄や换浣曚粛鍦ㄦ寕璧风殑鏃ф秷鎭€?

    - 鍏抽棴灏嗛€氱煡浠讳綍鐫＄湢鐨勮繘绋嬶紝瀹冧滑鐨?upcall 鏃犳硶瀹屾垚銆?

    - 鍏抽棴灏嗛噴鏀炬秷鎭槦鍒楀垎閰嶇殑鎵€鏈夊唴瀛樸€?

  2. 鍦ㄦ墦寮€鏃讹紝鍚嶇О缂撳瓨搴旇鍒濆鍖栦负绌虹姸鎬併€?

  3. 鍦ㄦ秷鎭槦鍒楁墦寮€涔嬪墠锛屾墍鏈?VFS 鎿嶄綔閮藉皢澶辫触銆傚垢杩愮殑鏄紝杩欏彲浠ラ€氳繃
     纭繚鎸傝浇 Coda 鏂囦欢绯荤粺鍦ㄦ墦寮€涔嬪墠涓嶈兘鎴愬姛鏉ュ疄鐜般€?

  4. 鍏抽棴闃熷垪鍚庯紝娌℃湁浠讳綍 VFS 鎿嶄綔鑳芥垚鍔熴€傝繖閲岄渶瑕佸皬蹇冿紝鍥犱负灏戞暟鎿嶄綔
     锛坙ookup銆乺ead/write銆乺eaddir锛夊彲浠ュ湪娌℃湁 upcall 鐨勬儏鍐典笅杩涜銆?
     杩欎簺蹇呴』琚樉寮忛樆姝€?

  5. 鍏抽棴鏃讹紝鍚嶇О缂撳瓨搴旇鍒锋柊骞剁鐢ㄣ€?

  6. 鎵€鏈夌敱 cnodes 鎸佹湁鐨勫唴瀛樺彲浠ュ湪涓嶄緷璧?upcall 鐨勬儏鍐典笅閲婃斁銆?

  7. 鍗歌浇鏂囦欢绯荤粺鍙互鍦ㄤ笉渚濊禆 upcall 鐨勬儏鍐典笅瀹屾垚銆?

  8. 濡傛灉 Venus 鏃犳硶鑾峰彇 rootfid 鎴?rootfid 鐨勫睘鎬э紝鎸傝浇 Coda 鏂囦欢
     绯荤粺搴斾紭闆呭湴澶辫触銆傚悗鑰呮渶濂界敱 Venus 鍦ㄥ皾璇曟寕杞戒箣鍓嶈幏鍙栬繖浜涘璞?
     鏉ュ疄鐜般€?

     NetBSD 灏ゅ叾鏄?Linux 灏氭湭瀹屽叏瀹炵幇涓婅堪闇€姹傘€備负浜嗛『鐣呰繍琛岋紝杩欓渶瑕?
     琚籂姝ｃ€?
