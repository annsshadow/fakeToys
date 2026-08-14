
### Extended Attributes


鎵╁睍灞炴€э紙xattrs锛夐€氬父瀛樺偍鍦ㄧ鐩樹笂鐙珛鐨勬暟鎹潡涓紝骞堕€氳繃 `inode.i_file_acl*` 浠?inode 寮曠敤銆傛墿灞曞睘鎬х殑棣栨浣跨敤浼间箮鏄敤浜庡瓨鍌ㄦ枃浠?ACL 鍜屽叾浠栧畨鍏ㄦ暟鎹紙selinux锛夈€傚€熷姪 `user_xattr` 鎸傝浇閫夐」锛岀敤鎴峰彲浠ュ瓨鍌ㄦ墿灞曞睘鎬э紝鍙鎵€鏈夊睘鎬у悕閮戒互 鈥渦ser鈥?寮€澶达紱杩欎竴闄愬埗鍦?Linux 3.0 涔嬪悗浼间箮宸叉秷澶便€?
鎵╁睍灞炴€ф湁涓や釜瀛樻斁浣嶇疆銆傜涓€涓綅缃湪姣忎釜 inode 鏉＄洰鏈熬涓庝笅涓€涓?inode 鏉＄洰寮€澶翠箣闂淬€備緥濡傦紝鑻?inode.i_extra_isize = 28 涓?sb.inode_size = 256锛屽垯鏈?256 - (128 + 28) = 100 瀛楄妭鍙敤浜?inode 鍐呯殑鎵╁睍灞炴€у瓨鍌ㄣ€傛墿灞曞睘鎬у彲鎵惧埌鐨勭浜屼釜浣嶇疆鏄敱 `inode.i_file_acl` 鎸囧悜鐨勫潡涓€備粠 Linux 3.11 璧凤紝璇ュ潡鏃犳硶鍖呭惈鎸囧悜绗簩涓墿灞曞睘鎬у潡锛堢敋鑷充竴涓皣鐨勫墿浣欏潡锛夌殑鎸囬拡銆傜悊璁轰笂锛屾瘡涓睘鎬х殑鍊煎彲浠ュ瓨鍌ㄥ湪涓€涓嫭绔嬬殑鏁版嵁鍧椾腑锛屼絾浠?Linux 3.11 璧蜂唬鐮佷笉鍏佽杩欐牱鍋氥€?
閿紙key锛夐€氬父琚亣瀹氫负 ASCIIZ 瀛楃涓诧紝鑰屽€煎彲浠ユ槸瀛楃涓叉垨浜岃繘鍒舵暟鎹€?
鎵╁睍灞炴€у湪 inode 涔嬪悗瀛樺偍鏃讹紝鏈変竴涓?4 瀛楄妭闀跨殑澶?`ext4_xattr_ibody_header`锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - h_magic
     - 鐢ㄤ簬鏍囪瘑鐨勫够鏁帮紝0xEA020000銆傝鍊肩敱 Linux 椹卞姩璁剧疆锛屼笉杩?e2fsprogs 浼间箮涓嶆鏌ュ畠锛堬紵锛?
鎵╁睍灞炴€у潡鐨勮捣濮嬪鏄?`struct ext4_xattr_header`锛岄暱 32 瀛楄妭锛?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - h_magic
     - 鐢ㄤ簬鏍囪瘑鐨勫够鏁帮紝0xEA020000銆?   - - 0x4
     - __le32
     - h_refcount
     - 寮曠敤璁℃暟銆?   - - 0x8
     - __le32
     - h_blocks
     - 浣跨敤鐨勭鐩樺潡鏁般€?   - - 0xC
     - __le32
     - h_hash
     - 鎵€鏈夊睘鎬х殑鍝堝笇鍊笺€?   - - 0x10
     - __le32
     - h_checksum
     - 鎵╁睍灞炴€у潡鐨勬牎楠屽拰銆?   - - 0x14
     - __u32
     - h_reserved[^3^]
     - 闆躲€?
鏍￠獙鍜屾槸閽堝 FS UUID銆佹墿灞曞睘鎬у潡鐨?64 浣嶅潡鍙蜂互鍙婃暣涓潡锛堝ご + 鏉＄洰锛夎绠楃殑銆?
鍦?`struct ext4_xattr_header` 鎴?`struct ext4_xattr_ibody_header` 涔嬪悗鏄竴涓?`struct ext4_xattr_entry` 鏁扮粍锛涙瘡涓潯鐩嚦灏戦暱 16 瀛楄妭銆傚綋瀛樺偍鍦ㄥ閮ㄥ潡涓椂锛宍struct ext4_xattr_entry` 鏉＄洰蹇呴』鎸夋帓搴忛『搴忓瓨鍌ㄣ€傛帓搴忛『搴忎负 `e_name_index`锛岀劧鍚庢槸 `e_name_len`锛屾渶鍚庢槸 `e_name`銆傚瓨鍌ㄥ湪 inode 鍐呯殑灞炴€т笉闇€瑕佹寜鎺掑簭椤哄簭瀛樺偍銆?
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __u8
     - e_name_len
     - 鍚嶇О闀垮害銆?   - - 0x1
     - __u8
     - e_name_index
     - 灞炴€у悕绱㈠紩銆備笅鏂囨湁鐩稿叧璁ㄨ銆?   - - 0x2
     - __le16
     - e_value_offs
     - 璇ュ睘鎬х殑鍊煎湪瀛樺偍瀹冪殑纾佺洏鍧椾笂鐨勪綅缃€傚涓睘鎬у彲浠ュ叡浜悓涓€涓€笺€傚浜?inode 灞炴€э紝璇ュ€肩浉瀵逛簬绗竴涓潯鐩殑璧峰锛涘浜庡潡锛岃鍊肩浉瀵逛簬鍧楃殑璧峰锛堝嵆澶达級銆?   - - 0x4
     - __le32
     - e_value_inum
     - 鍊兼墍瀛樺偍鐨?inode銆傞浂琛ㄧず璇ュ€间笌鏈潯鐩湪鍚屼竴鍧椾腑銆傝瀛楁浠呭湪鍚敤 INCOMPAT_EA_INODE 鐗规€ф椂浣跨敤銆?   - - 0x8
     - __le32
     - e_value_size
     - 灞炴€у€肩殑闀垮害銆?   - - 0xC
     - __le32
     - e_hash
     - 灞炴€у悕涓庡睘鎬у€肩殑鍝堝笇鍊笺€傚唴鏍镐笉浼氫负 inode 鍐呭睘鎬ф洿鏂板搱甯岋紝鍥犳瀵逛簬杩欑鎯呭喌璇ュ€煎繀椤讳负闆讹紝鍥犱负 e2fsck 浼氭牎楠屼换浣曢潪闆跺搱甯岋紝鏃犺 xattr 浣嶄簬浣曞銆?   - - 0x10
     - char
     - e_name[e_name_len]
     - 灞炴€у悕銆備笉鍖呭惈缁撳熬鐨?NULL銆?
灞炴€у€煎彲浠ヨ窡鍦ㄦ潯鐩〃鏈熬涔嬪悗銆備技涔庤姹傚畠浠寜 4 瀛楄妭杈圭晫瀵归綈銆傝繖浜涘€间粠鍧楃殑鏈熬寮€濮嬪瓨鍌紝骞跺悜 xattr_header/xattr_entry 琛ㄦ柟鍚戝闀裤€傚綋涓よ€呯浉鎾炴椂锛屾孩鍑洪儴鍒嗚鏀惧叆涓€涓嫭绔嬬殑纾佺洏鍧椼€傚鏋滅鐩樺潡濉弧锛屾枃浠剁郴缁熻繑鍥?-ENOSPC銆?
`ext4_xattr_entry` 鐨勫墠鍥涗釜瀛楁琚疆闆朵互鏍囪閿垪琛ㄧ殑缁撴潫銆?
#### Attribute Name Indices


浠庨€昏緫涓婅锛屾墿灞曞睘鎬ф槸涓€绯诲垪 key=value 瀵广€傞敭琚亣瀹氫负浠?NULL 缁撳熬鐨勫瓧绗︿覆銆備负浜嗗噺灏戦敭鍦ㄧ鐩樹笂鍗犵敤鐨勭┖闂达紝閿瓧绗︿覆鐨勫紑澶翠細涓庡睘鎬у悕绱㈠紩杩涜鍖归厤銆傚鏋滄壘鍒板尮閰嶏紝鍒欒缃睘鎬у悕绱㈠紩瀛楁锛屽苟浠庨敭鍚嶄腑鍘绘帀鍖归厤鍒扮殑瀛楃涓层€備笅闈㈡槸鍚嶇О绱㈠紩鍊煎埌閿墠缂€鐨勬槧灏勶細

   :widths: 16 64
   :header-rows: 1

   - - Name Index
     - Key Prefix
   - - 0
     - 锛堟棤鍓嶇紑锛?   - - 1
     - 鈥渦ser.鈥?   - - 2
     - 鈥渟ystem.posix_acl_access鈥?   - - 3
     - 鈥渟ystem.posix_acl_default鈥?   - - 4
     - 鈥渢rusted.鈥?   - - 6
     - 鈥渟ecurity.鈥?   - - 7
     - 鈥渟ystem.鈥濓紙浠?inline_data锛燂級
   - - 8
     - 鈥渟ystem.richacl鈥濓紙浠?SuSE 鍐呮牳锛燂級

渚嬪锛屽鏋滃睘鎬ч敭鏄?鈥渦ser.fubar鈥濓紝鍒欏睘鎬у悕绱㈠紩琚涓?1锛屽苟涓?鈥渇ubar鈥?鍚嶇О琚褰曞埌纾佺洏涓娿€?
#### POSIX ACLs


POSIX ACL 浠?Linux 鍐呮牳锛堝強 libacl锛夊唴閮?ACL 鏍煎紡鐨勭簿绠€鐗堟湰瀛樺偍銆傚叧閿尯鍒湪浜庣増鏈彿涓嶅悓锛?锛夛紝骞朵笖 `e_id` 瀛楁浠呬负鍏峰悕鐢ㄦ埛鍜岀粍 ACL 瀛樺偍銆?