## NFS LOCALIO


## 姒傝堪


LOCALIO 杈呭姪 RPC 鍗忚鍏佽 Linux NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄥ彲闈犲湴鎻℃墜锛屼互纭畾瀹冧滑鏄惁浣嶄簬鍚屼竴鍙颁富鏈轰笂銆傚湪 menuconfig 涓€夋嫨 "NFS client and server support for LOCALIO auxiliary protocol" 浠ュ湪鍐呮牳閰嶇疆涓惎鐢?CONFIG_NFS_LOCALIO锛堝悓鏃跺繀椤诲惎鐢?CONFIG_NFS_FS 涓?CONFIG_NFSD锛夈€?

涓€鏃?NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄦ彙鎵嬩负 "local"锛堟湰鍦帮級锛屽鎴风灏嗗湪璇汇€佸啓鍜屾彁浜ゆ搷浣滀腑缁曡繃缃戠粶 RPC 鍗忚銆傜敱浜庣粫杩囦簡 XDR 涓?RPC锛岃繖浜涙搷浣滀細杩愯寰楁洿蹇€?

LOCALIO 杈呭姪鍗忚鐨勫疄鐜颁娇鐢ㄤ笌 NFS 娴侀噺鐩稿悓鐨勮繛鎺ワ紝閬靛惊鐢?NFS ACL 鍗忚鎵╁睍鎵€纭珛鐨勬ā寮忋€?

闇€瑕?LOCALIO 杈呭姪鍗忚锛屾墠鑳界ǔ鍋ュ湴鍙戠幇涓庡叾鏈嶅姟鍣ㄤ綅浜庢湰鍦扮殑瀹㈡埛绔€傚湪浣跨敤鏈?LOCALIO 鍗忚涔嬪墠鐨勬煇涓鏈夊疄鐜颁腑锛屾浘灏濊瘯鍩轰簬 sockaddr 缃戠粶鍦板潃瀵规墍鏈夋湰鍦扮綉缁滄帴鍙ｈ繘琛岃剢寮辩殑鍖归厤銆備絾涓?LOCALIO 鍗忚涓嶅悓锛屽熀浜?sockaddr 鐨勫尮閰嶆棤娉曞鐞?iptables 鎴栧鍣ㄧ殑浣跨敤銆?

鏈湴瀹㈡埛绔笌鏈嶅姟鍣ㄤ箣闂寸殑绋冲仴鎻℃墜浠呬粎鏄釜寮€濮嬶紝杩欑灞€閮ㄦ€ф墍鏀拺鐨勭粓鏋佺敤渚嬫槸锛氬鎴风鑳藉鐩存帴鎵撳紑鏂囦欢骞跺悜鏈嶅姟鍣ㄥ彂璧疯銆佸啓鍜屾彁浜わ紝鑰屾棤闇€缁忚繃缃戠粶銆傝姹傛槸灏藉彲鑳介珮鏁堝湴鎵ц杩欎簺鐜洖 NFS 鎿嶄綔锛岃繖瀵逛簬瀹瑰櫒鐢ㄤ緥锛堜緥濡?kubernetes锛夊挨涓烘湁鐢紝鍥犱负鍙互鍦ㄦ湇鍔″櫒鏈湴杩愯 IO 浠诲姟銆?

LOCALIO 閫氳繃涓鸿銆佸啓鍜屾彁浜ょ粫杩?XDR 涓?RPC 鑰屽甫鏉ョ殑鎬ц兘浼樺娍鍙兘鏋佷负鏄捐憲锛屼緥濡傦細

fio锛屾椂闀?20 绉掞紝directio锛岄槦鍒楁繁搴?8锛?6 涓?libaio 绾跨▼锛?
  - With LOCALIO:
    4K read:    IOPS=979k,  BW=3825MiB/s (4011MB/s)(74.7GiB/20002msec)
    4K write:   IOPS=165k,  BW=646MiB/s  (678MB/s)(12.6GiB/20002msec)
    128K read:  IOPS=402k,  BW=49.1GiB/s (52.7GB/s)(982GiB/20002msec)
    128K write: IOPS=11.5k, BW=1433MiB/s (1503MB/s)(28.0GiB/20004msec)

  - Without LOCALIO:
    4K read:    IOPS=79.2k, BW=309MiB/s  (324MB/s)(6188MiB/20003msec)
    4K write:   IOPS=59.8k, BW=234MiB/s  (245MB/s)(4671MiB/20002msec)
    128K read:  IOPS=33.9k, BW=4234MiB/s (4440MB/s)(82.7GiB/20004msec)
    128K write: IOPS=11.5k, BW=1434MiB/s (1504MB/s)(28.0GiB/20011msec)

fio锛屾椂闀?20 绉掞紝directio锛岄槦鍒楁繁搴?8锛? 涓?libaio 绾跨▼锛?
  - With LOCALIO:
    4K read:    IOPS=230k,  BW=898MiB/s  (941MB/s)(17.5GiB/20001msec)
    4K write:   IOPS=22.6k, BW=88.3MiB/s (92.6MB/s)(1766MiB/20001msec)
    128K read:  IOPS=38.8k, BW=4855MiB/s (5091MB/s)(94.8GiB/20001msec)
    128K write: IOPS=11.4k, BW=1428MiB/s (1497MB/s)(27.9GiB/20001msec)

  - Without LOCALIO:
    4K read:    IOPS=77.1k, BW=301MiB/s  (316MB/s)(6022MiB/20001msec)
    4K write:   IOPS=32.8k, BW=128MiB/s  (135MB/s)(2566MiB/20001msec)
    128K read:  IOPS=24.4k, BW=3050MiB/s (3198MB/s)(59.6GiB/20001msec)
    128K write: IOPS=11.4k, BW=1430MiB/s (1500MB/s)(27.9GiB/20001msec)

## 甯歌闂瑙ｇ瓟


1. LOCALIO 鐨勭敤渚嬫湁鍝簺锛?

   a. NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄤ綅浜庡悓涓€涓绘満鐨勫伐浣滆礋杞藉彲鑾峰緱鏇撮珮鐨?IO 鎬ц兘銆傚挨鍏舵槸锛岃繍琛屽鍣ㄥ寲宸ヤ綔璐熻浇鏃讹紝浣滀笟甯稿父鍙戠幇鑷繁杩愯鍦ㄤ笌鐢ㄤ簬瀛樺偍鐨?knfsd 鏈嶅姟鍣ㄧ浉鍚岀殑涓绘満涓娿€?

2. LOCALIO 鏈夊摢浜涜姹傦紵

   a. 灏藉彲鑳界粫杩囩綉缁?RPC 鍗忚鐨勪娇鐢ㄣ€傝繖鍖呮嫭鍦?open銆佽銆佸啓鍜屾彁浜ゆ搷浣滀腑缁曡繃 XDR 涓?RPC銆?
   b. 鍏佽瀹㈡埛绔笌鏈嶅姟鍣ㄨ嚜涓诲彂鐜板郊姝ゆ槸鍚﹁繍琛屼簬鏈湴锛岃€屾棤闇€瀵规湰鍦扮綉缁滄嫇鎵戝仛浠讳綍鍋囪銆?
   c. 閫氳繃鍏煎鐩稿叧鍛藉悕绌洪棿锛堜緥濡?network銆乽ser銆乵ount锛夋潵鏀寔瀹瑰櫒鐨勪娇鐢ㄣ€?
   d. 鏀寔鎵€鏈夌増鏈殑 NFS銆侼FSv3 灏や负閲嶈锛屽洜涓哄畠鍦ㄤ紒涓氫腑骞挎硾浣跨敤锛屼笖 pNFS flexfiles 鍦ㄦ暟鎹矾寰勪笂浣跨敤浜嗗畠銆?

3. 涓轰綍 LOCALIO 鍦ㄥ垽鏂?NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄦ槸鍚︿綅浜庡悓涓€涓绘満鏃讹紝涓嶇洿鎺ユ瘮杈?IP 鍦板潃鎴栦富鏈哄悕锛?

   鐢变簬涓昏鐢ㄤ緥涔嬩竴鏄鍣ㄥ寲宸ヤ綔璐熻浇锛屾垜浠笉鑳藉亣璁惧鎴风涓庢湇鍔″櫒涔嬮棿浼氬叡浜?IP 鍦板潃銆傝繖灏变骇鐢熶簡瀵规彙鎵嬪崗璁殑闇€姹傦細璇ュ崗璁渶瑕佽蛋涓?NFS 娴侀噺鐩稿悓鐨勮繛鎺ワ紝浠ョ‘璁ゅ鎴风涓庢湇鍔″櫒纭疄杩愯鍦ㄥ悓涓€涓绘満涓娿€傛彙鎵嬩娇鐢ㄤ竴涓€氳繃绾胯矾鍙戦€佺殑瀵嗛挜锛坰ecret锛夛紝濡傛灉鍙屾柟纭疄浣嶄簬鍚屼竴浣嶇疆锛屽垯鍙€氳繃涓庡叡浜唴鏍稿唴瀛樹腑瀛樺偍鐨勫€艰繘琛屾瘮杈冩潵鐢卞弻鏂归獙璇併€?

4. LOCALIO 鏄惁浼氭敼鍠?pNFS flexfiles锛?

   鏄殑锛孡OCALIO 瀵?pNFS flexfiles 褰㈡垚琛ュ厖锛屼娇鍏惰兘鍒╃敤 NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄧ殑灞€閮ㄦ€с€傝瀹㈡埛绔?IO 鍦ㄥ敖鍙兘闈犺繎鏁版嵁瀛樺偍鏈嶅姟鍣ㄧ殑浣嶇疆鍙戣捣鐨勭瓥鐣ワ紝鑷劧浼氬彈鐩婁簬 LOCALIO 鎻愪緵鐨勬暟鎹矾寰勪紭鍖栥€?

5. 涓轰綍涓嶅紑鍙戜竴绉嶆柊鐨?pNFS 甯冨眬鏉ュ惎鐢?LOCALIO锛?

   鍙互寮€鍙戜竴绉嶆柊鐨?pNFS 甯冨眬锛屼絾閭ｆ牱浼氭妸璐ｄ换鎺ㄧ粰鏈嶅姟鍣細鍦ㄥ喅瀹氬彂鏀惧竷灞€鏃讹紝鏈嶅姟鍣ㄥ繀椤讳互鏌愮鏂瑰紡鍙戠幇瀹㈡埛绔綅浜庢湰鍦般€侺OCALIO 鎵€鎻愪緵鐨勬洿绠€鍗曟柟娉曟洿鏈変环鍊尖€斺€斿畠璁?NFS 瀹㈡埛绔崗鍟嗗苟鍒╃敤灞€閮ㄦ€э紝鑰屾棤闇€浠ユ洿闆嗕腑鐨勬柟寮忓杩欑灞€閮ㄦ€ц繘琛屾洿澶嶆潅鐨勫缓妯′笌鍙戠幇銆?

6. 璁╁鎴风鍦ㄤ笉浣跨敤 RPC 鐨勬儏鍐典笅鎵ц鏈嶅姟绔枃浠?OPEN 鏈変綍濂藉锛熻濂藉鏄惁鐗瑰畾浜?pNFS锛?

   鏃犺鏄惁浣跨敤 pNFS锛岄伩鍏嶄负鏂囦欢鎵撳紑浣跨敤 XDR 涓?RPC 閮藉鎬ц兘鏈夌泭銆傚挨鍏舵槸澶勭悊灏忔枃浠舵椂锛屾渶濂藉敖鍙兘涓嶉€氳繃缃戠粶浼犺緭锛屽惁鍒欏彲鑳戒細鍓婂急鐢氳嚦鎶垫秷"涓哄皬鏂囦欢 I/O 鏈韩閬垮厤缃戠粶浼犺緭"鎵€甯︽潵鐨勫ソ澶勩€傞壌浜?LOCALIO 鐨勮姹傦紝褰撳墠璁╁鎴风鍦ㄤ笉浣跨敤 RPC 鐨勬儏鍐典笅鎵ц鏈嶅姟绔枃浠舵墦寮€鐨勫仛娉曟槸鐞嗘兂鐨勩€傝嫢灏嗘潵瑕佹眰鍙戠敓鍙樺寲锛屾垜浠彲浠ョ浉搴旇皟鏁淬€?

7. 涓轰綍 LOCALIO 浠呮敮鎸?UNIX 璁よ瘉锛圓UTH_UNIX锛夛紵

   寮鸿璇侀€氬父涓庤繛鎺ユ湰韬粦瀹氥€傚叾鍘熺悊鏄缓绔嬩竴涓敱鏈嶅姟鍣ㄧ紦瀛樼殑涓婁笅鏂囷紝璇ヤ笂涓嬫枃鍏呭綋鍙戠幇鎺堟潈浠ょ墝鐨勫瘑閽ワ紝闅忓悗鍙浼犻€掔粰 rpc.mountd 浠ュ畬鎴愯璇佽繃绋嬨€傚彟涓€鏂归潰锛屽浜?AUTH_UNIX锛岄€氳繃绾胯矾浼犻€掔殑鍑嵁琚洿鎺ョ敤浣?upcall 鍒?rpc.mountd 鏃剁殑瀵嗛挜銆傝繖绠€鍖栦簡璁よ瘉杩囩▼锛屽洜鑰屼娇 AUTH_UNIX 鏇存槗浜庢敮鎸併€?

8. 瀵逛簬浼氳浆鎹?RPC 鐢ㄦ埛 ID 鐨勫鍑洪€夐」锛堜緥濡?root_squash銆乤ll_squash锛夛紝鍦?LOCALIO 鎿嶄綔涓浣曞鐞嗭紵

   杞崲鐢ㄦ埛 ID 鐨勫鍑洪€夐」鐢?nfsd_setuser() 绠＄悊锛岃鍑芥暟鐢?nfsd_setuser_and_check_port() 璋冪敤锛岃€屽悗鑰呯敱 __fh_verify() 璋冪敤銆傚洜姝ゅ畠浠 LOCALIO 鐨勫鐞嗘柟寮忎笌闈?LOCALIO 瀹屽叏鐩稿悓銆?

9. 閴翠簬 NFSD 涓?NFS 鍦ㄤ笉鍚屼笂涓嬫枃涓繍琛岋紝LOCALIO 濡備綍纭繚瀵硅薄鐢熷懡鍛ㄦ湡琚Ε鍠勭鐞嗭紵

   璇﹁涓嬫枃 "NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄤ簰閿? 涓€鑺傘€?

## RPC


LOCALIO 杈呭姪 RPC 鍗忚鐢卞崟涓?"UUID_IS_LOCAL" RPC 鏂规硶缁勬垚锛岃鏂规硶鍏佽 Linux NFS 瀹㈡埛绔獙璇佹湰鍦?Linux NFS 鏈嶅姟鍣ㄨ兘鍚︾湅鍒板鎴风鐢熸垚骞跺湪 nfs_common 涓彁渚涚殑 nonce锛堜竴娆℃€?UUID锛夈€傝鍗忚骞堕潪 IETF 鏍囧噯鐨勪竴閮ㄥ垎锛屼篃鏃犳蹇呰锛屽洜涓哄畠鏈川涓婃槸 Linux 瀵?Linux 鐨勮緟鍔?RPC 鍗忚锛屽睘浜庡疄鐜扮粏鑺傘€?

UUID_IS_LOCAL 鏂规硶浠ュ浐瀹?UUID_SIZE锛?6 瀛楄妭锛夊瀹㈡埛绔敓鎴愮殑 uuid_t 杩涜缂栫爜銆備娇鐢ㄥ浐瀹氬ぇ灏忕殑涓嶉€忔槑锛坥paque锛塭ncode 涓?decode XDR 鏂规硶锛岃€岄潪鏁堢巼杈冧綆鐨勫彉闀挎柟娉曘€?

NFS_LOCALIO_PROGRAM 鐨?RPC 绋嬪簭鍙蜂负 400122锛堢敱 IANA 鍒嗛厤锛屽弬瑙?https://www.iana.org/assignments/rpc-program-numbers/ 锛夛細Linux Kernel Organization 400122 nfslocalio

```
  /* raw RFC 9562 UUID */
  #define UUID_SIZE 16
  typedef u8 uuid_t<UUID_SIZE>;

  program NFS_LOCALIO_PROGRAM {
      version LOCALIO_V1 {
          void
              NULL(void) = 0;

          void
              UUID_IS_LOCAL(uuid_t) = 1;
      } = 1;
  } = 400122;
```

LOCALIO 浣跨敤涓?NFS 娴侀噺鐩稿悓鐨勪紶杈撹繛鎺ャ€傚洜姝わ紝LOCALIO 涓嶄細鍚?rpcbind 娉ㄥ唽銆?

## NFS Common 涓庡鎴风/鏈嶅姟鍣ㄦ彙鎵?


fs/nfs_common/nfslocalio.c 鎻愪緵浜嗚嫢骞叉帴鍙ｏ紝浣?NFS 瀹㈡埛绔兘澶熺敓鎴?nonce锛堜竴娆℃€?UUID锛夊強鍏宠仈鐨勭煭鐢熷懡鍛ㄦ湡 nfs_uuid_t 缁撴瀯浣擄紝灏嗗叾娉ㄥ唽鍒?nfs_common 涓紝渚?NFS 鏈嶅姟鍣ㄥ悗缁煡鎵句笌楠岃瘉锛涜嫢鍖归厤锛孨FS 鏈嶅姟鍣ㄤ細濉厖 nfs_uuid_t 缁撴瀯浣撶殑鎴愬憳銆傞殢鍚?NFS 瀹㈡埛绔娇鐢?nfs_common 灏?nfs_uuid_t 浠庤嚜韬殑 nfs_uuids 杞Щ鍒?nfs_common 鐨?uuids_list 涓殑 nn->nfsd_serv clients_list銆傚弬瑙侊細fs/nfs/localio.c:nfs_local_probe()

nfs_common 鐨?nfs_uuids 鍒楄〃鏄?LOCALIO 鍚敤鏈哄埗鐨勫熀纭€锛屽洜姝ゅ畠鍖呭惈鎸囧悜 nfsd 鍐呭瓨銆佷緵瀹㈡埛绔洿鎺ヤ娇鐢ㄧ殑鎴愬憳锛堜緥濡?'net' 鏄湇鍔″櫒鐨勭綉缁滃懡鍚嶇┖闂达紝瀹㈡埛绔彲閫氳繃瀹冧互姝ｇ‘鐨?rcu 璇昏闂柟寮忚闂?nn->nfsd_serv锛夈€傛鏄繖绉嶅鎴风涓庢湇鍔″櫒鐨勫悓姝ワ紝浣垮緱楂樼骇鐢ㄦ硶浠ュ強瀵硅薄鐢熷懡鍛ㄦ湡鑳藉璺ㄨ秺浠庝富鏈哄唴鏍哥殑 nfsd 鍒拌繛鎺ュ埌杩愯浜庡悓涓€鏈湴涓绘満鐨?NFS 瀹㈡埛绔殑姣忓鍣?knfsd 瀹炰緥銆?

## NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄤ簰閿?


LOCALIO 鎻愪緵 nfs_uuid_t 瀵硅薄鍙婄浉鍏虫帴鍙ｏ紝浠ユ敮鎸佹纭殑缃戠粶鍛藉悕绌洪棿锛坣et-ns锛変笌 NFSD 瀵硅薄寮曠敤璁℃暟銆?

LOCALIO 闇€瑕佸紩鍏ュ苟浣跨敤 NFSD 鐨?percpu nfsd_net_ref锛屼互灏?nfsd_shutdown_net() 涓?nfsd_open_local_fh() 浜掗攣锛岀‘淇濇瘡涓?net-ns 鍦?nfsd_open_local_fh() 浣跨敤鏈熼棿涓嶈閿€姣侊紝杩欓渶瑕佹洿璇︾粏鐨勮В閲婏細

    nfsd_open_local_fh() 鍦ㄦ墦寮€鍏?nfsd_file 鍙ユ焺涔嬪墠浼氫娇鐢?
    nfsd_net_try_get()锛岄殢鍚庤皟鐢ㄦ柟锛圢FS 瀹㈡埛绔級蹇呴』鍦ㄥ畬鎴愬叾 IO 涔嬪悗锛?
    浣跨敤 nfsd_file_put_local() 閲婃斁璇?nfsd_file 鍙婂叧鑱旂殑 net-ns
    寮曠敤銆?

    璇ヤ簰閿佹満鍒惰兘鍚︽甯稿伐浣滐紝寰堝ぇ绋嬪害涓婁緷璧栦簬 nfsd_open_local_fh() 鏄惁鍏峰瀹夊叏澶勭悊濡備笅鍙兘鎬х殑鑳藉姏锛歂FSD 鐨?net-ns锛堜互鍙婂叧鑱旂殑 nfsd_net锛夊彲鑳藉凡琚?nfsd_destroy_serv() 缁忕敱 nfsd_shutdown_net() 閿€姣併€?

缁忛獙璇侊紝NFS 瀹㈡埛绔笌鏈嶅姟鍣ㄧ殑杩欎竴浜掗攣鏈哄埗淇浜嗕竴涓鏄撹Е鍙戠殑宕╂簝锛氬綋瀹瑰櫒涓繍琛屻€佷笖鎸傝浇浜?LOCALIO 瀹㈡埛绔殑 NFSD 瀹炰緥琚叧闂椂浼氬彂鐢熻宕╂簝銆傚鍣ㄥ強鐩稿叧 NFSD 閲嶅惎鍚庯紝鐢变簬 LOCALIO 瀹㈡埛绔湪灏氭湭瀵?NFSD 鐨?net-ns 鎸佹湁姝ｇ‘寮曠敤鐨勬儏鍐典笅灏卞皾璇?nfsd_open_local_fh()锛屽鎴风浼氬洜 NULL 鎸囬拡瑙ｅ紩鐢ㄨ€岄殢涔嬪穿婧冦€?

## 鐢?NFS 瀹㈡埛绔彂璧?IO 鑰岄潪鏈嶅姟鍣?


鐢变簬 LOCALIO 鐫€鐪间簬閫氳繃鍗忚缁曡繃鏉ュ疄鐜版洿楂樼殑 IO 鎬ц兘锛屽繀椤绘彁渚涗紶缁?NFS 绾胯矾鍗忚锛圫UNRPC 鍔?XDR锛夌殑鏇夸唬鏂规锛屼互璁块棶搴曞眰鏂囦欢绯荤粺銆?

鍙傝 fs/nfs/localio.c:nfs_local_open_fh() 涓?fs/nfsd/localio.c:nfsd_open_local_fh()锛屽叾涓粰鍑轰簡鐩稿叧鎺ュ彛锛屾湁閫夋嫨鍦板埄鐢?NFS 鏈嶅姟鍣ㄥ璞★紝浣夸綅浜庢湇鍔″櫒鏈湴鐨勫鎴风鏃犻渶缁忚繃缃戠粶鍗冲彲鎵撳紑鏂囦欢鎸囬拡銆?

瀹㈡埛绔殑 fs/nfs/localio.c:nfs_local_open_fh() 浼氳皟鐢ㄦ湇鍔″櫒鐨?fs/nfsd/localio.c:nfsd_open_local_fh()锛屽苟浠?RCU 鏂瑰紡璋ㄦ厧璁块棶鐩稿叧鐨?nfsd 缃戠粶鍛藉悕绌洪棿涓?nn->nfsd_serv銆傚鏋?nfsd_open_local_fh() 鍙戠幇瀹㈡埛绔笉鍐嶇湅鍒版湁鏁堢殑 nfsd 瀵硅薄锛堟棤璁烘槸 struct net 杩樻槸 nn->nfsd_serv锛夛紝瀹冧細鍚?nfs_local_open_fh() 杩斿洖 -ENXIO锛屽鎴风鍒欎細閫氳繃鍐嶆璋冪敤 nfs_local_probe() 鏉ュ皾璇曢噸寤烘墍闇€鐨?LOCALIO 璧勬簮銆傚綋瀹瑰櫒涓繍琛岀殑 nfsd 瀹炰緥鍦?LOCALIO 瀹㈡埛绔繛鎺ユ湡闂撮噸鍚椂锛屽氨闇€瑕佽繖绉嶆仮澶嶃€?

涓€鏃﹀鎴风鎸佹湁宸叉墦寮€鐨?nfsd_file 鎸囬拡锛屽畠灏变細鐩存帴鍚戝簳灞傛湰鍦版枃浠剁郴缁燂紙閫氬父鐢?nfs 鏈嶅姟鍣ㄥ畬鎴愶級鍙戣捣璇汇€佸啓鍜屾彁浜ゃ€傚洜姝わ紝瀵逛簬杩欎簺鎿嶄綔锛孨FS 瀹㈡埛绔槸鍚戝叾涓?NFS 鏈嶅姟鍣ㄥ叡浜殑搴曞眰鏈湴鏂囦欢绯荤粺鍙戣捣 IO銆傚弬瑙侊細fs/nfs/localio.c:nfs_local_doio() 涓?fs/nfs/localio.c:nfs_local_commit()銆?

瀵逛簬浣跨敤 RPC 鍚戞湇鍔″櫒鍙戣捣 IO 鐨勬櫘閫?NFS锛屽鏋滃簲鐢ㄧ▼搴忎娇鐢?O_DIRECT锛孨FS 瀹㈡埛绔細缁曡繃 pagecache锛屼絾 NFS 鏈嶅姟鍣ㄤ笉浼氥€侼FS 鏈嶅姟鍣ㄤ娇鐢ㄧ紦鍐?IO锛屼娇搴旂敤绋嬪簭鍦ㄥ悜 NFS 瀹㈡埛绔彂璧?IO 鏃跺瀵归綈鐨勮姹傚彲浠ュ鏉句竴浜涖€備絾濡傛灉鎵€鏈夊簲鐢ㄧ▼搴忛兘姝ｇ‘瀵归綈鍏?IO锛屽垯鍙€氳繃灏?'localio_O_DIRECT_semantics' nfs 妯″潡鍙傛暟璁句负 Y锛屽皢 LOCALIO 閰嶇疆涓轰粠 NFS 瀹㈡埛绔埌鍏朵笌 NFS 鏈嶅姟鍣ㄥ叡浜殑搴曞眰鏈湴鏂囦欢绯荤粺浣跨敤绔埌绔殑 O_DIRECT 璇箟锛屼緥濡傦細

    echo Y > /sys/module/nfs/parameters/localio_O_DIRECT_semantics

涓€鏃﹀惎鐢紝杩欏皢浣?LOCALIO 浣跨敤绔埌绔殑 O_DIRECT 璇箟锛堜絾鍚屾牱锛屽鏋滃簲鐢ㄧ▼搴忔湭姝ｇ‘瀵归綈鍏?IO锛岃繖鍙兘瀵艰嚧 IO 澶辫触锛夈€?

## 瀹夊叏鎬?


LOCALIO 浠呭湪浣跨敤 UNIX 椋庢牸璁よ瘉锛圓UTH_UNIX锛屽嵆 AUTH_SYS锛夋椂鍙楁敮鎸併€?

鎴戜滑娉ㄦ剰纭繚鏃犺浣跨敤 LOCALIO 杩樻槸甯歌 NFS 璁块棶锛岄兘閲囩敤鐩稿悓鐨?NFS 瀹夊叏鏈哄埗锛堣璇佺瓑锛夈€備綔涓轰紶缁?NFS 瀹㈡埛绔闂?NFS 鏈嶅姟鍣ㄤ竴閮ㄥ垎鑰屽缓绔嬬殑 auth_domain锛屽悓鏍风敤浜?LOCALIO銆?

灏卞鍣ㄨ€岃█锛孡OCALIO 璁╁鎴风鑳藉璁块棶鏈嶅姟鍣ㄦ嫢鏈夌殑缃戠粶鍛藉悕绌洪棿銆傝繖鏄负浜嗚瀹㈡埛绔兘澶熻闂湇鍔″櫒鎸夊懡鍚嶇┖闂村垝鍒嗙殑 nfsd_net 缁撴瀯浣撴墍蹇呴渶鐨勩€傚浜庝紶缁?NFS锛屽鎴风浜湁鍚岀瓑鐨勮闂骇鍒紙灏界鏄€氳繃 SUNRPC 浠?NFS 鍗忚鐨勬柟寮忥級銆傛病鏈夊叾浠栧懡鍚嶇┖闂达紙user銆乵ount 绛夛級琚粠鏈嶅姟鍣ㄦ敼鍔ㄦ垨鐗规剰鎵╁睍鍒板鎴风銆?

## 妯″潡鍙傛暟


/sys/module/nfs/parameters/localio_enabled (bool)
鎺у埗鏄惁鍚敤 LOCALIO锛岄粯璁や负 Y銆傚鏋滃鎴风涓庢湇鍔″櫒浣嶄簬鏈湴锛屼絾 'localio_enabled' 璁句负 N锛屽垯涓嶄細浣跨敤 LOCALIO銆?

/sys/module/nfs/parameters/localio_O_DIRECT_semantics (bool)
鎺у埗 O_DIRECT 鏄惁鍚戜笅寤朵几鍒板簳灞傛枃浠剁郴缁燂紝榛樿涓?N銆傚簲鐢ㄧ▼搴?IO 蹇呴』鎸夐€昏緫鍧楀ぇ灏忓榻愶紝鍚﹀垯 O_DIRECT 浼氬け璐ャ€?

/sys/module/nfsv3/parameters/nfs3_localio_probe_throttle (uint)
鎺у埗 NFSv3 璇汇€佸啓 IO 鏄惁姣?N锛坣fs3_localio_probe_throttle锛夋 IO 瑙﹀彂 LOCALIO 鐨勶紙閲嶆柊锛夊惎鐢紝榛樿涓?0锛堢鐢級銆傚繀椤讳负 2 鐨勫箓锛涜嫢绠＄悊鍛橀厤缃笉褰擄紙鍊艰繃浣庢垨闈?2 鐨勫箓锛夛紝鍚庢灉鑷礋銆?

## 娴嬭瘯


LOCALIO 杈呭姪鍗忚鍙婄浉鍏崇殑 NFS LOCALIO 璇汇€佸啓鍜屾彁浜よ闂紝宸插湪鍚勭娴嬭瘯鍦烘櫙涓嬭璇佹槑鏄ǔ瀹氱殑锛?

- 瀹㈡埛绔笌鏈嶅姟鍣ㄥ潎浣嶄簬鍚屼竴涓绘満銆?

- 鏈湴涓庤繙绋嬪鎴风銆佹湇鍔″櫒鏀寔鐨勫惎鐢ㄧ粍鍚堢殑鎵€鏈夋帓鍒椼€?

- 涔熼拡瀵逛笉鏀寔 LOCALIO 鍗忚鐨?NFS 瀛樺偍浜у搧杩涜浜嗘祴璇曘€?

- 瀹㈡埛绔湪涓绘満涓娿€佹湇鍔″櫒鍦ㄥ鍣ㄥ唴锛坴3 涓?v4.2 鍧囪鐩栵級銆傚鍣ㄦ祴璇曞熀浜?podman 绠＄悊鐨勫鍣紝骞跺寘鍚垚鍔熺殑瀹瑰櫒鍋滄/閲嶅惎鍦烘櫙銆?

- 灏嗚繖浜涙祴璇曞満鏅舰寮忓寲绾冲叆鐜版湁娴嬭瘯鍩虹璁炬柦鐨勫伐浣滄鍦ㄨ繘琛屼腑銆傚垵姝ョ殑甯歌瑕嗙洊鐢?ktest 瀵瑰惎鐢ㄤ簡 LOCALIO 鐨?NFS 鐜洖鎸傝浇閰嶇疆杩愯 xfstests 鎻愪緵锛屽苟鍖呭惈 lockdep 涓?KASAN 瑕嗙洊锛屽弬瑙侊細
  https://evilpiepirate.org/~testdashboard/ci?user=snitzer&branch=snitm-nfs-next
  https://github.com/koverstreet/ktest

- 宸茶繘琛屽悇绉?kdevops 娴嬭瘯锛堝嵆 "Chuck's BuildBot"锛夛紝浠ュ畾鏈熼獙璇?LOCALIO 鏀瑰姩鏈闈?LOCALIO 鐨?NFS 鐢ㄤ緥閫犳垚浠讳綍鍥炲綊銆?

- Hammerspace 鐨勫悇绉嶅仴鍏ㄦ€ф祴璇曞湪鍚敤 LOCALIO 鏃跺叏閮ㄩ€氳繃锛堝叾涓寘鎷ぇ閲?pNFS 涓?flexfiles 娴嬭瘯锛夈€?
