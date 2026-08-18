
## 鐢ㄦ埛绌洪棿鍧楄澶囬┍鍔紙ublk 椹卞姩锛?

## 姒傝堪


ublk 鏄竴涓敤浜庝粠鐢ㄦ埛绌洪棿瀹炵幇鍧楄澶囬€昏緫鐨勯€氱敤妗嗘灦銆傚叾鑳屽悗鐨勫姩鏈烘槸锛氬皢铏氭嫙鍧?椹卞姩锛堝 loop銆乶bd 浠ュ強绫讳技鐨勯┍鍔級绉诲叆鐢ㄦ埛绌洪棿鍙兘浼氶潪甯告湁甯姪銆傚畠鏈夊姪浜庡疄鐜?鏂扮殑铏氭嫙鍧楄澶囷紝渚嬪 ublk-qcow2锛堜笟鐣屽凡鏈夋暟娆″湪鍐呮牳涓疄鐜?qcow2 椹卞姩鐨勫皾璇曪級銆?
鐢ㄦ埛绌洪棿鍧楄澶囦箣鎵€浠ユ湁鍚稿紩鍔涳紝鏄洜涓猴細

- 瀹冧滑鍙互鐢ㄥ绉嶇紪绋嬭瑷€缂栧啓銆?- 瀹冧滑鍙互浣跨敤鍐呮牳涓笉鍙敤鐨勫簱銆?- 瀹冧滑鍙互鐢ㄥ簲鐢ㄥ紑鍙戣€呯啛鎮夌殑宸ュ叿杩涜璋冭瘯銆?- 宕╂簝涓嶄細瀵艰嚧鏈哄櫒鍐呮牳鎭愭厡锛坘ernel panic锛夈€?- 涓庡唴鏍镐唬鐮佷腑鐨勭己闄风浉姣旓紝缂洪櫡鐨勫畨鍏ㄥ奖鍝嶅彲鑳芥洿灏忋€?- 瀹冧滑鍙互鐙珛浜庡唴鏍稿畨瑁呭拰鏇存柊銆?- 瀹冧滑鍙互鏂逛究鍦扮敤鐢ㄦ埛鎸囧畾鍙傛暟/璁剧疆鏉ユā鎷熷潡璁惧锛屼互鐢ㄤ簬娴嬭瘯/璋冭瘯鐩殑銆?
ublk 鍧楄澶囷紙`/dev/ublkb*`锛夌敱 ublk 椹卞姩娣诲姞銆傝璁惧涓婄殑浠讳綍 IO 璇锋眰閮藉皢琚浆鍙戠粰
ublk 鐢ㄦ埛绌洪棿绋嬪簭銆備负鏂逛究璧疯锛屽湪鏈枃妗ｄ腑锛宍ublk server` 鎸囬€氱敤鐨?ublk 鐢ㄦ埛绌洪棿
绋嬪簭銆俙ublksrv` [#userspace]_ 灏辨槸姝ょ被瀹炵幇涔嬩竴銆傚畠鎻愪緵 `libublksrv` [#userspace_lib]_
搴撶敤浜庢柟渚垮湴寮€鍙戠壒瀹氱殑鐢ㄦ埛鍧楄澶囷紝鍚屾椂涔熷寘鍚€氱敤鐨勭被鍨嬪潡璁惧锛屼緥濡?loop 涓?null銆?Richard W.M. Jones 鍩轰簬 `libublksrv` [#userspace_lib]_ 缂栧啓浜嗙敤鎴风┖闂?nbd 璁惧
`nbdublk` [#userspace_nbdublk]_銆?
IO 鐢辩敤鎴风┖闂村鐞嗗畬鎴愬悗锛岀粨鏋滀細琚彁浜ゅ洖椹卞姩锛屼粠鑰屽畬鎴愯姹傚懆鏈熴€傚姝や竴鏉ワ紝浠讳綍
鐗瑰畾鐨?IO 澶勭悊閫昏緫閮藉畬鍏ㄧ敱鐢ㄦ埛绌洪棿瀹屾垚锛屼緥濡?loop 鐨?IO 澶勭悊銆丯BD 鐨?IO 閫氫俊锛屾垨
qcow2 鐨?IO 鏄犲皠銆?
`/dev/ublkb*` 鐢卞熀浜?blk-mq 璇锋眰锛坮equest-based锛夌殑椹卞姩椹卞姩銆傛瘡涓姹傝鍒嗛厤涓€涓?闃熷垪鑼冨洿鍐呭敮涓€鐨?tag銆倁blk server 涔熶负姣忎釜 IO 鍒嗛厤鍞竴鐨?tag锛屽畠涓?`/dev/ublkb*`
鐨?IO 鏄?1:1 鏄犲皠鐨勩€?
IO 璇锋眰鐨勮浆鍙戜笌 IO 澶勭悊缁撴灉鐨勬彁浜ら兘閫氳繃 `io_uring` 鐩撮€氾紙passthrough锛夊懡浠ゅ畬鎴愶紱
杩欐鏄?ublk 涔熸槸涓€涓熀浜?io_uring 鐨勫潡椹卞姩鐨勫師鍥犮€傚凡缁忚瀵熷埌锛屼娇鐢?io_uring 鐩撮€?鍛戒护鍙互鑾峰緱姣斿潡 IO 鏇村ソ鐨?IOPS锛涜繖灏辨槸 ublk 鎴愪负鐢ㄦ埛绌洪棿鍧楄澶囩殑楂樻€ц兘瀹炵幇涔嬩竴
鐨勫師鍥狅細涓嶄粎 IO 璇锋眰閫氫俊閫氳繃 io_uring 瀹屾垚锛寀blk server 涓閫夌殑 IO 澶勭悊鏂瑰紡涔熸槸
鍩轰簬 io_uring 鐨勬柟妗堛€?
ublk 鎻愪緵鎺у埗鎺ュ彛鏉ヨ缃?鑾峰彇 ublk 鍧楄澶囩殑鍙傛暟銆傝鎺ュ彛鏄彲鎵╁睍鐨勶紝骞朵笖 kabi 鍏煎锛?鍩烘湰涓婁换浣?ublk 璇锋眰闃熷垪鐨勫弬鏁版垨 ublk 閫氱敤鐗规€у弬鏁伴兘鍙互閫氳繃璇ユ帴鍙ｈ缃?鑾峰彇銆傚洜姝わ紝
ublk 鏄€氱敤鐨勭敤鎴风┖闂村潡璁惧妗嗘灦銆備緥濡傦紝鍙互鏂逛究鍦颁粠鐢ㄦ埛绌洪棿鐢ㄦ寚瀹氱殑鍧楀弬鏁版潵寤虹珛
涓€涓?ublk 璁惧銆?
## 浣跨敤 ublk


ublk 闇€瑕佺敤鎴风┖闂寸殑 ublk server 鏉ュ鐞嗙湡瀹炵殑鍧楄澶囬€昏緫銆?
涓嬮潰鏄娇鐢?`ublksrv` 鎻愪緵鍩轰簬 ublk 鐨?loop 璁惧鐨勭ず渚嬨€?
```

     ublk add -t loop -f ublk-loop.img

```
```

     mkfs.xfs /dev/ublkb0
     mount /dev/ublkb0 /mnt
     # do anything. all IOs are handled by io_uring
     ...
     umount /mnt

```
```

     ublk list

```
```

     ublk del -a
     ublk del -n $ublk_dev_id

```
浣跨敤缁嗚妭鍙傝 `ublksrv` [#userspace_readme]_ 鐨?README銆?
## 璁捐


### 鎺у埗骞抽潰


ublk 椹卞姩鎻愪緵鍏ㄥ眬鏉傞」璁惧鑺傜偣锛坄/dev/ublk-control`锛夌敤浜庣鐞嗗拰鎺у埗 ublk 璁惧锛?鍊熷姪鑻ュ共鎺у埗鍛戒护锛?
- `UBLK_CMD_ADD_DEV`

  娣诲姞涓€涓?ublk 瀛楃璁惧锛坄/dev/ublkc*`锛夛紝ublk server 涓庝箣灏?IO 鍛戒护閫氫俊銆傚熀鏈殑
  璁惧淇℃伅闅忔鍛戒护涓€璧峰彂閫併€傚畠璁剧疆 `ublksrv_ctrl_dev_info` 鐨?UAPI 缁撴瀯锛屼緥濡?  `nr_hw_queues`銆乣queue_depth` 浠ュ強鏈€澶?IO 璇锋眰缂撳啿鍖哄ぇ灏忥紝杩欎簺淇℃伅涓庨┍鍔ㄥ崗鍟嗗悗
  鍥為€佺粰 server銆傚綋姝ゅ懡浠ゅ畬鎴愭椂锛屽熀鏈澶囦俊鎭彉涓轰笉鍙彉銆?
- `UBLK_CMD_SET_PARAMS` / `UBLK_CMD_GET_PARAMS`

  璁剧疆鎴栬幏鍙栬澶囩殑鍙傛暟锛屽彲浠ユ槸閫氱敤鐗规€х浉鍏崇殑锛屼篃鍙互鏄姹傞槦鍒楅檺鍒剁浉鍏崇殑锛屼絾涓嶈兘
  鏄?IO 閫昏緫鐗瑰畾鐨勶紝鍥犱负椹卞姩涓嶅鐞嗕换浣?IO 閫昏緫銆傛鍛戒护蹇呴』鍦ㄥ彂閫?`UBLK_CMD_START_DEV`
  涔嬪墠鍙戦€併€?
- `UBLK_CMD_START_DEV`

  鍦?server 鍑嗗濂界敤鎴风┖闂磋祫婧愶紙渚嬪鍒涘缓鐢ㄤ簬 handle ublk IO 鐨?I/O 澶勭悊绾跨▼ &
  io_uring锛変箣鍚庯紝鍙戦€佹鍛戒护缁欓┍鍔ㄤ互鍒嗛厤骞舵毚闇?`/dev/ublkb*`銆傞€氳繃
  `UBLK_CMD_SET_PARAMS` 璁剧疆鐨勫弬鏁颁細琚簲鐢ㄤ簬鍒涘缓璁惧銆?
- `UBLK_CMD_STOP_DEV`

  鍋滄 `/dev/ublkb*` 涓婄殑 IO 骞剁Щ闄よ澶囥€傚綋姝ゅ懡浠よ繑鍥炴椂锛寀blk server 灏嗛噴鏀捐祫婧?  锛堜緥濡傞攢姣?I/O 澶勭悊绾跨▼ & io_uring锛夈€?
- `UBLK_CMD_DEL_DEV`

  绉婚櫎 `/dev/ublkc*`銆傚綋姝ゅ懡浠よ繑鍥炴椂锛屽凡鍒嗛厤鐨?ublk 璁惧鍙峰彲琚鐢ㄣ€?
- `UBLK_CMD_GET_QUEUE_AFFINITY`

  褰撴坊鍔?`/dev/ublkc` 鏃讹紝椹卞姩鍒涘缓鍧楀眰 tagset锛屼簬鏄瘡涓槦鍒楃殑浜插拰鎬э紙affinity锛?  淇℃伅鍙敤銆俿erver 鍙戦€?`UBLK_CMD_GET_QUEUE_AFFINITY` 鏉ユ绱㈤槦鍒椾翰鍜屾€т俊鎭€傚畠鍙互
  楂樻晥鍦板缓绔嬫瘡闃熷垪涓婁笅鏂囷紝渚嬪灏嗕翰鍜?CPU 涓?IO pthread 缁戝畾锛屽苟灏濊瘯鍦?IO 绾跨▼涓婁笅鏂囦腑
  鍒嗛厤缂撳啿鍖恒€?
- `UBLK_CMD_GET_DEV_INFO`

  鐢ㄤ簬閫氳繃 `ublksrv_ctrl_dev_info` 妫€绱㈣澶囦俊鎭€傚湪鐢ㄦ埛绌洪棿淇濆瓨 IO 鐩爣鐗瑰畾淇℃伅鏄?  server 鐨勮亴璐ｃ€?
- `UBLK_CMD_GET_DEV_INFO2`
  涓?`UBLK_CMD_GET_DEV_INFO` 鐩殑鐩稿悓锛屼絾 ublk server 蹇呴』鎻愪緵 `/dev/ublkc*` 瀛楃
  璁惧鐨勮矾寰勶紝渚涘唴鏍告墽琛屾潈闄愭鏌ワ紝姝ゅ懡浠ゆ槸涓烘敮鎸侀潪鐗规潈 ublk 璁惧鑰屾坊鍔犵殑锛屽苟涓?  `UBLK_F_UNPRIVILEGED_DEV` 涓€璧峰紩鍏ャ€傚彧鏈夋嫢鏈夋墍璇锋眰璁惧鐨勭敤鎴锋墠鑳芥绱㈣澶囦俊鎭€?
  濡備綍澶勭悊鐢ㄦ埛绌洪棿/鍐呮牳鍏煎鎬э細

  1) 濡傛灉鍐呮牳鑳藉澶勭悊 `UBLK_F_UNPRIVILEGED_DEV`

    濡傛灉 ublk server 鏀寔 `UBLK_F_UNPRIVILEGED_DEV`锛?
    ublk server 搴旇鍙戦€?`UBLK_CMD_GET_DEV_INFO2`锛屽洜涓洪潪鐗规潈搴旂敤闅忔椂鍙兘闇€瑕佹煡璇?    褰撳墠鐢ㄦ埛鎵€鎷ユ湁鐨勮澶囷紱褰撳簲鐢ㄦ棤浠庣煡鏅?`UBLK_F_UNPRIVILEGED_DEV` 鏄惁宸茶缃紙鍥犱负
    鑳藉姏淇℃伅鏄棤鐘舵€佺殑锛夋椂锛屽簲鐢ㄥ簲濮嬬粓閫氳繃 `UBLK_CMD_GET_DEV_INFO2` 鏉ユ绱㈠畠銆?
    濡傛灉 ublk server 涓嶆敮鎸?`UBLK_F_UNPRIVILEGED_DEV`锛?
    `UBLK_CMD_GET_DEV_INFO` 濮嬬粓琚彂閫佺粰鍐呮牳锛岃€?`UBLK_F_UNPRIVILEGED_DEV` 鐗规€у鐢ㄦ埛
    涓嶅彲鐢ㄣ€?
  2) 濡傛灉鍐呮牳涓嶈兘澶勭悊 `UBLK_F_UNPRIVILEGED_DEV`

    濡傛灉 ublk server 鏀寔 `UBLK_F_UNPRIVILEGED_DEV`锛?
    鍏堝皾璇?`UBLK_CMD_GET_DEV_INFO2`锛屽皢浼氬け璐ワ紝鐒跺悗闇€瑕侀噸璇?`UBLK_CMD_GET_DEV_INFO`锛?    鍥犱负 `UBLK_F_UNPRIVILEGED_DEV` 鏃犳硶琚缃€?
    濡傛灉 ublk server 涓嶆敮鎸?`UBLK_F_UNPRIVILEGED_DEV`锛?
    `UBLK_CMD_GET_DEV_INFO` 濮嬬粓琚彂閫佺粰鍐呮牳锛岃€?`UBLK_F_UNPRIVILEGED_DEV` 鐗规€у鐢ㄦ埛
    涓嶅彲鐢ㄣ€?
- `UBLK_CMD_START_USER_RECOVERY`

  姝ゅ懡浠ゅ湪 `UBLK_F_USER_RECOVERY` 鐗规€у惎鐢ㄦ椂鏈夋晥銆傛鍛戒护鍦ㄦ棫杩涚▼宸查€€鍑恒€乽blk 璁惧宸?  闈欐锛坬uiesced锛変笖 `/dev/ublkc*` 宸查噴鏀惧悗琚帴鍙椼€傜敤鎴峰簲鍦ㄥ惎鍔ㄩ噸鏂版墦寮€ `/dev/ublkc*`
  鐨勬柊杩涚▼涔嬪墠鍙戦€佹鍛戒护銆傚綋姝ゅ懡浠よ繑鍥炴椂锛寀blk 璁惧宸蹭负鏂拌繘绋嬪噯澶囧氨缁€?
- `UBLK_CMD_END_USER_RECOVERY`

  姝ゅ懡浠ゅ湪 `UBLK_F_USER_RECOVERY` 鐗规€у惎鐢ㄦ椂鏈夋晥銆傛鍛戒护鍦?ublk 璁惧宸查潤姝€佷笖鏂拌繘绋?  宸叉墦寮€ `/dev/ublkc*` 骞朵娇鎵€鏈?ublk 闃熷垪鍑嗗灏辩华鍚庤鎺ュ彈銆傚綋姝ゅ懡浠よ繑鍥炴椂锛寀blk 璁惧
  鍙栨秷闈欐锛屾柊鐨?I/O 璇锋眰琚紶閫掔粰鏂拌繘绋嬨€?
- 鐢ㄦ埛鎭㈠锛坲ser recovery锛夌壒鎬ф弿杩?
  涓烘敮鎸佺敤鎴锋仮澶嶆柊澧炰簡涓変釜鐗规€э細`UBLK_F_USER_RECOVERY`銆乣UBLK_F_USER_RECOVERY_REISSUE`
  鍜?`UBLK_F_USER_RECOVERY_FAIL_IO`銆備负浜嗗湪 ublk server 閫€鍑哄悗鑳藉鎭㈠ ublk 璁惧锛寀blk
  server 搴斿湪鍒涘缓璁惧鏃舵寚瀹?`UBLK_F_USER_RECOVERY` 鏍囧織銆倁blk server 杩樺彲棰濆鎸囧畾鑷冲
  涓€涓?`UBLK_F_USER_RECOVERY_REISSUE` 涓?`UBLK_F_USER_RECOVERY_FAIL_IO`锛屼互淇敼鍦?ublk
  server 姝ｅ湪姝讳骸/宸叉浜℃椂锛堣繖琚О涓洪┍鍔ㄤ唬鐮佷腑鐨?`nosrv` 鎯呭舰锛夊浣曞鐞?I/O銆?
  浠呰缃?`UBLK_F_USER_RECOVERY` 鏃讹紝鍦?ublk server 閫€鍑哄悗锛寀blk 鍦ㄦ暣涓仮澶嶉樁娈甸兘涓嶄細
  鍒犻櫎 `/dev/ublkb*`锛屽苟涓?ublk 璁惧 ID 浼氳淇濈暀銆傜敱 ublk server 鑷璐熻矗鏍规嵁鑷韩鐭ヨ瘑
  鎭㈠璁惧涓婁笅鏂囥€傚皻鏈笅鍙戝埌鐢ㄦ埛绌洪棿鐨勮姹備細琚噸鏂板叆闃熴€傚凡涓嬪彂鍒扮敤鎴风┖闂寸殑璇锋眰浼氳
  涓锛坅bort锛夈€?
  棰濆璁剧疆 `UBLK_F_USER_RECOVERY_REISSUE` 鏃讹紝涓?`UBLK_F_USER_RECOVERY` 鐩稿弽锛屽湪 ublk
  server 閫€鍑哄悗锛屽凡涓嬪彂鍒扮敤鎴风┖闂寸殑璇锋眰浼氳閲嶆柊鍏ラ槦锛屽苟浼氬湪澶勭悊瀹?  `UBLK_CMD_END_USER_RECOVERY` 鍚庤閲嶆柊涓嬪彂缁欐柊杩涚▼銆俙UBLK_F_USER_RECOVERY_REISSUE` 鏄?  涓洪偅浜涘彲瀹瑰繊鍙岄噸鍐欏叆鐨勫悗绔璁＄殑锛屽洜涓洪┍鍔ㄥ彲鑳戒袱娆′笅鍙戝悓涓€涓?I/O 璇锋眰銆傚畠鍙兘瀵?  鍙鏂囦欢绯荤粺鎴?VM 鍚庣鏈夌敤銆?
  棰濆璁剧疆 `UBLK_F_USER_RECOVERY_FAIL_IO` 鏃讹紝鍦?ublk server 閫€鍑哄悗锛屽凡涓嬪彂鍒扮敤鎴风┖闂寸殑
  璇锋眰浼氬け璐ワ紝浠讳綍鍚庣画涓嬪彂鐨勮姹備篃鍚屾牱澶辫触銆傛寔缁璁剧疆浜嗚鏍囧織鐨勮澶囧彂璧?I/O 鐨勫簲鐢?  灏嗙湅鍒颁竴涓?I/O 閿欒锛岀洿鍒版柊鐨?ublk server 鎭㈠璇ヨ澶囥€?
闈炵壒鏉?ublk 璁惧閫氳繃浼犻€?`UBLK_F_UNPRIVILEGED_DEV` 鏉ユ敮鎸併€備竴鏃﹁缃簡璇ユ爣蹇楋紝鎵€鏈夋帶鍒?鍛戒护閮藉彲浠ョ敱闈炵壒鏉冪敤鎴峰彂閫併€傞櫎浜?`UBLK_CMD_ADD_DEV` 鍛戒护澶栵紝ublk 椹卞姩浼氬鎵€鏈夊叾浠?鎺у埗鍛戒护鎵ц閽堝鎸囧畾瀛楃璁惧锛坄/dev/ublkc*`锛夌殑鏉冮檺妫€鏌ワ紱涓烘锛岃繖浜涘懡浠ょ殑杞借嵎涓繀椤?鐢?ublk server 鎻愪緵瀛楃璁惧鐨勮矾寰勩€傞€氳繃杩欑鏂瑰紡锛寀blk 璁惧鍙樺緱瀹瑰櫒鎰熺煡锛坈ontainer-aware锛夛紝
鍦ㄤ竴涓鍣ㄤ腑鍒涘缓鐨勮澶囧彧鑳藉湪璇ュ鍣ㄥ唴閮ㄨ鎺у埗/璁块棶銆?
### 鏁版嵁骞抽潰


ublk server 搴斿垱寤轰笓鐢ㄧ嚎绋嬫潵澶勭悊 I/O銆傛瘡涓嚎绋嬪簲鏈夊叾鑷韩鐨?io_uring锛岄€氳繃瀹冩潵鑾风煡
鏂扮殑 I/O锛屼篃閫氳繃瀹冩潵瀹屾垚 I/O銆傝繖浜涗笓鐢ㄧ嚎绋嬪簲涓撴敞浜?IO 澶勭悊锛屼笉搴斿鐞嗕换浣曟帶鍒朵笌绠＄悊
浠诲姟銆?
ublk 鐨?IO 鐢变竴涓敮涓€鐨?tag 鍒嗛厤锛屽畠涓?`/dev/ublkb*` 鐨?IO 璇锋眰鏄?1:1 鏄犲皠銆?
瀹氫箟 `ublksrv_io_desc` 鐨?UAPI 缁撴瀯鐢ㄤ簬鎻忚堪鏉ヨ嚜椹卞姩鐨勬瘡涓?IO銆傚湪 `/dev/ublkc*` 涓?鎻愪緵浜嗕竴涓浐瀹氱殑 mmap 鍖哄煙锛堟暟缁勶級鐢ㄤ簬鍚?server 瀵煎嚭 IO 淇℃伅锛屼緥濡?IO 鍋忕Щ銆侀暱搴︺€?OP/鏍囧織浠ュ強缂撳啿鍖哄湴鍧€銆傛瘡涓?`ublksrv_io_desc` 瀹炰緥鍙€氳繃闃熷垪 id 鍜?IO tag 鐩存帴绱㈠紩銆?
浠ヤ笅 IO 鍛戒护閫氳繃 io_uring 鐩撮€氬懡浠ら€氫俊锛屾瘡涓懡浠や粎鐢ㄤ簬杞彂 IO 浠ュ強鎻愪氦鍛戒护鏁版嵁涓?鎸囧畾 IO tag 鐨勭粨鏋滐細

#### 浼犵粺鎸?I/O 鍛戒护


- `UBLK_U_IO_FETCH_REQ`

  浠?server 鐨?I/O pthread 鍙戦€侊紝鐢ㄤ簬鑾峰彇鍙戝線 `/dev/ublkb*` 鐨勬湭鏉ヤ紶鍏?I/O 璇锋眰銆傝
  鍛戒护浠呯敱 server 鐨?IO pthread 鍙戦€佷竴娆★紝浠ヤ究 ublk 椹卞姩寤虹珛 IO 杞彂鐜銆?
  涓€鏃︽煇绾跨▼閽堝缁欏畾鐨?(qid,tag) 瀵瑰彂鍑烘鍛戒护锛岃绾跨▼灏辨敞鍐屼负璇?I/O 鐨勫畧鎶よ繘绋?  锛坉aemon锛夈€備粖鍚庯紝鍙湁璇?I/O 鐨勫畧鎶よ繘绋嬫墠琚厑璁搁拡瀵硅 I/O 鍙戝嚭鍛戒护銆傚鏋滀换浣曞叾浠?  绾跨▼璇曞浘閽堝涓€涓叾骞堕潪瀹堟姢杩涚▼鐨?(qid,tag) 瀵瑰彂鍑哄懡浠わ紝璇ュ懡浠ゅ皢澶辫触銆傚畧鎶よ繘绋嬪彧鑳?  閫氳繃鎭㈠锛坮ecovery锛夋潵閲嶇疆銆?
  姣忎釜 (qid,tag) 瀵归兘鑳芥嫢鏈夊悇鑷嫭绔嬬殑瀹堟姢杩涚▼浠诲姟鐨勮兘鍔涳紝鐢?`UBLK_F_PER_IO_DAEMON`
  鐗规€ф寚绀恒€傚鏋滈┍鍔ㄤ笉鏀寔璇ョ壒鎬э紝鍒欏畧鎶よ繘绋嬪繀椤绘槸鎸夐槦鍒楃殑鈥斺€斿嵆涓庡崟涓?qid 鍏宠仈鐨勬墍鏈?  I/O 蹇呴』鐢卞悓涓€浠诲姟澶勭悊銆?
- `UBLK_U_IO_COMMIT_AND_FETCH_REQ`

  褰撴煇涓?IO 璇锋眰鍙戝線 `/dev/ublkb*` 鏃讹紝椹卞姩灏嗚 IO 鐨?`ublksrv_io_desc` 瀛樺叆鎸囧畾鐨?  鏄犲皠鍖哄煙锛涢殢鍚庯紝姝?IO tag 鍏堝墠鏀跺埌鐨?IO 鍛戒护锛堟棤璁烘槸 `UBLK_IO_FETCH_REQ` 杩樻槸
  `UBLK_IO_COMMIT_AND_FETCH_REQ`锛夊畬鎴愶紝浜庢槸 server 閫氳繃 io_uring 鑾峰緱 IO 閫氱煡銆?
  server 澶勭悊瀹?IO 鍚庯紝鍏跺鐞嗙粨鏋滄槸閫氳繃灏?`UBLK_IO_COMMIT_AND_FETCH_REQ` 鍙戝洖缁欓┍鍔?  鏉ユ彁浜ゅ洖鍘荤殑銆備竴鏃?ublkdrv 鏀跺埌姝ゅ懡浠わ紝瀹冭В鏋愮粨鏋滃苟瀹屾垚瀵?`/dev/ublkb*` 鐨勮姹傘€?  鍚屾椂寤虹珛鐜浠ョ敤鐩稿悓鐨?IO tag 鑾峰彇鏈潵鐨勮姹傘€備篃灏辨槸璇达紝`UBLK_IO_COMMIT_AND_FETCH_REQ`
  琚鐢ㄤ簬鑾峰彇璇锋眰涓庢彁浜ゅ洖 IO 缁撴灉涓や欢浜嬨€?
- `UBLK_U_IO_NEED_GET_DATA`

  鍦ㄥ惎鐢?`UBLK_F_NEED_GET_DATA` 鏃讹紝WRITE 璇锋眰灏嗛鍏堝湪涓嶆嫹璐濇暟鎹殑鎯呭喌涓嬩笅鍙戠粰 ublk
  server銆傜劧鍚庯紝ublk server 鐨?IO 鍚庣鏀跺埌璇ヨ姹傦紝瀹冨彲浠ュ垎閰嶆暟鎹紦鍐插尯骞跺皢鍏跺湴鍧€宓屽叆
  杩欎釜鏂扮殑 io 鍛戒护涓€傚唴鏍搁┍鍔ㄦ敹鍒拌鍛戒护鍚庯紝灏嗘墽琛屼粠璇锋眰椤靛埌姝ゅ悗绔紦鍐插尯鐨勬嫹璐濄€傛渶鍚庯紝
  鍚庣鍐嶆鏀跺埌甯﹀緟鍐欏叆鏁版嵁鐨勮姹傦紝瀹冨氨鑳界湡姝ｅ鐞嗚璇锋眰銆?
  `UBLK_IO_NEED_GET_DATA` 澧炲姞浜嗕竴瓒熼澶栫殑寰€杩斿拰涓€涓?io_uring_enter() 绯荤粺璋冪敤銆備换浣?  璁や负杩欎細闄嶄綆鎬ц兘鐨勭敤鎴烽兘涓嶅簲鍚敤 UBLK_F_NEED_GET_DATA銆傞粯璁ゆ儏鍐典笅锛寀blk server 涓烘瘡涓?  IO 棰勫垎閰?IO 缂撳啿鍖恒€備换浣曟柊椤圭洰閮藉簲灏濊瘯浣跨敤姝ょ紦鍐插尯鏉ヤ笌 ublk 椹卞姩閫氫俊銆備笉杩囷紝鐜版湁椤圭洰
  鍙兘浼氳鐮村潖锛屾垨鏃犳硶浣跨敤鏂扮殑缂撳啿鍖烘帴鍙ｏ紱杩欏氨鏄负浠€涔堟坊鍔犳鍛戒护鏄负浜嗗悜鍚庡吋瀹癸紝浣跨幇鏈?  椤圭洰浠嶈兘浣跨敤鐜版湁缂撳啿鍖恒€?
- ublk server IO 缂撳啿鍖轰笌 ublk 鍧?IO 璇锋眰涔嬮棿鐨勬暟鎹嫹璐?
  鍦ㄩ€氱煡 server 鍗冲皢鍒版潵鐨?IO 涔嬪墠锛岄┍鍔ㄩ渶瑕佸厛灏嗗潡 IO 璇锋眰椤垫嫹璐濆埌 server 缂撳啿鍖猴紙椤碉級
  涓紙閽堝 WRITE锛夛紝浠ヤ究 server 鑳藉澶勭悊 WRITE 璇锋眰銆?
  褰?server 澶勭悊 READ 璇锋眰骞跺彂閫?`UBLK_IO_COMMIT_AND_FETCH_REQ` 鏃讹紝ublkdrv 闇€瑕佸皢 server
  缂撳啿鍖猴紙椤碉級涓鍙栫殑鍐呭鎷疯礉鍒?IO 璇锋眰椤典腑銆?
#### 鎵归噺 I/O 鍛戒护锛圲BLK_F_BATCH_IO锛?

`UBLK_F_BATCH_IO` 鐗规€ф彁渚涗簡涓€绉嶆浛浠ｇ殑楂樻€ц兘 I/O 澶勭悊妯″瀷锛屽畠鐢ㄦ寜闃熷垪鐨勬壒閲忓懡浠ゆ浛鎹?浼犵粺鐨勬寜 I/O 鍛戒护銆傝繖鏄捐憲鍑忓皯浜嗛€氫俊寮€閿€锛屽苟鑳藉湪澶氫釜 server 浠诲姟闂村疄鐜版洿濂界殑璐熻浇鍧囪　銆?
涓庝紶缁熸ā寮忕殑涓昏鍖哄埆锛?
- **鎸夐槦鍒?vs 鎸?I/O**锛氬懡浠や綔鐢ㄤ簬闃熷垪鑰岄潪鍗曚釜 I/O
- **鎵归噺澶勭悊**锛氬涓?I/O 鍦ㄥ崟娆℃搷浣滀腑琚鐞?- **澶氶噸瑙﹀彂锛坢ultishot锛夊懡浠?*锛氫娇鐢?io_uring multishot 浠ュ噺灏戞彁浜ゅ紑閿€
- **鐏垫椿鐨勪换鍔″垎閰?*锛氫换浣曚换鍔￠兘鍙鐞嗕换浣?I/O锛堟棤鎸?I/O 鐨勫畧鎶よ繘绋嬶級
- **鏇村ソ鐨勮礋杞藉潎琛?*锛氫换鍔″彲鍔ㄦ€佽皟鏁村叾宸ヤ綔璐熻浇

鎵归噺 I/O 鍛戒护锛?
- `UBLK_U_IO_PREP_IO_CMDS`

  鎵归噺鍑嗗澶氫釜 I/O 鍛戒护銆俿erver 鎻愪緵涓€涓寘鍚涓皢琚竴璧峰鐞嗙殑 I/O 鎻忚堪绗︾殑缂撳啿鍖恒€?  杩欏噺灏戜簡鎵€闇€鐨勫崟涓懡浠ゆ彁浜ゆ暟閲忋€?
- `UBLK_U_IO_COMMIT_IO_CMDS`

  鎵归噺鎻愪氦澶氫釜 I/O 鎿嶄綔鐨勭粨鏋滐紝骞跺噯澶囧ソ I/O 鎻忚堪绗︿互鎺ュ彈鏂扮殑璇锋眰銆俿erver 鎻愪緵涓€涓寘鍚?  澶氫釜宸插畬鎴?I/O 缁撴灉鐨勭紦鍐插尯锛屼粠鑰屽厑璁搁珮鏁堝湴鎵归噺瀹屾垚璇锋眰銆?
- `UBLK_U_IO_FETCH_IO_CMDS`

  鐢ㄤ簬鎵归噺鑾峰彇 I/O 鍛戒护鐨?*澶氶噸瑙﹀彂鍛戒护**銆傝繖鏄疄鐜伴珮鎬ц兘鎵瑰鐞嗙殑鍏抽敭鍛戒护锛?
  - 浣跨敤 io_uring multishot 鑳藉姏浠ュ噺灏戞彁浜ゅ紑閿€
  - 鍗曚釜鍛戒护鍙湪涓€娈垫椂闂村唴鑾峰彇澶氫釜 I/O 璇锋眰
  - 缂撳啿鍖哄ぇ灏忓喅瀹氭瘡娆℃搷浣滅殑鏈€澶ф壒澶у皬
  - 鍙彁浜ゅ涓幏鍙栧懡浠や互瀹炵幇璐熻浇鍧囪　
  - 姣忎釜闃熷垪浠绘剰鏃跺埢鍙湁涓€涓幏鍙栧懡浠ゅ浜庢椿鍔ㄧ姸鎬?  - 鏀寔璺ㄥ涓?server 浠诲姟鐨勫姩鎬佽礋杞藉潎琛?
  瀹冩槸涓€涓吀鍨嬬殑甯︽彁渚涚紦鍐插尯鐨?multishot io_uring 璇锋眰锛屽湪瑙﹀彂浠讳綍澶辫触涔嬪墠涓嶄細琚畬鎴愩€?
  姣忎釜浠诲姟鍙互鎻愪氦鍏锋湁涓嶅悓缂撳啿鍖哄ぇ灏忕殑 `UBLK_U_IO_FETCH_IO_CMDS` 鏉ユ帶鍒跺叾澶勭悊鐨勫伐浣滈噺銆?  杩欎娇寰楀绾跨▼ server 涓鏉傜殑璐熻浇鍧囪　绛栫暐鎴愪负鍙兘銆?
杩佺Щ锛氫娇鐢ㄤ紶缁熷懡浠わ紙`UBLK_U_IO_FETCH_REQ`銆乣UBLK_U_IO_COMMIT_AND_FETCH_REQ`锛夌殑搴旂敤
鏃犳硶鍚屾椂浣跨敤鎵归噺妯″紡銆?
### 闆舵嫹璐?

ublk 闆舵嫹璐濅緷璧栦簬 io_uring 鐨勫浐瀹氬唴鏍哥紦鍐插尯锛屽畠鎻愪緵涓や釜 API锛歚io_buffer_register_bvec()`
鍜?`io_buffer_unregister_bvec`銆?
ublk 娣诲姞浜?`UBLK_IO_REGISTER_IO_BUF` IO 鍛戒护鏉ヨ皟鐢?`io_buffer_register_bvec()`锛屼互渚?ublk server 灏嗗鎴风璇锋眰缂撳啿鍖烘敞鍐岃繘 io_uring 缂撳啿鍖鸿〃锛岀劧鍚?ublk server 鍙敤宸叉敞鍐岀殑
缂撳啿鍖虹储寮曟彁浜?io_uring 鐨?IO銆俙UBLK_IO_UNREGISTER_IO_BUF` IO 鍛戒护璋冪敤
`io_buffer_unregister_bvec()` 鏉ユ敞閿€缂撳啿鍖猴紝璇ョ紦鍐插尯淇濊瘉鍦ㄨ皟鐢?`io_buffer_register_bvec()`
涓?`io_buffer_unregister_bvec()` 涔嬮棿涓€鐩村瓨娲汇€備换浣曟敮鎸佹绫诲唴鏍哥紦鍐插尯鐨?io_uring 鎿嶄綔閮戒細
鎸佹湁璇ョ紦鍐插尯鐨勪竴涓紩鐢紝鐩村埌鎿嶄綔瀹屾垚銆?
瀹炵幇闆舵嫹璐濇垨鐢ㄦ埛鎷疯礉鐨?ublk server 蹇呴』鍏锋湁 CAP_SYS_ADMIN 涓斿彈淇′换锛屽洜涓虹‘淇濆鐞?read 鍛戒护
鏃?IO 缂撳啿鍖哄凡濉厖鏁版嵁銆佸苟鍦ㄥ鐞?READ 鍛戒护鏃跺悜 ublk 椹卞姩杩斿洖姝ｇ‘缁撴灉鏄?ublk server 鐨勮矗浠伙紝
鑰屼笖璇ョ粨鏋滃繀椤讳笌濉厖鍒?IO 缂撳啿鍖虹殑瀛楄妭鏁扮浉绗︺€傚惁鍒欙紝鏈垵濮嬪寲鐨勫唴鏍?IO 缂撳啿鍖哄皢琚毚闇茬粰
瀹㈡埛绔簲鐢ㄣ€?
ublk server 闇€瑕佷娇 `struct ublk_param_dma_align` 鐨勫弬鏁颁笌鍚庣瀵归綈锛岄浂鎷疯礉鎵嶈兘姝ｅ父宸ヤ綔銆?
涓轰簡杈惧埌鏈€浣?IO 鎬ц兘锛寀blk server 搴斾娇 `struct ublk_param_segment` 鐨勬鍙傛暟涓庡悗绔榻愶紝
浠ラ伩鍏嶄笉蹇呰鐨?IO 鎷嗗垎锛岃繖閫氬父浼氭湁鎹?io_uring 鎬ц兘銆?
### 鑷姩缂撳啿鍖烘敞鍐?

`UBLK_F_AUTO_BUF_REG` 鐗规€ц嚜鍔ㄥ鐞?I/O 璇锋眰鐨勭紦鍐插尯娉ㄥ唽涓庢敞閿€锛岃繖绠€鍖栦簡缂撳啿鍖虹鐞嗘祦绋嬶紝骞?鍑忓皯浜?ublk server 瀹炵幇涓殑寮€閿€銆?
杩欐槸鐢ㄤ簬浣跨敤闆舵嫹璐濈殑鍙︿竴涓壒鎬ф爣蹇楋紝骞朵笖瀹冧笌 `UBLK_F_SUPPORT_ZERO_COPY` 鍏煎銆?
#### 鐗规€ф杩?

璇ョ壒鎬у湪灏?I/O 鍛戒护閫掍氦缁?ublk server 涔嬪墠锛岃嚜鍔ㄥ皢璇锋眰缂撳啿鍖烘敞鍐屽埌 io_uring 涓婁笅鏂囷紝骞跺湪
瀹屾垚 I/O 鍛戒护鏃舵敞閿€瀹冧滑銆傝繖娑堥櫎浜嗗閫氳繃 `UBLK_IO_REGISTER_IO_BUF` 涓?`UBLK_IO_UNREGISTER_IO_BUF`
鍛戒护杩涜鎵嬪姩缂撳啿鍖烘敞鍐?娉ㄩ攢鐨勯渶瑕侊紝浜庢槸 ublk server 涓殑 IO 澶勭悊鍙互鎽嗚劚瀵硅繖涓や釜 uring_cmd
鎿嶄綔鐨勪緷璧栥€?
濡傛灉杩欎簺 IO 涔嬮棿瀛樺湪浠讳綍渚濊禆锛屽氨涓嶈兘鍚?io_uring 骞跺彂涓嬪彂 IO銆傚洜姝よ繖绉嶆柟寮忎笉浠呯畝鍖栦簡 ublk
server 鐨勫疄鐜帮紝杩橀€氳繃绉婚櫎瀵圭紦鍐插尯娉ㄥ唽涓庢敞閿€鍛戒护鐨勪緷璧栵紝浣垮苟鍙?IO 澶勭悊鎴愪负鍙兘銆?
#### 浣跨敤瑕佹眰


1. ublk server 蹇呴』鍦ㄧ敤浜?`UBLK_IO_FETCH_REQ` 涓?`UBLK_IO_COMMIT_AND_FETCH_REQ` 鐨勫悓涓€涓?   `io_ring_ctx` 涓婂垱寤虹█鐤忕紦鍐插尯琛ㄣ€傚鏋?uring_cmd 鍦ㄤ笉鍚岀殑 `io_ring_ctx` 涓婂彂鍑猴紝鍒欓渶瑕?   鎵嬪姩娉ㄩ攢缂撳啿鍖恒€?
2. 缂撳啿鍖烘敞鍐屾暟鎹繀椤婚€氳繃 uring_cmd 鐨?`sqe->addr` 浼犻€掞紝骞朵笖浣跨敤
```

    struct ublk_auto_buf_reg {
        __u16 index;      /* Buffer index for registration */
        __u8 flags;       /* Registration flags */
        __u8 reserved0;   /* Reserved for future use */
        __u32 reserved1;  /* Reserved for future use */
    };

   ublk_auto_buf_reg_to_sqe_addr() 鐢ㄤ簬灏嗕笂闈㈢殑缁撴瀯杞崲涓?``sqe->addr``銆?
```
3. `ublk_auto_buf_reg` 涓殑鎵€鏈変繚鐣欏瓧娈靛繀椤绘竻闆躲€?
4. 鍙€夌殑鏍囧織鍙€氳繃 `ublk_auto_buf_reg.flags` 浼犻€掋€?
#### 鍥為€€琛屼负


濡傛灉鑷姩缂撳啿鍖烘敞鍐屽け璐ワ細

1. 褰撳惎鐢ㄤ簡 `UBLK_AUTO_BUF_REG_FALLBACK` 鏃讹細

   - uring_cmd 琚畬鎴?   - `UBLK_IO_F_NEED_REG_BUF` 琚缃湪 `ublksrv_io_desc.op_flags` 涓?   - ublk server 蹇呴』鎵嬪姩澶勭悊璇ュけ璐ワ紝渚嬪鎵嬪姩娉ㄥ唽缂撳啿鍖猴紝鎴栦娇鐢ㄧ敤鎴锋嫹璐濈壒鎬ф潵鑾峰彇鏁版嵁浠?     澶勭悊 ublk IO

2. 濡傛灉鏈惎鐢ㄥ洖閫€锛?
   - ublk I/O 璇锋眰闈欓粯澶辫触
   - uring_cmd 涓嶄細琚畬鎴?
#### 闄愬埗


- 鎵€鏈夋搷浣滈渶瑕佺浉鍚岀殑 `io_ring_ctx`
- 鍦ㄥ洖閫€鎯呭舰涓嬪彲鑳介渶瑕佹墜鍔ㄧ紦鍐插尯绠＄悊
- io_ring_ctx 缂撳啿鍖鸿〃鐨勬渶澶уぇ灏忎负 16K锛屽湪鐢卞崟涓?io_ring_ctx 澶勭悊杩囧 ublk 璁惧涓旀瘡涓澶?  闃熷垪娣卞害寰堝ぇ鏃讹紝鍙兘涓嶅鐢?
### 鍏变韩鍐呭瓨闆舵嫹璐濓紙UBLK_F_SHMEM_ZC锛?

`UBLK_F_SHMEM_ZC` 鐗规€ф彁渚涗簡涓€绉嶆浛浠ｇ殑闆舵嫹璐濊矾寰勶紝鍏跺伐浣滃師鐞嗘槸鍦ㄥ鎴风搴旂敤涓?ublk server
涔嬮棿鍏变韩鐗╃悊鍐呭瓨椤点€備笌涓婅堪 io_uring 鍥哄畾缂撳啿鍖烘柟妗堜笉鍚岋紝鍏变韩鍐呭瓨闆舵嫹璐濅笉闇€瑕佹瘡涓?I/O 閮?杩涜 io_uring 缂撳啿鍖烘敞鍐屸€斺€旂浉鍙嶏紝瀹冧緷璧栦簬鍐呮牳鍦?I/O 鏃跺尮閰嶇墿鐞嗛〉銆傝繖浣垮緱 ublk server 鑳藉
鐩存帴璁块棶鍏变韩缂撳啿鍖猴紝鑰岃繖鍦ㄤ娇鐢?io_uring 鍥哄畾缂撳啿鍖烘柟妗堟椂鏄笉澶彲鑳界殑銆?
#### 鍔ㄦ満


鍏变韩鍐呭瓨闆舵嫹璐濋噰鍙栦簡涓嶅悓鐨勬柟寮忥細濡傛灉瀹㈡埛绔簲鐢ㄤ笌 ublk server 閮芥槧灏勪簡鐩稿悓鐨勭墿鐞嗗唴瀛橈紝閭ｅ氨
娌℃湁浠€涔堥渶瑕佹嫹璐濈殑銆傚唴鏍镐細鑷姩妫€娴嬪叡浜〉锛屽苟鍛婅瘔 server 鏁版嵁宸插瓨鍦ㄤ簬浣曞銆?
`UBLK_F_SHMEM_ZC` 鍙瑙嗕綔閽堝浼樺寲杩囩殑瀹㈡埛绔簲鐢ㄧ殑涓€椤硅ˉ鍏呪€斺€斿綋瀹㈡埛绔効鎰忎粠鍏变韩鍐呭瓨鍒嗛厤
I/O 缂撳啿鍖烘椂锛屾暣涓暟鎹矾寰勫氨鍙樻垚浜嗛浂鎷疯礉銆?
#### 鐢ㄤ緥


褰撳鎴风搴旂敤鍙閰嶇疆涓哄鍏?I/O 缂撳啿鍖轰娇鐢ㄧ壒瀹氱殑鍏变韩鍐呭瓨鍖哄煙鏃讹紝姝ょ壒鎬у緢鏈夌敤锛?
- **鑷畾涔夊瓨鍌ㄥ鎴风**锛氫粠鍏变韩鍐呭瓨锛坢emfd銆乭ugetlbfs锛夊垎閰?I/O 缂撳啿鍖猴紝骞跺 ublk 璁惧鍙戣捣
  鐩存帴 I/O
- **鏁版嵁搴撳紩鎿?*锛氫娇鐢ㄥ甫鏈?O_DIRECT 鐨勯鍒嗛厤缂撳啿姹?
#### 宸ヤ綔鍘熺悊


1. ublk server 涓庡鎴风閮界敤 `MAP_SHARED` `mmap()` 鍚屼竴涓枃浠讹紙memfd 鎴?hugetlbfs锛夈€傝繖璁╀袱涓?   杩涚▼閮借兘璁块棶鐩稿悓鐨勭墿鐞嗛〉銆?
```

     struct ublk_shmem_buf_reg buf = { .addr = mmap_va, .len = size };
     ublk_ctrl_cmd(UBLK_U_CMD_REG_BUF, .addr = &buf);

   The kernel pins the pages and builds a PFN lookup tree.

```
3. 褰撳鎴风瀵?`/dev/ublkb*` 鍙戣捣鐩存帴 I/O锛坄O_DIRECT`锛夋椂锛屽唴鏍搁€氳繃姣旇緝 PFN 鏉ユ鏌?I/O 缂撳啿
   鍖洪〉鏄惁鍖归厤浠讳綍宸叉敞鍐岀殑椤点€?
4. 鍖归厤鏃讹紝鍐呮牳鍦?I/O 涓缃?`UBLK_IO_F_SHMEM_ZC`
```

     if (iod->op_flags & UBLK_IO_F_SHMEM_ZC) {
         /* Data is already in our shared mapping 鈥?zero copy */
         index  = ublk_shmem_zc_index(iod->addr);
         offset = ublk_shmem_zc_offset(iod->addr);
         buf = shmem_table[index].mmap_base + offset;
     }

```
5. 濡傛灉椤典笉鍖归厤锛堜緥濡傚鎴风浣跨敤浜嗛潪鍏变韩缂撳啿鍖猴級锛孖/O 浼氶潤榛樺洖閫€鍒版甯哥殑鎷疯礉璺緞銆?
鍏变韩鍐呭瓨鍙€氳繃涓ょ鏂规硶寤虹珛锛?
- **鍩轰簬濂楁帴瀛?*锛氬鎴风閫氳繃 unix 濂楁帴瀛椾笂鐨?`SCM_RIGHTS` 鍚?ublk server 鍙戦€佷竴涓?memfd銆?  server 鏄犲皠骞舵敞鍐屽畠銆?- **鍩轰簬 hugetlbfs**锛氫袱涓繘绋?`mmap(MAP_SHARED)` 鍚屼竴涓?hugetlbfs 鏂囦欢銆傛棤闇€ IPC鈥斺€斿悓涓€鏂囦欢
  缁欏嚭鐩稿悓鐨勭墿鐞嗛〉銆?
#### 浼樼偣


- **绠€鍗?*锛氭病鏈夋寜 I/O 鐨勭紦鍐插尯娉ㄥ唽鎴栨敞閿€鍛戒护銆備竴鏃﹀叡浜紦鍐插尯琚敞鍐岋紝鎵€鏈夊尮閰嶇殑 I/O 閮借嚜鍔?  鍙樹负闆舵嫹璐濄€?- **鐩存帴缂撳啿鍖鸿闂?*锛歶blk server 鍙互閫氳繃鑷韩鐨?mmap 鐩存帴璇诲啓鍏变韩缂撳啿鍖猴紝鑰屾棤闇€缁忚繃 io_uring
  鍥哄畾缂撳啿鍖烘搷浣溿€傝繖瀵?server 瀹炵幇鏇村弸濂姐€?- **蹇€?*锛歅FN 鍖归厤瀵规瘡涓?bvec 鏉ヨ鏄竴娆?maple tree 鏌ユ壘銆傜紦鍐插尯绠＄悊娌℃湁 io_uring 鍛戒护鐨?  寰€杩斿紑閿€銆?- **鍏煎**锛氫笉鍖归厤鐨?I/O 闈欓粯鍥為€€鍒版嫹璐濊矾寰勩€傝璁惧瀵逛换浣曞鎴风閮借兘姝ｅ父宸ヤ綔锛屽湪鍏变韩鍐呭瓨鍙敤鏃?  闆舵嫹璐濅綔涓轰竴绉嶄紭鍖栥€?
#### 闄愬埗


- **闇€瑕佸鎴风閰嶅悎**锛氬鎴风蹇呴』浠庡叾 I/O 缂撳啿鍖哄垎閰嶈嚜鍏变韩鍐呭瓨鍖哄煙銆傝繖闇€瑕佷竴涓嚜瀹氫箟鎴栫粡杩囬厤缃殑
  瀹㈡埛绔€斺€斾娇鐢ㄨ嚜韬紦鍐插尯鐨勬爣鍑嗗簲鐢ㄥ皢鏃犳硶鍙楃泭銆?- **浠呯洿鎺?I/O**锛氱紦鍐?I/O锛堜笉甯?`O_DIRECT`锛変細缁忚繃椤电紦瀛橈紝椤电紦瀛樹細鍒嗛厤鑷繁鐨勯〉銆傝繖浜涘唴鏍稿垎閰嶇殑
  椤垫案杩滀笉浼氬尮閰嶅凡娉ㄥ唽鐨勫叡浜紦鍐插尯銆傚彧鏈?`O_DIRECT` 鎵嶄細灏嗗鎴风鐨勭紦鍐插尯椤电洿鎺ユ斁鍏ュ潡 I/O 涓€?- **浠呰繛缁暟鎹?*锛氭瘡涓?I/O 璇锋眰鐨勬暟鎹繀椤诲湪鍗曚釜宸叉敞鍐岀紦鍐插尯鍐呴儴鏄繛缁殑銆傝法瓒婂涓笉鐩搁偦鐨勫凡娉ㄥ唽
  缂撳啿鍖虹殑鍒嗘暎/鑱氶泦 I/O 涓嶈兘浣跨敤闆舵嫹璐濊矾寰勩€?
#### 鎺у埗鍛戒护


- `UBLK_U_CMD_REG_BUF`

  娉ㄥ唽涓€涓叡浜唴瀛樼紦鍐插尯銆俙ctrl_cmd.addr` 鎸囧悜涓€涓?`struct ublk_shmem_buf_reg`锛屽叾涓寘鍚紦鍐插尯
  铏氭嫙鍦板潃鍜屽ぇ灏忋€傛垚鍔熸椂杩斿洖鎵€鍒嗛厤缂撳啿鍖虹储寮曪紙>= 0锛夈€傚唴鏍稿浐瀹氶〉骞跺缓绔?PFN 鏌ユ壘鏍戙€傞槦鍒楀喕缁撳湪
  鍐呴儴澶勭悊銆?
- `UBLK_U_CMD_UNREG_BUF`

  娉ㄩ攢鍏堝墠娉ㄥ唽鐨勭紦鍐插尯銆俙ctrl_cmd.data[^0^]` 鏄紦鍐插尯绱㈠紩銆傝В闄ゅ浐瀹氶〉骞朵粠鏌ユ壘鏍戜腑绉婚櫎 PFN 鏉＄洰銆?
## 鍙傝€冭祫鏂?