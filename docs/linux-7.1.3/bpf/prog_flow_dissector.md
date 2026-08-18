
## BPF_PROG_TYPE_FLOW_DISSECTOR


## 姒傝堪


娴佽В鏋愬櫒锛坒low dissector锛夋槸涓€涓粠鏁版嵁鍖呬腑瑙ｆ瀽鍏冩暟鎹殑渚嬬▼銆傚畠琚敤浜庣綉缁滃瓙绯荤粺鐨勫涓?
鍦版柟锛圧FS銆佹祦鍝堝笇绛夛級銆?

BPF 娴佽В鏋愬櫒璇曞浘鐢?BPF 閲嶆柊瀹炵幇鍩轰簬 C 鐨勬祦瑙ｆ瀽鍣ㄩ€昏緫锛屼互鑾峰緱 BPF 楠岃瘉鍣紙verifier锛夌殑鍏ㄩ儴
濂藉锛堝嵆鎸囦护鏁伴噺鍜屽熬璋冪敤鐨勯檺鍒讹級銆?

## API


BPF 娴佽В鏋愬櫒绋嬪簭杩愯鍦?`flow_keys` 涓娿€備絾鏄紝鍙厑璁镐竴缁勫彈闄愮殑瀛楁锛歚flow_keys`銆乣struct bpf_flow_keys` 鍜?`flow_keys`銆?
`flow_keys` 鏄?`struct bpf_flow_keys`锛屽寘鍚祦瑙ｆ瀽鍣ㄧ殑杈撳叆鍜岃緭鍑哄弬鏁般€?

杈撳叆濡備笅锛?
  - `flags` - 缃戠粶澶撮儴鐨勫垵濮嬪亸绉?
  - `flags` - 浼犺緭灞傚ご閮ㄧ殑鍒濆鍋忕Щ锛屽垵濮嬪寲涓?nhoff
  - `flags` - L3 鍗忚绫诲瀷锛屼粠 L2 澶撮儴瑙ｆ瀽寰楀嚭
  - `flags` - 鍙€夋爣蹇?

BPF 娴佽В鏋愬櫒绋嬪簭搴斿綋濉啓鍏朵綑鐨?` fields. Input arguments `struct bpf_flow_keys` fields. Input arguments ` 鐨?nhoff/thoff/n_proto``
涔熷簲鐩稿簲璋冩暣銆?

BPF 绋嬪簭鐨勮繑鍥炵爜鏄?BPF_OK锛堣〃绀鸿В鏋愭垚鍔燂級鎴?BPF_DROP锛堣〃绀鸿В鏋愰敊璇級銆?

## __sk_buff->data


鍦ㄦ棤 VLAN 鐨勬儏鍐典笅锛孊PF 娴佽В鏋愬櫒鐨勫垵濮嬬姸鎬佸涓嬶細
```

  +------+------+------------+-----------+
  | DMAC | SMAC | ETHER_TYPE | L3_HEADER |
  +------+------+------------+-----------+
                              ^
                              |
                              +-- flow dissector starts here


```

  skb->data + flow_keys->nhoff 鎸囧悜 L3_HEADER 鐨勭涓€涓瓧鑺?
  flow_keys->thoff = nhoff
  flow_keys->n_proto = ETHER_TYPE

鍦?VLAN 鐨勬儏鍐典笅锛屾祦瑙ｆ瀽鍣ㄥ彲鑳戒互涓ょ涓嶅悓鐨勭姸鎬佽璋冪敤銆?

```

  +------+------+------+-----+-----------+-----------+
  | DMAC | SMAC | TPID | TCI |ETHER_TYPE | L3_HEADER |
  +------+------+------+-----+-----------+-----------+
                        ^
                        |
                        +-- flow dissector starts here

```

  skb->data + flow_keys->nhoff 鎸囧悜 TCI 鐨勭涓€涓瓧鑺?
  flow_keys->thoff = nhoff
  flow_keys->n_proto = TPID

璇锋敞鎰?TPID 鍙互鏄?802.1AD锛屽洜姝?BPF 绋嬪簭瀵逛簬鍙屾爣绛撅紙double tagged锛夋暟鎹寘闇€瑕佽В鏋?VLAN
淇℃伅涓ゆ銆?


```

  +------+------+------+-----+-----------+-----------+
  | DMAC | SMAC | TPID | TCI |ETHER_TYPE | L3_HEADER |
  +------+------+------+-----+-----------+-----------+
                                          ^
                                          |
                                          +-- flow dissector starts here

```

  skb->data + flow_keys->nhoff 鎸囧悜 L3_HEADER 鐨勭涓€涓瓧鑺?
  flow_keys->thoff = nhoff
  flow_keys->n_proto = ETHER_TYPE

鍦ㄨ繖绉嶆儏鍐典笅锛孷LAN 淇℃伅鍦ㄦ祦瑙ｆ瀽鍣ㄤ箣鍓嶅凡缁忚澶勭悊锛孊PF 娴佽В鏋愬櫒涓嶉渶瑕佸鐞嗗畠銆?


杩欓噷鐨勮鐐瑰涓嬶細BPF 娴佽В鏋愬櫒绋嬪簭鍙兘甯︽湁鍙€夌殑 VLAN 澶撮儴琚皟鐢紝骞朵笖搴斿綋浼橀泤鍦板鐞嗕袱绉?
鎯呭喌锛氬瓨鍦ㄥ崟 VLAN 鎴栧弻 VLAN 浠ュ強涓嶅瓨鍦?VLAN 鐨勬儏鍐点€傚悓涓€涓▼搴忓彲鑳藉湪涓ょ鎯呭喌涓嬮兘琚皟鐢紝
鍥犳蹇呴』浠旂粏缂栧啓浠ュ鐞嗕袱绉嶆儏鍐点€?


## 鏍囧織


`flow_keys->flags` 鍙兘鍖呭惈鍙€夌殑杈撳叆鏍囧織锛屽叾浣滅敤濡備笅锛?

- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 鍛婅瘔 BPF 娴佽В鏋愬櫒缁х画瑙ｆ瀽绗竴涓垎鐗囷紱榛樿棰勬湡琛屼负鏄祦瑙ｆ瀽鍣ㄤ竴鏃﹀彂鐜版暟鎹寘琚?
  鍒嗙墖灏辩珛鍗宠繑鍥烇紱鐢?`BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` 鐢ㄤ簬涓?GRO 浼扮畻鎵€鏈夊ご閮ㄧ殑闀垮害銆?
- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 鍛婅瘔 BPF 娴佽В鏋愬櫒鍦ㄥ埌杈?IPv6 娴佹爣绛炬椂鍋滄瑙ｆ瀽锛涚敱 `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` 鐢ㄤ簬鑾峰彇娴佸搱甯屻€?
- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 鍛婅瘔 BPF 娴佽В鏋愬櫒鍦ㄥ埌杈惧皝瑁呭ご閮ㄦ椂鍋滄瑙ｆ瀽锛涚敱璺敱鍩虹璁炬柦浣跨敤銆?


## 鍙傝€冨疄鐜?


鍙傝 `tools/testing/selftests/bpf/flow_dissector_load.[hc]` 鑾峰彇鍙傝€冨疄鐜帮紝浠ュ強 `tools/testing/selftests/bpf/flow_dissector_load.[hc]` 鑾峰彇鍔犺浇鍣ㄣ€俠pftool 涔熷彲鐢ㄤ簬鍔犺浇 BPF 娴佽В鏋愬櫒绋嬪簭銆?

鍙傝€冨疄鐜扮殑缁勭粐鏂瑰紡濡備笅锛?
  - `bpf_tail_call` 鏄犲皠锛屽寘鍚瘡涓彈鏀寔 L3 鍗忚鐨勫瓙绋嬪簭
  - `bpf_tail_call` 渚嬬▼ - 鍏ュ彛鐐癸紱瀹冭繘琛岃緭鍏?`n_proto` 瑙ｆ瀽锛屽苟鍊熷姪 `bpf_tail_call` 鍒嗗彂鍒扮浉搴旂殑 L3 澶勭悊绋嬪簭

鐢变簬 BPF 鐩墠涓嶆敮鎸佸惊鐜紙鎴栦换浣曞洖璺筹級锛屾敼鐢?jmp_table 鏉ュ鐞嗗绾у皝瑁咃紙浠ュ強 IPv6 閫夐」锛夈€?


## 褰撳墠闄愬埗

BPF 娴佽В鏋愬櫒涓嶆敮鎸佸鍑哄唴鏍稿唴鍩轰簬 C 鐨勫疄鐜版墍鑳藉鍑虹殑鍏ㄩ儴鍏冩暟鎹€備竴涓樉钁楃殑渚嬪瓙鏄崟 VLAN
锛?02.1Q锛夊拰鍙?VLAN锛?02.1AD锛夋爣绛俱€傝鍙傝€?`struct bpf_flow_keys` 鑾峰彇褰撳墠鍙粠 BPF 涓婁笅鏂囧鍑虹殑淇℃伅闆嗗悎銆?

褰?BPF 娴佽В鏋愬櫒琚檮鍔犲埌鏍圭綉缁滃懡鍚嶇┖闂达紙machine-wide 绛栫暐锛夋椂锛岀敤鎴锋棤娉曞湪鍏跺瓙缃戠粶鍛藉悕绌洪棿涓?
瑕嗙洊瀹冦€?
