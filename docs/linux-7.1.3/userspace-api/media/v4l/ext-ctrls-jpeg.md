
######## JPEG 鎺у埗鍙傝€?

JPEG 绫诲寘鍚?JPEG 缂栫爜鍣ㄤ笌瑙ｇ爜鍣ㄩ€氱敤鐗规€х殑鎺т欢銆傜洰鍓嶅畠鍖呭惈瀹炵幇浜嗕娇鐢?Huffman 鐔电紪鐮佺殑
娓愯繘寮忓熀绾?DCT 鍘嬬缉杩囩▼鐨勭紪瑙ｇ爜鍣ㄧ壒鎬с€?
## JPEG 鎺т欢 ID


`V4L2_CID_JPEG_CLASS (class)`
    JPEG 绫绘弿杩扮銆傚姝ゆ帶浠惰皟鐢?VIDIOC_QUERYCTRL 灏嗚繑鍥炶鎺т欢绫荤殑鎻忚堪銆?
`V4L2_CID_JPEG_CHROMA_SUBSAMPLING (menu)`
    鑹插害瀛愰噰鏍峰洜瀛愭弿杩拌緭鍏ュ浘鍍忕殑姣忎釜鍒嗛噺濡備綍琚噰鏍凤紝鐩稿浜庢瘡涓┖闂寸淮搴︿腑鐨勬渶澶ч噰鏍风巼銆?    鏇村缁嗚妭璇峰弬瑙?itu-t81锛岀 A.1.1 鑺傘€俙V4L2_CID_JPEG_CHROMA_SUBSAMPLING` 鎺т欢鍐冲畾
    鍦ㄥ皢杈撳叆鍥惧儚浠?RGB 杞崲鍒?Y'CbCr 鑹插僵绌洪棿鍚庯紝Cb 涓?Cr 鍒嗛噺濡備綍琚笅閲囨牱銆?
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_444`
      - 鏃犺壊搴﹀瓙閲囨牱锛屾瘡涓儚绱犻兘鏈?Y銆丆r 涓?Cb 鍊笺€?    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_422`
      - 瀵?Cr銆丆b 鍒嗛噺鎸夊洜瀛?2 姘村钩瀛愰噰鏍枫€?    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_420`
      - 瀵?Cr銆丆b 鍒嗛噺姘村钩涓庡瀭鐩村悇瀛愰噰鏍?2 鍊嶃€?    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_411`
      - 瀵?Cr銆丆b 鍒嗛噺鎸夊洜瀛?4 姘村钩瀛愰噰鏍枫€?    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_410`
      - 瀵?Cr銆丆b 鍒嗛噺姘村钩瀛愰噰鏍?4 鍊嶃€佸瀭鐩村瓙閲囨牱 2 鍊嶃€?    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_GRAY`
      - 浠呬娇鐢ㄤ寒搴﹀垎閲忋€?
`V4L2_CID_JPEG_RESTART_INTERVAL (integer)`
    閲嶅惎闂撮殧鍐冲畾浜嗘彃鍏?RSTm 鏍囪锛坢 = 0..7锛夌殑闂撮殧銆傝繖浜涙爣璁扮殑鐩殑鏄澶栧湴閲嶆柊鍒濆鍖?    缂栫爜鍣ㄨ繃绋嬶紝浠ヤ究鐙珛鍦板鐞嗗浘鍍忕殑鍧椼€傚浜庢棤鎹熷帇缂╄繃绋嬶紝閲嶅惎闂撮殧鐨勫崟浣嶆槸 MCU锛堟渶灏忕紪鐮佸崟鍏冿級锛?    鍏跺€煎寘鍚湪 DRI锛圖efine Restart Interval锛夋爣璁颁腑銆傚鏋?`V4L2_CID_JPEG_RESTART_INTERVAL`
    鎺т欢璁句负 0锛屽垯涓嶄細鎻掑叆 DRI 涓?RSTm 鏍囪銆?
`V4L2_CID_JPEG_COMPRESSION_QUALITY (integer)`
    鍐冲畾鍥惧儚璐ㄩ噺涓庡ぇ灏忎箣闂寸殑鏉冭　銆傚畠涓哄簲鐢ㄧ▼搴忔彁渚涗簡涓€绉嶆洿绠€鍗曠殑鎺у埗鍥惧儚璐ㄩ噺鐨勬柟娉曪紝
    鑰屾棤闇€鐩存帴閲嶆柊閰嶇疆浜害涓庤壊搴﹂噺鍖栬〃銆傚湪椹卞姩浣跨敤鐢卞簲鐢ㄧ▼搴忛€氳繃鍏朵粬瀹氫箟鐨勬帴鍙ｇ洿鎺ラ厤缃殑
    閲忓寲琛ㄧ殑鎯呭喌涓嬶紝`V4L2_CID_JPEG_COMPRESSION_QUALITY` 鎺т欢搴旂敱椹卞姩璁句负 0銆?
    姝ゆ帶浠剁殑鍙栧€艰寖鍥寸敱椹卞姩鍐冲畾銆傚彧鏈夋鐨勩€侀潪闆跺€兼墠鏈夋剰涔夈€傛帹鑽愯寖鍥翠负 1 - 100锛屽叾涓緝澶х殑
    鍊煎搴旀洿濂界殑鍥惧儚璐ㄩ噺銆?
`V4L2_CID_JPEG_ACTIVE_MARKER (bitmask)`
    鎸囧畾鍘嬬缉娴佷腑鍖呭惈鍝簺 JPEG 鏍囪銆傛鎺т欢浠呭缂栫爜鍣ㄦ湁鏁堛€?
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_JPEG_ACTIVE_MARKER_APP0`
      - 搴旂敤鏁版嵁娈?APP\ `0`銆?    - - `V4L2_JPEG_ACTIVE_MARKER_APP1`
      - 搴旂敤鏁版嵁娈?APP\ `1`銆?    - - `V4L2_JPEG_ACTIVE_MARKER_COM`
      - 娉ㄩ噴娈点€?    - - `V4L2_JPEG_ACTIVE_MARKER_DQT`
      - 閲忓寲琛ㄦ銆?    - - `V4L2_JPEG_ACTIVE_MARKER_DHT`
      - Huffman 琛ㄦ銆?
鏈夊叧 JPEG 瑙勮寖鐨勬洿澶氱粏鑺傦紝璇峰弬鑰?itu-t81銆乯fif銆亀3c-jpeg-jfif銆?