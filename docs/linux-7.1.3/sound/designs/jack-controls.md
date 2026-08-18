## ALSA Jack 鎺т欢


## 鎴戜滑涓轰粈涔堥渶瑕?Jack kcontrol


ALSA 浣跨敤 kcontrol 鍚戠敤鎴风┖闂村鍑洪煶棰戞帶浠讹紙寮€鍏炽€侀煶閲忋€佸璺鐢ㄥ櫒绛夛級銆傝繖鎰忓懗鐫€鍍?pulseaudio 杩欐牱鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ュ湪娌℃湁鎻掑叆鑰虫満鏃跺叧闂€虫満骞舵墦寮€鎵０鍣ㄣ€?

鏃х殑 ALSA jack 浠ｇ爜浠呬负姣忎釜娉ㄥ唽鐨?jack 鍒涘缓杈撳叆璁惧銆傝繖浜?jack 杈撳叆璁惧鏃犳硶琚互闈?root 韬唤杩愯鐨勭敤鎴风┖闂磋澶囪鍙栥€?

鏂扮殑 jack 浠ｇ爜涓烘瘡涓?jack 鍒涘缓宓屽叆寮?jack kcontrol锛屼换浣曡繘绋嬮兘鍙互璇诲彇銆?

杩欏彲浠ョ粨鍚?UCM锛屼娇鐢ㄦ埛绌洪棿鑳藉鏍规嵁 jack 鎻掑叆鎴栨嫈鍑轰簨浠舵洿鏅鸿兘鍦拌矾鐢遍煶棰戙€?

## Jack Kcontrol 鍐呴儴鏈哄埗


姣忎釜 jack 閮戒細鏈変竴涓?kcontrol 鍒楄〃锛屼互渚挎垜浠彲浠ュ湪 jack 鍒涘缓闃舵鍒涘缓涓€涓?kcontrol 骞跺皢鍏堕檮鍔犲埌 jack 涓娿€傛垜浠篃鍙互鍦ㄤ换浣曢渶瑕佺殑鏃跺€欏悜宸叉湁鐨?jack 娣诲姞 kcontrol銆?

褰?Jack 琚噴鏀炬椂锛岃繖浜?kcontrol 浼氳鑷姩閲婃斁銆?

## 濡備綍浣跨敤 jack kcontrol


涓轰簡淇濇寔鍏煎鎬э紝snd_jack_new() 琚慨鏀癸紝娣诲姞浜嗕袱涓弬鏁帮細

initial_kctl
  濡傛灉涓?true锛屽垯鍒涘缓涓€涓?kcontrol 骞跺皢鍏舵坊鍔犲埌 jack 鍒楄〃銆?
phantom_jack
  涓嶄负 phantom jack 鍒涘缓杈撳叆璁惧銆?

HDA jack 鍙互灏?phantom_jack 璁句负 true 浠ュ垱寤轰竴涓?phantom jack锛屽苟灏?initial_kctl 璁句负 true 浠ヤ娇鐢ㄦ纭殑 id 鍒涘缓涓€涓垵濮?kcontrol銆?

ASoC jack 搴斿皢 initial_kctl 璁句负 false銆傚紩鑴氬悕绉板皢琚祴涓?jack kcontrol 鍚嶇О銆?
