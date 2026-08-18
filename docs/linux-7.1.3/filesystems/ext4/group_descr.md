
### Block Group Descriptors


鏂囦欢绯荤粺涓殑姣忎釜鍧楃粍锛坆lock group锛夐兘鍏宠仈鏈変竴涓繖鏍风殑鎻忚堪绗︺€傛濡備笂鏂?甯冨眬"涓€鑺傛墍杩帮紝缁勬弿杩扮锛堝鏋滃瓨鍦級鏄潡缁勪腑鐨勭浜岄」銆傛爣鍑嗛厤缃笅锛屾瘡涓潡缁勯兘鍖呭惈涓€浠藉畬鏁寸殑鍧楃粍鎻忚堪绗﹁〃鍓湰锛岄櫎闈炶缃簡 sparse_super 鐗规€ф爣蹇椼€?
娉ㄦ剰缁勬弿杩扮璁板綍浜嗕袱涓綅鍥句互鍙?inode 琛ㄧ殑浣嶇疆锛堝嵆瀹冧滑鍙互娴姩锛夈€傝繖鎰忓懗鐫€鍦ㄥ潡缁勫唴锛屽叿鏈夊浐瀹氫綅缃殑鍞竴鏁版嵁缁撴瀯鏄秴绾у潡锛坰uperblock锛夊拰缁勬弿杩扮琛ㄣ€俧lex_bg 鏈哄埗鍒╃敤杩欎竴鐗规€у皢鑻ュ共涓潡缁勫綊鍏ヤ竴涓?flex group锛屽苟鎶婃墍鏈夎繖浜涚粍鐨勪綅鍥惧拰 inode 琛ㄥ竷灞€鍒?flex group 绗竴涓粍涓殑涓€娈佃繛缁尯鍩熼噷銆?
濡傛灉璁剧疆浜?meta_bg 鐗规€ф爣蹇楋紝鍒欒嫢骞蹭釜鍧楃粍浼氳褰掑叆涓€涓?meta group銆傞渶瑕佹敞鎰忕殑鏄紝鍦?meta_bg 鐨勬儏鍐典笅锛岃緝澶х殑 meta group 涓涓€涓拰鏈€鍚庝袱涓潡缁勫彧鍖呭惈璇?meta group 鍐呴儴鍚勭粍鐨勭粍鎻忚堪绗︺€?
flex_bg 鍜?meta_bg 鐪嬭捣鏉ュ苟闈炰簰鏂ョ殑鐗规€с€?
鍦?ext2銆乪xt3 浠ュ強 ext4锛堟湭鍚敤 64bit 鐗规€ф椂锛変腑锛屽潡缁勬弿杩扮鍙湁 32 瀛楄妭闀匡紝鍥犳鍒?bg_checksum 澶勭粨鏉熴€傚湪鍚敤浜?64bit 鐗规€х殑 ext4 鏂囦欢绯荤粺涓婏紝鍧楃粍鎻忚堪绗﹁嚦灏戞墿灞曞埌涓嬮潰鎻忚堪鐨?64 瀛楄妭锛涘叾澶у皬瀛樺偍鍦ㄨ秴绾у潡涓€?
濡傛灉璁剧疆浜?gdt_csum 鑰屾湭璁剧疆 metadata_csum锛屽垯鍧楃粍鏍￠獙鍜屾槸 FS UUID銆佺粍鍙蜂互鍙婄粍鎻忚堪绗︾粨鏋勭殑 crc16銆傚鏋滆缃簡 metadata_csum锛屽垯鍧楃粍鏍￠獙鍜屾槸 FS UUID銆佺粍鍙蜂互鍙婄粍鎻忚堪绗︾粨鏋勬牎楠屽拰鐨勭 16 浣嶃€傚潡浣嶅浘鍜?inode 浣嶅浘鐨勬牎楠屽拰閮芥槸閽堝 FS UUID銆佺粍鍙蜂互鍙婃暣涓綅鍥捐绠楃殑銆?
鍧楃粍鎻忚堪绗︿互 `struct ext4_group_desc` 甯冨眬銆?
   :widths: 8 8 24 40
   :header-rows: 1

   - - 鍋忕Щ锛圤ffset锛?     - 澶у皬锛圫ize锛?     - 鍚嶇О锛圢ame锛?     - 鎻忚堪锛圖escription锛?   - - 0x0
     - __le32
     - bg_block_bitmap_lo
     - 鍧椾綅鍥句綅缃殑浣?32 浣嶃€?   - - 0x4
     - __le32
     - bg_inode_bitmap_lo
     - inode 浣嶅浘浣嶇疆鐨勪綆 32 浣嶃€?   - - 0x8
     - __le32
     - bg_inode_table_lo
     - inode 琛ㄤ綅缃殑浣?32 浣嶃€?   - - 0xC
     - __le16
     - bg_free_blocks_count_lo
     - 绌洪棽鍧楄鏁扮殑浣?16 浣嶃€?   - - 0xE
     - __le16
     - bg_free_inodes_count_lo
     - 绌洪棽 inode 璁℃暟鐨勪綆 16 浣嶃€?   - - 0x10
     - __le16
     - bg_used_dirs_count_lo
     - 鐩綍璁℃暟鐨勪綆 16 浣嶃€?   - - 0x12
     - __le16
     - bg_flags
     - 鍧楃粍鏍囧織銆傚弬瑙佷笅鏂囩殑 bgflags_ 琛ㄣ€?   - - 0x14
     - __le32
     - bg_exclude_bitmap_lo
     - 蹇収鎺掗櫎浣嶅浘浣嶇疆鐨勪綆 32 浣嶃€?   - - 0x18
     - __le16
     - bg_block_bitmap_csum_lo
     - 鍧椾綅鍥炬牎楠屽拰鐨勪綆 16 浣嶃€?   - - 0x1A
     - __le16
     - bg_inode_bitmap_csum_lo
     - inode 浣嶅浘鏍￠獙鍜岀殑浣?16 浣嶃€?   - - 0x1C
     - __le16
     - bg_itable_unused_lo
     - 鏈娇鐢?inode 璁℃暟鐨勪綆 16 浣嶃€傝嫢璁剧疆锛屽垯鏃犻渶鎵弿璇ョ粍 inode 琛ㄤ腑
       `(sb.s_inodes_per_group - gdt.bg_itable_unused)` 涔嬪悗鐨勬潯鐩€?   - - 0x1E
     - __le16
     - bg_checksum
     - 缁勬弿杩扮鏍￠獙鍜岋紱鑻ヨ缃簡 RO_COMPAT_GDT_CSUM 鐗规€э紝鍒欎负
       crc16(sb_uuid+group_num+bg_desc)锛屾垨鑰呰嫢璁剧疆浜?       RO_COMPAT_METADATA_CSUM 鐗规€э紝鍒欎负 crc32c(sb_uuid+group_num+bg_desc) & 0xFFFF銆?       璁＄畻 crc16 鏍￠獙鍜屾椂浼氳烦杩?bg_desc 涓殑 bg_checksum 瀛楁锛?       鑻ヤ娇鐢?crc32c 鏍￠獙鍜屽垯灏嗗叾缃负闆躲€?#    * -

     -
     - 浠ヤ笅瀛楁浠呭湪鍚敤 64bit 鐗规€т笖 s_desc_size > 32 鏃跺瓨鍦ㄣ€?   - - 0x20
     - __le32
     - bg_block_bitmap_hi
     - 鍧椾綅鍥句綅缃殑楂?32 浣嶃€?   - - 0x24
     - __le32
     - bg_inode_bitmap_hi
     - inode 浣嶅浘浣嶇疆鐨勯珮 32 浣嶃€?   - - 0x28
     - __le32
     - bg_inode_table_hi
     - inode 琛ㄤ綅缃殑楂?32 浣嶃€?   - - 0x2C
     - __le16
     - bg_free_blocks_count_hi
     - 绌洪棽鍧楄鏁扮殑楂?16 浣嶃€?   - - 0x2E
     - __le16
     - bg_free_inodes_count_hi
     - 绌洪棽 inode 璁℃暟鐨勯珮 16 浣嶃€?   - - 0x30
     - __le16
     - bg_used_dirs_count_hi
     - 鐩綍璁℃暟鐨勯珮 16 浣嶃€?   - - 0x32
     - __le16
     - bg_itable_unused_hi
     - 鏈娇鐢?inode 璁℃暟鐨勯珮 16 浣嶃€?   - - 0x34
     - __le32
     - bg_exclude_bitmap_hi
     - 蹇収鎺掗櫎浣嶅浘浣嶇疆鐨勯珮 32 浣嶃€?   - - 0x38
     - __le16
     - bg_block_bitmap_csum_hi
     - 鍧椾綅鍥炬牎楠屽拰鐨勯珮 16 浣嶃€?   - - 0x3A
     - __le16
     - bg_inode_bitmap_csum_hi
     - inode 浣嶅浘鏍￠獙鍜岀殑楂?16 浣嶃€?   - - 0x3C
     - __u32
     - bg_reserved
     - 濉厖鑷?64 瀛楄妭銆?

鍧楃粍鏍囧織鍙互鏄互涓嬩换鎰忕粍鍚堬細

   :widths: 16 64
   :header-rows: 1

   - - 鍊硷紙Value锛?     - 鎻忚堪锛圖escription锛?   - - 0x1
     - inode 琛ㄥ拰浣嶅浘鏈垵濮嬪寲锛圗XT4_BG_INODE_UNINIT锛夈€?   - - 0x2
     - 鍧椾綅鍥炬湭鍒濆鍖栵紙EXT4_BG_BLOCK_UNINIT锛夈€?   - - 0x4
     - inode 琛ㄥ凡娓呴浂锛圗XT4_BG_INODE_ZEROED锛夈€?