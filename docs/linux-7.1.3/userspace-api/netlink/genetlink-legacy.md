
## 鏃х増 Generic Netlink 鏃忕殑 Netlink 瑙勮寖鏀寔


鏈枃妗ｆ弿杩颁簡鎻忚堪杈冭€佺殑 Generic Netlink 鏃忥紙鏋勬垚 `genetlink-legacy` 鍗忚灞傜骇锛夋墍闇€鐨勮澶氶澶栫壒鎬т笌灞炴€с€?
## 瑙勮寖


### 鍏ㄥ眬灞炴€э紙Globals锛?

鍦ㄨ鑼冩枃浠舵牴灞傜骇鐩存帴鍒楀嚭鐨勫睘鎬с€?
#### version


Generic Netlink 鏃忕増鏈紝榛樿鍊间负 1銆?
`version` 鍦ㄥ巻鍙蹭笂鐢ㄤ簬寮曞叆鍙兘浼氱牬鍧忓悜鍚庡吋瀹规€х殑鏃忓彉鏇淬€傜敱浜庨€氬父涓嶅厑璁哥牬鍧忓吋瀹规€х殑鍙樻洿锛屽洜姝?`version` 寰堝皯琚娇鐢ㄣ€?
### 灞炴€х被鍨嬪祵濂楋紙Attribute type nests锛?

鏂扮殑 Netlink 鏃忓簲浣跨敤 `multi-attr` 鏉ュ畾涔夋暟缁勩€傝緝鑰佺殑鏃忥紙渚嬪 `genetlink` 鎺у埗鏃忥級灏濊瘯澶嶇敤灞炴€х被鍨嬫潵鎼哄甫鏁扮粍绫诲瀷淇℃伅銆?
```

  [ARRAY-ATTR]
    [INDEX (optionally)]
    [MEMBER1]
    [MEMBER2]
  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [INDEX (optionally)]
    [MEMBER1]
    [MEMBER2]

```
鍏朵腑 `ARRAY-ATTR` 鏄暟缁勬潯鐩被鍨嬨€?
#### indexed-array


`indexed-array` 灏嗘暣涓暟缁勫寘瑁瑰湪涓€涓澶栫殑灞炴€т腑锛堝洜姝ゅ叾澶у皬琚檺鍒朵负 64kB锛夈€俙ENTRY` 宓屽鏄壒娈婄殑锛屽叾绫诲瀷涓烘潯鐩殑绱㈠紩锛岃€屼笉鏄櫘閫氱殑灞炴€х被鍨嬨€?
闇€瑕佷竴涓?`sub-type` 鏉ユ弿杩?`ENTRY` 涓殑绫诲瀷銆俙nest` 杩欑 `sub-type` 琛ㄧず `ENTRY` 涓寘鍚祵濂楁暟缁勶紝鍏剁粨鏋勫涓嬶細

```

  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [ENTRY]
      [MEMBER1]
      [MEMBER2]
    [ENTRY]
      [MEMBER1]
      [MEMBER2]

```
鍏朵粬 `sub-type`锛堝 `u32`锛夎〃绀哄彧鏈変竴涓垚鍛橈紝濡備笅鎵€绀猴細

```

  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [ENTRY u32]
    [ENTRY u32]

```
#### type-value


`type-value` 鏄竴绉嶅埄鐢ㄥ睘鎬х被鍨嬫潵鎼哄甫鍗曚釜瀵硅薄淇℃伅鐨勬瀯閫狅紙甯哥敤浜庨€愭潯杞偍鏁扮粍鏉＄洰鏃讹級銆?
`type-value` 鍙互鏈夊灞傚祵濂楋紝渚嬪锛?
```

  [POLICY-IDX]
    [ATTR-IDX]
      [POLICY-INFO-ATTR1]
      [POLICY-INFO-ATTR2]

```
鍏朵腑绗竴灞傚祵濂椾互绛栫暐绱㈠紩浣滀负鍏跺睘鎬х被鍨嬶紝瀹冨寘鍚竴涓崟鐙殑宓屽锛岃宓屽浠ュ睘鎬х储寮曚綔涓哄叾绫诲瀷銆傚湪灞炴€х储寮曞祵濂楀唴閮ㄦ槸绛栫暐灞炴€с€傜幇浠ｇ殑 Netlink 鏃忔湰搴斿皢鍏跺畾涔変负鎵佸钩缁撴瀯锛岃繖閲岀殑宓屽娌℃湁浠讳綍濂藉銆?
## 鎿嶄綔


### 鏋氫妇锛堟秷鎭?ID锛夋ā鍨?

#### unified


鐜颁唬鏃忎娇鐢?`unified` 娑堟伅 ID 妯″瀷锛屽嵆鏃忓唴鎵€鏈夋秷鎭娇鐢ㄥ崟涓€鏋氫妇銆傝姹備笌鍝嶅簲鍏变韩鍚屼竴涓秷鎭?ID銆傞€氱煡浣跨敤鏉ヨ嚜鍚屼竴绌洪棿鐨勭嫭绔?ID銆備緥濡傜粰瀹氫互涓嬫搷浣滃垪琛細


  -
    name: a
    value: 1
    do: ...
  -
    name: b
    do: ...
  -
    name: c
    value: 4
    notify: a
  -
    name: d
    do: ...

鎿嶄綔 `a` 鐨勮姹傚拰鍝嶅簲灏嗕娇鐢?ID 1锛屾搷浣?`b` 鐨勮姹傚拰鍝嶅簲浣跨敤 2锛堢敱浜庢病鏈夋樉寮忕殑 `value`锛屽叾 ID 涓哄墠涓€鎿嶄綔 `+ 1`锛夈€傞€氱煡 `c` 灏嗕娇鐢?ID 4锛屾搷浣?`d` 浣跨敤 5锛屼緷姝ょ被鎺ㄣ€?
#### directional


`directional` 妯″瀷鎸夌収娑堟伅鐨勬柟鍚戞潵鍒嗛厤 ID銆傛潵鑷唴鏍稿拰鍙戝線鍐呮牳鐨勬秷鎭笉浼氱浉浜掓贩娣嗭紝鍥犳杩欑鏂瑰紡鑺傜渷浜?ID 绌洪棿锛堜唬浠锋槸浣跨紪绋嬫洿鍔犵箒鐞愶級銆?
鍦ㄨ繖绉嶆儏鍐典笅锛宍value` 灞炴€у簲鍦ㄦ搷浣滅殑 `request` 鍜?`reply` 娈典腑鎸囧畾锛堝鏋滀竴涓搷浣滃悓鏃舵湁 `do` 鍜?`dump`锛屽垯 ID 鏄叡浜殑锛宍value` 搴斿湪 `do` 涓缃級銆傚浜庨€氱煡锛宍value` 鍦?op 灞傜骇鎻愪緵锛屼絾瀹冨彧鍒嗛厤涓€涓?`reply`锛堝嵆鈥滄潵鑷唴鏍糕€濈殑 ID锛夈€傛潵鐪嬩竴涓緥瀛愶細


  -
    name: a
    do:
      request:
        value: 2
        attributes: ...
      reply:
        value: 1
        attributes: ...
  -
    name: b
    notify: a
  -
    name: c
    notify: a
    value: 7
  -
    name: d
    do: ...

鍦ㄨ繖绉嶆儏鍐典笅锛宍a` 鍦ㄥ彂閫佹秷鎭粰鍐呮牳鏃朵娇鐢?2锛屽苟鏈熸湜鏀跺埌 ID 涓?1 鐨勫搷搴斻€傞€氱煡 `b` 鍒嗛厤涓€涓€滄潵鑷唴鏍糕€濈殑 ID锛屽€间负 2銆俙c` 鍒嗛厤鈥滄潵鑷唴鏍糕€濈殑 ID 7銆傚鏋滄搷浣?`d` 娌℃湁鍦ㄨ鑼冧腑鏄惧紡璁剧疆 `values`锛屽垯浼氫负璇锋眰鍒嗛厤 3锛坄a` 鏄墠涓€涓甫 request 娈点€乿alue 涓?2 鐨勬搷浣滐級锛屼负鍝嶅簲鍒嗛厤 8锛坄c` 鏄€滄潵鑷唴鏍糕€濇柟鍚戜笂鐨勫墠涓€涓搷浣滐級銆?
## 鍏朵粬鐗规€?

### 缁撴瀯浣擄紙Structures锛?

鏃х増鏃忓彲浠ュ畾涔?C 缁撴瀯浣擄紝鏃㈢敤浣滃睘鎬х殑鍐呭锛屼篃鐢ㄤ綔鍥哄畾鐨勬秷鎭ご銆傜粨鏋勪綋鍦?`definitions` 涓畾涔夛紝骞跺湪鎿嶄綔鎴栧睘鎬т腑寮曠敤銆?
#### members


 - `name` - 缁撴瀯浣撴垚鍛樼殑灞炴€у悕
 - `type` - 鏍囬噺绫诲瀷涔嬩竴锛歚u8`銆乣u16`銆乣u32`銆乣u64`銆乣s8`銆乣s16`銆乣s32`銆乣s64`銆乣string`銆乣binary` 鎴?`bitfield32`
 - `byte-order` - `big-endian` 鎴?`little-endian`
 - `doc`銆乣enum`銆乣enum-as-flags`銆乣display-hint` - 涓庡睘鎬у畾涔?<attribute_properties> 鐩稿悓

娉ㄦ剰锛孻AML 涓畾涔夌殑缁撴瀯浣撴寜鐓?C 绾﹀畾闅愬紡鍦扮揣鍑戞帓鍒楋紙packed锛夈€備緥濡傦紝涓嬮潰鐨勭粨鏋勪綋鏄?4 瀛楄妭锛岃€屼笉鏄?6 瀛楄妭锛?

  struct {
          u8 a;
          u16 b;
          u8 c;
  }

浠讳綍濉厖閮藉繀椤绘樉寮忔坊鍔狅紝绫?C 璇█搴旀牴鎹垚鍛樻槸鍚﹁嚜鐒跺榻愭潵鎺ㄦ柇鏄惁闇€瑕佹樉寮忓～鍏呫€?
涓嬮潰鏄笂闈㈢粨鏋勪綋鐨?YAML 瀹氫箟锛?

  definitions:
    -
      name: message-header
      type: struct
      members:
        -
          name: a
          type: u8
        -
          name: b
          type: u16
        -
          name: c
          type: u8

#### Fixed Headers


鍥哄畾鐨勬秷鎭ご鍙互閫氳繃 `fixed-header` 娣诲姞鍒版搷浣滀腑銆俙fixed-header` 鐨勯粯璁ゅ€煎彲浠ュ湪 `operations` 涓缃紝涔熷彲浠ヤ负姣忎釜鎿嶄綔璁剧疆鎴栬鐩栥€?

  operations:
    fixed-header: message-header
    list:
      -
        name: get
        fixed-header: custom-header
        attribute-set: message-attrs

#### Attributes


`binary` 灞炴€у彲浠ラ€氳繃甯︽湁缁撴瀯浣撳畾涔夊悕绉扮殑 `struct` 灞炴€цВ閲婁负 C 缁撴瀯浣撱€俙struct` 灞炴€ч殣鍚?`sub-type: struct`锛屽洜姝ゆ棤闇€鍐嶆寚瀹氬瓙绫诲瀷銆?

  attribute-sets:
    -
      name: stats-attrs
      attributes:
        -
          name: stats
          type: binary
          struct: vport-stats

### C Arrays


鏃х増鏃忎篃浣跨敤 `binary` 灞炴€ф潵灏佽 C 鏁扮粍銆俙sub-type` 鐢ㄤ簬鏍囪瘑瑕佹彁鍙栫殑鏍囬噺绫诲瀷銆?

  attributes:
    -
      name: ports
      type: binary
      sub-type: u32

### Multi-message DO


鏂扮殑 Netlink 鏃忕粷涓嶅簲鍦ㄥ搷搴?DO 鎿嶄綔鏃惰缃?`NLM_F_MULTI` 骞惰繑鍥炲涓洖澶嶃€傚簲鏀圭敤杩囨护杞偍锛坒iltered dump锛夈€?
鍦ㄨ鑼冨眰闈紝鎴戜滑鍙互涓?`do` 瀹氫箟涓€涓?`dumps` 灞炴€э紝鍏跺€煎彲鑳戒负 `combine` 鍜?`multi-object`锛屽叿浣撳彇鍐充簬瑙ｆ瀽搴斿浣曞疄鐜帮紙瑙ｆ瀽涓哄崟涓洖澶嶏紝鎴栬В鏋愪负瀵硅薄鍒楄〃锛屽嵆鍑犱箮绛夊悓浜庝竴娆¤浆鍌級銆?