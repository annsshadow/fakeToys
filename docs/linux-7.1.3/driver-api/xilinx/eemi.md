## Xilinx Zynq MPSoC EEMI 鏂囨。


### Xilinx Zynq MPSoC 鍥轰欢鎺ュ彛

zynqmp-firmware 鑺傜偣鎻忚堪浜嗕笌骞冲彴鍥轰欢锛坧latform firmware锛夌殑鎺ュ彛銆俍ynqMP 鏈変竴涓笌瀹夊叏绠＄悊鍥轰欢閫氫俊鐨勬帴鍙ｃ€傚浐浠堕┍鍔ㄦ彁渚涗簡璁块棶鍥轰欢 API 鐨勬帴鍙ｃ€備换浣曢┍鍔ㄩ兘鍙互浣跨敤鎺ュ彛 API 涓?PMC锛堝钩鍙扮鐞嗘帶鍒跺櫒锛孭latform Management Controller锛夐€氫俊銆?

### 宓屽叆寮忚兘婧愮鐞嗘帴鍙ｏ紙EEMI锛?

宓屽叆寮忚兘婧愮鐞嗘帴鍙ｇ敤浜庡厑璁稿湪鑺墖鎴栬澶囦笂涓嶅悓澶勭悊绨囦笂杩愯鐨勮蒋浠剁粍浠朵笌璁惧涓婄殑鐢垫簮绠＄悊鎺у埗鍣紙PMC锛夐€氫俊锛屼互鍙戝嚭鎴栧搷搴旂數婧愮鐞嗚姹傘€?

浠讳綍甯屾湜閫氳繃 EEMI API 涓?PMC 閫氫俊鐨勯┍鍔ㄩ兘浣跨敤涓烘瘡涓嚱鏁版彁渚涚殑鍑芥暟銆?

### IOCTL

IOCTL API 鐢ㄤ簬璁惧鎺у埗鍜岄厤缃€傚畠涓嶆槸绯荤粺 IOCTL锛岃€屾槸 EEMI API銆傝 API 鍙敱涓昏澶囷紙master锛夌敤浜庢帶鍒朵换浣曠壒瀹氫簬璁惧鐨勯厤缃€侷OCTL 瀹氫箟鍙兘鐗瑰畾浜庡钩鍙般€傝 API 杩樼鐞嗗叡浜澶囬厤缃€?

浠ヤ笅 IOCTL ID 瀵硅澶囨帶鍒舵湁鏁堬細
- IOCTL_SET_PLL_FRAC_MODE	8
- IOCTL_GET_PLL_FRAC_MODE	9
- IOCTL_SET_PLL_FRAC_DATA	10
- IOCTL_GET_PLL_FRAC_DATA	11

鏈夊叧 IOCTL 鐗瑰畾鍙傛暟鍜屽叾浠?EEMI API锛岃鍙傞槄 EEMI API 鎸囧崡[^0^]銆?

### 鍙傝€?

[^0^] 宓屽叆寮忚兘婧愮鐞嗘帴鍙ｏ紙EEMI锛堿PI 鎸囧崡锛?
    https://www.xilinx.com/support/documentation/user_guides/ug1200-eemi-api.pdf
