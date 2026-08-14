## HD-Audio DP-MST 鏀寔


涓轰簡鏀寔 DP MST 闊抽锛孒D Audio hdmi 缂栬В鐮佸櫒椹卞姩寮曞叆浜嗚櫄鎷熷紩鑴氾紙virtual pin锛夊拰鍔ㄦ€?pcm 鍒嗛厤銆?
铏氭嫙寮曡剼鏄?per_pin 鐨勬墿灞曘€侱P MST 涓庝紶缁熺殑鏍规湰鍖哄埆鍦ㄤ簬 DP MST 寮曞叆浜嗚澶囨潯鐩紙device entry锛夈€傛瘡涓紩鑴氬彲浠ュ寘鍚涓澶囨潯鐩€傛瘡涓澶囨潯鐩殑琛屼负灏卞儚涓€涓紩鑴氥€?
鐢变簬姣忎釜寮曡剼鍙兘鍖呭惈澶氫釜璁惧鏉＄洰锛岃€屾瘡涓紪瑙ｇ爜鍣ㄥ彲鑳藉寘鍚涓紩鑴氾紝濡傛灉鎴戜滑瀵规瘡涓?per_pin 浣跨敤涓€涓?pcm锛屽氨浼氫骇鐢熷ぇ閲?PCM銆傛柊鐨勬柟妗堟槸鍒涘缓灏戦噺 PCM锛屽苟鍔ㄦ€佸湴灏?pcm 缁戝畾鍒?per_pin銆傞┍鍔ㄤ娇鐢?spec->dyn_pcm_assign 鏍囧織鏉ユ寚绀烘槸鍚︿娇鐢ㄦ柊鏂规銆?
## PCM

寰呰ˉ鍏?
## 寮曡剼鍒濆鍖?

姣忎釜寮曡剼鍙兘鏈夊涓澶囨潯鐩紙铏氭嫙寮曡剼锛夈€傚湪 Intel 骞冲彴涓婏紝璁惧鏉＄洰鏁伴噺鏄姩鎬佸彉鍖栫殑銆傚鏋滆繛鎺ヤ簡 DP MST hub锛屽垯澶勪簬 DP MST 妯″紡锛岃澶囨潯鐩暟閲忎负 3銆傚惁鍒欙紝璁惧鏉＄洰鏁伴噺涓?1銆?
涓轰簡绠€鍖栧疄鐜帮紝鏃犺鏄惁澶勪簬 DP MST 妯″紡锛屾墍鏈夎澶囨潯鐩兘浼氬湪鍚姩鏃跺垵濮嬪寲銆?
## 杩炴帴鍒楄〃


DP MST 澶嶇敤浜嗚繛鎺ュ垪琛ㄤ唬鐮併€備唬鐮佸彲浠ュ鐢ㄦ槸鍥犱负鍚屼竴寮曡剼涓婄殑璁惧鏉＄洰鍏锋湁鐩稿悓鐨勮繛鎺ュ垪琛ㄣ€?
杩欐剰鍛崇潃 DP MST 鏃犻渶璁惧鏉＄洰璁剧疆鍗冲彲鑾峰彇璁惧鏉＄洰鐨勮繛鎺ュ垪琛ㄣ€?
## 鎻掑瓟锛圝ack锛?

鍋囪锛? - MST 蹇呴』鏄?dyn_pcm_assign锛屼笖瀹冩槸 acomp锛堥拡瀵?Intel 鍦烘櫙锛夛紱
 - NON-MST 鍙兘鏄篃鍙兘涓嶆槸 dyn_pcm_assign锛屽畠鍙互鏄?acomp 鎴?!acomp锛?
鍥犳瀛樺湪浠ヤ笅鍦烘櫙锛? a. MST锛?& dyn_pcm_assign && acomp锛? b. NON-MST && dyn_pcm_assign && acomp
 c. NON-MST && !dyn_pcm_assign && !acomp

涓嬮潰鐨勮璁哄皢蹇界暐 MST 鍜?NON-MST 鐨勫尯鍒紝鍥犱负瀹冨鎻掑瓟澶勭悊褰卞搷涓嶅ぇ銆?
椹卞姩鍦?hdmi_spec 涓娇鐢?struct hdmi_pcm pcm[] 鏁扮粍锛宻nd_jack 鏄?hdmi_pcm 鐨勪竴涓垚鍛樸€傛瘡涓紩鑴氭湁涓€涓?struct hdmi_pcm * pcm 鎸囬拡銆?
瀵逛簬 !dyn_pcm_assign锛宲er_pin->pcm 浼氬湪鍒濆鍖栨椂闈欐€佸湴鍒嗛厤鍒?spec->pcm[n]銆?
瀵逛簬 dyn_pcm_assign锛宲er_pin->pcm 浼氬湪鏄剧ず鍣ㄧ儹鎻掓嫈鏃跺垎閰嶅埌 spec->pcm[n]銆?

### 鏋勫缓鎻掑瓟


- dyn_pcm_assign

  涓嶄娇鐢?hda_jack锛岃€屾槸鐩存帴浣跨敤 spec->pcm_rec[pcm_idx].jack 涓殑 snd_jack銆?
- !dyn_pcm_assign

  浣跨敤 hda_jack锛屽苟闈欐€佸湴灏?spec->pcm_rec[pcm_idx].jack = jack->jack銆?

### 寮€鍚潪璇锋眰浜嬩欢


濡傛灉涓嶆槸 acomp锛屽垯寮€鍚潪璇锋眰浜嬩欢锛坲nsolicited event锛夈€?

### 鏄剧ず鍣ㄧ儹鎻掓嫈浜嬩欢澶勭悊


- acomp

  pin_eld_notify() -> check_presence_and_report() -> hdmi_present_sense() ->
  sync_eld_via_acomp()銆?
  鏃犺鏄?dyn_pcm_assign 杩樻槸 !dyn_pcm_assign锛岄兘鐩存帴鍦?spec->pcm_rec[pcm_idx].jack 涓婅皟鐢?snd_jack_report()

- !acomp

  hdmi_unsol_event() -> hdmi_intrinsic_event() -> check_presence_and_report() ->
  hdmi_present_sense() -> hdmi_prepsent_sense_via_verbs()

  瀵逛簬 dyn_pcm_assign锛岀洿鎺ュ湪 spec->pcm_rec[pcm_idx].jack 涓婅皟鐢?snd_jack_report()銆?  浣跨敤 hda_jack 鏈哄埗鏉ュ鐞嗘彃瀛斾簨浠躲€?

## 鍏朵粬寰呭悗缁ˉ鍏?