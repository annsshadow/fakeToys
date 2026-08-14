## ALSA 椹卞姩绋嬪簭鐨?Proc 鏂囦欢


Takashi Iwai <tiwai@suse.de>

## 姒傝堪


ALSA 鎷ユ湁鑷繁鐨?proc 鏍戯紝鍗?/proc/asound銆傝澶氭湁鐢ㄧ殑淇℃伅閮藉彲浠ュ湪璇ユ爲涓壘鍒般€傚綋浣犻亣鍒?
闂闇€瑕佽皟璇曟椂锛岃妫€鏌ヤ笅闈㈠悇鑺備腑鍒楀嚭鐨勬枃浠躲€?

姣忓紶澹板崱閮芥湁瀹冭嚜宸辩殑瀛愭爲 cardX锛屽叾涓?X 鍙栧€间负 0 鍒?7銆傜壒瀹氫簬澹板崱鐨勬枃浠跺瓨鍌ㄥ湪 `card*` 瀛愮洰褰曚腑銆?


## 鍏ㄥ眬淇℃伅


cards
	鏄剧ず褰撳墠宸查厤缃殑 ALSA 椹卞姩鍒楄〃銆佺储寮曘€乮d 瀛楃涓层€佺畝鐭笌璇︾粏鎻忚堪銆?

version
	鏄剧ず鐗堟湰瀛楃涓蹭笌缂栬瘧鏃ユ湡銆?

modules
	鍒楀嚭姣忓紶澹板崱鐨勬ā鍧?

devices
	鍒楀嚭 ALSA 鍘熺敓璁惧鏄犲皠銆?

meminfo
	鏄剧ず閫氳繃 ALSA 椹卞姩鍒嗛厤鐨勯〉闈㈢姸鎬併€?
	浠呭湪 `CONFIG_SND_DEBUG=y` 鏃跺嚭鐜般€?

hwdep
	浠?`<card>-<device>: <name>` 鐨勬牸寮忓垪鍑哄綋鍓嶅彲鐢ㄧ殑 hwdep 璁惧

pcm
	浠?`<card>-<device>: <id>: <name> : <sub-streams>` 鐨勬牸寮忓垪鍑哄綋鍓嶅彲鐢ㄧ殑 PCM 璁惧

timer
	鍒楀嚭褰撳墠鍙敤鐨勫畾鏃跺櫒璁惧


oss/devices
	鍒楀嚭 OSS 璁惧鏄犲皠銆?

oss/sndstat
	鎻愪緵涓?/dev/sndstat 鍏煎鐨勮緭鍑恒€?
	浣犲彲浠ュ皢鍏剁鍙烽摼鎺ュ埌 /dev/sndstat銆?


## 鐗瑰畾浜庡０鍗＄殑鏂囦欢


鐗瑰畾浜庡０鍗＄殑鏂囦欢浣嶄簬 `/proc/asound/card*` 鐩綍涓€備竴浜涢┍鍔紙渚嬪 cmipci锛夋嫢鏈夎嚜宸辩殑
proc 鏉＄洰鐢ㄤ簬瀵勫瓨鍣ㄨ浆鍌ㄧ瓑锛堜緥濡?`/proc/asound/card*/cmipci` 鏄剧ず瀵勫瓨鍣ㄨ浆鍌級銆傝繖浜涙枃浠?
瀵硅皟璇曢潪甯告湁甯姪銆?

褰撹澹板崱涓婃湁鍙敤鐨?PCM 璁惧鏃讹紝浣犲彲浠ョ湅鍒拌濡?pcm0p 鎴?pcm1c 杩欐牱鐨勭洰褰曘€傚畠浠繚瀛樻瘡涓?
PCM 娴佺殑 PCM 淇℃伅銆俙pcm` 涔嬪悗鐨勬暟瀛楁槸 PCM 璁惧鍙凤紙浠?0 寮€濮嬶級锛屾湯灏剧殑 `p` 鎴?`c` 琛ㄧず
鍥炴斁锛坧layback锛夋垨鎹曡幏锛坈apture锛夋柟鍚戙€傛瀛愭爲涓殑鏂囦欢灏嗗湪鍚庢枃鎻忚堪銆?

MIDI I/O 鐨勭姸鎬佷綅浜?`midi*` 鏂囦欢涓€傚畠鏄剧ず璁惧鍚嶇О浠ュ強閫氳繃 MIDI 璁惧鎺ユ敹/鍙戦€佺殑瀛楄妭鏁般€?

褰撳０鍗￠厤澶?AC97 缂栬В鐮佸櫒鏃讹紝浼氭湁 `codec97#*` 瀛愮洰褰曪紙鍚庢枃鎻忚堪锛夈€?

褰撳惎鐢ㄤ簡 OSS 娣烽煶鍣ㄦā鎷燂紙涓旀ā鍧楀凡鍔犺浇锛夋椂锛岃繖閲屼篃浼氬嚭鐜?oss_mixer 鏂囦欢銆傚畠鏄剧ず褰撳墠 OSS
娣烽煶鍣ㄥ厓绱犲埌 ALSA 鎺у埗鍏冪礌鐨勬槧灏勩€備綘鍙互閫氳繃鍐欏叆璇ヨ澶囨潵鏇存敼鏄犲皠銆傝鎯呰闃呰
OSS-Emulation.txt銆?


## PCM Proc 鏂囦欢


`card**/pcm**/info`
	姝?PCM 璁惧鐨勯€氱敤淇℃伅锛氬０鍗＄紪鍙枫€佽澶囩紪鍙枫€佸瓙娴佺瓑銆?

`card**/pcm**/xrun_debug`
	褰?`CONFIG_SND_DEBUG=y` 涓?`CONFIG_SND_PCM_XRUN_DEBUG=y` 鏃舵鏂囦欢鍑虹幇銆?
	瀹冩樉绀?xrun锛? 缂撳啿鍖烘孩鍑?娆犺浇锛夌殑鐘舵€侊紝浠ュ強瀵?ALSA PCM 涓棿灞傜殑
	鏃犳晥 PCM 浣嶇疆璋冭瘯/妫€鏌ャ€傚畠鎺ュ彈涓€涓暣鏁板€硷紝鍙互閫氳繃鍐欏叆鏉ユ洿鏀?

```

		 # echo 5 > /proc/asound/card0/pcm0p/xrun_debug

	The value consists of the following bit flags:

	* bit 0 = Enable XRUN/jiffies debug messages
	* bit 1 = Show stack trace at XRUN / jiffies check
	* bit 2 = Enable additional jiffies check

	When the bit 0 is set, the driver will show the messages to
	kernel log when an xrun is detected.  The debug message is
	shown also when the invalid H/W pointer is detected at the
	update of periods (usually called from the interrupt
	handler).

	When the bit 1 is set, the driver will show the stack trace
	additionally.  This may help the debugging.

	Since 2.6.30, this option can enable the hwptr check using
	jiffies.  This detects spontaneous invalid pointer callback
	values, but can be lead to too much corrections for a (mostly
	buggy) hardware that doesn't give smooth pointer updates.
	This feature is enabled via the bit 2.

```
`card**/pcm**/sub*/info`
	姝?PCM 瀛愭祦鐨勯€氱敤淇℃伅銆?

`card**/pcm**/sub*/status`
	姝?PCM 瀛愭祦鐨勫綋鍓嶇姸鎬併€佺粡杩囨椂闂淬€佺‖浠朵綅缃瓑銆?

`card**/pcm**/sub*/hw_params`
	涓烘瀛愭祦璁剧疆鐨勭‖浠跺弬鏁般€?

`card**/pcm**/sub*/sw_params`
	涓烘瀛愭祦璁剧疆鐨勮蒋浠跺弬鏁般€?

`card**/pcm**/sub*/prealloc`
	缂撳啿鍖洪鍒嗛厤淇℃伅銆?

`card**/pcm**/sub*/xrun_injection`
	褰撳悜姝?proc 鏂囦欢鍐欏叆浠绘剰鍊兼椂锛屼細鍚戞鍦ㄨ繍琛岀殑娴佽Е鍙戜竴涓?XRUN銆傜敤浜庢晠闅滄敞鍏ャ€?
	姝ゆ潯鐩槸鍙啓鐨勩€?

## AC97 缂栬В鐮佸櫒淇℃伅


`card**/codec97#**/ac97#?-?`
	鏄剧ず姝?AC97 缂栬В鐮佸櫒鑺墖鐨勯€氱敤淇℃伅锛屼緥濡傚悕绉般€佽兘鍔涖€佽缃€?

`card*/codec97#0/ac97#?-?+regs`
	鏄剧ず AC97 瀵勫瓨鍣ㄨ浆鍌ㄣ€傚璋冭瘯寰堟湁鐢ㄣ€?

	褰撳惎鐢ㄤ簡 CONFIG_SND_DEBUG 鏃讹紝浣犲彲浠ュ啓鍏ユ鏂囦欢浠ョ洿鎺ユ洿鏀?AC97 瀵勫瓨鍣ㄣ€備紶鍏ヤ袱涓崄鍏繘鍒舵暟銆?
	渚嬪锛?

```

	# echo 02 9f1f > /proc/asound/card0/codec97#0/ac97#0-0+regs


```
## USB 闊抽娴?


`card**/stream**`
	鏄剧ず缁欏畾澹板崱涓瘡涓煶棰戞祦鐨勫垎閰嶄笌褰撳墠鐘舵€併€傛淇℃伅瀵硅皟璇曢潪甯告湁鐢ㄣ€?


## HD-Audio 缂栬В鐮佸櫒


`card**/codec#**`
	鏄剧ず閫氱敤缂栬В鐮佸櫒淇℃伅浠ュ強姣忎釜 widget 鑺傜偣鐨勫睘鎬с€?

`card**/eld#**`
	鍙敤浜?HDMI 鎴?DisplayPort 鎺ュ彛銆?
	鏄剧ず浠庢墍杩?HDMI 鎺ユ敹绔幏鍙栫殑 ELD锛圗DID Like Data锛岀被 EDID 鏁版嵁锛変俊鎭紝
	骞舵弿杩板叾闊抽鑳藉姏涓庨厤缃€?

	鍙互閫氳繃鎵ц `echo name hex_value > eld#*` 鏉ヤ慨鏀规煇浜?ELD 瀛楁銆?
	鍙湁鍦ㄤ綘纭畾 HDMI 鎺ユ敹绔彁渚涚殑鍊兼湁璇椂鎵嶈繖鏍峰仛銆傚鏋滆繖鏍疯兘璁╀綘鐨?HDMI 闊抽宸ヤ綔锛?
	璇峰悜鎴戜滑鎶ュ憡锛屼互渚挎垜浠湪鏈潵鐨勫唴鏍哥増鏈腑淇瀹冦€?


## 瀹氬簭鍣紙Sequencer锛変俊鎭?


seq/drivers
	鍒楀嚭褰撳墠鍙敤鐨?ALSA 瀹氬簭鍣ㄩ┍鍔ㄣ€?

seq/clients
	鏄剧ず褰撳墠鍙敤鐨勫畾搴忓櫒瀹㈡埛绔笌绔彛鍒楄〃銆傝繛鎺ョ姸鎬佷笌杩愯鐘舵€佷篃鏄剧ず鍦ㄦ鏂囦欢涓€?

seq/queues
	鍒楀嚭褰撳墠宸插垎閰?杩愯鐨勫畾搴忓櫒闃熷垪銆?

seq/timer
	鍒楀嚭褰撳墠宸插垎閰?杩愯鐨勫畾搴忓櫒瀹氭椂鍣ㄣ€?

seq/oss
	鍒楀嚭涓?OSS 鍏煎鐨勫畾搴忓櫒鐩稿叧鍐呭銆?


## 璋冭瘯甯姪锛?


褰撻棶棰樹笌 PCM 鐩稿叧鏃讹紝棣栧厛灏濊瘯鎵撳紑 xrun_debug 妯″紡銆傝繖浼氬湪 xrun 鍙戠敓鐨勬椂闂村拰浣嶇疆缁欏嚭
鍐呮牳娑堟伅銆?

濡傛灉杩欑‘瀹炴槸涓€涓?bug锛岃闄勪笂浠ヤ笅淇℃伅鎶ュ憡锛?

- 椹卞姩/澹板崱鐨勫悕绉帮紝鏄剧ず鍦?`/proc/asound/cards`
- 瀵勫瓨鍣ㄨ浆鍌紙濡傛灉鍙敤锛屼緥濡?`card*/cmipci`锛?

褰撳畠鏄?PCM 闂鏃讹細

- PCM 鐨勮缃紝鏄剧ず鍦?PCM 瀛愭祦鐩綍涓殑 hw_parms銆乻w_params 涓?status

褰撳畠鏄贩闊冲櫒闂鏃讹細

- AC97 proc 鏂囦欢锛宍codec97#**/**` 鏂囦欢

瀵逛簬 USB 闊抽/MIDI锛?

- `lsusb -v` 鐨勮緭鍑?
- 澹板崱鐩綍涓殑 `stream*` 鏂囦欢


ALSA 鐨?bug 璺熻釜绯荤粺浣嶄簬锛?
https://bugtrack.alsa-project.org/alsa-bug/
