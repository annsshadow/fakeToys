
## 铏氭嫙 PCM 娴嬭瘯椹卞姩

铏氭嫙 PCM 娴嬭瘯椹卞姩妯℃嫙涓€涓€氱敤鐨?PCM 璁惧锛屽彲鐢ㄤ簬瀵圭敤鎴锋€?ALSA 搴旂敤杩涜
娴嬭瘯/妯＄硦娴嬭瘯锛屼篃鍙敤浜庡 PCM 涓棿灞傝繘琛屾祴璇?妯＄硦娴嬭瘯銆傛澶栵紝瀹冭繕鍙敤浜?妯℃嫙闅句互澶嶇幇鐨?PCM 璁惧闂銆?
#### 璇ラ┍鍔ㄨ兘鍋氫粈涔堬紵

鐩墠璇ラ┍鍔ㄥ彲浠ュ畬鎴愪互涓嬩簨鎯咃細
 - 妯℃嫙閲囬泦锛坈apture锛変笌鍥炴斁锛坧layback锛夎繃绋? - 鐢熸垚闅忔満鎴栧熀浜庢ā寮忥紙pattern锛夌殑閲囬泦鏁版嵁
 - 鍚戝洖鏀惧拰閲囬泦杩囩▼涓敞鍏ュ欢杩? - 鍦?PCM 鍥炶皟鏈熼棿娉ㄥ叆閿欒

瀹冩渶澶氭敮鎸?8 涓瓙娴侊紙substream锛夊拰 4 涓€氶亾銆傚悓鏃舵敮鎸佷氦閿欙紙interleaved锛夊拰
闈炰氦閿欙紙non-interleaved锛夎闂ā寮忋€?
姝ゅ锛岃椹卞姩鍙互妫€鏌ュ洖鏀炬暟鎹祦鏄惁鍖呭惈棰勫畾涔夌殑妯″紡锛岃妯″紡鍦ㄥ搴旂殑鑷祴璇?锛坅lsa/pcmtest-test.sh锛変腑鐢ㄤ簬妫€鏌?PCM 涓棿灞傜殑鏁版嵁浼犺緭鍔熻兘銆傚彟澶栵紝璇ラ┍鍔?閲嶅畾涔変簡榛樿鐨?RESET ioctl锛岃嚜娴嬭瘯涔熶細瑕嗙洊姝?PCM API 鍔熻兘銆?
### 閰嶇疆

闄や簡閫氱敤鐨?ALSA 妯″潡鍙傛暟澶栵紝璇ラ┍鍔ㄨ繕鏈変互涓嬪弬鏁帮細

 - fill_mode (bool) - 缂撳啿鍖哄～鍏呮ā寮忥紙瑙佷笅鏂囷級
 - inject_delay (int)
 - inject_hwpars_err (bool)
 - inject_prepare_err (bool)
 - inject_trigger_err (bool)

### 閲囬泦鏁版嵁鐢熸垚

璇ラ┍鍔ㄦ湁涓ょ鏁版嵁鐢熸垚妯″紡锛氱涓€绉嶏紙fill_mode 鍙傛暟涓?0锛夎〃绀洪殢鏈烘暟鎹敓鎴愶紝
绗簩绉嶏紙fill_mode 涓?1锛夎〃绀哄熀浜庢ā寮忕殑鏁版嵁鐢熸垚銆傛垜浠潵鐪嬬湅绗簩绉嶆ā寮忋€?
棣栧厛锛屼綘鍙兘闇€瑕佹寚瀹氱敤浜庢暟鎹敓鎴愮殑妯″紡銆傚彲浠ラ€氳繃鎶婃ā寮忓啓鍏?debugfs 鏂囦欢鏉?瀹炵幇銆傛瘡涓€氶亾閮芥湁瀵瑰簲鐨勬ā寮忕紦鍐插尯 debugfs 椤癸紝浠ュ強鍖呭惈妯″紡缂撳啿鍖洪暱搴︾殑
椤广€?
 - /sys/kernel/debug/pcmtest/fill_pattern[0-3]
 - /sys/kernel/debug/pcmtest/fill_pattern[0-3]_len

瑕佷负閫氶亾 0 璁剧疆妯″紡锛屽彲浠ユ墽琛屼互涓嬪懡浠わ細


	echo -n mycoolpattern > /sys/kernel/debug/pcmtest/fill_pattern0

涔嬪悗锛屽 'pcmtest' 璁惧鐨勬瘡娆￠噰闆嗘搷浣滅粨鏉熷悗锛岄€氶亾 0 鐨勭紦鍐插尯閮戒細鍖呭惈
'mycoolpatternmycoolpatternmycoolpatternmy...'銆?
妯″紡鏈韩鏈€闀垮彲杈?4096 瀛楄妭銆?
### 寤惰繜娉ㄥ叆

璇ラ┍鍔ㄦ湁 'inject_delay' 鍙傛暟锛屽叾鍚嶇О闈炲父鐩磋锛屽彲鐢ㄤ簬妯℃嫙鏃堕棿寤惰繜/鍔犻€熴€傝
鍙傛暟涓烘暣鏁扮被鍨嬶紝琛ㄧず鍦ㄦā鍧楀唴閮ㄥ畾鏃跺櫒鑺傛媿涔嬮棿娣诲姞鐨勫欢杩熴€?
濡傛灉 'inject_delay' 鍊间负姝ｏ紝缂撳啿鍖哄～鍏呬細鍙樻參锛涘鏋滀负璐燂紝鍒欎細鍙樺揩銆備綘鍙互
鑷繁灏濊瘯锛氬湪浠讳綍褰曢煶搴旂敤锛堝 Audacity锛変腑鍚姩褰曞埗锛屽苟閫夋嫨 'pcmtest' 璁惧
浣滀负闊虫簮銆?
璇ュ弬鏁颁篃鍙敤浜庡湪涓€涓潪甯哥煭鐨勬椂闂存鍐呯敓鎴愬ぇ閲忕殑澹伴煶鏁版嵁锛堜娇鐢ㄨ礋鐨?'inject_delay' 鍊硷級銆?
### 閿欒娉ㄥ叆

璇ユā鍧楀彲鐢ㄤ簬鍚?PCM 閫氫俊杩囩▼涓敞鍏ラ敊璇€傝繖涓€鎿嶄綔鏈夊姪浜庝綘浜嗚В鐢ㄦ埛鎬?ALSA
绋嬪簭鍦ㄥ紓甯告儏鍐典笅濡備綍琛ㄧ幇銆?
渚嬪锛屽彲浠ラ€氳繃鍚?'inject_hwpars_err' 妯″潡鍙傛暟鍐欏叆 '1'锛屼娇鎵€鏈?'hw_params'
PCM 鍥炶皟杩斿洖 EBUSY 閿欒锛?

	echo 1 > /sys/module/snd_pcmtest/parameters/inject_hwpars_err

鍙互鍚戜互涓?PCM 鍥炶皟娉ㄥ叆閿欒锛?
 - hw_params (EBUSY)
 - prepare (EINVAL)
 - trigger (EINVAL)

### 鍥炴斁娴嬭瘯

璇ラ┍鍔ㄤ篃鍙敤浜庡洖鏀惧姛鑳芥祴璇曗€斺€旀瘡褰撲綘鍚?'pcmtest' PCM 璁惧鍐欏叆鍥炴斁鏁版嵁骞跺叧闂?瀹冩椂锛岄┍鍔ㄤ細妫€鏌ョ紦鍐插尯鏄惁鍖呭惈寰幆妯″紡锛堣妯″紡鍦ㄦ瘡涓€氶亾鐨?fill_pattern
debugfs 鏂囦欢涓寚瀹氾級銆傚鏋滃洖鏀剧紦鍐插尯鍐呭琛ㄧず寰幆妯″紡锛屽垯 'pc_test' debugfs
椤硅璁句负 '1'銆傚惁鍒欙紝椹卞姩灏嗗叾璁句负 '0'銆?
### ioctl 閲嶅畾涔夋祴璇?
璇ラ┍鍔ㄩ噸瀹氫箟浜嗘墍鏈?PCM 璁惧榛樿鐨?'reset' ioctl銆傝娴嬭瘯姝ゅ姛鑳斤紝鎴戜滑鍙互
瑙﹀彂 reset ioctl 骞舵鏌?'ioctl_test' debugfs 椤癸細


	cat /sys/kernel/debug/pcmtest/ioctl_test

濡傛灉 ioctl 瑙﹀彂鎴愬姛锛岃鏂囦欢灏嗗寘鍚?'1'锛屽惁鍒欎负 '0'銆?