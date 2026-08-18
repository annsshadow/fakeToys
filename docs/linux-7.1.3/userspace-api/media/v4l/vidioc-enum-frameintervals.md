
######## ioctl VIDIOC_ENUM_FRAMEINTERVALS


## 鍚嶇О


VIDIOC_ENUM_FRAMEINTERVALS - 鏋氫妇甯ч棿闅?
## 姒傝



`int ioctl(int fd, VIDIOC_ENUM_FRAMEINTERVALS, struct v4l2_frmivalenum *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_frmivalenum` 鐨勬寚閽堬紝
    鍏朵腑鍖呭惈鍍忕礌鏍煎紡鍜屽昂瀵革紝骞舵帴鏀朵竴涓抚闂撮殧銆?
## 鎻忚堪


姝?ioctl 鍏佽搴旂敤绋嬪簭鏋氫妇璁惧閽堝缁欏畾鍍忕礌鏍煎紡鍜屽抚灏哄鎵€鏀寔鐨勬墍鏈夊抚闂撮殧銆?
鏀寔鐨勫儚绱犳牸寮忓拰甯у昂瀵稿彲浠ラ€氳繃浣跨敤 VIDIOC_ENUM_FMT 鍜?VIDIOC_ENUM_FRAMESIZES 鍑芥暟鑾峰緱銆?
杩斿洖鍊间互鍙?`v4l2_frmivalenum.type` 瀛楁鐨勫唴瀹瑰彇鍐充簬璁惧鏀寔鐨勫抚闂撮殧绫诲瀷銆備互涓嬫槸璇?鍑芥暟鍦ㄤ笉鍚屾儏鍐典笅鐨勮涔夛細

- **绂绘暎锛圖iscrete锛夛細** 濡傛灉缁欏畾鐨勭储寮曞€硷紙浠庨浂寮€濮嬶級鏈夋晥锛屽嚱鏁拌繑鍥炴垚鍔熴€傚簲鐢ㄧ▼搴?   搴斿皢绱㈠紩姣忔鍔犱竴杩涜璋冪敤锛岀洿鍒拌繑鍥?`EINVAL`銆俙v4l2_frmivalenum.type` 瀛楁鐢遍┍鍔?   璁剧疆涓?`V4L2_FRMIVAL_TYPE_DISCRETE`銆傚湪鑱斿悎浣撲腑锛屽彧鏈?`discrete` 鎴愬憳鏈夋晥銆?
- **姝ヨ繘锛圫tep-wise锛夛細** 濡傛灉缁欏畾鐨勭储寮曞€间负闆讹紝鍑芥暟杩斿洖鎴愬姛锛涘浜庝换浣曞叾浠栫储寮曞€?   鍒欒繑鍥?`EINVAL`銆俙v4l2_frmivalenum.type` 瀛楁鐢遍┍鍔ㄨ缃负
   `V4L2_FRMIVAL_TYPE_STEPWISE`銆傚湪鑱斿悎浣撲腑锛屽彧鏈?`stepwise` 鎴愬憳鏈夋晥銆?
- **杩炵画锛圕ontinuous锛夛細** 杩欐槸涓婅堪姝ヨ繘绫诲瀷鐨勪竴绉嶇壒娈婃儏鍐点€傚鏋滅粰瀹氱殑绱㈠紩鍊间负闆讹紝
   鍑芥暟杩斿洖鎴愬姛锛涘浜庝换浣曞叾浠栫储寮曞€煎垯杩斿洖 `EINVAL`銆俙v4l2_frmivalenum.type` 瀛楁鐢?   椹卞姩璁剧疆涓?`V4L2_FRMIVAL_TYPE_CONTINUOUS`銆傚湪鑱斿悎浣撲腑锛屽彧鏈?`stepwise` 鎴愬憳鏈夋晥锛?   涓?`step` 鍊艰璁句负 1銆?
褰撳簲鐢ㄧ▼搴忎互绱㈠紩闆惰皟鐢ㄨ鍑芥暟鏃讹紝瀹冨繀椤绘鏌?`type` 瀛楁浠ョ‘瀹氳澶囨敮鎸佺殑甯ч棿闅旀灇涓?绫诲瀷銆傚彧鏈夊浜?`V4L2_FRMIVAL_TYPE_DISCRETE` 绫诲瀷锛岄€掑绱㈠紩鍊间互鎺ユ敹鏇村甯ч棿闅旀墠鏈?鎰忎箟銆?

   甯ч棿闅旇繑鍥炵殑椤哄簭娌℃湁鐗规畩鍚箟銆傚挨鍏跺畠骞朵笉琛ㄧず浠讳綍鍏充簬娼滃湪榛樿甯ч棿闅旂殑淇℃伅銆?
搴旂敤绋嬪簭鍙互鍋囧畾鏋氫妇鏁版嵁涓嶄細鍦ㄦ病鏈夊簲鐢ㄧ▼搴忚嚜韬氦浜掔殑鎯呭喌涓嬪彂鐢熷彉鍖栥€傝繖鎰忓懗鐫€濡傛灉
搴旂敤绋嬪簭鍦ㄨ繍琛屽抚闂撮殧鏋氫妇鏃朵笉鎵ц浠讳綍鍏朵粬 ioctl 璋冪敤锛屽垯鏋氫妇鏁版嵁鏄竴鑷寸殑銆?

   **甯ч棿闅斾笌甯х巼锛?* V4L2 API 浣跨敤甯ч棿闅旇€岄潪甯х巼銆傜粰瀹氬抚闂撮殧鍚庯紝甯х巼鍙寜濡備笅鏂瑰紡
   璁＄畻锛?
```
       frame_rate = 1 / frame_interval

```
## 缁撴瀯浣?

鍦ㄤ笅杩扮粨鏋勪綋涓紝**IN** 琛ㄧず蹇呴』鐢卞簲鐢ㄧ▼搴忓～鍏ョ殑鍊硷紝**OUT** 琛ㄧず鐢遍┍鍔ㄥ～鍏ョ殑鍊笺€?搴旂敤绋嬪簭搴斿皢闄?**IN** 瀛楁涔嬪鐨勬墍鏈夋垚鍛樼疆闆躲€?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_fract`
      - `min`
      - 鏈€灏忓抚闂撮殧 [s]銆?    - - struct `v4l2_fract`
      - `max`
      - 鏈€澶у抚闂撮殧 [s]銆?    - - struct `v4l2_fract`
      - `step`
      - 甯ч棿闅旀闀?[s]銆?





    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - IN锛氭灇涓句腑缁欏畾甯ч棿闅旂殑绱㈠紩銆?    - - __u32
      - `pixel_format`
      - IN锛氳鏋氫妇甯ч棿闅旂殑鍍忕礌鏍煎紡銆?    - - __u32
      - `width`
      - IN锛氳鏋氫妇甯ч棿闅旂殑甯у搴︺€?    - - __u32
      - `height`
      - IN锛氳鏋氫妇甯ч棿闅旂殑甯ч珮搴︺€?    - - __u32
      - `type`
      - OUT锛氳澶囨敮鎸佺殑甯ч棿闅旂被鍨嬨€?    - - union {
      - (anonymous)
      - OUT锛氬叿鏈夌粰瀹氱储寮曠殑甯ч棿闅斻€?    - - struct `v4l2_fract`
      - `discrete`
      - 甯ч棿闅?[s]銆?    - - struct `v4l2_frmival_stepwise`
      - `stepwise`
      -
    - - }
#       -

    - - __u32
      - `reserved[^2^]`
      - 涓烘湭鏉ヤ娇鐢ㄤ繚鐣欑殑绌洪棿銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗗叾缃浂銆?

## 鏋氫妇



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FRMIVAL_TYPE_DISCRETE`
      - 1
      - 绂绘暎甯ч棿闅斻€?    - - `V4L2_FRMIVAL_TYPE_CONTINUOUS`
      - 2
      - 杩炵画甯ч棿闅斻€?    - - `V4L2_FRMIVAL_TYPE_STEPWISE`
      - 3
      - 姝ヨ繘寮忓畾涔夌殑甯ч棿闅斻€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?