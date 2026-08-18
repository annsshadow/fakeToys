


######## ioctl MEDIA_IOC_ENUM_LINKS


## 鍚嶇О


MEDIA_IOC_ENUM_LINKS - 鏋氫妇缁欏畾瀹炰綋鐨勬墍鏈?pad 鍜岄摼鎺?
## 姒傝


`int ioctl(int fd, MEDIA_IOC_ENUM_LINKS, struct media_links_enum *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `media_links_enum` 鐨勬寚閽堛€?
## 鎻忚堪


涓烘灇涓剧粰瀹氬疄浣撶殑 pad 鍜?鎴栭摼鎺ワ紝搴旂敤绋嬪簭璁剧疆 struct `media_links_enum` 鐨?entity 瀛楁锛屽苟鍒濆鍖栫敱 `pads` 鍜?`links` 瀛楁鎸囧悜鐨?struct `media_pad_desc` 鍜?struct `media_link_desc` 缁撴瀯鏁扮粍銆傜劧鍚庡畠浠互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 MEDIA_IOC_ENUM_LINKS ioctl銆?
濡傛灉 `pads` 瀛楁闈?NULL锛岄┍鍔ㄤ細鐢ㄥ叧浜庤瀹炰綋 pad 鐨勪俊鎭～鍏?`pads` 鏁扮粍銆傝鏁扮粍蹇呴』鏈夎冻澶熺┖闂村瓨鍌ㄨ瀹炰綋鐨勬墍鏈?pad銆俻ad 鐨勬暟閲忓彲閫氳繃 MEDIA_IOC_ENUM_ENTITIES 鑾峰彇銆?
濡傛灉 `links` 瀛楁闈?NULL锛岄┍鍔ㄤ細鐢ㄥ叧浜庤瀹炰綋鍑虹珯閾炬帴鐨勪俊鎭～鍏?`links` 鏁扮粍銆傝鏁扮粍蹇呴』鏈夎冻澶熺┖闂村瓨鍌ㄨ瀹炰綋鐨勬墍鏈夊嚭绔欓摼鎺ャ€傚嚭绔欓摼鎺ョ殑鏁伴噺鍙€氳繃 MEDIA_IOC_ENUM_ENTITIES 鑾峰彇銆?
鍦ㄦ灇涓捐繃绋嬩腑锛屼粎杩斿洖璧锋簮浜庤瀹炰綋鏌愪釜 source pad 鐨勫墠鍚戦摼鎺ャ€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  __u32
       - `entity`
       - 瀹炰綋 id锛岀敱搴旂敤绋嬪簭璁剧疆銆?
    - -  struct `media_pad_desc`
       - \*\ `pads`
       - 鎸囧悜鐢卞簲鐢ㄧ▼搴忓垎閰嶇殑 pads 鏁扮粍鐨勬寚閽堛€傝嫢涓?NULL 鍒欏拷鐣ャ€?
    - -  struct `media_link_desc`
       - \*\ `links`
       - 鎸囧悜鐢卞簲鐢ㄧ▼搴忓垎閰嶇殑 links 鏁扮粍鐨勬寚閽堛€傝嫢涓?NULL 鍒欏拷鐣ャ€?
    - -  __u32
       - `reserved[^4^]`
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  __u32
       - `entity`
       - 姝?pad 鎵€灞炲疄浣撶殑 ID銆?
    - -  __u16
       - `index`
       - pad 绱㈠紩锛屼粠 0 寮€濮嬨€?
    - -  __u32
       - `flags`
       - pad 鏍囧織锛岃瑙?media-pad-flag銆?
    - -  __u32
       - `reserved[^2^]`
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  struct `media_pad_desc`
       - `source`
       - 姝ら摼鎺ヨ捣鐐圭殑 pad銆?
    - -  struct `media_pad_desc`
       - `sink`
       - 姝ら摼鎺ョ洰鏍囩殑 pad銆?
    - -  __u32
       - `flags`
       - 閾炬帴鏍囧織锛岃瑙?media-link-flag銆?
    - -  __u32
       - `reserved[^2^]`
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `media_links_enum` 鐨?`id` 寮曠敤浜嗕笉瀛樺湪鐨勫疄浣撱€?