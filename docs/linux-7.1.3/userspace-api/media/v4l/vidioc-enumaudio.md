

######## ioctl VIDIOC_ENUMAUDIO


## 濮撳悕


VIDIOC_ENUMAUDIO - 鏋氫妇闊抽杈撳叆

## 姒傝



`int ioctl(int fd, VIDIOC_ENUMAUDIO, struct v4l2_audio *argp)`

## 璁虹偣


`fd`
`open()`杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
鎸囧悜缁撴瀯浣揱v4l2_audio`鐨勬寚閽堛€?

## 鎻忚堪


瑕佹煡璇㈤煶棰戣緭鍏ュ簲鐢ㄧ▼搴忕殑灞炴€э紝璇峰垵濮嬪寲
`index`瀛楁骞跺皢缁撴瀯浣撶殑`reserved`鏁扮粍娓呴浂
璋冪敤 VIDIOC_ENUMAUDIO
ioctl 甯︽湁鎸囧悜璇ョ粨鏋勭殑鎸囬拡銆傚徃鏈哄～琛ヤ簡鍓╀笅鐨勭┖缂?
鏋勯€犳垨褰撶储寮曡秴鍑烘椂杩斿洖`EINVAL`閿欒浠ｇ爜
鐣岄檺銆傝鏋氫妇鎵€鏈夐煶棰戣緭鍏ュ簲鐢ㄧ▼搴忓簲浠庣储寮曞紑濮?
闆讹紝鍔犱竴鐩村埌椹卞姩绋嬪簭杩斿洖`EINVAL`銆?

鏈夊叧缁撴瀯浣撶殑璇存槑锛岃鍙傞槄 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO>
`v4l2_audio`銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛岄敊璇椂杩斿洖 -1 骞朵笖璁剧疆 `errno` 鍙橀噺
閫傚綋鍦般€傞€氱敤閿欒浠ｇ爜鐨勬弿杩拌
閫氱敤閿欒浠ｇ爜 <gen-errors> 绔犺妭銆?

鍗曢」閫夋嫨
闊抽杈撳叆鏁伴噺瓒婄晫銆?
