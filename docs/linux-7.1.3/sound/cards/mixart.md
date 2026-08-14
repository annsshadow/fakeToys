## Digigram miXart8 涓?miXart8AES/EBU 澹板崱鐨?Alsa 椹卞姩


Digigram <alsa@digigram.com>

## 姒傝堪


miXart8 鏄竴娆惧澹伴亾闊抽澶勭悊涓庢贩闊冲０鍗★紝鍏锋湁 4 涓珛浣撳０闊抽杈撳叆涓?4 涓珛浣撳０闊抽杈撳嚭銆?miXart8AES/EBU 涓庝箣鐩稿悓锛屼絾澧炲姞浜嗕竴鍧楅檮鍔犲崱锛屾彁渚涢澶栫殑 4 涓暟瀛楃珛浣撳０闊抽杈撳叆涓庤緭鍑恒€?姝ゅ锛岃闄勫姞鍗℃彁渚涘閮ㄦ椂閽熷悓姝ワ紙AES/EBU銆乄ord Clock銆乀ime Code 涓?Video Synchro锛夈€?
涓绘澘涓婃湁涓€涓?PowerPC锛屾彁渚涙澘杞?mpeg 缂栫爜涓庤В鐮併€侀噰鏍风巼杞崲浠ュ強鍚勭鏁堟灉銆?
鍦ㄥ姞杞界壒瀹氬浐浠朵箣鍓嶏紝椹卞姩鏍规湰鏃犳硶姝ｅ父宸ヤ綔锛屽嵆涓嶄細鍑虹幇浠讳綍 PCM 鎴栨贩闊冲櫒璁惧銆?璇蜂娇鐢?alsa-tools 杞欢鍖呬腑鐨?mixartloader銆?
## 鐗堟湰 0.1.0


涓€鍧?miXart8 鏉夸細琚〃绀轰负 4 涓?alsa 鍗★紝姣忎釜鍗℃湁 1 涓珛浣撳０妯℃嫙閲囬泦 'pcm0c' 涓?1 涓珛浣撳０妯℃嫙
鍥炴斁 'pcm0p' 璁惧銆傚浜?miXart8AES/EBU锛屾瘡鍧楀崱鍙﹀杩樻湁 1 涓珛浣撳０鏁板瓧杈撳叆 'pcm1c' 涓?1 涓珛浣撳０
鏁板瓧杈撳嚭 'pcm1p'銆?
### 鏍煎紡


U8銆丼16_LE銆丼16_BE銆丼24_3LE銆丼24_3BE銆丗LOAT_LE銆丗LOAT_BE
閲囨牱鐜囷細8000 - 48000 Hz 杩炵画

### 鍥炴斁


渚嬪锛屽洖鏀捐澶囪閰嶇疆涓烘渶澶?4 涓瓙娴佹墽琛岀‖浠舵贩闊炽€傚鏋滈渶瑕侊紝杩欏彲浠ユ洿鏀逛负鏈€澶?24 涓瓙娴併€?鍗曞０閬撴枃浠跺皢鍦ㄥ乏銆佸彸澹伴亾鎾斁銆傛瘡涓０閬撻兘鍙互涓烘瘡涓祦闈欓煶锛屼互鍗曠嫭浣跨敤 8 涓ā鎷?鏁板瓧杈撳嚭銆?
### 閲囬泦


姣忎釜閲囬泦璁惧鏈変竴涓瓙娴併€備緥濡備粎鏀寔绔嬩綋澹版牸寮忋€?
### 娣烽煶鍣?

<Master> 涓?<Master Capture>
	鍥炴斁涓庨噰闆?PCM 鐨勬ā鎷熼煶閲忔帶鍒躲€?<PCM 0-3> 涓?<PCM Capture>
	姣忎釜妯℃嫙瀛愭祦鐨勬暟瀛楅煶閲忔帶鍒躲€?<AES 0-3> 涓?<AES Capture>
	姣忎釜 AES/EBU 瀛愭祦鐨勬暟瀛楅煶閲忔帶鍒躲€?<Monitoring>
	浠?'pcm0c' 鍒?'pcm0p' 鐨勭幆鍥烇紝甯︽暟瀛楅煶閲忎笌闈欓煶鎺у埗銆?
娉ㄦ剰锛氫负鑾峰緱鏈€浣抽煶璐紝灏介噺璁?PCM 涓?AES 闊抽噺鎺у埗淇濇寔 0 琛板噺锛屽嵆鍦?0 鍒?255 鑼冨洿鍐呰涓?219
锛堜娇鐢?alsamixer 绾︿负 86%锛夈€?
## 灏氭湭瀹炵幇


- 澶栭儴鏃堕挓鏀寔锛圓ES/EBU銆乄ord Clock銆乀ime Code銆乂ideo Sync锛?- MPEG 闊抽鏍煎紡
- 鍗曞０閬撳綍闊?- 鏉胯浇鏁堟灉涓庨噰鏍风巼杞崲
- 閾炬帴娴?
## 鍥轰欢


[鑷?2.6.11 璧凤紝褰撹缃簡 CONFIG_FW_LOADER 鏃讹紝鍥轰欢鍙互閫氳繃鐑彃鎷旇嚜鍔ㄥ姞杞姐€俶ixartloader 浠呭
杈冩棫鐗堟湰鎴栧皢椹卞姩缂栬瘧杩涘唴鏍告椂鏄繀闇€鐨勩€俔

瑕佸湪妯″潡鍔犺浇鍚庤嚜鍔ㄥ姞杞藉浐浠讹紝璇蜂娇鐢?install 鍛戒护銆備緥濡傦紝灏嗕互涓嬫潯鐩坊鍔犲埌 miXart 椹卞姩鐨?/etc/modprobe.d/mixart.conf锛?```

	install snd-mixart /sbin/modprobe --first-time -i snd-mixart && \
			   /usr/bin/mixartloader


```
锛堝浜?2.2/2.4 鍐呮牳锛屾敼涓哄皢 "post-install snd-mixart /usr/bin/vxloader" 娣诲姞鍒?/etc/modules.conf銆傦級

鍥轰欢浜岃繘鍒舵枃浠跺畨瑁呭湪 /usr/share/alsa/firmware锛堟垨 /usr/local/share/alsa/firmware锛屽彇鍐充簬
configure 鐨?prefix 閫夐」锛夈€傚叾涓細鏈変竴涓?miXart.conf 鏂囦欢锛屽畾涔?dsp 鏄犲儚鏂囦欢銆?
鍥轰欢鏂囦欢鐨勭増鏉冨綊 Digigram SA 鎵€鏈夈€?
## 鐗堟潈


Copyright (c) 2003 Digigram SA <alsa@digigram.com>
鍙湪 GPL 涓嬪垎鍙戙€?