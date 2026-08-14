


######## ioctl VIDIOC_ENUMOUTPUT


## 鍚嶇О


VIDIOC_ENUMOUTPUT - 鏋氫妇瑙嗛杈撳嚭

## 姒傝


`int ioctl(int fd, VIDIOC_ENUMOUTPUT, struct v4l2_output *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_output` 鐨勬寚閽堛€?
## 鎻忚堪


涓烘煡璇㈣棰戣緭鍑虹殑灞炴€э紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_output` 鐨?`index` 瀛楁锛屽苟浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_ENUMOUTPUT銆傚綋绱㈠紩瓒婄晫鏃讹紝椹卞姩濉厖缁撴瀯鐨勫叾浣欓儴鍒嗘垨杩斿洖 `EINVAL` 閿欒鐮併€備负鏋氫妇鎵€鏈夎緭鍑猴紝搴旂敤绋嬪簭搴斾粠绱㈠紩闆跺紑濮嬶紝姣忔閫掑涓€锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏍囪瘑杈撳嚭锛岀敱搴旂敤绋嬪簭璁剧疆銆?    - - __u8
      - `name`\ [^32^]
      - 瑙嗛杈撳嚭鐨勫悕绉帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓诧紝渚嬪锛?Vout"銆傛淇℃伅闈㈠悜鐢ㄦ埛锛屾渶濂戒娇鐢ㄨ澶囨湰韬笂鐨勮繛鎺ュ櫒鏍囩銆?    - - __u32
      - `type`
      - 杈撳嚭鐨勭被鍨嬶紝鍙傝 output-type銆?    - - __u32
      - `audioset`
      - 椹卞姩鍙互鏋氫妇澶氳揪 32 涓棰戝拰闊抽杈撳嚭銆傚鏋滆繖鏄綋鍓嶉€変腑鐨勮棰戣緭鍑猴紝璇ュ瓧娈垫樉绀哄摢浜涢煶棰戣緭鍑哄彲浣滀负褰撳墠杈撳嚭琚€変腑銆傚畠鏄竴涓綅鎺╃爜銆侺SB 瀵瑰簲闊抽杈撳嚭 0锛孧SB 瀵瑰簲杈撳嚭 31銆傚彲浠ヨ缃换鎰忔暟閲忕殑浣嶏紝涔熷彲浠ヤ笉璁剧疆銆?
	褰撻┍鍔ㄤ笉鏋氫妇闊抽杈撳嚭鏃讹紝涓嶅緱璁剧疆浠讳綍浣嶃€傚簲鐢ㄧ▼搴忎笉搴斿皢姝よВ閲婁负缂轰箯闊抽鏀寔銆傞┍鍔ㄥ彲浠ュ湪涓嶆灇涓剧殑鎯呭喌涓嬭嚜鍔ㄩ€夋嫨闊抽杈撳嚭銆?
	鍏充簬闊抽杈撳嚭浠ュ強濡備綍閫夋嫨褰撳墠杈撳嚭鐨勭粏鑺傦紝璇峰弬瑙?audio銆?    - - __u32
      - `modulator`
      - 杈撳嚭璁惧鍙互鏈夐浂涓垨澶氫釜 RF 璋冨埗鍣ㄣ€傚綋 `type` 涓?`V4L2_OUTPUT_TYPE_MODULATOR` 鏃讹紝杩欐槸涓€涓?RF 杩炴帴鍣紝璇ュ瓧娈垫爣璇嗚皟鍒跺櫒銆傚畠瀵瑰簲 struct `v4l2_modulator` 鐨?`index` 瀛楁銆傚叧浜庤皟鍒跺櫒鐨勭粏鑺傦紝璇峰弬瑙?tuner銆?    - - v4l2_std_id <v4l2-std-id>
      - `std`
      - 姣忎釜瑙嗛杈撳嚭鏀寔涓€绉嶆垨澶氱涓嶅悓鐨勮棰戞爣鍑嗐€傝瀛楁鏄墍鏈夋敮鎸佹爣鍑嗙殑闆嗗悎銆傚叧浜庤棰戞爣鍑嗗強濡備綍鍒囨崲鐨勭粏鑺傦紝璇峰弬瑙?standard銆?    - - __u32
      - `capabilities`
      - 璇ュ瓧娈垫彁渚涜緭鍑虹殑鑳藉姏銆傚弬瑙?output-capabilities 涓殑鏍囧織銆?    - - __u32
      - `reserved`\ [^3^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ繀椤诲皢鏁扮粍缃浂銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_OUTPUT_TYPE_MODULATOR`
      - 1
      - 姝よ緭鍑轰负妯℃嫙 TV 璋冨埗鍣ㄣ€?    - - `V4L2_OUTPUT_TYPE_ANALOG`
      - 2
      - 浠讳綍闈炶皟鍒跺櫒鐨勮棰戣緭鍑猴紝渚嬪 Composite Video銆丼-Video銆丠DMI銆傚懡鍚嶄负 `_TYPE_ANALOG` 鏄巻鍙插師鍥狅紝浠婂ぉ鎴戜滑浼氱О鍏朵负 `_TYPE_VIDEO`銆?    - - `V4L2_OUTPUT_TYPE_ANALOGVGAOVERLAY`
      - 3
      - 瑙嗛杈撳嚭灏嗚澶嶅埗鍒拌棰戝彔鍔?<overlay>銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_OUT_CAP_DV_TIMINGS`
      - 0x00000002
      - 姝よ緭鍑烘敮鎸佷娇鐢?`VIDIOC_S_DV_TIMINGS` 璁剧疆瑙嗛鏃跺簭銆?    - - `V4L2_OUT_CAP_STD`
      - 0x00000004
      - 姝よ緭鍑烘敮鎸佷娇鐢?`VIDIOC_S_STD` 璁剧疆 TV 鏍囧噯銆?    - - `V4L2_OUT_CAP_NATIVE_SIZE`
      - 0x00000008
      - 姝よ緭鍑烘敮鎸佷娇鐢?`V4L2_SEL_TGT_NATIVE_SIZE` 閫夋嫨鐩爣璁剧疆鍘熺敓灏哄锛岃鍙傝 v4l2-selections-common銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `v4l2_output` 鐨?`index` 瓒婄晫銆?