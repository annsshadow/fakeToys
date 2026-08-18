
## HTM锛堢‖浠惰窡韪畯锛孒ardware Trace Macro锛?

Athira Rajeev, 2 Mar 2025

    :depth: 3


## 鍩烘湰姒傝堪


H_HTM 鐢ㄤ綔鎵ц纭欢璺熻釜瀹忥紙HTM锛夊姛鑳界殑鎺ュ彛锛屽寘鎷?HTM 鏁版嵁鐨勮缃€侀厤缃€佹帶鍒跺拰杞偍銆備娇鐢?HTM
闇€瑕佽缃?HTM 缂撳啿鍖猴紝骞朵笖 HTM 鎿嶄綔鍙互浣跨敤 H_HTM hcall 杩涜鎺у埗銆傝 hcall 鍙互浠庡垎鍖哄唴閮ㄩ拡瀵?绯荤粺鐨勪换浣曟牳蹇?鑺墖璋冪敤銆傝浣跨敤姝ょ壒鎬э紝鍦?/sys/kernel/debug/powerpc 涓嬪瓨鍦ㄤ竴涓悕涓?鈥渉tmdump鈥?鐨?debugfs 鏂囦欢澶广€?

## HTM debugfs 浣跨敤绀轰緥


  #  ls /sys/kernel/debug/powerpc/htmdump/
  coreindexonchip  htmcaps  htmconfigure  htmflags  htminfo  htmsetup
  htmstart  htmstatus  htmtype  nodalchipindex  nodeindex  trace

姣忎釜鏂囦欢鐨勮缁嗕俊鎭細

- nodeindex銆乶odalchipindex銆乧oreindexonchip 鎸囧畾瑕佷负鍝釜鍒嗗尯閰嶇疆 HTM銆?- htmtype锛氭寚瀹?HTM 鐨勭被鍨嬨€傛敮鎸佺殑鐩爣鏄?hardwareTarget銆?- trace锛氱敤浜庤鍙?HTM 鏁版嵁銆?- htmconfigure锛氶厤缃?鍙栨秷閰嶇疆 HTM銆傚悜鏂囦欢鍐欏叆 1 灏嗛厤缃窡韪紝鍐欏叆 0 灏嗗彇娑堥厤缃€?- htmstart锛氬惎鍔?鍋滄 HTM銆傚悜鏂囦欢鍐欏叆 1 灏嗗惎鍔ㄨ窡韪紝鍐欏叆 0 灏嗗仠姝㈣窡韪€?- htmstatus锛氳幏鍙?HTM 鐨勭姸鎬併€傝繖鐢ㄤ簬浜嗚В姣忔鎿嶄綔鍚庣殑 HTM 鐘舵€併€?- htmsetup锛氳缃?HTM 缂撳啿鍖哄ぇ灏忋€侶TM 缂撳啿鍖哄ぇ灏忎负 2 鐨勫箓
- htminfo锛氭彁渚涚郴缁熷鐞嗗櫒閰嶇疆璇︾粏淇℃伅銆傝繖鐢ㄤ簬浜嗚В nodeindex銆乶odalchipindex銆乧oreindexonchip
  鐨勯€傚綋鍊笺€?- htmcaps锛氭彁渚?HTM 鐨勮兘鍔涳紝濡傛渶灏?鏈€澶х紦鍐插尯澶у皬銆丠TM 鏀寔浣曠璺熻釜绛夈€?- htmflags锛氬厑璁稿悜 hcall 浼犻€掓爣蹇椼€傜洰鍓嶆敮鎸佹帶鍒?HTM 缂撳啿鍖虹殑鍥炵粫銆?
瑕佹煡鐪嬬郴缁熷鐞嗗櫒閰嶇疆璇︾粏淇℃伅锛?

  # cat /sys/kernel/debug/powerpc/htmdump/htminfo > htminfo_file

缁撴灉鍙互浣跨敤 hexdump 杩涜瑙ｆ瀽銆?
瑕佷负 nodeindex 涓?0銆乶odalchipindex 涓?1銆乧oreindexonchip 涓?12 鐨勫垎鍖烘敹闆?HTM 璺熻釜


  # cd /sys/kernel/debug/powerpc/htmdump/
  # echo 2 > htmtype
  # echo 33 > htmsetup ( 璁剧疆 8GB 鍐呭瓨鐢ㄤ簬 HTM 缂撳啿鍖猴紝鏁板瓧涓?2 鐨勫箓澶у皬 )

杩欓渶瑕侀噸鍚?CEC 浠ュ垎閰?HTM 缂撳啿鍖恒€?

  # cd /sys/kernel/debug/powerpc/htmdump/
  # echo 2 > htmtype
  # echo 0 > nodeindex
  # echo 1 > nodalchipindex
  # echo 12 > coreindexonchip
  # echo 1 > htmflags     # 涓?HTM 缂撳啿鍖鸿缃?noWrap
  # echo 1 > htmconfigure # 閰嶇疆 HTM
  # echo 1 > htmstart     # 鍚姩 HTM
  # echo 0 > htmstart     # 鍋滄 HTM
  # echo 0 > htmconfigure # 鍙栨秷閰嶇疆 HTM
  # cat htmstatus         # 灏?HTM 鏉＄洰鐘舵€佷綔涓烘暟鎹浆鍌?
涓婅堪鎿嶄綔灏嗚缃?htmtype 鍜屾牳蹇冭缁嗕俊鎭紝鐒跺悗鎵ц鐩稿簲鐨?HTM 鎿嶄綔銆?
## 璇诲彇 HTM 璺熻釜鏁版嵁


寮€濮嬭窡韪敹闆嗗悗锛岃繍琛屼綘鎰熷叴瓒ｇ殑宸ヤ綔璐熻浇銆傚湪鎵€闇€鏃堕棿鍚庡仠姝㈣窡韪敹闆嗭紝骞惰鍙栬窡韪枃浠躲€?

  # cat /sys/kernel/debug/powerpc/htmdump/trace > trace_file

姝よ窡韪枃浠跺皢鍖呭惈鍦ㄥ伐浣滆礋杞芥墽琛屾湡闂存敹闆嗙殑鐩稿叧鎸囦护璺熻釜銆傚畠鍙綔涓鸿窡韪В鐮佸櫒鐨勮緭鍏ユ枃浠舵潵鐞嗚В鏁版嵁銆?
## 浣跨敤 HTM debugfs 鎺ュ彛鐨勫ソ澶?

鐜板湪鍙互浠庣郴缁熺殑浠讳綍鍒嗗尯鍐呴儴涓虹壒瀹氭牳蹇?鑺墖鏀堕泦璺熻釜骞惰В鐮併€傞€氳繃姝ゅ姛鑳斤紝涓€涓皬鐨勫垎鍖哄彲浠ヨ
涓撻棬鐢ㄤ簬鏀堕泦璺熻釜鏁版嵁骞惰繘琛屽垎鏋愶紝浠庤€屼负鎬ц兘鍒嗘瀽銆佽蒋浠惰皟浼樻垨纭欢璋冭瘯鎻愪緵閲嶈淇℃伅銆?