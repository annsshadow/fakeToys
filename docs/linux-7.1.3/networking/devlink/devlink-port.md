## Devlink 绔彛


`devlink-port` 鏄澶囦笂瀛樺湪鐨勭鍙ｃ€傚畠鍏锋湁閫昏緫涓婄嫭绔嬬殑璁惧鍏ュ彛/鍑哄彛鐐广€備竴涓?devlink 绔彛鍙互鏄绉嶇被鍨嬶紙flavour锛変腑鐨勪换鎰忎竴绉嶃€俤evlink 绔彛鐨勭被鍨嬶紙flavour锛夎繛鍚岀鍙ｅ睘鎬т竴璧锋弿杩颁簡璇ョ鍙ｄ唬琛ㄤ粈涔堛€?
鎵撶畻鍙戝竷涓€涓?devlink 绔彛鐨勮澶囬┍鍔ㄤ細璁剧疆璇?devlink 绔彛鐨勫睘鎬э紝骞舵敞鍐岃 devlink 绔彛銆?
Devlink 绔彛绫诲瀷锛坒lavour锛夋弿杩板涓嬨€?
   :widths: 33 90

   - - 绫诲瀷
     - 鎻忚堪
   - - `DEVLINK_PORT_FLAVOUR_PHYSICAL`
     - 浠绘剰绉嶇被鐨勭墿鐞嗙鍙ｃ€傚彲浠ユ槸 eswitch 鐗╃悊绔彛锛屾垨璁惧涓婄殑浠讳綍鍏朵粬鐗╃悊绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_DSA`
     - 琛ㄧず涓€涓?DSA 浜掕繛绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_CPU`
     - 琛ㄧず涓€涓粎閫傜敤浜?DSA 鐨?CPU 绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_PCI_PF`
     - 琛ㄧず涓€涓唬琛?PCI 鐗╃悊鍔熻兘锛圥F锛夌鍙ｇ殑 eswitch 绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_PCI_VF`
     - 琛ㄧず涓€涓唬琛?PCI 铏氭嫙鍔熻兘锛圴F锛夌鍙ｇ殑 eswitch 绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_PCI_SF`
     - 琛ㄧず涓€涓唬琛?PCI 瀛愬姛鑳斤紙SF锛夌鍙ｇ殑 eswitch 绔彛銆?   - - `DEVLINK_PORT_FLAVOUR_VIRTUAL`
     - 琛ㄧず涓€涓敤浜?PCI 铏氭嫙鍔熻兘鐨勮櫄鎷熺鍙ｃ€?
Devlink 绔彛鍙互鍩轰簬涓嬭堪閾捐矾灞傛嫢鏈変笉鍚岀殑绫诲瀷銆?
   :widths: 23 90

   - - 绫诲瀷
     - 鎻忚堪
   - - `DEVLINK_PORT_TYPE_ETH`
     - 褰撶鍙ｇ殑閾捐矾灞備负浠ュお缃戞椂锛岄┍鍔ㄥ簲璁剧疆姝ょ鍙ｇ被鍨嬨€?   - - `DEVLINK_PORT_TYPE_IB`
     - 褰撶鍙ｇ殑閾捐矾灞備负 InfiniBand 鏃讹紝椹卞姩搴旇缃绔彛绫诲瀷銆?   - - `DEVLINK_PORT_TYPE_AUTO`
     - 褰撶敤鎴峰笇鏈涢┍鍔ㄨ嚜鍔ㄦ娴嬬鍙ｇ被鍨嬫椂锛屾寚绀烘绫诲瀷銆?
### PCI 鎺у埗鍣?
鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝涓€涓?PCI 璁惧鍙湁涓€涓帶鍒跺櫒銆備竴涓帶鍒跺櫒鐢辨綔鍦ㄥ涓墿鐞嗗姛鑳姐€佽櫄鎷熷姛鑳戒互鍙婂瓙鍔熻兘缁勬垚銆備竴涓姛鑳界敱涓€涓垨澶氫釜绔彛缁勬垚銆傝绔彛鐢?devlink eswitch 绔彛琛ㄧず銆?
浣嗘槸锛岃繛鎺ュ埌澶氫釜 CPU銆佹垨澶氫釜 PCI 鏍瑰鍚堜綋銆佹垨涓€涓?SmartNIC 鐨?PCI 璁惧锛屽彲鑳芥嫢鏈夊涓帶鍒跺櫒銆傚浜庡叿鏈夊涓帶鍒跺櫒鐨勮澶囷紝姣忎釜鎺у埗鍣ㄩ€氳繃涓€涓敮涓€鐨勬帶鍒跺櫒缂栧彿鏉ュ尯鍒嗐€俥switch 浣嶄簬鏀寔澶氫釜鎺у埗鍣ㄧ鍙ｇ殑 PCI 璁惧涓娿€?
```
                 ---------------------------------------------------------
                 |                                                       |
                 |           --------- ---------         ------- ------- |
    -----------  |           | vf(s) | | sf(s) |         |vf(s)| |sf(s)| |
    | server  |  | -------   ----/---- ---/----- ------- ---/--- ---/--- |
    | pci rc  |=== | pf0 |______/________/       | pf1 |___/_______/     |
    | connect |  | -------                       -------                 |
    -----------  |     | controller_num=1 (no eswitch)                   |
                 ------|--------------------------------------------------
                 (internal wire)
                       |
                 ---------------------------------------------------------
                 | devlink eswitch ports and reps                        |
                 | ----------------------------------------------------- |
                 | |ctrl-0 | ctrl-0 | ctrl-0 | ctrl-0 | ctrl-0 |ctrl-0 | |
                 | |pf0    | pf0vfN | pf0sfN | pf1    | pf1vfN |pf1sfN | |
                 | ----------------------------------------------------- |
                 | |ctrl-1 | ctrl-1 | ctrl-1 | ctrl-1 | ctrl-1 |ctrl-1 | |
                 | |pf0    | pf0vfN | pf0sfN | pf1    | pf1vfN |pf1sfN | |
                 | ----------------------------------------------------- |
                 |                                                       |
                 |                                                       |
    -----------  |           --------- ---------         ------- ------- |
    | smartNIC|  |           | vf(s) | | sf(s) |         |vf(s)| |sf(s)| |
    | pci rc  |==| -------   ----/---- ---/----- ------- ---/--- ---/--- |
    | connect |  | | pf0 |______/________/       | pf1 |___/_______/     |
    -----------  | -------                       -------                 |
                 |                                                       |
                 |  local controller_num=0 (eswitch)                     |
                 ---------------------------------------------------------
```

鍦ㄤ笂杩扮ず渚嬩腑锛屽閮ㄦ帶鍒跺櫒锛堢敱 controller number = 1 鏍囪瘑锛夋病鏈?eswitch銆傛湰鍦版帶鍒跺櫒锛堢敱 controller number = 0 鏍囪瘑锛夋嫢鏈?eswitch銆傛湰鍦版帶鍒跺櫒涓婄殑 Devlink 瀹炰緥涓轰袱涓帶鍒跺櫒閮芥彁渚涗簡 eswitch devlink 绔彛銆?
## 鍔熻兘閰嶇疆

鐢ㄦ埛鍙互鍦ㄦ灇涓?PCI 鍔熻兘涔嬪墠閰嶇疆涓€涓垨澶氫釜鍔熻兘灞炴€с€傞€氬父杩欐剰鍛崇潃锛岀敤鎴峰簲褰撳湪涓鸿鍔熻兘鍒涘缓鐗瑰畾浜庢€荤嚎鐨勮澶囦箣鍓嶉厤缃姛鑳藉睘鎬с€備絾鏄紝褰撳惎鐢?SRIOV 鏃讹紝铏氭嫙鍔熻兘璁惧浼氬湪 PCI 鎬荤嚎涓婂垱寤恒€傚洜姝わ紝搴斿綋鍦ㄥ皢铏氭嫙鍔熻兘璁惧缁戝畾鍒伴┍鍔ㄤ箣鍓嶉厤缃姛鑳藉睘鎬с€傚浜庡瓙鍔熻兘锛岃繖鎰忓懗鐫€鐢ㄦ埛搴斿綋鍦ㄦ縺娲荤鍙ｅ姛鑳戒箣鍓嶉厤缃鍙ｅ姛鑳藉睘鎬с€?
鐢ㄦ埛鍙互浣跨敤 `devlink port function set hw_addr` 鍛戒护璁剧疆璇ュ姛鑳界殑纭欢鍦板潃銆傚浜庝互澶綉绔彛鍔熻兘锛岃繖琛ㄧず MAC 鍦板潃銆?
鐢ㄦ埛涔熷彲浠ヤ娇鐢?`devlink port function set roce` 鍛戒护璁剧疆璇ュ姛鑳界殑 RoCE 鑳藉姏銆?
鐢ㄦ埛涔熷彲浠ヤ娇鐢?`devlink port function set migratable` 鍛戒护灏嗚鍔熻兘璁剧疆涓哄彲杩佺Щ鐨勩€?
鐢ㄦ埛涔熷彲浠ヤ娇鐢?`devlink port function set ipsec_crypto` 鍛戒护璁剧疆璇ュ姛鑳界殑 IPsec crypto 鑳藉姏銆?
鐢ㄦ埛涔熷彲浠ヤ娇鐢?`devlink port function set ipsec_packet` 鍛戒护璁剧疆璇ュ姛鑳界殑 IPsec packet 鑳藉姏銆?
鐢ㄦ埛涔熷彲浠ヤ娇鐢?`devlink port function set max_io_eqs` 鍛戒护璁剧疆璇ュ姛鑳界殑鏈€澶?IO 浜嬩欢闃熷垪鏁般€?
## 鍔熻兘灞炴€?
### MAC 鍦板潃璁剧疆

涓?PCI VF/SF 閰嶇疆鐨?MAC 鍦板潃灏嗚涓鸿 PCI VF/SF 鍒涘缓鐨?netdevice 鍜?rdma 璁惧浣跨敤銆?
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
      function:
        hw_addr 00:00:00:00:00:00

```
```
    $ devlink port function set pci/0000:06:00.0/2 hw_addr 00:11:22:33:44:55

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
      function:
        hw_addr 00:11:22:33:44:55

```
```
    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:00:00

```
```
    $ devlink port function set pci/0000:06:00.0/32768 hw_addr 00:00:00:00:88:88

    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:88:88

```
### RoCE 鑳藉姏璁剧疆

骞堕潪鎵€鏈?PCI VF/SF 閮介渶瑕?RoCE 鑳藉姏銆?
褰撶鐢?RoCE 鑳藉姏鏃讹紝浼氫负姣忎釜 PCI VF/SF 鑺傜渷绯荤粺鍐呭瓨銆?
褰撶敤鎴蜂负鏌愪釜 VF/SF 绂佺敤 RoCE 鑳藉姏鏃讹紝鐢ㄦ埛搴旂敤绋嬪簭鏃犳硶閫氳繃璇?VF/SF 鍙戦€佹垨鎺ユ敹浠讳綍 RoCE 鏁版嵁鍖咃紝骞朵笖璇?PCI 鐨?RoCE GID 琛ㄥ皢涓虹┖銆?
褰撲娇鐢ㄧ鍙ｅ姛鑳藉睘鎬у湪璁惧涓鐢?RoCE 鑳藉姏鏃讹紝VF/SF 椹卞姩鏃犳硶瑕嗙洊瀹冦€?
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 roce enable

```
```
    $ devlink port function set pci/0000:06:00.0/2 roce disable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 roce disable

```
### 鍙縼绉昏兘鍔涜缃?
瀹炴椂杩佺Щ锛圠ive migration锛夋槸鍦ㄤ笉涓柇鍏舵甯歌繍琛岀殑鎯呭喌涓嬶紝灏嗘鍦ㄨ繍琛岀殑铏氭嫙鏈轰粠涓€鍙扮墿鐞嗕富鏈鸿浆绉诲埌鍙︿竴鍙扮墿鐞嗕富鏈虹殑杩囩▼銆?
甯屾湜 PCI VF 鑳藉鎵ц瀹炴椂杩佺Щ鐨勭敤鎴凤紝闇€瑕佹樉寮忓湴鍚敤 VF 鐨勫彲杩佺Щ鑳藉姏銆?
褰撶敤鎴蜂负 VF 鍚敤鍙縼绉昏兘鍔涳紝骞朵笖 HV 灏?VF 缁戝畾鍒版敮鎸佽縼绉荤殑 VFIO 椹卞姩鏃讹紝鐢ㄦ埛鍙互灏嗗甫鏈夎 VF 鐨勮櫄鎷熸満浠庝竴鍙?HV 杩佺Щ鍒板彟涓€鍙?HV銆?
浣嗘槸锛屽綋鍚敤鍙縼绉昏兘鍔涙椂锛岃澶囦細绂佺敤閭ｄ簺鏃犳硶杩佺Щ鐨勭壒鎬с€傚洜姝ゅ彲杩佺Щ鑳藉姏浼氬 VF 鏂藉姞闄愬埗锛岀敱鐢ㄦ埛鑷鍐冲畾銆?
浣跨敤鍙縼绉诲姛鑳介厤缃殑 LM 绀轰緥锛?```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 migratable disable

```
```
    $ devlink port function set pci/0000:06:00.0/2 migratable enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 migratable enable

```
```
    $ echo <pci_id> > /sys/bus/pci/devices/0000:08:00.0/driver/unbind
    $ echo mlx5_vfio_pci > /sys/bus/pci/devices/0000:08:00.0/driver_override
    $ echo <pci_id> > /sys/bus/pci/devices/0000:08:00.0/driver/bind

```
灏?VF 闄勫姞鍒拌櫄鎷熸満銆?鍚姩铏氭嫙鏈恒€?鎵ц瀹炴椂杩佺Щ銆?
### IPsec crypto 鑳藉姏璁剧疆

褰撶敤鎴蜂负 VF 鍚敤 IPsec crypto 鑳藉姏鏃讹紝鐢ㄦ埛搴旂敤绋嬪簭鍙互灏?XFRM 鐘舵€?crypto 鎿嶄綔锛堝姞瀵?瑙ｅ瘑锛夊嵏杞藉埌璇?VF銆?
褰?VF 鐨?IPsec crypto 鑳藉姏琚鐢紙榛樿锛夋椂锛孹FRM 鐘舵€佺敱鍐呮牳鍦ㄨ蒋浠朵腑澶勭悊銆?
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_crypto disabled

```
```
    $ devlink port function set pci/0000:06:00.0/2 ipsec_crypto enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_crypto enabled

```
### IPsec packet 鑳藉姏璁剧疆

褰撶敤鎴蜂负 VF 鍚敤 IPsec packet 鑳藉姏鏃讹紝鐢ㄦ埛搴旂敤绋嬪簭鍙互灏?XFRM 鐘舵€佸拰绛栫暐 crypto 鎿嶄綔锛堝姞瀵?瑙ｅ瘑锛変互鍙?IPsec 灏佽鍗歌浇鍒拌 VF銆?
褰?VF 鐨?IPsec packet 鑳藉姏琚鐢紙榛樿锛夋椂锛孹FRM 鐘舵€佸拰绛栫暐鐢卞唴鏍稿湪杞欢涓鐞嗐€?
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled

```
```
    $ devlink port function set pci/0000:06:00.0/2 ipsec_packet enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet enabled

```
### 鏈€澶?IO 浜嬩欢闃熷垪璁剧疆

褰撶敤鎴蜂负 SF 鎴?VF 璁剧疆鏈€澶?IO 浜嬩欢闃熷垪鏁版椂锛岃鍔熻兘椹卞姩琚檺鍒朵负鍙兘娑堣€楁墍寮哄埗瑙勫畾鐨?IO 浜嬩欢闃熷垪鏁般€?
IO 浜嬩欢闃熷垪浼犻€掍笌 IO 闃熷垪鐩稿叧鐨勪簨浠讹紝鍖呮嫭缃戠粶璁惧鍙戦€佸拰鎺ユ敹闃熷垪锛坱xq 鍜?rxq锛変互鍙?RDMA 闃熷垪瀵癸紙QP锛夈€備緥濡傦紝netdevice 閫氶亾鏁板拰 RDMA 璁惧瀹屾垚鍚戦噺鐨勬暟閲忛兘娲剧敓鑷鍔熻兘鐨?IO 浜嬩欢闃熷垪銆傞€氬父锛岄┍鍔ㄦ秷鑰楃殑缁堢鍚戦噺鏁伴噺鍙楁瘡涓澶囩殑 IO 浜嬩欢闃熷垪鏁伴噺闄愬埗锛屽洜涓烘瘡涓?IO 浜嬩欢闃熷垪閮借繛鎺ュ埌涓€涓腑鏂悜閲忋€?
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled max_io_eqs 10

```
```
    $ devlink port function set pci/0000:06:00.0/2 max_io_eqs 32

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled max_io_eqs 32

```
## 瀛愬姛鑳斤紙Subfunction锛?

瀛愬姛鑳斤紙Subfunction锛夋槸涓€绉嶈交閲忕骇鍔熻兘锛屽畠閮ㄧ讲鍦ㄤ竴涓埗 PCI 鍔熻兘涔嬩笂銆傚瓙鍔熻兘浠?1 涓哄崟浣嶅垱寤哄拰閮ㄧ讲銆備笌 SRIOV VF 涓嶅悓锛屽瓙鍔熻兘涓嶉渶瑕佽嚜宸辩殑 PCI 铏氭嫙鍔熻兘銆傚瓙鍔熻兘閫氳繃鐖?PCI 鍔熻兘涓庣‖浠堕€氫俊銆?
瑕佷娇鐢ㄥ瓙鍔熻兘锛岄渶瑕侀伒寰?3 姝ヨ缃祦绋嬶細

1) 鍒涘缓锛坈reate锛夆€斺€斿垱寤轰竴涓瓙鍔熻兘锛?2) 閰嶇疆锛坈onfigure锛夆€斺€旈厤缃瓙鍔熻兘灞炴€э紱
3) 閮ㄧ讲锛坉eploy锛夆€斺€旈儴缃茶瀛愬姛鑳斤紱

瀛愬姛鑳界鐞嗛€氳繃 devlink 绔彛鐢ㄦ埛鐣岄潰瀹屾垚銆傜敤鎴峰湪瀛愬姛鑳界鐞嗚澶囦笂鎵ц璁剧疆銆?
### (1) 鍒涘缓

瀛愬姛鑳介€氳繃 devlink 绔彛鎺ュ彛鍒涘缓銆傜敤鎴烽€氳繃娣诲姞涓€涓瓙鍔熻兘绫诲瀷鐨?devlink 绔彛鏉ユ坊鍔犲瓙鍔熻兘銆俤evlink 鍐呮牳浠ｇ爜鍚戜笅璋冪敤瀛愬姛鑳界鐞嗛┍鍔紙devlink ops锛夛紝骞惰姹傚畠鍒涘缓涓€涓瓙鍔熻兘 devlink 绔彛銆傜劧鍚庨┍鍔ㄥ疄渚嬪寲璇ュ瓙鍔熻兘绔彛浠ュ強浠讳綍鍏宠仈鐨勫璞★紝渚嬪鍋ュ悍鎶ュ憡鍣紙health reporter锛夊拰浠ｈ〃锛坮epresentor锛?netdevice銆?
### (2) 閰嶇疆

宸插垱寤哄瓙鍔熻兘 devlink 绔彛锛屼絾瀹冨皻鏈縺娲汇€傝繖鎰忓懗鐫€瀹炰綋宸插湪 devlink 涓€渚у垱寤猴紝e-switch 绔彛浠ｈ〃锛坮epresentor锛変篃宸插垱寤猴紝浣嗗瓙鍔熻兘璁惧鏈韩灏氭湭鍒涘缓銆傜敤鎴峰彲浠ヤ娇鐢?e-switch 绔彛浠ｈ〃杩涜璁剧疆銆佸皢鍏跺姞鍏ョ綉妗ャ€佹坊鍔?TC 瑙勫垯绛夈€傜敤鎴蜂篃鍙互鍦ㄥ瓙鍔熻兘澶勪簬闈炴椿鍔ㄧ姸鎬佹椂閰嶇疆鍏剁‖浠跺湴鍧€锛堜緥濡?MAC 鍦板潃锛夈€?
### (3) 閮ㄧ讲

涓€鏃﹀瓙鍔熻兘閰嶇疆瀹屾垚锛岀敤鎴峰繀椤绘縺娲诲畠鎵嶈兘浣跨敤瀹冦€傛縺娲绘椂锛屽瓙鍔熻兘绠＄悊椹卞姩浼氳姹傚瓙鍔熻兘绠＄悊璁惧鍦ㄧ壒瀹?PCI 鍔熻兘涓婂疄渚嬪寲瀛愬姛鑳借澶囥€傚瓙鍔熻兘璁惧鍦?Documentation/driver-api/auxiliary_bus.rst <auxiliary_bus> 涓婂垱寤恒€傛鏃讹紝涓€涓尮閰嶇殑瀛愬姛鑳介┍鍔ㄤ細缁戝畾鍒拌瀛愬姛鑳界殑杈呭姪璁惧銆?
## 閫熺巼瀵硅薄绠＄悊

Devlink 鎻愪緵鐢ㄤ簬绠＄悊鍗曚釜 devlink 绔彛鎴栦竴缁勭鍙ｇ殑 tx 閫熺巼鐨?API銆傝繖鏄€氳繃閫熺巼瀵硅薄瀹屾垚鐨勶紝閫熺巼瀵硅薄鍙互鏄互涓嬩袱绉嶇被鍨嬩箣涓€锛?
`leaf`
  浠ｈ〃鍗曚釜 devlink 绔彛锛涚敱椹卞姩鍒涘缓/閿€姣併€傜敱浜?leaf 涓庡叾 devlink 绔彛鏄?1 瀵?1 鏄犲皠锛屽湪鐢ㄦ埛绌洪棿涓畠琚О涓?`pci/<bus_addr>/<port_index>`锛?
`node`
  浠ｈ〃涓€缁勯€熺巼瀵硅薄锛坙eaf 鍜?鎴?node锛夛紱鐢辨潵鑷敤鎴风┖闂寸殑璇锋眰鍒涘缓/鍒犻櫎锛涙渶鍒濅负绌猴紙鏈坊鍔犱换浣曢€熺巼瀵硅薄锛夈€傚湪鐢ㄦ埛绌洪棿涓畠琚О涓?`pci/<bus_addr>/<node_name>`锛屽叾涓?`node_name` 鍙互鏄櫎鍗佽繘鍒舵暟瀛椾箣澶栫殑浠讳綍鏍囪瘑绗︼紝浠ラ伩鍏嶄笌 leaf 鍐茬獊銆?
API 鍏佽閰嶇疆浠ヤ笅閫熺巼瀵硅薄鍙傛暟锛?
`tx_share`
  鍦ㄦ墍鏈夊叾浠栭€熺巼瀵硅薄涔嬮棿鍏变韩鐨勬渶灏?TX 閫熺巼鍊硷紱濡傛灉瀹冩槸鍚屼竴缁勭殑涓€閮ㄥ垎锛屽垯鏄埗缁勭殑閫熺巼瀵硅薄鐨勪竴閮ㄥ垎銆?
`tx_max`
  鏈€澶?TX 閫熺巼鍊笺€?
`tx_priority`
  鍏佽鍦ㄥ厔寮熻妭鐐逛箣闂翠娇鐢ㄤ弗鏍间紭鍏堢骇浠茶鍣ㄣ€傝浠茶鏂规灏濊瘯鏍规嵁鑺傜偣鐨勪紭鍏堢骇鏉ヨ皟搴﹁妭鐐癸紝鍙鑺傜偣浠嶅湪鍏跺甫瀹介檺鍒跺唴銆備紭鍏堢骇瓒婇珮锛岃鑺傜偣琚€変腑杩涜璋冨害鐨勬鐜囧氨瓒婂ぇ銆?
`tx_weight`
  鍏佽鍦ㄥ厔寮熻妭鐐逛箣闂翠娇鐢ㄦ潈閲嶅叕骞虫帓闃燂紙Weighted Fair Queuing锛変徊瑁佹柟妗堛€傝浠茶鏂规鍙互涓庝弗鏍间紭鍏堢骇鍚屾椂浣跨敤銆傚綋鑺傜偣閰嶇疆鏈夋洿楂樼殑閫熺巼鏃讹紝瀹冪浉瀵逛簬鍏跺厔寮熻妭鐐硅幏寰楁洿澶氱殑甯﹀锛圔W锛夈€傚€煎氨鍍忕櫨鍒嗘瘮鐐逛竴鏍锋槸鐩稿鐨勶紝瀹冧滑鍩烘湰涓婂憡璇夎妭鐐圭浉瀵逛簬鍏跺厔寮熻妭鐐瑰簲鑾峰彇澶氬皯甯﹀銆?
`parent`
  鐖惰妭鐐瑰悕绉般€傜埗鑺傜偣閫熺巼闄愬埗琚涓哄鍏舵墍鏈夊瓙鑺傜偣闄愬埗鐨勯澶栭檺鍒躲€俙tx_max` 鏄瓙鑺傜偣鐨勪笂闄愩€俙tx_share` 鏄湪瀛愯妭鐐逛箣闂村垎閰嶇殑鎬诲甫瀹姐€?
`tc_bw`
  鍏佽鐢ㄦ埛璁剧疆閫熺巼瀵硅薄涓婃瘡涓祦閲忕被鐨勫甫瀹藉垎閰嶃€傝繖閫氳繃瀵规瘡涓祦閲忕被鍒嗛厤涓€涓浉瀵逛唤棰濆€硷紝瀹炵幇浜嗙粏绮掑害鐨?QoS 閰嶇疆銆傚甫瀹芥寜鐓ф瘡涓被鐨勪唤棰濆€肩浉瀵逛簬鎵€鏈変唤棰濅箣鍜岀殑姣斾緥杩涜鍒嗛厤銆傚綋搴旂敤浜庨潪鍙跺瓙鑺傜偣鏃讹紝tc_bw 鍐冲畾浜嗗叾鍚勪釜瀛愬厓绱犱箣闂村浣曞叡浜甫瀹姐€?
`tx_priority` 鍜?`tx_weight` 鍙互鍚屾椂浣跨敤銆傚湪杩欑鎯呭喌涓嬶紝鍏锋湁鐩稿悓浼樺厛绾х殑鑺傜偣鍦ㄥ厔寮熺粍涓舰鎴愪竴涓?WFQ 瀛愮粍锛屽畠浠箣闂寸殑浠茶鍩轰簬鎵€鍒嗛厤鐨勬潈閲嶃€?
浠庨珮灞傛鐪嬶紝浠茶娴佺▼濡備笅锛?
#. 閫夋嫨涓€涓紭鍏堢骇鏈€楂樸€佸浜庡甫瀹介檺鍒跺唴涓旀湭琚樆濉炵殑鑺傜偣鎴栬妭鐐圭粍銆備娇鐢?`tx_priority` 浣滀负姝や徊瑁佺殑鍙傛暟銆?
#. 濡傛灉涓€缁勮妭鐐瑰叿鏈夌浉鍚岀殑浼樺厛绾э紝鍒欏湪璇ュ瓙缁勪笂鎵ц WFQ 浠茶銆備娇鐢?`tx_weight` 浣滀负姝や徊瑁佺殑鍙傛暟銆?
#. 閫夊嚭鑾疯儨鑺傜偣锛屽苟缁х画鍦ㄥ叾瀛愯妭鐐归棿杩涜浠茶锛岀洿鍒板埌杈惧彾瀛愯妭鐐癸紝浠庤€岀‘瀹氳幏鑳滆€呫€?
#. 濡傛灉鏈€楂樹紭鍏堢骇瀛愮粍涓殑鎵€鏈夎妭鐐归兘宸叉弧瓒虫垨瓒呭嚭鍏跺垎閰嶇殑甯﹀锛屽垯杞悜杈冧綆浼樺厛绾х殑鑺傜偣銆?
椹卞姩瀹炵幇鍏佽鏀寔涓ょ閫熺巼瀵硅薄绫诲瀷涔嬩竴鎴栦袱鑰咃紝浠ュ強瀹冧滑鍙傛暟鐨勮缃柟娉曘€傛澶栵紝椹卞姩瀹炵幇鍙互瀵煎嚭 node/leaf 鍙婂叾鐖跺瓙鍏崇郴銆?
## 鏈涓庡畾涔?

   :widths: 22 90

   - - 鏈
     - 瀹氫箟
   - - `PCI device`
     - 涓€涓墿鐞?PCI 璁惧锛屽叿鏈変竴涓垨澶氫釜鐢?PCI 鎬荤嚎缁勬垚鐨?PCI 鎺у埗鍣ㄣ€?   - - `PCI controller`
     - 涓€涓帶鍒跺櫒鐢辨綔鍦ㄥ涓墿鐞嗗姛鑳姐€佽櫄鎷熷姛鑳戒互鍙婂瓙鍔熻兘缁勬垚銆?   - - `Port function`
     - 鐢ㄤ簬绠＄悊绔彛鍔熻兘鐨勫璞°€?   - - `Subfunction`
     - 涓€绉嶈交閲忕骇鍔熻兘锛岄儴缃插湪鐖?PCI 鍔熻兘涔嬩笂銆?   - - `Subfunction device`
     - 瀛愬姛鑳界殑鎬荤嚎璁惧锛岄€氬父浣嶄簬杈呭姪鎬荤嚎涓娿€?   - - `Subfunction driver`
     - 瀛愬姛鑳借緟鍔╄澶囩殑璁惧椹卞姩銆?   - - `Subfunction management device`
     - 鏀寔瀛愬姛鑳界鐞嗙殑 PCI 鐗╃悊鍔熻兘銆?   - - `Subfunction management driver`
     - 鏀寔浣跨敤 devlink 绔彛鎺ュ彛杩涜瀛愬姛鑳界鐞嗙殑 PCI 鐗╃悊鍔熻兘鐨勮澶囬┍鍔ㄣ€?   - - `Subfunction host driver`
     - 鎵胯浇瀛愬姛鑳借澶囩殑 PCI 鐗╃悊鍔熻兘鐨勮澶囬┍鍔ㄣ€傚湪澶у鏁版儏鍐典笅瀹冧笌瀛愬姛鑳界鐞嗛┍鍔ㄧ浉鍚屻€傚綋瀛愬姛鑳界敤浜庡閮ㄦ帶鍒跺櫒鏃讹紝瀛愬姛鑳界鐞嗗拰瀹夸富椹卞姩鏄笉鍚岀殑銆?