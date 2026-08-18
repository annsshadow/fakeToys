
## ORANGEFS


OrangeFS 鏄竴涓?LGPL 鐨勭敤鎴风┖闂存í鍚戞墿灞曪紙scale-out锛夊苟琛屽瓨鍌ㄧ郴缁熴€傚畠闈炲父閫傚悎 HPC銆?
澶ф暟鎹紙BigData锛夈€佹祦濯掍綋瑙嗛锛圫treaming Video锛夈€佸熀鍥犵粍瀛︼紙Genomics锛夈€佺敓鐗╀俊鎭
锛圔ioinformatics锛夋墍闈复鐨勫ぇ瑙勬ā瀛樺偍闂銆?

Orangefs 鏈€鍒濈О涓?PVFS锛屼簬 1993 骞寸敱 Walt Ligon 鍜?Eric Blumer 棣栨寮€鍙戯紝浣滀负涓€涓?
骞惰铏氭嫙鏈猴紙Parallel Virtual Machine, PVM锛夌殑骞惰鏂囦欢绯荤粺锛屼綔涓?NASA 璧勫姪鐮旂┒骞惰
绋嬪簭 I/O 妯″紡鐨勪竴閮ㄥ垎銆?

Orangefs 鐨勭壒鎬у寘鎷細

  - 鍦ㄥ涓枃浠舵湇鍔″櫒涔嬮棿鍒嗗竷鏂囦欢鏁版嵁
  - 鏀寔澶氫釜瀹㈡埛绔悓鏃惰闂?
  - 浣跨敤鏈湴鏂囦欢绯荤粺鍜岃闂柟娉曞湪鏈嶅姟鍣ㄤ笂瀛樺偍鏂囦欢鏁版嵁鍜屽厓鏁版嵁
  - 鐢ㄦ埛绌洪棿瀹炵幇鏄撲簬瀹夎鍜岀淮鎶?
  - 鐩存帴鐨?MPI 鏀寔
  - 鏃犵姸鎬侊紙Stateless锛?


## 閭欢鍒楄〃褰掓。


http://lists.orangefs.org/pipermail/devel_lists.orangefs.org/


## 閭欢鍒楄〃鎶曠


devel@lists.orangefs.org


## 鏂囨。


http://www.orangefs.org/documentation/

## 鍦ㄥ崟鍙版湇鍔″櫒涓婅繍琛?ORANGEFS


OrangeFS 閫氬父鍦ㄥ叿鏈夊涓湇鍔″櫒鍜屽鎴风鐨勫簽澶ч儴缃蹭腑杩愯锛屼絾涓轰簡寮€鍙戝拰娴嬭瘯锛屼篃鍙互鍦ㄥ崟鍙?
鏈哄櫒涓婅繍琛屼竴涓畬鏁寸殑鏂囦欢绯荤粺銆?

```
    dnf -y install orangefs orangefs-server
```

鍦?/etc/orangefs/orangefs.conf 涓湁涓€涓ず渚嬫湇鍔″櫒閰嶇疆鏂囦欢銆傚鏈夊繀瑕侊紝灏?localhost 鏀逛负
浣犵殑涓绘満鍚嶏紙hostname锛夈€?

鍏充簬鐢熸垚涓€涓敤浜庤繍琛?xfstests 鐨勬枃浠剁郴缁燂紝璇疯涓嬫枃銆?

鍦?/etc/pvfs2tab 涓湁涓€涓ず渚嬪鎴风閰嶇疆鏂囦欢銆傚畠鏄崟琛屻€傚彇娑堝叾娉ㄩ噴锛屽苟鍦ㄥ繀瑕佹椂鏇存敼
涓绘満鍚嶃€傝繖鎺у埗浣跨敤 libpvfs2 鐨勫鎴风銆傝繖骞朵笉鎺у埗 pvfs2-client-core銆?

```
    pvfs2-server -f /etc/orangefs/orangefs.conf
```

```
    systemctl start orangefs-server
```

```
    pvfs2-ping -m /pvfsmnt
```

鍚姩瀹㈡埛绔€傚湪姝ゆ搷浣滀箣鍓嶏紝妯″潡蹇呴』宸茬紪璇戣繘鍐呮牳鎴栧凡鍔犺浇锛?

```
    systemctl start orangefs-client
```

```
    mount -t pvfs2 tcp://localhost:3334/orangefs /pvfsmnt
```

## 鐢ㄦ埛绌洪棿鏂囦欢绯荤粺婧愮爜


http://www.orangefs.org/download

2.9.3 涔嬪墠鐨?Orangefs 鐗堟湰涓庝笂娓哥増鏈殑鍐呮牳瀹㈡埛绔笉鍏煎銆?


## 鍦ㄥ崟鍙版湇鍔″櫒涓婃瀯寤?ORANGEFS


濡傛灉 OrangeFS 鏃犳硶浠庡彂琛岀増杞欢鍖呭畨瑁咃紝鍙互浠庢簮鐮佹瀯寤恒€?

濡傛灉浣犱笉浠嬫剰涓滆タ鏁ｈ惤鍦?/usr/local 鍚勫锛屽彲浠ョ渷鐣?--prefix銆備粠 2.9.6 鐗堟湰璧凤紝OrangeFS
榛樿浣跨敤 Berkeley DB锛屾垜浠彲鑳藉緢蹇細灏嗛粯璁ゅ€兼敼涓?LMDB銆?

```
    ./configure --prefix=/opt/ofs --with-db-backend=lmdb --disable-usrint

    make

    make install
```

閫氳繃杩愯 pvfs2-genconfig 骞舵寚瀹氫竴涓洰鏍囬厤缃枃浠舵潵鍒涘缓 orangefs 閰嶇疆鏂囦欢銆侾vfs2-genconfig
浼氶€氳繃鎻愮ず寮曞浣犲畬鎴愩€傞€氬父鐩存帴閲囩敤榛樿鍊煎嵆鍙紝浣嗕綘搴斿綋浣跨敤浣犵殑鏈嶅姟鍣ㄤ富鏈哄悕锛岃€屼笉鏄?
鈥渓ocalhost鈥濓細

```
    /opt/ofs/bin/pvfs2-genconfig /etc/pvfs2.conf
```

```
    echo tcp://localhost:3334/orangefs /pvfsmnt pvfs2 defaults,noauto 0 0 > \
	/etc/pvfs2tab
```

```
    mkdir /pvfsmnt
```

```
    /opt/ofs/sbin/pvfs2-server -f /etc/pvfs2.conf
```

```
    /opt/ofs/sbin/pvfs2-server /etc/pvfs2.conf
```

鐜板湪鏈嶅姟鍣ㄥ簲褰撳凡缁忓湪杩愯銆侾vfs2-ls 鏄竴涓畝鍗曠殑锛?

```
    /opt/ofs/bin/pvfs2-ls /pvfsmnt
```

濡傛灉涓€鍒囦技涔庡伐浣滄甯革紝鍔犺浇鍐呮牳妯″潡骞舵墽琛岋細

```
    /opt/ofs/sbin/pvfs2-client -p /opt/ofs/sbin/pvfs2-client-core
```

```
    mount -t pvfs2 tcp://`hostname`:3334/orangefs /pvfsmnt
```

## 杩愯 xfstests


灏?xfstests 涓?scratch 鏂囦欢绯荤粺閰嶅悎浣跨敤寰堟湁鐢ㄣ€傝繖鍙互鍙娇鐢ㄤ竴鍙版湇鍔″櫒鏉ュ畬鎴愩€?

鍦ㄦ湇鍔″櫒閰嶇疆鏂囦欢锛堝嵆 /etc/orangefs/orangefs.conf锛変腑澶嶅埗涓€浠?FileSystem 娈点€傚皢 Name
鏀逛负 scratch銆傚皢 ID 鏀逛负涓庣涓€涓?FileSystem 娈电殑 ID 涓嶅悓鐨勫€硷紙2 閫氬父鏄釜濂介€夋嫨锛夈€?

杩欐牱灏辨湁涓や釜 FileSystem 娈碉細orangefs 鍜?scratch銆?

姝ゆ洿鏀瑰簲鍦ㄥ垱寤烘枃浠剁郴缁熶箣鍓嶈繘琛屻€?

```
    pvfs2-server -f /etc/orangefs/orangefs.conf
```

```
    TEST_DIR=/orangefs
    TEST_DEV=tcp://localhost:3334/orangefs
    SCRATCH_MNT=/scratch
    SCRATCH_DEV=tcp://localhost:3334/scratch
```

```
    ./check -pvfs2
```

## 閫夐」


鎺ュ彈浠ヤ笅鎸傝浇锛坢ount锛夐€夐」锛?

  acl
    鍏佽鍦ㄦ枃浠跺拰鐩綍涓婁娇鐢ㄨ闂帶鍒跺垪琛紙Access Control List锛夈€?

  intr
    鍐呮牳瀹㈡埛绔笌鐢ㄦ埛绌洪棿鏂囦欢绯荤粺涔嬮棿鐨勪竴浜涙搷浣滃彲浠ヨ涓柇锛坕nterruptible锛夛紝渚嬪
    璋冭瘯锛坉ebug锛夌骇鍒殑鏇存敼鍜?tunable 鍙傛暟鐨勮缃€?

  local_lock
    浠?鈥滄湰鈥?鍐呮牳鐨勮瑙掑惎鐢?posix 閿佸畾銆傞粯璁ょ殑 file_operations 閿佸畾鍔ㄤ綔鏄繑鍥?ENOSYS銆?
    濡傛灉鏂囦欢绯荤粺浠?-o local_lock 鎸傝浇锛屽垯 posix 閿佸畾鐢熸晥銆傚垎甯冨紡閿佸畾姝ｅ湪涓烘湭鏉ヨ繘琛?
    寮€鍙戜腑銆?


## 璋冭瘯


濡傛灉浣犳兂鍦ㄧ壒瀹氱殑 GOSSIP 璇彞涓惎鐢ㄨ皟璇曪紝鍒欙細

```
  echo inode > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo none > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo inode,dir > /sys/kernel/debug/orangefs/kernel-debug
```

```
  echo all > /sys/kernel/debug/orangefs/kernel-debug
```

```
  cat /sys/kernel/debug/orangefs/debug-help
```

## 鍐呮牳妯″潡涓庣敤鎴风┖闂翠箣闂寸殑鍗忚


Orangefs 鏄竴涓敤鎴风┖闂存枃浠剁郴缁熶互鍙婄浉鍏宠仈鐨勫唴鏍告ā鍧椼€傛鍚庢垜浠皢 Orangefs 鐨勭敤鎴风┖闂?
閮ㄥ垎绠€绉颁负 鈥渦serspace鈥濄€侽rangefs 婧愯嚜 PVFS锛岃€岀敤鎴风┖闂翠唬鐮佸湪鍑芥暟鍜屽彉閲忓悕涓粛鐒朵娇鐢?
PVFS銆傜敤鎴风┖闂?typedef 浜嗚澶氶噸瑕佺殑缁撴瀯銆傚唴鏍告ā鍧椾腑鐨勫嚱鏁板拰鍙橀噺鍚嶅凡缁忚繃娓″埌
鈥渙rangefs鈥濓紝鑰屼笖 Linux 缂栫爜椋庢牸锛圕oding Style锛夐伩鍏嶄娇鐢?typedef锛屽洜姝や笌鐢ㄦ埛绌洪棿缁撴瀯
瀵瑰簲鐨勫唴鏍告ā鍧楃粨鏋勬病鏈夎 typedef銆?

鍐呮牳妯″潡瀹炵幇浜嗕竴涓吉璁惧锛坧seudo device锛夛紝鐢ㄦ埛绌洪棿鍙互瀵瑰叾杩涜璇诲拰鍐欍€傜敤鎴风┖闂磋繕鍙互
閫氳繃浼澶囩敤 ioctl 鎿嶆帶鍐呮牳妯″潡銆?

### Bufmap锛堢紦鍐插尯鏄犲皠锛?


鍦ㄥ惎鍔ㄦ椂锛岀敤鎴风┖闂村垎閰嶄袱涓寜椤靛ぇ灏忓榻愶紙posix_memalign锛夌殑 mlocked 鍐呭瓨缂撳啿鍖猴紝涓€涓?
鐢ㄤ簬 IO锛屼竴涓敤浜?readdir 鎿嶄綔銆侷O 缂撳啿鍖轰负 41943040 瀛楄妭锛宺eaddir 缂撳啿鍖轰负 4194304
瀛楄妭銆傛瘡涓紦鍐插尯鍖呭惈閫昏緫鍧楋紙chunk锛夋垨鍒嗗尯锛坧artition锛夛紝骞朵笖姣忎釜缂撳啿鍖虹殑鎸囬拡琚姞鍏ュ叾
鑷繁鐨?PVFS_dev_map_desc 缁撴瀯涓紝璇ョ粨鏋勮繕鎻忚堪浜嗗叾鎬诲ぇ灏忥紝浠ュ強鍒嗗尯鐨勫ぇ灏忓拰鏁伴噺銆?

鎸囧悜 IO 缂撳啿鍖虹殑 PVFS_dev_map_desc 缁撴瀯鐨勬寚閽堥€氳繃 ioctl 琚彂閫佺粰鍐呮牳妯″潡涓殑涓€涓槧灏?
渚嬬▼銆傝缁撴瀯閫氳繃 copy_from_user 浠庣敤鎴风┖闂村鍒跺埌鍐呮牳绌洪棿锛屽苟鐢ㄤ簬鍒濆鍖栧唴鏍告ā鍧楃殑
鈥渂ufmap鈥濓紙struct orangefs_bufmap锛夛紝鍏堕殢鍚庡寘鍚細

  - refcnt
    - 涓€涓紩鐢ㄨ鏁板櫒
  - desc_size - PVFS2_BUFMAP_DEFAULT_DESC_SIZE (4194304) - IO 缂撳啿鍖虹殑
    鍒嗗尯澶у皬锛屼唬琛ㄦ枃浠剁郴缁熺殑鍧楀ぇ灏忥紝骞剁敤浜庤秴绾у潡锛坰uper block锛変腑鐨?s_blocksize銆?
  - desc_count - PVFS2_BUFMAP_DEFAULT_DESC_COUNT (10) - IO 缂撳啿鍖轰腑鐨勫垎鍖烘暟閲忋€?
  - desc_shift - log2(desc_size)锛岀敤浜庤秴绾у潡涓殑 s_blocksize_bits銆?
  - total_size - IO 缂撳啿鍖虹殑鎬诲ぇ灏忋€?
  - page_count - IO 缂撳啿鍖轰腑 4096 瀛楄妭椤电殑鏁伴噺銆?
  - page_array - 鎸囧悜 `page_count * (sizeof(struct page *))` 瀛楄妭鐨?kcalloced
    鍐呭瓨鐨勬寚閽堛€傝鍐呭瓨閫氳繃璋冪敤 get_user_pages 琚敤浣滄寚鍚?IO 缂撳啿鍖轰腑姣忎釜椤电殑鎸囬拡鏁扮粍銆?
  - desc_array - 鎸囧悜 `desc_count * (sizeof(struct orangefs_bufmap_desc))` 瀛楄妭鐨?
    kcalloced 鍐呭瓨鐨勬寚閽堛€傝鍐呭瓨琚繘涓€姝ュ垵濮嬪寲锛?

      user_desc 鏄?IO 缂撳啿鍖虹殑 ORANGEFS_dev_map_desc 缁撴瀯鐨勫唴鏍稿壇鏈€?
      user_desc->ptr 鎸囧悜 IO 缂撳啿鍖恒€?

```
	pages_per_desc = bufmap->desc_size / PAGE_SIZE
	offset = 0

        bufmap->desc_array[0].page_array = &bufmap->page_array[offset]
        bufmap->desc_array[0].array_count = pages_per_desc = 1024
        bufmap->desc_array[0].uaddr = (user_desc->ptr) + (0 * 1024 * 4096)
        offset += 1024
                           .
                           .
                           .
        bufmap->desc_array[9].page_array = &bufmap->page_array[offset]
        bufmap->desc_array[9].array_count = pages_per_desc = 1024
        bufmap->desc_array[9].uaddr = (user_desc->ptr) +
                                               (9 * 1024 * 4096)
        offset += 1024

  * buffer_index_array - 涓€涓?desc_count 澶у皬鐨?int 鏁扮粍锛岀敤浜庢寚绀?IO 缂撳啿鍖虹殑
    鍝簺鍒嗗尯鍙緵浣跨敤銆?
  * buffer_index_lock - 涓€涓嚜鏃嬮攣锛坰pinlock锛夛紝鐢ㄤ簬鍦ㄦ洿鏂版湡闂翠繚鎶?buffer_index_array銆?
  * readdir_index_array - 涓€涓簲锛圤RANGEFS_READDIR_DEFAULT_DESC_COUNT锛夊厓绱犵殑 int
    鏁扮粍锛岀敤浜庢寚绀?readdir 缂撳啿鍖虹殑鍝簺鍒嗗尯鍙緵浣跨敤銆?
  * readdir_index_lock - 涓€涓嚜鏃嬮攣锛岀敤浜庡湪鏇存柊鏈熼棿淇濇姢 readdir_index_array銆?
```

### 鎿嶄綔锛圤perations锛?


褰撳唴鏍告ā鍧楅渶瑕佷笌鐢ㄦ埛绌洪棿閫氫俊鏃讹紝瀹冧細鏋勫缓涓€涓?鈥渙p鈥濓紙struct orangefs_kernel_op_s锛夈€俹p
鐨勪竴閮ㄥ垎鍖呭惈鍚戠敤鎴风┖闂磋〃杈捐姹傜殑 鈥渦pcall锛堜笂琛岃皟鐢級鈥濄€俹p 鐨勪竴閮ㄥ垎鏈€缁堝寘鍚〃杈捐姹?
缁撴灉鐨?鈥渄owncall锛堜笅琛岃皟鐢級鈥濄€?

slab 鍒嗛厤鍣ㄨ鐢ㄦ潵淇濇寔涓€涓殢鏃跺彲鐢ㄧ殑 op 缁撴瀯缂撳瓨銆?

鍦ㄥ垵濮嬪寲鏃讹紝鍐呮牳妯″潡瀹氫箟骞跺垵濮嬪寲涓€涓姹傚垪琛紙request list锛夊拰涓€涓?in_progress 鍝堝笇琛?
锛坔ash table锛夛紝浠ヨ窡韪湪浠讳綍缁欏畾鏃跺埢鎵€鏈夊湪閫旓紙in flight锛夌殑 op銆?

Op 鏄湁鐘舵€佺殑锛?

 - unknown
     - op 鍒氬垰琚垵濮嬪寲
 - waiting
     - op 鍦?request_list 涓婏紙鍚戜笂绛夊緟锛?
 - inprogr
     - op 姝ｅ湪杩涜涓紙绛夊緟 downcall锛?
 - serviced
     - op 鏈夊尮閰嶇殑 downcall锛涙甯?
 - purged
     - op 蹇呴』鍚姩涓€涓畾鏃跺櫒锛屽洜涓?client-core 鍦ㄦ湇鍔′簬璇?op 涔嬪墠涓嶅共鍑€鍦伴€€鍑轰簡
 - given up
     - 鎻愪氦鑰呭凡鏀惧純绛夊緟瀹?

褰撴煇涓换鎰忕殑鐢ㄦ埛绌洪棿绋嬪簭闇€瑕佸湪 Orangefs 涓婃墽琛屼竴涓枃浠剁郴缁熸搷浣滐紙readdir銆両/O銆乧reate
鎴栧叾瀹冿級鏃讹紝浼氬垵濮嬪寲涓€涓?op 缁撴瀯骞舵墦涓婁竴涓敤浜庡尯鍒嗙殑 ID 鍙锋爣绛俱€俹p 鐨?upcall 閮ㄥ垎琚?
濉厖锛岀劧鍚庤 op 琚紶閫掔粰 鈥渟ervice_operation鈥?鍑芥暟銆?

service_operation 灏?op 鐨勭姸鎬佹敼涓?鈥渨aiting鈥濓紝灏嗗叾鏀惧叆璇锋眰鍒楄〃锛屽苟閫氳繃绛夊緟闃熷垪锛坵ait
queue锛夊悜 Orangefs 鐨?file_operations.poll 鍑芥暟鍙戜俊鍙枫€傜敤鎴风┖闂存鍦ㄨ疆璇紙poll锛変吉璁惧锛?
浠庤€屽緱鐭ラ渶瑕佽璇诲彇鐨?upcall 璇锋眰銆?

褰?Orangefs 鐨?file_operations.read 鍑芥暟琚Е鍙戞椂锛屼細鍦ㄨ姹傚垪琛ㄤ腑鎼滅储涓€涓技涔庡凡鍑嗗濂?
澶勭悊鐨?op銆傝 op 浠庤姹傚垪琛ㄤ腑绉婚櫎銆俹p 鐨?tag 鍜屽凡濉厖鐨?upcall 缁撴瀯閫氳繃 copy_to_user
澶嶅埗鍥炵敤鎴风┖闂淬€?

濡傛灉杩欎簺 copy_to_user锛堜互鍙婁竴浜涢澶栫殑鍗忚锛変腑鏈変换浣曞け璐ワ紝op 鐨勭姸鎬佽璁句负 鈥渨aiting鈥濓紝
骞朵笖璇?op 琚姞鍥炶姹傚垪琛ㄣ€傚惁鍒欙紝op 鐨勭姸鎬佽鏀逛负 鈥渋n progress鈥濓紝骞朵笖璇?op 鎸夊叾 tag 琚?
鍝堝笇锛坔ash锛夛紝鏀惧埌 in_progress 鍝堝笇琛ㄤ腑璇?tag 鎵€鍝堝笇鍒扮殑绱㈠紩澶勭殑鍒楄〃鏈熬銆?

褰撶敤鎴风┖闂寸粍瑁呭ソ瀵?upcall 鐨勫搷搴斿悗锛屽畠灏嗗寘鍚鍖哄垎 tag 鐨勫搷搴旓紝浠ヤ竴绯诲垪 io_vecs 鍐欏洖
浼澶囥€傝繖浼氳Е鍙?Orangefs 鐨?file_operations.write_iter 鍑芥暟鎵惧埌鍏锋湁鍏宠仈 tag 鐨?op锛屽苟
灏嗗叾浠?in_progress 鍝堝笇琛ㄤ腑绉婚櫎銆傚彧瑕佽 op 鐨勭姸鎬佷笉鏄?鈥渃anceled鈥?鎴?鈥済iven up鈥濓紝鍏?
鐘舵€佸氨琚涓?鈥渟erviced鈥濄€俧ile_operations.write_iter 鍑芥暟杩斿洖鍒扮瓑寰呬腑鐨?vfs锛屽苟缁忕敱
wait_for_matching_downcall 杩斿洖鍒?service_operation銆?

service_operation 甯︾潃 op 鐨?downcall 閮ㄥ垎锛堝 upcall 鐨勫搷搴旓級琚～鍏呭畬姣曡€岃繑鍥炵粰鍏惰皟鐢ㄨ€呫€?

鈥渃lient-core鈥?鏄唴鏍告ā鍧椾笌鐢ㄦ埛绌洪棿涔嬮棿鐨勬ˉ姊併€俢lient-core 鏄竴涓畧鎶よ繘绋嬶紙daemon锛夈€?
client-core 鏈変竴涓浉鍏宠仈鐨勭湅闂ㄧ嫍锛坵atchdog锛夊畧鎶よ繘绋嬨€傚鏋?client-core 琚俊鍙疯姹傞€€鍑猴紝
鐪嬮棬鐙楀畧鎶よ繘绋嬩細閲嶅惎 client-core銆傚嵆浣?client-core 琚?鈥滅珛鍗斥€?閲嶅惎锛屽湪姝ょ被浜嬩欢鍙戠敓鏈熼棿
浠嶆湁涓€娈垫椂闂?client-core 鏄鐨勩€傛鐨?client-core 鏃犳硶琚?Orangefs 鐨?
file_operations.poll 鍑芥暟瑙﹀彂銆傚湪 鈥滄浜℃湡鈥?闂撮€氳繃 service_operation 鐨?op 鍙兘浼氬湪绛夊緟
闃熷垪涓婅秴鏃讹紝姝ゆ椂浼氬皾璇曞洖鏀跺畠浠竴娆°€傛樉鐒讹紝濡傛灉 client-core 姝讳骸鏃堕棿杩囬暱锛岃瘯鍥句娇鐢?
Orangefs 鐨勯偅浜涗换鎰忕敤鎴风┖闂磋繘绋嬪皢鍙楀埌璐熼潰褰卞搷銆傛棤娉曡鏈嶅姟鐨勭瓑寰呬腑鐨?op 灏嗕粠璇锋眰鍒楄〃涓?
绉婚櫎锛屽苟灏嗗叾鐘舵€佽涓?鈥済iven up鈥濄€傛棤娉曡鏈嶅姟涓殑杩涜涓殑 op 灏嗕粠 in_progress 鍝堝笇琛ㄤ腑
绉婚櫎锛屽苟灏嗗叾鐘舵€佽涓?鈥済iven up鈥濄€?

readdir 鍜?I/O op 鍦ㄨ礋杞斤紙payload锛夋柟闈㈡槸涓嶅吀鍨嬬殑銆?

  - readdir op 浣跨敤涓や釜棰勫垎閰嶃€侀鍒嗗尯鐨勮緝灏忓唴瀛樼紦鍐插尯涔嬩竴銆俽eaddir 缂撳啿鍖哄彧鑳借鐢ㄦ埛绌洪棿
    浣跨敤銆傚唴鏍告ā鍧楀湪鍙戣捣 readdir op 涔嬪墠鑾峰彇涓€涓┖闂插垎鍖虹殑绱㈠紩銆傜敤鎴风┖闂村皢缁撴灉瀛樺叆璇?
    绱㈠紩鍒嗗尯锛岀劧鍚庡皢鍏跺啓鍥?pvfs 璁惧銆?

  - io锛堣鍜屽啓锛塷p 浣跨敤涓や釜棰勫垎閰嶃€侀鍒嗗尯鐨勮緝澶у唴瀛樼紦鍐插尯涔嬩竴銆侷O 缂撳啿鍖烘棦鍙粠鐢ㄦ埛绌洪棿
    涔熷彲浠庡唴鏍告ā鍧楄闂€傚唴鏍告ā鍧楀湪鍙戣捣 io op 涔嬪墠鑾峰彇涓€涓┖闂插垎鍖虹殑绱㈠紩銆傚唴鏍告ā鍧楀皢鍐?
    鏁版嵁瀛樺叆绱㈠紩鍒嗗尯锛岀洿鎺ヤ緵鐢ㄦ埛绌洪棿娑堣垂銆傜敤鎴风┖闂村皢璇昏姹傜殑缁撴灉瀛樺叆绱㈠紩鍒嗗尯锛岀洿鎺ヤ緵
    鍐呮牳妯″潡娑堣垂銆?

瀵瑰唴鏍歌姹傜殑鍝嶅簲閮借鎵撳寘鍦?pvfs2_downcall_t 缁撴瀯涓€傞櫎浜嗗皯鏁板嚑涓叾瀹冩垚鍛樺锛?
pvfs2_downcall_t 鍖呭惈涓€涓粨鏋勪綋鑱斿悎浣擄紙union锛夛紝鍏朵腑姣忎釜缁撴瀯浣撻兘涓庝竴绉嶇壒瀹氱殑鍝嶅簲绫诲瀷
鐩稿叧鑱斻€?

鑱斿悎浣撳闈㈢殑鍑犱釜鎴愬憳鏄細

 `int32_t type`
    - 鎿嶄綔绫诲瀷銆?
 `int32_t status`
    - 鎿嶄綔鐨勮繑鍥炵爜銆?
 `int64_t trailer_size`
    - 闄ら潪鏄?readdir 鎿嶄綔锛屽惁鍒欎负 0銆?
 `char *trailer_buf`
    - 鍒濆鍖栦负 NULL锛屽湪 readdir 鎿嶄綔鏈熼棿浣跨敤銆?

鑱斿悎浣撳唴閮ㄩ€傚綋鐨勬垚鍛樹細琚拡瀵逛换浣曠壒瀹氬搷搴旇€屽～鍏呫€?

  PVFS2_VFS_OP_FILE_IO
    fill a pvfs2_io_response_t

  PVFS2_VFS_OP_LOOKUP
    fill a PVFS_object_kref

  PVFS2_VFS_OP_CREATE
    fill a PVFS_object_kref

  PVFS2_VFS_OP_SYMLINK
    fill a PVFS_object_kref

  PVFS2_VFS_OP_GETATTR
    fill in a PVFS_sys_attr_s锛堝唴鏍镐笉闇€瑕佺殑澶ч噺鍐呭锛?
    褰撳璞℃槸绗﹀彿閾炬帴锛坰ymlink锛夋椂锛岀敤涓€涓寘鍚摼鎺ョ洰鏍囩殑瀛楃涓插～鍏呫€?

  PVFS2_VFS_OP_MKDIR
    fill a PVFS_object_kref

  PVFS2_VFS_OP_STATFS
    fill a pvfs2_statfs_response_t with useless info <g>銆傛垜浠緢闅惧強鏃跺湴鐭ラ亾
    鍏充簬鎴戜滑杩欎釜鍒嗗竷寮忕綉缁滄枃浠剁郴缁熺殑杩欎簺缁熻淇℃伅銆?

  PVFS2_VFS_OP_FS_MOUNT
    fill a pvfs2_fs_mount_response_t锛屽畠涓?PVFS_object_kref 绫讳技锛屽彧鏄叾鎴愬憳椤哄簭涓嶅悓锛?
    骞朵笖 鈥淿_pad1鈥?琚浛鎹负 鈥渋d鈥濄€?

  PVFS2_VFS_OP_GETXATTR
    fill a pvfs2_getxattr_response_t

  PVFS2_VFS_OP_LISTXATTR
    fill a pvfs2_listxattr_response_t

  PVFS2_VFS_OP_PARAM
    fill a pvfs2_param_response_t

  PVFS2_VFS_OP_PERF_COUNT
    fill a pvfs2_perf_count_response_t

  PVFS2_VFS_OP_FSKEY
    file a pvfs2_fs_key_response_t

  PVFS2_VFS_OP_READDIR
    jamb everything needed to represent a pvfs2_readdir_response_t into
    the readdir buffer descriptor specified in the upcall銆?

鐢ㄦ埛绌洪棿浣跨敤 writev() 鍦?/dev/pvfs2-req 涓婁紶閫掑鍐呮牳渚ф墍鍙戝嚭璇锋眰鐨勫搷搴斻€?

涓€涓?buffer_list 鍖呭惈锛?

  - 涓€涓寚鍚戝唴鏍歌姹傚搷搴旓紙struct pvfs2_downcall_t锛夌殑鎸囬拡銆?
  - 姝ゅ锛屽湪 readdir 璇锋眰鐨勬儏鍐典笅锛屼竴涓寚鍚戝寘鍚洰鏍囩洰褰曚腑瀵硅薄鎻忚堪绗︾殑缂撳啿鍖虹殑鎸囬拡銆?

... 琚彂閫佺粰鎵ц writev 鐨勫嚱鏁帮紙PINT_dev_write_list锛夈€?

PINT_dev_write_list 鏈変竴涓眬閮?iovec 鏁扮粍锛歴truct iovec io_array[^10^];

io_array 鐨勫墠鍥涗釜鍏冪礌瀵规墍鏈夊搷搴旈兘鍍忚繖鏍峰垵濮嬪寲锛?

```
  io_array[0].iov_base = address of local variable "proto_ver" (int32_t)
  io_array[0].iov_len = sizeof(int32_t)

  io_array[1].iov_base = address of global variable "pdev_magic" (int32_t)
  io_array[1].iov_len = sizeof(int32_t)

  io_array[2].iov_base = address of parameter "tag" (PVFS_id_gen_t)
  io_array[2].iov_len = sizeof(int64_t)

  io_array[3].iov_base = address of out_downcall member (pvfs2_downcall_t)
                         of global variable vfs_request (vfs_request_t)
  io_array[3].iov_len = sizeof(pvfs2_downcall_t)
```

```
  io_array[4].iov_base = contents of member trailer_buf (char *)
                         from out_downcall member of global variable
                         vfs_request
  io_array[4].iov_len = contents of member trailer_size (PVFS_size)
                        from out_downcall member of global variable
                        vfs_request
```

Orangefs 鍒╃敤 dcache 浠ラ伩鍏嶅悜鐢ㄦ埛绌洪棿鍙戦€佸啑浣欒姹傘€傛垜浠€氳繃 orangefs_inode_getattr
浣垮璞＄殑 inode 灞炴€т繚鎸佹渶鏂般€俹rangefs_inode_getattr 浣跨敤涓や釜鍙傛暟鏉ュ府鍔╁畠鍐冲畾鏄惁鏇存柊
涓€涓?inode锛氣€渘ew鈥?鍜?鈥渂ypass鈥濄€侽rangefs 鍦ㄥ璞＄殑 inode 涓繚瀛樼鏈夋暟鎹紝鍏朵腑鍖呮嫭涓€涓?
杈冪煭鐨勮秴鏃跺€?getattr_time锛屽畠浣?orangefs_inode_getattr 鐨勪换浣曚竴娆¤凯浠ｉ兘鑳界煡閬撹 inode
鑷笂娆℃洿鏂颁互鏉ョ粡杩囦簡澶氫箙銆傚綋瀵硅薄涓嶆槸鏂扮殑锛坣ew == 0锛変笖 bypass 鏍囧織鏈缃紙bypass == 0锛?
鏃讹紝濡傛灉 getattr_time 灏氭湭瓒呮椂锛宱rangefs_inode_getattr 浼氫笉缁忔洿鏂扮洿鎺ヨ繑鍥炪€俫etattr_time
鍦ㄦ瘡娆℃洿鏂?inode 鏃惰鍒锋柊銆?

鍒涘缓涓€涓柊瀵硅薄锛堟枃浠躲€佺洰褰曘€佺鍙烽摼鎺ワ級鍖呮嫭瀵瑰叾璺緞鍚嶇殑瑙ｆ瀽锛岀粨鏋滀负璇ュ璞＄殑涓€涓礋鐩綍椤?
锛坣egative directory entry锛夈€傚垎閰嶄竴涓柊鐨?inode 骞朵笌璇?dentry 鍏宠仈锛屽皢鍏朵粠涓€涓礋 dentry
鍙樻垚 鈥滃绀句細鏈夎础鐚殑姝ｅ紡涓€鍛樷€濄€侽rangefs 閫氳繃 new_inode() 浠?Linux 鑾峰彇鏂扮殑 inode锛屽苟閫氳繃
鐢?d_instantiate() 灏嗚瀵癸紙inode 鍜?dentry锛夐€佸洖 Linux 鏉ュ皢 inode 涓?dentry 鍏宠仈銆?

瀵瑰璞¤矾寰勫悕鐨勮В鏋愪細瀵瑰簲鍒板叾 dentry銆傚鏋滄病鏈夊搴旂殑 dentry锛屽垯鍦?dcache 涓负瀹冨垱寤轰竴涓€?
姣忓綋涓€涓?dentry 琚慨鏀规垨楠岃瘉鏃讹紝Orangefs 浼氬湪璇?dentry 鐨?d_time 涓瓨鍌ㄤ竴涓緝鐭殑瓒呮椂鍊硷紝
鍦ㄨ娈垫椂闂村収璇?dentry 浼氳淇′换銆侽rangefs 鏄竴涓綉缁滄枃浠剁郴缁燂紝瀵硅薄鏈夊彲鑳藉湪甯﹀锛坥ut-of-band锛?
琚换浣曠壒瀹氱殑 Orangefs 鍐呮牳妯″潡瀹炰緥鏀瑰彉锛屽洜姝や俊浠?dentry 鏄湁椋庨櫓鐨勩€備俊浠?dentry 鐨勬浛浠?
鏂规鏄€绘槸浠庣敤鎴风┖闂磋幏鍙栨墍闇€淇℃伅鈥斺€旇嚦灏戞槸涓€娆″埌 client-core 鐨勫線杩旓紝鎴栬杩樿鍒版湇鍔″櫒銆?
浠?dentry 鑾峰彇淇℃伅寰堜究瀹滐紝鑰屼粠鐢ㄦ埛绌洪棿鑾峰彇淇℃伅鐩稿鏄傝吹锛岃繖灏辨槸灏藉彲鑳戒娇鐢?dentry 鐨勫姩鏈恒€?

瓒呮椂鍊?d_time 鍜?getattr_time 鏄熀浜?jiffy 鐨勶紝骞朵笖锛?

```
    "涓€鑸€岃█锛屽鏋滄椂閽熷彲鑳藉凡缁忓洖缁曪紙wrap around锛夎秴杩囦竴娆★紝灏辨棤娉曞垽鏂凡缁忚繃鍘讳簡澶氬皯
    鏃堕棿銆傜劧鑰岋紝濡傛灉宸茬煡鏃堕棿 t1 鍜?t2 鐩稿綋鎺ヨ繎锛屾垜浠氨鍙互浠ヤ竴绉嶈€冭檻鍒版椂閽熷彲鑳藉湪涓ゆ
    鏃堕棿涔嬮棿鍙戠敓杩囧洖缁曠殑鍙兘鎬х殑鏂瑰紡锛屽彲闈犲湴璁＄畻鍑哄樊鍊笺€?
```

锛堝紩鑷?Andy Wang 璁插笀鐨勮绋嬬瑪璁帮級
