..
..


######## ioctl CEC_ADAP_G_CONNECTOR_INFO


## 鍚嶇О


CEC_ADAP_G_CONNECTOR_INFO - 鏌ヨ HDMI 杩炴帴鍣ㄤ俊鎭?
## 姒傝



`int ioctl(int fd, CEC_ADAP_G_CONNECTOR_INFO, struct cec_connector_info *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct cec_connector_info 鐨勬寚閽堛€?
## 鎻忚堪


浣跨敤璇?ioctl锛屽簲鐢ㄧ▼搴忓彲浠ヨ幏鐭ユ CEC 璁惧瀵瑰簲浜庡摢涓?HDMI 杩炴帴鍣ㄣ€傝皟鐢ㄦ ioctl 鏃讹紝
搴旂敤绋嬪簭搴旀彁渚涗竴涓寚鍚?cec_connector_info 缁撴瀯浣撶殑鎸囬拡锛屽唴鏍稿皢鐢ㄩ€傞厤鍣ㄩ┍鍔ㄦ彁渚涚殑淇℃伅
濉厖璇ョ粨鏋勪綋銆備粎褰撹缃簡 `CEC_CAP_CONNECTOR_INFO` 鑳藉姏鏃讹紝姝?ioctl 鎵嶅彲鐢ㄣ€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u32
      - `type`
      - 璇ラ€傞厤鍣ㄦ墍鍏宠仈鐨勯€傞厤鍣ㄧ被鍨嬨€?    - - union {
      - `(anonymous)`
    - - `struct cec_drm_connector_info`
      - drm
      - cec-drm-connector-info
    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-CONNECTOR-TYPE-NO-CONNECTOR`:

      - `CEC_CONNECTOR_TYPE_NO_CONNECTOR`
      - 0
      - 娌℃湁涓庤閫傞厤鍣ㄥ叧鑱旂殑杩炴帴鍣?椹卞姩鏈彁渚涜淇℃伅銆?    - .. _`CEC-CONNECTOR-TYPE-DRM`:

      - `CEC_CONNECTOR_TYPE_DRM`
      - 1
      - 琛ㄧず鏈変竴涓?DRM 杩炴帴鍣ㄤ笌璇ラ€傞厤鍣ㄥ叧鑱斻€傛湁鍏宠杩炴帴鍣ㄧ殑淇℃伅鍙湪
	cec-drm-connector-info 涓壘鍒般€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-DRM-CONNECTOR-TYPE-CARD-NO`:

      - __u32
      - `card_no`
      - DRM 鍗＄紪鍙凤細鏉ヨ嚜鍗¤矾寰勭殑缂栧彿锛屼緥濡?/dev/card0 涓殑 0銆?    - .. _`CEC-DRM-CONNECTOR-TYPE-CONNECTOR_ID`:

      - __u32
      - `connector_id`
      - DRM 杩炴帴鍣?ID銆?