
## 铏氭嫙濯掍綋鎺у埗鍣ㄩ┍鍔紙vimc锛?

vimc 椹卞姩浣跨敤 V4L2 API 鍜?Media API 妯℃嫙澶嶆潅鐨勮棰戠‖浠躲€傚畠鏈変竴涓崟鑾疯澶囦互鍙婁笁涓瓙璁惧锛歴ensor锛堜紶鎰熷櫒锛夈€乨ebayer锛堝幓鎷滆€筹級鍜?scaler锛堢缉鏀惧櫒锛夈€?
### 鎷撴墤


鎷撴墤鏄‖缂栫爜鐨勶紝涓嶈繃浣犲彲浠ヤ慨鏀?vimc-core 骞堕噸鏂扮紪璇戦┍鍔ㄦ潵瀹炵幇鑷繁鐨勬嫇鎵戙€傝繖鏄粯璁ゆ嫇鎵戯細


    :alt:   榛樿濯掍綋娴佹按绾挎嫇鎵戝浘
    :align: center

    vimc 涓婄殑濯掍綋娴佹按绾垮浘

#### 閰嶇疆鎷撴墤


姣忎釜瀛愯澶囬兘浼氬甫鏈夊叾榛樿閰嶇疆锛坧ixelformat銆乭eight銆亀idth 绛夛級銆傞渶瑕侀厤缃鎷撴墤锛屼娇姣忎釜琚摼鎺ュ瓙璁惧涓婄殑閰嶇疆鐩镐簰鍖归厤锛屾墠鑳介€氳繃娴佹按绾挎祦寮忎紶杈撳抚銆傚鏋滈厤缃笉鍖归厤锛屾祦灏嗗け璐ャ€俙v4l-utils` 杞欢鍖呮槸涓€缁勭敤鎴风┖闂村簲鐢ㄧ▼搴忕殑闆嗗悎锛岃嚜甯?`media-ctl` 鍜?`v4l2-ctl`锛屽彲鐢ㄤ簬閰嶇疆 vimc 閰嶇疆銆備互涓嬪懡浠ゅ簭鍒楅€傜敤浜庨粯璁ゆ嫇鎵戯細


        media-ctl -d platform:vimc -V '"Sensor A":0[fmt:SBGGR8_1X8/640x480]'
        media-ctl -d platform:vimc -V '"Debayer A":0[fmt:SBGGR8_1X8/640x480]'
        media-ctl -d platform:vimc -V '"Scaler":0[fmt:RGB888_1X24/640x480]'
        media-ctl -d platform:vimc -V '"Scaler":0[crop:(100,50)/400x150]'
        media-ctl -d platform:vimc -V '"Scaler":1[fmt:RGB888_1X24/300x700]'
        v4l2-ctl -z platform:vimc -d "RGB/YUV Capture" -v width=300,height=700
        v4l2-ctl -z platform:vimc -d "Raw Capture 0" -v pixelformat=BA81

### 瀛愯澶?

瀛愯澶囧畾涔変簡鎷撴墤涓疄浣擄紙entity锛夌殑琛屼负銆傛牴鎹瓙璁惧鐨勪笉鍚岋紝瀹炰綋鍙互鍏锋湁澶氫釜 source 鎴?sink 绫诲瀷鐨?pad銆?
vimc-sensor:
	浣跨敤瑙嗛娴嬭瘯鍥炬鐢熸垚鍣ㄤ互澶氱鏍煎紡鐢熸垚鍥惧儚銆?	鏆撮湶锛?
 - 1 涓?source pad

vimc-lens:
	浼犳劅鍣ㄧ殑杈呭姪闀滃ご銆傛敮鎸佽嚜鍔ㄥ鐒︽帶鍒躲€備娇鐢ㄨ緟鍔╅摼鎺ワ紙ancillary link锛夎繛鎺ュ埌 vimc-sensor銆傝闀滃ご鏀寔 FOCUS_ABSOLUTE 鎺у埗銆?

	media-ctl -p
	...
 - entity 28: Lens A (0 pad, 0 link)
			type V4L2 subdev subtype Lens flags 0
			device node name /dev/v4l-subdev6
 - entity 29: Lens B (0 pad, 0 link)
			type V4L2 subdev subtype Lens flags 0
			device node name /dev/v4l-subdev7
	v4l2-ctl -d /dev/v4l-subdev7 -C focus_absolute
	focus_absolute: 0


vimc-debayer:
	灏嗘嫓鑰筹紙bayer锛夋牸寮忕殑鍥惧儚杞崲涓洪潪鎷滆€虫牸寮忋€?	鏆撮湶锛?
 - 1 涓?sink pad
 - 1 涓?source pad

vimc-scaler:
	閲嶆柊璋冩暣鍥惧儚澶у皬浠ュ尮閰?source pad 鐨勫垎杈ㄧ巼銆備緥濡傦細濡傛灉 sink pad 閰嶇疆涓?360x480 鑰?source 閰嶇疆涓?1280x720锛屽浘鍍忓皢琚媺浼镐互閫傞厤 source 鍒嗚鲸鐜囥€傞€傜敤浜?vimc 闄愬埗鍐呯殑浠讳綍鍒嗚鲸鐜囷紙蹇呰鏃剁敋鑷崇缉灏忓浘鍍忥級銆?	鏆撮湶锛?
 - 1 涓?sink pad
 - 1 涓?source pad

vimc-capture:
	鏆撮湶鑺傜偣 /dev/videoX 浠ュ厑璁哥敤鎴风┖闂存崟鑾锋祦銆?	鏆撮湶锛?
 - 1 涓?sink pad
 - 1 涓?source pad

### 妯″潡鍙傛暟


Vimc 鏈変竴涓敤浜庨厤缃┍鍔ㄧ殑妯″潡鍙傛暟銆?
- `allocator=<unsigned int>`

	鍐呭瓨鍒嗛厤鍣ㄩ€夋嫨锛岄粯璁や负 0銆傚畠鎸囧畾缂撳啿鍖虹殑鍒嗛厤鏂瑰紡銆?
  - 0: vmalloc
  - 1: dma-contig
