## 鍏充簬鍐呮牳 OSS 妯℃嫙鐨勮鏄?

Jan. 22, 2004  Takashi Iwai <tiwai@suse.de>

## 妯″潡

ALSA 鍦ㄥ唴鏍镐腑鎻愪緵浜嗕竴濂楀己澶х殑 OSS 妯℃嫙銆傞拡瀵?PCM銆佹贩闊冲櫒锛坢ixer锛夊拰闊冲簭鍣紙sequencer锛夎澶囩殑 OSS 妯℃嫙锛屼綔涓洪檮鍔犲唴鏍告ā鍧?snd-pcm-oss銆乻nd-mixer-oss 鍜?snd-seq-oss 瀹炵幇銆傚綋鎮ㄩ渶瑕佽闂?OSS 鐨?PCM銆乵ixer 鎴?sequencer 璁惧鏃讹紝蹇呴』鍔犺浇鐩稿簲鐨勬ā鍧椼€?

杩欎簺妯″潡鍦ㄨ皟鐢ㄧ浉搴旀湇鍔℃椂浼氳嚜鍔ㄥ姞杞姐€傚叾鍒悕瀹氫箟涓?`sound-service-x-y`锛屽叾涓?x 鍜?y 鍒嗗埆鏄０鍗＄紪鍙峰拰娆¤澶囧彿锛坢inor unit number锛夈€傞€氬父鎮ㄤ笉闇€瑕佽嚜宸卞畾涔夎繖浜涘埆鍚嶃€?

瀹炵幇 OSS 妯″潡鑷姩鍔犺浇鎵€闇€鐨勫敮涓€姝ラ鏄畾涔?
```

	alias sound-slot-0 snd-emu10k1

```
浣滀负绗簩鍧楀０鍗★紝鍚屾牱瀹氫箟 `sound-slot-1`銆傝娉ㄦ剰锛屾偍涓嶈兘灏嗗埆鍚嶅悕绉扮敤浣滅洰鏍囧悕绉帮紙鍗?`alias sound-slot-0 snd-card-0` 涓嶅啀鍍忔棫鐗?modutils 閭ｆ牱璧蜂綔鐢級銆?

褰撳墠鍙敤鐨?OSS 閰嶇疆鏄剧ず鍦?/proc/asound/oss/sndstat銆傚畠閲囩敤涓?/dev/sndstat 鐩稿悓鐨勮娉曪紝鑰屽悗鑰呭湪鍟嗕笟 OSS 椹卞姩涓婂彲鐢ㄣ€傚湪 ALSA 涓婏紝鎮ㄥ彲浠ュ皢 /dev/sndstat 绗﹀彿閾炬帴鍒拌繖涓?proc 鏂囦欢銆?

璇锋敞鎰忥紝proc 鏂囦欢涓垪鍑虹殑璁惧鍙湁鍦ㄧ浉搴旂殑 OSS 妯℃嫙妯″潡鍔犺浇涔嬪悗鎵嶄細鍑虹幇銆傚嵆渚垮叾涓樉绀?"NOT ENABLED IN CONFIG" 涔熶笉蹇呮媴蹇冦€?

## 璁惧鏄犲皠

ALSA 鏀寔浠ヤ笅 OSS 璁惧鏂囦欢锛?
```

	PCM:
		/dev/dspX
		/dev/adspX

	Mixer:
		/dev/mixerX

	MIDI:
		/dev/midi0X
		/dev/amidi0X

	Sequencer:
		/dev/sequencer
		/dev/sequencer2 (aka /dev/music)

```
鍏朵腑 X 鏄?0 鍒?7 鐨勫０鍗＄紪鍙枫€?

锛堟敞鎰忥細鏌愪簺鍙戣鐗堟嫢鏈夎濡?/dev/midi0 鍜?/dev/midi1 杩欐牱鐨勮澶囨枃浠躲€傚畠浠苟闈炵敤浜?OSS锛岃€屾槸鐢ㄤ簬 tclmidi锛岄偅鏄畬鍏ㄤ笉鍚岀殑涓滆タ銆傦級

涓庣湡姝ｇ殑 OSS 涓嶅悓锛孉LSA 涓嶈兘浣跨敤瓒呭嚭鎵€鍒嗛厤鑼冨洿鐨勮澶囨枃浠躲€備緥濡傦紝绗竴鍧楀０鍗′笉鑳戒娇鐢?/dev/dsp1 鎴?/dev/dsp2锛岃€屽彧鑳戒娇鐢?/dev/dsp0 鍜?/dev/adsp0銆?

濡備笂鎵€绀猴紝PCM 鍜?MIDI 鍙兘鎷ユ湁涓や釜璁惧銆傞€氬父锛岀涓€涓?PCM 璁惧锛圓LSA 涓殑 `hw:0,0`锛夋槧灏勫埌 /dev/dsp锛岃€岀浜屼釜璁惧锛坄hw:0,1`锛夋槧灏勫埌 /dev/adsp锛堝鏋滃彲鐢級銆傚浜?MIDI锛屽垯鍒嗗埆鏄?/dev/midi 鍜?/dev/amidi銆?

鎮ㄥ彲浠ラ€氳繃 snd-pcm-oss 鍜?snd-rawmidi 鐨勬ā鍧楅€夐」鏉ユ敼鍙樿繖涓€璁惧鏄犲皠銆傚氨 PCM 鑰岃█锛宻nd-pcm-oss 鎻愪緵浠ヤ笅閫夐」锛?

dsp_map
	鍒嗛厤缁?/dev/dspX 鐨?PCM 璁惧缂栧彿
	锛堥粯璁ゅ€?= 0锛?
adsp_map
	鍒嗛厤缁?/dev/adspX 鐨?PCM 璁惧缂栧彿
	锛堥粯璁ゅ€?= 1锛?

渚嬪锛岃灏嗙涓変釜 PCM 璁惧锛坄hw:0,2`锛夋槧灏勫埌 /dev/adsp0锛屾寜濡備笅鏂瑰紡瀹氫箟锛?
```

	options snd-pcm-oss adsp_map=2

```
杩欎簺閫夐」鎺ュ彈鏁扮粍銆傝閰嶇疆绗簩鍧楀０鍗★紝璇风敤閫楀彿鍒嗛殧鎸囧畾涓や釜鏉＄洰銆備緥濡傦紝瑕佸皢绗簩鍧楀０鍗′笂鐨勭涓変釜 PCM 璁惧鏄犲皠鍒?/dev/adsp1锛屾寜濡備笅鏂瑰紡瀹氫箟锛?
```

	options snd-pcm-oss adsp_map=0,2

```
瑕佹敼鍙?MIDI 璁惧鐨勬槧灏勶紝snd-rawmidi 鎻愪緵浠ヤ笅閫夐」锛?

midi_map
	鍒嗛厤缁?/dev/midi0X 鐨?MIDI 璁惧缂栧彿
	锛堥粯璁ゅ€?= 0锛?
amidi_map
	鍒嗛厤缁?/dev/amidi0X 鐨?MIDI 璁惧缂栧彿
	锛堥粯璁ゅ€?= 1锛?

渚嬪锛岃灏嗙涓€鍧楀０鍗′笂鐨勭涓変釜 MIDI 璁惧鍒嗛厤缁?/dev/midi00锛屾寜濡備笅鏂瑰紡瀹氫箟锛?
```

	options snd-rawmidi midi_map=2



```
## PCM 妯″紡

榛樿鎯呭喌涓嬶紝ALSA 閫氳繃鎵€璋撶殑鎻掍欢灞傦紙plugin layer锛夋潵妯℃嫙 OSS PCM锛屼篃灏辨槸璇达紝褰撳０鍗℃湰韬笉鏀寔鏃讹紝瀹冧細灏濊瘯鑷姩杞崲閲囨牱鏍煎紡銆侀噰鏍风巼鎴栭€氶亾鏁般€傝繖浼氫负鏌愪簺搴旂敤绋嬪簭锛堝 quake 鎴?wine锛夊甫鏉ヤ竴浜涢棶棰橈紝灏ゅ叾鏄綋瀹冧滑浠呭湪 MMAP 妯″紡涓嬩娇鐢ㄥ０鍗℃椂銆?

鍦ㄨ繖绉嶆儏鍐典笅锛屾偍鍙互閫氳繃鍚?proc 鏂囦欢鍐欏叆鍛戒护鏉ユ寜搴旂敤绋嬪簭鏀瑰彉 PCM 鐨勮涓恒€傛瘡涓?PCM 娴侀兘鏈変竴涓?proc 鏂囦欢锛宍/proc/asound/cardX/pcmY[cp]/oss`锛屽叾涓?X 鏄０鍗＄紪鍙凤紙浠?0 寮€濮嬶級锛孻 鏄?PCM 璁惧缂栧彿锛堜粠 0 寮€濮嬶級锛宍p` 浠ｈ〃鍥炴斁锛坧layback锛夛紝`c` 浠ｈ〃閲囬泦锛坈apture锛夈€傝娉ㄦ剰锛岃 proc 鏂囦欢鍙湁鍦?snd-pcm-oss 妯″潡鍔犺浇鍚庢墠瀛樺湪銆?

鍛戒护搴忓垪鍏锋湁浠ヤ笅璇硶锛?
```

	app_name fragments fragment_size [options]

```
`app_name` 鏄甫璺緞锛堜紭鍏堢骇鏇撮珮锛夋垨涓嶅甫璺緞鐨勫簲鐢ㄧ▼搴忓悕绉般€?
`fragments` 鎸囧畾鐗囨锛坒ragment锛夌殑鏁伴噺锛岃嫢鏈粰瀹氬叿浣撴暟閲忓垯涓?0銆?
`fragment_size` 鏄墖娈电殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級锛岃嫢鏈粰瀹氬垯涓?0銆?
`options` 鏄彲閫夊弬鏁般€傚彲鐢ㄧ殑閫夐」濡備笅锛?

disable
	搴旂敤绋嬪簭灏濊瘯涓鸿閫氶亾鎵撳紑涓€涓?pcm 璁惧浣嗕笉鎯充娇鐢ㄥ畠銆?
direct
	涓嶄娇鐢ㄦ彃浠?
block
	寮哄埗闃诲鎵撳紑妯″紡
non-block
	寮哄埗闈為樆濉炴墦寮€妯″紡
partial-frag
	涔熷啓鍏ラ儴鍒嗙墖娈碉紙浠呭奖鍝嶅洖鏀撅級
no-silence
	涓嶈棰勫厛濉厖闈欓煶鏁版嵁浠ラ伩鍏嶇垎闊?

`disable` 閫夐」鍦ㄥ簲鐢ㄧ▼搴忔湭鑳芥纭鐞嗘煇涓€娴佹柟鍚戯紙鍥炴斁鎴栭噰闆嗭級銆佽€岀‖浠舵湰韬悓鏃舵敮鎸佷袱涓柟鍚戞椂寰堟湁鐢ㄣ€傚涓婃墍杩帮紝`direct` 閫夐」鐢ㄤ簬缁曡繃鑷姩杞崲锛屽 MMAP 搴旂敤绋嬪簭寰堟湁鐢ㄣ€備緥濡傦紝瑕侀拡瀵?quake 鍦ㄤ笉浣跨敤鎻掍欢鐨勬儏鍐典笅鍥炴斁绗竴涓?PCM 璁惧锛岄€氳繃 echo 鍙戦€佸涓嬪懡浠わ細
```

	% echo "quake 0 0 direct" > /proc/asound/card0/pcm0p/oss

```
鐢变簬 quake 鍙渶瑕佸洖鏀撅紝鎮ㄥ彲浠ヨ拷鍔犵浜屾潯鍛戒护锛岄€氱煡椹卞姩绋嬪簭鍗冲皢鍒嗛厤鐨勬柟鍚戜粎姝や竴涓細
```

	% echo "quake 0 0 disable" > /proc/asound/card0/pcm0c/oss

```
proc 鏂囦欢鐨勬潈闄愬彇鍐充簬 snd 鐨勬ā鍧楅€夐」銆傞粯璁ゆ儏鍐典笅瀹冭璁剧疆涓?root锛屽洜姝ゅ彂閫佷笂杩板懡浠ゆ椂鎮ㄥ緢鍙兘蹇呴』鏄秴绾х敤鎴枫€?

block 鍜?non-block 閫夐」鐢ㄤ簬鏀瑰彉鎵撳紑璁惧鏂囦欢鐨勮涓恒€?

榛樿鎯呭喌涓嬶紝ALSA 鐨勮涓轰笌鍘熷 OSS 椹卞姩涓€鑷达紝鍗冲湪鏂囦欢蹇欐椂涓嶉樆濉炪€傝繖绉嶆儏鍐典笅浼氳繑鍥?-EBUSY 閿欒銆?

杩欎竴闃诲琛屼负鍙互閫氳繃 snd-pcm-oss 鐨?nonblock_open 妯″潡閫夐」杩涜鍏ㄥ眬鏀瑰彉銆傝嫢瑕佸皢闃诲妯″紡浣滀负 OSS 璁惧鐨勯粯璁ゆā寮忥紝鎸夊涓嬫柟寮忓畾涔夛細
```

	options snd-pcm-oss nonblock_open=0

```
`partial-frag` 鍜?`no-silence` 杩欎袱涓懡浠ゆ槸鏈€杩戞墠鍔犲叆鐨勩€傝繖涓や釜鍛戒护浠呯敤浜庝紭鍖栥€傚墠鑰呭懡浠ゆ寚瀹氫粎鍦ㄦ暣娈电墖娈佃濉弧鏃舵墠鍙戣捣鍐欏叆浼犺緭銆傚悗鑰呬細鍋滄鑷姩棰勫厛鍐欏叆闈欓煶鏁版嵁銆備袱鑰呴粯璁ゅ潎绂佺敤銆?

鎮ㄥ彲浠ラ€氳繃璇诲彇 proc 鏂囦欢鏉ユ鏌ュ綋鍓嶅畾涔夌殑閰嶇疆銆傝鍙栧埌鐨勬槧鍍忓彲浠ュ啀娆″彂閫佺粰 proc 鏂囦欢锛屽洜姝ゆ偍鍙互淇濆瓨褰撳墠閰嶇疆
```

	% cat /proc/asound/card0/pcm0p/oss > /somewhere/oss-cfg

```
骞舵寜濡備笅鏂瑰紡鎭㈠瀹?
```

	% cat /somewhere/oss-cfg > /proc/asound/card0/pcm0p/oss

```
姝ゅ锛岃娓呴櫎鎵€鏈夊綋鍓嶉厤缃紝鍙戦€?`erase` 鍛戒护锛屽涓嬶細
```

	% echo "erase" > /proc/asound/card0/pcm0p/oss


```
## 娣烽煶鍣ㄥ厓绱?

鐢变簬 ALSA 鍏锋湁瀹屽叏涓嶅悓鐨勬贩闊冲櫒鎺ュ彛锛屽 OSS 娣烽煶鍣ㄧ殑妯℃嫙鐩稿澶嶆潅銆侫LSA 鍩轰簬鍚嶇О瀛楃涓诧紝鐢辫嫢骞蹭笉鍚岀殑 ALSA锛坢ixer锛夋帶浠舵瀯寤哄嚭涓€涓贩闊冲櫒鍏冪礌銆備緥濡傦紝闊抽噺鍏冪礌 SOUND_MIXER_PCM 鐢卞洖鏀炬柟鍚戠殑 "PCM Playback Volume" 鍜?"PCM Playback Switch" 鎺т欢锛屼互鍙婇噰闆嗘柟鍚戯紙濡傛灉瀛樺湪锛夌殑 "PCM Capture Volume" 鍜?"PCM Capture Switch" 鎺т欢缁勬垚銆傚綋 OSS 鐨?PCM 闊抽噺鏀瑰彉鏃讹紝涓婅堪鎵€鏈夐煶閲忓拰寮€鍏虫帶浠堕兘浼氳嚜鍔ㄨ璋冩暣銆?

榛樿鎯呭喌涓嬶紝ALSA 瀵?OSS 闊抽噺鐨勪娇鐢ㄥ涓嬫帶浠讹細

====================	=====================	=====
OSS volume		ALSA control		Index
====================	=====================	=====
SOUND_MIXER_VOLUME 	Master			0
SOUND_MIXER_BASS	Tone Control - Bass	0
SOUND_MIXER_TREBLE	Tone Control - Treble	0
SOUND_MIXER_SYNTH	Synth			0
SOUND_MIXER_PCM		PCM			0
SOUND_MIXER_SPEAKER	PC Speaker 		0
SOUND_MIXER_LINE		Line			0
SOUND_MIXER_MIC		Mic 			0
SOUND_MIXER_CD		CD 			0
SOUND_MIXER_IMIX		Monitor Mix 		0
SOUND_MIXER_ALTPCM	PCM			1
SOUND_MIXER_RECLEV	锛堟湭鍒嗛厤锛?
SOUND_MIXER_IGAIN	Capture			0
SOUND_MIXER_OGAIN	Playback		0
SOUND_MIXER_LINE1	Aux			0
SOUND_MIXER_LINE2	Aux			1
SOUND_MIXER_LINE3	Aux			2
SOUND_MIXER_DIGITAL1	Digital			0
SOUND_MIXER_DIGITAL2	Digital			1
SOUND_MIXER_DIGITAL3	Digital			2
SOUND_MIXER_PHONEIN	Phone			0
SOUND_MIXER_PHONEOUT	Phone			1
SOUND_MIXER_VIDEO	Video			0
SOUND_MIXER_RADIO	Radio			0
SOUND_MIXER_MONITOR	Monitor			0
====================	=====================	=====

绗簩鍒楁槸鐩稿簲 ALSA 鎺т欢鐨勫熀瀛楃涓诧紙base-string锛夈€傚疄闄呬笂锛岃繕浼氶澶栨鏌ュ甫鏈?``XXX [Playback|Capture] [Volume|Switch]`` 鐨勬帶浠躲€?

杩欎簺娣烽煶鍣ㄥ厓绱犵殑褰撳墠鍒嗛厤鍒楀湪 proc 鏂囦欢 /proc/asound/cardX/oss_mixer 涓紝鍏跺舰寮忓涓?
```

	VOLUME "Master" 0
	BASS "" 0
	TREBLE "" 0
	SYNTH "" 0
	PCM "PCM" 0
	...

```
鍏朵腑绗竴鍒楁槸 OSS 闊抽噺鍏冪礌锛岀浜屽垪鏄浉搴?ALSA 鎺т欢鐨勫熀瀛楃涓诧紝绗笁鍒楁槸鎺т欢绱㈠紩锛坈ontrol index锛夈€傚綋瀛楃涓蹭负绌烘椂锛岃〃绀虹浉搴旂殑 OSS 鎺т欢涓嶅彲鐢ㄣ€?

瑕佹敼鍙樺垎閰嶏紝鎮ㄥ彲浠ュ悜杩欎釜 proc 鏂囦欢鍐欏叆閰嶇疆銆備緥濡傦紝瑕佸皢 "Wave Playback" 鏄犲皠鍒?PCM 闊抽噺锛屽彂閫佸涓嬪懡浠わ細
```

	% echo 'VOLUME "Wave Playback" 0' > /proc/asound/card0/oss_mixer

```
璇ュ懡浠や笌 proc 鏂囦欢涓垪鍑虹殑瀹屽叏涓€鑷淬€傛偍鍙互涓€娆℃敼鍙樹竴涓垨澶氫釜鍏冪礌锛屾瘡琛屼竴涓煶閲忋€傚湪鏈€鍚庝竴涓ず渚嬩腑锛屽綋 PCM 闊抽噺鏀瑰彉鏃讹紝"Wave Playback Volume" 鍜?"Wave Playback Switch" 閮戒細鍙楀埌褰卞搷銆?

涓?PCM proc 鏂囦欢鐨勬儏鍐典竴鏍凤紝proc 鏂囦欢鐨勬潈闄愬彇鍐充簬 snd 鐨勬ā鍧楅€夐」銆傚彂閫佷笂杩板懡浠ゆ椂鎮ㄥ緢鍙兘蹇呴』鏄秴绾х敤鎴枫€?

涓?PCM proc 鏂囦欢鐨勬儏鍐电浉鍚岋紝鎮ㄥ彲浠ラ€氳繃璇诲彇骞跺啓鍏ユ暣涓枃浠舵槧鍍忔潵淇濆瓨鍜屾仮澶嶅綋鍓嶇殑娣烽煶鍣ㄩ厤缃€?

## 鍙屽伐娴?

璇锋敞鎰忥紝褰撳皾璇曚娇鐢ㄥ崟涓€璁惧鏂囦欢杩涜鍥炴斁鍜岄噰闆嗘椂锛孫SS API 鏃犳硶鎻愪緵鏂规硶鏉ヤ负涓や釜鏂瑰悜鍒嗗埆璁剧疆涓嶅悓鐨勬牸寮忋€侀噰鏍风巼鎴栭€氶亾鏁般€傚洜姝?
```

	io_handle = open("device", O_RDWR)

```
鍙湁鍦ㄤ袱涓柟鍚戠殑鍊肩浉鍚屾椂鎵嶈兘姝ｇ‘宸ヤ綔銆?

鑻ヨ鍦ㄤ袱涓柟鍚戜娇鐢ㄤ笉鍚岀殑鍊硷紝璇峰悓鏃朵娇鐢?
```

	input_handle = open("device", O_RDONLY)
	output_handle = open("device", O_WRONLY)

```
骞朵负鐩稿簲鐨勫彞鏌勮缃€笺€?

## 涓嶆敮鎸佺殑鐗规€?

### MMAP 鍦?ICE1712 椹卞姩涓?

ICE1712 浠呮敮鎸侀潪鎯緥鐨勬牸寮忥紝鍗充氦閿欙紙interleaved锛夌殑 10 閫氶亾 24 浣嶏紙鎵撳寘杩?32 浣嶏級鏍煎紡銆傚洜姝ゆ偍鏃犳硶鍦?OSS 涓婁互鎯緥锛堝崟澹伴亾鎴?2 閫氶亾锛? 鎴?16 浣嶏級鏍煎紡鏉?mmap 璇ョ紦鍐插尯銆?
