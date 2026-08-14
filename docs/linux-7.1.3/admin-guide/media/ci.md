
## 鏁板瓧鐢佃鏉′欢鎺ユ敹鎺ュ彛锛圕I锛?

   This documentation is outdated.

鏈枃妗ｆ弿杩伴珮灞?CI API 鐨勭敤娉曪紝閬靛惊 Linux DVB API銆傝繖涓嶆槸瀵圭幇鏈変綆灞?CI API 鐨勬枃妗ｃ€?

   瀵逛簬 Twinhan/Twinhan 鍏嬮殕鍗★紝dst_ca 妯″潡璐熻矗 CI 鐨勭‖浠跺鐞嗐€傚鏋滄娴嬪埌涓€涓?CI
   锛圕ommon Interface锛屽嵆瀹圭撼 CAM锛圕onditional Access Module锛屾潯浠舵帴鏀舵ā鍧楋級鐨勬帴鍙ｏ級锛?   璇ユā鍧椾細鑷姩鍔犺浇銆?
#### ca_zap


鍍?`ca_zap` 杩欐牱鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忔槸澶勭悊鍔犲瘑鐨?MPEG-TS 娴佹墍蹇呴渶鐨勩€?
`ca_zap` 鐢ㄦ埛鎬佸簲鐢ㄧ▼搴忚礋璐ｅ皢瑙ｆ壈锛坉escrambling锛夌浉鍏充俊鎭彂閫佺粰鏉′欢鎺ユ敹妯″潡锛圕AM锛夈€?
灏辩洰鍓嶈€岃█锛岃搴旂敤绋嬪簭闇€瑕佷互涓嬫潯浠舵墠鑳芥甯稿伐浣溿€?
a) 浣跨敤 szap 璋冭皭鍒颁竴涓湁鏁堥閬撱€?
  eg: $ szap -c channels.conf -r "TMC" -x

b) 涓€涓寘鍚湁鏁?PMT PID 鐨?channels.conf

  eg: TMC:11996:h:0:27500:278:512:650:321

  杩欓噷鐨?278 鏄竴涓湁鏁堢殑 PMT PID銆傚叾浣欑殑鍊间笌 szap 浣跨敤鐨勫€肩浉鍚屻€?
c) 杩愯 szap 涔嬪悗锛屼綘蹇呴』杩愯 ca_zap锛岃В鎵板櫒鎵嶈兘宸ヤ綔锛?
  eg: $ ca_zap channels.conf "TMC"

d) 甯屾湜浣犺兘鍍忎娇鐢?FTA 鍗′竴鏍锋璧忎綘璁㈤槄鐨勫枩鐖遍閬撱€?

  鐩墠 ca_zap 鍜?dst_test 閮戒粎鐢ㄤ簬婕旂ず鐩殑锛屽鏈夊繀瑕佸畠浠彲浠ュ彂灞曚负瀹屾暣鐨勫簲鐢ㄧ▼搴忋€?

#### 灞炰簬姝ょ被鍒殑鍗?

鐩墠灞炰簬姝ょ被鍒殑鍗℃湁 Twinhan 鍙婂叾鍏嬮殕鍗★紝杩欎簺鍗′互 VVMER銆乀omato銆丠ercules銆丱range 绛夊悕绉伴攢鍞€?
#### 鍙楁敮鎸佺殑 CI 妯″潡


CI 妯″潡鐨勬敮鎸佸湪寰堝ぇ绋嬪害涓婂彇鍐充簬鍗′笂鐨勫浐浠躲€傛湁浜涘崱纭疄鏀寔鍑犱箮鍏ㄩ儴鍙敤鐨?CI 妯″潡銆傝璁╄繖浜涘崱鏀寔棰濆鐨?CI 妯″潡锛岀洰鍓嶆病鏈変粈涔堝お澶氬彲鍋氱殑銆?
鐩墠璇ラ┍鍔ㄥ凡娴嬭瘯杩囩殑妯″潡鏈夛細

(1) SCM 鐨?Irdeto 1 鍜?2
(2) SCM 鐨?Viaccess
(3) Dragoncam
