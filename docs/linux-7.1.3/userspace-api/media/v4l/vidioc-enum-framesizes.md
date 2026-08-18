


######## ioctl VIDIOC_ENUM_FRAMESIZES


## 鍚嶇О


VIDIOC_ENUM_FRAMESIZES - 鏋氫妇甯уぇ灏?
## 姒傝



`int ioctl(int fd, VIDIOC_ENUM_FRAMESIZES, struct v4l2_frmsizeenum *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_frmsizeenum` 鐨勬寚閽堬紝鍏朵腑鍖呭惈涓€涓储寮曞拰鍍忕礌鏍煎紡锛?    骞舵帴鏀跺抚鐨勫搴﹀拰楂樺害銆?
## 鎻忚堪


璇?ioctl 鍏佽搴旂敤绋嬪簭鏋氫妇璁惧閽堝缁欏畾鍍忕礌鏍煎紡鎵€鏀寔鐨勬墍鏈夊抚澶у皬锛堝嵆瀹藉害
鍜岄珮搴︼紝鍗曚綅涓哄儚绱狅級銆?
鏀寔鐨勫儚绱犳牸寮忓彲浠ラ€氳繃 VIDIOC_ENUM_FMT 鍑芥暟鑾峰緱銆?
`v4l2_frmsizeenum.type` 瀛楁鐨勮繑鍥炲€煎強鍏跺唴瀹瑰彇鍐充簬璁惧鎵€鏀寔鐨勫抚澶у皬绫诲瀷銆?浠ヤ笅鏄鍑芥暟鍦ㄤ笉鍚屾儏鍐典笅鐨勮涔夛細

- **Discrete锛堢鏁ｏ級锛?* 鑻ョ粰瀹氱殑绱㈠紩鍊硷紙浠庨浂寮€濮嬶級鏈夋晥锛屽嚱鏁拌繑鍥炴垚鍔熴€?  搴旂敤绋嬪簭搴斿皢绱㈠紩姣忔鍔犱竴鍚庨噸澶嶈皟鐢紝鐩村埌杩斿洖 `EINVAL`銆傞┍鍔ㄥ皢
  `v4l2_frmsizeenum.type` 瀛楁璁句负 `V4L2_FRMSIZE_TYPE_DISCRETE`銆傝仈鍚堜綋涓?  鍙湁 `discrete` 鎴愬憳鏈夋晥銆?
- **Step-wise锛堟杩涳級锛?* 鑻ョ粰瀹氱殑绱㈠紩鍊间负闆跺垯鍑芥暟杩斿洖鎴愬姛锛屽叾瀹冧换浣曠储寮曞€?  閮借繑鍥?`EINVAL`銆傞┍鍔ㄥ皢 `v4l2_frmsizeenum.type` 瀛楁璁句负
  `V4L2_FRMSIZE_TYPE_STEPWISE`銆傝仈鍚堜綋涓彧鏈?`stepwise` 鎴愬憳鏈夋晥銆?
- **Continuous锛堣繛缁級锛?* 杩欐槸涓婅堪姝ヨ繘绫诲瀷鐨勪竴绉嶇壒娈婃儏鍐点€傝嫢缁欏畾鐨勭储寮曞€?  涓洪浂鍒欏嚱鏁拌繑鍥炴垚鍔燂紝鍏跺畠浠讳綍绱㈠紩鍊奸兘杩斿洖 `EINVAL`銆傞┍鍔ㄥ皢
  `v4l2_frmsizeenum.type` 瀛楁璁句负 `V4L2_FRMSIZE_TYPE_CONTINUOUS`銆傝仈鍚堜綋涓?  鍙湁 `stepwise` 鎴愬憳鏈夋晥锛屼笖 `step_width` 鍜?`step_height` 鐨勫€艰璁句负 1銆?
褰撳簲鐢ㄧ▼搴忎互绱㈠紩闆惰皟鐢ㄨ鍑芥暟鏃讹紝瀹冨繀椤绘鏌?`type` 瀛楁浠ョ‘瀹氳澶囨敮鎸佺殑
甯уぇ灏忔灇涓剧被鍨嬨€傚彧鏈夊浜?`V4L2_FRMSIZE_TYPE_DISCRETE` 绫诲瀷锛岄€掑绱㈠紩鍊间互
鑾峰彇鏇村甯уぇ灏忔墠鏈夋剰涔夈€?

   甯уぇ灏忚繑鍥炵殑椤哄簭娌℃湁鐗规畩鍚箟銆傜壒鍒湴锛屽畠骞朵笉琛ㄧず浠讳綍娼滃湪鐨勯粯璁ゆ牸寮忓ぇ灏忋€?
搴旂敤绋嬪簭鍙互鍋囧畾锛屽湪娌℃湁搴旂敤绋嬪簭鑷韩浠讳綍浜や簰鐨勬儏鍐典笅锛屾灇涓炬暟鎹笉浼氬彂鐢熷彉鍖栥€?杩欐剰鍛崇潃濡傛灉搴旂敤绋嬪簭鍦ㄨ繍琛屽抚澶у皬鏋氫妇鏈熼棿涓嶆墽琛屼换浣曞叾瀹?ioctl 璋冪敤锛屾灇涓炬暟鎹?灏辨槸涓€鑷寸殑銆?
## 缁撴瀯浣?


In the structs below, **IN** denotes a value that has to be filled in by
the application, **OUT** denotes values that the driver fills in. The
application should zero out all members except for the **IN** fields.


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 甯у搴?[鍍忕礌]銆?    - - __u32
      - `height`
      - 甯ч珮搴?[鍍忕礌]銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `min_width`
      - 鏈€灏忓抚瀹藉害 [鍍忕礌]銆?    - - __u32
      - `max_width`
      - 鏈€澶у抚瀹藉害 [鍍忕礌]銆?    - - __u32
      - `step_width`
      - 甯у搴︽闀?[鍍忕礌]銆?    - - __u32
      - `min_height`
      - 鏈€灏忓抚楂樺害 [鍍忕礌]銆?    - - __u32
      - `max_height`
      - 鏈€澶у抚楂樺害 [鍍忕礌]銆?    - - __u32
      - `step_height`
      - 甯ч珮搴︽闀?[鍍忕礌]銆?

    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - IN锛氭灇涓句腑缁欏畾甯уぇ灏忕殑绱㈠紩銆?    - - __u32
      - `pixel_format`
      - IN锛氳鏋氫妇甯уぇ灏忕殑鍍忕礌鏍煎紡銆?    - - __u32
      - `type`
      - OUT锛氳澶囨敮鎸佺殑甯уぇ灏忕被鍨嬨€?    - - union {
      - (anonymous)
      - OUT锛氬叿鏈夌粰瀹氱储寮曠殑甯уぇ灏忋€?    - - struct `v4l2_frmsize_discrete`
      - `discrete`
      -
    - - struct `v4l2_frmsize_stepwise`
      - `stepwise`
      -
    - - }
#       -

    - - __u32
      - `reserved[^2^]`
      - 涓哄皢鏉ヤ娇鐢ㄤ繚鐣欑殑绌洪棿銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗗叾娓呴浂銆?

## 鏋氫妇



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FRMSIZE_TYPE_DISCRETE`
      - 1
      - 绂绘暎甯уぇ灏忋€?    - - `V4L2_FRMSIZE_TYPE_CONTINUOUS`
      - 2
      - 杩炵画甯уぇ灏忋€?    - - `V4L2_FRMSIZE_TYPE_STEPWISE`
      - 3
      - 姝ヨ繘寮忓畾涔夌殑甯уぇ灏忋€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?