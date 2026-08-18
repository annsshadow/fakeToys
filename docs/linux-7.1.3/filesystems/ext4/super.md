
### 瓒呯骇鍧?

瓒呯骇鍧楄褰曚簡鎵€鍦ㄦ枃浠剁郴缁熺殑鍚勭淇℃伅锛屼緥濡傚潡璁℃暟銆乮node 璁℃暟銆佹敮鎸佺殑鐗规€с€佺淮鎶や俊鎭瓑銆?
鑻ヨ缃簡 sparse_super 鐗规€ф爣蹇楋紝鍒欒秴绾у潡鍜屽潡缁勬弿杩扮鐨勫啑浣欏壇鏈粎淇濆瓨鍦ㄧ粍鍙蜂负 0 鎴?3銆?銆? 鐨勫箓鐨勫潡缁勪腑銆傝嫢鏈缃鏍囧織锛屽啑浣欏壇鏈繚瀛樺湪鎵€鏈夊潡缁勪腑銆?
瓒呯骇鍧楃殑鏍￠獙鍜岄拡瀵硅秴绾у潡缁撴瀯杩涜璁＄畻锛岃缁撴瀯鍖呭惈鏂囦欢绯荤粺 UUID銆?
ext4 瓒呯骇鍧楃殑甯冨眬鍦?`struct ext4_super_block` 涓涓嬶細

   :widths: 8 8 24 40
   :header-rows: 1

   - - 鍋忕Щ閲?     - 澶у皬
     - 鍚嶇О
     - 璇存槑
   - - 0x0
     - __le32
     - s_inodes_count
     - 鎬?inode 鏁般€?   - - 0x4
     - __le32
     - s_blocks_count_lo
     - 鎬诲潡鏁般€?   - - 0x8
     - __le32
     - s_r_blocks_count_lo
     - 杩欎簺鏁伴噺鐨勫潡鍙兘鐢辫秴绾х敤鎴峰垎閰嶃€?   - - 0xC
     - __le32
     - s_free_blocks_count_lo
     - 绌洪棽鍧楁暟銆?   - - 0x10
     - __le32
     - s_free_inodes_count
     - 绌洪棽 inode 鏁般€?   - - 0x14
     - __le32
     - s_first_data_block
     - 绗竴涓暟鎹潡銆傚浜?1KiB 鍧楃殑鏂囦欢绯荤粺锛岃鍊艰嚦灏戜负 1锛涘浜庡叾浠栨墍鏈夊潡澶у皬锛岄€氬父涓?0銆?   - - 0x18
     - __le32
     - s_log_block_size
     - 鍧楀ぇ灏忎负 2 ^ (10 + s_log_block_size)銆?   - - 0x1C
     - __le32
     - s_log_cluster_size
     - 鑻ュ惎鐢ㄤ簡 bigalloc锛屽垯绨囧ぇ灏忎负 2 ^ (10 + s_log_cluster_size) 涓潡銆傚惁鍒?s_log_cluster_size 蹇呴』绛変簬 s_log_block_size銆?   - - 0x20
     - __le32
     - s_blocks_per_group
     - 姣忕粍鍧楁暟銆?   - - 0x24
     - __le32
     - s_clusters_per_group
     - 姣忕粍绨囨暟锛岃嫢鍚敤浜?bigalloc銆傚惁鍒?s_clusters_per_group 蹇呴』绛変簬 s_blocks_per_group銆?   - - 0x28
     - __le32
     - s_inodes_per_group
     - 姣忕粍 inode 鏁般€?   - - 0x2C
     - __le32
     - s_mtime
     - 鎸傝浇鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x30
     - __le32
     - s_wtime
     - 鍐欏叆鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x34
     - __le16
     - s_mnt_count
     - 鑷笂娆?fsck 浠ユ潵鐨勬寕杞芥鏁般€?   - - 0x36
     - __le16
     - s_max_mnt_count
     - 瓒呰繃璇ユ寕杞芥鏁板氨闇€瑕佽繘琛?fsck銆?   - - 0x38
     - __le16
     - s_magic
     - 榄旀暟绛惧悕锛?xEF53
   - - 0x3A
     - __le16
     - s_state
     - 鏂囦欢绯荤粺鐘舵€併€傛洿澶氫俊鎭 super_state_銆?   - - 0x3C
     - __le16
     - s_errors
     - 妫€娴嬪埌閿欒鏃剁殑琛屼负銆傛洿澶氫俊鎭 super_errors_銆?   - - 0x3E
     - __le16
     - s_minor_rev_level
     - 娆＄増鏈彿銆?   - - 0x40
     - __le32
     - s_lastcheck
     - 涓婃妫€鏌ョ殑鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x44
     - __le32
     - s_checkinterval
     - 涓ゆ妫€鏌ヤ箣闂寸殑鏈€澶ф椂闂撮棿闅旓紝鍗曚綅涓虹銆?   - - 0x48
     - __le32
     - s_creator_os
     - 鍒涘缓鏂囦欢绯荤粺鏃剁殑鎿嶄綔绯荤粺銆傛洿澶氫俊鎭 super_creator_ 琛ㄣ€?   - - 0x4C
     - __le32
     - s_rev_level
     - 鐗堟湰鍙枫€傛洿澶氫俊鎭 super_revision_ 琛ㄣ€?   - - 0x50
     - __le16
     - s_def_resuid
     - 淇濈暀鍧楃殑榛樿 uid銆?   - - 0x52
     - __le16
     - s_def_resgid
     - 淇濈暀鍧楃殑榛樿 gid銆?#    * -

     -
     - 杩欎簺瀛楁浠呴€傜敤浜?EXT4_DYNAMIC_REV 瓒呯骇鍧椼€?
       娉ㄦ剰锛氬吋瀹圭壒鎬ч泦涓庝笉鍏煎鐗规€ч泦鐨勫尯鍒湪浜庯紝濡傛灉鍐呮牳涓嶈璇嗕笉鍏煎鐗规€ч泦涓殑鏌愪釜琚疆浣嶇殑浣嶏紝瀹冨簲璇ユ嫆缁濇寕杞借鏂囦欢绯荤粺銆?
       e2fsck 鐨勮姹傛洿涓轰弗鏍硷紱濡傛灉瀹冧笉璁よ瘑鍏煎鎴栦笉鍏煎鐗规€ч泦涓殑鏌愪釜鐗规€э紝瀹冨繀椤讳腑姝紝涓嶅幓灏濊瘯鏀瑰姩瀹冧笉鐞嗚В鐨勫唴瀹光€︹€?   - - 0x54
     - __le32
     - s_first_ino
     - 绗竴涓潪淇濈暀 inode銆?   - - 0x58
     - __le16
     - s_inode_size
     - inode 缁撴瀯鐨勫ぇ灏忥紝鍗曚綅涓哄瓧鑺傘€?   - - 0x5A
     - __le16
     - s_block_group_nr
     - 鏈秴绾у潡鎵€鍦ㄧ殑鍧楃粍缂栧彿銆?   - - 0x5C
     - __le32
     - s_feature_compat
     - 鍏煎鐗规€ч泦鏍囧織銆傚嵆浣垮唴鏍镐笉鐞嗚В鏌愪釜鏍囧織锛屼粛鍙鍐欒鏂囦欢绯荤粺锛沠sck 鍒欎笉搴斿姝ゃ€傛洿澶氫俊鎭 super_compat_ 琛ㄣ€?   - - 0x60
     - __le32
     - s_feature_incompat
     - 涓嶅吋瀹圭壒鎬ч泦銆傚鏋滃唴鏍告垨 fsck 涓嶇悊瑙ｅ叾涓煇涓綅锛屽簲鍋滄鎸傝浇銆傛洿澶氫俊鎭 super_incompat_ 琛ㄣ€?   - - 0x64
     - __le32
     - s_feature_ro_compat
     - 鍙鍏煎鐗规€ч泦銆傚鏋滃唴鏍镐笉鐞嗚В鍏朵腑鏌愪釜浣嶏紝浠嶅彲浠ュ彧璇绘柟寮忔寕杞姐€傛洿澶氫俊鎭 super_rocompat_ 琛ㄣ€?   - - 0x68
     - __u8
     - s_uuid[^16^]
     - 鍗风殑 128 浣?UUID銆?   - - 0x78
     - char
     - s_volume_name[^16^]
     - 鍗锋爣銆?   - - 0x88
     - char
     - s_last_mounted[^64^]
     - 鏂囦欢绯荤粺涓婃鎸傝浇鐨勭洰褰曘€?   - - 0xC8
     - __le32
     - s_algorithm_usage_bitmap
     - 鐢ㄤ簬鍘嬬缉锛坋2fsprogs/Linux 涓湭浣跨敤锛?#    * -

     -
     - 鎬ц兘鎻愮ず銆傚彧鏈夊綋 EXT4_FEATURE_COMPAT_DIR_PREALLOC 鏍囧織寮€鍚椂锛屾墠搴旇繘琛岀洰褰曢鍒嗛厤銆?   - - 0xCC
     - __u8
     - s_prealloc_blocks
     - 灏濊瘯涓衡€︹€︽枃浠堕鍒嗛厤鐨勫潡鏁帮紵锛坋2fsprogs/Linux 涓湭浣跨敤锛?   - - 0xCD
     - __u8
     - s_prealloc_dir_blocks
     - 涓虹洰褰曢鍒嗛厤鐨勫潡鏁般€傦紙e2fsprogs/Linux 涓湭浣跨敤锛?   - - 0xCE
     - __le16
     - s_reserved_gdt_blocks
     - 涓烘枃浠剁郴缁熸湭鏉ユ墿灞曚繚鐣欑殑 GDT 鏉＄洰鏁般€?#    * -

     -
     - 鏃ュ織鏀寔浠呭湪璁剧疆浜?EXT4_FEATURE_COMPAT_HAS_JOURNAL 鏃舵湁鏁堛€?   - - 0xD0
     - __u8
     - s_journal_uuid[^16^]
     - 鏃ュ織瓒呯骇鍧楃殑 UUID
   - - 0xE0
     - __le32
     - s_journal_inum
     - 鏃ュ織鏂囦欢鐨?inode 鍙枫€?   - - 0xE4
     - __le32
     - s_journal_dev
     - 鏃ュ織鏂囦欢鐨勮澶囧彿锛岃嫢璁剧疆浜嗗閮ㄦ棩蹇楃壒鎬ф爣蹇椼€?   - - 0xE8
     - __le32
     - s_last_orphan
     - 寰呭垹闄ゅ绔?inode 鍒楄〃鐨勮捣濮嬩綅缃€?   - - 0xEC
     - __le32
     - s_hash_seed[^4^]
     - HTREE 鍝堝笇绉嶅瓙銆?   - - 0xFC
     - __u8
     - s_def_hash_version
     - 鐢ㄤ簬鐩綍鍝堝笇鐨勯粯璁ゅ搱甯岀畻娉曘€傛洿澶氫俊鎭 super_def_hash_銆?   - - 0xFD
     - __u8
     - s_jnl_backup_type
     - 鑻ヨ鍊间负 0 鎴?EXT3_JNL_BACKUP_BLOCKS (1)锛屽垯 `s_jnl_blocks` 瀛楁鍖呭惈璇?inode 鐨?`i_block[]` 鏁扮粍涓?`i_size` 鐨勫壇鏈€?   - - 0xFE
     - __le16
     - s_desc_size
     - 鍧楃粍鎻忚堪绗︾殑澶у皬锛堝瓧鑺傦級锛岃嫢鍚敤浜?64bit 涓嶅吋瀹圭壒鎬ф爣蹇椼€?   - - 0x100
     - __le32
     - s_default_mount_opts
     - 榛樿鎸傝浇閫夐」銆傛洿澶氫俊鎭 super_mountopts_ 琛ㄣ€?   - - 0x104
     - __le32
     - s_first_meta_bg
     - 绗竴涓厓鍧楀潡缁勶紝鑻ュ惎鐢ㄤ簡 meta_bg 鐗规€с€?   - - 0x108
     - __le32
     - s_mkfs_time
     - 鏂囦欢绯荤粺鍒涘缓鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x10C
     - __le32
     - s_jnl_blocks[^17^]
     - 鏃ュ織 inode 鐨?`i_block[]` 鏁扮粍鐨勫墠 15 涓厓绱犲浠斤紝浠ュ強 i_size_high 鍜?i_size 鍒嗗埆浣嶄簬绗?16 鍜?17 涓厓绱犮€?#    * -

     -
     - 64bit 鏀寔浠呭湪璁剧疆浜?EXT4_FEATURE_COMPAT_64BIT 鏃舵湁鏁堛€?   - - 0x150
     - __le32
     - s_blocks_count_hi
     - 鍧楄鏁扮殑楂?32 浣嶃€?   - - 0x154
     - __le32
     - s_r_blocks_count_hi
     - 淇濈暀鍧楄鏁扮殑楂?32 浣嶃€?   - - 0x158
     - __le32
     - s_free_blocks_count_hi
     - 绌洪棽鍧楄鏁扮殑楂?32 浣嶃€?   - - 0x15C
     - __le16
     - s_min_extra_isize
     - 鎵€鏈?inode 鑷冲皯鎷ユ湁 # 瀛楄妭銆?   - - 0x15E
     - __le16
     - s_want_extra_isize
     - 鏂?inode 搴旈鐣?# 瀛楄妭銆?   - - 0x160
     - __le32
     - s_flags
     - 鏉傞」鏍囧織銆傛洿澶氫俊鎭 super_flags_ 琛ㄣ€?   - - 0x164
     - __le16
     - s_raid_stride
     - RAID stride锛堟闀匡級銆傝繖鏄湪鍒囨崲鍒颁笅涓€涓鐩樹箣鍓嶄粠纾佺洏璇诲嚭鎴栧啓鍏ョ鐩樼殑閫昏緫鍧楁暟銆傝繖浼氬奖鍝嶆枃浠剁郴缁熷厓鏁版嵁鐨勫竷灞€锛屾湁鏈涙彁鍗?RAID 瀛樺偍閫熷害銆?   - - 0x166
     - __le16
     - s_mmp_interval
     - 澶氭寕杞介槻鎶わ紙MMP锛夋鏌ヤ腑绛夊緟鐨勭鏁般€傜悊璁轰笂锛孧MP 鏄竴绉嶅湪瓒呯骇鍧椾腑璁板綍宸叉寕杞借鏂囦欢绯荤粺鐨勪富鏈哄拰璁惧鐨勬満鍒讹紝浠ラ槻姝㈠娆℃寕杞姐€傝鐗规€т技涔庡苟鏈疄鐜扳€︹€?   - - 0x168
     - __le64
     - s_mmp_block
     - 澶氭寕杞介槻鎶ゆ暟鎹殑鍧楀彿銆?   - - 0x170
     - __le32
     - s_raid_stripe_width
     - RAID 鏉″甫瀹藉害銆傝繖鏄湪鍥炲埌褰撳墠纾佺洏涔嬪墠浠庣鐩樿鍑烘垨鍐欏叆纾佺洏鐨勯€昏緫鍧楁暟銆傚潡鍒嗛厤鍣ㄥ埄鐢ㄥ畠鏉ュ敖閲忓噺灏?RAID5/6 涓殑璇?淇敼-鍐欐搷浣滄鏁般€?   - - 0x174
     - __u8
     - s_log_groups_per_flex
     - 鐏垫椿鍧楃粍鐨勫ぇ灏忎负 2 ^ `s_log_groups_per_flex`銆?   - - 0x175
     - __u8
     - s_checksum_type
     - 鍏冩暟鎹牎楠屽拰绠楁硶绫诲瀷銆傚敮涓€鏈夋晥鐨勫€间负 1锛坈rc32c锛夈€?   - - 0x176
     - \_\_u8
     - s\_encryption\_level
     - 鍔犲瘑鐨勭増鏈骇鍒€?   - - 0x177
     - \_\_u8
     - s\_reserved\_pad
     - 濉厖鍒颁笅涓€涓?32 浣嶃€?   - - 0x178
     - __le64
     - s_kbytes_written
     - 璇ユ枃浠剁郴缁熺敓鍛藉懆鏈熷唴鍐欏叆鐨?KiB 鏁般€?   - - 0x180
     - __le32
     - s_snapshot_inum
     - 娲诲姩蹇収鐨?inode 鍙枫€傦紙e2fsprogs/Linux 涓湭浣跨敤銆傦級
   - - 0x184
     - __le32
     - s_snapshot_id
     - 娲诲姩蹇収鐨勯『搴?ID銆傦紙e2fsprogs/Linux 涓湭浣跨敤銆傦級
   - - 0x188
     - __le64
     - s_snapshot_r_blocks_count
     - 涓烘椿鍔ㄥ揩鐓ф湭鏉ヤ娇鐢ㄨ€屼繚鐣欑殑鍧楁暟銆傦紙e2fsprogs/Linux 涓湭浣跨敤銆傦級
   - - 0x190
     - __le32
     - s_snapshot_list
     - 纾佺洏涓婂揩鐓у垪琛ㄥご閮ㄧ殑 inode 鍙枫€傦紙e2fsprogs/Linux 涓湭浣跨敤銆傦級
   - - 0x194
     - __le32
     - s_error_count
     - 閬囧埌鐨勯敊璇暟銆?   - - 0x198
     - __le32
     - s_first_error_time
     - 棣栨鍙戠敓閿欒鐨勬椂闂达紝鑷邯鍏冭捣鐨勭鏁般€?   - - 0x19C
     - __le32
     - s_first_error_ino
     - 棣栨閿欒娑夊強鐨?inode銆?   - - 0x1A0
     - __le64
     - s_first_error_block
     - 棣栨閿欒娑夊強鐨勫潡鍙枫€?   - - 0x1A8
     - __u8
     - s_first_error_func[^32^]
     - 鍙戠敓閿欒鐨勫嚱鏁板悕銆?   - - 0x1C8
     - __le32
     - s_first_error_line
     - 鍙戠敓閿欒鐨勮鍙枫€?   - - 0x1CC
     - __le32
     - s_last_error_time
     - 鏈€杩戜竴娆￠敊璇殑鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x1D0
     - __le32
     - s_last_error_ino
     - 鏈€杩戜竴娆￠敊璇秹鍙婄殑 inode銆?   - - 0x1D4
     - __le32
     - s_last_error_line
     - 鏈€杩戜竴娆￠敊璇彂鐢熺殑琛屽彿銆?   - - 0x1D8
     - __le64
     - s_last_error_block
     - 鏈€杩戜竴娆￠敊璇秹鍙婄殑鍧楀彿銆?   - - 0x1E0
     - __u8
     - s_last_error_func[^32^]
     - 鍙戠敓鏈€杩戜竴娆￠敊璇殑鍑芥暟鍚嶃€?   - - 0x200
     - __u8
     - s_mount_opts[^64^]
     - 鎸傝浇閫夐」鐨?ASCIIZ 瀛楃涓层€?   - - 0x240
     - __le32
     - s_usr_quota_inum
     - 鐢ㄦ埛 `quota <quota>`__ 鏂囦欢鐨?inode 鍙枫€?   - - 0x244
     - __le32
     - s_grp_quota_inum
     - 缁?`quota <quota>`__ 鏂囦欢鐨?inode 鍙枫€?   - - 0x248
     - __le32
     - s_overhead_blocks
     - 鏂囦欢绯荤粺涓殑寮€閿€鍧?绨囥€傦紙鍡紵璇ュ瓧娈靛缁堜负闆讹紝鎰忓懗鐫€鍐呮牳浼氬姩鎬佽绠楀畠銆傦級
   - - 0x24C
     - __le32
     - s_backup_bgs[^2^]
     - 鍖呭惈瓒呯骇鍧楀浠界殑鍧楃粍锛堣嫢 sparse_super2锛?   - - 0x254
     - __u8
     - s_encrypt_algos[^4^]
     - 姝ｅ湪浣跨敤鐨勫姞瀵嗙畻娉曘€備换鎰忔椂鍒绘渶澶氬彲浣跨敤鍥涚绠楁硶锛涙湁鏁堢殑绠楁硶浠ｇ爜瑙佷笅鏂?super_encrypt_ 琛ㄣ€?   - - 0x258
     - __u8
     - s_encrypt_pw_salt[^16^]
     - 鐢ㄤ簬鍔犲瘑鐨?string2key 绠楁硶鐨勭洂鍊笺€?   - - 0x268
     - __le32
     - s_lpf_ino
     - lost+found 鐨?inode 鍙?   - - 0x26C
     - __le32
     - s_prj_quota_inum
     - 璺熻釜椤圭洰閰嶉鐨?inode銆?   - - 0x270
     - __le32
     - s_checksum_seed
     - 鐢ㄤ簬 metadata_csum 璁＄畻鐨勬牎楠屽拰绉嶅瓙銆傝鍊间负 crc32c(~0, $orig_fs_uuid)銆?   - - 0x274
     - __u8
     - s_wtime_hi
     - s_wtime 瀛楁鐨勯珮 8 浣嶃€?   - - 0x275
     - __u8
     - s_mtime_hi
     - s_mtime 瀛楁鐨勯珮 8 浣嶃€?   - - 0x276
     - __u8
     - s_mkfs_time_hi
     - s_mkfs_time 瀛楁鐨勯珮 8 浣嶃€?   - - 0x277
     - __u8
     - s_lastcheck_hi
     - s_lastcheck 瀛楁鐨勯珮 8 浣嶃€?   - - 0x278
     - __u8
     - s_first_error_time_hi
     - s_first_error_time 瀛楁鐨勯珮 8 浣嶃€?   - - 0x279
     - __u8
     - s_last_error_time_hi
     - s_last_error_time 瀛楁鐨勯珮 8 浣嶃€?   - - 0x27A
     - \_\_u8
     - s\_first\_error\_errcode
     -
   - - 0x27B
     - \_\_u8
     - s\_last\_error\_errcode
     -
   - - 0x27C
     - __le16
     - s_encoding
     - 鏂囦欢鍚嶅瓧绗﹂泦缂栫爜銆?   - - 0x27E
     - __le16
     - s_encoding_flags
     - 鏂囦欢鍚嶅瓧绗﹂泦缂栫爜鏍囧織銆?   - - 0x280
     - __le32
     - s_orphan_file_inum
     - 瀛ゅ効鏂囦欢 inode 鍙枫€?   - - 0x284
     - __le32
     - s_reserved[^94^]
     - 濉厖鑷冲潡鏈熬銆?   - - 0x3FC
     - __le32
     - s_checksum
     - 瓒呯骇鍧楁牎楠屽拰銆?

瓒呯骇鍧楃姸鎬佹槸浠ヤ笅鍚勯」鐨勬煇绉嶇粍鍚堬細

   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x0001
     - 宸插共鍑€鍗歌浇
   - - 0x0002
     - 妫€娴嬪埌閿欒
   - - 0x0004
     - 姝ｅ湪鎭㈠瀛ゅ効锛坥rphan锛塱node


瓒呯骇鍧楅敊璇鐞嗙瓥鐣ヤ负浠ヤ笅涔嬩竴锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 1
     - 缁х画
   - - 2
     - 浠ュ彧璇绘柟寮忛噸鏂版寕杞?   - - 3
     - Panic锛堝唴鏍告厡涔憋級


鏂囦欢绯荤粺鍒涘缓鑰咃紙鎿嶄綔绯荤粺锛変负浠ヤ笅涔嬩竴锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0
     - Linux
   - - 1
     - Hurd
   - - 2
     - Masix
   - - 3
     - FreeBSD
   - - 4
     - Lites


瓒呯骇鍧楃増鏈负浠ヤ笅涔嬩竴锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0
     - 鍘熷鏍煎紡
   - - 1
     - 甯︽湁鍔ㄦ€?inode 澶у皬鐨?v2 鏍煎紡

娉ㄦ剰 `EXT4_DYNAMIC_REV` 鎸囩殑鏄増鏈?1 鎴栨洿鏂扮殑鏂囦欢绯荤粺銆?

瓒呯骇鍧楀吋瀹圭壒鎬у瓧娈垫槸浠ヤ笅浠绘剰椤圭殑缁勫悎锛?
   :widths: 16 64
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x1
     - 鐩綍棰勫垎閰嶏紙COMPAT_DIR_PREALLOC锛夈€?   - - 0x2
     - 鈥渋magic inodes鈥濄€備粠浠ｇ爜涓笉娓呮鍏朵綔鐢紙COMPAT_IMAGIC_INODES锛夈€?   - - 0x4
     - 鎷ユ湁鏃ュ織锛圕OMPAT_HAS_JOURNAL锛夈€?   - - 0x8
     - 鏀寔鎵╁睍灞炴€э紙COMPAT_EXT_ATTR锛夈€?   - - 0x10
     - 鎷ユ湁鐢ㄤ簬鏂囦欢绯荤粺鎵╁睍鐨勪繚鐣?GDT 鍧楋紙COMPAT_RESIZE_INODE锛夈€傞渶瑕?RO_COMPAT_SPARSE_SUPER銆?   - - 0x20
     - 鎷ユ湁鐩綍绱㈠紩锛圕OMPAT_DIR_INDEX锛夈€?   - - 0x40
     - 鈥淟azy BG鈥濄€備笉鍦?Linux 鍐呮牳涓紝浼间箮鏇剧敤浜庢湭鍒濆鍖栫殑鍧楃粍锛燂紙COMPAT_LAZY_BG锛?   - - 0x80
     - 鈥淓xclude inode鈥濄€傛湭浣跨敤銆傦紙COMPAT_EXCLUDE_INODE锛夈€?   - - 0x100
     - 鈥淓xclude bitmap鈥濄€備技涔庣敤浜庢寚绀哄瓨鍦ㄤ笌蹇収鐩稿叧鐨勬帓闄や綅鍥撅紵鍐呮牳涓湭瀹氫箟锛宔2fsprogs 涓篃鏈娇鐢紙COMPAT_EXCLUDE_BITMAP锛夈€?   - - 0x200
     - 绋€鐤忚秴绾у潡 v2銆傝嫢璁剧疆璇ユ爣蹇楋紝瓒呯骇鍧楃殑 s_backup_bgs 瀛楁鎸囧悜鍖呭惈澶囦唤瓒呯骇鍧楃殑涓や釜鍧楃粍锛圕OMPAT_SPARSE_SUPER2锛夈€?   - - 0x400
     - 鏀寔蹇€熸彁浜わ紙fast commit锛夈€傚敖绠″揩閫熸彁浜ゅ潡鏄悜鍚庝笉鍏煎鐨勶紝浣嗘棩蹇椾腑骞朵笉鎬绘槸鍖呭惈蹇€熸彁浜ゅ潡銆傝嫢鏃ュ織涓瓨鍦ㄥ揩閫熸彁浜ゅ潡锛屽垯 JBD2 涓嶅吋瀹圭壒鎬э紙JBD2_FEATURE_INCOMPAT_FAST_COMMIT锛変細琚缃紙COMPAT_FAST_COMMIT锛夈€?   - - 0x1000
     - 宸插垎閰嶅鍎挎枃浠躲€傝繖鏄敤浜庢洿楂樻晥鍦拌窡韪凡鍒犻櫎浣嗕粛鎵撳紑鐨?inode 鐨勭壒娈婃枃浠躲€傚綋璇ユ枃浠跺彲鑳藉瓨鍦ㄤ换浣曟潯鐩椂锛屾垜浠繕浼氳缃浉搴旂殑 rocompat 鐗规€э紙RO_COMPAT_ORPHAN_PRESENT锛夈€?

瓒呯骇鍧椾笉鍏煎鐗规€у瓧娈垫槸浠ヤ笅浠绘剰椤圭殑缁勫悎锛?
   :widths: 16 64
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x1
     - 鍘嬬缉锛圛NCOMPAT_COMPRESSION锛夈€?   - - 0x2
     - 鐩綍椤硅褰曟枃浠剁被鍨嬨€傝涓嬫枃鐨?ext4_dir_entry_2锛圛NCOMPAT_FILETYPE锛夈€?   - - 0x4
     - 鏂囦欢绯荤粺闇€瑕佹仮澶嶏紙INCOMPAT_RECOVER锛夈€?   - - 0x8
     - 鏂囦欢绯荤粺鎷ユ湁鐙珛鐨勬棩蹇楄澶囷紙INCOMPAT_JOURNAL_DEV锛夈€?   - - 0x10
     - 鍏冨潡缁勩€傝鍓嶆枃瀵硅鐗规€х殑璁ㄨ锛圛NCOMPAT_META_BG锛夈€?   - - 0x40
     - 璇ユ枃浠剁郴缁熶腑鐨勬枃浠朵娇鐢?extents锛圛NCOMPAT_EXTENTS锛夈€?   - - 0x80
     - 鍚敤 2^64 涓潡鐨勬枃浠剁郴缁熷ぇ灏忥紙INCOMPAT_64BIT锛夈€?   - - 0x100
     - 澶氭寕杞戒繚鎶わ紙INCOMPAT_MMP锛夈€?   - - 0x200
     - 鐏垫椿鍧楃粍銆傝鍓嶆枃瀵硅鐗规€х殑璁ㄨ锛圛NCOMPAT_FLEX_BG锛夈€?   - - 0x400
     - inode 鍙敤浜庡瓨鍌ㄨ緝澶х殑鎵╁睍灞炴€у€硷紙INCOMPAT_EA_INODE锛夈€?   - - 0x1000
     - 鐩綍椤逛腑鐨勬暟鎹紙INCOMPAT_DIRDATA锛夈€傦紙鏈疄鐜帮紵锛?   - - 0x2000
     - 鍏冩暟鎹牎楠屽拰绉嶅瓙瀛樺偍鍦ㄨ秴绾у潡涓€傝鐗规€у厑璁哥鐞嗗憳鍦ㄦ枃浠剁郴缁熸寕杞芥椂鏇存敼 metadata_csum 鏂囦欢绯荤粺鐨?UUID锛涙病鏈夊畠锛屾牎楠屽拰瀹氫箟瑕佹眰閲嶅啓鎵€鏈夊厓鏁版嵁鍧楋紙INCOMPAT_CSUM_SEED锛夈€?   - - 0x4000
     - 澶х洰褰?>2GB 鎴?3 绾?htree锛圛NCOMPAT_LARGEDIR锛夈€傚湪姝ょ壒鎬т箣鍓嶏紝鐩綍涓嶈兘澶т簬 4GiB锛屼笖 htree 娣卞害涓嶈兘瓒呰繃 2 灞傘€傝嫢鍚敤璇ョ壒鎬э紝鐩綍鍙互澶т簬 4GiB锛屼笖 htree 鏈€澶ф繁搴︿负 3銆?   - - 0x8000
     - inode 涓殑鏁版嵁锛圛NCOMPAT_INLINE_DATA锛夈€?   - - 0x10000
     - 鍙兘瀛樺湪鍔犲瘑 inode銆傦紙INCOMPAT_ENCRYPT锛夈€?   - - 0x20000
     - 鐩綍鍙鏍囪涓轰笉鍖哄垎澶у皬鍐欍€傦紙INCOMPAT_CASEFOLD锛夈€?

瓒呯骇鍧楀彧璇诲吋瀹圭壒鎬у瓧娈垫槸浠ヤ笅浠绘剰椤圭殑缁勫悎锛?
   :widths: 16 64
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x1
     - 绋€鐤忚秴绾у潡銆傝鍓嶆枃瀵硅鐗规€х殑璁ㄨ锛圧O_COMPAT_SPARSE_SUPER锛夈€?   - - 0x2
     - 璇ユ枃浠剁郴缁熸浘鐢ㄤ簬瀛樺偍澶т簬 2GiB 鐨勬枃浠讹紙RO_COMPAT_LARGE_FILE锛夈€?   - - 0x4
     - 鍐呮牳鎴?e2fsprogs 涓湭浣跨敤锛圧O_COMPAT_BTREE_DIR锛夈€?   - - 0x8
     - 璇ユ枃浠剁郴缁熶腑鐨勬枃浠跺ぇ灏忎互閫昏緫鍧椾负鍗曚綅琛ㄧず锛岃€岄潪 512 瀛楄妭鎵囧尯銆傝繖鎰忓懗鐫€鏂囦欢纭疄闈炲父澶э紒锛圧O_COMPAT_HUGE_FILE锛?   - - 0x10
     - 鍧楃粍鎻忚堪绗﹀甫鏈夋牎楠屽拰銆傞櫎浜嗘娴嬫崯鍧忓锛岃繖瀵瑰甫鏈夋湭鍒濆鍖栫粍鐨勬儼鎬ф牸寮忓寲涔熷緢鏈夌敤锛圧O_COMPAT_GDT_CSUM锛夈€?   - - 0x20
     - 琛ㄧず鏃х殑 ext3 涓変竾浜屽崈瀛愮洰褰曢檺鍒朵笉鍐嶉€傜敤锛圧O_COMPAT_DIR_NLINK锛夈€傝嫢鐩綍鐨?i_links_count 瓒呰繃 64,999锛屽皢琚涓?1銆?   - - 0x40
     - 琛ㄧず璇ユ枃浠剁郴缁熶笂瀛樺湪杈冨ぇ鐨?inode锛圧O_COMPAT_EXTRA_ISIZE锛夈€?   - - 0x80
     - 璇ユ枃浠剁郴缁熸嫢鏈夊揩鐓э紙RO_COMPAT_HAS_SNAPSHOT锛夈€?   - - 0x100
     - `Quota <Quota>`__锛圧O_COMPAT_QUOTA锛夈€?   - - 0x200
     - 璇ユ枃浠剁郴缁熸敮鎸?鈥渂igalloc鈥濓紝鍗虫枃浠?extents 浠ョ皣锛堝潡鐨勯泦鍚堬級涓哄崟浣嶈€岄潪浠ュ潡涓哄崟浣嶈繘琛岃窡韪紙RO_COMPAT_BIGALLOC锛夈€?   - - 0x400
     - 璇ユ枃浠剁郴缁熸敮鎸佸厓鏁版嵁鏍￠獙鍜屻€傦紙RO_COMPAT_METADATA_CSUM锛涢殣鍚?RO_COMPAT_GDT_CSUM锛屼絾 GDT_CSUM 涓嶅緱璁剧疆锛?   - - 0x800
     - 鏂囦欢绯荤粺鏀寔鍓湰銆傝鐗规€ф棦涓嶅湪鍐呮牳涓紝涔熶笉鍦?e2fsprogs 涓€傦紙RO_COMPAT_REPLICA锛?   - - 0x1000
     - 鍙鏂囦欢绯荤粺闀滃儚锛涘唴鏍镐笉浼氫互璇诲啓鏂瑰紡鎸傝浇璇ラ暅鍍忥紝涓斿ぇ澶氭暟宸ュ叿浼氭嫆缁濆啓鍏ラ暅鍍忋€傦紙RO_COMPAT_READONLY锛?   - - 0x2000
     - 鏂囦欢绯荤粺璺熻釜椤圭洰閰嶉銆傦紙RO_COMPAT_PROJECT锛?   - - 0x8000
     - 鏂囦欢绯荤粺涓婂彲鑳藉瓨鍦?Verity inode銆傦紙RO_COMPAT_VERITY锛?   - - 0x10000
     - 琛ㄧず瀛ゅ効鏂囦欢鍙兘鍚湁鏈夋晥鐨勫鍎挎潯鐩紝鍥犳鎸傝浇鏂囦欢绯荤粺鏃堕渶瑕佹竻鐞嗗畠浠紙RO_COMPAT_ORPHAN_PRESENT锛夈€?

`s_def_hash_version` 瀛楁涓轰互涓嬩箣涓€锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x0
     - 浼犵粺鏂瑰紡銆?   - - 0x1
     - 鍗?MD4銆?   - - 0x2
     - Tea銆?   - - 0x3
     - 浼犵粺鏂瑰紡锛屾棤绗﹀彿銆?   - - 0x4
     - 鍗?MD4锛屾棤绗﹀彿銆?   - - 0x5
     - Tea锛屾棤绗﹀彿銆?

`s_default_mount_opts` 瀛楁鏄互涓嬪悇椤圭殑浠绘剰缁勫悎锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x0001
     - 鍦紙閲嶆柊锛夋寕杞芥椂鎵撳嵃璋冭瘯淇℃伅銆傦紙EXT4_DEFM_DEBUG锛?   - - 0x0002
     - 鏂版枃浠堕噰鐢ㄥ叾鎵€鍦ㄧ洰褰曠殑 gid锛堣€岄潪褰撳墠杩涚▼鐨?fsgid锛夈€傦紙EXT4_DEFM_BSDGROUPS锛?   - - 0x0004
     - 鏀寔鐢ㄦ埛绌洪棿鎻愪緵鐨勬墿灞曞睘鎬с€傦紙EXT4_DEFM_XATTR_USER锛?   - - 0x0008
     - 鏀寔 POSIX 璁块棶鎺у埗鍒楄〃锛圓CL锛夈€傦紙EXT4_DEFM_ACL锛?   - - 0x0010
     - 涓嶆敮鎸?32 浣?UID銆傦紙EXT4_DEFM_UID16锛?   - - 0x0020
     - 鎵€鏈夋暟鎹拰鍏冩暟鎹兘鎻愪氦鍒版棩蹇椼€傦紙EXT4_DEFM_JMODE_DATA锛?   - - 0x0040
     - 鍦ㄥ厓鏁版嵁鎻愪氦鍒版棩蹇椾箣鍓嶏紝鎵€鏈夋暟鎹兘宸插埛鏂板埌纾佺洏銆傦紙EXT4_DEFM_JMODE_ORDERED锛?   - - 0x0060
     - 涓嶄繚鐣欐暟鎹『搴忥紱鏁版嵁鍙兘鍦ㄥ厓鏁版嵁鍐欏叆涔嬪悗鎵嶅啓鍏ャ€傦紙EXT4_DEFM_JMODE_WBACK锛?   - - 0x0100
     - 绂佺敤鍐欏埛鏂般€傦紙EXT4_DEFM_NOBARRIER锛?   - - 0x0200
     - 璺熻釜鏂囦欢绯荤粺涓摢浜涘潡鏄厓鏁版嵁锛屽洜姝や笉搴旇鐢ㄤ綔鏁版嵁鍧椼€傝閫夐」鏈夋湜鍦?3.18 涓粯璁ゅ惎鐢ㄣ€傦紙EXT4_DEFM_BLOCK_VALIDITY锛?   - - 0x0400
     - 鍚敤 DISCARD 鏀寔锛屽嵆閫氱煡瀛樺偍璁惧鍝簺鍧楀凡鍙樹负鏈娇鐢ㄣ€傦紙EXT4_DEFM_DISCARD锛?   - - 0x0800
     - 绂佺敤寤惰繜鍒嗛厤銆傦紙EXT4_DEFM_NODELALLOC锛?

`s_flags` 瀛楁鏄互涓嬪悇椤圭殑浠绘剰缁勫悎锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0x0001
     - 浣跨敤甯︾鍙风殑鐩綍鍝堝笇銆?   - - 0x0002
     - 浣跨敤鏃犵鍙风殑鐩綍鍝堝笇銆?   - - 0x0004
     - 鐢ㄤ簬娴嬭瘯寮€鍙戜唬鐮併€?

`s_encrypt_algos` 鍒楄〃鍙寘鍚互涓嬩换鎰忛」锛?
   :widths: 8 72
   :header-rows: 1

   - - 鍊?     - 璇存槑
   - - 0
     - 鏃犳晥绠楁硶锛圗NCRYPTION_MODE_INVALID锛夈€?   - - 1
     - XTS 妯″紡涓嬬殑 256 浣?AES锛圗NCRYPTION_MODE_AES_256_XTS锛夈€?   - - 2
     - GCM 妯″紡涓嬬殑 256 浣?AES锛圗NCRYPTION_MODE_AES_256_GCM锛夈€?   - - 3
     - CBC 妯″紡涓嬬殑 256 浣?AES锛圗NCRYPTION_MODE_AES_256_CBC锛夈€?
瓒呯骇鍧楃殑鎬诲ぇ灏忎负 1024 瀛楄妭銆?