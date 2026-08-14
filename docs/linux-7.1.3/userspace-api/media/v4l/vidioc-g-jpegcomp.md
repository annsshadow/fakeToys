


######## ioctl VIDIOC_G_JPEGCOMP, VIDIOC_S_JPEGCOMP


## 鍚嶇О锛圢ame锛?

VIDIOC_G_JPEGCOMP - VIDIOC_S_JPEGCOMP

## 姒傝锛圫ynopsis锛?

`int ioctl(int fd, VIDIOC_G_JPEGCOMP, v4l2_jpegcompression *argp)`


`int ioctl(int fd, VIDIOC_S_JPEGCOMP, const v4l2_jpegcompression *argp)`

## 鍙傛暟锛圓rguments锛?

`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_jpegcompression` 鐨勬寚閽堛€?
## 鎻忚堪锛圖escription锛?

杩欎簺 ioctl 宸?*琚純鐢?*銆傛柊鐨勯┍鍔ㄤ笌搴旂敤绋嬪簭搴斾娇鐢?JPEG 绫绘帶浠?<jpeg-controls> 鏉?鎺у埗鍥惧儚璐ㄩ噺涓?JPEG 鏍囪锛坢arkers锛夈€?
[寰呭姙]

Ronald Bultje 璇︾粏璇存槑锛?
APP 鏄竴浜涘簲鐢ㄧ▼搴忕壒瀹氱殑淇℃伅銆傚簲鐢ㄧ▼搴忓彲浠ヨ嚜琛岃缃畠锛屽畠浼氳瀛樺偍鍦?JPEG 缂栫爜瀛楁
涓紙渚嬪锛岀敤浜?AVI 涓殑浜ら敊淇℃伅绛夛級銆侰OM 涓庝箣鐩稿悓锛屼絾瀹冩槸娉ㄩ噴锛屾瘮濡傗€滅敱鎴戠紪鐮佲€濅箣绫汇€?
jpeg_markers 鎻忚堪鏄惁搴斿皢 Huffman 琛ㄣ€侀噺鍖栬〃涓庨噸鍚棿闅斾俊鎭紙閮芥槸 JPEG 鐗瑰畾鐨?鍐呭锛夊瓨鍌ㄥ湪 JPEG 缂栫爜瀛楁涓€傚畠浠畾涔変簡 JPEG 瀛楁濡備綍琚紪鐮併€傚鏋滅渷鐣ュ畠浠紝搴旂敤
绋嬪簭浼氬亣瀹氫綘浣跨敤浜嗘爣鍑嗙紪鐮併€備綘閫氬父纭疄鎯宠娣诲姞瀹冧滑銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - int
      - `quality`
      - 宸插純鐢ㄣ€傚鏋滈┍鍔ㄦ毚闇蹭簡
	V4L2_CID_JPEG_COMPRESSION_QUALITY <jpeg-quality-control>
	鎺т欢锛屽簲鐢ㄧ▼搴忓簲浣跨敤瀹冿紝骞跺拷鐣ユ瀛楁銆?    - - int
      - `APPn`
      -
    - - int
      - `APP_len`
      -
    - - char
      - `APP_data`\ [^60^]
      -
    - - int
      - `COM_len`
      -
    - - char
      - `COM_data`\ [^60^]
      -
    - - __u32
      - `jpeg_markers`
      - 鍙傝 jpeg-markers銆傚凡寮冪敤銆傚鏋滈┍鍔ㄦ毚闇蹭簡
	V4L2_CID_JPEG_ACTIVE_MARKER <jpeg-active-marker-control>
	鎺т欢锛屽簲鐢ㄧ▼搴忓簲浣跨敤瀹冿紝骞跺拷鐣ユ瀛楁銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_JPEG_MARKER_DHT`
      - (1<<3)
      - 瀹氫箟 Huffman 琛?    - - `V4L2_JPEG_MARKER_DQT`
      - (1<<4)
      - 瀹氫箟閲忓寲琛?    - - `V4L2_JPEG_MARKER_DRI`
      - (1<<5)
      - 瀹氫箟閲嶅惎闂撮殧
    - - `V4L2_JPEG_MARKER_COM`
      - (1<<6)
      - 娉ㄩ噴娈?    - - `V4L2_JPEG_MARKER_APP`
      - (1<<7)
      - App 娈碉紝椹卞姩灏嗗缁堜娇鐢?APP0

## 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟閫傚綋鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?