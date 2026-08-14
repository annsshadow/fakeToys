
## CXL 椹卞姩鎿嶄綔


```

  /sys/bus/cxl/devices/
  /dev/cxl/

```
`cxl-cli` 搴撲綔涓?NDTCL 椤圭洰鐨勪竴閮ㄥ垎杩涜缁存姢锛屽彲鐢ㄤ簬缂栧啓涓庤繖浜涜澶囦氦浜掔殑鑴氭湰銆?
## 椹卞姩

CXL 椹卞姩琚媶鍒嗕负澶氫釜椹卞姩銆?
- cxl_core  - 鍩虹鍒濆鍖栨帴鍙ｄ笌鏍稿績瀵硅薄鍒涘缓
- cxl_port  - 鍒濆鍖栨牴骞舵彁渚涚鍙ｆ灇涓炬帴鍙ｃ€?- cxl_acpi  - 鍒濆鍖栨牴瑙ｇ爜鍣ㄥ苟涓?ACPI 鏁版嵁浜や簰銆?- cxl_p/mem - 鍒濆鍖栧唴瀛樿澶?- cxl_pci   - 浣跨敤 cxl_port 鏋氫妇瀹為檯鐨?fabric 灞傜骇缁撴瀯銆?
## 椹卞姩璁惧

涓嬮潰鏄竴涓潵鑷崟璺紙single-socket锛夌郴缁熴€佸甫鏈?4 涓富鏈烘ˉ锛坔ost bridge锛夌殑绀轰緥銆傚叾涓袱涓富鏈烘ˉ鍚勬寕杞戒簡涓€涓唴瀛樿澶囷紝涓旇繖浜涜澶囪浜ら敊锛坕nterleaved锛?```

  # ls /sys/bus/cxl/devices/
    dax_region0  decoder3.0  decoder6.0  mem0   port3
    decoder0.0   decoder4.0  decoder6.1  mem1   port4
    decoder1.0   decoder5.0  endpoint5   port1  region0
    decoder2.0   decoder5.1  endpoint6   port2  root0


```
   :alt: 鎻忚堪涓绘満妗ヤ氦閿欑殑 CXL fabric 鏈夊悜鍥?   :caption: 甯︽湁涓绘満妗ヤ氦閿欏唴瀛樺尯鍩熺殑 CXL fabric 鏈夊悜鍥?
   digraph foo {
     "root0" -> "port1";
     "root0" -> "port3";
     "root0" -> "decoder0.0";
     "port1" -> "endpoint5";
     "port3" -> "endpoint6";
     "port1" -> "decoder1.0";
     "port3" -> "decoder3.0";
     "endpoint5" -> "decoder5.0";
     "endpoint6" -> "decoder6.0";
     "decoder0.0" -> "region0";
     "decoder0.0" -> "decoder1.0";
     "decoder0.0" -> "decoder3.0";
     "decoder1.0" -> "decoder5.0";
     "decoder3.0" -> "decoder6.0";
     "decoder5.0" -> "region0";
     "decoder6.0" -> "region0";
     "region0" -> "dax_region0";
     "dax_region0" -> "dax0.0";
   }

鏈妭鎴戜滑灏嗘帰绱㈡閰嶇疆涓瓨鍦ㄧ殑璁惧锛屼絾鏇村閰嶇疆灏嗗湪涓嬮潰鐨勭ず渚嬮厤缃腑娣卞叆璁ㄨ銆?
### 鍩虹璁惧

CXL fabric 涓殑澶у鏁拌澶囬兘鏄煇绉嶇被鍨嬬殑 `port`锛堝洜涓烘瘡涓澶囦富瑕佹槸灏嗚姹備粠涓€涓澶囪矾鐢卞埌涓嬩竴涓紝鑰岄潪鎻愪緵鐩存帴鏈嶅姟锛夈€?
#### Root

`CXL Root` 鏄竴涓€昏緫瀵硅薄锛岀敱 `cxl_acpi` 椹卞姩鍦?`cxl_acpi_probe` 鏈熼棿鍒涘缓鈥斺€斿墠鎻愭槸鎵惧埌浜?`ACPI0017` `Compute Express Link
Root Object`锛堟牴瀵硅薄锛夎澶囩被銆?
Root 鍖呭惈鎸囧悜浠ヤ笅瀵硅薄鐨勯摼鎺ワ細

- 鐢?CHBS 鍦?[CEDT<../platform/acpi/cedt>](CEDT<../platform/acpi/cedt>) 涓畾涔夌殑 `Host Bridge Ports`

- 閫氬父杩炴帴鍒?`Host Bridge Ports` 鐨?`Downstream Ports`銆?
- 鐢?CFMWS 鍦?[CEDT<../platform/acpi/cedt>](CEDT<../platform/acpi/cedt>) 涓畾涔夌殑 `Root Decoders`

```

  # ls /sys/bus/cxl/devices/root0
    decoder0.0          dport0  dport5    port2  subsystem
    decoders_committed  dport1  modalias  port3  uevent
    devtype             dport4  port1     port4  uport

  # cat /sys/bus/cxl/devices/root0/devtype
    cxl_port

  # cat port1/devtype
    cxl_port

  # cat decoder0.0/devtype
    cxl_decoder_root

```
root 鏄敱 Linux CXL 椹卞姩鍛堢幇鐨?CXL fabric 涓涓€涓?`logical port`锛堥€昏緫绔彛锛夈€俙CXL root` 鏄竴绉嶇壒娈婄被鍨嬬殑 `switch port`锛堜氦鎹㈢鍙ｏ級锛屽洜涓哄畠鍙湁涓嬫父绔彛杩炴帴銆?
#### Port

`port` 瀵硅薄鏇村噯纭湴琚弿杩颁负涓€涓?`switch port`锛堜氦鎹㈢鍙ｏ級銆傚畠鍙互琛ㄧず涓€涓埌 root 鐨勪富鏈烘ˉ锛屾垨鑰呬氦鎹㈡満涓婄殑涓€涓疄闄呬氦鎹㈢鍙ｃ€備竴涓?`switch port` 鍖呭惈涓€涓垨澶氫釜瑙ｇ爜鍣紝鐢ㄤ簬灏嗗唴瀛樿姹傝矾鐢卞埌涓嬫父绔彛锛岃繖浜涗笅娓哥鍙ｅ彲鑳借繛鎺ュ埌鍙︿竴涓?`switch port` 鎴栦竴涓?`endpoint port`銆?
```

  # ls /sys/bus/cxl/devices/port1
    decoder1.0          dport0    driver     parent_dport  uport
    decoders_committed  dport113  endpoint5  subsystem
    devtype             dport2    modalias   uevent

  # cat devtype
    cxl_port

  # cat decoder1.0/devtype
    cxl_decoder_switch

  # cat endpoint5/devtype
    cxl_port

```
CXL fabric 涓殑 `Host Bridges` 鍦ㄦ帰娴?`CXL Root` 鐨勫悓鏃讹紝浜?`cxl_acpi_probe` 鏈熼棿琚帰娴嬨€傝繖浣垮緱 root 涓庝富鏈烘ˉ涔嬮棿鑳藉绔嬪嵆寤虹珛閫昏緫杩炴帴銆?
- root 鏈変竴涓埌涓绘満妗ョ殑涓嬫父绔彛杩炴帴

- 涓绘満妗ユ湁涓€涓埌 root 鐨勪笂娓哥鍙ｈ繛鎺ャ€?
- 涓绘満妗ユ湁涓€涓垨澶氫釜鍒颁氦鎹㈡満鎴栫鐐圭鍙ｇ殑涓嬫父绔彛杩炴帴銆?
`Host Bridge` 鏄竴绉嶇壒娈婄被鍨嬬殑 CXL `switch port`銆傚畠鍦?ACPI 瑙勮寖涓€氳繃 `ACPI0016` ID 鏄惧紡瀹氫箟銆俙Host Bridge` 绔彛灏嗗湪 `acpi_probe` 鏃惰鎺㈡祴锛岃€屽疄闄呬氦鎹㈡満涓婄殑绫讳技绔彛灏嗗湪绋嶅悗琚帰娴嬨€傞櫎姝や箣澶栵紝浜ゆ崲鏈虹鍙ｄ笌涓绘満妗ョ鍙ｇ湅璧锋潵闈炲父鐩镐技鈥斺€斿畠浠兘鍖呭惈鐢ㄤ簬鍦ㄤ笂涓嬫父绔彛涔嬮棿璺敱璁块棶鐨勪氦鎹㈡満瑙ｇ爜鍣ㄣ€?
#### Endpoint

`endpoint` 鏄?fabric 涓殑涓€涓粓绔鍙ｃ€傚畠鏄竴涓?`logical device`锛堥€昏緫璁惧锛夛紝骞朵笖鍙兘鏄敱鏌愪釜鍐呭瓨璁惧鍛堢幇鐨勪紬澶?`logical devices` 涔嬩竴銆傚湪 fabric 涓畠浠嶈瑙嗕负涓€绉?`port`銆?
涓€涓?`endpoint` 鍖呭惈 `endpoint decoders`锛堢鐐硅В鐮佸櫒锛変互鍙婅澶囩殑 Coherent Device
```

  # ls /sys/bus/cxl/devices/endpoint5
    CDAT        decoders_committed  modalias      uevent
    decoder5.0  devtype             parent_dport  uport
    decoder5.1  driver              subsystem

  # cat /sys/bus/cxl/devices/endpoint5/devtype
    cxl_port

  # cat /sys/bus/cxl/devices/endpoint5/decoder5.0/devtype
    cxl_decoder_endpoint


```
#### Memory Device锛坢emdev锛?
`memdev` 鐢?`cxl_pci` 椹卞姩鍦?`cxl_pci_probe` 涓帰娴嬪苟娣诲姞锛屽苟鐢?`cxl_mem` 椹卞姩绠＄悊銆傚畠涓昏閫氳繃 `/dev/cxl/memN` 鎻愪緵鍒板唴瀛樿澶囩殑 `IOCTL` 鎺ュ彛锛屽苟鏆撮湶鍚勭
```

  # ls /sys/bus/cxl/devices/mem0
    dev       firmware_version    payload_max  security   uevent
    driver    label_storage_size  pmem         serial
    firmware  numa_node           ram          subsystem

```
涓€涓?Memory Device 鏄竴涓笉灞炵鍙ｇ被鍨嬬殑绂绘暎鍩虹瀵硅薄銆傝櫧鐒跺畠鎵€灞炵殑鐗╃悊璁惧涔熷彲鑳芥壙杞戒竴涓?`endpoint`锛屼絾 `endpoint` 涓?`memdev` 涔嬮棿鐨勫叧绯诲苟鏈湪 sysfs 涓綋鐜般€?
#### Port Relationships

鍦ㄤ笂杩扮ず渚嬩腑锛屾湁鍥涗釜涓绘満妗ヨ繛鎺ュ埌 root锛屽叾涓袱涓富鏈烘ˉ鍚勬寕杞戒簡涓€涓鐐广€?
   :alt: 鎻忚堪涓绘満妗ヤ氦閿欑殑 CXL fabric 鏈夊悜鍥?   :caption: 甯︽湁涓绘満妗ヤ氦閿欏唴瀛樺尯鍩熺殑 CXL fabric 鏈夊悜鍥?
   digraph foo {
     "root0"    -> "port1";
     "root0"    -> "port2";
     "root0"    -> "port3";
     "root0"    -> "port4";
     "port1" -> "endpoint5";
     "port3" -> "endpoint6";
   }

### Decoders

`Decoder`锛堣В鐮佸櫒锛夋槸 CXL Host-Managed Device Memory锛圚DM锛屼富鏈虹鐞嗚澶囧唴瀛橈級Decoder 鐨勭畝绉般€傚畠鏄竴涓皢璁块棶閫氳繃 CXL fabric 璺敱鍒扮鐐广€佸苟鍦ㄧ鐐瑰灏?`Host Physical`锛堜富鏈虹墿鐞嗗湴鍧€锛夎浆鎹负 `Device Physical`锛堣澶囩墿鐞嗗湴鍧€锛夊鍧€鐨勮澶囥€?
CXL 3.1 瑙勮寖寮虹儓鏆楃ず鍙湁绔偣瑙ｇ爜鍣ㄦ墠搴斿弬涓?`Host Physical Address` 鍒?`Device Physical Address` 鐨勮浆鎹€?```

  8.2.4.20 CXL HDM Decoder Capability Structure

  IMPLEMENTATION NOTE
  CXL Host Bridge and Upstream Switch Port Decode Flow

  IMPLEMENTATION NOTE
  Device Decode Logic

```
杩欎簺娉ㄨ鏆楃ず瀛樺湪涓や釜閫昏緫鐨勮В鐮佸櫒鍒嗙粍銆?
- Routing Decoder锛堣矾鐢辫В鐮佸櫒锛? 浠呰矾鐢辫闂絾涓嶇炕璇戝湴鍧€锛堜粠 HPA 鍒?DPA锛夌殑瑙ｇ爜鍣ㄣ€?
- Translating Decoder锛堣浆鎹㈣В鐮佸櫒锛? 涓虹鐐规湇鍔¤€屽皢璁块棶浠?HPA 杞崲涓?DPA 鐨勮В鐮佸櫒銆?
CXL 椹卞姩鍖哄垎 3 绉嶈В鐮佸櫒绫诲瀷锛歳oot銆乻witch 鍜?endpoint銆傚彧鏈夌鐐硅В鐮佸櫒鏄?Translating Decoder锛堣浆鎹㈣В鐮佸櫒锛夛紝鍏朵綑閮芥槸 Routing Decoder锛堣矾鐢辫В鐮佸櫒锛夈€?

   Linux 寮虹儓鍋囪绔偣瑙ｇ爜鍣ㄦ槸 fabric 涓敮涓€涓诲姩灏?HPA 杞崲涓?DPA 鐨勮В鐮佸櫒銆侺inux 鍋囪璺敱瑙ｇ爜鍣ㄥ皢 HPA 鍘熸牱浼犻€掔粰 fabric 涓殑涓嬩竴涓В鐮佸櫒銆?
   鍥犳锛屽亣璁?fabric 涓换浣曠粰瀹氱殑瑙ｇ爜鍣ㄧ殑鍦板潃鑼冨洿閮芥槸鍏朵笂娓哥鍙ｈВ鐮佸櫒鍦板潃鑼冨洿鐨勫瓙闆嗐€傚姝ゆ柟妗堢殑浠讳綍鍋忕鍦ㄨ鑼冧腑閮藉睘浜庢湭瀹氫箟琛屼负銆侺inux 浼樺厛閲囩敤瑙勮寖瀹氫箟/鏋舵瀯瀹氫箟鐨勮涓恒€?
瑙ｇ爜鍣ㄥ鏋滈厤缃负浜ら敊鍐呭瓨璁块棶锛屽垯鍙兘鍏锋湁涓€涓垨澶氫釜 `Downstream Targets`锛堜笅娓哥洰鏍囷級銆傝繖灏嗛€氳繃 `target_list` 鍙傛暟鍦?sysfs 涓憟鐜般€?
#### Root Decoder

`Root Decoder` 鏄?:doc:`CEDT
<../platform/acpi/cedt>` 涓?CFMWS 瀛楁鎵€琛ㄧず鐨勭墿鐞嗗湴鍧€涓庝氦閿欓厤缃殑閫昏緫鏋勯€犮€?Linux 灏嗘淇℃伅鍛堢幇涓哄瓨鍦ㄤ簬 `CXL Root` 涓殑涓€涓В鐮佸櫒銆傛垜浠皢鍏惰涓轰竴涓?`Root Decoder`锛屽敖绠′弗鏍兼潵璇村畠瀛樺湪浜?CXL 瑙勮寖涓庡钩鍙扮浉鍏崇殑 CXL root 瀹炵幇鐨勮竟鐣屼笂銆?
Linux 灏嗚繖浜涢€昏緫瑙ｇ爜鍣ㄨ涓轰竴绉?`Routing Decoder`锛堣矾鐢辫В鐮佸櫒锛夛紝骞朵笖鏄?CXL fabric 涓涓€涓帴鏀舵潵鑷钩鍙板唴瀛樻帶鍒跺櫒鐨勫唴瀛樿闂殑瑙ｇ爜鍣ㄣ€?
`Root Decoders` 鍦?`cxl_acpi_probe` 鏈熼棿鍒涘缓銆傛瘡涓?CFMWS 鏉＄洰鍦?[CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 涓垱寤轰竴涓?root 瑙ｇ爜鍣ㄣ€?
`target_list` 鍙傛暟鐢?CFMWS 鐨?target 瀛楁濉厖銆俽oot 瑙ｇ爜鍣ㄧ殑鐩爣鏄?`Host Bridges`锛堜富鏈烘ˉ锛夛紝杩欐剰鍛崇潃鍦?root 瑙ｇ爜鍣ㄧ骇鍒畬鎴愮殑浜ら敊鏄竴绉?`Inter-Host-Bridge Interleave`锛堜富鏈烘ˉ闂翠氦閿欙級銆?
鍙湁 root 瑙ｇ爜鍣ㄨ兘澶熻繘琛?`Inter-Host-Bridge Interleave`锛堜富鏈烘ˉ闂翠氦閿欙級銆?
姝ょ被浜ら敊蹇呴』鐢卞钩鍙伴厤缃紝骞舵弿杩板湪 ACPI CEDT CFMWS 涓紝鍥犱负 CFMWS 涓殑鐩爣 CXL 涓绘満妗?UID 蹇呴』涓?:doc:`CEDT
<../platform/acpi/cedt>` 鐨?CHBS 瀛楁涓殑 CXL 涓绘満妗?UID锛屼互鍙?[DSDT <../platform/acpi/dsdt>](DSDT <../platform/acpi/dsdt>) 涓畾涔夌殑 CXL 涓绘満妗?UID 瀛楁鐩稿尮閰嶃€?
root 瑙ｇ爜鍣ㄤ腑鐨勪氦閿欒缃弿杩扮殑鏄浣曞湪**鐩存帴涓嬫父鐩爣**涔嬮棿浜ら敊璁块棶锛岃€岄潪鏁翠釜浜ら敊闆嗗悎銆?
root 瑙ｇ爜鍣ㄦ弿杩扮殑鍐呭瓨鑼冨洿鐢ㄤ簬

1) 鍒涘缓涓€涓唴瀛樺尯鍩燂紙鏈緥涓负 `region0`锛夛紝浠ュ強

2) 灏嗚鍖哄煙涓庝竴涓?IO Memory Resource锛坄kernel/resource.c`锛夊叧鑱?
```

  # ls /sys/bus/cxl/devices/decoder0.0/
    cap_pmem           devtype                 region0
    cap_ram            interleave_granularity  size
    cap_type2          interleave_ways         start
    cap_type3          locked                  subsystem
    create_ram_region  modalias                target_list
    delete_region      qos_class               uevent

  # cat /sys/bus/cxl/devices/decoder0.0/region0/resource
    0xc050000000

```
IO Memory Resource 鍦ㄦ棭鏈熷紩瀵兼湡闂村垱寤猴紝姝ゆ椂鍦?EFI Memory Map 鎴?E820 琛紙鍦?x86 涓婏級涓瘑鍒埌 CFMWS 鍖哄煙銆?
Root 瑙ｇ爜鍣ㄨ瀹氫箟涓轰竴涓嫭绔嬬殑 devtype锛屼絾瀹冨悓鏃朵篃鏄煇绉嶇被鍨?```

  # cat /sys/bus/cxl/devices/decoder0.0/devtype
    cxl_decoder_root

```
#### Switch Decoder

浠讳綍闈?root 鐨勩€佽繘琛岃浆鎹㈢殑瑙ｇ爜鍣ㄩ兘琚涓?`Switch Decoder`锛堜氦鎹㈡満瑙ｇ爜鍣級锛屽苟鍛堢幇涓?`cxl_decoder_switch` 绫诲瀷銆俙Host Bridge` 鍜?`CXL
```

  # ls /sys/bus/cxl/devices/decoder1.0/
    devtype                 locked    size       target_list
    interleave_granularity  modalias  start      target_type
    interleave_ways         region    subsystem  uevent

  # cat /sys/bus/cxl/devices/decoder1.0/devtype
    cxl_decoder_switch

  # cat /sys/bus/cxl/devices/decoder1.0/region
    region0

```
`Switch Decoder` 寤虹珛浜嗙敱 root 瑙ｇ爜鍣ㄥ畾涔夌殑鍖哄煙涓庝笅娓哥洰鏍囩鍙ｄ箣闂寸殑鍏宠仈銆傚湪浜ゆ崲鏈鸿В鐮佸櫒鍐呴儴瀹屾垚鐨勪氦閿欐槸澶氫笅娓哥鍙ｄ氦閿欙紙瀵逛簬涓绘満妗ュ垯鏄?`Intra-Host-Bridge Interleave`锛屼富鏈烘ˉ鍐呬氦閿欙級銆?
浜ゆ崲鏈鸿В鐮佸櫒涓殑浜ら敊璁剧疆鎻忚堪鐨勬槸濡備綍鍦?*鐩存帴涓嬫父鐩爣**涔嬮棿浜ら敊璁块棶锛岃€岄潪鏁翠釜浜ら敊闆嗗悎銆?
浜ゆ崲鏈鸿В鐮佸櫒鍦?`cxl_port` 椹卞姩鐨?`cxl_switch_port_probe` 鏈熼棿鍒涘缓锛屽苟鍩轰簬 PCI 璁惧鐨?DVSEC 瀵勫瓨鍣ㄥ垱寤恒€?
浜ゆ崲鏈鸿В鐮佸櫒缂栫▼鍦ㄦ帰娴嬫湡闂磋繘琛岄獙璇侊紙濡傛灉骞冲彴鍦ㄥ紩瀵兼椂瀵瑰叾杩涜浜嗙紪绋嬶紝瑙佷笅鏂?`Auto Decoders`锛夛紝鎴栧湪鎻愪氦鏃惰繘琛岄獙璇侊紙濡傛灉鍦ㄨ繍琛屾椂缂栫▼锛岃涓嬫枃 `Runtime Programming`锛夈€?
#### Endpoint Decoder

浠讳綍杩炴帴鍒?CXL fabric 涓?*缁堢**鐐癸紙`An Endpoint`锛夌殑瑙ｇ爜鍣ㄩ兘琚涓?`Endpoint Decoder`锛堢鐐硅В鐮佸櫒锛夈€傜鐐硅В鐮佸櫒鐨勭被鍨嬩负
```

  # ls /sys/bus/cxl/devices/decoder5.0
    devtype                 locked    start
    dpa_resource            modalias  subsystem
    dpa_size                mode      target_type
    interleave_granularity  region    uevent
    interleave_ways         size

  # cat /sys/bus/cxl/devices/decoder5.0/devtype
    cxl_decoder_endpoint

  # cat /sys/bus/cxl/devices/decoder5.0/region
    region0

```
`Endpoint Decoder` 涓庣敱 root 瑙ｇ爜鍣ㄥ畾涔夌殑鍖哄煙鐩稿叧鑱旓紝骞舵弿杩颁笌璇ュ尯鍩熷叧鑱旂殑璁惧鏈湴璧勬簮銆?
涓?root 鍜屼氦鎹㈡満瑙ｇ爜鍣ㄤ笉鍚岋紝绔偣瑙ｇ爜鍣ㄥ皢 `Host Physical`锛堜富鏈虹墿鐞嗗湴鍧€锛夎浆鎹负 `Device Physical`锛堣澶囩墿鐞嗗湴鍧€锛夊湴鍧€鑼冨洿銆傚洜姝ょ鐐逛笂鐨勪氦閿欒缃弿杩扮殑鏄暣涓?*浜ら敊闆嗗悎**銆?
`Device Physical Address`锛堣澶囩墿鐞嗗湴鍧€锛夊尯鍩熷繀椤绘寜椤哄簭鎻愪氦銆備緥濡傦紝璧峰浜?0x80000000 鐨?DPA 鍖哄煙涓嶈兘鍦ㄨ捣濮嬩簬 0x0 鐨?DPA 鍖哄煙涔嬪墠鎻愪氦銆?
鑷?Linux v6.15 璧凤紝Linux 涓嶆敮鎸?*涓嶅钩琛?*鐨勪氦閿欓厤缃紝浜ら敊闆嗗悎涓殑鎵€鏈夌鐐归兘搴斿叿鏈夌浉鍚岀殑浜ら敊璁剧疆锛坓ranularity 涓?ways 蹇呴』鐩稿悓锛夈€?
绔偣瑙ｇ爜鍣ㄥ湪 `cxl_port` 椹卞姩鐨?`cxl_endpoint_port_probe` 鏈熼棿鍒涘缓锛屽苟鍩轰簬 PCI 璁惧鐨?DVSEC 瀵勫瓨鍣ㄥ垱寤恒€?
#### Decoder Relationships

鍦ㄤ笂杩扮ず渚嬩腑锛屽瓨鍦ㄤ竴涓?root 瑙ｇ爜鍣紝瀹冮€氳繃涓や釜涓绘満妗ヨ矾鐢卞唴瀛樿闂€傛瘡涓富鏈烘ˉ鏈変竴涓В鐮佸櫒锛屽皢璁块棶璺敱鍒板叾鍞竴鐨勭鐐圭洰鏍囥€傛瘡涓鐐规湁涓€涓В鐮佸櫒锛屽皢 HPA 杞崲涓?DPA 骞舵湇鍔′簬鍐呭瓨璇锋眰銆?
椹卞姩閫氳繃瑙ｇ爜鍣ㄧ紪绋嬮獙璇佺鍙ｄ箣闂寸殑鍏崇郴锛屽洜姝ゆ垜浠彲浠ュ皢瑙ｇ爜鍣ㄤ箣闂寸殑鍏崇郴瑙嗕负涓庣鍙ｇ被浼肩殑灞傜骇缁撴瀯銆?
   :alt: root銆乻witch 涓?endpoint 瑙ｇ爜鍣ㄤ箣闂村眰绾у叧绯荤殑鏈夊悜鍥俱€?   :caption: CXL root銆乻witch 涓?endpoint 瑙ｇ爜鍣ㄧ殑鏈夊悜鍥俱€?
   digraph foo {
     "root0"    -> "decoder0.0";
     "decoder0.0" -> "decoder1.0";
     "decoder0.0" -> "decoder3.0";
     "decoder1.0" -> "decoder5.0";
     "decoder3.0" -> "decoder6.0";
   }

### Regions

#### Memory Region

`Memory Region`锛堝唴瀛樺尯鍩燂級鏄竴涓€昏緫鏋勯€狅紝瀹冨皢 fabric 涓殑涓€缁?CXL 绔彛杩炴帴鍒颁竴涓?IO Memory Resource銆傚畠鏈€缁堢敤浜庨€氳繃 `DAX Region` 灏嗚繖浜涜澶囦笂鐨勫唴瀛樻毚闇茬粰 DAX 瀛愮郴缁熴€?
```

  # ls /sys/bus/cxl/devices/region0/
    access0      devtype                 modalias  subsystem  uuid
    access1      driver                  mode      target0
    commit       interleave_granularity  resource  target1
    dax_region0  interleave_ways         size      uevent

```
涓€涓唴瀛樺尯鍩熷彲浠ュ湪绔偣鎺㈡祴鏈熼棿鏋勯€狅紙濡傛灉瑙ｇ爜鍣ㄧ敱 BIOS/EFI 缂栫▼锛岃 `Auto Decoders`锛夛紝鎴栬€呴€氳繃 `Root Decoder` 鐨?`create_ram_region` 鎴?`create_pmem_region` 鎺ュ彛鎵嬪姩鍒涘缓銆?
`Memory Region` 涓殑浜ら敊璁剧疆鎻忚堪浜?`Interleave Set`锛堜氦閿欓泦鍚堬級鐨勯厤缃€斺€斾篃灏辨槸鍦ㄧ鐐逛氦閿欒缃腑鎵€鑳介鏈熺湅鍒扮殑鍐呭銆?
   :alt: root 涓?endpoint 瑙ｇ爜鍣ㄤ箣闂?CXL 鍐呭瓨鍖哄煙鍏崇郴鐨勬湁鍚戝浘銆?   :caption: 鍖哄煙鍩轰簬 root 瑙ｇ爜鍣ㄩ厤缃垱寤恒€傜鐐硅В鐮佸櫒蹇呴』浣跨敤涓庡尯鍩熺浉鍚岀殑浜ら敊璁剧疆杩涜缂栫▼銆?
   digraph foo {
     "root0"    -> "decoder0.0";
     "decoder0.0" -> "region0";
     "region0" -> "decoder5.0";
     "region0" -> "decoder6.0";
   }

#### DAX Region

`DAX Region` 鐢ㄤ簬灏嗕竴涓?CXL `Memory Region` 杞崲涓轰竴涓?DAX 璁惧銆傞殢鍚庡彲閫氳繃鏂囦欢鎻忚堪绗︽帴鍙ｇ洿鎺ヨ闂 DAX 璁惧锛屾垨閫氳繃 DAX kmem 椹卞姩杞崲涓?System RAM銆傚弬瑙?DAX 椹卞姩灏忚妭
```

  # ls /sys/bus/cxl/devices/dax_region0/
    dax0.0      devtype  modalias   uevent
    dax_region  driver   subsystem

```
### Mailbox Interfaces

```

  /dev/cxl/mem0
  /dev/cxl/mem1

```
杩欎簺閭鍙互鎺ユ敹浠讳綍瑙勮寖瀹氫箟鐨勫懡浠ゃ€傚師濮嬪懡浠わ紙鑷畾涔夊懡浠わ級鍙湁鍦ㄦ瀯寤洪厤缃?`CXL_MEM_RAW_COMMANDS` 琚缃椂鎵嶈兘鍙戦€佸埌杩欎簺鎺ュ彛銆傝繖琚涓轰竴涓皟璇曞拰/鎴栧紑鍙戞帴鍙ｏ紝骞堕潪鐢ㄤ簬鍒涘缓鍘傚晢鐗瑰畾鍛戒护鐨勫畼鏂规敮鎸佹満鍒讹紙鐩稿叧璇峰弬瑙?`fwctl` 瀛愮郴缁燂級銆?
## Decoder Programming

### Runtime Programming

鍦ㄦ帰娴嬫湡闂达紝**蹇呴』**缂栫▼鐨勮В鐮佸櫒鍙湁 `Root Decoders`銆傚疄闄呬笂锛宍Root Decoders` 鏄弿杩颁富鏈烘ˉ绾у埆鍐呭瓨鍖哄煙涓庝氦閿欓厤缃殑閫昏緫鏋勯€犫€斺€斿 ACPI CEDT CFMWS 涓墍杩般€?
鎵€鏈夊叾浠?`Switch` 涓?`Endpoint` 瑙ｇ爜鍣ㄩ兘鍙互鍦ㄨ繍琛屾椂鐢辩敤鎴风紪绋嬧€斺€斿墠鎻愭槸骞冲彴鏀寔姝ょ被閰嶇疆銆?
杩欑浜や簰鍒涢€犱簡 `Software Defined Memory`锛堣蒋浠跺畾涔夊唴瀛橈級鐜銆?
鏈夊叧濡備綍鍦ㄨ繍琛屾椂閰嶇疆 CXL 瑙ｇ爜鍣ㄧ殑鏇村淇℃伅锛岃鍙傞槄 `cxl-cli` 鏂囨。銆?
### Auto Decoders

Auto Decoders 鏄敱 BIOS/EFI 鍦ㄥ紩瀵兼椂缂栫▼鐨勮В鐮佸櫒锛屽嚑涔庢€绘槸琚攣瀹氾紙涓嶅彲鏇存敼锛夈€傝繖鏄敱鍙兘鍏锋湁闈欐€侀厤缃殑骞冲彴瀹屾垚鐨勨€斺€旀垨鑰呮煇浜涙€紓鐗规€у彲鑳介樆姝㈠瑙ｇ爜鍣ㄨ繘琛屽姩鎬佽繍琛屾椂鏇存敼锛堜緥濡傚湪 CXL 鑼冨洿涔嬪鐨?CPU 澶嶅悎浣撳唴闇€瑕侀澶栫殑鎺у埗鍣ㄧ紪绋嬶級銆?
鍙 Auto Decoders 鎵€鍏宠仈鐨勮澶囦笌鍐呭瓨鍖哄煙鑳藉鏃犻棶棰樺湴鎺㈡祴锛屽畠浠氨浼氳嚜鍔ㄨ鎺㈡祴銆傚湪鎺㈡祴 Auto Decoders 鏃讹紝椹卞姩鐨勪富瑕佽亴璐ｆ槸纭繚 fabric 鐘舵€佹甯革紙sane锛夆€斺€斿鍚岄獙璇佽繍琛屾椂缂栫▼鐨勫尯鍩熶笌瑙ｇ爜鍣ㄤ竴鏍枫€?
濡傛灉 Linux 鏃犳硶楠岃瘉 auto-decoder 閰嶇疆锛岃鍐呭瓨灏嗕笉浼氳浣滀负 DAX 璁惧鍛堢幇鈥斺€斿洜姝や篃涓嶄細鏆撮湶缁欓〉鍒嗛厤鍣ㄢ€斺€斿疄闄呬笂琚悂缃紙stranding锛変簡銆?
### Interleave

Linux CXL 椹卞姩鏀寔 `Cross-Link First`锛堜氦鍙夐摼璺紭鍏堬級浜ら敊銆傝繖瑙勫畾浜嗗湪姣忎釜瑙ｇ爜鍣ㄦ楠ゅ浣曠紪绋嬩氦閿欙紝鍥犱负椹卞姩浼氶獙璇佽В鐮佸櫒涓庡叾鐖剁骇涔嬮棿鐨勫叧绯汇€?
渚嬪锛屽湪涓€涓?`Cross-Link First` 浜ら敊閰嶇疆涓紝16 涓鐐硅繛鎺ュ埌 4 涓富鏈烘ˉ锛孡inux 鏈熸湜鍦?root銆佷富鏈烘ˉ鍜岀鐐逛笂鍒嗗埆鏈夊涓嬬殑 ways/granularity锛?

  - - decoder
    - ways
    - granularity

  - - root
    - 4
    - 256

  - - host bridge
    - 4
    - 1024

  - - endpoint
    - 16
    - 256

鍦?root 绾у埆锛屾瘡娆＄粰瀹氱殑璁块棶灏嗚璺敱鍒?`((HPA / 256) % 4)` 鍙风洰鏍囦富鏈烘ˉ銆傚湪涓绘満妗ュ唴锛岃矾鐢卞埌 `((HPA / 1024) % 4)` 鍙风洰鏍囩鐐广€傛瘡涓鐐瑰熀浜庢暣涓?16 璁惧浜ら敊闆嗗悎杩涜杞崲銆?
涓嶆敮鎸佷笉骞宠　鐨勪氦閿欓泦鍚堚€斺€斿眰绾х粨鏋勪腑鐩镐技浣嶇疆鐨勮В鐮佸櫒锛堜緥濡傛墍鏈変富鏈烘ˉ瑙ｇ爜鍣級蹇呴』鍏锋湁鐩稿悓鐨?ways 涓?granularity 閰嶇疆銆?
#### At Root

Root 瑙ｇ爜鍣ㄤ氦閿欑敱 :doc:`CEDT
<../platform/acpi/cedt>` 鐨?CFMWS 瀛楁瀹氫箟銆侰EDT 瀹為檯涓婂彲鑳藉畾涔夊涓?CFMWS 閰嶇疆鏉ユ弿杩扮浉鍚岀殑鐗╃悊瀹归噺锛屾剰鍥炬槸鍏佽鐢ㄦ埛鍦ㄨ繍琛屾椂鍐冲畾鏄皢鍐呭瓨浣滀负浜ら敊鏂瑰紡涓婄嚎锛岃繕鏄?```

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000100000000
               Window size : 0000000100000000
  Interleave Members (2^n) : 00
     Interleave Arithmetic : 00
              First Target : 00000007

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000200000000
               Window size : 0000000100000000
  Interleave Members (2^n) : 00
     Interleave Arithmetic : 00
              First Target : 00000006

             Subtable Type : 01 [CXL Fixed Memory Window Structure]
       Window base address : 0000000300000000
               Window size : 0000000200000000
  Interleave Members (2^n) : 01
     Interleave Arithmetic : 00
              First Target : 00000007
               Next Target : 00000006

```
鍦ㄦ湰渚嬩腑锛孋FMWS 涓烘瘡涓富鏈烘ˉ瀹氫箟浜嗕袱涓鏁ｇ殑闈炰氦閿?4GB 鍖哄煙锛屼互鍙婁竴涓互涓よ€呬负鐩爣鐨?8GB 浜ら敊鍖哄煙銆傝繖
```

  # ls /sys/bus/cxl/devices/root0/decoder*
    decoder0.0  decoder0.1  decoder0.2

  # cat /sys/bus/cxl/devices/decoder0.0/target_list start size
    7
    0x100000000
    0x100000000

  # cat /sys/bus/cxl/devices/decoder0.1/target_list start size
    6
    0x200000000
    0x100000000

  # cat /sys/bus/cxl/devices/decoder0.2/target_list start size
    7,6
    0x300000000
    0x200000000

```
杩欎簺瑙ｇ爜鍣ㄤ笉鍙湪杩愯鏃剁紪绋嬨€傚畠浠敤浜庣敓鎴愪竴涓?`Memory Region`锛屼互渚块€氳繃 `Switch` 涓?`Endpoint` 瑙ｇ爜鍣ㄤ笂杩愯鏃剁紪绋嬬殑璁剧疆灏嗘鍐呭瓨涓婄嚎銆?
#### At Host Bridge or Switch

`Host Bridge` 涓?`Switch` 瑙ｇ爜鍣ㄥ彲閫氳繃浠ヤ笅瀛楁缂栫▼锛?
- `start` - 涓庡唴瀛樺尯鍩熷叧鑱旂殑 HPA 鍖哄煙
- `size` - 鍖哄煙鐨勫ぇ灏?- `target_list` - 涓嬫父绔彛鍒楄〃
- `interleave_ways` - 瑕佷氦閿欒法瓒婄殑涓嬫父绔彛鏁伴噺
- `interleave_granularity` - 浜ら敊绮掑害銆?
Linux 鏈熸湜浜ゆ崲鏈鸿В鐮佸櫒鐨?`interleave_granularity` 鐢卞叾涓婃父绔彛杩炴帴鎺ㄥ鑰屾潵銆傚湪 `Cross-Link First` 浜ら敊閰嶇疆涓紝瑙ｇ爜鍣ㄧ殑 `interleave_granularity` 绛変簬 `parent_interleave_granularity * parent_interleave_ways`銆?
#### At Endpoint

`Endpoint Decoders` 鐨勭紪绋嬫柟寮忎笌 Host Bridge 鍜?Switch 瑙ｇ爜鍣ㄧ被浼硷紝涓嶅悓涔嬪鍦ㄤ簬 ways 涓?granularity 鐢变氦閿欓泦鍚堝畾涔夛紙渚嬪鐢辩浉鍏宠仈鐨?`Memory Region` 瀹氫箟鐨勭殑浜ら敊璁剧疆锛夈€?
- `start` - 涓庡唴瀛樺尯鍩熷叧鑱旂殑 HPA 鍖哄煙
- `size` - 鍖哄煙鐨勫ぇ灏?- `interleave_ways` - 浜ら敊闆嗗悎涓殑绔偣鏁伴噺
- `interleave_granularity` - 浜ら敊绮掑害銆?
杩欎簺璁剧疆琚鐐硅В鐮佸櫒鐢ㄤ簬灏嗕粠 HPA **缈昏瘧**涓?DPA 鐨勫唴瀛樿姹傘€傝繖灏辨槸涓轰粈涔堝畠浠繀椤讳簡瑙ｆ暣涓氦閿欓泦鍚堛€?
Linux 涓嶆敮鎸佷笉骞宠　鐨勪氦閿欓厤缃€傚洜姝わ紝浜ら敊闆嗗悎涓殑鎵€鏈夌鐐瑰繀椤诲叿鏈夌浉鍚岀殑 ways 涓?granularity銆?
## Example Configurations

- [example-configurations/single-device.rst](example-configurations/single-device.rst)
- [example-configurations/hb-interleave.rst](example-configurations/hb-interleave.rst)
- [example-configurations/intra-hb-interleave.rst](example-configurations/intra-hb-interleave.rst)
- [example-configurations/multi-interleave.rst](example-configurations/multi-interleave.rst)
