


######## ioctl VIDIOC_G_AUDIO, VIDIOC_S_AUDIO


## 鍚嶇О


VIDIOC_G_AUDIO - VIDIOC_S_AUDIO - 鏌ヨ鎴栭€夋嫨褰撳墠鐨勯煶棰戣緭鍏ュ強鍏跺睘鎬?
## 姒傝


`int ioctl(int fd, VIDIOC_G_AUDIO, struct v4l2_audio *argp)`


`int ioctl(int fd, VIDIOC_S_AUDIO, const struct v4l2_audio *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_audio` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠綋鍓嶉煶棰戣緭鍏ワ紝搴旂敤绋嬪簭鍏堝皢 struct `v4l2_audio` 鐨?`reserved` 鏁扮粍娓呴浂锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl銆傚綋璁惧娌℃湁闊抽杈撳叆锛屾垨鑰呮病鏈変笌褰撳墠瑙嗛杈撳叆鐩哥粍鍚堢殑闊抽杈撳叆鏃讹紝椹卞姩浼氬～鍏呯粨鏋勭殑鍏朵綑閮ㄥ垎锛屾垨鑰呰繑鍥?`EINVAL` 閿欒鐮併€?
闊抽杈撳叆鏈変竴涓彲鍐欏睘鎬э紝鍗抽煶棰戞ā寮忋€傝閫夋嫨褰撳墠闊抽杈撳叆**骞?*鏇存敼闊抽妯″紡锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 struct `v4l2_audio` 缁撴瀯鐨?`index` 鍜?`mode` 瀛楁浠ュ強 `reserved` 鏁扮粍锛岀劧鍚庤皟鐢?VIDIOC_S_AUDIO <VIDIOC_G_AUDIO> ioctl銆傚鏋滆姹傛棤娉曡婊¤冻锛岄┍鍔ㄥ彲鑳戒細鍒囨崲鍒颁笉鍚岀殑闊抽妯″紡銆備笉杩囷紝杩欐槸涓€涓彧鍐欙紙write-only锛塱octl锛屽畠涓嶄細杩斿洖瀹為檯鐨勬柊鐨勯煶棰戞ā寮忋€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏍囪瘑闊抽杈撳叆锛岀敱椹卞姩鎴栧簲鐢ㄧ▼搴忚缃€?    - - __u8
      - `name`\ [^32^]
      - 闊抽杈撳叆鐨勫悕绉帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓诧紝渚嬪锛?Line In"銆傛淇℃伅渚涚敤鎴蜂娇鐢紝鏈€濂芥槸璁惧鏈韩涓婄殑杩炴帴鍣ㄦ爣绛俱€?    - - __u32
      - `capability`
      - 闊抽鑳藉姏鏍囧織锛屽弬瑙?audio-capability銆?    - - __u32
      - `mode`
      - 鐢遍┍鍔ㄥ拰搴旂敤绋嬪簭璁剧疆鐨勯煶棰戞ā寮忔爣蹇楋紙鍦?VIDIOC_S_AUDIO <VIDIOC_G_AUDIO> ioctl 涓級锛屽弬瑙?audio-mode銆?    - - __u32
      - `reserved`\ [^2^]
      - 淇濈暀渚涘皢鏉ユ墿灞曘€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_AUDCAP_STEREO`
      - 0x00001
      - 杩欐槸涓€涓珛浣撳０杈撳叆銆傝鏍囧織鐢ㄤ簬鍦ㄤ俊鍙峰缁堜负鍗曞０閬撴椂鑷姩绂佺敤绔嬩綋澹板綍鍒剁瓑銆傞櫎闈為煶棰戣緭鍏ュ睘浜庤皟璋愬櫒锛屽惁鍒?API 娌℃湁鎻愪緵妫€娴嬫槸鍚?*鎺ユ敹鍒?*绔嬩綋澹扮殑鎵嬫銆?    - - `V4L2_AUDCAP_AVL`
      - 0x00002
      - 鏀寔鑷姩闊抽噺鐢靛钩锛圓utomatic Volume Level锛夋ā寮忋€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_AUDMODE_AVL`
      - 0x00001
      - AVL 妯″紡寮€鍚€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    娌℃湁闊抽杈撳叆涓庡綋鍓嶈棰戣緭鍏ョ粍鍚堬紝鎴栬€呮墍閫夐煶棰戣緭鍏ョ殑缂栧彿瓒呭嚭鑼冨洿锛屾垨鑰呭畠鏃犳硶缁勫悎銆?