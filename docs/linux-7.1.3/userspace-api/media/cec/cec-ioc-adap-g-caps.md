


######## ioctl CEC_ADAP_G_CAPS


## 鍚嶇О


CEC_ADAP_G_CAPS - 鏌ヨ璁惧鑳藉姏

## 姒傝


`int ioctl(int fd, CEC_ADAP_G_CAPS, struct cec_caps *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`


## 鎻忚堪


鎵€鏈?cec 璁惧閮藉繀椤绘敮鎸?ioctl CEC_ADAP_G_CAPS <CEC_ADAP_G_CAPS>銆備负鏌ヨ璁惧淇℃伅锛屽簲鐢ㄧ▼搴忎互鎸囧悜 struct `cec_caps` 鐨勬寚閽堣皟鐢ㄨ ioctl銆傞┍鍔ㄥ～鍏呰缁撴瀯骞跺皢淇℃伅杩斿洖缁欏簲鐢ㄧ▼搴忋€傝 ioctl 姘歌繙涓嶄細澶辫触銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - char
      - `driver[^32^]`
      - cec 閫傞厤鍣ㄩ┍鍔ㄧ殑鍚嶇О銆?    - - char
      - `name[^32^]`
      - 姝?CEC 閫傞厤鍣ㄧ殑鍚嶇О銆俙driver` 涓?`name` 鐨勭粍鍚堝繀椤诲敮涓€銆?    - - __u32
      - `available_log_addrs`
      - 鍙厤缃殑閫昏緫鍦板潃鏈€澶ф暟閲忋€?    - - __u32
      - `capabilities`
      - CEC 閫傞厤鍣ㄧ殑鑳藉姏锛屽弬瑙?cec-capabilities銆?    - - __u32
      - `version`
      - CEC 妗嗘灦 API 鐗堟湰锛屼娇鐢?`KERNEL_VERSION()` 瀹忔牸寮忓寲銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-CAP-PHYS-ADDR`:

      - `CEC_CAP_PHYS_ADDR`
      - 0x00000001
      - 鐢ㄦ埛绌洪棿蹇呴』閫氳繃璋冪敤 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 閰嶇疆鐗╃悊鍦板潃銆傚鏋滄湭璁剧疆姝よ兘鍔涳紝鍒欑墿鐞嗗湴鍧€鐨勮缃湪 EDID 琚缃紙瀵?HDMI 鎺ユ敹鍣級鎴栬鍙栵紙瀵?HDMI 鍙戦€佸櫒锛夋椂鐢卞唴鏍稿鐞嗐€?    - .. _`CEC-CAP-LOG-ADDRS`:

      - `CEC_CAP_LOG_ADDRS`
      - 0x00000002
      - 鐢ㄦ埛绌洪棿蹇呴』閫氳繃璋冪敤 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 閰嶇疆閫昏緫鍦板潃銆傚鏋滄湭璁剧疆姝よ兘鍔涳紝鍒欑敱鍐呮牳瀹屾垚閰嶇疆銆?    - .. _`CEC-CAP-TRANSMIT`:

      - `CEC_CAP_TRANSMIT`
      - 0x00000004
      - 鐢ㄦ埛绌洪棿鍙互閫氳繃璋冪敤 ioctl CEC_TRANSMIT <CEC_TRANSMIT> 鍙戦€?CEC 娑堟伅銆傝繖鎰忓懗鐫€鐢ㄦ埛绌洪棿涔熷彲浠ユ垚涓?follower锛屽洜涓鸿兘澶熷彂閫佹秷鎭槸鎴愪负 follower 鐨勫墠鎻愩€傚鏋滄湭璁剧疆姝よ兘鍔涳紝鍒欑敱鍐呮牳澶勭悊鎵€鏈?CEC 鍙戦€佸苟澶勭悊瀹冩敹鍒扮殑鎵€鏈?CEC 娑堟伅銆?    - .. _`CEC-CAP-PASSTHROUGH`:

      - `CEC_CAP_PASSTHROUGH`
      - 0x00000008
      - 鐢ㄦ埛绌洪棿鍙互閫氳繃璋冪敤 ioctl CEC_S_MODE <CEC_S_MODE> 浣跨敤鐩撮€氾紙passthrough锛夋ā寮忋€?    - .. _`CEC-CAP-RC`:

      - `CEC_CAP_RC`
      - 0x00000010
      - 姝ら€傞厤鍣ㄦ敮鎸侀仴鎺э紙remote control锛夊崗璁€?    - .. _`CEC-CAP-MONITOR-ALL`:

      - `CEC_CAP_MONITOR_ALL`
      - 0x00000020
      - CEC 纭欢鍙互鐩戞帶鎵€鏈夋秷鎭紝鑰屼笉浠呬粎鏄畾鍚戝拰骞挎挱娑堟伅銆?    - .. _`CEC-CAP-NEEDS-HPD`:

      - `CEC_CAP_NEEDS_HPD`
      - 0x00000040
      - CEC 纭欢浠呭湪 HDMI Hotplug Detect 寮曡剼涓洪珮鐢靛钩鏃舵墠澶勪簬娲诲姩鐘舵€併€傝繖浣垮緱鏃犳硶浣跨敤 CEC 鍞ら啋鍦ㄥ緟鏈烘ā寮忎笅灏?HPD 寮曡剼缃綆銆佷絾淇濇寔 CEC 鎬荤嚎瀛樻椿鐨勬樉绀哄櫒銆?    - .. _`CEC-CAP-MONITOR-PIN`:

      - `CEC_CAP_MONITOR_PIN`
      - 0x00000080
      - CEC 纭欢鍙互鐩戞帶 CEC 寮曡剼浠庝綆鐢靛帇鍒伴珮鐢靛帇鐨勫彉鍖栧強鍏跺弽鍚戝彉鍖栥€傚湪寮曡剼鐩戞帶妯″紡涓嬶紝搴旂敤绋嬪簭灏嗘敹鍒?`CEC_EVENT_PIN_CEC_LOW` 鍜?`CEC_EVENT_PIN_CEC_HIGH` 浜嬩欢銆?    - .. _`CEC-CAP-CONNECTOR-INFO`:

      - `CEC_CAP_CONNECTOR_INFO`
      - 0x00000100
      - 濡傛灉璁剧疆浜嗘鑳藉姏锛屽垯鍙互浣跨敤 CEC_ADAP_G_CONNECTOR_INFO銆?    - .. _`CEC-CAP-REPLY-VENDOR-ID`:

      - `CEC_CAP_REPLY_VENDOR_ID`
      - 0x00000200
      - 濡傛灉璁剧疆浜嗘鑳藉姏锛屽垯鍙互浣跨敤 CEC_MSG_FL_REPLY_VENDOR_ID <cec-msg-flags>銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?