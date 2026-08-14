


######## ioctl MEDIA_IOC_ENUM_ENTITIES


## 鍚嶇О


MEDIA_IOC_ENUM_ENTITIES - 鏋氫妇瀹炰綋鍙婂叾灞炴€?
## 姒傝



`int ioctl(int fd, MEDIA_IOC_ENUM_ENTITIES, struct media_entity_desc *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 `media_entity_desc` 缁撴瀯浣撶殑鎸囬拡銆?
## 鎻忚堪


瑕佹煡璇㈡煇涓疄浣撶殑灞炴€э紝搴旂敤绋嬪簭闇€璁剧疆 `media_entity_desc` 缁撴瀯浣撶殑 id 瀛楁锛屽苟
浠ユ寚鍚戣缁撴瀯浣撶殑鎸囬拡璋冪敤 MEDIA_IOC_ENUM_ENTITIES ioctl銆傚綋 id 鏃犳晥鏃讹紝椹卞姩浼?濉厖缁撴瀯浣撶殑鍏朵綑閮ㄥ垎锛屾垨杩斿洖 EINVAL 閿欒鐮併€?

瀹炰綋鍙互閫氳繃灏?id 涓?`MEDIA_ENT_ID_FLAG_NEXT` 鏍囧織杩涜鎴栬繍绠楁潵鏋氫妇銆傞┍鍔ㄥ皢杩斿洖
id 涓ユ牸澶т簬鎵€璇锋眰 id 鐨勬渶灏忓疄浣撶殑淇℃伅锛堚€滀笅涓€涓疄浣撯€濓級锛涜嫢涓嶅瓨鍦紝鍒欒繑鍥?`EINVAL` 閿欒鐮併€?
瀹炰綋 ID 鍙互鏄潪杩炵画鐨勩€傚簲鐢ㄧ▼搴?*涓嶅緱**灏濊瘯閫氳繃浠ラ€掑鐨?id 涓嶆柇璋冪敤
MEDIA_IOC_ENUM_ENTITIES 鐩村埌杩斿洖閿欒鐨勬柟寮忔潵鏋氫妇瀹炰綋銆?


    :header-rows:  0
    :stub-columns: 0
    :widths: 2 2 1 8

    - -  __u32
       - `id`
       -
       - 瀹炰綋 ID锛岀敱搴旂敤绋嬪簭璁剧疆銆傚綋 ID 涓?`MEDIA_ENT_ID_FLAG_NEXT` 杩涜鎴栬繍绠楁椂锛?	  椹卞姩浼氭竻闄よ鏍囧織骞惰繑鍥?ID 鏇村ぇ鐨勭涓€涓疄浣撱€備笉瑕佹湡鏈涙瘡娆℃墦寮€璁惧瀹炰緥鏃?	  ID 閮界浉鍚屻€傛崲瑷€涔嬶紝涓嶈鍦ㄥ簲鐢ㄧ▼搴忎腑灏嗗疄浣?ID 纭紪鐮併€?
    - -  char
       - `name`\ [^32^]
       -
       - 浠?UTF-8 NULL 缁撳熬瀛楃涓茶〃绀虹殑瀹炰綋鍚嶇О銆傝鍚嶇О鍦ㄥ獟浣撴嫇鎵戝唴蹇呴』鍞竴銆?
    - -  __u32
       - `type`
       -
       - 瀹炰綋绫诲瀷锛岃瑙?media-entity-functions銆?
    - -  __u32
       - `revision`
       -
       - 瀹炰綋鐗堟湰鍙枫€傚缁堜负闆讹紙宸插簾寮冿級銆?
    - -  __u32
       - `flags`
       -
       - 瀹炰綋鏍囧織锛岃瑙?media-entity-flag銆?
    - -  __u32
       - `group_id`
       -
       - 瀹炰綋缁?ID銆傚缁堜负闆讹紙宸插簾寮冿級銆?
    - -  __u16
       - `pads`
       -
       - pad 鐨勬暟閲忋€?
    - -  __u16
       - `links`
       -
       - 鍑虹珯閾炬帴鐨勬€绘暟銆傚叆绔欓摼鎺ヤ笉璁″叆璇ュ瓧娈点€?
    - -  __u32
       - `reserved[^4^]`
       -
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃负闆躲€?
    - -  union {
       - (anonymous)

    - -  struct
       - `dev`
       -
       - 瀵瑰垱寤哄崟涓澶囪妭鐐圭殑锛堝瓙锛夎澶囨湁鏁堛€?
    - -
       - __u32
       - `major`
       - 璁惧鑺傜偣涓昏澶囧彿銆?
    - -
       - __u32
       - `minor`
       - 璁惧鑺傜偣娆¤澶囧彿銆?
    - -  __u8
       - `raw`\ [^184^]
#        -

    - - }
       -

## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    缁撴瀯浣?`media_entity_desc` 鐨?`id` 寮曠敤浜嗕竴涓笉瀛樺湪鐨勫疄浣撱€?
