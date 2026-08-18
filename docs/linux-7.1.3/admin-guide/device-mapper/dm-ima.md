## dm-ima


瀵逛簬缁欏畾绯荤粺锛屽悇绉嶅閮ㄦ湇鍔?鍩虹璁炬柦宸ュ叿锛堝寘鎷瘉鏄庢湇鍔★級浼氫笌涔嬩氦浜掆€斺€旀棦鍦ㄥ垵濮嬪寲璁剧疆鏈熼棿锛屼篃鍦ㄧ郴缁熷叾浣欒繍琛屾椂娈点€傚畠浠叡浜晱鎰熸暟鎹紝鍜?鎴栧湪璇ョ郴缁熶笂鎵ц鍏抽敭宸ヤ綔璐熻浇銆傚湪灏嗕笟鍔″叧閿瀷鏁版嵁/宸ヤ綔璐熻浇瀹屽叏鎵樹粯缁欒绯荤粺涔嬪墠锛屽閮ㄦ湇鍔″彲鑳藉笇鏈涘厛楠岃瘉鐩稿叧鍐呮牳瀛愮郴缁熺殑褰撳墠杩愯鏃剁姸鎬併€?
璁惧鏄犲皠鍣紙Device mapper锛夐€氳繃鍦ㄥ潡璁惧涓婁娇鐢?crypt銆乿erity銆乮ntegrity 绛夊绉嶇洰鏍囩被鍨嬶紝涓哄潡璁惧鎻愪緵鍚勭閲嶈鍔熻兘锛屼粠鑰屽湪鐗瑰畾绯荤粺涓婂彂鎸ョ潃鍏抽敭浣滅敤銆傝繖浜涚洰鏍囩被鍨嬪悇鑷殑鍔熻兘閮藉彲浠ラ€氳繃鍚勭灞炴€ц繘琛岄厤缃€傜敤浜庨厤缃繖浜涚洰鏍囩被鍨嬬殑灞炴€ч€夋嫨锛屼細鏄捐憲褰卞搷鍧楄澶囦箖鑷虫暣涓郴缁熺殑瀹夊叏閰嶇疆銆備緥濡傦紝鍔犲瘑绠楁硶鐨勭被鍨嬪拰瀵嗛挜闀垮害鍐冲畾浜嗙粰瀹氬潡璁惧鐨勫姞瀵嗗己搴︺€?
鍥犳锛屽湪灏嗕笟鍔″叧閿瀷鏁版嵁/宸ヤ綔璐熻浇瀹屽叏鎵樹粯缁欒绯荤粺涔嬪墠锛屽閮ㄦ湇鍔￠獙璇佸悇绉嶅潡璁惧鐨勫綋鍓嶇姸鎬佸強鍏跺悇绉嶇洰鏍囧睘鎬ц嚦鍏抽噸瑕併€?
IMA 鍐呮牳瀛愮郴缁熶负璁惧鏄犲皠鍣ㄦ彁渚涗簡蹇呰鐨勫姛鑳斤紝鐢ㄤ簬搴﹂噺锛坢easure锛夊悇绉嶅潡璁惧鐨勭姸鎬佷笌閰嶇疆鈥斺€?
- 鐢辫澶囨槧灏勫櫒鑷韩鍦ㄥ唴鏍稿唴閮ㄥ畬鎴愶紝
- 浠ヤ竴绉嶆姉绡℃敼锛坱amper resistant锛夌殑鏂瑰紡锛?- 骞跺湪鐘舵€?閰嶇疆鍙戠敓鍙樺寲鏃堕噸鏂板害閲忋€?
## 璁剧疆 IMA 绛栫暐锛?
涓轰簡璁?IMA 搴﹂噺缁欏畾绯荤粺涓婄殑鏁版嵁锛岄渶瑕佹洿鏂拌绯荤粺涓婄殑 IMA 绛栫暐浠ュ寘鍚涓嬩竴琛屽唴瀹癸紝骞朵笖闇€瑕侀噸鍚郴缁熸墠鑳戒娇搴﹂噺鐢熸晥銆?
```

 /etc/ima/ima-policy
    measure func=CRITICAL_DATA label=device-mapper template=ima-buf

```
搴﹂噺缁撴灉浼氬弽鏄犲湪 IMA 鏃ュ織涓紝杩欎簺鏃ュ織浣嶄簬锛?
```

 /sys/kernel/security/integrity/ima/ascii_runtime_measurements
 /sys/kernel/security/integrity/ima/binary_runtime_measurements

```
鐒跺悗 IMA ASCII 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 <PCR> <TEMPLATE_DATA_DIGEST> <TEMPLATE_NAME> <TEMPLATE_DATA>

 PCR := Platform Configuration Register锛屽嵆骞冲彴閰嶇疆瀵勫瓨鍣紝鍊间細琚櫥璁板湪鍏朵腑銆?       杩欎粎鍦ㄤ娇鐢ㄤ簡 TPM 鑺墖鏃堕€傜敤銆?
 TEMPLATE_DATA_DIGEST := IMA 璁板綍鐨勬ā鏉挎暟鎹憳瑕併€? TEMPLATE_NAME := 鐧昏瀹屾暣鎬у€硷紙integrity value锛夌殑妯℃澘鍚嶇О锛堝 ima-buf锛夈€?
 TEMPLATE_DATA := <ALG> ":" <EVENT_DIGEST> <EVENT_NAME> <EVENT_DATA>
                  瀹冨寘鍚浠ョ粰瀹氭ā鏉挎暟鎹牸寮忚繘琛屽害閲忕殑鐗瑰畾浜嬩欢鏁版嵁銆?
 ALG := 鐢ㄤ簬璁＄畻浜嬩欢鎽樿鐨勭畻娉? EVENT_DIGEST := 浜嬩欢鏁版嵁鐨勬憳瑕? EVENT_NAME := 浜嬩欢鐨勬弿杩帮紙濡?'dm_table_load'锛夈€? EVENT_DATA := 瑕佽搴﹂噺鐨勪簨浠舵暟鎹€?
```
|
| **NOTE #1:**
| IMA 瀛愮郴缁熷害閲忕殑 DM 鐩爣鏁版嵁锛屼篃鍙互閫氳繃璁剧疆 DM_IMA_MEASUREMENT_FLAG 涓?DM_TABLE_STATUS_CMD锛屼粠鐢ㄦ埛绌洪棿鏌ヨ銆?|
|
| **NOTE #2:**
| 鍐呮牳閰嶇疆椤?CONFIG_IMA_DISABLE_HTABLE 鍏佽搴﹂噺閲嶅璁板綍銆?| 涓烘敮鎸佸湪 IMA 鏃ュ織涓褰曢噸澶嶇殑 IMA 浜嬩欢锛屽唴鏍搁渶瑕侀厤缃?CONFIG_IMA_DISABLE_HTABLE=y銆?
## 鏀寔鐨勮澶囩姸鎬侊細

浠ヤ笅璁惧鐘舵€佸彉鏇翠細瑙﹀彂 IMA 搴﹂噺锛?
 1. Table load
 #. Device resume
 #. Device remove
 #. Table clear
 #. Device rename

### 1. 琛ㄥ姞杞斤紙Table load锛夛細

褰撲竴涓柊鐨勮〃琚姞杞藉埌璁惧鐨勯潪娲昏穬琛ㄦЫ锛坕nactive table slot锛夋椂锛岃澶囦俊鎭互鍙婅〃涓悇鐩爣鐨勭洰鏍囩壒瀹氱粏鑺備細琚害閲忋€?
瀵逛簬 'dm_table_load'锛孖MA 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 EVENT_NAME := "dm_table_load"
 EVENT_DATA := <dm_version_str> ";" <device_metadata> ";" <table_load_data>

 dm_version_str := "dm_version=" <N> "." <N> "." <N>
                  涓庤澶囨槧灏勫櫒椹卞姩鐗堟湰鐩稿悓銆? device_metadata := <device_name> "," <device_uuid> "," <device_major> "," <device_minor> ","
                   <minor_count> "," <num_device_targets> ";"

 device_name := "name=" <dm-device-name>
 device_uuid := "uuid=" <dm-device-uuid>
 device_major := "major=" <N>
 device_minor := "minor=" <N>
 minor_count := "minor_count=" <N>
 num_device_targets := "num_targets=" <N>
 dm-device-name := 璁惧鍚嶇О銆傚鏋滃叾涓寘鍚?'\'銆?,'銆?;' 绛夌壒娈婂瓧绗︼紝
                   浼氬湪鍏跺墠闈㈠姞涓?'\' 鍓嶇紑銆? dm-device-uuid := 璁惧鐨?UUID銆傚鏋滃叾涓寘鍚?'\'銆?,'銆?;' 绛夌壒娈婂瓧绗︼紝
                   浼氬湪鍏跺墠闈㈠姞涓?'\' 鍓嶇紑銆?
 table_load_data := <target_data>
                    琛ㄧず姝ｅ湪鍔犺浇鍒?DM 璁惧闈炴椿璺冭〃妲戒腑鐨勩€佹潵鑷〃涓悇鐩爣鐨勬暟鎹?                    锛堜互 name=value 瀵圭殑褰㈠紡锛夈€? target_data := <target_data_row> | <target_data><target_data_row>

 target_data_row := <target_index> "," <target_begin> "," <target_len> "," <target_name> ","
                    <target_version> "," <target_attributes> ";"
 target_index := "target_index=" <N>
                 琛ㄧず琛ㄤ腑鐨勭 n 涓洰鏍囷紙鑼冨洿浠?<num_device_targets> 鎸囧畾鐨?0 鍒?N-1锛夈€?                 濡傛灉 N 涓洰鏍囩殑鏁版嵁鏃犳硶鍏ㄩ儴鏀惧叆缁欏畾缂撳啿鍖猴紝鍒欒兘澶熸斁鍏ョ紦鍐插尯鐨勯偅閮ㄥ垎鏁版嵁
                 锛堜緥濡備粠鐩爣 0 鍒?x锛変細鍦ㄤ竴娆＄粰瀹氱殑 IMA 浜嬩欢涓搴﹂噺銆?                 鍏朵綑浠庣洰鏍?x+1 鍒?N-1 鐨勬暟鎹細鍦ㄥ悗缁殑 IMA 浜嬩欢涓害閲忥紝
                 鍏舵牸寮忎笌 'dm_table_load' 鐩稿悓锛?                 鍗?<dm_version_str> ";" <device_metadata> ";" <table_load_data>銆?
 target_begin := "target_begin=" <N>
 target_len := "target_len=" <N>
 target_name := 鐩爣鐨勫悕绉般€?linear'銆?crypt'銆?integrity' 绛夈€?                鏀寔 IMA 搴﹂噺鐨勭洰鏍囧湪涓嬮潰鐨勨€滄敮鎸佺殑鐩爣鈥濆皬鑺備腑鏈夋枃妗ｈ鏄庛€? target_version := "target_version=" <N> "." <N> "." <N>
 target_attributes := 鍖呭惈浠ラ€楀彿鍒嗛殧鐨勩€佺洰鏍囩壒瀹氬睘鎬?name=value 瀵圭殑鏁版嵁銆?
 渚嬪锛屽鏋滀娇鐢ㄤ互涓嬭〃椤瑰垱寤轰竴涓?linear 璁惧锛?  # dmsetup create linear1
  0 2 linear /dev/loop0 512
  2 2 linear /dev/loop0 512
  4 2 linear /dev/loop0 512
  6 2 linear /dev/loop0 512

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈濡備笅鏉＄洰锛? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 a8c5ff755561c7a28146389d1514c318592af49a ima-buf sha256:4d73481ecce5eadba8ab084640d85bb9ca899af4d0a122989252a76efadc5b72
 dm_table_load
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=0,minor_count=1,num_targets=4;
 target_index=0,target_begin=0,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=1,target_begin=2,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=2,target_begin=4,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=3,target_begin=6,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;

```
### 2. 璁惧鎭㈠锛圖evice resume锛夛細

褰撹鎸傝捣鐨勮澶囨仮澶嶏紙resume锛夋椂锛岃澶囦俊鎭互鍙婁笂娆″姞杞界殑娲昏穬琛紙active table锛夋暟鎹殑鍝堝笇浼氳搴﹂噺銆?
瀵逛簬 'dm_device_resume'锛孖MA 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 EVENT_NAME := "dm_device_resume"
 EVENT_DATA := <dm_version_str> ";" <device_metadata> ";" <active_table_hash> ";" <current_device_capacity> ";"

 dm_version_str := 濡備笂闈⑩€滆〃鍔犺浇鈥濆皬鑺傛墍杩般€? device_metadata := 濡備笂闈⑩€滆〃鍔犺浇鈥濆皬鑺傛墍杩般€? active_table_hash := "active_table_hash=" <table_hash_alg> ":" <table_hash>
                      琛ㄧず姝ｅ湪搴﹂噺鐨勩€佽璁惧娲昏穬琛ㄧ殑 IMA 鏁版嵁鐨勫搱甯屻€? table_hash_alg := 鐢ㄤ簬璁＄畻鍝堝笇鐨勭畻娉曘€? table_hash := 瀵?(<dm_version_str> ";" <device_metadata> ";" <table_load_data> ";")
               鐨勫搱甯岋紝濡備笂闈㈢殑 'dm_table_load' 鎵€杩般€?               娉ㄦ剰锛氬鏋?table_load 鏁版嵁璺ㄨ秺浜嗘煇涓澶囩殑澶氫釜 IMA 'dm_table_load'
               浜嬩欢锛屽垯鍝堝笇鏄粨鍚堟墍鏈夎繖浜涗簨浠剁殑鏁版嵁
               鍗?(<dm_version_str> ";" <device_metadata> ";" <table_load_data> ";")
               璁＄畻寰楀埌鐨勩€? current_device_capacity := "current_device_capacity=" <N>

 渚嬪锛屽鏋滀娇鐢ㄤ互涓嬪懡浠ゆ仮澶嶄竴涓?linear 璁惧锛? #dmsetup resume linear1

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈涓€涓潯鐩細
 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 56c00cc062ffc24ccd9ac2d67d194af3282b934e ima-buf sha256:e7d12c03b958b4e0e53e7363a06376be88d98a1ac191fdbd3baf5e4b77f329b6
 dm_device_resume
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=0,minor_count=1,num_targets=4;
 active_table_hash=sha256:4d73481ecce5eadba8ab084640d85bb9ca899af4d0a122989252a76efadc5b72;current_device_capacity=8;

```
### 3. 璁惧绉婚櫎锛圖evice remove锛夛細

褰撹澶囪绉婚櫎鏃讹紝璁惧淇℃伅浠ュ強娲昏穬琛ㄤ笌闈炴椿璺冭〃锛坕nactive table锛夋暟鎹殑 sha256 鍝堝笇浼氳搴﹂噺銆?
瀵逛簬 'dm_device_remove'锛孖MA 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 EVENT_NAME := "dm_device_remove"
 EVENT_DATA := <dm_version_str> ";" <device_active_metadata> ";" <device_inactive_metadata> ";"
               <active_table_hash> "," <inactive_table_hash> "," <remove_all> ";" <current_device_capacity> ";"

 dm_version_str := 濡備笂闈⑩€滆〃鍔犺浇鈥濆皬鑺傛墍杩般€? device_active_metadata := 鍙嶆槧褰撳墠宸插姞杞芥椿璺冭〃鐨勮澶囧厓鏁版嵁銆?                           鍏舵牸寮忎笌涓婇潰鈥滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'device_metadata' 鐩稿悓銆? device_inactive_metadata := 鍙嶆槧闈炴椿璺冭〃鐨勮澶囧厓鏁版嵁銆?                             鍏舵牸寮忎笌涓婇潰鈥滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'device_metadata' 鐩稿悓銆? active_table_hash := 褰撳墠宸插姞杞芥椿璺冭〃鐨勫搱甯屻€?                      鍏舵牸寮忎笌涓婇潰鈥滆澶囨仮澶嶁€濆皬鑺備腑鎻忚堪鐨?'active_table_hash' 鐩稿悓銆? inactive_table_hash :=  闈炴椿璺冭〃鐨勫搱甯屻€?                         鍏舵牸寮忎笌涓婇潰鈥滆澶囨仮澶嶁€濆皬鑺備腑鎻忚堪鐨?'active_table_hash' 鐩稿悓銆? remove_all := "remove_all=" <yes_no>
 yes_no := "y" | "n"
 current_device_capacity := "current_device_capacity=" <N>

 渚嬪锛屽鏋滀娇鐢ㄤ互涓嬪懡浠ょЩ闄や竴涓?linear 璁惧锛?  #dmsetup remove l1

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈濡備笅鏉＄洰锛? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 790e830a3a7a31590824ac0642b3b31c2d0e8b38 ima-buf sha256:ab9f3c959367a8f5d4403d6ce9c3627dadfa8f9f0e7ec7899299782388de3840
 dm_device_remove
 dm_version=4.45.0;
 device_active_metadata=name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=2;
 device_inactive_metadata=name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 active_table_hash=sha256:4a7e62efaebfc86af755831998b7db6f59b60d23c9534fb16a4455907957953a,
 inactive_table_hash=sha256:9d79c175bc2302d55a183e8f50ad4bafd60f7692fd6249e5fd213e2464384b86,remove_all=n;
 current_device_capacity=2048;

```
### 4. 琛ㄦ竻闄わ紙Table clear锛夛細

褰撻潪娲昏穬琛ㄤ粠璁惧涓竻闄ゆ椂锛岃澶囦俊鎭互鍙婅闈炴椿璺冭〃鏁版嵁鐨?sha256 鍝堝笇浼氳搴﹂噺銆?
瀵逛簬 'dm_table_clear'锛孖MA 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 EVENT_NAME := "dm_table_clear"
 EVENT_DATA := <dm_version_str> ";" <device_inactive_metadata> ";" <inactive_table_hash> ";" <current_device_capacity> ";"

 dm_version_str := 濡備笂闈⑩€滆〃鍔犺浇鈥濆皬鑺傛墍杩般€? device_inactive_metadata := 鍦ㄥ姞杞芥椂鎹曡幏鐨勩€佹琚竻闄ょ殑闈炴椿璺冭〃鐨勮澶囧厓鏁版嵁銆?                             鍏舵牸寮忎笌涓婇潰鈥滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'device_metadata' 鐩稿悓銆? inactive_table_hash := 姝ｈ浠庤澶囨竻闄ょ殑闈炴椿璺冭〃鐨勫搱甯屻€?                        鍏舵牸寮忎笌涓婇潰鈥滆澶囨仮澶嶁€濆皬鑺備腑鎻忚堪鐨?'active_table_hash' 鐩稿悓銆? current_device_capacity := "current_device_capacity=" <N>

 渚嬪锛屽鏋滀竴涓?linear 璁惧鐨勯潪娲昏穬琛ㄨ娓呴櫎锛?  #dmsetup clear l1

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈涓€涓潯鐩細
 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 77d347408f557f68f0041acb0072946bb2367fe5 ima-buf sha256:42f9ca22163fdfa548e6229dece2959bc5ce295c681644240035827ada0e1db5
 dm_table_clear
 dm_version=4.45.0;
 name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 inactive_table_hash=sha256:75c0dc347063bf474d28a9907037eba060bfe39d8847fc0646d75e149045d545;current_device_capacity=1024;

```
### 5. 璁惧閲嶅懡鍚嶏紙Device rename锛夛細

褰撹澶囩殑 NAME 鎴?UUID 琚洿鏀规椂锛岃澶囦俊鎭互鍙婃柊鐨?NAME 鍜?UUID 浼氳搴﹂噺銆?
瀵逛簬 'dm_device_rename'锛孖MA 搴﹂噺鏃ュ織鐨勬牸寮忓涓嬶細

```

 EVENT_NAME := "dm_device_rename"
 EVENT_DATA := <dm_version_str> ";" <device_active_metadata> ";" <new_device_name> "," <new_device_uuid> ";" <current_device_capacity> ";"

 dm_version_str := 濡備笂闈⑩€滆〃鍔犺浇鈥濆皬鑺傛墍杩般€? device_active_metadata := 鍙嶆槧褰撳墠宸插姞杞芥椿璺冭〃鐨勮澶囧厓鏁版嵁銆?                           鍏舵牸寮忎笌涓婇潰鈥滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'device_metadata' 鐩稿悓銆? new_device_name := "new_name=" <dm-device-name>
 dm-device-name := 涓庝笂闈⑩€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?<dm-device-name> 鐩稿悓
 new_device_uuid := "new_uuid=" <dm-device-uuid>
 dm-device-uuid := 涓庝笂闈⑩€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?<dm-device-uuid> 鐩稿悓
 current_device_capacity := "current_device_capacity=" <N>

 渚?1锛氬鏋滀娇鐢ㄤ互涓嬪懡浠ゆ洿鏀逛竴涓?linear 璁惧鐨勫悕绉帮紝
  #dmsetup rename linear1 --setuuid 1234-5678

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈涓€涓潯鐩細
 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 8b0423209b4c66ac1523f4c9848c9b51ee332f48 ima-buf sha256:6847b7258134189531db593e9230b257c84f04038b5a18fd2e1473860e0569ac
 dm_device_rename
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;new_name=linear1,new_uuid=1234-5678;
 current_device_capacity=1024;

 渚?2锛氬鏋滀娇鐢ㄤ互涓嬪懡浠ゆ洿鏀逛竴涓?linear 璁惧鐨勫悕绉帮紝
  # dmsetup rename linear1 linear=2

 閭ｄ箞 IMA ASCII 搴﹂噺鏃ュ織涓皢鍖呭惈涓€涓潯鐩細
 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 10 bef70476b99c2bdf7136fae033aa8627da1bf76f ima-buf sha256:8c6f9f53b9ef9dc8f92a2f2cca8910e622543d0f0d37d484870cb16b95111402
 dm_device_rename
 dm_version=4.45.0;
 name=linear1,uuid=1234-5678,major=253,minor=2,minor_count=1,num_targets=1;
 new_name=linear\=2,new_uuid=1234-5678;
 current_device_capacity=1024;

```
## 鏀寔鐨勭洰鏍囷紙targets锛夛細


浠ヤ笅鐩爣锛坱argets锛夋敮鎸佷娇鐢?IMA 搴﹂噺瀹冧滑鐨勬暟鎹細

 1. cache
 #. crypt
 #. integrity
 #. linear
 #. mirror
 #. multipath
 #. raid
 #. snapshot
 #. striped
 #. verity

### 1. cache

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'cache' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <metadata_mode> "," <cache_metadata_device> ","
                      <cache_device> "," <cache_origin_device> "," <writethrough> "," <writeback> ","
                      <passthrough> "," <no_discard_passdown> ";"

 target_name := "target_name=cache"
 target_version := "target_version=" <N> "." <N> "." <N>
 metadata_mode := "metadata_mode=" <cache_metadata_mode>
 cache_metadata_mode := "fail" | "ro" | "rw"
 cache_device := "cache_device=" <cache_device_name_string>
 cache_origin_device := "cache_origin_device=" <cache_origin_device_string>
 writethrough := "writethrough=" <yes_no>
 writeback := "writeback=" <yes_no>
 passthrough := "passthrough=" <yes_no>
 no_discard_passdown := "no_discard_passdown=" <yes_no>
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'cache' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'cache' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;name=cache1,uuid=cache_uuid,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=28672,target_name=cache,target_version=2.2.0,metadata_mode=rw,
 cache_metadata_device=253:4,cache_device=253:3,cache_origin_device=253:5,writethrough=y,writeback=n,
 passthrough=n,metadata2=y,no_discard_passdown=n;


```
### 2. crypt

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'crypt' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <allow_discards> "," <same_cpu_crypt> ","
                      <submit_from_crypt_cpus> "," <no_read_workqueue> "," <no_write_workqueue> ","
                      <iv_large_sectors> "," <iv_large_sectors> "," [<integrity_tag_size> ","] [<cipher_auth> ","]
                      [<sector_size> ","] [<cipher_string> ","] <key_size> "," <key_parts> ","
                      <key_extra_size> "," <key_mac_size> ";"

 target_name := "target_name=crypt"
 target_version := "target_version=" <N> "." <N> "." <N>
 allow_discards := "allow_discards=" <yes_no>
 same_cpu_crypt := "same_cpu_crypt=" <yes_no>
 submit_from_crypt_cpus := "submit_from_crypt_cpus=" <yes_no>
 no_read_workqueue := "no_read_workqueue=" <yes_no>
 no_write_workqueue := "no_write_workqueue=" <yes_no>
 iv_large_sectors := "iv_large_sectors=" <yes_no>
 integrity_tag_size := "integrity_tag_size=" <N>
 cipher_auth := "cipher_auth=" <string>
 sector_size := "sector_size="  <N>
 cipher_string := "cipher_string="
 key_size := "key_size="  <N>
 key_parts := "key_parts="  <N>
 key_extra_size := "key_extra_size="  <N>
 key_mac_size := "key_mac_size="  <N>
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'crypt' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'crypt' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=crypt1,uuid=crypt_uuid1,major=253,minor=0,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=1953125,target_name=crypt,target_version=1.23.0,
 allow_discards=y,same_cpu=n,submit_from_crypt_cpus=n,no_read_workqueue=n,no_write_workqueue=n,
 iv_large_sectors=n,cipher_string=aes-xts-plain64,key_size=32,key_parts=1,key_extra_size=0,key_mac_size=0;

```
### 3. integrity

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'integrity' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <dev_name> "," <start>
                      <tag_size> "," <mode> "," [<meta_device> ","] [<block_size> ","] <recalculate> ","
                      <allow_discards> "," <fix_padding> "," <fix_hmac> "," <legacy_recalculate> ","
                      <journal_sectors> "," <interleave_sectors> "," <buffer_sectors> ";"

 target_name := "target_name=integrity"
 target_version := "target_version=" <N> "." <N> "." <N>
 dev_name := "dev_name=" <device_name_str>
 start := "start=" <N>
 tag_size := "tag_size=" <N>
 mode := "mode=" <integrity_mode_str>
 integrity_mode_str := "J" | "B" | "D" | "R"
 meta_device := "meta_device=" <meta_device_str>
 block_size := "block_size=" <N>
 recalculate := "recalculate=" <yes_no>
 allow_discards := "allow_discards=" <yes_no>
 fix_padding := "fix_padding=" <yes_no>
 fix_hmac := "fix_hmac=" <yes_no>
 legacy_recalculate := "legacy_recalculate=" <yes_no>
 journal_sectors := "journal_sectors=" <N>
 interleave_sectors := "interleave_sectors=" <N>
 buffer_sectors := "buffer_sectors=" <N>
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'integrity' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'integrity' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=integrity1,uuid=,major=253,minor=1,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=7856,target_name=integrity,target_version=1.10.0,
 dev_name=253:0,start=0,tag_size=32,mode=J,recalculate=n,allow_discards=n,fix_padding=n,
 fix_hmac=n,legacy_recalculate=n,journal_sectors=88,interleave_sectors=32768,buffer_sectors=128;


```
### 4. linear

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'linear' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <device_name> <,> <start> ";"

 target_name := "target_name=linear"
 target_version := "target_version=" <N> "." <N> "." <N>
 device_name := "device_name=" <linear_device_name_str>
 start := "start=" <N>

 渚嬪锛? 褰撳姞杞戒竴涓?'linear' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'linear' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=linear1,uuid=linear_uuid1,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=28672,target_name=linear,target_version=1.4.0,
 device_name=253:1,start=2048;

```
### 5. mirror

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'mirror' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <nr_mirrors> ","
                      <mirror_device_data> "," <handle_errors> "," <keep_log> "," <log_type_status> ";"

 target_name := "target_name=mirror"
 target_version := "target_version=" <N> "." <N> "." <N>
 nr_mirrors := "nr_mirrors=" <NR>
 mirror_device_data := <mirror_device_row> | <mirror_device_data><mirror_device_row>
                       mirror_device_row 浼氶噸澶?<NR> 娆♀€斺€斿搴?<nr_mirrors> 涓弿杩扮殑 <NR>銆? mirror_device_row := <mirror_device_name> "," <mirror_device_status>
 mirror_device_name := "mirror_device_" <X> "=" <mirror_device_name_str>
                       鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NR> -1)鈥斺€斿搴?<nr_mirrors> 涓弿杩扮殑 <NR>銆? mirror_device_status := "mirror_device_" <X> "_status=" <mirror_device_status_char>
                         鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NR> -1)鈥斺€斿搴?<nr_mirrors> 涓弿杩扮殑 <NR>銆? mirror_device_status_char := "A" | "F" | "D" | "S" | "R" | "U"
 handle_errors := "handle_errors=" <yes_no>
 keep_log := "keep_log=" <yes_no>
 log_type_status := "log_type_status=" <log_type_status_str>
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'mirror' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'mirror' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=mirror1,uuid=mirror_uuid1,major=253,minor=6,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2048,target_name=mirror,target_version=1.14.0,nr_mirrors=2,
    mirror_device_0=253:4,mirror_device_0_status=A,
    mirror_device_1=253:5,mirror_device_1_status=A,
 handle_errors=y,keep_log=n,log_type_status=;

```
### 6. multipath

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'multipath' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <nr_priority_groups>
                      ["," <pg_state> "," <priority_groups> "," <priority_group_paths>] ";"

 target_name := "target_name=multipath"
 target_version := "target_version=" <N> "." <N> "." <N>
 nr_priority_groups := "nr_priority_groups=" <NPG>
 priority_groups := <priority_groups_row>|<priority_groups_row><priority_groups>
 priority_groups_row := "pg_state_" <X> "=" <pg_state_str> "," "nr_pgpaths_" <X>  "=" <NPGP> ","
                        "path_selector_name_" <X> "=" <string> "," <priority_group_paths>
                        鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NPG> -1)鈥斺€斿搴?<nr_priority_groups> 涓弿杩扮殑 <NPG>銆? pg_state_str := "E" | "A" | "D"
 <priority_group_paths> := <priority_group_paths_row> | <priority_group_paths_row><priority_group_paths>
 priority_group_paths_row := "path_name_" <X> "_" <Y> "=" <string> "," "is_active_" <X> "_" <Y> "=" <is_active_str>
                             "fail_count_" <X> "_" <Y> "=" <N> "," "path_selector_status_" <X> "_" <Y> "=" <path_selector_status_str>
                             鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NPG> -1)鈥斺€斿搴?<nr_priority_groups> 涓弿杩扮殑 <NPG>锛?                             鑰?<Y> 鐨勮寖鍥翠粠 0 鍒?(<NPGP> -1)鈥斺€斿搴?<priority_groups_row> 涓弿杩扮殑 <NPGP>銆? is_active_str := "A" | "F"

 渚嬪锛? 褰撳姞杞戒竴涓?'multipath' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'multipath' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=mp,uuid=,major=253,minor=0,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2097152,target_name=multipath,target_version=1.14.0,nr_priority_groups=2,
    pg_state_0=E,nr_pgpaths_0=2,path_selector_name_0=queue-length,
        path_name_0_0=8:16,is_active_0_0=A,fail_count_0_0=0,path_selector_status_0_0=,
        path_name_0_1=8:32,is_active_0_1=A,fail_count_0_1=0,path_selector_status_0_1=,
    pg_state_1=E,nr_pgpaths_1=2,path_selector_name_1=queue-length,
        path_name_1_0=8:48,is_active_1_0=A,fail_count_1_0=0,path_selector_status_1_0=,
        path_name_1_1=8:64,is_active_1_1=A,fail_count_1_1=0,path_selector_status_1_1=;

```
### 7. raid

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'raid' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <raid_type> "," <raid_disks> "," <raid_state>
                      <raid_device_status> ["," journal_dev_mode] ";"

 target_name := "target_name=raid"
 target_version := "target_version=" <N> "." <N> "." <N>
 raid_type := "raid_type=" <raid_type_str>
 raid_disks := "raid_disks=" <NRD>
 raid_state := "raid_state=" <raid_state_str>
 raid_state_str := "frozen" | "reshape" |"resync" | "check" | "repair" | "recover" | "idle" |"undef"
 raid_device_status := <raid_device_status_row> | <raid_device_status_row><raid_device_status>
                       <raid_device_status_row> 浼氶噸澶?<NRD> 娆♀€斺€斿搴?<raid_disks> 涓弿杩扮殑 <NRD>銆? raid_device_status_row := "raid_device_" <X> "_status=" <raid_device_status_str>
                           鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NRD> -1)鈥斺€斿搴?<raid_disks> 涓弿杩扮殑 <NRD>銆? raid_device_status_str := "A" | "D" | "a" | "-"
 journal_dev_mode := "journal_dev_mode=" <journal_dev_mode_str>
 journal_dev_mode_str := "writethrough" | "writeback" | "invalid"

 渚嬪锛? 褰撳姞杞戒竴涓?'raid' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'raid' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=raid_LV1,uuid=uuid_raid_LV1,major=253,minor=12,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2048,target_name=raid,target_version=1.15.1,
 raid_type=raid10,raid_disks=4,raid_state=idle,
    raid_device_0_status=A,
    raid_device_1_status=A,
    raid_device_2_status=A,
    raid_device_3_status=A;


```
### 8. snapshot

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'snapshot' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <snap_origin_name> ","
                      <snap_cow_name> "," <snap_valid> "," <snap_merge_failed> "," <snapshot_overflowed> ";"

 target_name := "target_name=snapshot"
 target_version := "target_version=" <N> "." <N> "." <N>
 snap_origin_name := "snap_origin_name=" <string>
 snap_cow_name := "snap_cow_name=" <string>
 snap_valid := "snap_valid=" <yes_no>
 snap_merge_failed := "snap_merge_failed=" <yes_no>
 snapshot_overflowed := "snapshot_overflowed=" <yes_no>
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'snapshot' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'snapshot' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=snap1,uuid=snap_uuid1,major=253,minor=13,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=4096,target_name=snapshot,target_version=1.16.0,
 snap_origin_name=253:11,snap_cow_name=253:12,snap_valid=y,snap_merge_failed=n,snapshot_overflowed=n;

```
### 9. striped

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'striped' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <stripes> "," <chunk_size> ","
                      <stripe_data> ";"

 target_name := "target_name=striped"
 target_version := "target_version=" <N> "." <N> "." <N>
 stripes := "stripes=" <NS>
 chunk_size := "chunk_size=" <N>
 stripe_data := <stripe_data_row>|<stripe_data><stripe_data_row>
 stripe_data_row := <stripe_device_name> "," <stripe_physical_start> "," <stripe_status>
 stripe_device_name := "stripe_" <X> "_device_name=" <stripe_device_name_str>
                       鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NS> -1)鈥斺€斿搴?<stripes> 涓弿杩扮殑 <NS>銆? stripe_physical_start := "stripe_" <X> "_physical_start=" <N>
                          鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NS> -1)鈥斺€斿搴?<stripes> 涓弿杩扮殑 <NS>銆? stripe_status := "stripe_" <X> "_status=" <stripe_status_str>
                  鍏朵腑 <X> 鐨勮寖鍥翠粠 0 鍒?(<NS> -1)鈥斺€斿搴?<stripes> 涓弿杩扮殑 <NS>銆? stripe_status_str := "D" | "A"

 渚嬪锛? 褰撳姞杞戒竴涓?'striped' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'striped' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=striped1,uuid=striped_uuid1,major=253,minor=5,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=640,target_name=striped,target_version=1.6.0,stripes=2,chunk_size=64,
    stripe_0_device_name=253:0,stripe_0_physical_start=2048,stripe_0_status=A,
    stripe_1_device_name=253:3,stripe_1_physical_start=2048,stripe_1_status=A;

```
### 10. verity

浣滀负 EVENT_DATA 涓€閮ㄥ垎鍦ㄤ笂杩扳€滆〃鍔犺浇鈥濆皬鑺備腑鎻忚堪鐨?'target_attributes'锛屽浜?'verity' 鐩爣鍏锋湁浠ヤ笅鏁版嵁鏍煎紡銆?
```

 target_attributes := <target_name> "," <target_version> "," <hash_failed> "," <verity_version> ","
                      <data_device_name> "," <hash_device_name> "," <verity_algorithm> "," <root_digest> ","
                      <salt> "," <ignore_zero_blocks> "," <check_at_most_once> ["," <root_hash_sig_key_desc>]
                      ["," <verity_mode>] ";"

 target_name := "target_name=verity"
 target_version := "target_version=" <N> "." <N> "." <N>
 hash_failed := "hash_failed=" <hash_failed_str>
 hash_failed_str := "C" | "V"
 verity_version := "verity_version=" <verity_version_str>
 data_device_name := "data_device_name=" <data_device_name_str>
 hash_device_name := "hash_device_name=" <hash_device_name_str>
 verity_algorithm := "verity_algorithm=" <verity_algorithm_str>
 root_digest := "root_digest=" <root_digest_str>
 salt := "salt=" <salt_str>
 salt_str := "-" <verity_salt_str>
 ignore_zero_blocks := "ignore_zero_blocks=" <yes_no>
 check_at_most_once := "check_at_most_once=" <yes_no>
 root_hash_sig_key_desc := "root_hash_sig_key_desc="
 verity_mode := "verity_mode=" <verity_mode_str>
 verity_mode_str := "ignore_corruption" | "restart_on_corruption" | "panic_on_corruption" | "invalid"
 yes_no := "y" | "n"

 渚嬪锛? 褰撳姞杞戒竴涓?'verity' 鐩爣鏃讹紝IMA ASCII 搴﹂噺鏃ュ織浼氭湁涓€鏉＄被浼间簬涓嬮潰鐨勬潯鐩紝
 灞曠ず鍦?'dm_table_load' 浜嬩欢鐨?EVENT_DATA 涓害閲忎簡鍝簺 'verity' 灞炴€с€? 锛堜负渚夸簬闃呰锛屽凡浠?ASCII 杞崲涓烘枃鏈級

 dm_version=4.45.0;
 name=test-verity,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=1953120,target_name=verity,target_version=1.8.0,hash_failed=V,
 verity_version=1,data_device_name=253:1,hash_device_name=253:0,verity_algorithm=sha256,
 root_digest=29cb87e60ce7b12b443ba6008266f3e41e93e403d7f298f8e3f316b29ff89c5e,
 salt=e48da609055204e89ae53b655ca2216dd983cf3cb829f34f63a297d106d53e2d,
 ignore_zero_blocks=n,check_at_most_once=n;

```
