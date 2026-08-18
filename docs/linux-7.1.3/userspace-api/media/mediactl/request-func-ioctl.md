
######## request ioctl()


## 鍚嶇О


request-ioctl - 鎺у埗璇锋眰鏂囦欢鎻忚堪绗?

## 姒傝


    #include <sys/ioctl.h>

`int ioctl(int fd, int cmd, void *argp)`

## 鍙傛暟


`fd`
    鐢?MEDIA_IOC_REQUEST_ALLOC 杩斿洖鐨勬枃浠舵弿杩扮銆?

`cmd`
    璇锋眰 ioctl 鍛戒护浠ｇ爜锛屽畾涔夊湪 media.h 澶存枃浠朵腑锛屼緥濡?MEDIA_REQUEST_IOC_QUEUE銆?

`argp`
    鎸囧悜璇锋眰鐗瑰畾缁撴瀯鐨勬寚閽堛€?

## 鎻忚堪


ioctl() <request-func-ioctl> 鍑芥暟鎿嶇旱璇锋眰鍙傛暟銆傚弬鏁?`fd` 蹇呴』鏄凡鎵撳紑鐨勬枃浠舵弿杩扮銆?

ioctl `cmd` 浠ｇ爜鎸囧畾瑕佽皟鐢ㄧ殑璇锋眰鍑芥暟銆傚畠缂栫爜浜嗗弬鏁版槸杈撳叆銆佽緭鍑鸿繕鏄/鍐欏弬鏁帮紝浠ュ強鍙傛暟 `argp` 鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?

鎸囧畾璇锋眰 ioctl 鍛戒护鍙婂叾鍙傛暟鐨勫畯鍜岀粨鏋勫畾涔変綅浜?media.h 澶存枃浠朵腑銆傛墍鏈夎姹?ioctl 鍛戒护銆佸悇鑷殑鍑芥暟鍜屽弬鏁伴兘鍦?media-user-func 涓寚瀹氥€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

鐗瑰畾浜庡懡浠ょ殑閿欒鐮佸垪鍦ㄥ悇涓懡浠ょ殑鎻忚堪涓€?

褰撳甫鏈夎緭鍑烘垨璇?鍐欏弬鏁扮殑 ioctl 澶辫触鏃讹紝璇ュ弬鏁颁繚鎸佷笉鍙樸€?
