## ALSA Jack 杞欢娉ㄥ叆


## Jack 娉ㄥ叆绠€浠?

杩欓噷鐨?jack 娉ㄥ叆鏄寚鐢ㄦ埛鍙互閫氳繃 debugfs 鎺ュ彛鍚戦煶棰?jack 娉ㄥ叆鎻掑叆锛坧lugin锛夋垨鎷斿嚭锛坧lugout锛?浜嬩欢锛岃繖鏈夊姪浜庨獙璇?ALSA 鐢ㄦ埛绌洪棿鐨勬敼鍔ㄣ€備緥濡傦紝鎴戜滑淇敼浜?pulseaudio 涓殑闊抽閰嶇疆鏂囦欢鍒囨崲浠ｇ爜锛?鎴戜滑鎯抽獙璇佽鏀瑰姩鏄惁濡傞鏈熷伐浣溿€佹槸鍚﹀紩鍏ヤ簡鍥炲綊銆傝繖绉嶆儏鍐典笅锛屾垜浠彲浠ュ悜涓€涓垨鏌愪簺闊抽 jack
娉ㄥ叆鎻掑叆鎴栨嫈鍑轰簨浠讹紝鑰屾棤闇€瀹為檯鎺ヨЕ鏈哄櫒骞跺皢鐗╃悊璁惧鎻掓嫈鍒伴煶棰?jack 涓娿€?
鍦ㄨ繖涓璁′腑锛屼竴涓煶棰?jack 骞朵笉绛夊悓浜庝竴涓墿鐞嗛煶棰?jack銆傛湁鏃朵竴涓墿鐞嗛煶棰?jack 鍖呭惈澶氫釜鍔熻兘锛?ALSA 椹卞姩浼氫负涓€涓?`snd_jack` 鍒涘缓澶氫釜 `jack_kctl`锛岃繖閲?`snd_jack` 浠ｈ〃涓€涓墿鐞嗛煶棰?jack锛?鑰?`jack_kctl` 浠ｈ〃涓€涓姛鑳斤紝渚嬪涓€涓墿鐞?jack 鏈変袱涓姛鑳斤細headphone 鍜?mic_in锛孉LSA ASoC
椹卞姩浼氫负姝?jack 鏋勫缓 2 涓?`jack_kctl`銆俲ack 娉ㄥ叆鏄熀浜?`jack_kctl` 鑰岄潪 `snd_jack` 瀹炵幇鐨勩€?
瑕佸悜闊抽 jack 娉ㄥ叆浜嬩欢锛屾垜浠渶瑕佸厛閫氳繃 `sw_inject_enable` 鍚敤 jack 娉ㄥ叆锛屼竴鏃﹀惎鐢紝璇?jack
灏嗕笉鍐嶅洜纭欢浜嬩欢鑰屾敼鍙樼姸鎬侊紝鎴戜滑鍙互閫氳繃 `jackin_inject` 娉ㄥ叆鎻掑叆鎴栨嫈鍑轰簨浠讹紝骞堕€氳繃 `status`
妫€鏌?jack 鐘舵€侊紝娴嬭瘯瀹屾垚鍚庢垜浠篃闇€瑕侀€氳繃 `sw_inject_enable` 绂佺敤 jack 娉ㄥ叆锛屼竴鏃︾鐢紝jack
鐘舵€佸皢鏍规嵁鏈€鍚庝竴娆℃姤鍛婄殑纭欢浜嬩欢鎭㈠锛屽苟灏嗛殢鏈潵鐨勭‖浠朵簨浠惰€屾敼鍙樸€?
## Jack 娉ㄥ叆鎺ュ彛鐨勫竷灞€


濡傛灉鐢ㄦ埛鍦ㄥ唴鏍镐腑鍚敤浜?SND_JACK_INJECTION_DEBUG锛岄煶棰?jack 娉ㄥ叆鎺ュ彛灏嗗涓嬪垱寤猴細
```

   $debugfs_mount_dir/sound
   |-- card0
   |-- |-- HDMI_DP_pcm_10_Jack
   |-- |-- |-- jackin_inject
   |-- |-- |-- kctl_id
   |-- |-- |-- mask_bits
   |-- |-- |-- status
   |-- |-- |-- sw_inject_enable
   |-- |-- |-- type
   ...
   |-- |-- HDMI_DP_pcm_9_Jack
   |--     |-- jackin_inject
   |--     |-- kctl_id
   |--     |-- mask_bits
   |--     |-- status
   |--     |-- sw_inject_enable
   |--     |-- type
   |-- card1
       |-- HDMI_DP_pcm_5_Jack
       |-- |-- jackin_inject
       |-- |-- kctl_id
       |-- |-- mask_bits
       |-- |-- status
       |-- |-- sw_inject_enable
       |-- |-- type
       ...
       |-- Headphone_Jack
       |-- |-- jackin_inject
       |-- |-- kctl_id
       |-- |-- mask_bits
       |-- |-- status
       |-- |-- sw_inject_enable
       |-- |-- type
       |-- Headset_Mic_Jack
           |-- jackin_inject
           |-- kctl_id
           |-- mask_bits
           |-- status
           |-- sw_inject_enable
           |-- type

```
## 鍚勮妭鐐圭殑瑙ｉ噴


kctl_id
  read-only锛岃幏鍙?jack_kctl->kctl 鐨?id
```

     sound/card1/Headphone_Jack# cat kctl_id
     Headphone Jack

```
mask_bits
  read-only锛岃幏鍙?jack_kctl 鏀寔鐨?events mask_bits
```

     sound/card1/Headphone_Jack# cat mask_bits
     0x0001 HEADPHONE(0x0001)

```
status
  read-only锛岃幏鍙?jack_kctl 鐨勫綋鍓嶇姸鎬?
- 鑰虫満鏈彃鍏ワ細

```

     sound/card1/Headphone_Jack# cat status
     Unplugged

```
- 鑰虫満宸叉彃鍏ワ細

```

     sound/card1/Headphone_Jack# cat status
     Plugged

```
type
  read-only锛屼粠 type 鑾峰彇 snd_jack 鏀寔鐨?events锛堢墿鐞嗛煶棰?jack 涓婃墍鏈夋敮鎸佺殑 events锛?```

     sound/card1/Headphone_Jack# cat type
     0x7803 HEADPHONE(0x0001) MICROPHONE(0x0002) BTN_3(0x0800) BTN_2(0x1000) BTN_1(0x2000) BTN_0(0x4000)

```
sw_inject_enable
  read-write锛屽惎鐢ㄦ垨绂佺敤娉ㄥ叆

- 娉ㄥ叆宸茬鐢細

```

     sound/card1/Headphone_Jack# cat sw_inject_enable
     Jack: Headphone Jack		Inject Enabled: 0

```
- 娉ㄥ叆宸插惎鐢細

```

     sound/card1/Headphone_Jack# cat sw_inject_enable
     Jack: Headphone Jack		Inject Enabled: 1

```
- 鍚敤 jack 娉ㄥ叆锛?
```

     sound/card1/Headphone_Jack# echo 1 > sw_inject_enable

```
- 绂佺敤 jack 娉ㄥ叆锛?
```

     sound/card1/Headphone_Jack# echo 0 > sw_inject_enable

```
jackin_inject
  write-only锛屾敞鍏ユ彃鍏ユ垨鎷斿嚭

- 娉ㄥ叆鎻掑叆锛?
```

     sound/card1/Headphone_Jack# echo 1 > jackin_inject

```
- 娉ㄥ叆鎷斿嚭锛?
```

     sound/card1/Headphone_Jack# echo 0 > jackin_inject

```
