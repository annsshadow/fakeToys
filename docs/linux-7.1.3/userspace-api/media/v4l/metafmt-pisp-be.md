


######## V4L2_META_FMT_RPI_BE_CFG


## Raspberry Pi PiSP 鍚庣锛圔ack End锛夐厤缃牸寮?

Raspberry Pi PiSP 鍚庣鍐呭瓨鍒板唴瀛?鍥惧儚淇″彿澶勭悊鍣ㄧ敱鐢ㄦ埛绌洪棿閫氳繃 `v4l2_meta_format` 鎺ュ彛锛屽悜 `pispbe-config` 杈撳嚭瑙嗛璁惧鑺傜偣鎻愪緵涓€缁勯厤缃弬鏁扮紦鍐插尯鏉ヨ繘琛岄厤缃€?
PiSP 鍚庣浠ュ垎鍧楋紙tiles锛夋柟寮忓鐞嗗浘鍍忥紝鍏堕厤缃渶瑕佸～鍏?`pisp_be_config.h` 澶存枃浠朵腑瀹氫箟鐨?`pisp_be_tiles_config` 鐨勬垚鍛橈紝浠ユ寚瀹氫袱缁勪笉鍚岀殑鍙傛暟銆?
`Raspberry Pi PiSP technical specification
<https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_
鎻愪緵浜嗗 ISP 鍚庣閰嶇疆鍜岀紪绋嬫ā鍨嬬殑璇︾粏鎻忚堪銆?
### 鍏ㄥ眬閰嶇疆鏁版嵁


鍏ㄥ眬閰嶇疆鏁版嵁鎻忚堪浜嗙壒瀹氬浘鍍忎腑鐨勫儚绱犲簲褰撳浣曞鐞嗭紝鍥犳鍦ㄥ浘鍍忕殑鎵€鏈夊垎鍧椾箣闂村叡浜€備緥濡傦紝LSC锛堥暅澶撮槾褰辨牎姝ｏ紝Lens Shading Correction锛夋垨闄嶅櫔锛圖enoise锛夊弬鏁板湪鍚屼竴甯х殑鎵€鏈夊垎鍧椾腑鏄€氱敤鐨勩€?
鍏ㄥ眬閰嶇疆鏁版嵁閫氳繃濉厖 `pisp_be_config` 鐨勬垚鍛樹紶閫掔粰 ISP銆?
### 鍒嗗潡锛圱ile锛夊弬鏁?

鐢变簬 ISP 浠ュ垎鍧楁柟寮忓鐞嗗浘鍍忥紝姣忎竴缁勫垎鍧楀弬鏁版弿杩颁簡鍥惧儚涓崟涓垎鍧楀皢濡備綍澶勭悊銆備竴缁勫垎鍧楀弬鏁扮敱 160 瀛楄妭鐨勬暟鎹粍鎴愶紝瑕佸鐞嗕竴鎵瑰垎鍧楅渶瑕佸缁勫垎鍧楀弬鏁般€?
鍒嗗潡鍙傛暟閫氳繃濉厖 `pisp_tile` 鐨勬垚鍛樹互鍙?`pisp_be_tiles_config` 鐨?`num_tiles` 瀛楁浼犻€掔粰 ISP銆?
## Raspberry Pi PiSP 鍚庣 uAPI 鏁版嵁绫诲瀷


鏈妭鎻忚堪鐢?Raspberry Pi PiSP 鍚庣鏆撮湶缁欑敤鎴风┖闂寸殑鏁版嵁绫诲瀷銆傛湰鑺備粎渚涘弬鑰冿紝鍏充簬姣忎釜瀛楁鐨勮缁嗘弿杩帮紝璇峰弬鑰?`Raspberry Pi PiSP technical specification
<https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_銆?