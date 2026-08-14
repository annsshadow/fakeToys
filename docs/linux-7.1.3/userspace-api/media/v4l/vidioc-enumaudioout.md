


######## ioctl VIDIOC_ENUMAUDOUT


## 鍚嶇О


VIDIOC_ENUMAUDOUT - 鏋氫妇闊抽杈撳嚭

## 姒傝


`int ioctl(int fd, VIDIOC_ENUMAUDOUT, struct v4l2_audioout *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_audioout` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈤煶棰戣緭鍑虹殑灞炴€э紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_audioout` 鐨?`index` 瀛楁
骞舵竻闆?`reserved` 鏁扮粍锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_G_AUDOUT` ioctl銆?褰撶储寮曡秺鐣屾椂锛岄┍鍔ㄥ～鍏呯粨鏋勭殑鍏朵綑閮ㄥ垎鎴栬繑鍥?`EINVAL` 閿欒鐮併€傝鏋氫妇鎵€鏈夐煶棰?杈撳嚭锛屽簲鐢ㄧ▼搴忓簲浠庣储寮曢浂寮€濮嬶紝姣忔閫掑涓€锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`銆?

    灏嗙數瑙嗗崱涓婄敤浜庢妸鎺ユ敹鍒扮殑闊抽淇″彿鐜洖鍒板０鍗＄殑杩炴帴鍣紝鍦ㄦ鎰忎箟涓婁笉鏄煶棰?    杈撳嚭銆?
鏈夊叧 struct `v4l2_audioout` 鐨勬弿杩帮紝璇峰弬闃?VIDIOC_G_AUDIOout <VIDIOC_G_AUDOUT>銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    闊抽杈撳嚭鐨勭紪鍙疯秺鐣屻€?