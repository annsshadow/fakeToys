
## 缃戠粶娴佸鐞嗗櫒锛圢etwork Flow Processor锛孨FP锛夊唴鏍搁┍鍔?
:Copyright: |copy| 2019, Netronome Systems, Inc.
:Copyright: |copy| 2022, Corigine, Inc.

## 鐩綍

- `姒傝堪`_
- `鑾峰彇鍥轰欢`_
- `Devlink 淇℃伅`_
- `閰嶇疆璁惧`_
- `缁熻淇℃伅`_

## 姒傝堪

鏈┍鍔ㄦ敮鎸?Netronome 鍜?Corigine 鐨勭郴鍒楃綉缁滄祦澶勭悊鍣紙Network Flow Processor锛夎澶囷紝
鍖呮嫭 NFP3800銆丯FP4000銆丯FP5000 鍜?NFP6000 鍨嬪彿锛岃繖浜涜澶囦篃琚泦鎴愬埌璇ュ叕鍙哥殑 Agilio
SmartNIC 绯诲垪涓€傞┍鍔ㄦ敮鎸佽繖浜涜澶囩殑 SR-IOV 鐗╃悊鍔熻兘涓庤櫄鎷熷姛鑳姐€?
## 鑾峰彇鍥轰欢

NFP3800銆丯FP4000 鍜?NFP6000 璁惧闇€瑕佺壒瀹氱殑搴旂敤鍥轰欢鎵嶈兘宸ヤ綔銆傚簲鐢ㄥ浐浠跺彲浠ヤ綅浜庝富鏈?鏂囦欢绯荤粺涓婏紝涔熷彲浠ヤ綅浜庤澶囬棯瀛樹腑锛堝墠鎻愭槸绠＄悊鍥轰欢鏀寔锛夈€?
涓绘満鏂囦欢绯荤粺涓婄殑鍥轰欢鏂囦欢鍖呭惈鍗＄被鍨嬶紙`AMDA-*` 瀛楃涓诧級銆佷粙璐ㄩ厤缃瓑淇℃伅銆傝嫢瑕佷粠涓绘満
鏂囦欢绯荤粺鍔犺浇鍥轰欢锛屽簲灏嗗叾鏀惧湪 `/lib/firmware/netronome` 鐩綍涓€?
鐢ㄤ簬鍩烘湰 NIC 鎿嶄綔鐨勫浐浠跺彲鍦ㄤ笂娓哥殑 `linux-firmware.git` 浠撳簱涓幏鍙栥€?
鏇村畬鏁寸殑鍥轰欢鍒楄〃鍙粠 `Corigine 鏀寔绔欑偣 <https://www.corigine.com/DPUDownload.html>`_
涓嬭浇銆?
### 闂瓨涓殑鍥轰欢

杩戞湡鐗堟湰鐨勭鐞嗗浐浠舵敮鎸佸湪涓绘満椹卞姩琚帰娴嬶紙probe锛夋椂浠庨棯瀛樺姞杞藉簲鐢ㄥ浐浠躲€傚彲浠ヤ娇鐢ㄥ浐浠?鍔犺浇绛栫暐閰嶇疆鏉ユ伆褰撳湴閰嶇疆姝ゅ姛鑳姐€?
鍙互浣跨敤 Devlink 鎴?ethtool锛岄€氳繃鍚戠浉搴斿懡浠ゆ彁渚涘悎閫傜殑 `nic_AMDA*.nffw` 鏂囦欢鏉ユ洿鏂?璁惧闂瓨涓婄殑搴旂敤鍥轰欢銆傜敤鎴烽渶瑕佹敞鎰忓悜闂瓨鍐欏叆涓庡崱鍜屼粙璐ㄩ厤缃浉鍖归厤鐨勬纭浐浠舵槧鍍忋€?
闂瓨涓彲鐢ㄧ殑瀛樺偍绌洪棿鍙栧喅浜庢墍浣跨敤鐨勫崱銆?
### 澶勭悊澶氫釜椤圭洰

NFP 纭欢鏄畬鍏ㄥ彲缂栫▼鐨勶紝鍥犳鍙兘瀛樺湪闈㈠悜涓嶅悓搴旂敤鐨勪笉鍚屽浐浠舵槧鍍忋€?
褰撲娇鐢ㄦ潵鑷富鏈轰笂鐨勫簲鐢ㄥ浐浠舵椂锛屾垜浠缓璁皢瀹為檯鐨勫浐浠舵枃浠舵斁鍦ㄤ互搴旂敤鍛藉悕鐨勫瓙鐩綍涓紝
渚嬪锛?
```
    $ tree /lib/firmware/netronome/
    /lib/firmware/netronome/
    鈹溾攢鈹€ bpf
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ flower
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ nic
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw -> bpf/nic_AMDA0081-0001_1x40.nffw
    鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw -> bpf/nic_AMDA0081-0001_4x10.nffw

    3 directories, 8 files

```
鍦ㄤ娇鐢ㄦ棫鐨?`mkinitrd` 鍛戒护鑰岄潪 `dracut`锛堜緥濡?Ubuntu锛夌殑鍙戣鐗堜笂锛屽彲鑳介渶瑕佷娇鐢ㄧ‖閾炬帴
鑰岄潪绗﹀彿閾炬帴銆?
鏇存敼鍥轰欢鏂囦欢鍚庯紝鍙兘闇€瑕侀噸鏂扮敓鎴?initramfs 鏄犲儚銆俰nitramfs 鍖呭惈绯荤粺鍚姩鍙兘闇€瑕佺殑
椹卞姩鍜屽浐浠舵枃浠躲€傝鍙傝€冧綘鐨勫彂琛岀増鏂囨。浠ヤ簡瑙ｅ浣曟洿鏂?initramfs銆俰nitramfs 杩囨椂鐨勪竴涓?鏄庢樉杩硅薄鏄細绯荤粺鍚姩鏃跺姞杞戒簡閿欒鐨勯┍鍔ㄦ垨鍥轰欢锛屼絾鎵嬪姩閲嶆柊鍔犺浇椹卞姩鍚庝竴鍒囨甯搞€?
### 鎸夎澶囬€夋嫨鍥轰欢

鏈€甯歌鐨勬儏鍐垫槸绯荤粺涓婄殑鎵€鏈夊崱閮戒娇鐢ㄧ浉鍚岀被鍨嬬殑鍥轰欢銆傚鏋滀綘鎯充负鐗瑰畾鐨勫崱鍔犺浇鐗瑰畾鐨?鍥轰欢鏄犲儚锛屽彲浠ヤ娇鐢?PCI 鎬荤嚎鍦板潃鎴栧簭鍒楀彿銆傞┍鍔ㄤ細鎸変互涓嬩紭鍏堥『搴忔煡鎵惧浐浠舵枃浠讹細

```
    nfp: Looking for firmware file in order of priority:
    nfp:  netronome/serial-00-12-34-aa-bb-cc-10-ff.nffw: not found
    nfp:  netronome/pci-0000:02:00.0.nffw: not found
    nfp:  netronome/nic_AMDA0081-0001_1x40.nffw: found, loading...

```
鍦ㄨ繖绉嶆儏鍐典笅锛屽鏋?`/lib/firmware/netronome` 涓瓨鍦ㄥ悕涓?**serial-00-12-34-aa-bb-5d-10-ff.nffw** 鎴?**pci-0000:02:00.0.nffw** 鐨勬枃浠讹紙鎴?閾炬帴锛夛紝鍒欒鍥轰欢鏂囦欢浼氫紭鍏堜簬 `nic_AMDA*` 鏂囦欢銆?
璇锋敞鎰忥紝`serial-**` 鍜?`pci-**` 鏂囦欢**涓嶄細**鑷姩鍖呭惈鍦?initramfs 涓紝浣犻渶瑕佸弬鑰?鐩稿簲宸ュ叿鐨勬枃妗ｆ潵浜嗚В濡備綍鍖呭惈瀹冧滑銆?
### 杩愯涓殑鍥轰欢鐗堟湰

鍙互閫氳繃 `ethtool -i` 鏌ョ湅鐗瑰畾 <netdev> 鎺ュ彛锛堜緥濡?enp4s0锛夋垨鎺ュ彛绔彛 <netdev port>
锛堜緥濡?enp4s0np0锛夋墍鍔犺浇鐨勫浐浠剁増鏈細

```
  $ ethtool -i <netdev>

```
### 鍥轰欢鍔犺浇绛栫暐

鍥轰欢鍔犺浇绛栫暐鐢变笁涓?HWinfo 鍙傛暟鎺у埗锛岃繖浜涘弬鏁颁互閿€煎鐨勫舰寮忓瓨鍌ㄥ湪璁惧闂瓨涓細

app_fw_from_flash
    瀹氫箟搴斾娇鐢ㄥ摢绉嶅浐浠朵紭鍏堬紝'Disk'锛?锛夈€?Flash'锛?锛夋垨 'Preferred'锛?锛夊浐浠躲€?    褰撻€夋嫨 'Preferred' 鏃讹紝绠＄悊鍥轰欢浼氭瘮杈冮棯瀛樺浐浠朵笌涓绘満鎻愪緵鍥轰欢鐨勭増鏈紝浠庤€屽喅瀹?    鍔犺浇鍝釜鍥轰欢銆傝鍙橀噺鍙娇鐢?'fw_load_policy' devlink 鍙傛暟杩涜閰嶇疆銆?
abi_drv_reset
    瀹氫箟椹卞姩鍦ㄨ鎺㈡祴鏃舵槸鍚﹀簲閲嶇疆鍥轰欢锛屽彲閫夋嫨 'Disk'锛?锛屽嵆濡傛灉鍦ㄧ鐩樹笂鎵惧埌鍥轰欢锛夈€?    'Always'锛?锛屾€绘槸閲嶇疆锛夋垨 'Never'锛?锛屼粠涓嶉噸缃級銆傛敞鎰忥細濡傛灉鍦ㄩ┍鍔ㄨ鎺㈡祴鏃?    鍥轰欢宸插姞杞斤紝鍒欏湪椹卞姩鍗歌浇鏃惰澶囨€绘槸浼氳閲嶇疆銆傝鍙橀噺鍙娇鐢?'reset_dev_on_drv_probe'
    devlink 鍙傛暟杩涜閰嶇疆銆?
abi_drv_load_ifc
    瀹氫箟鍏佽鍦ㄨ澶囦笂鍔犺浇 FW 鐨?PF 璁惧鍒楄〃銆傝鍙橀噺褰撳墠涓嶅彲鐢辩敤鎴烽厤缃€?
## Devlink 淇℃伅

devlink info 鍛戒护浼氭樉绀鸿澶囦笂杩愯涓拰宸插瓨鍌ㄧ殑鍥轰欢鐗堟湰銆佸簭鍒楀彿浠ュ強鏉垮崱淇℃伅銆?
```
  $ devlink dev info pci/0000:03:00.0
    pci/0000:03:00.0:
      driver nfp
      serial_number CSAAMDA2001-1003000111
      versions:
          fixed:
            board.id AMDA2001-1003
            board.rev 01
            board.manufacture CSA
            board.model mozart
          running:
            fw.mgmt 22.10.0-rc3
            fw.cpld 0x1000003
            fw.app nic-22.09.0
            chip.init AMDA-2001-1003  1003000111
          stored:
            fw.bundle_id bspbundle_1003000111
            fw.mgmt 22.10.0-rc3
            fw.cpld 0x0
            chip.init AMDA-2001-1003  1003000111

```
## 閰嶇疆璁惧

鏈妭浠嬬粛濡備綍浣跨敤杩愯鍩烘湰 NIC 鍥轰欢鐨?Agilio SmartNIC銆?
### 閰嶇疆鎺ュ彛閾捐矾閫熺巼

浠ヤ笅姝ラ璇存槑濡備綍鍦?Agilio CX 2x25GbE 缃戝崱涓婂湪 10G 妯″紡涓?25G 妯″紡涔嬮棿鍒囨崲銆傜鍙?閫熺巼鐨勬洿鏀瑰繀椤绘寜椤哄簭杩涜锛氱鍙?0锛坧0锛夊繀椤诲厛璁句负 10G锛屼箣鍚庣鍙?1锛坧1锛夋墠鑳借涓?10G銆?
```
  $ ip link set dev <netdev port 0> down
  $ ip link set dev <netdev port 1> down

```
```
  $ ethtool -s <netdev port 0> speed 10000
  $ ethtool -s <netdev port 1> speed 10000

```
```
  $ ethtool -s <netdev port 0> speed 25000
  $ ethtool -s <netdev port 1> speed 25000

```
```
  $ rmmod nfp; modprobe nfp

```
### 閰嶇疆鎺ュ彛鏈€澶т紶杈撳崟鍏冿紙MTU锛?
鎺ュ彛鐨?MTU 鍙互浣跨敤 iproute2銆乮p link 鎴?ifconfig 宸ュ叿涓存椂璁剧疆銆傝娉ㄦ剰姝ゆ洿鏀逛笉浼?鎸佷箙鍖栥€傚缓璁娇鐢?Network Manager 鎴栧叾浠栧悎閫傜殑鎿嶄綔绯荤粺閰嶇疆宸ュ叿杩涜璁剧疆锛屽洜涓洪€氳繃
Network Manager 瀵?MTU 鐨勬洿鏀瑰彲浠ユ寔涔呭寲銆?
```
  $ ip link set dev <netdev port> mtu 9000

```
鍦ㄥ鐞嗗法鍨嬪抚锛坖umbo frames锛夋垨浣跨敤闅ч亾鏃讹紝鐢辩敤鎴锋垨缂栨帓灞傝礋璐ｈ缃悎閫傜殑 MTU 鍊笺€備緥濡傦紝
濡傛灉浠?VM 鍙戝嚭鐨勬暟鎹寘瑕佸湪鍗′笂灏佽骞朵粠鐗╃悊绔彛鍙戝嚭锛屽垯 VF 鐨?MTU 搴旇缃负浣庝簬鐗╃悊绔彛
鐨?MTU锛屼互瀹圭撼闄勫姞澶撮儴鎵€澧炲姞鐨勫瓧鑺傛暟銆傚鏋滈鏈?SmartNIC 涓庡唴鏍镐箣闂翠細鏈夊洖閫€娴侀噺锛岄偅涔?鐢ㄦ埛杩樺簲纭繚 PF MTU 璁剧疆寰楀綋锛屼互閬垮厤璇ヨ矾寰勪笂鍑虹幇鎰忓涓㈠寘銆?
### 閰嶇疆鍓嶅悜绾犻敊锛團EC锛夋ā寮?
Agilio SmartNIC 鏀寔 FEC 妯″紡閰嶇疆锛屼緥濡?Auto銆丗irecode Base-R銆丷eedSolomon 浠ュ強 Off
妯″紡銆傛瘡涓墿鐞嗙鍙ｇ殑 FEC 妯″紡閮藉彲浠ラ€氳繃 ethtool 鐙珛璁剧疆銆傚彲浠ラ€氳繃 `ethtool <netdev>`
鏌ョ湅鏌愭帴鍙ｆ墍鏀寔鐨?FEC 妯″紡锛?
```
  $ ethtool <netdev>

```
```
  $ ethtool --show-fec <netdev>

```
瑕佸己鍒剁壒瀹氱鍙ｇ殑 FEC 妯″紡锛屽繀椤诲厛绂佺敤鑷姩鍗忓晢锛堣 `鑷姩鍗忓晢`_ 涓€鑺傦級銆傝缃?FEC 妯″紡鐨?绀轰緥濡備笅锛?
```
  $ ethtool --set-fec <netdev> encoding rs

```
### 鑷姩鍗忓晢

瑕佹洿鏀硅嚜鍔ㄥ崗鍟嗚缃紝蹇呴』鍏堣閾捐矾 down銆傚湪閾捐矾 down 鍚庯細

```
  ethtool -s <netdev> autoneg <on|off>

```
## 缁熻淇℃伅

浠ヤ笅璁惧缁熻淇℃伅鍙€氳繃 `ethtool -S` 鎺ュ彛鑾峰彇锛?
   :header-rows: 1
   :widths: 3 1 11

   - - Name
     - ID
     - Meaning

   - - dev_rx_discards
     - 1
     - 鏁版嵁鍖呭彲鑳藉洜浠ヤ笅浠讳竴鍘熷洜鍦?RX 璺緞涓婅涓㈠純锛?
        - NIC 鏈浜庢贩鏉傛ā寮忥紝涓旂洰鐨?MAC 鍦板潃涓庢帴鍙ｇ殑 MAC 鍦板潃涓嶅尮閰嶃€?        - 鎺ユ敹鍒扮殑鏁版嵁鍖呭ぇ浜庝富鏈轰笂鐨勬渶澶х紦鍐插尯澶у皬锛屽嵆瓒呰繃浜嗙 3 灞?MRU銆?        - 涓绘満涓婃病鏈夊彲鐢ㄤ簬璇ユ暟鎹寘鐨勭┖闂插垪琛ㄦ弿杩扮銆傚緢鍙兘鏄?NIC 鏈兘鍙婃椂缂撳瓨涓€涓€?        - 鏌愪釜 BPF 绋嬪簭涓㈠純浜嗚鏁版嵁鍖呫€?        - 鎵ц浜嗘暟鎹潰涓㈠寘鍔ㄤ綔銆?        - MAC 鍥?NIC 涓婄己灏戝叆鍙ｇ紦鍐插尯绌洪棿鑰屼涪寮冧簡璇ユ暟鎹寘銆?
   - - dev_rx_errors
     - 2
     - 鏁版嵁鍖呭彲鑳藉洜浠ヤ笅鍘熷洜琚涓猴紙骞朵涪寮冧负锛塕X 閿欒锛?
       - VEB 鏌ユ壘鍑虹幇闂锛堜粎鍦ㄤ娇鐢?SR-IOV 鏃讹級銆?       - 瀵艰嚧浠ュお缃戦敊璇殑鐗╃悊灞傞棶棰橈紝渚嬪 FCS 鎴栧榻愰敊璇€傚師鍥犻€氬父鏄晠闅滅嚎缂嗘垨 SFP銆?
   - - dev_rx_bytes
     - 3
     - 鎺ユ敹鍒扮殑瀛楄妭鎬绘暟銆?
   - - dev_rx_uc_bytes
     - 4
     - 鎺ユ敹鍒扮殑鍗曟挱瀛楄妭鏁般€?
   - - dev_rx_mc_bytes
     - 5
     - 鎺ユ敹鍒扮殑澶氭挱瀛楄妭鏁般€?
   - - dev_rx_bc_bytes
     - 6
     - 鎺ユ敹鍒扮殑骞挎挱瀛楄妭鏁般€?
   - - dev_rx_pkts
     - 7
     - 鎺ユ敹鍒扮殑鏁版嵁鍖呮€绘暟銆?
   - - dev_rx_mc_pkts
     - 8
     - 鎺ユ敹鍒扮殑澶氭挱鏁版嵁鍖呮暟銆?
   - - dev_rx_bc_pkts
     - 9
     - 鎺ユ敹鍒扮殑骞挎挱鏁版嵁鍖呮暟銆?
   - - dev_tx_discards
     - 10
     - 褰?MAC 琚祦鎺т笖 NIC 鐨?TX 闃熷垪绌洪棿鑰楀敖鏃讹紝鏁版嵁鍖呭彲鑳藉湪 TX 鏂瑰悜琚涪寮冦€?
   - - dev_tx_errors
     - 11
     - 鏁版嵁鍖呭彲鑳藉洜浠ヤ笅浠讳竴鍘熷洜琚涓?TX 閿欒锛堝苟涓㈠純锛夛細

       - 鏁版嵁鍖呮槸涓€涓?LSO 鍒嗙墖锛屼絾鏃犳硶纭畾绗?3 灞傛垨绗?4 灞傜殑鍋忕Щ锛屽洜姝?LSO 鏃犳硶杩涜銆?       - 閫氳繃 PCIe 鏀跺埌浜嗘棤鏁堢殑鏁版嵁鍖呮弿杩扮銆?       - 鏁版嵁鍖呯殑绗?3 灞傞暱搴﹁秴杩囦簡璁惧 MTU銆?       - MAC/鐗╃悊灞傚嚭閿欍€傞€氬父鐢变簬鏁呴殰绾跨紗鎴?SFP 鎵€鑷淬€?       - 鏃犳硶鍒嗛厤 CTM 缂撳啿鍖恒€?       - 鏁版嵁鍖呭亸绉讳笉姝ｇ‘涓?NIC 鏃犳硶淇銆?
   - - dev_tx_bytes
     - 12
     - 鍙戦€佺殑瀛楄妭鎬绘暟銆?
   - - dev_tx_uc_bytes
     - 13
     - 鍙戦€佺殑鍗曟挱瀛楄妭鏁般€?
   - - dev_tx_mc_bytes
     - 14
     - 鍙戦€佺殑澶氭挱瀛楄妭鏁般€?
   - - dev_tx_bc_bytes
     - 15
     - 鍙戦€佺殑骞挎挱瀛楄妭鏁般€?
   - - dev_tx_pkts
     - 16
     - 鍙戦€佺殑鏁版嵁鍖呮€绘暟銆?
   - - dev_tx_mc_pkts
     - 17
     - 鍙戦€佺殑澶氭挱鏁版嵁鍖呮暟銆?
   - - dev_tx_bc_pkts
     - 18
     - 鍙戦€佺殑骞挎挱鏁版嵁鍖呮暟銆?
娉ㄦ剰锛岄┍鍔ㄦ湭鐭ョ殑缁熻淇℃伅浼氭樉绀轰负 `dev_unknown_stat$ID`锛屽叾涓?`$ID` 鎸囦笂琛ㄤ腑鐨勭浜屽垪銆?