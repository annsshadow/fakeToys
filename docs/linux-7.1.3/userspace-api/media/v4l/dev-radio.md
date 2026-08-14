


######## 鏃犵嚎鐢垫帴鍙?


璇ユ帴鍙ｉ潰鍚?AM 鍜?FM锛堟ā鎷燂級鏃犵嚎鐢垫帴鏀舵満涓?
鍙戝皠鏈恒€?

浼犵粺涓婏紝V4L2 鏃犵嚎鐢佃澶囬€氳繃鍚嶄负 `/dev/radio` 鍜?`/dev/radio0` 鑷?
`/dev/radio63` 鐨勫瓧绗﹁澶囩壒娈婃枃浠惰闂紝
鍏朵富璁惧鍙蜂负 81锛屾璁惧鍙蜂负 64 鑷?127銆?


## 鏌ヨ鑳藉姏


鏀寔鏃犵嚎鐢垫帴鍙ｇ殑璁惧浼氬湪
`v4l2_capability` 缁撴瀯浣擄紙鐢?VIDIOC_QUERYCAP ioctl 杩斿洖锛夌殑
`capabilities` 瀛楁涓缃?`V4L2_CAP_RADIO`
浠ュ強 `V4L2_CAP_TUNER` 鎴?`V4L2_CAP_MODULATOR` 鏍囧織銆?
鍏朵粬鑳藉姏鏍囧織鐨?
缁勫悎淇濈暀渚涘皢鏉ユ墿灞曘€?


## 闄勫姞鍔熻兘


鏃犵嚎鐢佃澶囧彲鏀寔 controls <control>锛屼笖蹇呴』鏀寔
tuner 鎴?modulator <tuner> ioctls銆?

瀹冧滑涓嶆敮鎸佽棰戣緭鍏ユ垨杈撳嚭銆侀煶棰戣緭鍏ユ垨杈撳嚭銆?
瑙嗛鍒跺紡銆佽鍓笌缂╂斁銆佸帇缂╀笌娴?
鍙傛暟锛屾垨 overlay ioctls銆傛墍鏈夊叾浠?ioctls 鍜?I/O 鏂规硶鍧?
淇濈暀渚涘皢鏉ユ墿灞曘€?


## 缂栫▼


鏃犵嚎鐢佃澶囧彲鑳藉叿鏈夎嫢骞查煶棰戞帶鍒讹紙濡?control 涓墍杩帮級锛?
渚嬪闊抽噺鎺у埗锛屼篃鍙兘鏈夎嚜瀹氫箟鎺у埗銆?

姝ゅ锛屾墍鏈夋棤绾跨數璁惧閮芥湁涓€涓?tuner 鎴?modulator锛堝湪 tuner 涓璁猴級锛?
鍏剁储寮曞彿涓?0锛岀敤浜庨€夋嫨鏃犵嚎鐢?
棰戠巼锛屽苟纭畾鎺ユ敹/鍙戝皠鐨勬槸鍗曞０閬撹繕鏄?FM 绔嬩綋澹拌妭鐩€?
椹卞姩浼氭牴鎹墍閫夐鐜囧湪 AM 鍜?FM 涔嬮棿鑷姩鍒囨崲銆?
VIDIOC_G_TUNER <VIDIOC_G_TUNER> 鎴?
VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> ioctl 鎶ュ憡
鏀寔鐨勯鐜囪寖鍥淬€?
