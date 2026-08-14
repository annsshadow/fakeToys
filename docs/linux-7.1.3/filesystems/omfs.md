## 浼樺寲鐨?MPEG 鏂囦欢绯荤粺锛圤MFS锛?


## 姒傝堪


OMFS 鏄敱 SonicBlue 鍒涘缓鐨勬枃浠剁郴缁燂紝鐢ㄤ簬 ReplayTV DVR 鍜?Rio Karma MP3 鎾斁鍣ㄣ€傝鏂囦欢绯荤粺鏄熀浜庡尯娈碉紙extent锛夌殑锛屼娇鐢?2k 鍒?8k 鐨勫潡澶у皬锛屽苟閲囩敤鍩轰簬鍝堝笇鐨勭洰褰曘€傛鏂囦欢绯荤粺椹卞姩鍙敤浜庤鍐欐潵鑷繖浜涜澶囩殑纾佺洏銆?

娉ㄦ剰锛屼笉寤鸿鐢ㄦ鏂囦欢绯荤粺鏇夸唬閫氱敤鏂囦欢绯荤粺鐢ㄤ簬浣犺嚜宸辩殑娴佸獟浣撹澶囥€傚師鐢熺殑 Linux 鏂囦欢绯荤粺鍙兘浼氳〃鐜版洿濂姐€?

鏇村淇℃伅瑙侊細

    http://linux-karma.sf.net/

鍚勭瀹炵敤宸ュ叿锛屽寘鎷?mkomfs 鍜?omfsck锛岄殢 omfsprogs 涓€璧锋彁渚涳紝鍙湪浠ヤ笅鍦板潃鑾峰彇锛?

    https://bobcopeland.com/karma/

鍏?README 涓寘鍚鏄庛€?

## 閫夐」


OMFS 鏀寔浠ヤ笅鎸傝浇鏃堕€夐」锛?

    ============   ========================================
    uid=n          浣挎墍鏈夋枃浠跺綊鎸囧畾鐢ㄦ埛鎵€鏈?
    gid=n          浣挎墍鏈夋枃浠跺綊鎸囧畾缁勬墍鏈?
    umask=xxx      灏嗘潈闄?umask 璁剧疆涓?xxx
    fmask=xxx      涓烘枃浠跺皢 umask 璁剧疆涓?xxx
    dmask=xxx      涓虹洰褰曞皢 umask 璁剧疆涓?xxx
    ============   ========================================

## 纾佺洏鏍煎紡


OMFS 鍖哄垎鈥渟ysblock鈥濆拰鏅€氭暟鎹潡銆俿ysblock 缁勭敱瓒呯骇鍧椾俊鎭€佹枃浠跺厓鏁版嵁銆佺洰褰曠粨鏋勫拰鍖烘缁勬垚銆傛瘡涓?sysblock 閮芥湁涓€涓ご閮紝鍖呭惈鏁翠釜 sysblock 鐨?CRC锛屽苟涓斿彲鑳藉湪纾佺洏涓婄殑杩炵画鍧椾腑杩涜闀滃儚銆俿ysblock 鐨勫ぇ灏忓彲鑳藉皬浜庢暟鎹潡锛屼絾鐢变簬浜岃€呴兘鐢辩浉鍚岀殑 64 浣嶅潡鍙峰鍧€锛岃緝灏?sysblock 涓殑浠讳綍鍓╀綑绌洪棿閮芥湭琚娇鐢ㄣ€?

```

    struct omfs_header {
	    __be64 h_self;                  /* FS block where this is located */
	    __be32 h_body_size;             /* size of useful data after header */
	    __be16 h_crc;                   /* crc-ccitt of body_size bytes */
	    char h_fill1[2];
	    u8 h_version;                   /* version, always 1 */
	    char h_type;                    /* OMFS_INODE_X */
	    u8 h_magic;                     /* OMFS_IMAGIC */
	    u8 h_check_xor;                 /* XOR of header bytes before this */
	    __be32 h_fill2;
    };

```
```

    struct omfs_inode {
	    struct omfs_header i_head;      /* header */
	    __be64 i_parent;                /* parent containing this inode */
	    __be64 i_sibling;               /* next inode in hash bucket */
	    __be64 i_ctime;                 /* ctime, in milliseconds */
	    char i_fill1[35];
	    char i_type;                    /* OMFS_[DIR,FILE] */
	    __be32 i_fill2;
	    char i_fill3[64];
	    char i_name[OMFS_NAMELEN];      /* filename */
	    __be64 i_size;                  /* size of file, in bytes */
    };

```
OMFS 涓殑鐩綍瀹炵幇涓轰竴涓ぇ鍨嬪搱甯岃〃銆傛枃浠跺悕琚搱甯屽悗锛屼粠 OMFS_DIR_START 寮€濮嬫彃鍏ュ埌妗跺垪琛ㄤ腑銆傛煡鎵鹃渶瑕佸搱甯屾枃浠跺悕锛岀劧鍚庨亶鍘?i_sibling 鎸囬拡锛岀洿鍒板湪 i_name 涓婃壘鍒板尮閰嶃€傜┖妗剁敱鍏ㄤ负 1锛垀0锛夌殑鍧楁寚閽堣〃绀恒€?

涓€涓枃浠舵槸涓€涓?omfs_inode 缁撴瀯锛屽叾鍚庤窡鐫€涓€涓粠 ```

    struct omfs_extent_entry {
	    __be64 e_cluster;               /* start location of a set of blocks */
	    __be64 e_blocks;                /* number of blocks after e_cluster */
    };

    struct omfs_extent {
	    __be64 e_next;                  /* next extent table location */
	    __be32 e_extent_count;          /* total # extents in this table */
	    __be32 e_fill;
	    struct omfs_extent_entry e_entry;       /* start of extent entries */
    };

寮€濮嬬殑鍖烘琛ㄣ€傛瘡涓尯娈典繚瀛樺潡鍋忕Щ锛屽悗璺熷垎閰嶇粰璇ュ尯娈电殑鍧楁暟銆傛瘡涓〃涓殑鏈€鍚庝竴涓尯娈垫槸涓€涓粓姝㈢锛屽叾 e_cluster 涓?~0锛宔_blocks 涓鸿〃涓尯鍧楁€绘暟鐨勫弽鐮併€?

濡傛灉璇ヨ〃婧㈠嚭锛屼細鍐欏叆涓€涓欢缁?inode锛屽苟鐢?e_next 鎸囧悜銆傝繖浜涘欢缁?inode 鏈夊ご閮紝浣嗙己灏?inode 缁撴瀯鐨勫叾浣欓儴鍒嗐€?
