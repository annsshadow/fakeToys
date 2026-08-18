
### 绱㈠紩鑺傜偣锛圛ndex Nodes锛?
鍦ㄥ父瑙勭殑 UNIX 鏂囦欢绯荤粺涓紝inode 瀛樺偍涓庢枃浠剁浉鍏崇殑鎵€鏈夊厓鏁版嵁锛堟椂闂存埑銆佸潡鏄犲皠銆佹墿灞曞睘鎬х瓑锛夛紝
鑰屼笉瀛樺偍鐩綍椤广€傝鏌ユ壘涓庢枃浠跺叧鑱旂殑淇℃伅锛屽繀椤婚亶鍘嗙洰褰曟枃浠朵互鎵惧埌涓庤鏂囦欢鍏宠仈鐨勭洰褰曢」锛?鐒跺悗鍔犺浇 inode 浠ユ壘鍒拌鏂囦欢鐨勫厓鏁版嵁銆傚嚭浜庢€ц兘鍘熷洜锛宔xt4 浼间箮锛堢暐寰級鑰嶄簡鐐瑰皬鑱槑锛氬畠鍦ㄧ洰褰曢」涓?瀛樺偍涓€浠芥枃浠剁被鍨嬶紙閫氬父瀛樹簬 inode 涓級鐨勫壇鏈€傦紙灏嗚繖涓€鍒囦笌 FAT 瀵规瘮锛欶AT 鎶婃墍鏈夋枃浠朵俊鎭洿鎺ュ瓨浜?鐩綍椤逛腑锛屼絾涓嶆敮鎸佺‖閾炬帴锛屽苟涓旂敱浜庡叾鏇寸畝鍗曠殑鍧楀垎閰嶅櫒浠ュ強澶ч噺浣跨敤閾捐〃锛岄€氬父姣?ext4 鏇撮绻佸湴瀵婚亾銆傦級

inode 琛ㄦ槸涓€涓?`struct ext4_inode` 鐨勭嚎鎬ф暟缁勩€傝琛ㄧ殑澶у皬琚涓鸿冻浠ュ瓨鍌ㄨ嚦灏?`sb.s_inode_size * sb.s_inodes_per_group` 瀛楄妭銆傚寘鍚煇涓?inode 鐨勫潡缁勭紪鍙峰彲璁＄畻涓?`(inode_number - 1) / sb.s_inodes_per_group`锛岃€岃缁勫唴琛ㄧ殑鍋忕Щ涓?`(inode_number - 1) % sb.s_inodes_per_group`銆備笉瀛樺湪 inode 0銆?
inode 鏍￠獙鍜屾槸閽堝 FS UUID銆乮node 缂栧彿浠ュ強 inode 缁撴瀯鏈韩璁＄畻鐨勩€?
inode 琛ㄩ」甯冨眬浜?`struct ext4_inode` 涓€?
   :widths: 8 8 24 40
   :header-rows: 1
   :class: longtable

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - i_mode
     - 鏂囦欢妯″紡銆傚弬瑙佷笅鏂囩殑 i_mode_ 琛ㄣ€?   - - 0x2
     - __le16
     - i_uid
     - 鎵€鏈夎€?UID 鐨勪綆 16 浣嶃€?   - - 0x4
     - __le32
     - i_size_lo
     - 澶у皬锛堝瓧鑺傦級鐨勪綆 32 浣嶃€?   - - 0x8
     - __le32
     - i_atime
     - 涓婃璁块棶鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆備絾鏄紝鑻ヨ缃簡 EA_INODE inode 鏍囧織锛屾 inode 瀛樺偍涓€涓?       鎵╁睍灞炴€у€硷紝璇ュ瓧娈靛寘鍚鍊肩殑鏍￠獙鍜屻€?   - - 0xC
     - __le32
     - i_ctime
     - 涓婃 inode 鏇存敼鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆備絾鏄紝鑻ヨ缃簡 EA_INODE inode 鏍囧織锛屾 inode 瀛樺偍
       涓€涓墿灞曞睘鎬у€硷紝璇ュ瓧娈靛寘鍚睘鎬у€煎紩鐢ㄨ鏁扮殑浣?32 浣嶃€?   - - 0x10
     - __le32
     - i_mtime
     - 涓婃鏁版嵁淇敼鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆備絾鏄紝鑻ヨ缃簡 EA_INODE inode 鏍囧織锛屾 inode 瀛樺偍涓€涓?       鎵╁睍灞炴€у€硷紝璇ュ瓧娈靛寘鍚嫢鏈夎鎵╁睍灞炴€х殑 inode 鐨勭紪鍙枫€?   - - 0x14
     - __le32
     - i_dtime
     - 鍒犻櫎鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x18
     - __le16
     - i_gid
     - GID 鐨勪綆 16 浣嶃€?   - - 0x1A
     - __le16
     - i_links_count
     - 纭摼鎺ヨ鏁般€傞€氬父锛宔xt4 涓嶅厑璁镐竴涓?inode 鎷ユ湁瓒呰繃 65,000 涓‖閾炬帴銆傝繖閫傜敤浜庢枃浠跺拰鐩綍锛?       鎰忓懗鐫€涓€涓洰褰曚腑涓嶈兘鏈夎秴杩?64,998 涓瓙鐩綍锛堟瘡涓瓙鐩綍鐨?'..' 椤硅涓轰竴涓‖閾炬帴锛岀洰褰曡嚜韬殑
       '.' 椤逛篃鏄姝わ級銆傚惎鐢?DIR_NLINK 鐗规€у悗锛宔xt4 閫氳繃灏嗘瀛楁璁句负 1 鏉ヨ〃绀虹‖閾炬帴鏁版湭鐭ワ紝
       浠庤€屾敮鎸佽秴杩?64,998 涓瓙鐩綍銆?   - - 0x1C
     - __le32
     - i_blocks_lo
     - 鈥滃潡鈥濊鏁扮殑浣?32 浣嶃€傚鏋滄枃浠剁郴缁熸湭璁剧疆 huge_file 鐗规€ф爣蹇楋紝鏂囦欢鍦ㄧ鐩樹笂娑堣€?       `i_blocks_lo` 涓?512 瀛楄妭鍧椼€傚鏋滆缃簡 huge_file 涓?`inode.i_flags` 涓湭璁剧疆
       EXT4_HUGE_FILE_FL锛屽垯鏂囦欢鍦ㄧ鐩樹笂娑堣€?``i_blocks_lo + (i_blocks_hi << 32)`` 涓?       512 瀛楄妭鍧椼€傚鏋滆缃簡 huge_file 涓?`inode.i_flags` 涓缃簡 EXT4_HUGE_FILE_FL锛屽垯鏂囦欢
       鍦ㄧ鐩樹笂娑堣€?(`i_blocks_lo + i_blocks_hi` << 32) 涓枃浠剁郴缁熷潡銆?   - - 0x20
     - __le32
     - i_flags
     - Inode 鏍囧織銆傚弬瑙佷笅鏂囩殑 i_flags_ 琛ㄣ€?   - - 0x24
     - 4 bytes
     - i_osd1
     - 鏇村缁嗚妭鍙傝 i_osd1_ 琛ㄣ€?   - - 0x28
     - 60 bytes
     - i_block[EXT4_N_BLOCKS=15]
     - 鍧楁槧灏勬垨鑼冨洿鏍戙€傚弬瑙佲€渋node.i_block 鐨勫唴瀹光€濆皬鑺傘€?   - - 0x64
     - __le32
     - i_generation
     - 鏂囦欢鐗堟湰锛堢敤浜?NFS锛夈€?   - - 0x68
     - __le32
     - i_file_acl_lo
     - 鎵╁睍灞炴€у潡鐨勪綆 32 浣嶃€侫CL 褰撶劧鏄紬澶氬彲鑳芥墿灞曞睘鎬т箣涓€锛涙瀛楁鐨勫悕绉版簮浜庢墿灞曞睘鎬ф渶鍒?       鐢ㄤ簬 ACL銆?   - - 0x6C
     - __le32
     - i_size_high / i_dir_acl
     - 鏂囦欢/鐩綍澶у皬鐨勯珮 32 浣嶃€傚湪 ext2/3 涓瀛楁鍚嶄负 i_dir_acl锛屽敖绠￠€氬父璁句负闆朵笖浠庢湭浣跨敤銆?   - - 0x70
     - __le32
     - i_obso_faddr
     - 锛堝凡搴熷純锛夌墖娈靛湴鍧€銆?   - - 0x74
     - 12 bytes
     - i_osd2
     - 鏇村缁嗚妭鍙傝 i_osd2_ 琛ㄣ€?   - - 0x80
     - __le16
     - i_extra_isize
     - 姝?inode 澶у皬鍑忓幓 128銆傛垨鑰咃紝鍘熷 ext2 inode 涔嬪鐨勬墿灞?inode 瀛楁澶у皬锛堝寘鎷瀛楁锛夈€?   - - 0x82
     - __le16
     - i_checksum_hi
     - inode 鏍￠獙鍜岀殑楂?16 浣嶃€?   - - 0x84
     - __le32
     - i_ctime_extra
     - 棰濆鐨勬洿鏀规椂闂翠綅銆傛彁渚涗簹绉掔骇绮惧害銆傚弬瑙?Inode 鏃堕棿鎴冲皬鑺傘€?   - - 0x88
     - __le32
     - i_mtime_extra
     - 棰濆鐨勪慨鏀规椂闂翠綅銆傛彁渚涗簹绉掔骇绮惧害銆?   - - 0x8C
     - __le32
     - i_atime_extra
     - 棰濆鐨勮闂椂闂翠綅銆傛彁渚涗簹绉掔骇绮惧害銆?   - - 0x90
     - __le32
     - i_crtime
     - 鏂囦欢鍒涘缓鏃堕棿锛岃嚜绾厓璧风殑绉掓暟銆?   - - 0x94
     - __le32
     - i_crtime_extra
     - 棰濆鐨勬枃浠跺垱寤烘椂闂翠綅銆傛彁渚涗簹绉掔骇绮惧害銆?   - - 0x98
     - __le32
     - i_version_hi
     - 鐗堟湰鍙风殑楂?32 浣嶃€?   - - 0x9C
     - __le32
     - i_projid
     - 椤圭洰 ID銆?

`i_mode` 鍊兼槸浠ヤ笅鏍囧織鐨勭粍鍚堬細

   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x1
     - S_IXOTH锛堝叾瀹冪敤鎴峰彲鎵ц锛?   - - 0x2
     - S_IWOTH锛堝叾瀹冪敤鎴峰彲鍐欙級
   - - 0x4
     - S_IROTH锛堝叾瀹冪敤鎴峰彲璇伙級
   - - 0x8
     - S_IXGRP锛堢粍鎴愬憳鍙墽琛岋級
   - - 0x10
     - S_IWGRP锛堢粍鎴愬憳鍙啓锛?   - - 0x20
     - S_IRGRP锛堢粍鎴愬憳鍙锛?   - - 0x40
     - S_IXUSR锛堟墍鏈夎€呭彲鎵ц锛?   - - 0x80
     - S_IWUSR锛堟墍鏈夎€呭彲鍐欙級
   - - 0x100
     - S_IRUSR锛堟墍鏈夎€呭彲璇伙級
   - - 0x200
     - S_ISVTX锛堢矘婊炰綅锛?   - - 0x400
     - S_ISGID锛堣缃?GID锛?   - - 0x800
     - S_ISUID锛堣缃?UID锛?   - -
     - 浠ヤ笅鏄簰鏂ョ殑鏂囦欢绫诲瀷锛?   - - 0x1000
     - S_IFIFO锛團IFO锛?   - - 0x2000
     - S_IFCHR锛堝瓧绗﹁澶囷級
   - - 0x4000
     - S_IFDIR锛堢洰褰曪級
   - - 0x6000
     - S_IFBLK锛堝潡璁惧锛?   - - 0x8000
     - S_IFREG锛堟櫘閫氭枃浠讹級
   - - 0xA000
     - S_IFLNK锛堢鍙烽摼鎺ワ級
   - - 0xC000
     - S_IFSOCK锛堝鎺ュ瓧锛?

`i_flags` 瀛楁鏄互涓嬪€肩殑缁勫悎锛?
   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x1
     - 姝ゆ枃浠堕渶瑕佸畨鍏ㄥ垹闄わ紙EXT4_SECRM_FL锛夈€傦紙鏈疄鐜帮級
   - - 0x2
     - 鑻ュ笇鏈涘弽鍒犻櫎锛屽簲淇濈暀姝ゆ枃浠讹紙EXT4_UNRM_FL锛夈€傦紙鏈疄鐜帮級
   - - 0x4
     - 鏂囦欢宸插帇缂╋紙EXT4_COMPR_FL锛夈€傦紙骞舵湭鐪熸瀹炰綔锛?   - - 0x8
     - 瀵规枃浠剁殑鎵€鏈夊啓鍏ュ繀椤绘槸鍚屾鐨勶紙EXT4_SYNC_FL锛夈€?   - - 0x10
     - 鏂囦欢涓嶅彲鍙橈紙EXT4_IMMUTABLE_FL锛夈€?   - - 0x20
     - 鏂囦欢鍙兘杩藉姞锛圗XT4_APPEND_FL锛夈€?   - - 0x40
     - dump(1) 宸ュ叿涓嶅簲杞偍姝ゆ枃浠讹紙EXT4_NODUMP_FL锛夈€?   - - 0x80
     - 涓嶆洿鏂拌闂椂闂达紙EXT4_NOATIME_FL锛夈€?   - - 0x100
     - 鑴忕殑宸插帇缂╂枃浠讹紙EXT4_DIRTY_FL锛夈€傦紙鏈娇鐢級
   - - 0x200
     - 鏂囦欢鍏锋湁涓€涓垨澶氫釜宸插帇缂╃皣锛圗XT4_COMPRBLK_FL锛夈€傦紙鏈娇鐢級
   - - 0x400
     - 涓嶅帇缂╂枃浠讹紙EXT4_NOCOMPR_FL锛夈€傦紙鏈娇鐢級
   - - 0x800
     - 宸插姞瀵嗙殑 inode锛圗XT4_ENCRYPT_FL锛夈€傛浣嶅€兼鍓嶄负 EXT4_ECOMPR_FL锛堝帇缂╅敊璇級锛屼粠鏈娇鐢ㄣ€?   - - 0x1000
     - 鐩綍鍏锋湁鍝堝笇绱㈠紩锛圗XT4_INDEX_FL锛夈€?   - - 0x2000
     - AFS 榄旀硶鐩綍锛圗XT4_IMAGIC_FL锛夈€?   - - 0x4000
     - 鏂囦欢鏁版嵁蹇呴』濮嬬粓閫氳繃鏃ュ織鍐欏叆锛圗XT4_JOURNAL_DATA_FL锛夈€?   - - 0x8000
     - 鏂囦欢灏鹃儴涓嶅簲鍚堝苟锛圗XT4_NOTAIL_FL锛夈€傦紙ext4 鏈娇鐢級
   - - 0x10000
     - 鎵€鏈夌洰褰曢」鏁版嵁搴斿悓姝ュ啓鍏ワ紙鍙傝 `dirsync`锛夛紙EXT4_DIRSYNC_FL锛夈€?   - - 0x20000
     - 鐩綍灞傜骇鐨勯《绔紙EXT4_TOPDIR_FL锛夈€?   - - 0x40000
     - 杩欐槸涓€涓ぇ鏂囦欢锛圗XT4_HUGE_FILE_FL锛夈€?   - - 0x80000
     - Inode 浣跨敤鑼冨洿锛坋xtents锛夛紙EXT4_EXTENTS_FL锛夈€?   - - 0x100000
     - 缁忚繃 verity 淇濇姢鏂囦欢锛圗XT4_VERITY_FL锛夈€?   - - 0x200000
     - Inode 鍦ㄥ叾鏁版嵁鍧椾腑瀛樺偍涓€涓ぇ鍨嬫墿灞曞睘鎬у€硷紙EXT4_EA_INODE_FL锛夈€?   - - 0x400000
     - 姝ゆ枃浠跺垎閰嶄簡瓒婅繃 EOF 鐨勫潡锛圗XT4_EOFBLOCKS_FL锛夈€傦紙宸插純鐢級
   - - 0x01000000
     - Inode 鏄揩鐓э紙`EXT4_SNAPFILE_FL`锛夈€傦紙涓嶅湪涓荤嚎涓級
   - - 0x04000000
     - 蹇収姝ｅ湪琚垹闄わ紙`EXT4_SNAPFILE_DELETED_FL`锛夈€傦紙涓嶅湪涓荤嚎涓級
   - - 0x08000000
     - 蹇収鏀剁缉宸插畬鎴愶紙`EXT4_SNAPFILE_SHRUNK_FL`锛夈€傦紙涓嶅湪涓荤嚎涓級
   - - 0x10000000
     - Inode 鍏锋湁鍐呰仈鏁版嵁锛圗XT4_INLINE_DATA_FL锛夈€?   - - 0x20000000
     - 浠ョ浉鍚岀殑椤圭洰 ID 鍒涘缓瀛愰」锛圗XT4_PROJINHERIT_FL锛夈€?   - - 0x40000000
     - 瀵圭洰褰曞唴瀹逛娇鐢ㄥぇ灏忓啓涓嶆晱鎰熺殑鏌ユ壘锛圗XT4_CASEFOLD_FL锛夈€?   - - 0x80000000
     - 淇濈暀缁?ext4 搴擄紙EXT4_RESERVED_FL锛夈€?   - -
     - 鑱氬悎鏍囧織锛?   - - 0x705BDFFF
     - 鐢ㄦ埛鍙鏍囧織銆?   - - 0x604BC0FF
     - 鐢ㄦ埛鍙慨鏀规爣蹇椼€傛敞鎰忥紝铏界劧 EXT4_JOURNAL_DATA_FL 鍜?EXT4_EXTENTS_FL 鍙互閫氳繃 setattr 璁剧疆锛?       浣嗗畠浠笉鍦ㄥ唴鏍哥殑 EXT4_FL_USER_MODIFIABLE 鎺╃爜涓紝鍥犱负鍐呮牳闇€瑕佷互鐗规畩鏂瑰紡澶勭悊杩欎簺鏍囧織鐨勮缃紝
       骞朵笖瀹冧滑琚粠鐩存帴淇濆瓨鍒?i_flags 鐨勬爣蹇楅泦鍚堜腑灞忚斀鎺夈€?

`osd1` 瀛楁鏍规嵁鍒涘缓鑰呯殑涓嶅悓鏈夊涓惈涔夛細

Linux锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - l_i_version
     - Inode 鐗堟湰銆備絾鏄紝鑻ヨ缃簡 EA_INODE inode 鏍囧織锛屾 inode 瀛樺偍涓€涓墿灞曞睘鎬у€硷紝璇ュ瓧娈靛寘鍚?       灞炴€у€煎紩鐢ㄨ鏁扮殑楂?32 浣嶃€?
Hurd锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - h_i_translator
     - ??

Masix锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - m_i_reserved
     - ??


`osd2` 瀛楁鏍规嵁鏂囦欢绯荤粺鍒涘缓鑰呯殑涓嶅悓鏈夊涓惈涔夛細

Linux锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - l_i_blocks_high
     - 鍧楄鏁扮殑楂?16 浣嶃€傝鍙傞槄闄勪簬 i_blocks_lo 鐨勮鏄庛€?   - - 0x2
     - __le16
     - l_i_file_acl_high
     - 鎵╁睍灞炴€у潡鐨勯珮 16 浣嶏紙鍘嗗彶涓婃寚鏂囦欢 ACL 浣嶇疆锛夈€傚弬瑙佷笅鏂囩殑鎵╁睍灞炴€у皬鑺傘€?   - - 0x4
     - __le16
     - l_i_uid_high
     - 鎵€鏈夎€?UID 鐨勯珮 16 浣嶃€?   - - 0x6
     - __le16
     - l_i_gid_high
     - GID 鐨勯珮 16 浣嶃€?   - - 0x8
     - __le16
     - l_i_checksum_lo
     - inode 鏍￠獙鍜岀殑浣?16 浣嶃€?   - - 0xA
     - __le16
     - l_i_reserved
     - 鏈娇鐢ㄣ€?
Hurd锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - h_i_reserved1
     - ??
   - - 0x2
     - __u16
     - h_i_mode_high
     - 鏂囦欢妯″紡鐨勯珮 16 浣嶃€?   - - 0x4
     - __le16
     - h_i_uid_high
     - 鎵€鏈夎€?UID 鐨勯珮 16 浣嶃€?   - - 0x6
     - __le16
     - h_i_gid_high
     - GID 鐨勯珮 16 浣嶃€?   - - 0x8
     - __u32
     - h_i_author
     - 浣滆€呬唬鐮侊紵

Masix锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - h_i_reserved1
     - ??
   - - 0x2
     - __u16
     - m_i_file_acl_high
     - 鎵╁睍灞炴€у潡鐨勯珮 16 浣嶏紙鍘嗗彶涓婃寚鏂囦欢 ACL 浣嶇疆锛夈€?   - - 0x4
     - __u32
     - m_i_reserved2[^2^]
     - ??

#### Inode 澶у皬

鍦?ext2 鍜?ext3 涓紝inode 缁撴瀯澶у皬鍥哄畾涓?128 瀛楄妭锛坄EXT2_GOOD_OLD_INODE_SIZE`锛夛紝
姣忎釜 inode 鐨勭鐩樿褰曞ぇ灏忎负 128 瀛楄妭銆備粠 ext4 寮€濮嬶紝鍙互鍦ㄦ牸寮忓寲鏃朵负鏂囦欢绯荤粺涓墍鏈?inode
鍒嗛厤涓€涓洿澶х殑纾佺洏 inode锛屼互鎻愪緵瓒呭嚭鍘熷 ext2 inode 鏈熬涔嬪鐨勭┖闂淬€傜鐩?inode 璁板綍澶у皬璁板綍浜?瓒呯骇鍧椾腑鐨?`s_inode_size`銆傞櫎鍘熷 128 瀛楄妭 ext2 inode 涔嬪锛宍struct ext4_inode` 瀹為檯浣跨敤鐨?瀛楄妭鏁拌褰曚簬姣忎釜 inode 鐨?`i_extra_isize` 瀛楁涓紝杩欎娇寰?`struct ext4_inode` 鑳藉涓烘柊鍐呮牳澧為暱锛?鑰屾棤闇€鍗囩骇鎵€鏈夌鐩樹笂鐨?inode銆傚瓒呭嚭 EXT2_GOOD_OLD_INODE_SIZE 鐨勫瓧娈电殑璁块棶锛屽簲楠岃瘉鍏朵綅浜?`i_extra_isize` 涔嬪唴銆傞粯璁ゆ儏鍐典笅锛宔xt4 inode 璁板綍涓?256 瀛楄妭锛屼笖锛堟埅鑷?2019 骞?8 鏈堬級inode 缁撴瀯涓?160 瀛楄妭锛坄i_extra_isize = 32`锛夈€俰node 缁撴瀯鏈熬涓?inode 璁板綍鏈熬涔嬮棿鐨勯澶栫┖闂村彲鐢ㄤ簬瀛樺偍
鎵╁睍灞炴€с€傛瘡涓?inode 璁板綍鏈€澶у彲杈炬枃浠剁郴缁熷潡澶у皬锛屽敖绠¤繖骞堕潪鐗瑰埆楂樻晥銆?
#### 鏌ユ壘涓€涓?Inode

姣忎釜鍧楃粍鍖呭惈 `sb->s_inodes_per_group` 涓?inode銆傜敱浜?inode 0 琚畾涔変负涓嶅瓨鍦紝鍙娇鐢ㄤ互涓嬪叕寮?鎵惧埌鏌愪釜 inode 鎵€鍦ㄧ殑鍧楃粍锛歚bg = (inode_num - 1) / sb->s_inodes_per_group`銆?鐗瑰畾鐨?inode 鍙湪鍧楃粍鐨?inode 琛ㄤ腑浜?`index = (inode_num - 1) % sb->s_inodes_per_group`
澶勬壘鍒般€傝鑾峰彇 inode 琛ㄤ腑鐨勫瓧鑺傚湴鍧€锛屼娇鐢?`offset = index * sb->s_inode_size`銆?
#### Inode 鏃堕棿鎴?
鍥涗釜鏃堕棿鎴宠褰曚簬 inode 缁撴瀯鐨勪綆 128 瀛楄妭涓€斺€攊node 鏇存敼鏃堕棿锛坈time锛夈€佽闂椂闂达紙atime锛夈€?鏁版嵁淇敼鏃堕棿锛坢time锛変互鍙婂垹闄ゆ椂闂达紙dtime锛夈€傝繖鍥涗釜瀛楁鏄?32 浣嶆湁绗﹀彿鏁存暟锛岃〃绀鸿嚜 Unix 绾厓
锛?970-01-01 00:00:00 GMT锛夎捣鐨勭鏁帮紝杩欐剰鍛崇潃杩欎簺瀛楁灏嗗湪 2038 骞?1 鏈堟孩鍑恒€傚鏋滄枃浠剁郴缁熸病鏈?orphan_file 鐗规€э紝閭ｄ簺鏈粠浠讳綍鐩綍閾炬帴浣嗕粛澶勪簬鎵撳紑鐘舵€佺殑 inode锛堝鍎?inode锛変細灏?dtime 瀛楁
閲嶈浇鐢ㄤ簬瀛ゅ効鍒楄〃銆傝秴绾у潡瀛楁 `s_last_orphan` 鎸囧悜瀛ゅ効鍒楄〃涓殑绗竴涓?inode锛沝time 闅忓悗鏄笅涓€涓?瀛ゅ効 inode 鐨勭紪鍙凤紝鑻ユ病鏈夋洿澶氬鍎垮垯涓?0銆?
濡傛灉 inode 缁撴瀯澶у皬 `sb->s_inode_size` 澶т簬 128 瀛楄妭锛屼笖 `i_inode_extra` 瀛楁瓒冲澶т互瀹圭撼
鐩稿簲鐨?`i_[cma]time_extra` 瀛楁锛屽垯 ctime銆乤time 鍜?mtime inode 瀛楁琚嫇瀹戒负 64 浣嶃€傚湪杩欎釜
鈥滈澶栤€濈殑 32 浣嶅瓧娈典腑锛屼綆 2 浣嶇敤浜庡皢 32 浣嶇瀛楁鎵╁睍涓?34 浣嶅锛涢珮 30 浣嶇敤浜庢彁渚涚撼绉掔骇鏃堕棿鎴崇簿搴︺€?鍥犳锛屾椂闂存埑鍦?2446 骞?5 鏈堜箣鍓嶄笉搴旀孩鍑恒€俤time 鏈鎷撳銆傝繕鏈変竴涓浜斾釜鏃堕棿鎴崇敤浜庤褰?inode 鍒涘缓鏃堕棿
锛坈rtime锛夛紱姝ゅ瓧娈典负 64 浣嶅锛屽苟浠ヤ笌 64 浣?[cma]time 鐩稿悓鐨勬柟寮忚В鐮併€俢rtime 鍜?dtime 閮芥棤娉曢€氳繃
甯歌鐨?stat() 鎺ュ彛璁块棶锛屼笉杩?debugfs 浼氭姤鍛婂畠浠€?
鎴戜滑浣跨敤 32 浣嶆湁绗﹀彿鏃堕棿鍊煎姞涓婏紙2^32 *锛堥澶栫邯鍏冧綅锛夛級銆傛崲瑷€涔嬶細

   :widths: 20 20 20 20 20
   :header-rows: 1

   - - Extra epoch bits
     - MSB of 32-bit time
     - Adjustment for signed 32-bit to 64-bit tv_sec
     - Decoded 64-bit tv_sec
     - valid time range
   - - 0 0
     - 1
     - 0
     - `-0x80000000 - -0x00000001`
     - 1901-12-13 鑷?1969-12-31
   - - 0 0
     - 0
     - 0
     - `0x000000000 - 0x07fffffff`
     - 1970-01-01 鑷?2038-01-19
   - - 0 1
     - 1
     - 0x100000000
     - `0x080000000 - 0x0ffffffff`
     - 2038-01-19 鑷?2106-02-07
   - - 0 1
     - 0
     - 0x100000000
     - `0x100000000 - 0x17fffffff`
     - 2106-02-07 鑷?2174-02-25
   - - 1 0
     - 1
     - 0x200000000
     - `0x180000000 - 0x1ffffffff`
     - 2174-02-25 鑷?2242-03-16
   - - 1 0
     - 0
     - 0x200000000
     - `0x200000000 - 0x27fffffff`
     - 2242-03-16 鑷?2310-04-04
   - - 1 1
     - 1
     - 0x300000000
     - `0x280000000 - 0x2ffffffff`
     - 2310-04-04 鑷?2378-04-22
   - - 1 1
     - 0
     - 0x300000000
     - `0x300000000 - 0x37fffffff`
     - 2378-04-22 鑷?2446-05-10

杩欐槸涓€绉嶆湁浜涘鎬殑缂栫爜锛屽洜涓烘鍊肩殑鏁伴噺瀹為檯涓婃槸璐熷€兼暟閲忕殑涓冨€嶃€傚浜?2038 骞翠箣鍚庣殑鏃ユ湡锛屼篃闀挎湡瀛樺湪
瑙ｇ爜鍜岀紪鐮佺殑 bug锛屾埅鑷冲唴鏍?3.12 涓?e2fsprogs 1.42.8 浼间箮浠嶆湭淇銆?4 浣嶅唴鏍搁敊璇湴浣跨敤棰濆绾厓浣?1,1 鏉ュ鐞?1901 鑷?1970 骞翠箣闂寸殑鏃ユ湡銆傚唴鏍哥粓灏嗚淇锛宔2fsck 涔熶細淇姝ょ姸鍐碉紝鍓嶆彁鏄畠鍦?2310 骞?涔嬪墠杩愯銆?