


######## FM Transmitter Control Reference


FM 鍙戝皠鍣紙FM_TX锛夌被鍖呭惈鍏峰 FM 鍙戝皠鑳藉姏璁惧鐨勫父瑙佸姛鑳芥帶浠躲€傜洰鍓嶈绫诲寘鍚煶棰?鍘嬬缉銆佸棰戦煶鐢熸垚銆侀煶棰戦鍋忛檺鍒跺櫒銆丷DS 鍙戝皠涓庤皟璋愬姛鐜囩瓑鍔熻兘鐨勫弬鏁般€?

## FM_TX 鎺у埗 ID


`V4L2_CID_FM_TX_CLASS (class)`
    FM_TX 绫绘弿杩扮銆傚璇ユ帶浠惰皟鐢?VIDIOC_QUERYCTRL 灏嗚繑鍥炴鎺т欢绫荤殑鎻忚堪銆?
`V4L2_CID_RDS_TX_DEVIATION (integer)`
    浠?Hz 涓哄崟浣嶉厤缃?RDS 淇″彿棰戝亸鐢靛钩銆傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_RDS_TX_PI (integer)`
    璁剧疆鐢ㄤ簬鍙戝皠鐨?RDS 鑺傜洰璇嗗埆锛圥rogramme Identification锛夊瓧娈点€?
`V4L2_CID_RDS_TX_PTY (integer)`
    璁剧疆鐢ㄤ簬鍙戝皠鐨?RDS 鑺傜洰绫诲瀷锛圥rogramme Type锛夊瓧娈点€傚叾缂栫爜鏈€澶?31 绉?    棰勫畾涔夎妭鐩被鍨嬨€?
`V4L2_CID_RDS_TX_PS_NAME (string)`
    璁剧疆鐢ㄤ簬鍙戝皠鐨勮妭鐩湇鍔″悕绉帮紙PS_NAME锛夈€傚畠鐢ㄤ簬鎺ユ敹鏈轰笂鐨勯潤鎬佹樉绀猴紝鏄?    鍚紬璇嗗埆鍜岄€夋嫨鑺傜洰鏈嶅姟鐨勪富瑕佽緟鍔┿€傚湪 RDS 瑙勮寖 iec62106 鐨勯檮褰?E 涓紝瀵?    鑺傜洰鏈嶅姟鍚嶇О瀛楃涓茬殑姝ｇ‘瀛楃缂栫爜鏈夊畬鏁磋鏄庛€傚悓鏍锋牴鎹?RDS 瑙勮寖锛孭S 閫氬父
    鏄竴娈?8 涓瓧绗︾殑鏂囨湰銆備絾涔熷彲鑳芥壘鍒拌兘澶熸粴鍔ㄦ樉绀?8脳N 涓瓧绗﹀瓧绗︿覆鐨勬帴鏀舵満銆?    鍥犳锛屾鎺т欢蹇呴』浠?8 涓瓧绗︿负姝ヨ繘閰嶇疆锛岀粨鏋滃畠蹇呴』濮嬬粓鍖呭惈涓€涓ぇ灏忎负 8 鐨?    鏁存暟鍊嶇殑瀛楃涓层€?
`V4L2_CID_RDS_TX_RADIO_TEXT (string)`
    璁剧疆鐢ㄤ簬鍙戝皠鐨?Radio Text 淇℃伅銆傚畠鏄姝ｅ湪骞挎挱鍐呭鐨勬枃瀛楁弿杩般€傚綋骞挎挱鑰?    甯屾湜浼犺緭鏇撮暱鐨?PS 鍚嶇О銆佷笌鑺傜洰鐩稿叧鐨勪俊鎭垨浠讳綍鍏朵粬鏂囨湰鏃讹紝鍙互浣跨敤 RDS
    Radio Text銆傚湪杩欎簺鎯呭喌涓嬶紝RadioText 搴斾笌 `V4L2_CID_RDS_TX_PS_NAME` 閰嶅悎
    浣跨敤銆俁adio Text 瀛楃涓茬殑缂栫爜鍚屾牱鍦?iec62106 鐨勯檮褰?E 涓湁瀹屾暣璇存槑銆俁adio
    Text 瀛楃涓茬殑闀垮害鍙栧喅浜庣敤浜庝紶杈撳畠鐨?RDS 鍧楋紝鍗?32锛?A 鍧楋級鎴?64锛?B 鍧楋級銆?    浣嗕篃鍙兘鎵惧埌鑳藉婊氬姩鏄剧ず 32脳N 鎴?64脳N 涓瓧绗﹀瓧绗︿覆鐨勬帴鏀舵満銆傚洜姝わ紝姝ゆ帶浠?    蹇呴』浠?32 鎴?64 涓瓧绗︿负姝ヨ繘閰嶇疆锛岀粨鏋滃畠蹇呴』濮嬬粓鍖呭惈涓€涓ぇ灏忎负 32 鎴?64 鐨?    鏁存暟鍊嶇殑瀛楃涓层€?
`V4L2_CID_RDS_TX_MONO_STEREO (boolean)`
    璁剧疆瑙ｇ爜鍣ㄨ瘑鍒爜锛圖ecoder Identification code锛夌殑 Mono/Stereo 浣嶃€傝嫢璁剧疆锛?    鍒欒〃绀洪煶棰戜互绔嬩綋澹板綍鍒躲€?
`V4L2_CID_RDS_TX_ARTIFICIAL_HEAD (boolean)`
    璁剧疆瑙ｇ爜鍣ㄨ瘑鍒爜鐨?`Artificial Head <http://en.wikipedia.org/wiki/Artificial_head>`__
    浣嶃€傝嫢璁剧疆锛屽垯琛ㄧず闊抽浣跨敤浜哄伐澶达紙artificial head锛夊綍鍒躲€?
`V4L2_CID_RDS_TX_COMPRESSED (boolean)`
    璁剧疆瑙ｇ爜鍣ㄨ瘑鍒爜鐨?Compressed 浣嶃€傝嫢璁剧疆锛屽垯琛ㄧず闊抽缁忚繃鍘嬬缉銆?
`V4L2_CID_RDS_TX_DYNAMIC_PTY (boolean)`
    璁剧疆瑙ｇ爜鍣ㄨ瘑鍒爜鐨?Dynamic PTY 浣嶃€傝嫢璁剧疆锛屽垯琛ㄧず PTY 鐮佽鍔ㄦ€佸垏鎹€?
`V4L2_CID_RDS_TX_TRAFFIC_ANNOUNCEMENT (boolean)`
    鑻ヨ缃紝鍒欒〃绀烘鍦ㄨ繘琛屼氦閫氬叕鍛娿€?
`V4L2_CID_RDS_TX_TRAFFIC_PROGRAM (boolean)`
    鑻ヨ缃紝鍒欒〃绀哄綋鍓嶈皟璋愮殑鑺傜洰鎼哄甫浜ら€氬叕鍛娿€?
`V4L2_CID_RDS_TX_MUSIC_SPEECH (boolean)`
    鑻ヨ缃紝鍒欒〃绀鸿棰戦亾骞挎挱闊充箰锛涜嫢娓呴櫎锛屽垯琛ㄧず骞挎挱璇煶銆傚鏋滃彂灏勫櫒涓嶅仛姝?    鍖哄垎锛屽垯搴旇璁剧疆瀹冦€?
`V4L2_CID_RDS_TX_ALT_FREQS_ENABLE (boolean)`
    鑻ヨ缃紝鍒欒〃绀哄彂灏勫鐢ㄩ鐜囥€?
`V4L2_CID_RDS_TX_ALT_FREQS (__u32 array)`
    浠?kHz 涓哄崟浣嶇殑澶囩敤棰戠巼銆俁DS 鏍囧噯鍏佽瀹氫箟鏈€澶?25 涓鐜囥€傞┍鍔ㄥ彲鑳芥敮鎸?    鏇村皯棰戠巼锛屽洜姝よ妫€鏌ユ暟缁勫ぇ灏忋€?
`V4L2_CID_AUDIO_LIMITER_ENABLED (boolean)`
    鍚敤鎴栫鐢ㄩ煶棰戦鍋忛檺鍒跺櫒鍔熻兘銆傚綋璇曞浘鏈€澶у寲闊抽闊抽噺銆佹渶灏忓寲鎺ユ敹鏈轰骇鐢熺殑
    澶辩湡骞堕槻姝㈣繃璋冨埗鏃讹紝闄愬埗鍣ㄥ緢鏈夌敤銆?
`V4L2_CID_AUDIO_LIMITER_RELEASE_TIME (integer)`
    璁剧疆闊抽棰戝亸闄愬埗鍣ㄥ姛鑳界殑閲婃斁鏃堕棿銆傚崟浣嶄负寰銆傛杩涗笌鑼冨洿鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_AUDIO_LIMITER_DEVIATION (integer)`
    浠?Hz 涓哄崟浣嶉厤缃煶棰戦鍋忕數骞炽€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_AUDIO_COMPRESSION_ENABLED (boolean)`
    鍚敤鎴栫鐢ㄩ煶棰戝帇缂╁姛鑳姐€傝鍔熻兘浠ュ浐瀹氬鐩婃斁澶т綆浜庨槇鍊肩殑淇″彿锛屽苟鎸?    Threshold/(Gain + Threshold) 鐨勬瘮鐜囧帇缂╅珮浜庨槇鍊肩殑闊抽淇″彿銆?
`V4L2_CID_AUDIO_COMPRESSION_GAIN (integer)`
    璁剧疆闊抽鍘嬬缉鍔熻兘鐨勫鐩娿€備负 dB 鍊笺€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_AUDIO_COMPRESSION_THRESHOLD (integer)`
    璁剧疆闊抽鍘嬬缉鍔熻兘鐨勯槇鍊肩數骞炽€備负 dB 鍊笺€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_AUDIO_COMPRESSION_ATTACK_TIME (integer)`
    璁剧疆闊抽鍘嬬缉鍔熻兘鐨勫惎鍔ㄦ椂闂淬€備负寰鍊笺€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_AUDIO_COMPRESSION_RELEASE_TIME (integer)`
    璁剧疆闊抽鍘嬬缉鍔熻兘鐨勯噴鏀炬椂闂淬€備负寰鍊笺€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_PILOT_TONE_ENABLED (boolean)`
    鍚敤鎴栫鐢ㄥ棰戦煶鐢熸垚鍔熻兘銆?
`V4L2_CID_PILOT_TONE_DEVIATION (integer)`
    閰嶇疆瀵奸闊抽鍋忕數骞炽€傚崟浣嶄负 Hz銆傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_PILOT_TONE_FREQUENCY (integer)`
    閰嶇疆瀵奸闊抽鐜囧€笺€傚崟浣嶄负 Hz銆傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_TUNE_PREEMPHASIS (enum)`
    閰嶇疆鐢ㄤ簬骞挎挱鐨勯鍔犻噸鍊笺€傚骞挎挱搴旂敤棰勫姞閲嶆护娉㈠櫒浠ョ獊鍑洪珮棰戦煶棰戙€傛牴鎹湴鍖?    涓嶅悓锛屼娇鐢?50 鎴?75 寰鐨勬椂闂村父鏁般€傛灇涓?v4l2_preemphasis 瀹氫箟浜嗛鍔犻噸鐨?    鍙兘鍙栧€硷紝濡備笅锛?
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_PREEMPHASIS_DISABLED`
      - 涓嶅簲鐢ㄩ鍔犻噸銆?    - - `V4L2_PREEMPHASIS_50_uS`
      - 浣跨敤 50 寰鐨勯鍔犻噸銆?    - - `V4L2_PREEMPHASIS_75_uS`
      - 浣跨敤 75 寰鐨勯鍔犻噸銆?
`V4L2_CID_TUNE_POWER_LEVEL (integer)`
    璁剧疆淇″彿鍙戝皠鐨勮緭鍑哄姛鐜囩數骞炽€傚崟浣嶄负 dBuV銆傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_TUNE_ANTENNA_CAPACITOR (integer)`
    鎵嬪姩鎴栵紙鑻ヨ涓?0锛夎嚜鍔ㄩ€夋嫨澶╃嚎璋冭皭鐢靛鐨勫€笺€傚崟浣嶃€佽寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
鏈夊叧 RDS 瑙勮寖鐨勬洿澶氱粏鑺傦紝璇峰弬闃?CENELEC 鐨?iec62106 鏂囨。銆?