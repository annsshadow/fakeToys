######## ioctl VIDIOC_G_AUDOUT, VIDIOC_S_AUDOUT


## 鍚嶇О


VIDIOC_G_AUDOUT - VIDIOC_S_AUDOUT - 鏌ヨ鎴栭€夋嫨褰撳墠鐨勯煶棰戣緭鍑?
## 鎽樿


`int ioctl(int fd, VIDIOC_G_AUDOUT, struct v4l2_audioout *argp)`


`int ioctl(int fd, VIDIOC_S_AUDOUT, const struct v4l2_audioout *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_audioout` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠綋鍓嶇殑闊抽杈撳嚭锛屽簲鐢ㄧ▼搴忛渶灏嗕竴涓?struct `v4l2_audioout` 鐨?`reserved`
鏁扮粍娓呴浂锛屽苟浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_G_AUDOUT` ioctl銆傞┍鍔ㄤ細濉厖缁撴瀯鐨?鍏朵綑閮ㄥ垎锛涘綋璁惧娌℃湁闊抽杈撳叆锛屾垨涓庡綋鍓嶈棰戣緭鍑烘棤娉曠粍鍚堟椂锛屽垯杩斿洖 `EINVAL`
閿欒鐮併€?
闊抽杈撳嚭娌℃湁鍙啓灞炴€с€備笉杩囷紝瑕侀€夋嫨褰撳墠鐨勯煶棰戣緭鍑猴紝搴旂敤绋嬪簭鍙互鍒濆鍖栦竴涓?struct `v4l2_audioout` 缁撴瀯鐨?`index` 瀛楁鍜?`reserved` 鏁扮粍锛堟湭鏉ュ彲鑳藉寘鍚?鍙啓灞炴€э級锛岀劧鍚庤皟鐢?`VIDIOC_S_AUDOUT` ioctl銆傞┍鍔ㄤ細鍒囨崲鍒版墍璇锋眰鐨勮緭鍑猴紝鎴栧綋
index 瓒婄晫鏃惰繑鍥?`EINVAL` 閿欒鐮併€傝繖鏄竴涓彧鍐?ioctl锛屽畠涓嶄細鍍?`VIDIOC_G_AUDOUT`
閭ｆ牱杩斿洖褰撳墠闊抽杈撳嚭鐨勫睘鎬с€?

   TV 鍗′笂鐢ㄤ簬鎶婃帴鏀跺埌鐨勯煶棰戜俊鍙风幆鍥炲埌澹板崱鐨勬帴鍙ｄ笉灞炰簬姝ゆ剰涔変笂鐨勯煶棰戣緭鍑恒€?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏍囪瘑闊抽杈撳嚭锛岀敱椹卞姩鎴栧簲鐢ㄧ▼搴忚缃€?    - - __u8
      - `name`\ [^32^]
      - 闊抽杈撳嚭鐨勫悕绉帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓诧紝渚嬪锛氣€淟ine Out鈥濄€?	姝や俊鎭緵鐢ㄦ埛鍙傝€冿紝鏈€濂芥槸璁惧鏈韩鐨勬帴鍙ｆ爣绛俱€?    - - __u32
      - `capability`
      - 闊抽鑳藉姏鏍囧織锛岀洰鍓嶅皻鏈畾涔夈€傞┍鍔ㄥ繀椤诲皢鏈瓧娈佃缃负闆躲€?    - - __u32
      - `mode`
      - 闊抽妯″紡锛岀洰鍓嶅皻鏈畾涔夈€傞┍鍔ㄥ拰搴旂敤绋嬪簭锛堝湪 `VIDIOC_S_AUDOUT` 鏃讹級
	蹇呴』灏嗘湰瀛楁璁剧疆涓洪浂銆?    - - __u32
      - `reserved`\ [^2^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗘湰鏁扮粍璁剧疆涓洪浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    娌℃湁涓庡綋鍓嶈棰戣緭鍑虹粍鍚堢殑闊抽杈撳嚭锛屾垨鑰呮墍閫夐煶棰戣緭鍑虹殑缂栧彿瓒婄晫锛屾垨鏃犳硶缁勫悎銆?