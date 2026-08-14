
## ice devlink 鏀寔


鏈枃妗ｆ弿杩颁簡鐢?`ice` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


   :widths: 5 5 90

   - - Name
     - Mode
     - Notes
   - - `enable_roce`
     - runtime
     - 涓?`enable_iwarp` 浜掓枼
   - - `enable_iwarp`
     - runtime
     - 涓?`enable_roce` 浜掓枼
   - - `tx_scheduling_layers`
     - permanent
     - ice 纭欢瀵?Tx 浣跨敤鍒嗗眰璋冨害锛岃皟搴︽爲涓湁鍥哄畾鏁伴噺鐨勫眰绾с€傚畠浠瘡涓€涓兘鏄喅绛栫偣銆傛牴鑺傜偣浠ｈ〃涓€涓鍙ｏ紝鑰屾墍鏈夊彾瀛愪唬琛ㄩ槦鍒椼€傝繖绉嶉厤缃?Tx 璋冨害鍣ㄧ殑鏂瑰紡鍏佽璇稿 DCB 鎴?devlink-rate锛堜笅鏂囨湁鏂囨。璇存槑锛変箣绫荤殑鐗规€ф潵閰嶇疆缁欎簣浠讳綍缁欏畾闃熷垪鎴栭槦鍒楃粍鐨勫甫瀹介噺锛屼粠鑰屽疄鐜扮粏绮掑害鎺у埗锛屽洜涓鸿皟搴﹀弬鏁板彲浠ュ湪鏍戠殑浠绘剰灞傜骇涓婇厤缃€?
       榛樿鐨?9 灞傛爲鎷撴墤琚涓烘渶閫傚悎澶у鏁板伐浣滆礋杞斤紝鍥犱负瀹冩彁渚涗簡鎬ц兘涓庡彲閰嶇疆鎬х殑鏈€浣虫瘮渚嬨€傜劧鑰岋紝鍦ㄦ煇浜涚壒瀹氭儏鍐典笅锛岃繖绉?9 灞傛嫇鎵戝彲鑳藉苟闈炴墍鎰裤€備竴涓緥瀛愭槸鍚戜笉鏄?8 鐨勫€嶆暟鐨勯槦鍒楀彂閫佹祦閲忋€傚洜涓哄湪 9 灞傛嫇鎵戜腑鏈€澶у熀鏁伴檺鍒朵负 8锛岀 9 涓槦鍒椾笌鍏朵綑闃熷垪鏈変笉鍚岀殑鐖惰妭鐐癸紝骞惰缁欎簣鏇村甯﹀淇＄敤銆傚綋绯荤粺鍚?9 涓槦鍒楀彂閫佹祦閲忔椂锛岃繖浼氬鑷翠竴涓棶棰橈細

       | tx_queue_0_packets: 24163396
       | tx_queue_1_packets: 24164623
       | tx_queue_2_packets: 24163188
       | tx_queue_3_packets: 24163701
       | tx_queue_4_packets: 24163683
       | tx_queue_5_packets: 24164668
       | tx_queue_6_packets: 23327200
       | tx_queue_7_packets: 24163853
       | tx_queue_8_packets: 91101417 < Too much traffic is sent from 9th

       涓轰簡婊¤冻杩欎竴闇€姹傦紝浣犲彲浠ュ垏鎹㈠埌 5 灞傛嫇鎵戯紝瀹冨皢鏈€澶ф嫇鎵戝熀鏁版敼涓?512銆傛湁浜嗚繖涓€澧炲己锛屾€ц兘鐗瑰緛鏄潎绛夌殑锛屽洜涓烘墍鏈夐槦鍒楅兘鍙互琚垎閰嶅埌鏍戜腑鍚屼竴涓埗鑺傜偣銆傛瑙ｅ喅鏂规鏄庢樉鐨勭己鐐规槸鏍戠殑閰嶇疆娣卞害杈冧綆銆?
       浣跨敤 devlink 鍛戒护鐨?`tx_scheduling_layer` 鍙傛暟鏉ユ敼鍙樺彂閫佽皟搴﹀櫒鎷撴墤銆傝浣跨敤 5 灞傛嫇鎵戯紝浣跨敤鍊?5銆備緥濡傦細
       $ devlink dev param set pci/0000:16:00.0 name tx_scheduling_layers
       value 5 cmode permanent
       浣跨敤鍊?9 灏嗗叾璁惧洖榛樿鍊笺€?
       浣犲繀椤诲 PCI 鎻掓Ы杩涜鏂數鍐嶄笂鐢碉紝鎵€閫夋嫇鎵戞墠鑳界敓鏁堛€?
       瑕侀獙璇佸€煎凡璁剧疆锛?       $ devlink dev param show pci/0000:16:00.0 name tx_scheduling_layers
   - - `msix_vec_per_pf_max`
     - driverinit
     - 璁剧疆 PF 鍙互浣跨敤鐨勬渶澶?MSI-X锛屽叾浣欏彲鐢ㄤ簬 SRIOV銆傝寖鍥翠粠 msix_vec_per_pf_min 涓缃殑鏈€灏忓€煎埌 2k/绔彛鏁般€?   - - `msix_vec_per_pf_min`
     - driverinit
     - 璁剧疆 PF 灏嗕娇鐢ㄧ殑鏈€灏?MSI-X銆傛鍊兼寚鏄庡皢闈欐€佸垎閰嶅灏?MSI-X銆傝寖鍥翠粠 2 鍒板湪 msix_vec_per_pf_max 涓缃殑鍊笺€?
    :widths: 5 5 90

    - - Name
      - Mode
      - Description
    - - `local_forwarding`
      - runtime
      - 閫氳繃璋冧紭璋冨害鍣ㄥ甫瀹芥潵鎺у埗鐜洖琛屼负銆傚畠褰卞搷鎵€鏈夌被鍨嬬殑鍑芥暟锛氱墿鐞嗐€佽櫄鎷熷拰瀛愬嚱鏁般€?        鏀寔鐨勫€兼湁锛?
        `enabled` - 绔彛涓婂厑璁哥幆鍥炴祦閲?
        `disabled` - 姝ょ鍙ｄ笂涓嶅厑璁哥幆鍥炴祦閲?
        `prioritized` - 姝ょ鍙ｄ笂鐜洖娴侀噺琚紭鍏堝鐞?
        `local_forwarding` 鍙傛暟鐨勯粯璁ゅ€间负 `enabled`銆俙prioritized` 鎻愪緵璋冩暣鐜洖娴侀噺閫熺巼鐨勮兘鍔涳紝浠ョ壓鐗插彟涓€涓鍙ｄ负浠ｄ环鏉ュ鍔犱竴涓鍙ｇ殑瀹归噺銆傜敤鎴烽渶瑕佸湪涓€涓鍙ｄ笂绂佺敤鏈湴杞彂锛屼互渚垮湪 `prioritized` 绔彛涓婅幏寰楀鍔犵殑瀹归噺銆?
## 淇℃伅鐗堟湰


`ice` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

    :widths: 5 5 5 90

    - - Name
      - Type
      - Example
      - Description
    - - `board.id`
      - fixed
      - K65390-000
      - 鏉垮崱鐨勪骇鍝佹澘瑁呴厤锛圥BA锛夋爣璇嗙銆?    - - `cgu.id`
      - fixed
      - 36
      - 鏃堕挓鐢熸垚鍗曞厓锛圕GU锛夌‖浠朵慨璁㈡爣璇嗙銆?    - - `fw.mgmt`
      - running
      - 2.1.7
      - 杩愯鍦ㄨ澶囧祵鍏ュ紡绠＄悊澶勭悊鍣ㄤ笂鐨勭鐞嗗浐浠剁殑 3 浣嶇増鏈彿銆傚畠鎺у埗 PHY銆侀摼璺€佸璁惧璧勬簮鐨勮闂瓑銆侷ntel 鏂囨。灏嗗叾绉颁负 EMP 鍥轰欢銆?    - - `fw.mgmt.api`
      - running
      - 1.5.1
      - 绠＄悊鍥轰欢閫氳繃 AdminQ 瀵煎嚭鐨?API 鐨?3 浣嶇増鏈彿锛坢ajor.minor.patch锛夈€傞┍鍔ㄧ敤瀹冩潵璇嗗埆鏀寔鍝簺鍛戒护銆傚唴鏍哥殑鏃╂湡鐗堟湰鍙樉绀?2 浣嶇増鏈彿锛坢ajor.minor锛夈€?    - - `fw.mgmt.build`
      - running
      - 0x305d955f
      - 绠＄悊鍥轰欢鏉ユ簮鐨勫敮涓€鏍囪瘑绗︺€?    - - `fw.undi`
      - running
      - 1.2581.0
      - 鍖呭惈 UEFI 椹卞姩鐨?Option ROM 鐨勭増鏈€傜増鏈互 `major.minor.patch` 鏍煎紡鎶ュ憡銆傛瘡褰撳彂鐢熼噸澶х殑鐮村潖鎬у彉鏇达紝鎴栨鐗堟湰灏嗚婧㈠嚭鏃讹紝涓荤増鏈€掑銆傛鐗堟湰鍦ㄩ潪鐮村潖鎬у彉鏇存椂閫掑锛屽苟鍦ㄤ富鐗堟湰閫掑鏃堕噸缃负 1銆傝ˉ涓佺増鏈€氬父涓?0锛屼絾褰撲慨澶嶄綔涓洪拡瀵硅緝鏃у熀纭€ Option ROM 鐨勮ˉ涓佹彁渚涙椂閫掑銆?    - - `fw.psid.api`
      - running
      - 0.80
      - 瀹氫箟闂瓨鍐呭鏍煎紡鐨勭増鏈€?    - - `fw.bundle_id`
      - running
      - 0x80002ec0
      - 鍔犺浇鍒拌澶囦笂鐨勫浐浠舵槧鍍忔枃浠剁殑鍞竴鏍囪瘑绗︺€備篃绉颁负 NVM 鐨?EETRACK 鏍囪瘑绗︺€?    - - `fw.app.name`
      - running
      - ICE OS Default Package
      - 璁惧涓浜庢椿鍔ㄧ姸鎬佺殑 DDP 鍖呯殑鍚嶇О銆侱DP 鍖呯敱椹卞姩鍦ㄥ垵濮嬪寲鏈熼棿鍔犺浇銆侱DP 鍖呯殑姣忕鍙樹綋閮芥湁鍞竴鐨勫悕绉般€?    - - `fw.app`
      - running
      - 1.3.1.0
      - 璁惧涓浜庢椿鍔ㄧ姸鎬佺殑 DDP 鍖呯殑鐗堟湰銆傛敞鎰忥紝瑕佸敮涓€鏍囪瘑璇ュ寘锛屽悕绉板拰鐗堟湰锛堢敱 `fw.app.name` 鎶ュ憡锛夐兘鏄繀闇€鐨勩€?    - - `fw.app.bundle_id`
      - running
      - 0xc0000001
      - 璁惧涓姞杞界殑 DDP 鍖呯殑鍞竴鏍囪瘑绗︺€備篃绉颁负 DDP Track ID銆傚彲鐢ㄤ簬鍞竴鏍囪瘑鐗瑰畾鐨?DDP 鍖呫€?    - - `fw.netlist`
      - running
      - 1.1.2000-6.7.0
      - netlist 妯″潡鐨勭増鏈€傝妯″潡瀹氫箟璁惧鐨勪互澶綉鑳藉姏鍜岄粯璁よ缃紝骞惰绠＄悊鍥轰欢鐢ㄤ綔绠＄悊閾捐矾鍜岃澶囪繛鎺ユ€х殑涓€閮ㄥ垎銆?    - - `fw.netlist.build`
      - running
      - 0xee16ced7
      - netlist 妯″潡鍐呭鐨勫搱甯岀殑鍓?4 涓瓧鑺傘€?    - - `fw.cgu`
      - running
      - 8032.16973825.6021
      - 鏃堕挓鐢熸垚鍗曞厓锛圕GU锛夌殑鐗堟湰銆傛牸寮忥細<CGU 绫诲瀷>.<閰嶇疆鐗堟湰>.<鍥轰欢鐗堟湰>銆?
## 闂瓨鏇存柊


`ice` 椹卞姩浣跨敤 `devlink-flash` 鎺ュ彛瀹炵幇闂瓨鏇存柊鏀寔銆傚畠鏀寔浣跨敤鍖呭惈 `fw.mgmt`銆乣fw.undi` 鍜?`fw.netlist` 缁勪欢鐨勫悎骞堕棯瀛樻槧鍍忔潵鏇存柊璁惧闂瓨銆?
   :widths: 5 95

   - - Bits
     - Behavior
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 涓嶄繚鐣欐鍦ㄦ洿鏂扮殑闂瓨缁勪欢涓瓨鍌ㄧ殑璁剧疆銆傝繖鍖呮嫭瑕嗙洊纭畾璁惧灏嗗垵濮嬪寲澶氬皯涓墿鐞嗗嚱鏁扮殑绔彛閰嶇疆銆?   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS` and `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 鏃笉淇濈暀璁剧疆涔熶笉淇濈暀鏍囪瘑绗︺€傜敤鎵€鎻愪緵鏄犲儚鐨勫唴瀹硅鐩栭棯瀛樹腑鐨勪竴鍒囷紝涓嶈繘琛屼换浣曚繚鐣欍€傝繖鍖呮嫭瑕嗙洊璁惧鏍囪瘑瀛楁锛屽 MAC 鍦板潃銆乂PD 鍖哄煙鍜岃澶囧簭鍒楀彿銆傞鏈熸缁勫悎涓庝负鐗瑰畾璁惧瀹氬埗鐨勬槧鍍忎竴璧蜂娇鐢ㄣ€?
ice 纭欢涓嶆敮鎸佸湪淇濈暀璁剧疆鐨勫悓鏃朵粎瑕嗙洊鏍囪瘑绗︼紝鍥犳鍗曠嫭鐨?`DEVLINK_FLASH_OVERWRITE_IDENTIFIERS` 浼氳鎷掔粷銆傚鏋滄湭鎻愪緵瑕嗙洊鎺╃爜锛屽浐浠跺皢琚寚绀哄湪鏇存柊鏃朵繚鐣欐墍鏈夎缃拰鏍囪瘑瀛楁銆?
## 閲嶆柊鍔犺浇


`ice` 椹卞姩鏀寔鍦ㄩ棯瀛樻洿鏂板悗浣跨敤甯︽湁 `DEVLINK_RELOAD_ACTION_FW_ACTIVATE` 鍔ㄤ綔鐨?`DEVLINK_CMD_RELOAD` 鏉ユ縺娲绘柊鍥轰欢銆?

    $ devlink dev reload pci/0000:01:00.0 reload action fw_activate

鏂板浐浠堕€氳繃鍙戝嚭璁惧鐗瑰畾鐨勫祵鍏ュ紡绠＄悊澶勭悊鍣ㄥ浣嶆潵婵€娲伙紝璇ュ浣嶈姹傝澶囬噸缃苟閲嶆柊鍔犺浇 EMP 鍥轰欢鏄犲儚銆?
椹卞姩褰撳墠涓嶆敮鎸侀€氳繃 `DEVLINK_RELOAD_ACTION_DRIVER_REINIT` 閲嶆柊鍔犺浇椹卞姩銆?
## 绔彛鎷嗗垎


`ice` 椹卞姩浠呮敮鎸佺鍙?0 鐨勭鍙ｆ媶鍒嗭紝鍥犱负 FW 涓烘暣涓澶囬瀹氫箟浜嗕竴缁勫彲鐢ㄧ殑绔彛鎷嗗垎閫夐」銆?
搴旂敤绔彛鎷嗗垎闇€瑕佺郴缁熼噸鍚€?
浠ヤ笅鍛戒护灏嗛€夋嫨鍏锋湁 4 涓鍙ｇ殑绔彛鎷嗗垎閫夐」锛?

    $ devlink port split pci/0000:16:00.0/0 count 4

姣忔 `split` 鍜?`unsplit` 鍛戒护鍚庯紝鎵€鏈夊彲鐢ㄧ鍙ｉ€夐」鐨勫垪琛ㄥ皢琚墦鍗板埌鍔ㄦ€佽皟璇曚腑銆傜涓€涓€夐」鏄粯璁ゅ€笺€?

    ice 0000:16:00.0: Available port split options and max port speeds (Gbps):
    ice 0000:16:00.0: Status  Split      Quad 0          Quad 1
    ice 0000:16:00.0:         count  L0  L1  L2  L3  L4  L5  L6  L7
    ice 0000:16:00.0: Active  2     100   -   -   - 100   -   -   -
    ice 0000:16:00.0:         2      50   -  50   -   -   -   -   -
    ice 0000:16:00.0: Pending 4      25  25  25  25   -   -   -   -
    ice 0000:16:00.0:         4      25  25   -   -  25  25   -   -
    ice 0000:16:00.0:         8      10  10  10  10  10  10  10  10
    ice 0000:16:00.0:         1     100   -   -   -   -   -   -   -

鍙兘瀛樺湪澶氫釜鍏锋湁鐩稿悓绔彛鎷嗗垎璁℃暟鐨?FW 绔彛閫夐」銆傚綋鍐嶆鍙戝嚭鐩稿悓鐨勭鍙ｆ媶鍒嗚鏁拌姹傛椂锛屽皢閫夋嫨鍏锋湁鐩稿悓绔彛鎷嗗垎璁℃暟鐨勪笅涓€涓?FW 绔彛閫夐」銆?
`devlink port unsplit` 灏嗛€夋嫨鎷嗗垎璁℃暟涓?1 鐨勯€夐」銆傚鏋滄病鏈夋媶鍒嗚鏁颁负 1 鐨?FW 閫夐」鍙敤锛屼綘灏嗘敹鍒颁竴涓敊璇€?
## 鍖哄煙


`ice` 椹卞姩瀹炵幇浜嗕互涓嬬敤浜庤闂唴閮ㄨ澶囨暟鎹殑鍖哄煙銆?
    :widths: 15 85

    - - Name
      - Description
    - - `nvm-flash`
      - 鏁翠釜闂瓨鑺墖鐨勫唴瀹癸紝鏈夋椂绉颁负璁惧鐨勯潪鏄撳け鎬у瓨鍌ㄥ櫒銆?    - - `shadow-ram`
      - Shadow RAM 鐨勫唴瀹癸紝瀹冧粠闂瓨寮€澶村姞杞姐€傚敖绠″唴瀹逛富瑕佹潵鑷棯瀛橈紝璇ュ尯鍩熻繕鍖呭惈鍦ㄨ澶囧惎鍔ㄦ湡闂寸敓鎴愩€佷絾鏈瓨鍌ㄥ湪闂瓨涓殑鏁版嵁銆?    - - `device-caps`
      - 璁惧鍥轰欢鑳藉姏缂撳啿鍖虹殑鍐呭銆傛湁鍔╀簬纭畾璁惧鐨勫綋鍓嶇姸鎬佸拰閰嶇疆銆?
`nvm-flash` 鍜?`shadow-ram` 鍖哄煙閮藉彲浠ュ湪娌℃湁蹇収鐨勬儏鍐典笅璁块棶銆俙device-caps` 鍖哄煙闇€瑕佸揩鐓э紝鍥犱负鍏跺唴瀹圭敱鍥轰欢鍙戦€侊紝鏃犳硶鎷嗗垎涓哄崟鐙殑璇诲彇銆?
鐢ㄦ埛鍙互閫氳繃 `DEVLINK_CMD_REGION_NEW` 鍛戒护璇锋眰绔嬪嵆鎹曡幏鎵€鏈変笁涓尯鍩熺殑蹇収銆?

    $ devlink region show
    pci/0000:01:00.0/nvm-flash: size 10485760 snapshot [] max 1
    pci/0000:01:00.0/device-caps: size 4096 snapshot [] max 10

    $ devlink region new pci/0000:01:00.0/nvm-flash snapshot 1
    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    $ devlink region read pci/0000:01:00.0/nvm-flash snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    $ devlink region delete pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region new pci/0000:01:00.0/device-caps snapshot 1
    $ devlink region dump pci/0000:01:00.0/device-caps snapshot 1
    0000000000000000 01 00 01 00 00 00 00 00 01 00 00 00 00 00 00 00
    0000000000000010 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000020 02 00 02 01 32 03 00 00 0a 00 00 00 25 00 00 00
    0000000000000030 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000040 04 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000050 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000060 05 00 01 00 03 00 00 00 00 00 00 00 00 00 00 00
    0000000000000070 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000080 06 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000090 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000a0 08 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000b0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000c0 12 00 01 00 01 00 00 00 01 00 01 00 00 00 00 00
    00000000000000d0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000e0 13 00 01 00 00 01 00 00 00 00 00 00 00 00 00 00
    00000000000000f0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000100 14 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000110 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000120 15 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000130 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000140 16 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000150 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000160 17 00 01 00 06 00 00 00 00 00 00 00 00 00 00 00
    0000000000000170 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000180 18 00 01 00 01 00 00 00 01 00 00 00 08 00 00 00
    0000000000000190 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001a0 22 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    00000000000001b0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001c0 40 00 01 00 00 08 00 00 08 00 00 00 00 00 00 00
    00000000000001d0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001e0 41 00 01 00 00 08 00 00 00 00 00 00 00 00 00 00
    00000000000001f0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000200 42 00 01 00 00 08 00 00 00 00 00 00 00 00 00 00
    0000000000000210 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

    $ devlink region delete pci/0000:01:00.0/device-caps snapshot 1

## Devlink 閫熺巼


`ice` 椹卞姩瀹炵幇浜?devlink-rate API銆傚畠鍏佽灏嗗垎灞?QoS 鍗歌浇鍒扮‖浠躲€傚畠浣跨敤鎴疯兘澶熷皢铏氭嫙鍑芥暟鍒嗙粍涓烘爲褰㈢粨鏋勶紝骞跺悜鏍戜腑鐨勬瘡涓妭鐐瑰垎閰嶅彈鏀寔鐨勫弬鏁帮細tx_share銆乼x_max銆乼x_priority 鍜?tx_weight銆傚洜姝わ紝鐢ㄦ埛瀹為檯涓婅幏寰椾簡鎺у埗涓烘瘡涓?VF 缁勫垎閰嶅灏戝甫瀹界殑鑳藉姏銆傝繖闅忓悗鐢辩‖浠跺己鍒舵墽琛屻€?
鍋囧畾姝ょ壒鎬т笌 FW 涓墽琛岀殑 DCB 鍜?ADQ锛屾垨浠讳綍浼氳Е鍙?QoS 鍙樻洿锛堜緥濡傚垱寤烘柊鐨勬祦閲忕被锛夌殑椹卞姩鐗规€т簰鏂ャ€傚鏋滅敤鎴峰紑濮嬩娇鐢?devlink-rate API 瀵硅妭鐐硅繘琛屼换浣曟洿鏀癸紝椹卞姩灏嗛樆姝?DCB 鎴?ADQ 閰嶇疆銆傝閰嶇疆杩欎簺鐗规€э紝闇€瑕侀噸鏂板姞杞介┍鍔ㄣ€傜浉搴斿湴锛屽鏋?ADQ 鎴?DCB 琚厤缃紝椹卞姩灏嗘牴鏈笉瀵煎嚭灞傜骇缁撴瀯锛涙垨鑰咃紝濡傛灉鍦ㄥ眰绾х粨鏋勫鍑轰箣鍚庛€佷絾鍦ㄨ繘琛屼换浣曟洿鏀逛箣鍓嶅惎鐢ㄤ簡杩欎簺鐗规€э紝椹卞姩灏嗙Щ闄ゆ湭瑙﹀強鐨勫眰绾х粨鏋勩€?
姝ょ壒鎬ц繕渚濊禆浜庣郴缁熶腑鍚敤浜?switchdev銆傝繖鏄繀闇€鐨勶紝鍥犱负 devlink-rate 闇€瑕?devlink-port 瀵硅薄瀛樺湪锛岃€岃繖浜涘璞′粎鍦?switchdev 妯″紡涓嬪垱寤恒€?
濡傛灉椹卞姩璁剧疆涓?switchdev 妯″紡锛屽畠灏嗗湪 VF 鍒涘缓鐨勯偅涓€鍒诲鍑哄唴閮ㄥ眰绾х粨鏋勩€傛爲鐨勬牴濮嬬粓鐢?node_0 琛ㄧず銆傛鑺傜偣涓嶈兘琚敤鎴峰垹闄ゃ€傚彾瀛愯妭鐐瑰拰鏈夊瓙鑺傜偣鐨勮妭鐐逛篃涓嶈兘琚垹闄ゃ€?
    :widths: 15 85

    - - Name
      - Description
    - - `tx_max`
      - 鏍戣妭鐐硅娑堣€楃殑鏈€澶у甫瀹姐€傞€熺巼闄愬埗鏄竴涓粷瀵规暟瀛楋紝鎸囧畾鑺傜偣鍦ㄤ竴绉掑唴鍙互娑堣€楃殑鏈€澶у瓧鑺傛暟銆傞€熺巼闄愬埗淇濊瘉閾捐矾涓嶄細浣胯繙绔帴鏀舵柟杩囬ケ鍜岋紝骞跺湪璁㈤槄鑰呬笌缃戠粶鎻愪緵鑰呬箣闂村己鍒舵墽琛?SLA銆?    - - `tx_share`
      - 褰撴爲鑺傜偣鏈闃诲鏃跺垎閰嶇粰瀹冪殑鏈€灏忓甫瀹姐€傚畠鎸囧畾涓€涓粷瀵瑰甫瀹姐€傝櫧鐒?tx_max 瀹氫箟浜嗚妭鐐瑰彲浠ユ秷鑰楃殑鏈€澶у甫瀹斤紝tx_share 鏍囪涓鸿鑺傜偣鎵胯鐨勫甫瀹姐€?    - - `tx_priority`
      - 鍏佽鍦ㄥ厔寮熻妭鐐逛箣闂翠娇鐢ㄤ弗鏍间紭鍏堢骇浠茶鍣ㄣ€傚彧瑕佽妭鐐瑰湪鍏跺甫瀹介檺鍒跺唴锛屾浠茶鏂规灏卞皾璇曟牴鎹妭鐐圭殑浼樺厛绾ц繘琛岃皟搴︺€傝寖鍥?0-7銆備紭鍏堢骇涓?7 鐨勮妭鐐瑰叿鏈夋渶楂樹紭鍏堢骇骞堕鍏堣閫変腑锛岃€屼紭鍏堢骇涓?0 鐨勮妭鐐逛紭鍏堢骇鏈€浣庛€傚叿鏈夌浉鍚屼紭鍏堢骇鐨勮妭鐐硅骞崇瓑瀵瑰緟銆?    - - `tx_weight`
      - 鍏佽鍦ㄥ厔寮熻妭鐐逛箣闂翠娇鐢ㄥ姞鏉冨叕骞抽槦鍒椾徊瑁佹柟妗堛€傛浠茶鏂规鍙互涓庝弗鏍间紭鍏堢骇鍚屾椂浣跨敤銆傝寖鍥?1-200銆傚浜庝徊瑁侊紝鍙湁鐩稿鍊兼墠鏈夋剰涔夈€?
`tx_priority` 鍜?`tx_weight` 鍙互鍚屾椂浣跨敤銆傚湪杩欑鎯呭喌涓嬶紝鍏锋湁鐩稿悓浼樺厛绾х殑鑺傜偣鍦ㄥ厔寮熻妭鐐圭粍涓舰鎴愪竴涓?WFQ 瀛愮粍锛屽畠浠箣闂寸殑浠茶鍩轰簬鍒嗛厤鐨勬潈閲嶃€?

    # enable switchdev
    $ devlink dev eswitch set pci/0000:4b:00.0 mode switchdev

    # at this point driver should export internal hierarchy
    $ echo 2 > /sys/class/net/ens785np0/device/sriov_numvfs

    $ devlink port function rate show
    pci/0000:4b:00.0/node_25: type node parent node_24
    pci/0000:4b:00.0/node_24: type node parent node_0
    pci/0000:4b:00.0/node_32: type node parent node_31
    pci/0000:4b:00.0/node_31: type node parent node_30
    pci/0000:4b:00.0/node_30: type node parent node_16
    pci/0000:4b:00.0/node_19: type node parent node_18
    pci/0000:4b:00.0/node_18: type node parent node_17
    pci/0000:4b:00.0/node_17: type node parent node_16
    pci/0000:4b:00.0/node_14: type node parent node_5
    pci/0000:4b:00.0/node_5: type node parent node_3
    pci/0000:4b:00.0/node_13: type node parent node_4
    pci/0000:4b:00.0/node_12: type node parent node_4
    pci/0000:4b:00.0/node_11: type node parent node_4
    pci/0000:4b:00.0/node_10: type node parent node_4
    pci/0000:4b:00.0/node_9: type node parent node_4
    pci/0000:4b:00.0/node_8: type node parent node_4
    pci/0000:4b:00.0/node_7: type node parent node_4
    pci/0000:4b:00.0/node_6: type node parent node_4
    pci/0000:4b:00.0/node_4: type node parent node_3
    pci/0000:4b:00.0/node_3: type node parent node_16
    pci/0000:4b:00.0/node_16: type node parent node_15
    pci/0000:4b:00.0/node_15: type node parent node_0
    pci/0000:4b:00.0/node_2: type node parent node_1
    pci/0000:4b:00.0/node_1: type node parent node_0
    pci/0000:4b:00.0/node_0: type node
    pci/0000:4b:00.0/1: type leaf parent node_25
    pci/0000:4b:00.0/2: type leaf parent node_25

    # let's create some custom node
    $ devlink port function rate add pci/0000:4b:00.0/node_custom parent node_0

    # second custom node
    $ devlink port function rate add pci/0000:4b:00.0/node_custom_1 parent node_custom

    # reassign second VF to newly created branch
    $ devlink port function rate set pci/0000:4b:00.0/2 parent node_custom_1

    # assign tx_weight to the VF
    $ devlink port function rate set pci/0000:4b:00.0/2 tx_weight 5

    # assign tx_share to the VF
    $ devlink port function rate set pci/0000:4b:00.0/2 tx_share 500Mbps
