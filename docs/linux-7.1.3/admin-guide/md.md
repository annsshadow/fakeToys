## RAID 闃靛垪


### 鍚姩鏃剁粍瑁?RAID 闃靛垪


绠＄悊 md 璁惧鐨勫伐鍏峰彲鍦ㄤ互涓嬩綅缃壘鍒帮細
   https://www.kernel.org/pub/linux/utils/raid/


浣犲彲浠ヤ娇鐢ㄤ互涓嬪唴鏍稿懡浠よ鍙傛暟鏉ョ敤浣犵殑 md 璁惧鍚姩锛?

```
  md=<md device no.>,<raid level>,<chunk size factor>,<fault level>,dev0,dev1,...,devn

```
```
  md=<md device no.>,dev0,dev1,...,devn

```
```
  md=d<md device no.>,dev0,dev1,...,devn

```
`md device no.`
+++++++++++++++++

md 璁惧鐨勭紪鍙?

================= =========
`md device no.` device
================= =========
              0		md0
	      1		md1
	      2		md2
	      3		md3
	      4		md4
================= =========

`raid level`
++++++++++++++

RAID 闃靛垪鐨勭骇鍒?

=============== =============
`raid level`  level
=============== =============
-1		linear mode
0		striped mode
=============== =============

鍏朵粬妯″紡浠呭湪鍏锋湁鎸佷箙瓒呯骇鍧楋紙persistent super blocks锛夋椂鎵嶅彈鏀寔

`chunk size factor`
+++++++++++++++++++++

锛堜粎 raid-0 鍜?raid-1锛?

灏嗗潡澶у皬璁句负 4k << n銆?

`fault level`
+++++++++++++++

瀹屽叏琚拷鐣?

`dev0` to `devn`
++++++++++++++++++++

e.g. `/dev/hda1`, `/dev/hdc1`, `/dev/sda1`, `/dev/sdb1`

```
	e:\loadlin\loadlin e:\zimage root=/dev/md0 md=0,0,4,0,/dev/hdb2,/dev/hdc3 ro

```
### 鍚姩鏃惰嚜鍔ㄦ帰娴?RAID 闃靛垪


褰?md 琚紪璇戣繘鍐呮牳锛堣€岄潪浣滀负妯″潡锛夋椂锛岀被鍨嬩负 0xfd 鐨勫垎鍖轰細琚壂鎻忓苟鑷姩缁勮涓?RAID 闃靛垪銆傝繖绉嶈嚜鍔ㄦ帰娴嬪彲浠ラ€氳繃鍐呮牳鍙傛暟 `raid=noautodetect` 鏉ョ姝€傝嚜鍐呮牳 2.6.9 璧凤紝鍙湁甯︽湁 0 绫诲瀷瓒呯骇鍧楃殑椹卞姩鍣ㄦ墠鑳借鑷姩鎺㈡祴骞跺湪鍚姩鏃惰繍琛屻€?

鍐呮牳鍙傛暟 `raid=partitionable`锛堟垨 `raid=part`锛夋剰鍛崇潃鎵€鏈夎嚜鍔ㄦ帰娴嬪埌鐨勯樀鍒楅兘琚粍瑁呬负鍙垎鍖猴紙partitionable锛夌殑褰㈠紡銆?

### 鍚姩鏃剁粍瑁呴檷绾?鑴忛樀鍒?


濡傛灉涓€涓?raid5 鎴?raid6 闃靛垪鏃㈠浜庤剰锛坉irty锛夊張澶勪簬闄嶇骇锛坉egraded锛夌姸鎬侊紝瀹冨彲鑳戒細鏈夋棤娉曟娴嬪埌鐨勬暟鎹崯鍧忋€傝繖鏄洜涓哄畠澶勪簬 `dirty` 鐘舵€佹剰鍛崇潃濂囧伓鏍￠獙涓嶅彲淇★紝鑰屽畠澶勪簬闄嶇骇鐘舵€佹剰鍛崇潃鏌愪簺鏁版嵁鍧楃己澶变笖鏃犳硶鍙潬鍦伴噸寤猴紙鍥犱负娌℃湁濂囧伓鏍￠獙锛夈€?

鍑轰簬杩欎釜鍘熷洜锛宮d 閫氬父浼氭嫆缁濆惎鍔ㄨ繖鏍风殑闃靛垪銆傝繖闇€瑕佺郴缁熺鐞嗗憳閲囧彇琛屽姩鏉ユ樉寮忓惎鍔ㄨ闃靛垪銆?

```
   mdadm --assemble --force ....

```
濡傛灉闃靛垪涓婃湁鏍规枃浠剁郴缁燂紝杩欎釜閫夐」瀹為檯涓婁笉鍙敤銆備负浜嗘敮鎸佷粠杩欐牱鐨勯樀鍒楀惎鍔紝md 鏀寔涓€涓ā鍧楀弬鏁?`start_dirty_degraded`锛屽綋灏嗗叾璁句负 1 鏃讹紝浼氱粫杩囪繖浜涙鏌ュ苟鍏佽鍚姩鑴忕殑闄嶇骇闃靛垪銆?

```
   md-mod.start_dirty_degraded=1

```
### 瓒呯骇鍧楁牸寮?


md 椹卞姩鍙互鏀寔澶氱涓嶅悓鐨勮秴绾у潡鏍煎紡銆傜洰鍓嶏紝瀹冩敮鎸?`0.90.0` 瓒呯骇鍧楁牸寮忎互鍙婂湪鍐呮牳 2.5 寮€鍙戠郴鍒椾腑寮曞叆鐨?`md-1` 鏍煎紡銆?

鍐呮牳浼氳嚜鍔ㄦ帰娴嬫鍦ㄤ娇鐢ㄧ殑鏄摢绉嶆牸寮忕殑瓒呯骇鍧椼€?

鍑轰簬鍏煎鎬у師鍥狅紝瓒呯骇鍧楁牸寮?`0` 鐨勫鐞嗘柟寮忎笌鍏朵粬鏍煎紡涓嶅悓鈥斺€斿畠鏄師濮嬬殑瓒呯骇鍧楁牸寮忋€?


### 閫氱敤瑙勫垯鈥斺€旈€傜敤浜庢墍鏈夎秴绾у潡鏍煎紡


闃靛垪鏄€氳繃鍚戞墍鏈夎澶囧啓鍏ラ€傚綋鐨勮秴绾у潡鏉?`created`锛堝垱寤猴級鐨勩€?

瀹冩槸閫氳繃灏嗘瘡涓澶囦笌鐗瑰畾鐨?md 铏氭嫙璁惧鍏宠仈璧锋潵鑰?`assembled`锛堢粍瑁咃級鐨勩€備竴鏃﹀畬鍏ㄧ粍瑁呭畬鎴愶紝瀹冨氨鍙互琚闂€?

闃靛垪搴旂敱鐢ㄦ埛绌洪棿宸ュ叿鍒涘缓銆傝繖浼氬皢瓒呯骇鍧楀啓鍏ユ墍鏈夎澶囥€傚畠閫氬父浼氬皢闃靛垪鏍囪涓?`unclean`锛堜笉骞插噣锛夛紝鎴栬€呮爣璁版煇浜涜澶囩己澶憋紝浠ヤ究鍐呮牳 md 椹卞姩鍙互鍒涘缓閫傚綋鐨勫啑浣欙紙鍦?raid 1 涓鍒讹紝鍦?raid 4/5 涓绠楀鍋舵牎楠岋級銆?

褰撲竴涓樀鍒楄缁勮鏃讹紝棣栧厛浣跨敤 SET_ARRAY_INFO ioctl 杩涜鍒濆鍖栥€傚畠鐗瑰埆鍖呭惈涓荤増鏈彿鍜屾鐗堟湰鍙枫€備富鐗堟湰鍙烽€夋嫨瑕佷娇鐢ㄧ殑瓒呯骇鍧楁牸寮忋€傛鐗堟湰鍙峰彲鑳界敤浜庤皟鏁磋鏍煎紡鐨勫鐞嗘柟寮忥紝渚嬪寤鸿鍦ㄦ瘡涓澶囦笂浣曞鏌ユ壘瓒呯骇鍧椼€?

鐒跺悗锛屼娇鐢?ADD_NEW_DISK ioctl 娣诲姞姣忎釜璁惧銆傚畠鐗瑰埆鎻愪緵鏍囪瘑瑕佹坊鍔犺澶囩殑涓汇€佹璁惧鍙枫€?

璇ラ樀鍒楅€氳繃 RUN_ARRAY ioctl 鍚姩銆?

鍚姩鍚庯紝鍙互娣诲姞鏂拌澶囥€傚簲鍏堝悜瀹冧滑鍐欏叆閫傚綋鐨勮秴绾у潡锛岀劧鍚庨€氳繃 ADD_NEW_DISK 浼犲叆銆?

宸插け璐ユ垨灏氭湭婵€娲荤殑璁惧鍙互浣跨敤 HOT_REMOVE_DISK 浠庨樀鍒椾腑鍒嗙銆?


### 閫傜敤浜?format-0 瓒呯骇鍧楅樀鍒椾互鍙婃棤瓒呯骇鍧楋紙闈炴寔涔咃級闃靛垪鐨勭壒瀹氳鍒?


鍙互閫氳繃鍦?SET_ARRAY_INFO ioctl 涓弿杩伴樀鍒楋紙绾у埆銆佸潡澶у皬绛夛級鏉?`created`锛堝垱寤猴級涓€涓樀鍒椼€傝繖蹇呴』鍏锋湁 `major_version==0` 涓?`raid_disks != 0`銆?

鐒跺悗锛屽彲浠ヤ娇鐢?ADD_NEW_DISK 娣诲姞鏈垵濮嬪寲鐨勮澶囥€備紶缁?ADD_NEW_DISK 鐨勭粨鏋勫繀椤绘寚瀹氳澶囩殑鐘舵€佸強鍏跺湪闃靛垪涓殑瑙掕壊銆?

涓€鏃﹂€氳繃 RUN_ARRAY 鍚姩锛屽氨鍙互浣跨敤 HOT_ADD_DISK 娣诲姞鏈垵濮嬪寲鐨勭儹澶囩洏銆?


### sysfs 涓殑 MD 璁惧


md 璁惧浣滀负甯歌鍧楄澶囧嚭鐜板湪 sysfs锛坄/sys`锛変腑锛?

```
   /sys/block/md0

```
姣忎釜 `md` 璁惧閮藉寘鍚竴涓悕涓?`md` 鐨勫瓙鐩綍锛屽叾涓瓨鏀剧潃鍏充簬璇ヨ澶囩殑鏇村 md 鐗瑰畾淇℃伅銆?

鎵€鏈?md 璁惧閮藉寘鍚細

  level
     涓€涓枃鏈枃浠讹紝鎸囩ず `raid level`锛圧AID 绾у埆锛夛紝渚嬪 raid0銆乺aid1銆?
     raid5銆乴inear銆乵ultipath銆乫aulty銆?
     濡傛灉灏氭湭璁剧疆 RAID 绾у埆锛堥樀鍒椾粛鍦ㄧ粍瑁呬腑锛夛紝璇ュ€煎皢鍙嶆槧宸插啓鍏?
     鐨勫唴瀹癸紝鍙兘鏄笂杩板悕绉颁箣涓€锛屼篃鍙兘鏄濡?`0`銆乣5` 绛夋暟瀛椼€?

  raid_disks
     涓€涓寘鍚畝鍗曟暟瀛楃殑鏂囨湰鏂囦欢锛屾寚绀轰竴涓姛鑳藉畬鏁寸殑闃靛垪涓殑
     璁惧鏁伴噺銆傚鏋滃皻鏈彲鐭ワ紝璇ユ枃浠朵负绌恒€傚鏋滈樀鍒楁鍦ㄨ皟鏁村ぇ灏忥紝
     瀹冨皢鍖呭惈鏂扮殑璁惧鏁伴噺銆?
     鏌愪簺 RAID 绾у埆鍏佽鍦ㄩ樀鍒楀浜庢椿鍔ㄧ姸鎬佹椂璁剧疆姝ゅ€笺€傝繖浼氶噸鏂伴厤缃?
     闃靛垪銆傚惁鍒欙紝鍙兘鍦ㄧ粍瑁呴樀鍒楁椂璁剧疆銆?
     濡傛灉鏀瑰彉姝ゅ睘鎬т細缂╁皬闃靛垪鐨勫ぇ灏忥紝鍒欎笉鍏佽鏇存敼銆傝鍑忓皯
     渚嬪 raid5 涓殑椹卞姩鍣ㄦ暟閲忥紝蹇呴』棣栧厛閫氳繃璁剧疆 `array_size`
     灞炴€ф潵缂╁皬闃靛垪澶у皬銆?

  chunk_size
     杩欐槸 `chunks`锛堝潡锛夌殑瀛楄妭澶у皬锛屼粎涓庢秹鍙婃潯甯﹀寲锛坰triping锛夌殑
     RAID 绾у埆锛?銆?銆?銆?銆?0锛夌浉鍏炽€傞樀鍒楃殑鍦板潃绌洪棿鍦ㄦ蹇典笂琚?
     鍒掑垎涓哄潡锛岃繛缁殑鍧楄鏉″甫鍖栧埌鐩搁偦鐨勮澶囦笂銆?
     璇ュぇ灏忓簲鑷冲皯涓?PAGE_SIZE锛?k锛夛紝骞朵笖搴斾负 2 鐨勫箓銆?
     杩欏彧鑳藉湪缁勮闃靛垪鏃惰缃€?

  layout
     鐗瑰畾绾у埆涓嬮樀鍒楃殑 `layout`锛堝竷灞€锛夈€傝繖鍙槸涓€涓暟瀛楋紝鐢变笉鍚岀殑
     绾у埆浠ヤ笉鍚屾柟寮忚В閲娿€傚畠鍙互鍦ㄧ粍瑁呴樀鍒楁椂鍐欏叆銆?

  array_size
     杩欏彲鐢ㄤ簬浜轰负鍦板皢闃靛垪涓彲鐢ㄧ殑绌洪棿闄愬埗涓哄皬浜庡悎骞惰澶囦笂瀹為檯
     鍙敤鐨勭┖闂淬€傚啓鍏ヤ竴涓皬浜庡彲鐢ㄥぇ灏忕殑鏁板瓧锛堝崟浣嶅崈瀛楄妭锛夊皢璁剧疆
     璇ュぇ灏忋€傚闃靛垪鐨勪换浣曢噸鏂伴厤缃紙渚嬪娣诲姞璁惧锛夐兘涓嶄細瀵艰嚧澶у皬
     鏀瑰彉銆傚啓鍏ュ崟璇?`default` 浼氫娇闃靛垪鐨勬湁鏁堝ぇ灏忓彉涓哄熀浜?`level`銆?
     `chunk_size` 鍜?`component_size` 瀹為檯鍙敤鐨勪换鎰忓ぇ灏忋€?

     杩欏彲鐢ㄤ簬鍦ㄥ噺灏?raid4/5/6 涓澶囨暟閲忎箣鍓嶅厛缂╁皬闃靛垪澶у皬锛?
     鎴栫敤浜庢敮鎸佽姹傛绫昏鍓殑澶栭儴鍏冩暟鎹牸寮忋€?

  logical_block_size
     閰嶇疆闃靛垪鐨勯€昏緫鍧楀ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€傛灞炴€т粎鏀寔 1.x 鍏冩暟鎹€?
     鍦ㄥ惎鍔ㄩ樀鍒椾箣鍓嶅啓鍏ヨ鍊笺€傛渶缁堥樀鍒楃殑 LBS 鍙栨閰嶇疆涓庢墍鏈夊悎骞惰澶?
     LBS 涔嬮棿鐨勬渶澶у€笺€傛敞鎰忥紝鍦?RAID 鏀寔 folio 涔嬪墠锛孡BS 涓嶈兘瓒呰繃
     PAGE_SIZE銆?
     璀﹀憡锛氬湪鏂板唴鏍镐笂鍒涘缓鐨勯樀鍒楃敱浜庡～鍏呮鏌ユ棤娉曞湪鏃у唴鏍镐笂缁勮锛?
     鍙皢妯″潡鍙傛暟 'check_new_feature' 璁句负 false 鏉ョ粫杩囷紝浣嗗彲鑳戒細
     瀵艰嚧鏁版嵁涓㈠け銆?

  reshape_position
     杩欐槸 `none`锛屾垨鑰呮槸闃靛垪璁惧鍐?`reshape` 宸茶繘琛屽埌鐨勬墖鍖哄彿銆?
     濡傛灉璁剧疆浜嗘椤癸紝涓婅堪涓変釜灞炴€э紙raid_disks銆乧hunk_size銆乴ayout锛?
     鍙兘鍏锋湁涓や釜鍊硷紝鍗虫棫鍊煎拰鏂板€笺€傚鏋滃畠浠?

```
        new (old)

     and writing will effect the ``new`` value, leaving the ``old``
     unchanged.

  component_size
     For arrays with data redundancy (i.e. not raid0, linear, faulty,
     multipath), all components must be the same size - or at least
     there must a size that they all provide space for.  This is a key
     part or the geometry of the array.  It is measured in sectors
     and can be read from here.  Writing to this value may resize
     the array if the personality supports it (raid1, raid5, raid6),
     and if the component drives are large enough.

  metadata_version
     This indicates the format that is being used to record metadata
     about the array.  It can be 0.90 (traditional format), 1.0, 1.1,
     1.2 (newer format in varying locations) or ``none`` indicating that
     the kernel isn't managing metadata at all.
     Alternately it can be ``external:`` followed by a string which
     is set by user-space.  This indicates that metadata is managed
     by a user-space program.  Any device failure or other event that
     requires a metadata update will cause array activity to be
     suspended until the event is acknowledged.

  resync_start
     The point at which resync should start.  If no resync is needed,
     this will be a very large number (or ``none`` since 2.6.30-rc1).  At
     array creation it will default to 0, though starting the array as
     ``clean`` will set it much larger.

  new_dev
     This file can be written but not read.  The value written should
     be a block device number as major:minor.  e.g. 8:0
     This will cause that device to be attached to the array, if it is
     available.  It will then appear at md/dev-XXX (depending on the
     name of the device) and further configuration is then possible.

  safe_mode_delay
     When an md array has seen no write requests for a certain period
     of time, it will be marked as ``clean``.  When another write
     request arrives, the array is marked as ``dirty`` before the write
     commences.  This is known as ``safe_mode``.
     The ``certain period`` is controlled by this file which stores the
     period as a number of seconds.  The default is 200msec (0.200).
     Writing a value of 0 disables safemode.

  array_state
     This file contains a single word which describes the current
     state of the array.  In many cases, the state can be set by
     writing the word for the desired state, however some states
     cannot be explicitly set, and some transitions are not allowed.

     Select/poll works on this file.  All changes except between
     Active_idle and active (which can be frequent and are not
     very interesting) are notified.  active->active_idle is
     reported if the metadata is externally managed.

     clear
         No devices, no size, no level

         Writing is equivalent to STOP_ARRAY ioctl

     inactive
         May have some settings, but array is not active
         all IO results in error

         When written, doesn't tear down array, but just stops it

     suspended (not supported yet)
         All IO requests will block. The array can be reconfigured.

         Writing this, if accepted, will block until array is quiescent

     readonly
         no resync can happen.  no superblocks get written.

         Write requests fail

     read-auto
         like readonly, but behaves like ``clean`` on a write request.

     clean
         no pending writes, but otherwise active.

         When written to inactive array, starts without resync

         If a write request arrives then
         if metadata is known, mark ``dirty`` and switch to ``active``.
         if not known, block and switch to write-pending

         If written to an active array that has pending writes, then fails.
     active
         fully active: IO and resync can be happening.
         When written to inactive array, starts with resync

     write-pending
         clean, but writes are blocked waiting for ``active`` to be written.

     active-idle
         like active, but no writes have been seen for a while (safe_mode_delay).

  consistency_policy
     This indicates how the array maintains consistency in case of unexpected
     shutdown. It can be:

     none
       Array has no redundancy information, e.g. raid0, linear.

     resync
       Full resync is performed and all redundancy is regenerated when the
       array is started after unclean shutdown.

     bitmap
       Resync assisted by a write-intent bitmap.

     journal
       For raid4/5/6, journal device is used to log transactions and replay
       after unclean shutdown.

     ppl
       For raid5 only, Partial Parity Log is used to close the write hole and
       eliminate resync.

     The accepted values when writing to this file are ``ppl`` and ``resync``,
     used to enable and disable PPL.

  uuid
     This indicates the UUID of the array in the following format:
     xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx

  bitmap_type
     [RW] When read, this file will display the current and available
     bitmap for this array. The currently active bitmap will be enclosed
     in [] brackets. Writing an bitmap name or ID to this file will switch
     control of this array to that new bitmap. Note that writing a new
     bitmap for created array is forbidden.

```
濡傛灉 bitmap_type 涓嶄负 none锛屽垯鍦?md 璁惧 KOBJ_CHANGE 浜嬩欢涔嬪悗浼氬垱寤洪澶栫殑 bitmap 灞炴€?bitmap/xxx 鎴?llbitmap/xxx銆?

濡傛灉 bitmap_type 涓?bitmap锛屽垯 md 璁惧杩樺皢鍖呭惈锛?

  bitmap/location
     杩欐寚绀洪樀鍒楃殑鍐欐剰鍥句綅鍥撅紙write-intent bitmap锛夊瓨鍌ㄥ湪鍝噷銆?

     瀹冨彲浠ユ槸 `none`銆乣file` 鎴?`[+-]N` 涔嬩竴銆?
     `file` 浠ュ悗鍙兘鎵╁睍涓?`file:/file/name`銆?
     `[+-]N` 琛ㄧず浠庡厓鏁版嵁璧峰澶勮捣閭ｄ箞澶氭墖鍖恒€?

     杩欎細鍦ㄦ墍鏈夎澶囦笂澶嶅埗銆傚浜庡叿鏈夊閮ㄧ鐞嗗厓鏁版嵁鐨勯樀鍒楋紝
     鍋忕Щ閲忔槸浠庤澶囧紑澶寸畻璧枫€?

  bitmap/chunksize
     鐢卞崟涓綅鎵€琛ㄧず鐨勫潡鐨勫瓧鑺傚ぇ灏忋€傚浜?RAID456锛屽畠鏄崟涓澶囩殑
     涓€閮ㄥ垎锛涘浜?RAID10锛屽畠鏄樀鍒楃殑涓€閮ㄥ垎锛涘浜?RAID1锛屼袱鑰呯殕鏄?
     锛堢粨鏋滅浉鍚岋級銆?

  bitmap/time_base
     涓ゆ鏌ユ壘浣嶅浘涓緟娓呴櫎浣嶄箣闂寸殑鏃堕棿闂撮殧锛堢锛夈€傚湪褰撳墠瀹炵幇涓紝
     褰撴墍鏈夎瑕嗙洊鐨勫潡宸茬煡澶勪簬鍚屾锛坕n-sync锛夌姸鎬佸悗锛屼竴涓綅浼氬湪
     2 鍒?3 鍊嶇殑 `time_base` 鏃堕棿鍐呰娓呴櫎銆?

  bitmap/backlog
     褰?RAID1 涓瓨鍦?write-mostly 璁惧澶勪簬娲诲姩鐘舵€佹椂锛屽杩欎簺璁惧鐨?
     鍐欒姹傚湪鍚庡彴杩涜鈥斺€旀枃浠剁郴缁燂紙鎴栬澶囩殑鍏朵粬浣跨敤鑰咃級涓嶅繀绛夊緟瀹冧滑銆?
     `backlog` 璁剧疆骞跺彂鍚庡彴鍐欏叆鏁伴噺鐨勯檺鍒躲€傚鏋滆秴杩囨闄愬埗锛屾柊鐨?
     鍐欏叆灏嗗彉涓哄悓姝ョ殑銆?

  bitmap/metadata
     鍙互鏄?`internal` 鎴?`external`銆?

     `internal`
       鏄粯璁ゅ€硷紝鎰忓懗鐫€浣嶅浘鐨勫厓鏁版嵁瀛樺偍鍦ㄦ墍鍒嗛厤绌洪棿鐨勫墠 256 瀛楄妭涓紝
       骞剁敱 md 妯″潡绠＄悊銆?

     `external`
       鎰忓懗鐫€浣嶅浘鍏冩暟鎹敱鍐呮牳涔嬪锛堝嵆鐢辨煇涓敤鎴风┖闂寸▼搴忥級绠＄悊銆?

  bitmap/can_clear
     杩欐槸 `true` 鎴?`false`銆傚鏋滀负 `true`锛屽垯褰撶浉搴斿潡琚涓哄浜?
     鍚屾鐘舵€佹椂锛屼綅鍥句腑鐨勪綅灏嗚娓呴櫎銆傚鏋滀负 `false`锛屼綅灏嗘案杩滀笉浼?
     琚竻闄ゃ€傚鏋滃湪闄嶇骇闃靛垪涓婂彂鐢熷啓鍏ワ紝鎴栬€呴樀鍒楀湪鍐欏叆鏈熼棿鍙樹负闄嶇骇锛?
     姝ら」浼氳嚜鍔ㄨ涓?`false`銆傚綋鍏冩暟鎹敱澶栭儴绠＄悊鏃讹紝涓€鏃﹂樀鍒楀彉涓?
     闈為檷绾х姸鎬佸苟涓旀浜嬪疄宸茶褰曞埌鍏冩暟鎹腑锛屽簲灏嗗叾璁句负 true銆?

濡傛灉 bitmap_type 涓?llbitmap锛屽垯 md 璁惧杩樺皢鍖呭惈锛?

  llbitmap/bits
     鍙锛屾樉绀轰綅鍥句綅鐨勭姸鎬侊紝鍗虫瘡涓€肩殑鏁伴噺銆?

  llbitmap/metadata
     鍙锛屾樉绀轰綅鍥惧厓鏁版嵁锛屽寘鎷?chunksize銆乧hunkshift銆乧hunks銆?
     offset 鍜?daemon_sleep銆?

  llbitmap/daemon_sleep
     鍙鍐欙紝鍗冲畧鎶よ繘绋嬪嚱鏁拌瑙﹀彂浠ユ竻闄よ剰浣嶇殑闂撮殧鏃堕棿锛堢锛夈€?

  llbitmap/barrier_idle
     鍙鍐欙紝鍗抽〉闈㈠睆闅滅┖闂茬殑鏃堕棿锛堢锛夛紝鎰忓懗鐫€椤甸潰涓殑鑴忎綅
     灏嗚娓呴櫎銆?

闅忕潃缁勪欢璁惧琚坊鍔犲埌 md 闃靛垪锛屽畠浠細鍑虹幇鍦?`md`

```
      dev-XXX

```
鍏朵腑 `XXX` 鏄唴鏍告墍鐭ョ殑璇ヨ澶囧悕绉帮紝渚嬪 hdb1銆?
姣忎釜鐩綍鍖呭惈锛?

```
	     /sys/block/md0/md/dev-hdb1/block -> ../../../../block/hdb/hdb1

```
      super
        涓€涓寘鍚粠璇ヨ澶囪鍙栨垨鍐欏叆璇ヨ澶囩殑瓒呯骇鍧楁槧鍍忕殑鏂囦欢銆?

      state
	涓€涓褰曡澶囧湪闃靛垪涓綋鍓嶇姸鎬佺殑鏂囦欢锛屽彲浠ユ槸閫楀彿鍒嗛殧鐨勫垪琛細

	      faulty
			璁惧鐢变簬妫€娴嬪埌鏁呴殰锛屾垨瀛樺湪鏈‘璁ょ殑鍧忓潡锛?
			鑰岃绉诲嚭娲诲姩浣跨敤銆?

	      in_sync
			璁惧鏄樀鍒椾腑瀹屽叏鍚屾鐨勬垚鍛樸€?

	      writemostly
			璁惧浠呭湪鏃犲叾浠栧彲閫夋柟妗堟椂鎵嶄細琚彁浜よ璇锋眰銆?

			杩欎粎閫傜敤浜?raid1 闃靛垪銆?

	      blocked
			璁惧宸插け璐ワ紝涓旀晠闅滃皻鏈鍏冩暟鎹鐞嗙▼搴忕‘璁ゃ€?

			鏈簲鍐欏叆璇ヨ澶囩殑鍐欒姹傦紙鑻ュ叾鏈晠闅滐級浼氳闃诲銆?

	      spare
			璁惧宸ヤ綔姝ｅ父锛屼絾涓嶆槸瀹屾暣鎴愬憳銆?

			杩欏寘鎷鍦ㄦ仮澶嶈繃绋嬩腑鐨勭儹澶囩洏銆?

	      write_error
			璁惧鏇剧粡鍑虹幇杩囧啓閿欒銆?

	      want_replacement
			璁惧锛堝ぇ澶氾級宸ヤ綔姝ｅ父锛屼絾鍙兘搴旇琚浛鎹紝鏃犺鏄?
			鐢变簬閿欒杩樻槸鐢ㄦ埛璇锋眰銆?

	      replacement
			璁惧鏄敤浜庢浛鎹㈠彟涓€涓叿鏈夌浉鍚?raid_disk 鐨勬椿鍔ㄨ澶囩殑
			鏇夸唬璁惧銆?


	姝ゅ垪琛ㄥ皢鏉ュ彲鑳戒細鎵╁厖銆?

	姝ゆ枃浠跺彲鍐欏叆銆?

	鍐欏叆 ``faulty`` 妯℃嫙璁惧涓婄殑鏁呴殰銆?

	鍐欏叆 ``remove`` 灏嗚澶囦粠闃靛垪涓Щ闄ゃ€?

	鍐欏叆 ``writemostly`` 璁剧疆 writemostly 鏍囧織銆?

	鍐欏叆 ``-writemostly`` 娓呴櫎 writemostly 鏍囧織銆?

	鍐欏叆 ``blocked`` 璁剧疆 ``blocked`` 鏍囧織銆?

	鍐欏叆 ``-blocked`` 娓呴櫎 ``blocked`` 鏍囧織锛屽苟鍏佽鍐欒姹?
	瀹屾垚锛屼笖鍙兘妯℃嫙涓€涓敊璇€?

	鍐欏叆 ``in_sync`` 璁剧疆 in_sync 鏍囧織銆?

	鍐欏叆 ``write_error`` 璁剧疆 writeerrorseen 鏍囧織銆?

	鍐欏叆 ``-write_error`` 娓呴櫎 writeerrorseen 鏍囧織銆?

	闄ゆ浛鎹㈣澶囨垨鐑鐩樺锛屽彲闅忔椂鍐欏叆 ``want_replacement``銆傚畠浼氳缃鏍囧織銆?

	鍙殢鏃跺啓鍏?``-want_replacement``銆傚畠浼氭竻闄よ鏍囧織銆?

	浠呭湪鍚姩闃靛垪涔嬪墠鍏佽鍐欏叆 ``replacement`` 鎴?``-replacement``銆傚畠浼氳缃垨娓呴櫎璇ユ爣蹇椼€?


	姝ゆ枃浠跺搷搴?select/poll銆傚 ``faulty`` 鎴?``blocked`` 鐨勪换浣曟洿鏀归兘浼氳Е鍙戜竴涓簨浠躲€?

      errors
	鍦ㄦ璁惧涓婃娴嬪埌浣嗗皻鏈鑷磋澶囪绉诲嚭闃靛垪鐨勮閿欒鐨勮繎浼艰鏁?
	锛堝彲鑳芥槸鍥犱负瀹冧滑宸茶绾犳锛屾垨鑰呭洜涓哄畠浠彂鐢熷湪闃靛垪澶勪簬鍙鐘舵€佹椂锛夈€?
	褰撲娇鐢?version-1 鍏冩暟鎹椂锛屾鍊间細鍦ㄩ樀鍒楅噸鍚悗淇濇寔銆?

	姝ゅ€煎彲鍦ㄧ粍瑁呴樀鍒楁椂鍐欏叆锛屼粠鑰屼负鍏锋湁鐢ㄦ埛绌洪棿绠＄悊鍏冩暟鎹殑
	闃靛垪鎻愪緵涓€涓寔缁殑璁℃暟銆?

      slot
        杩欑粰鍑鸿澶囧湪璇ラ樀鍒椾腑鐨勮鑹层€傚鏋滆澶囦笉鍦ㄩ樀鍒椾腑娲诲姩
        锛堝嵆瀹冩槸鐑鐩樻垨宸插け璐ワ級锛屽垯涓?``none``锛屽惁鍒欎负灏忎簬闃靛垪鐨?
        ``raid_disks`` 鏁伴噺鐨勪竴涓暣鏁帮紝鎸囩ず瀹冨綋鍓嶅～鍏呯殑浣嶇疆銆?
        杩欏彧鑳藉湪缁勮闃靛垪鏃惰缃€傝缃簡姝ら」鍊肩殑璁惧琚涓烘甯稿伐浣溿€?

      offset
        杩欑粰鍑鸿澶囦腑锛堜粠璧峰澶勮捣鐨勬墖鍖烘暟锛夊瓨鍌ㄩ樀鍒楁暟鎹殑浣嶇疆銆?
        璇ュ亸绉婚噺涔嬪墠鐨勮澶囬儴鍒嗕笉浼氳瑙︾锛岄櫎闈炲畠鐢ㄤ簬瀛樺偍鍏冩暟鎹?
        锛堟牸寮?1.1 鍜?1.2锛夈€?

      size
        鍋忕Щ閲忎箣鍚庡彲鐢ㄤ簬鏁版嵁瀛樺偍鐨勮澶囧閲忋€傞€氬父涓?
	component_size 鐩稿悓銆傝繖鍙湪缁勮闃靛垪鏃跺啓鍏ャ€傚鏋滃啓鍏ョ殑鍊?
        灏忎簬褰撳墠 component_size锛屽垯浼氳鎷掔粷銆?

      recovery_start
        褰撹澶囦笉澶勪簬 ``in_sync`` 鐘舵€佹椂锛岃繖璁板綍浠庤澶囪捣濮嬪璧峰凡鐭ョ殑
	姝ｇ‘鎵囧尯鏁般€傞€氬父涓?0锛屼絾鍦ㄦ仮澶嶆搷浣滄湡闂翠細绋冲畾澧炲姞锛涘鏋滄仮澶?
	琚腑鏂紝鎭㈠姝ゅ€煎彲浣挎仮澶嶉伩鍏嶉噸澶嶈緝鏃╃殑鍧椼€傚浜?v1.x 鍏冩暟鎹紝
	姝ゅ€间細琚嚜鍔ㄤ繚瀛樺拰鎭㈠銆?

	鍙璁惧涓嶆槸闃靛垪鐨勬椿鍔ㄦ垚鍛橈紝鏃犺鏄湪闃靛垪婵€娲讳箣鍓嶈繕鏄?
	鍦?``slot`` 璁剧疆涔嬪墠锛岄兘鍙互璁剧疆姝ら」銆?

	灏嗗叾璁句负 ``none`` 绛夊悓浜庤缃?``in_sync``銆?
	璁句负浠讳綍鍏朵粬鍊间篃浼氭竻闄?``in_sync`` 鏍囧織銆?

      bad_blocks
	杩欎互璧峰鍦板潃鍜岄暱搴︼紙鍗曚綅鍧囦负鎵囧尯锛夌殑褰㈠紡缁欏嚭鎵€鏈夊凡鐭ュ潖鍧楃殑
	鍒楄〃銆傚鏋滆緭鍑鸿繃澶ц€屾棤娉曟斁鍏ヤ竴椤碉紝灏嗚鎴柇銆傚悜姝ゆ枃浠跺啓鍏?
	``sector length`` 浼氭坊鍔犳柊鐨勫凡纭锛堝嵆宸插畨鍏ㄨ褰曞埌纾佺洏锛夊潖鍧椼€?

      unacknowledged_bad_blocks
	杩欎互涓?``bad_blocks`` 鐩稿悓鐨勫舰寮忕粰鍑哄凡鐭ヤ絾灏氭湭淇濆瓨鍒扮鐩樼殑
	鍧忓潡鍒楄〃銆傚鏋滆緭鍑鸿繃澶ц€屾棤娉曟斁鍏ヤ竴椤碉紝灏嗚鎴柇銆傚啓鍏ユ鏂囦欢
	浼氭坊鍔犲潖鍧楄€屼笉纭瀹冧滑銆傝繖涓昏鐢ㄤ簬娴嬭瘯銆?

      ppl_sector, ppl_size
        姝よ澶囦笂鐢ㄤ簬閮ㄥ垎濂囧伓鏍￠獙鏃ュ織锛圥artial Parity Log锛夌殑
        绌洪棿鐨勮捣濮嬩綅缃拰澶у皬锛堝崟浣嶆墖鍖猴級銆?


涓€涓椿璺冪殑 md 璁惧杩樹細鍖呭惈姣忎釜娲诲姩璁惧鐨勬潯鐩?

```
    rdNN

```
鍏朵腑 `NN` 鏄樀鍒椾腑鐨勪綅缃紝浠?0 寮€濮嬨€?
鍥犳锛屽浜庝竴涓敱 3 涓┍鍔ㄥ櫒缁勬垚鐨勯樀鍒楋紝灏嗘湁 rd0銆乺d1銆乺d2銆?
瀹冧滑鏄寚鍚戠浉搴?`dev-XXX` 鏉＄洰鐨勭鍙烽摼鎺ャ€?

```
       cat /sys/block/md*/md/rd*/state

```
灏嗗湪姣忎竴琛屾樉绀?`in_sync`銆?



鏀寔鏁版嵁鍐椾綑锛?銆?銆?銆?銆?0锛夌骇鍒殑娲昏穬 md 璁惧杩樺寘鍚?

   sync_action
     涓€涓彲鐢ㄤ簬鐩戣鍜屾帶鍒堕噸寤鸿繃绋嬬殑鏂囨湰鏂囦欢銆傚畠鍖呭惈涓€涓崟璇嶏紝
     鍙互鏄互涓嬩箣涓€锛?

       resync锛堥噸鏂板悓姝ワ級
		鍦ㄨ剰鍏抽棴鎴栧垱寤轰箣鍚庢鍦ㄩ噸鏂拌绠楀啑浣?

       recover锛堟仮澶嶏級
		姝ｅ湪鏋勫缓鐑鐩樹互鏇挎崲澶辫触/缂哄け鐨勮澶?

       idle锛堢┖闂诧級
		娌℃湁浠讳綍鎿嶄綔鍙戠敓
       check锛堟鏌ワ級
		宸茶姹傚苟姝ｅ湪杩涜鍐椾綑鐨勫叏闈㈡鏌ャ€傝繖浼氳鍙栨墍鏈夊潡骞?
                妫€鏌ュ畠浠€傚浜庢煇浜?RAID 绾у埆锛屼篃鍙兘杩涜淇銆?

       repair锛堜慨澶嶏級
		姝ｅ湪杩涜鍏ㄩ潰鐨勬鏌ュ拰淇銆傝繖涓?`resync` 绫讳技锛屼絾鐢?
                鐢ㄦ埛璇锋眰锛屽苟涓斾笉浣跨敤鍐欐剰鍥句綅鍥炬潵浼樺寲杩囩▼銆?

      璇ユ枃浠跺彲鍐欙紝姣忎釜鍙鐨勫瓧绗︿覆瀵逛簬鍐欏叆閮芥湁鎰忎箟銆?

	`idle` 灏嗗仠姝㈡椿鍔ㄧ殑閲嶆柊鍚屾/鎭㈠绛夋搷浣溿€傛棤娉曚繚璇佷笉浼氬啀娆?
	鑷姩鍚姩鍙︿竴娆￠噸鏂板悓姝?鎭㈠锛屼絾闇€瑕佹湁鏌愪釜浜嬩欢鏉ヨЕ鍙戙€?

	濡傛灉鎿嶄綔琚?`idle` 鍋滄锛屽彲浠ヤ娇鐢?`resync` 鎴?`recovery`
        鏉ラ噸鏂板惎鍔ㄧ浉搴旂殑鎿嶄綔銆?

	濡傛灉褰撳墠鐘舵€佷负 `idle`锛宍check` 鍜?`repair` 灏嗗惎鍔ㄧ浉搴旂殑杩囩▼銆?

      璇ユ枃浠跺搷搴?select/poll銆傚€肩殑浠讳綍閲嶈鍙樺寲閮戒細瑙﹀彂涓€娆?poll 浜嬩欢銆?
      鏈夋椂锛屽鏋滈渶瑕佹仮澶嶄絾鏃犳硶瀹屾垚鏃讹紝璇ュ€间細鐭殏鍦颁负 `recover`銆?
      鍦ㄨ繖绉嶆儏鍐典笅锛屽悜 `recover` 鐨勮浆鎹笉浼氳閫氱煡锛屼絾绂诲紑璇ョ姸鎬佺殑
      杞崲浼氳閫氱煡銆?

   degraded
      杩欏寘鍚樀鍒楅檷绾ф墍缂哄皯鐨勮澶囨暟閲忚鏁般€傚洜姝わ紝鏈€浼橀樀鍒楀皢鏄剧ず `0`锛?
      鍗曚釜澶辫触/缂哄け鐨勯┍鍔ㄥ櫒灏嗘樉绀?`1`锛屼緷姝ょ被鎺ㄣ€?

      璇ユ枃浠跺搷搴?select/poll锛岀己澶辫澶囪鏁扮殑浠讳綍澧炲姞鎴栧噺灏戦兘浼氳Е鍙戜簨浠躲€?

   mismatch_count
      鍦ㄦ墽琛?`check` 鍜?`repair` 鏃讹紝浠ュ強鍙兘鎵ц `resync` 鏃讹紝md 浼?
      缁熻鍙戠幇鐨勯敊璇暟閲忋€俙mismatch_cnt` 涓殑璁℃暟鏄閲嶅啓锛屾垨
      锛堝浜?`check`锛夋湰搴旇閲嶅啓鐨勬墖鍖烘暟銆傜敱浜庡ぇ澶氭暟 RAID 绾у埆浠ラ〉
      鑰岄潪鎵囧尯涓哄崟浣嶅伐浣滐紝鍥犳璇ュ€煎彲鑳芥瘮瀹為檯閿欒鏁伴噺澶т竴涓〉涓?
      鎵囧尯鏁扮殑鍊嶆暟銆?

   bitmap_set_bits
      濡傛灉闃靛垪鍏锋湁鍐欐剰鍥句綅鍥撅紝鍒欏啓鍏ユ灞炴€у彲鍦ㄤ綅鍥句腑璁剧疆浣嶏紝
      鎸囩ず閲嶆柊鍚屾闇€瑕佹鏌ョ浉搴旂殑鍧椼€傚彲浠ュ啓鍏ュ崟涓暟瀛楁垨璧峰-缁撴潫
      瀵广€傚涓暟瀛楀彲浠ョ敤绌烘牸鍒嗛殧銆?

      娉ㄦ剰锛岃繖浜涙暟瀛楁槸 `bit`锛堜綅锛夌紪鍙凤紝鑰岄潪 `block`锛堝潡锛夌紪鍙枫€?
      瀹冧滑搴旀寜 bitmap_chunksize 缂╂斁銆?

   sync_speed_min, sync_speed_max
     杩欎笌 `/proc/sys/dev/raid/speed_limit_{min,max}` 绫讳技锛屼絾浠呴€傜敤浜?
     鐗瑰畾鐨勯樀鍒椼€?

     濡傛灉瀵硅繖浜涙枃浠舵病鏈夊啓鍏ヤ换浣曞€硷紝鎴栬€呭啓鍏ヤ簡鍗曡瘝 `system`锛屽垯浣跨敤
     绯荤粺鑼冨洿鐨勫€笺€傚鏋滃啓鍏ヤ簡浠?kibibytes-per-second锛堝崈瀛楄妭/绉掞級涓?
     鍗曚綅鐨勫€硷紝鍒欎娇鐢ㄨ鍊笺€?

     璇诲彇杩欎簺鏂囦欢鏃讹紝瀹冧滑鏄剧ず褰撳墠娲诲姩鐨勫€硷紝鍚庤窡 `(local)` 鎴?
     `(system)`锛屽叿浣撳彇鍐充簬瀹冩槸鏈湴璁剧疆鐨勫€艰繕鏄郴缁熻寖鍥寸殑鍊笺€?

   sync_completed
     杩欐樉绀哄綋鍓?sync_action 宸插畬鎴愬鐞嗙殑鎵囧尯鏁帮紝浠ュ強鎬诲叡鍙兘闇€瑕?
     澶勭悊鐨勬墖鍖烘暟銆備袱涓暟瀛椾互 `/` 鍒嗛殧锛屽洜姝ゅ疄闄呬笂鏄剧ず涓€涓€硷紝
     鍗冲凡瀹屾垚鐨勮繘绋嬫瘮渚嬨€?

     褰撻噸鏂板悓姝ュ畬鎴愭椂銆佸綋杈惧埌褰撳墠 sync_max锛堣涓嬶級鏃讹紝浠ュ強鍙兘鍦ㄥ叾浠?
     鏃跺€欙紝瀵规灞炴€х殑 `select` 浼氳繑鍥炪€?

   sync_speed
     杩欐樉绀哄綋鍓?sync_action 鐨勫疄闄呭綋鍓嶉€熷害锛屽崟浣嶄负 K/绉掋€傚畠鏄渶杩?
     30 绉掔殑骞冲潎鍊笺€?

   suspend_lo, suspend_hi
     杩欎袱涓€间互鎵囧尯鏁扮粰鍑猴紝鎸囩ず闃靛垪涓?IO 灏嗚闃诲鐨勮寖鍥淬€傜洰鍓?
     浠呮敮鎸?raid4/5/6銆?

   sync_min, sync_max
     杩欎袱涓€间互鎵囧尯鏁扮粰鍑猴紝鎸囩ず `check`/`repair` 灏嗘搷浣滅殑鑼冨洿銆傚繀椤?
     涓?chunk_size 鐨勫€嶆暟銆傚綋杈惧埌 `sync_max` 鏃讹紝瀹冧細鏆傚仠鑰岄潪瀹屾垚銆?
     鍙互浣跨敤 `sync_completed` 涓婄殑 `select` 鎴?`poll` 鏉ョ瓑寰呰鏁板瓧
     杈惧埌 sync_max銆傜劧鍚庡彲浠ュ鍔?`sync_max`锛屾垨鍚?`sync_action` 鍐欏叆
     `idle`銆?

     `sync_max` 鐨?`max` 鍊煎疄闄呬笂浼氱鐢ㄨ闄愬埗銆傚綋閲嶆柊鍚屾澶勪簬娲诲姩
     鐘舵€佹椂锛岃鍊煎彧鑳藉鍔狅紝缁濅笉鑳藉噺灏戙€?
     `sync_min` 鐨勬渶灏忓€间负 `0`銆?



姣忎釜娲昏穬鐨?md 璁惧杩樺彲鑳藉叿鏈夌壒瀹氫簬绠＄悊瀹冪殑 personality 妯″潡鐨勫睘鎬с€?
杩欎簺灞炴€х壒瀹氫簬璇ユā鍧楃殑瀹炵幇锛屽鏋滃疄鐜板彂鐢熷彉鍖栵紝鍙兘浼氬彂鐢?
閲嶅ぇ鏀瑰彉銆?

杩欎簺鐩墠鍖呮嫭锛?

  stripe_cache_size  锛堢洰鍓嶄粎 raid5锛?
      鏉″甫缂撳瓨锛坰tripe cache锛変腑鐨勬潯鐩暟銆傛鍊煎彲鍐欙紝浣嗘湁涓婁笅闄?
      锛?2768銆?7锛夈€傞粯璁ゅ€间负 256銆?

  strip_cache_active 锛堢洰鍓嶄粎 raid5锛?
      鏉″甫缂撳瓨涓椿鍔ㄦ潯鐩殑鏁伴噺

  preread_bypass_threshold 锛堢洰鍓嶄粎 raid5锛?
      闇€瑕侀璇荤殑鏉″甫琚笉闇€瑕侀璇荤殑鏉″甫鎵€缁曡繃鐨勬鏁般€備负鍏钩璧疯锛?
      榛樿涓?1銆傚皢鍏惰涓?0 浼氱鐢ㄧ粫杩囪鏁帮紝骞惰姹傞璇绘潯甯︾瓑寰呮墍鏈?
      鍏ㄥ鏉″甫鍐欏叆瀹屾垚銆傛湁鏁堝€艰寖鍥翠负 0 鍒?stripe_cache_size銆?

  journal_mode 锛堢洰鍓嶄粎 raid5锛?
      raid5 鐨勭紦瀛樻ā寮忋€俽aid5 鍙互鍖呭惈涓€涓澶栫殑纾佺洏鐢ㄤ簬缂撳瓨銆?
      妯″紡鍙互鏄?"write-through"锛堥€忓啓锛夋垨 "write-back"锛堝洖鍐欙級銆?
      榛樿涓?"write-through"銆?

  ppl_write_hint
      涓烘瘡涓?PPL 鍐欒姹傝缃殑 NVMe 娴?ID銆?
