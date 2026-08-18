## ALSA PCM 閫氶亾鏄犲皠 API


Takashi Iwai <tiwai@suse.de>

## 姒傝堪


閫氶亾鏄犲皠 API 鍏佽鐢ㄦ埛鏌ヨ鍙兘鐨勯€氶亾鏄犲皠鍜屽綋鍓嶉€氶亾鏄犲皠锛岃繕鍙互閫夋嫨鎬у湴淇敼褰撳墠娴佺殑閫氶亾鏄犲皠銆?
閫氶亾鏄犲皠鏄瘡涓?PCM 閫氶亾浣嶇疆鐨勪竴涓暟缁勩€傞€氬父锛岀珛浣撳０ PCM 娴佺殑閫氶亾鏄犲皠涓?`{ front_left, front_right }`
鑰?4.0 鐜粫 PCM 娴佺殑閫氶亾鏄犲皠涓?`{ front left, front right, rear left, rear right }.`

鍒扮洰鍓嶄负姝紝闂鍦ㄤ簬鎴戜滑娌℃湁鏄惧紡鐨勬爣鍑嗛€氶亾鏄犲皠锛屽簲鐢ㄧ▼搴忔棤娉曠煡閬撳摢涓€氶亾瀵瑰簲鍝釜锛堟壃澹板櫒锛変綅缃€傚洜姝わ紝搴旂敤绋嬪簭瀵?5.1 杈撳嚭搴旂敤浜嗛敊璇殑閫氶亾锛屼綘浼氱獊鐒朵粠鍚庢柟鍚埌濂囨€殑澹伴煶銆傛垨鑰咃紝鏌愪簺璁惧绉佷笅鍋囪 center/LFE 鏄涓?绗洓閫氶亾锛岃€屽叾浠栬澶囧垯鍋囪 C/LFE 鏄浜?绗叚閫氶亾銆?
姝ゅ锛屾煇浜涜澶囷紙濡?HDMI锛夊嵆浣垮湪鐩稿悓鐨勬€婚€氶亾鏁颁笅涔熷彲浠ラ厤缃负涓嶅悓鐨勬壃澹板櫒浣嶇疆銆傜劧鑰岋紝鐢变簬缂轰箯閫氶亾鏄犲皠瑙勮寖锛屾鍓嶆病鏈夊姙娉曟寚瀹氳繖涓€鐐广€傝繖浜涢兘鏄柊閫氶亾鏄犲皠 API 鐨勪富瑕佸姩鏈恒€?
## 璁捐


瀹為檯涓婏紝浠庡唴鏍?鐢ㄦ埛绌洪棿 ABI 鐨勮搴︽潵鐪嬶紝鈥滈€氶亾鏄犲皠 API鈥濆苟娌℃湁寮曞叆浠讳綍鏂颁笢瑗裤€傚畠浠呬娇鐢ㄤ簡鐜版湁鐨勬帶鍒跺厓绱犵壒鎬с€?
鍦ㄥ熀鏈璁′笂锛屾瘡涓?PCM 瀛愭祦鍙互鍖呭惈涓€涓彁渚涢€氶亾鏄犲皠淇℃伅鍜岄厤缃殑鎺у埗鍏冪礌銆傝鍏冪礌鐢变互涓嬫寚瀹氾細

- iface = SNDRV_CTL_ELEM_IFACE_PCM
- name = "Playback Channel Map" 鎴?"Capture Channel Map"
- device = 鎵€鍒嗛厤 PCM 瀛愭祦鐨勭浉鍚岃澶囧彿
- index = 鎵€鍒嗛厤 PCM 瀛愭祦鐨勭浉鍚岀储寮曞彿

娉ㄦ剰鍚嶇О鍙栧喅浜?PCM 瀛愭祦鐨勬柟鍚戣€屼笉鍚屻€?
姣忎釜鎺у埗鍏冪礌鑷冲皯鎻愪緵 TLV 璇绘搷浣滃拰璇绘搷浣溿€傚彲閫夊湴锛屽彲浠ユ彁渚涘啓鎿嶄綔浠ュ厑璁哥敤鎴峰姩鎬佹洿鏀归€氶亾鏄犲皠銆?
### TLV


TLV 鎿嶄綔缁欏嚭鍙敤閫氶亾鏄犲皠鐨勫垪琛ㄣ€傞€氶亾鏄犲皠鐨勫垪琛ㄩ」閫氬父鏄?`type data-bytes ch0 ch1 ch2...`
鍏朵腑 type 鏄?TLV 绫诲瀷鍊硷紝绗簩涓弬鏁版槸閫氶亾鍊肩殑鎬诲瓧鑺傛暟锛堜笉鏄暟閲忥級锛屽叾浣欐槸姣忎釜閫氶亾鐨勪綅缃€笺€?
浣滀负 TLV 绫诲瀷锛屽彲浠ヤ娇鐢?`SNDRV_CTL_TLVT_CHMAP_FIXED`銆乣SNDRV_CTL_TLVT_CHMAP_VAR` 鎴?`SNDRV_CTL_TLVT_CHMAP_PAIRED`銆俙_FIXED` 绫诲瀷鐢ㄤ簬閫氶亾浣嶇疆鍥哄畾鐨勯€氶亾鏄犲皠锛岃€屽悗涓よ€呯敤浜庣伒娲荤殑閫氶亾浣嶇疆銆俙_VAR` 绫诲瀷鐢ㄤ簬鎵€鏈夐€氶亾鍙嚜鐢变氦鎹㈢殑閫氶亾鏄犲皠锛宍_PAIRED` 绫诲瀷鐢ㄤ簬鎴愬閫氶亾鍙氦鎹㈢殑閫氶亾鏄犲皠銆備緥濡傦紝褰撲綘鏈?{FL/FR/RL/RR} 閫氶亾鏄犲皠鏃讹紝`_PAIRED` 绫诲瀷鍙厑璁镐綘浜ゆ崲 {RL/RR/FL/FR}锛岃€?`_VAR` 绫诲瀷鐢氳嚦鍏佽浜ゆ崲 FL 鍜?RR銆?
杩欎簺鏂扮殑 TLV 绫诲瀷瀹氫箟鍦?`sound/tlv.h` 涓€?
鍙敤鐨勯€氶亾浣嶇疆鍊煎畾涔夊湪 `sound/asound.h` 涓紝浠ヤ笅鏄妭閫夛細

```

  /* channel positions */
  enum {
	SNDRV_CHMAP_UNKNOWN = 0,
	SNDRV_CHMAP_NA,		/* N/A, silent */
	SNDRV_CHMAP_MONO,	/* mono stream */
	/* this follows the alsa-lib mixer channel value + 3 */
	SNDRV_CHMAP_FL,		/* front left */
	SNDRV_CHMAP_FR,		/* front right */
	SNDRV_CHMAP_RL,		/* rear left */
	SNDRV_CHMAP_RR,		/* rear right */
	SNDRV_CHMAP_FC,		/* front center */
	SNDRV_CHMAP_LFE,	/* LFE */
	SNDRV_CHMAP_SL,		/* side left */
	SNDRV_CHMAP_SR,		/* side right */
	SNDRV_CHMAP_RC,		/* rear center */
	/* new definitions */
	SNDRV_CHMAP_FLC,	/* front left center */
	SNDRV_CHMAP_FRC,	/* front right center */
	SNDRV_CHMAP_RLC,	/* rear left center */
	SNDRV_CHMAP_RRC,	/* rear right center */
	SNDRV_CHMAP_FLW,	/* front left wide */
	SNDRV_CHMAP_FRW,	/* front right wide */
	SNDRV_CHMAP_FLH,	/* front left high */
	SNDRV_CHMAP_FCH,	/* front center high */
	SNDRV_CHMAP_FRH,	/* front right high */
	SNDRV_CHMAP_TC,		/* top center */
	SNDRV_CHMAP_TFL,	/* top front left */
	SNDRV_CHMAP_TFR,	/* top front right */
	SNDRV_CHMAP_TFC,	/* top front center */
	SNDRV_CHMAP_TRL,	/* top rear left */
	SNDRV_CHMAP_TRR,	/* top rear right */
	SNDRV_CHMAP_TRC,	/* top rear center */
	SNDRV_CHMAP_LAST = SNDRV_CHMAP_TRC,
  };

```
褰撲竴涓?PCM 娴佸彲浠ユ彁渚涘涓€氶亾鏄犲皠鏃讹紝浣犲彲浠ュ湪涓€涓?TLV 瀹瑰櫒绫诲瀷涓彁渚涘涓€氶亾鏄犲皠銆傝杩斿洖鐨?TLV 鏁版嵁灏嗗寘鍚涓嬪唴瀹癸細
```

	SNDRV_CTL_TLVT_CONTAINER 96
	    SNDRV_CTL_TLVT_CHMAP_FIXED 4 SNDRV_CHMAP_FC
	    SNDRV_CTL_TLVT_CHMAP_FIXED 8 SNDRV_CHMAP_FL SNDRV_CHMAP_FR
	    SNDRV_CTL_TLVT_CHMAP_FIXED 16 NDRV_CHMAP_FL SNDRV_CHMAP_FR \
		SNDRV_CHMAP_RL SNDRV_CHMAP_RR

```
閫氶亾浣嶇疆鍦ㄦ渶浣?16 浣嶏紙LSB锛変腑鎻愪緵銆傞珮浣嶇敤浜庝綅鏍囧織銆?```

	#define SNDRV_CHMAP_POSITION_MASK	0xffff
	#define SNDRV_CHMAP_PHASE_INVERSE	(0x01 << 16)
	#define SNDRV_CHMAP_DRIVER_SPEC		(0x02 << 16)

```
`SNDRV_CHMAP_PHASE_INVERSE` 琛ㄧず璇ラ€氶亾鐩镐綅鍙嶈浆锛堝洜姝ゅ皢宸﹀彸閫氶亾鐩稿姞浼氬鑷村嚑涔庨潤闊筹級銆傛煇浜涙暟瀛楅害鍏嬮璁惧鍏锋湁姝ょ壒鎬с€?
褰撹缃簡 `SNDRV_CHMAP_DRIVER_SPEC` 鏃讹紝鎵€鏈夐€氶亾浣嶇疆鍊间笉閬靛惊涓婅堪鏍囧噯瀹氫箟锛岃€屾槸椹卞姩鐗瑰畾鐨勩€?
### 璇绘搷浣?

鎺у埗璇绘搷浣滅敤浜庢彁渚涚粰瀹氭祦鐨勫綋鍓嶉€氶亾鏄犲皠銆傛帶鍒跺厓绱犺繑鍥炰竴涓寘鍚瘡涓€氶亾浣嶇疆鐨勬暣鏁版暟缁勩€?
濡傛灉鍦ㄦ寚瀹氶€氶亾鏁颁箣鍓嶏紙鍗宠缃?hw_params 涔嬪墠锛夋墽琛屾鎿嶄綔锛屽畠搴旇繑鍥炴墍鏈夐€氶亾閮借缃负 `UNKNOWN`銆?
### 鍐欐搷浣?

鎺у埗鍐欐搷浣滄槸鍙€夌殑锛屼粎閫傜敤浜庡彲浠ュ姩鎬佹洿鏀归€氶亾閰嶇疆鐨勮澶囷紝渚嬪 HDMI銆傜敤鎴烽渶瑕佷紶閫掍竴涓暣鏁板€硷紝鍏朵腑鍖呭惈鎵€鍒嗛厤 PCM 瀛愭祦鎵€鏈夐€氶亾鐨勬湁鏁堥€氶亾浣嶇疆銆?
姝ゆ搷浣滀粎鍦?PCM PREPARED 鐘舵€佷笅鍏佽銆傚湪鍏朵粬鐘舵€佷笅璋冪敤鏃讹紝搴旇繑鍥為敊璇€?