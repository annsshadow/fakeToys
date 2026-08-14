
## NFSD IO 妯″紡


## 姒傝堪


NFSD 鍦ㄥ鐞?READ 鍜?WRITE 鎿嶄綔鏃讹紝鍘嗗彶涓婁竴鐩翠娇鐢ㄧ紦鍐?IO銆侭UFFERED 鏄?NFSD 鐨勯粯璁?IO 妯″紡锛屼絾鍙互灏嗚榛樿鍊艰鐩栦负浣跨敤 DONTCACHE 鎴?DIRECT IO 妯″紡銆?
鎻愪緵浜嗗疄楠屾€х殑 NFSD debugfs 鎺ュ彛锛屽厑璁哥嫭绔嬮厤缃敤浜?READ 鍜?WRITE 鐨?NFSD IO 妯″紡銆?璇峰弬瑙侊細

- /sys/kernel/debug/nfsd/io_cache_read
- /sys/kernel/debug/nfsd/io_cache_write

io_cache_read 鍜?io_cache_write 鐨勯粯璁ゅ€煎弽鏄犱簡 NFSD 鐨勯粯璁?IO 妯″紡锛堝嵆
NFSD_IO_BUFFERED=0锛夈€?
鏍规嵁閰嶇疆鐨勮缃紝NFSD 鐨?IO 灏嗕负浠ヤ笅涔嬩竴锛?
- 浣跨敤椤电紦瀛樼紦瀛橈紙NFSD_IO_BUFFERED=0锛?- 缂撳瓨浣嗗湪瀹屾垚鏃朵粠椤电紦瀛樼Щ闄わ紙NFSD_IO_DONTCACHE=1锛?- 涓嶇紦瀛?stable_how=NFS_UNSTABLE锛圢FSD_IO_DIRECT=2锛?
瑕佽缃?NFSD IO 妯″紡锛屽悜
```

  echo 2 > /sys/kernel/debug/nfsd/io_cache_read
  echo 2 > /sys/kernel/debug/nfsd/io_cache_write

```
瑕佹鏌?NFSD 瀵?READ 鎴?WRITE 姝ｅ湪浣跨敤鍝釜 IO 妯″紡锛屽彧闇€璇诲彇
```

  cat /sys/kernel/debug/nfsd/io_cache_read
  cat /sys/kernel/debug/nfsd/io_cache_write

```
濡傛灉浣犲湪杩戞湡鍐呮牳涓婅瘯楠?NFSD 鐨?IO 妯″紡骞跺緱鍒颁簡鏈夎叮鐨勭粨鏋滐紝璇峰皢鍏舵姤鍛婂埌
linux-nfs@vger.kernel.org

## NFSD DONTCACHE


DONTCACHE 鎻愪緵浜嗕竴绉嶅鐞?IO 鐨勬贩鍚堟柟娉曪紝鏃ㄥ湪鎻愪緵浣跨敤 DIRECT IO 鐨勫ソ澶勶紝鑰屼笉甯︽潵
DIRECT IO 鎵€鏂藉姞鐨勪换浣曚弗鏍煎榻愯姹傘€備负姝わ紝瀹冧娇鐢ㄧ紦鍐?IO锛屼絾 IO 琚爣璁颁负鈥滆惤鍚庝涪寮冣€?锛堝嵆鐩稿叧鑱旂殑椤靛湪 IO 瀹屾垚鏃朵粠椤电紦瀛樹腑涓㈠純锛夈€?
DONTCACHE 鏃ㄥ湪閬垮厤 Linux 鍐呭瓨绠＄悊瀛愮郴缁熷凡琚瘉鏄庣浉褰撴樉钁楃殑涓€涓檺鍒讹細褰?濡傛灉澶ч噺
鏁版嵁琚笉棰戠箒璁块棶鏃讹紙渚嬪鍙鍙栦竴娆鎴朹鍙啓鍏ヤ竴娆°€佷絾寰堜箙涔嬪悗鎵嶈鍙栵級銆傝繖绫荤敤渚?灏ゅ叾鎴愰棶棰橈紝鍥犱负椤电紦瀛樻渶缁堜細鎴愪负鏈嶅姟鏂?IO 璇锋眰鐨勭摱棰堛€?
鍏充簬 DONTCACHE 鐨勬洿澶氳儗鏅紝璇峰弬闃呰繖浜?Linux 鎻愪氦璇存槑锛?
- 姒傝堪锛? 9ad6344568cc3 ("mm/filemap: change filemap_create_folio()
  to take a struct kiocb")
- 鐢ㄤ簬 READ锛? 8026e49bff9b1 ("mm/filemap: add read support for
  RWF_DONTCACHE")
- 鐢ㄤ簬 WRITE锛?974c5e6139db3 ("xfs: flag as supporting FOP_DONTCACHE")

濡傛灉搴曞眰鏂囦欢绯荤粺娌℃湁閫氳繃璁剧疆 FOP_DONTCACHE 鏉ヨ〃鏄庢敮鎸侊紝NFSD_IO_DONTCACHE 灏嗗洖閫€鍒?NFSD_IO_BUFFERED銆?
## NFSD DIRECT


DIRECT IO 涓嶄娇鐢ㄩ〉缂撳瓨锛屽洜姝ゅ畠鑳藉閬垮厤 Linux 鍐呭瓨绠＄悊鐨勯〉鍥炴敹锛坧age reclaim锛夊彲鎵╁睍鎬?闂锛岃€屾棤闇€鍍?DONTCACHE 閭ｆ牱娣峰悎浣跨敤椤电紦瀛樸€?
涓€浜涘伐浣滆礋杞藉彈鐩婁簬 NFSD 閬垮紑椤电紦瀛橈紝鐗瑰埆鏄偅浜涘伐浣滈泦鏄捐憲澶т簬鍙敤绯荤粺鍐呭瓨鐨勮礋杞姐€?NFSD DIRECT 琚瘉鏄庡府鍔╂渶澶х殑鐥呮€佹渶鍧忔儏鍐靛伐浣滆礋杞芥槸锛歂FS 瀹㈡埛绔涓€涓ぇ灏忎负 NFS 鏈嶅姟鍣?鍙敤绯荤粺鍐呭瓨 2-3 鍊嶇殑鏂囦欢鍙戣捣澶у瀷椤哄簭 IO銆傝繖绉嶆敼杩涚殑鍘熷洜鍦ㄤ簬 NFSD DIRECT 娑堥櫎浜嗗唴瀛?绠＄悊瀛愮郴缁熷師鏈渶瑕佹墽琛岀殑璁稿宸ヤ綔锛堜緥濡傞〉鍒嗛厤銆佽剰椤靛洖鍐欍€侀〉鍥炴敹锛夈€備娇鐢?NFSD DIRECT
鏃讹紝kswapd 鍜?kcompactd 涓嶅啀鍗犳嵁 CPU 鏃堕棿鍘诲鎵捐冻澶熺殑绌洪棽椤典互鎺ㄨ繘 IO銆?
涓?NFSD DIRECT 鐩稿叧鐨勬€ц兘鎻愬崌姝ゅ墠鍦?linux-nfs 涓婅璁鸿繃锛屽弬瑙侊細
https://lore.kernel.org/linux-nfs/aEslwqa9iMeZjjlV@kernel.org/

鎬荤粨濡備笅锛?
- NFSD DIRECT 鍙互鏄捐憲鍑忓皯鍐呭瓨闇€姹?- NFSD DIRECT 鍙互閫氳繃閬垮厤浠ｄ环楂樻槀鐨勯〉鍥炴敹宸ヤ綔鏉ラ檷浣?CPU 璐熻浇
- NFSD DIRECT 鍙互鎻愪緵鏇村叿纭畾鎬х殑 IO 鎬ц兘

涓€濡傛棦寰€锛屾晥鏋滃洜浜鸿€屽紓锛屽洜姝や粩缁嗚€冭檻鏄惁/浣曟椂浣跨敤 NFSD DIRECT 鏈夌泭寰堥噸瑕併€傚湪璇勪及浣犵殑
宸ヤ綔璐熻浇鐨勭浉瀵规€ц兘鏃讹紝璇峰姟蹇呭湪娴嬭瘯鏈熼棿璁板綍鐩稿叧鐨勬€ц兘鎸囨爣锛堜緥濡傚唴瀛樹娇鐢ㄣ€丆PU 浣跨敤銆?IO 鎬ц兘锛夈€備娇鐢?perf 鏀堕泦 perf 鏁版嵁锛岀敤浜庣敓鎴?Linux 涓轰綘鐨勬祴璇曟墍蹇呴』鎵ц鐨勫伐浣滅殑
鈥滅伀鐒板浘鈥濓紝鏄竴绉嶇湡姝ｆ湁鎰忎箟鐨勬柟寮忔潵姣旇緝绯荤粺鐨勭浉瀵瑰仴搴风姸鍐碉紝浠ュ強鍒囨崲 NFSD 鐨?IO 妯″紡
濡備綍鏀瑰彉鎵€瑙傚療鍒扮殑鎯呭喌銆?
濡傛灉閫氳繃鍚?NFSD 鐨?debugfs 鎺ュ彛鍐欏叆 2锛堟垨鐢ㄤ簬 WRITE 鐨?3 鍜?4锛夋潵鎸囧畾 NFSD_IO_DIRECT锛?鐞嗘兂鎯呭喌涓?IO 搴旂浉瀵逛簬搴曞眰鍧楄澶囩殑 logical_block_size 瀵归綈銆傛澶栵紝鐢ㄤ簬瀛樺偍 READ 鎴?WRITE 杞借嵎鐨勫唴瀛樼紦鍐插尯蹇呴』鐩稿浜庡簳灞傚潡璁惧鐨?dma_alignment 瀵归綈銆?
浣?NFSD DIRECT 鍦?O_DIRECT 鐨勬剰涔変笂浼氬敖鏈€澶у姫鍔涘鐞嗘湭瀵归綈鐨?IO锛?
鏈榻愮殑 READ锛?    濡傛灉浣跨敤 NFSD_IO_DIRECT锛屽皢浠讳綍鏈榻愮殑 READ 鎵╁睍鍒颁笅涓€涓?DIO 瀵归綈鐨勫潡锛堝湪 READ
    鐨勪袱绔級銆傛墿灞曞悗鐨?READ 浼氭牎楠屽叿鏈夋纭殑 offset/len锛坙ogical_block_size锛変互鍙?    dma_alignment 妫€鏌ャ€?
鏈榻愮殑 WRITE锛?    濡傛灉浣跨敤 NFSD_IO_DIRECT锛屾寜闇€灏嗕换浣曟湭瀵归綈鐨?WRITE 鎷嗗垎涓鸿捣濮嬨€佷腑闂村拰缁撳熬銆傝緝澶х殑
    涓棿娈垫槸 DIO 瀵归綈鐨勶紝鑰岃捣濮嬪拰/鎴栫粨灏炬槸鏈榻愮殑銆傚鏈榻愮殑娈典娇鐢ㄧ紦鍐?IO锛屽涓棿
    DIO 瀵归綈鐨勬浣跨敤 O_DIRECT銆傛湭瀵归綈鐨勬_涓峗浣跨敤 DONTCACHE 缂撳啿 IO锛屽洜涓轰娇鐢ㄦ櫘閫氱紦鍐?    IO 鍦ㄥ鐞嗘祦寮忔湭瀵归綈 WRITE 鏃跺叿鏈夋樉钁楃殑 RMW 鎬ц兘浼樺娍銆?
璺熻釜锛?    nfsd_read_direct 璺熻釜浜嬩欢灞曠ず浜?NFSD 濡備綍灏嗕换浣曟湭瀵归綈鐨?READ 鎵╁睍鍒颁笅涓€涓?DIO 瀵归綈
    鐨勫潡锛堝湪鍘熷 READ 鐨勪袱绔紝鎸夐渶锛夈€?
```

      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_vector/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_direct/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_io_done/enable
      echo 1 > /sys/kernel/tracing/events/xfs/xfs_file_direct_read/enable

    nfsd_write_direct 璺熻釜浜嬩欢灞曠ず浜?NFSD 濡備綍灏嗙粰瀹氭湭瀵归綈鐨?WRITE 鎷嗗垎涓轰竴涓?DIO 瀵归綈
    鐨勪腑闂存銆?
    杩欎竴缁勫悎璺熻釜浜嬩欢瀵?WRITE 寰堟湁鐢?:

      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_opened/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_direct/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_io_done/enable
      echo 1 > /sys/kernel/tracing/events/xfs/xfs_file_direct_write/enable

```
