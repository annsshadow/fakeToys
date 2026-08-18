
######## V4L2 ioctl()


## 鍚嶇О


v4l2-ioctl - 缂栫▼ V4L2 璁惧

## 姒傝


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`request`
    鍦?`videodev2.h` 澶存枃浠朵腑瀹氫箟鐨?V4L2 ioctl 璇锋眰鐮侊紝渚嬪
    VIDIOC_QUERYCAP銆?
`argp`
    鎸囧悜鍑芥暟鍙傛暟鐨勬寚閽堬紝閫氬父鏄竴涓粨鏋勩€?
## 鎻忚堪


ioctl() <func-ioctl> 鍑芥暟鐢ㄤ簬缂栫▼ V4L2 璁惧銆傚弬鏁?`fd` 蹇呴』鏄竴涓凡鎵撳紑鐨?鏂囦欢鎻忚堪绗︺€俰octl `request` 涓紪鐮佷簡鍙傛暟鏄緭鍏ャ€佽緭鍑鸿繕鏄鍐欏弬鏁帮紝浠ュ強
鍙傛暟 `argp` 鐨勫ぇ灏忥紙瀛楄妭鏁帮級銆傛寚瀹?V4L2 ioctl 璇锋眰鐨勫畯涓?define 浣嶄簬
`videodev2.h` 澶存枃浠朵腑銆傚簲鐢ㄧ▼搴忓簲浣跨敤鑷繁鐨勫壇鏈紝鑰岄潪鍖呭惈鍏剁紪璇戞墍鍦ㄧ郴缁?鍐呮牳婧愮爜涓殑鐗堟湰銆傛墍鏈?V4L2 ioctl 璇锋眰鍙婂叾鍚勮嚜鐨勫嚱鏁颁笌鍙傛暟鍦?user-func 涓?璇存槑銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
褰撻噰鐢ㄨ緭鍑烘垨璇诲啓鍙傛暟鐨?ioctl 澶辫触鏃讹紝璇ュ弬鏁颁繚鎸佷笉鍙樸€?