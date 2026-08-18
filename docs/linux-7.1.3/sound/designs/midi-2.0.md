
## Linux 涓婄殑 MIDI 2.0


## 姒傝堪


MIDI 2.0 鏄竴绉嶆墿灞曞崗璁紝鐢ㄤ簬鎻愪緵姣斾紶缁?MIDI 1.0 鏇撮珮鐨勫垎杈ㄧ巼浠ュ強鏇寸簿缁嗙殑鎺у埗銆備负鏀拺 MIDI 2.0 鑰屽紩鍏ョ殑鏍规湰鎬у彉鍖栧寘鎷細

- 鏀寔閫氱敤 MIDI 鏁版嵁鍖咃紙Universal MIDI Packet锛岀畝绉?UMP锛?- 鏀寔 MIDI 2.0 鍗忚娑堟伅
- UMP 涓庝紶缁?MIDI 1.0 瀛楄妭娴佷箣闂寸殑閫忔槑杞崲
- 鐢ㄤ簬灞炴€т笌閰嶇疆鏂囦欢閰嶇疆鐨?MIDI-CI

UMP 鏄竴绉嶆柊鐨勫鍣ㄦ牸寮忥紝鐢ㄤ簬鎵胯浇鎵€鏈?MIDI 鍗忚 1.0 涓?MIDI 2.0 鍗忚娑堟伅銆備笌浠ュ線鐨勫瓧鑺傛祦涓嶅悓锛屽畠鏄?32 浣嶅榻愮殑锛屽苟涓旀瘡鏉℃秷鎭兘鍙互鏀惧叆鍗曚釜鏁版嵁鍖呬腑銆俇MP 鏈€澶氬彲浠ュ彂閫?16 涓€淯MP 缁勶紙UMP Group锛夆€濈殑浜嬩欢锛屽叾涓瘡涓?UMP 缁勬渶澶氬寘鍚?16 涓?MIDI 閫氶亾銆?
MIDI 2.0 鍗忚鏄竴绉嶆墿灞曞崗璁紝鐢ㄤ簬瀹炵幇姣旀棫鐨?MIDI 1.0 鍗忚鏇撮珮鐨勫垎杈ㄧ巼涓庢洿澶氱殑鎺у埗銆?
MIDI-CI 鏄竴绉嶉珮灞傚崗璁紝鍙互涓?MIDI 璁惧杩涜鐏垫椿閰嶇疆鏂囦欢涓庨厤缃殑鍗忓晢銆傚畠浠ョ壒娈?SysEx 鐨勫舰寮忚〃绀恒€?
瀵逛簬 Linux 瀹炵幇锛屽唴鏍告敮鎸?UMP 浼犺緭浠ュ強鍦?UMP 涓婂 MIDI 鍗忚杩涜缂栬В鐮侊紝鑰?MIDI-CI 鍒欏湪鐢ㄦ埛绌洪棿閫氳繃鏍囧噯 SysEx 鑾峰緱鏀寔銆?
鎴嚦鏈枃鎾板啓鏃讹紝鍙湁 USB MIDI 璁惧鍘熺敓鏀寔 UMP 涓?Linux 2.0銆俇MP 鏀寔鏈韩鏄浉褰撻€氱敤鐨勶紝鍥犳瀹冧篃鍙互琚叾浠栦紶杈撳眰浣跨敤锛屽敖绠″畠涔熷彲鑳戒互涓嶅悓鐨勬柟寮忓疄鐜帮紙渚嬪浣滀负 ALSA 闊冲簭鍣ㄥ鎴风锛夈€?
瀵?UMP 璁惧鐨勮闂互涓ょ鏂瑰紡鎻愪緵锛氶€氳繃 rawmidi 璁惧鐨勮闂紝浠ュ強閫氳繃 ALSA 闊冲簭鍣?API 鐨勮闂€?
ALSA 闊冲簭鍣?API 宸茶鎵╁睍浠ュ厑璁?UMP 鏁版嵁鍖呯殑璐熻浇銆傚厑璁稿湪 MIDI 1.0 涓?MIDI 2.0 闊冲簭鍣ㄥ鎴风涔嬮棿鑷敱杩炴帴锛屽苟涓斾簨浠朵細琚€忔槑鍦拌浆鎹€?

## 鍐呮牳閰嶇疆


涓烘敮鎸?MIDI 2.0锛屾柊澧炰簡浠ヤ笅閰嶇疆椤癸細
`CONFIG_SND_UMP`銆乣CONFIG_SND_UMP_LEGACY_RAWMIDI`銆?`CONFIG_SND_SEQ_UMP`銆乣CONFIG_SND_SEQ_UMP_CLIENT`锛屼互鍙?`CONFIG_SND_USB_AUDIO_MIDI_V2`銆傜涓€涓彲瑙佺殑鏄?`CONFIG_SND_USB_AUDIO_MIDI_V2`锛屽綋浣犻€夋嫨瀹冿紙璁剧疆涓?`=y`锛夋椂锛?UMP 鐨勬牳蹇冩敮鎸侊紙`CONFIG_SND_UMP`锛変笌闊冲簭鍣ㄧ粦瀹?锛坄CONFIG_SND_SEQ_UMP_CLIENT`锛変細琚嚜鍔ㄩ€変腑銆?
姝ゅ锛宍CONFIG_SND_UMP_LEGACY_RAWMIDI=y` 灏嗕负 UMP 绔偣鍚敤瀵?浼犵粺 raw MIDI 璁惧鐨勬敮鎸併€?

## 浣跨敤 USB MIDI 2.0 鐨?Rawmidi 璁惧


褰撹澶囨敮鎸?MIDI 2.0 鏃讹紝USB 闊抽椹卞姩浼氭帰娴嬪苟浣跨敤 MIDI 2.0 鎺ュ彛
锛堝缁堜綅浜?altset 1锛変綔涓洪粯璁ゆ帴鍙ｏ紝鑰岄潪 MIDI 1.0 鎺ュ彛锛堜綅浜?altset 0锛夈€?浣犱篃鍙互閫氳繃灏?`midi2_enable=0` 閫夐」浼犻€掔粰 snd-usb-audio 椹卞姩妯″潡锛?鍒囨崲鍥炰娇鐢ㄦ棫鐨?MIDI 1.0 鎺ュ彛鐨勭粦瀹氥€?
USB 闊抽椹卞姩浼氬皾璇曟煡璇㈣嚜 UMP v1.1 璧锋彁渚涚殑 UMP Endpoint 涓?UMP Function
Block 淇℃伅锛屽苟鍩轰簬杩欎簺淇℃伅鏋勫缓鎷撴墤銆傚綋璁惧杈冩棫銆佸鏂?UMP 鏌ヨ鏃犲搷搴旀椂锛?椹卞姩浼氬洖閫€骞跺熀浜庢潵鑷?USB 鎻忚堪绗︾殑 Group Terminal Block锛圙TB锛変俊鎭瀯寤烘嫇鎵戙€?鏌愪簺璁惧鍙兘浼氳鎰忓鐨?UMP 鍛戒护鎼炰贡锛涘湪杩欑鎯呭喌涓嬶紝鍚?snd-usb-audio 椹卞姩
浼犻€?`midi2_ump_probe=0` 閫夐」浠ヨ烦杩?UMP v1.1 鏌ヨ銆?
褰撴帰娴嬪埌 MIDI 2.0 璁惧鏃讹紝鍐呮牳浼氫负璇ヨ澶囩殑姣忎釜 UMP Endpoint 鍒涘缓涓€涓?rawmidi 璁惧銆傚叾璁惧鍚嶄负 `/dev/snd/umpC**D**`锛屼笉鍚屼簬鏍囧噯 rawmidi 璁惧鍚?`/dev/snd/midiC**D**`锛堝搴?MIDI 1.0锛夛紝浠ラ伩鍏嶄紶缁熷簲鐢ㄧ▼搴忚璁块棶 UMP 璁惧銆?
浣犲彲浠ョ洿鎺ュ璇?UMP rawmidi 璁惧杩涜 UMP 鏁版嵁鍖呯殑璇诲彇涓庡啓鍏ャ€備緥濡傦紝鍍忎笅闈㈣繖鏍?閫氳繃 `hexdump` 璇诲彇锛屽皢浠ュ崄鍏繘鍒跺舰寮忔樉绀哄崱 0 璁惧 0 鐨勪紶鍏?UMP 鏁版嵁鍖?
```
  % hexdump -C /dev/snd/umpC0D0
  00000000  01 07 b0 20 00 07 b0 20  64 3c 90 20 64 3c 80 20  |... ... d<. d<. |
```

涓?MIDI 1.0 瀛楄妭娴佷笉鍚岋紝UMP 鏄竴涓?32 浣嶆暟鎹寘锛屽苟涓旇鍙栨垨鍐欏叆璁惧鏃剁殑
澶у皬涔熸寜 32 浣嶏紙鍗?4 瀛楄妭锛夊榻愩€?
UMP 鏁版嵁鍖呰礋杞戒腑鐨?32 浣嶅瓧濮嬬粓閲囩敤 CPU 鏈満瀛楄妭搴忋€備紶杈撻┍鍔ㄨ礋璐ｅ皢 UMP 瀛?浠?鍚戠郴缁熷瓧鑺傚簭杞崲涓烘墍闇€鐨勪紶杈撳瓧鑺傚簭/瀛楄妭椤哄簭銆?
褰撹缃簡 `CONFIG_SND_UMP_LEGACY_RAWMIDI` 鏃讹紝椹卞姩浼氶澶栧垱寤轰竴涓爣鍑?raw MIDI
璁惧 `/dev/snd/midiC**D**`銆傚畠鍖呭惈 16 涓瓙娴侊紝姣忎釜瀛愭祦瀵瑰簲涓€涓紙浠?0 寮€濮嬭鏁扮殑锛?UMP 缁勩€備紶缁熷簲鐢ㄧ▼搴忓彲浠ラ€氳繃姣忎釜瀛愭祦浠?MIDI 1.0 瀛楄妭娴佹牸寮忚闂寚瀹氱殑缁勩€?浣跨敤 ALSA rawmidi API 鏃讹紝浣犲彲浠ユ墦寮€浠绘剰瀛愭祦锛岃€屼粎鎵撳紑 `/dev/snd/midiC**D**`
鏈€缁堜細鎵撳紑绗竴涓瓙娴併€?
姣忎釜 UMP Endpoint 閮藉彲浠ユ彁渚涢檮鍔犱俊鎭紝杩欎簺淇℃伅鐢遍€氳繃 UMP 1.1 Stream 娑堟伅鎴?USB MIDI 2.0 鎻忚堪绗︽煡璇㈠緱鍒扮殑淇℃伅鏋勫缓鑰屾垚銆備竴涓?UMP Endpoint 鍙互鍖呭惈涓€涓垨澶氫釜
UMP Block锛屽叾涓?UMP Block 鏄?ALSA UMP 瀹炵幇涓紩鍏ョ殑涓€绉嶆娊璞★紝鐢ㄤ簬琛ㄧず UMP 缁勪箣闂寸殑
鍏宠仈銆俇MP Block 瀵瑰簲浜?UMP 1.1 瑙勮寖涓殑 Function Block銆傚綋 UMP 1.1 Function Block
淇℃伅涓嶅彲鐢ㄦ椂锛屼細閮ㄥ垎鍦颁粠 USB MIDI 2.0 瑙勮寖涓畾涔夌殑 Group Terminal Block锛圙TB锛?濉厖銆?
UMP Endpoint 涓?UMP Block 鐨勪俊鎭彲浠ュ湪 proc 鏂囦欢涓壘鍒?
```
  % cat /proc/asound/card1/midi0
  ProtoZOA MIDI

  Type: UMP
  EP Name: ProtoZOA
  EP Product ID: ABCD12345678
  UMP Version: 0x0000
  Protocol Caps: 0x00000100
  Protocol: 0x00000100
  Num Blocks: 3

  Block 0 (ProtoZOA Main)
    Direction: bidirection
    Active: Yes
    Groups: 1-1
    Is MIDI1: No

  Block 1 (ProtoZOA Ext IN)
    Direction: output
    Active: Yes
    Groups: 2-2
    Is MIDI1: Yes (Low Speed)
  ....

```

娉ㄦ剰锛屼笂闈?proc 鏂囦欢涓樉绀虹殑 `Groups` 瀛楁琛ㄧず鐨勬槸浠?1 寮€濮嬭鏁扮殑 UMP 缁勭紪鍙?锛堜粠-鍒帮級銆?
杩欎簺闄勫姞鐨?UMP Endpoint 涓?UMP Block 淇℃伅鍙互鍒嗗埆閫氳繃鏂扮殑 ioctl
`SNDRV_UMP_IOCTL_ENDPOINT_INFO` 涓?`SNDRV_UMP_IOCTL_BLOCK_INFO` 鑾峰彇銆?
rawmidi 鍚嶇О涓?UMP Endpoint 鍚嶇О閫氬父鐩稿悓锛屽浜?USB MIDI锛屽畠鍙栬嚜鐩稿簲 USB MIDI
鎺ュ彛鎻忚堪绗︾殑 `iInterface`銆傚鏋滄湭鎻愪緵锛屽垯浣滀负鍥為€€浠?USB 璁惧鎻忚堪绗︾殑 `iProduct`
澶嶅埗銆?
Endpoint Product ID 鏄竴涓瓧绗︿覆瀛楁锛屽簲褰撴槸鍞竴鐨勩€傚浜?USB MIDI锛屽畠浠庤澶囩殑
`iSerialNumber` 澶嶅埗鑰屾潵銆?
鍗忚鑳藉姏涓庡疄闄呭崗璁綅瀹氫箟鍦?`asound.h` 涓€?

## 浣跨敤 USB MIDI 2.0 鐨?ALSA 闊冲簭鍣?

闄や簡 rawmidi 鎺ュ彛涔嬪锛孉LSA 闊冲簭鍣ㄦ帴鍙ｄ篃鏀寔鏂扮殑 UMP MIDI 2.0 璁惧銆?鐜板湪锛屾瘡涓?ALSA 闊冲簭鍣ㄥ鎴风閮藉彲浠ヨ缃叾 MIDI 鐗堟湰锛?銆? 鎴?2锛夛紝浠ュ垎鍒０鏄?鑷韩涓轰紶缁熻澶囥€乁MP MIDI 1.0 璁惧鎴?UMP MIDI 2.0 璁惧銆傜涓€涓嵆浼犵粺瀹㈡埛绔紝
鎸夊師鏍峰彂閫?鎺ユ敹鏃у紡闊冲簭鍣ㄤ簨浠躲€傝€?UMP MIDI 1.0 涓?2.0 瀹㈡埛绔垯浠ョ敤浜?UMP 鐨?鎵╁睍浜嬩欢璁板綍鍙戦€佸拰鎺ユ敹銆侻IDI 鐗堟湰鍙互鍦?`snd_seq_client_info` 鐨勬柊瀛楁
`midi_version` 涓湅鍒般€?
閫氳繃鍦ㄩ煶搴忓櫒浜嬩欢涓寚瀹氭柊鐨勪簨浠舵爣蹇椾綅 `SNDRV_SEQ_EVENT_UMP`锛屽彲浠ヤ互宓屽叆鏂瑰紡
鍙戦€?鎺ユ敹 UMP 鏁版嵁鍖呫€傚綋璁剧疆姝ゆ爣蹇楁椂锛屼簨浠舵嫢鏈?16 瀛楄妭锛?28 浣嶏級鐨勬暟鎹礋杞芥潵
瀛樻斁 UMP 鏁版嵁鍖呫€傚鏋滀笉甯?`SNDRV_SEQ_EVENT_UMP` 鏍囧織浣嶏紝浜嬩欢灏嗗儚浠ュ墠涓€鏍疯瑙嗕负
浼犵粺浜嬩欢锛堟渶澶?12 瀛楄妭鏁版嵁璐熻浇锛夈€?
璁剧疆 `SNDRV_SEQ_EVENT_UMP` 鏍囧織鏃讹紝UMP 闊冲簭鍣ㄤ簨浠剁殑 type 瀛楁浼氳蹇界暐锛堜絾榛樿
搴旇涓?0锛夈€?
姣忎釜瀹㈡埛绔殑绫诲瀷鍙互鍦?`/proc/asound/seq/clients` 涓湅鍒般€?
```
  % cat /proc/asound/seq/clients
  Client info
    cur  clients : 3
  ....
  Client  14 : "Midi Through" [Kernel Legacy]
    Port   0 : "Midi Through Port-0" (RWe-)
  Client  20 : "ProtoZOA" [Kernel UMP MIDI1]
    UMP Endpoint: ProtoZOA
    UMP Block 0: ProtoZOA Main [Active]
      Groups: 1-1
    UMP Block 1: ProtoZOA Ext IN [Active]
      Groups: 2-2
    UMP Block 2: ProtoZOA Ext OUT [Active]
      Groups: 3-3
    Port   0 : "MIDI 2.0" (RWeX) [In/Out]
    Port   1 : "ProtoZOA Main" (RWeX) [In/Out]
    Port   2 : "ProtoZOA Ext IN" (-We-) [Out]
    Port   3 : "ProtoZOA Ext OUT" (R-e-) [In]

```

鍦ㄨ繖閲屼綘鍙互鎵惧埌涓ょ被鍐呮牳瀹㈡埛绔紝瀹㈡埛绔?14 涓?鈥淟egacy鈥濓紝瀹㈡埛绔?20 涓?鈥淯MP MIDI1鈥濓紝
瀹冨氨鏄竴涓?USB MIDI 2.0 璁惧銆?USB MIDI 2.0 瀹㈡埛绔缁堝皢绔彛 0 浣滀负 鈥淢IDI 2.0鈥?鎻愪緵锛屽叾浣欑鍙ｄ粠 1 寮€濮嬪搴旀瘡涓?UMP 缁勶紙渚嬪绔彛 1 瀵瑰簲缁?1锛夈€?鍦ㄦ绀轰緥涓紝璁惧鏈変笁涓椿鍔ㄧ粍锛圡ain銆丒xt IN 涓?Ext OUT锛夛紝瀹冧滑浣滀负闊冲簭鍣ㄧ鍙ｄ粠 1 鍒?3
鏆撮湶鍑烘潵銆?鈥淢IDI 2.0鈥?绔彛鐢ㄤ簬 UMP Endpoint锛屽畠涓庡叾浠?UMP 缁勭鍙ｇ殑鍖哄埆鍦ㄤ簬锛歎MP Endpoint 绔彛
鍙戦€佹潵鑷澶囦笂鎵€鏈夌鍙ｇ殑浜嬩欢锛堚€滄崟鑾峰叏閮ㄢ€濓紝catch-all锛夛紝鑰屾瘡涓?UMP 缁勭鍙ｅ彧鍙戦€?鏉ヨ嚜缁欏畾 UMP 缁勭殑浜嬩欢銆?姝ゅ锛屾棤缁勭殑 UMP 娑堟伅锛堜緥濡?UMP 娑堟伅绫诲瀷 0x0f锛夊彧浼氬彂閫佸埌 UMP Endpoint 绔彛銆?
娉ㄦ剰锛岃櫧鐒舵瘡涓?UMP 闊冲簭鍣ㄥ鎴风閫氬父浼氬垱寤?16 涓鍙ｏ紝浣嗛偅浜涗笉灞炰簬浠讳綍 UMP Block
锛堟垨灞炰簬闈炴椿鍔?UMP Block锛夌殑绔彛浼氳鏍囪涓轰笉娲诲姩锛屽苟涓斾笉浼氬嚭鐜板湪 proc 杈撳嚭涓€?鍦ㄤ笂闈㈢殑绀轰緥涓紝浠?4 鍒?16 鐨勯煶搴忓櫒绔彛鏄瓨鍦ㄧ殑锛屼絾娌℃湁鏄剧ず鍦ㄩ偅閲屻€?
涓婇潰鐨?proc 鏂囦欢涔熸樉绀轰簡 UMP Block 淇℃伅銆傚悓鏍风殑鏉＄洰锛堜絾甯︽湁鏇磋缁嗙殑淇℃伅锛夊彲浠ュ湪
rawmidi 鐨?proc 杈撳嚭涓壘鍒般€?
褰撳鎴风鍦ㄤ笉鍚?MIDI 鐗堟湰涔嬮棿杩炴帴鏃讹紝浜嬩欢浼氭牴鎹鎴风鐨勭増鏈嚜鍔ㄨ浆鎹紝涓嶄粎鏄湪
浼犵粺绫诲瀷涓?UMP MIDI 1.0/2.0 绫诲瀷涔嬮棿锛屼篃鍦?UMP MIDI 1.0 涓?2.0 绫诲瀷涔嬮棿銆備緥濡傦紝
鍦?ProtoZOA Main 绔彛涓婁互浼犵粺妯″紡杩愯 `aseqdump` 绋嬪簭灏?
```
  % aseqdump -p 20:1
  Waiting for data. Press Ctrl+C to end.
  Source  Event                  Ch  Data
   20:1   Note on                 0, note 60, velocity 100
   20:1   Note off                0, note 60, velocity 100
   20:1   Control change          0, controller 11, value 4
```

褰撲綘浠?MIDI 2.0 妯″紡杩愯 `aseqdump` 鏃讹紝瀹冨皢鎺ユ敹鍒伴珮

```
  % aseqdump -u 2 -p 20:1
  Waiting for data. Press Ctrl+C to end.
  Source  Event                  Ch  Data
   20:1   Note on                 0, note 60, velocity 0xc924, attr type = 0, data = 0x0
   20:1   Note off                0, note 60, velocity 0xc924, attr type = 0, data = 0x0
   20:1   Control change          0, controller 11, value 0x2000000
```

鑰屾暟鎹敱 ALSA 闊冲簭鍣ㄦ牳蹇冭嚜鍔ㄨ浆鎹€?

## Rawmidi API 鎵╁睍


- 鍙互閫氳繃鏂扮殑 ioctl `SNDRV_UMP_IOCTL_ENDPOINT_INFO` 鑾峰彇闄勫姞鐨?UMP Endpoint
  淇℃伅銆傚畠鍖呭惈鍏宠仈鐨勯煶鍗′笌璁惧缂栧彿銆佷綅鏍囧織銆佸崗璁€乁MP Block 鏁伴噺銆佺鐐圭殑鍚嶇О
  瀛楃涓茬瓑銆?
  鍗忚鐢?protocol capabilities锛堝崗璁兘鍔涳級涓?current protocol锛堝綋鍓嶅崗璁級涓や釜
  瀛楁鎸囧畾銆備簩鑰呴兘鍖呭惈浣嶆爣蹇楋紝鍦ㄤ笂瀛楄妭涓寚瀹?MIDI 鍗忚鐗堟湰
  锛坄SNDRV_UMP_EP_INFO_PROTO_MIDI1` 鎴?`SNDRV_UMP_EP_INFO_PROTO_MIDI2`锛夛紝
  鍦ㄤ笅瀛楄妭涓寚瀹氭姈鍔ㄦ秷闄ゆ椂闂存埑锛坄SNDRV_UMP_EP_INFO_PROTO_JRTS_TX` 涓?  `SNDRV_UMP_EP_INFO_PROTO_JRTS_RX`锛夈€?
  涓€涓?UMP Endpoint 鏈€澶氬彲鍖呭惈 32 涓?UMP Block锛屽綋鍓嶅凡鍒嗛厤鍧楃殑鏁伴噺鏄剧ず鍦?Endpoint
  淇℃伅涓€?
- 姣忎釜 UMP Block 鐨勪俊鎭彲浠ラ€氳繃鍙︿竴涓柊鐨?ioctl `SNDRV_UMP_IOCTL_BLOCK_INFO`
  鑾峰彇銆傚繀椤讳紶鍏ヨ鏌ヨ鐨勫潡鐨勫潡 ID 鍙凤紙浠?0 寮€濮嬶級銆傛帴鏀跺埌鐨勬暟鎹寘鍚鍧楃殑鍏宠仈
  鏂瑰悜銆佺涓€涓叧鑱旂粍 ID锛堜粠 0 寮€濮嬶級涓庣粍鏁伴噺銆佸潡鐨勫悕绉板瓧绗︿覆绛夈€?
  鏂瑰悜涓?`SNDRV_UMP_DIR_INPUT`銆乣SNDRV_UMP_DIR_OUTPUT` 鎴?  `SNDRV_UMP_DIR_BIDIRECTION` 涔嬩竴銆?
- 瀵逛簬鏀寔 UMP v1.1 鐨勮澶囷紝鍙互閫氳繃 鈥淪tream Configuration Request鈥?娑堟伅
  锛圲MP 绫诲瀷 0x0f锛岀姸鎬佺爜 0x05锛夊垏鎹?UMP MIDI 鍗忚銆傚綋 UMP 鏍稿績鏀跺埌杩欐牱鐨勬秷鎭椂锛?  瀹冧細鐩稿簲鍦版洿鏂?UMP EP 淇℃伅浠ュ強鐩稿簲鐨勯煶搴忓櫒瀹㈡埛绔€?
- 浼犵粺 rawmidi 璁惧缂栧彿鍙互鍦?rawmidi 淇℃伅鐨勬柊瀛楁 `tied_device` 涓壘鍒般€?  鍙︿竴鏂归潰锛孶MP rawmidi 璁惧缂栧彿涔熷彲浠ュ湪浼犵粺 rawmidi 淇℃伅鐨?`tied_device` 瀛楁
  涓壘鍒般€?
- 浼犵粺 rawmidi 鐨勬瘡涓瓙娴佸彲浠ユ牴鎹?UMP FB 鐘舵€佸姩鎬佸惎鐢?绂佺敤銆?  褰撴墍閫夊瓙娴佷笉娲诲姩鏃讹紝浼氶€氳繃浼犵粺 rawmidi 淇℃伅 `flags` 瀛楁涓殑浣?0x10
  锛坄SNDRV_RAWMIDI_INFO_STREAM_INACTIVE`锛夋潵鎸囩ず銆?

## Control API 鎵╁睍


- 寮曞叆浜嗘柊鐨?ioctl `SNDRV_CTL_IOCTL_UMP_NEXT_DEVICE` 鐢ㄤ簬鏌ヨ涓嬩竴涓?UMP rawmidi
  璁惧锛岃€岀幇鏈夌殑 ioctl `SNDRV_CTL_IOCTL_RAWMIDI_NEXT_DEVICE` 鍙煡璇紶缁?rawmidi
  璁惧銆?
  瑕佽缃鎵撳紑鐨勫瓙璁惧锛堝瓙娴佺紪鍙凤級锛岃鍍忔櫘閫?rawmidi 涓€鏍蜂娇鐢?ioctl
  `SNDRV_CTL_IOCTL_RAWMIDI_PREFER_SUBDEVICE`銆?
- 涓や釜鏂扮殑 ioctl `SNDRV_CTL_IOCTL_UMP_ENDPOINT_INFO` 涓?  `SNDRV_CTL_IOCTL_UMP_BLOCK_INFO` 閫氳繃 ALSA control API 鎻愪緵鎸囧畾 UMP 璁惧鐨?  UMP Endpoint 涓?UMP Block 淇℃伅锛岃€屾棤闇€鎵撳紑瀹為檯鐨勶紙UMP锛塺awmidi 璁惧銆?  鏌ヨ鏃跺拷鐣?`card` 瀛楁锛屽缁堜笌 control 鎺ュ彛鎵€鍦ㄧ殑闊冲崱缁戝畾銆?

## Sequencer API 鎵╁睍


- 鍚?`snd_seq_client_info` 娣诲姞浜?`midi_version` 瀛楁锛岀敤浜庢寚绀烘瘡涓鎴风鐨?  褰撳墠 MIDI 鐗堟湰锛?銆? 鎴?2锛夈€傚綋 `midi_version` 涓?1 鎴?2 鏃讹紝浠?UMP 闊冲簭鍣?  瀹㈡埛绔鍙栫殑瀵归綈鏂瑰紡涔熶粠鍘熸潵鐨?28 瀛楄妭鏀逛负 32 瀛楄妭锛屼互閫傚簲鎵╁睍璐熻浇銆傚啓鍏ョ殑
  瀵归綈澶у皬鏈敼鍙橈紝浣嗘瘡涓簨浠剁殑澶у皬鍙兘鍥犱笅闈㈢殑鏂颁綅鏍囧織鑰屼笉鍚屻€?
- 涓烘瘡涓煶搴忓櫒浜嬩欢鏍囧織娣诲姞浜?`SNDRV_SEQ_EVENT_UMP` 鏍囧織浣嶃€傚綋璁剧疆璇ヤ綅鏍囧織鏃讹紝
  闊冲簭鍣ㄤ簨浠惰鎵╁睍涓烘嫢鏈夋洿澶х殑 16 瀛楄妭璐熻浇锛堝彇浠ｄ紶缁熺殑 12 瀛楄妭锛夛紝骞朵笖浜嬩欢鍦ㄨ礋杞戒腑
  鍖呭惈 UMP 鏁版嵁鍖呫€?
- 鏂扮殑闊冲簭鍣ㄧ鍙ｇ被鍨嬩綅锛坄SNDRV_SEQ_PORT_TYPE_MIDI_UMP`锛夎〃绀鸿绔彛鏀寔 UMP銆?
- 闊冲簭鍣ㄧ鍙ｆ嫢鏈夋柊鐨勮兘鍔涗綅浠ユ寚绀轰笉娲诲姩绔彛锛坄SNDRV_SEQ_PORT_CAP_INACTIVE`锛変笌
  UMP Endpoint 绔彛锛坄SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT`锛夈€?
- 鍙互閫氳繃璁剧疆鍒板鎴风淇℃伅鐨勬柊鐨勮繃婊や綅 `SNDRV_SEQ_FILTER_NO_CONVERT` 鏉ユ姂鍒?  ALSA 闊冲簭鍣ㄥ鎴风鐨勪簨浠惰浆鎹€備緥濡傦紝鍐呮牳閫忎紶瀹㈡埛绔紙`snd-seq-dummy`锛変細鍦ㄥ唴閮?  璁剧疆姝ゆ爣蹇椼€?
- 绔彛淇℃伅鑾峰緱浜嗘柊瀛楁 `direction`锛岀敤浜庢寚绀虹鍙ｇ殑鏂瑰悜锛堜负
  `SNDRV_SEQ_PORT_DIR_INPUT`銆乣SNDRV_SEQ_PORT_DIR_OUTPUT` 鎴?  `SNDRV_SEQ_PORT_DIR_BIDIRECTION` 涔嬩竴锛夈€?
- 绔彛淇℃伅鐨勫彟涓€涓檮鍔犲瓧娈垫槸 `ump_group`锛屽畠鎸囧畾鍏宠仈鐨?UMP 缁勭紪鍙凤紙浠?1 寮€濮嬶級銆?  褰撳畠闈為浂鏃讹紝UMP 鏁版嵁鍖呬腑鐨?UMP 缁勫瓧娈典細鍦ㄦ姇閫掑埌鎸囧畾缁勬椂鏇存柊锛堜慨姝ｄ负浠?0 寮€濮嬶級銆?  姣忎釜闊冲簭鍣ㄧ鍙ｅ鏋滄槸涓€涓壒瀹氫簬鏌愪釜 UMP 缁勭殑绔彛锛屽簲褰撹缃瀛楁銆?
- 姣忎釜瀹㈡埛绔彲浠ュ湪 `group_filter` 浣嶅浘涓负 UMP 缁勮缃檮鍔犵殑浜嬩欢杩囨护鍣ㄣ€傝杩囨护鍣?  鐢变粠 1 寮€濮嬭鏁扮殑缁勭紪鍙风粍鎴愮殑浣嶅浘銆備緥濡傦紝褰撹缃綅 1 鏃讹紝鏉ヨ嚜缁?1锛堝嵆绗竴涓粍锛?  鐨勬秷鎭細琚繃婊よ€屼笉琚姇閫掋€備綅 0 鐢ㄤ簬杩囨护鏃犵粍鐨?UMP 娑堟伅銆?
- 涓烘敮鎸?UMP 鐨勫鎴风鏂板浜嗕袱涓?ioctl锛?  `SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO` 涓?  `SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO`銆傚畠浠敤浜庤幏鍙栧拰璁剧疆涓庨煶搴忓櫒瀹㈡埛绔叧鑱旂殑
  `snd_ump_endpoint_info` 鎴?`snd_ump_block_info` 鏁版嵁銆俇SB MIDI 椹卞姩浠庡簳灞傜殑 UMP
  rawmidi 鎻愪緵杩欎簺淇℃伅锛岃€岀敤鎴风┖闂村鎴风鍙互閫氳繃 `*_SET` ioctl 鎻愪緵鍏惰嚜韬殑鏁版嵁銆?  瀵逛簬 Endpoint 鏁版嵁锛屽悜 `type` 瀛楁浼犲叆 0锛涘浜?Block 鏁版嵁锛屽悜 `type` 瀛楁浼犲叆
  鍧楀彿 + 1銆?  涓哄唴鏍稿鎴风璁剧疆鏁版嵁灏嗗鑷撮敊璇€?
- 鍦?UMP 1.1 涓嬶紝Function Block 淇℃伅鍙兘浼氬姩鎬佹敼鍙樸€傚綋浠庤澶囨敹鍒?Function Block
  鐨勬洿鏂版椂锛孉LSA 闊冲簭鍣ㄦ牳蹇冧細鐩稿簲鍦版洿鏀圭浉搴旂殑闊冲簭鍣ㄧ鍙ｅ悕绉颁笌灞炴€э紝骞跺儚鏅€氱殑绔彛
  鍙樻洿閫氱煡涓€鏍凤紝閫氳繃鍚?ALSA 闊冲簭鍣ㄧ郴缁熺鍙ｇ殑鍏憡鏉ラ€氱煡杩欎簺鍙樻洿銆?
- 鏈変袱涓墿灞曚簨浠剁被鍨嬬敤浜庨€氳繃绯荤粺鍏憡绔彛閫氱煡 UMP Endpoint 涓?Function Block 鐨?  鍙樻洿锛氱被鍨?68锛坄SNDRV_SEQ_EVENT_UMP_EP_CHANGE`锛変笌绫诲瀷 69
  锛坄SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE`锛夈€傚畠浠湪璐熻浇涓噰鐢ㄦ柊绫诲瀷
  `snd_seq_ev_ump_notify`锛屾寚绀哄彂鐢熷彉鏇寸殑瀹㈡埛绔紪鍙蜂笌 FB 缂栧彿銆?

## MIDI2 USB 澶嶅悎璁惧鍔熻兘椹卞姩


鏈€鏂扮殑鍐呮牳鍖呭惈瀵?USB MIDI 2.0 澶嶅悎璁惧鍔熻兘椹卞姩鐨勬敮鎸侊紝瀹冨彲鐢ㄤ簬 MIDI 2.0 鐗规€х殑
鍘熷瀷璁捐涓庤皟璇曘€?
闇€瑕佸惎鐢?`CONFIG_USB_GADGET`銆乣CONFIG_USB_CONFIGFS` 涓?`CONFIG_USB_CONFIGFS_F_MIDI2` 鎵嶈兘浣跨敤璇?MIDI2 澶嶅悎璁惧椹卞姩銆?
姝ゅ锛岃浣跨敤澶嶅悎璁惧椹卞姩锛屼綘闇€瑕佷竴涓彲鐢ㄧ殑 UDC 椹卞姩銆傚湪涓嬮潰鐨勭ず渚嬩腑锛屾垜浠娇鐢?`dummy_hcd` 椹卞姩锛堥€氳繃 `CONFIG_USB_DUMMY_HCD` 鍚敤锛夛紝瀹冨湪 PC 涓?VM 涓婂彲鐢ㄤ簬
璋冭瘯鐩殑銆傛牴鎹钩鍙颁笉鍚岃繕鏈夊叾浠?UDC 椹卞姩锛屽畠浠篃鍙互鐢ㄤ簬鐪熷疄璁惧銆?
```
  % modprobe libcomposite
```

鐒跺悗浣犱細鍦?configfs 绌洪棿涓嬫嫢鏈?`usb_gadget` 瀛愮洰褰曪紙鍦ㄧ幇浠ｆ搷浣滅郴缁熶笂閫氬父涓?`/sys/kernel/config`锛夈€傛帴鐫€鍒涘缓涓€涓鍚堣澶?
```
  % cd /sys/kernel/config
  % mkdir usb_gadget/g1

  % cd usb_gadget/g1
  % mkdir configs/c.1
  % mkdir functions/midi2.usb0

  % echo 0x0004 > idProduct
  % echo 0x17b3 > idVendor
  % mkdir strings/0x409
  % echo "ACME Enterprises" > strings/0x409/manufacturer
  % echo "ACMESynth" > strings/0x409/product
  % echo "ABCD12345" > strings/0x409/serialnumber

  % mkdir configs/c.1/strings/0x409
  % echo "Monosynth" > configs/c.1/strings/0x409/configuration
  % echo 120 > configs/c.1/MaxPower
```

姝ゆ椂蹇呴』瀛樺湪涓€涓瓙鐩綍 `ep.0`锛屽畠灏辨槸涓€涓?UMP Endpoint 鐨勯厤缃€備綘鍙互濉啓璇?Endpoint

```
  % echo "ACMESynth" > functions/midi2.usb0/iface_name
  % echo "ACMESynth" > functions/midi2.usb0/ep.0/ep_name
  % echo "ABCD12345" > functions/midi2.usb0/ep.0/product_id
  % echo 0x0123 > functions/midi2.usb0/ep.0/family
  % echo 0x4567 > functions/midi2.usb0/ep.0/model
  % echo 0x123456 > functions/midi2.usb0/ep.0/manufacturer
  % echo 0x12345678 > functions/midi2.usb0/ep.0/sw_revision
```

```
  % echo 2 > functions/midi2.usb0/ep.0/protocol
```

骞朵笖锛屼綘鍙互鍦ㄦ Endpoint 涓嬫壘鍒颁竴涓瓙鐩綍 `block.0`

```
  % echo "Monosynth" > functions/midi2.usb0/ep.0/block.0/name
  % echo 0 > functions/midi2.usb0/ep.0/block.0/first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.0/num_groups
```

```
  % ln -s functions/midi2.usb0 configs/c.1
  % echo dummy_udc.0 > UDC
```

鍏朵腑 `dummy_udc.0` 鏄竴涓ず渚嬫儏鍐碉紝浼氬洜绯荤粺鑰屽紓銆備綘鍙互鍦?`/sys/class/udc` 涓?鎵惧埌 UDC 瀹炰緥骞朵紶鍏?
```
  % ls /sys/class/udc
  dummy_udc.0
```

鐜板湪锛孧IDI 2.0 澶嶅悎璁惧宸插惎鐢紝澶嶅悎璁惧涓绘満浼氬垱寤轰竴涓寘鍚?UMP rawmidi 璁惧鐨勬柊
澹板崱瀹炰緥

```
  % cat /proc/asound/cards
  ....
  1 [Gadget         ]: f_midi2 - MIDI 2.0 Gadget
                       MIDI 2.0 Gadget
```

鑰屽湪鎵€杩炴帴鐨勪富鏈轰笂锛屼篃搴旇浼氬嚭鐜颁竴寮犵被浼肩殑鍗★紝浣嗗甫鏈?
```
  % cat /proc/asound/cards
  ....
  2 [ACMESynth      ]: USB-Audio - ACMESynth
                       ACME Enterprises ACMESynth at usb-dummy_hcd.0-1, high speed
```

```
  % aplaymidi -p 20:1 to_host.mid
```

鑰岃繖浼氬嚭鐜板湪宸茶繛鎺ヤ富鏈轰笂浣滀负涓€涓潵鑷?MIDI 璁惧鐨勮緭鍏?
```
  % aseqdump -p 20:0 -u 2
```

鍙嶄箣浜︾劧锛屽湪宸茶繛鎺ヤ富鏈轰笂鐨勫洖鏀句篃浼氫綔涓哄鍚堣澶囦笂鐨勮緭鍏ュ伐浣溿€?
姣忎釜 Function Block 鍙互鏈変笉鍚岀殑鏂瑰悜涓?UI 鎻愮ず锛圲I-hint锛夛紝閫氳繃 `direction` 涓?`ui_hint` 灞炴€ф寚瀹氥€備紶鍏?`1` 琛ㄧず浠呰緭鍏ワ紝`2` 琛ㄧず浠呰緭鍑猴紝`3` 琛ㄧず

```
  % echo 2 > functions/midi2.usb0/ep.0/block.0/direction
  % echo 2 > functions/midi2.usb0/ep.0/block.0/ui_hint
```

褰撲綘闇€瑕佸浜庝竴涓?Function Block 鏃讹紝鍙互鍔ㄦ€佸垱寤哄瓙鐩綍 `block.1`銆乣block.2` 绛夛紝
骞跺湪涓婇潰閾炬帴涔嬪墠鐨勯厤缃楠や腑閰嶇疆瀹冧滑銆?
```
  % mkdir functions/midi2.usb0/ep.0/block.1
  % echo "Keyboard" > functions/midi2.usb0/ep.0/block.1/name
  % echo 1 > functions/midi2.usb0/ep.0/block.1/first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.1/num_groups
  % echo 1 > functions/midi2.usb0/ep.0/block.1/direction
  % echo 1 > functions/midi2.usb0/ep.0/block.1/ui_hint
```

`block.*` 瀛愮洰褰曚篃鍙互鍔ㄦ€佺Щ闄わ紙闄や簡鎸佷箙瀛樺湪鐨?`block.0`锛夈€?
瑕佷负 MIDI 1.0 I/O 鍒嗛厤涓€涓?Function Block锛岃鍦?`is_midi1` 灞炴€т腑璁剧疆銆? 琛ㄧず
MIDI 1.0锛? 琛ㄧず浣庨€熺巼鐨?MIDI 1.0

```
  % echo 2 > functions/midi2.usb0/ep.0/block.1/is_midi1
```

瑕佺鐢ㄥ鍚堣澶囦腑瀵?UMP Stream 娑堟伅鐨勫鐞?
```
  % echo 0 > functions/midi2.usb0/process_ump
```

澶嶅悎璁惧椹卞姩涔熸敮鎸佷綅浜?altset 0 鐨?MIDI 1.0 鎺ュ彛銆傚綋宸茶繛鎺ヤ富鏈洪€夋嫨浜?MIDI 1.0
鎺ュ彛鏃讹紝澶嶅悎璁惧涓婄殑 UMP I/O 浼氱浉搴斿湴涓?USB MIDI 1.0 鏁版嵁鍖呯浉浜掕浆鎹紝鑰屽鍚堣澶?椹卞姩浠嶉€氳繃 UMP rawmidi 涓庣敤鎴风┖闂撮€氫俊銆?
MIDI 1.0 绔彛鐢辨瘡涓?Function Block 涓殑閰嶇疆寤虹珛銆?
```
  % echo 0 > functions/midi2.usb0/ep.0/block.0/midi1_first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.0/midi1_num_groups
```

涓婇潰鐨勯厤缃皢涓?MIDI 1.0 鎺ュ彛鍚敤缁?1锛堢储寮?0锛夈€傛敞鎰忚繖浜涚粍蹇呴』浣嶄簬涓?Function
Block 鏈韩瀹氫箟鐨勭粍涔嬩腑銆?
澶嶅悎璁惧椹卞姩涔熸敮鎸佸浜庝竴涓?UMP Endpoint銆備笌 Function Block 绫讳技锛屼綘鍙互鍒涘缓涓€涓柊鐨?瀛愮洰褰?
```
  % mkdir functions/midi2.usb0/ep.1
```

骞跺湪鍏朵腑鍒涘缓涓€涓柊鐨?Function Block銆備緥濡傦紝瑕佸垱寤?4 涓?
```
  % mkdir functions/midi2.usb0/ep.1/block.0
  % echo 4 > functions/midi2.usb0/ep.1/block.0/num_groups
```

鐜板湪锛屼綘鎬诲叡浼氭湁 4 涓?rawmidi 璁惧锛氬墠涓や釜鏄?Endpoint 0 涓?Endpoint 1 鐨?UMP
rawmidi 璁惧锛屽彟澶栦袱涓槸瀵瑰簲鐨?EP 0 涓?EP 1 鐨勪紶缁?MIDI 1.0 rawmidi 璁惧銆?
澶嶅悎璁惧涓婄殑褰撳墠 altsetting 鍙互閫氳繃涓€涓甫鏈?`RAWMIDI` iface 鐨勫悕涓?鈥淥peration Mode鈥?锛堟搷浣滄ā寮忥級鐨勬帶鍒跺厓绱犳潵鍛婄煡銆備緥濡傦紝浣犲彲浠ヨ鍙栧畠

```
  % amixer -c1 cget iface=RAWMIDI,name='Operation Mode'
  ; type=INTEGER,access=r--v----,values=1,min=0,max=2,step=0
  : values=2
```

璇ュ€硷紙鍦ㄧ浜岃杩斿洖鍐呭涓互 `: values=` 鏄剧ず锛夎〃绀猴細1 涓?MIDI 1.0锛坅ltset 0锛夛紝
2 涓?MIDI 2.0锛坅ltset 1锛夛紝0 涓烘湭璁剧疆銆?
鎴嚦鐩墠锛岀粦瀹氫箣鍚庢棤娉曟洿鏀归厤缃€?