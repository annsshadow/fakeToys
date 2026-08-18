## ALSA 涓殑璺熻釜鐐?
2017/07/02
Takasahi Sakamoto

## ALSA PCM 鏍稿績涓殑璺熻釜鐐?
ALSA PCM 鏍稿績鍚戝唴鏍歌窡韪偣绯荤粺娉ㄥ唽浜?`snd_pcm` 瀛愮郴缁熴€傝瀛愮郴缁熷寘鍚袱绫昏窡韪偣锛?涓€绫荤敤浜?PCM 缂撳啿鍖虹殑鐘舵€侊紝鍙︿竴绫荤敤浜?PCM 纭欢鍙傛暟鐨勫鐞嗐€傚綋鍚敤鐩稿簲鐨勫唴鏍搁厤缃?鏃讹紝杩欎簺璺熻釜鐐规墠鍙敤銆傚綋鍚敤 `CONFIG_SND_DEBUG` 鏃讹紝鍚庝竴绫昏窡韪偣鍙敤銆傚綋杩樺惎鐢ㄤ簡
`SND_PCM_XRUN_DEBUG` 鏃讹紝鍓嶄竴绫昏窡韪偣涔熶細琚惎鐢ㄣ€?
### 鐢ㄤ簬 PCM 缂撳啿鍖虹姸鎬佺殑璺熻釜鐐?
璇ョ被鍒寘鍚洓涓窡韪偣锛歚hwptr`銆乣applptr`銆乣xrun` 鍜?`hw_ptr_error`銆?
### 鐢ㄤ簬 PCM 纭欢鍙傛暟澶勭悊鐨勮窡韪偣

璇ョ被鍒寘鍚袱涓窡韪偣锛歚hw_mask_param` 鍜?`hw_interval_param`銆?
鍦?ALSA PCM 鏍稿績鐨勮璁′腑锛屾暟鎹紶杈撹鎶借薄涓?PCM 瀛愭祦锛坰ubstream锛夈€傚簲鐢ㄧ▼搴忕鐞?PCM 瀛愭祦浠ョ淮鎶?PCM 甯х殑鏁版嵁浼犺緭銆傚湪寮€濮嬫暟鎹紶杈撲箣鍓嶏紝搴旂敤绋嬪簭闇€瑕侀厤缃?PCM 瀛愭祦銆?鍦ㄦ杩囩▼涓紝PCM 纭欢鍙傛暟鐢卞簲鐢ㄧ▼搴忎笌 ALSA PCM 鏍稿績涔嬮棿鐨勪氦浜掓潵鍐冲畾銆備竴鏃﹀喅瀹氾紝
PCM 瀛愭祦鐨勮繍琛屾椂锛坮untime锛夊氨浼氫繚瀛樿繖浜涘弬鏁般€?
杩欎簺鍙傛暟鍦?struct snd_pcm_hw_params 涓弿杩般€傝缁撴瀯浣撳寘鍚嚑绉嶇被鍨嬬殑鍙傛暟銆傚簲鐢ㄧ▼搴?涓鸿繖浜涘弬鏁拌缃亸濂界殑鍊硷紝鐒跺悗鎵ц甯?SNDRV_PCM_IOCTL_HW_REFINE 鎴?SNDRV_PCM_IOCTL_HW_PARAMS 鐨?ioctl(2)銆傚墠鑰呬粎鐢ㄤ簬绮剧畝鍙敤鐨勫弬鏁伴泦鍚堬紝鍚庤€呯敤浜?瀹為檯鍐冲畾鍙傛暟銆?
struct snd_pcm_hw_params 缁撴瀯浣撳叿鏈変互涓嬫垚鍛橈細

`flags`
        鍙厤缃€侫LSA PCM 鏍稿績鍜屾煇浜涢┍鍔ㄤ細澶勭悊璇ユ爣蹇楋紝浠ラ€夋嫨鏂逛究鐨勫弬鏁版垨鏀瑰彉鍏惰涓恒€?`masks`
        鍙厤缃€傝繖绫诲弬鏁板湪 struct snd_mask 涓弿杩帮紝琛ㄧず鎺╃爜鍊笺€傛埅鑷?PCM 鍗忚
        v2.0.13锛屽畾涔変簡涓夌绫诲瀷銆?
        - SNDRV_PCM_HW_PARAM_ACCESS
        - SNDRV_PCM_HW_PARAM_FORMAT
        - SNDRV_PCM_HW_PARAM_SUBFORMAT
`intervals`
        鍙厤缃€傝繖绫诲弬鏁板湪 struct snd_interval 涓弿杩帮紝琛ㄧず甯﹁寖鍥寸殑鍊笺€傛埅鑷?        PCM 鍗忚 v2.0.13锛屽畾涔変簡鍗佷簩绉嶇被鍨嬨€?
        - SNDRV_PCM_HW_PARAM_SAMPLE_BITS
        - SNDRV_PCM_HW_PARAM_FRAME_BITS
        - SNDRV_PCM_HW_PARAM_CHANNELS
        - SNDRV_PCM_HW_PARAM_RATE
        - SNDRV_PCM_HW_PARAM_PERIOD_TIME
        - SNDRV_PCM_HW_PARAM_PERIOD_SIZE
        - SNDRV_PCM_HW_PARAM_PERIOD_BYTES
        - SNDRV_PCM_HW_PARAM_PERIODS
        - SNDRV_PCM_HW_PARAM_BUFFER_TIME
        - SNDRV_PCM_HW_PARAM_BUFFER_SIZE
        - SNDRV_PCM_HW_PARAM_BUFFER_BYTES
        - SNDRV_PCM_HW_PARAM_TICK_TIME
`rmask`
        鍙厤缃€備粎鍦ㄥ甫 SNDRV_PCM_IOCTL_HW_REFINE 鐨?ioctl(2) 涓眰鍊笺€傚簲鐢ㄧ▼搴?        鍙互閫夋嫨鍝簺鎺╃爜/鍖洪棿鍙傛暟鍙互鐢?ALSA PCM 鏍稿績鏇存敼銆傚浜?        SNDRV_PCM_IOCTL_HW_PARAMS锛岃鎺╃爜浼氳蹇界暐锛屾墍鏈夊弬鏁伴兘灏嗚鏇存敼銆?`cmask`
        鍙銆備粠 ioctl(2) 杩斿洖鍚庯紝鐢ㄦ埛绌洪棿涓敤浜?struct snd_pcm_hw_params 鐨?        缂撳啿鍖哄寘鍚瘡娆℃搷浣滅殑缁撴灉銆傝鎺╃爜琛ㄧず瀹為檯鏇存敼浜嗗摢涓帺鐮?鍖洪棿鍙傛暟銆?`info`
        鍙銆備互 SNDRV_PCM_INFO_XXX 浣嶆爣蹇楄〃绀虹‖浠?椹卞姩鑳藉姏銆傞€氬父锛屽簲鐢ㄧ▼搴?        鎵ц甯?SNDRV_PCM_IOCTL_HW_REFINE 鐨?ioctl(2) 鏉ユ绱㈣鏍囧織锛岀劧鍚庡喅瀹?        鍙傛暟鐨勫€欓€夊€硷紝骞舵墽琛屽甫 SNDRV_PCM_IOCTL_HW_PARAMS 鐨?ioctl(2) 鏉ラ厤缃?        PCM 瀛愭祦銆?`msbits`
        鍙銆傝鍊艰〃绀?PCM 鏍锋湰涓?MSB 涓€渚у彲鐢ㄧ殑浣嶅銆傚綋
        SNDRV_PCM_HW_PARAM_SAMPLE_BITS 鍙傛暟琚喅瀹氫负涓€涓浐瀹氭暟鍊兼椂锛岃鍊间篃浼?        鎹璁＄畻鍑烘潵銆傚惁鍒欎负闆躲€備絾璇ヨ涓哄彇鍐充簬椹卞姩渚х殑瀹炵幇銆?`rate_num`
        鍙銆傝鍊艰〃绀哄垎鏁拌〃绀烘硶涓噰鏍风巼鐨勫垎瀛愩€傚熀鏈笂锛屽綋 SNDRV_PCM_HW_PARAM_RATE
        鍙傛暟琚喅瀹氫负鍗曚竴鍊兼椂锛岃鍊间篃浼氭嵁姝よ绠楀嚭鏉ャ€傚惁鍒欎负闆躲€備絾璇ヨ涓哄彇鍐充簬
        椹卞姩渚х殑瀹炵幇銆?`rate_den`
        鍙銆傝鍊艰〃绀哄垎鏁拌〃绀烘硶涓噰鏍风巼鐨勫垎姣嶃€傚熀鏈笂锛屽綋 SNDRV_PCM_HW_PARAM_RATE
        鍙傛暟琚喅瀹氫负鍗曚竴鍊兼椂锛岃鍊间篃浼氭嵁姝よ绠楀嚭鏉ャ€傚惁鍒欎负闆躲€備絾璇ヨ涓哄彇鍐充簬
        椹卞姩渚х殑瀹炵幇銆?`fifo_size`
        鍙銆傝鍊艰〃绀虹‖浠朵覆琛岄煶棰戞帴鍙ｄ腑 FIFO 鐨勫ぇ灏忋€傚熀鏈笂锛屾瘡涓┍鍔ㄩ兘鍙互
        涓鸿鍙傛暟鍒嗛厤鍚堥€傜殑鍊硷紝浣嗘煇浜涢┍鍔ㄥ嚭浜庡纭欢璁捐鎴栨暟鎹紶杈撳崗璁殑鑰冭檻
        浼氭晠鎰忚涓洪浂銆?
褰撳簲鐢ㄧ▼搴忔墽琛屽甫 SNDRV_PCM_IOCTL_HW_REFINE 鎴?SNDRV_PCM_IOCTL_HW_PARAMS 鐨?ioctl(2) 鏃讹紝ALSA PCM 鏍稿績浼氬鐞?struct snd_pcm_hw_params 鐨勭紦鍐插尯銆傜紦鍐插尯涓殑
鍙傛暟浼氭牴鎹?struct snd_pcm_hardware 浠ュ強杩愯鏃朵腑鐨勭害鏉熻鍒欒€屾敼鍙樸€傝缁撴瀯浣撴弿杩?鎵€澶勭悊纭欢鐨勮兘鍔涖€傝繖浜涜鍒欐弿杩颁簡鍙傛暟渚濇嵁鑻ュ共鍙傛暟琚喅瀹氱殑渚濊禆鍏崇郴銆備竴鏉¤鍒欏甫鏈?涓€涓洖璋冨嚱鏁帮紝椹卞姩鍙互娉ㄥ唽浠绘剰鍑芥暟鏉ヨ绠楃洰鏍囧弬鏁般€侫LSA PCM 鏍稿績浼氶粯璁ゅ悜杩愯鏃?娉ㄥ唽涓€浜涜鍒欍€?
鍙椹卞姩鍦?struct snd_pcm_ops.open 鐨勫洖璋冧腑鍑嗗濂戒簡涓や欢浜嬶紝灏卞彲浠ュ弬涓庤繖涓€浜や簰銆?
1. 鍦ㄨ鍥炶皟涓紝椹卞姩搴斿綋渚濇嵁鐩稿簲纭欢鐨勮兘鍔涳紝鏀瑰彉杩愯鏃朵腑 struct snd_pcm_hardware
   绫诲瀷鐨勬垚鍛樸€?2. 鍦ㄥ悓涓€涓洖璋冧腑锛屽綋鑻ュ共鍙傛暟鍥犵‖浠惰璁¤€屽瓨鍦ㄤ緷璧栧叧绯绘椂锛岄┍鍔ㄨ繕搴斿綋鍚戣繍琛屾椂
   娉ㄥ唽棰濆鐨勭害鏉熻鍒欍€?
椹卞姩鍙互鍦?struct snd_pcm_ops.hw_params 鐨勫洖璋冧腑寮曠敤浜や簰鐨勭粨鏋滐紝浣嗕笉搴旀洿鏀瑰叾鍐呭銆?
璇ョ被鍒腑鐨勮窡韪偣鏃ㄥ湪杩借釜鎺╃爜/鍖洪棿鍙傛暟鐨勫彉鍖栥€傚綋 ALSA PCM 鏍稿績鏇存敼瀹冧滑鏃讹紝浼氭牴鎹?鎵€鏇存敼鍙傛暟鐨勭被鍨嬫帰娴嬪埌 `hw_mask_param` 鎴?`hw_interval_param` 浜嬩欢銆?
ALSA PCM 鏍稿績杩樹负姣忎釜璺熻釜鐐规彁渚涗簡婕備寒鐨勬墦鍗版牸寮忋€備笅闈㈡槸 `hw_mask_param` 鐨勭ず渚嬨€?
```

    hw_mask_param: pcmC0D0p 001/023 FORMAT 00000000000000000000001000000044 00000000000000000000001000000044

```
涓嬮潰鏄?`hw_interval_param` 鐨勭ず渚嬨€?
```

    hw_interval_param: pcmC0D0p 000/023 BUFFER_SIZE 0 0 [0 4294967295] 0 1 [0 4294967295]

```
鍓嶄笁涓瓧娈垫槸閫氱敤鐨勩€傚畠浠緷娆¤〃绀?ALSA PCM 瀛楃璁惧鐨勫悕绉般€佺害鏉熻鍒欎互鍙婅鏇存敼
鍙傛暟鐨勫悕绉般€傜害鏉熻鍒欏瓧娈电敱涓や釜瀛愬瓧娈电粍鎴愶細鎵€搴旂敤瑙勫垯鐨勭储寮曪紝浠ュ強娣诲姞鍒拌繍琛屾椂鐨?瑙勫垯鎬绘暟銆備綔涓轰緥澶栵紝绱㈠紩 000 琛ㄧず璇ュ弬鏁扮敱 ALSA PCM 鏍稿績鏇存敼锛屼笌瑙勫垯鏃犲叧銆?
鍏朵綑瀛楁琛ㄧず鍙傛暟鏇存敼涔嬪墠/涔嬪悗鐨勭姸鎬併€傝繖浜涘瓧娈垫牴鎹弬鏁扮殑绫诲瀷鑰屼笉鍚屻€傚浜庢帺鐮佺被鍨?鐨勫弬鏁帮紝杩欎簺瀛楁琛ㄧず璇ュ弬鏁板唴瀹圭殑鍗佸叚杩涘埗杞偍銆傚浜庡尯闂寸被鍨嬬殑鍙傛暟锛岃繖浜涘瓧娈垫寜
姝ら『搴忚〃绀?struct snd_interval 涓?`empty`銆乣integer`銆乣openmin`銆乣min`銆乣max`銆?`openmax` 鍚勬垚鍛樼殑鍊笺€?
## 椹卞姩涓殑璺熻釜鐐?
鏌愪簺椹卞姩涓轰簡寮€鍙戣€呯殑渚垮埄鎻愪緵浜嗚窡韪偣銆傚叧浜庡畠浠紝璇峰弬鑰冨悇鑷殑鏂囨。鎴栧疄鐜般€?