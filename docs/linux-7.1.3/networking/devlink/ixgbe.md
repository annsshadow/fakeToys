
## ixgbe devlink support


鏈枃妗ｆ弿杩颁簡 `ixgbe` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## Info versions


`devlink-info` 鍛堢幇鐨勪笌瀹夊叏鎬х浉鍏崇殑浠讳綍鐗堟湰閮界函绮规槸淇℃伅鎬х殑銆侱evlink 涓嶄娇鐢ㄥ畨鍏ㄩ€氶亾涓庤澶囬€氫俊銆?
`ixgbe` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

    :widths: 5 5 5 90

    - - Name
      - Type
      - Example
      - Description
    - - `board.id`
      - fixed
      - H49289-000
      - 鏉垮崱鐨勪骇鍝佹澘缁勪欢锛圥BA锛夋爣璇嗙銆?    - - `fw.undi`
      - running
      - 1.1937.0
      - 鍖呭惈 UEFI 椹卞姩鐨?Option ROM 鐗堟湰銆傜増鏈互 `major.minor.patch` 鏍煎紡鎶ュ憡銆備富鐗堟湰鍦ㄤ换浣曢噸澶т笉鍏煎鍙樻洿鍙戠敓鏃堕€掑锛屾垨鍦ㄦ鐗堟湰灏嗘孩鍑烘椂閫掑銆傛鐗堟湰鍦ㄩ潪鐮村潖鎬у彉鏇存椂閫掑锛屽苟鍦ㄤ富鐗堟湰閫掑鏃堕噸缃负 1銆傝ˉ涓佺増鏈€氬父涓?0锛屼絾褰撲慨澶嶄綔涓洪拡瀵硅緝鏃у熀纭€ Option ROM 鐨勮ˉ涓佷氦浠樻椂閫掑銆?    - - `fw.undi.srev`
      - running
      - 4
      - 鎸囩ず Option ROM 瀹夊叏淇鍙风殑缂栧彿銆?    - - `fw.bundle_id`
      - running
      - 0x80000d0d
      - 鍔犺浇鍒拌澶囦笂鐨勫浐浠舵槧鍍忔枃浠剁殑鍞竴鏍囪瘑绗︺€備篃绉颁负 NVM 鐨?EETRACK 鏍囪瘑绗︺€?    - - `fw.mgmt.api`
      - running
      - 1.5.1
      - 鐢辩鐞嗗浐浠堕€氳繃 AdminQ 瀵煎嚭鐨?API 鐨?3 浣嶇増鏈彿锛坢ajor.minor.patch锛夈€傞┍鍔ㄧ敤瀹冩潵璇嗗埆鏀寔鍝簺鍛戒护銆傚巻鍙茬増鏈殑鍐呮牳鍙樉绀?2 浣嶇増鏈彿锛坢ajor.minor锛夈€?    - - `fw.mgmt.build`
      - running
      - 0x305d955f
      - 绠＄悊鍥轰欢鏉ユ簮鐨勫敮涓€鏍囪瘑绗︺€?    - - `fw.mgmt.srev`
      - running
      - 3
      - 鎸囩ず鍥轰欢瀹夊叏淇鍙风殑缂栧彿銆?    - - `fw.psid.api`
      - running
      - 0.80
      - 瀹氫箟闂瓨鍐呭鏍煎紡鐨勭増鏈€?    - - `fw.netlist`
      - running
      - 1.1.2000-6.7.0
      - netlist 妯″潡鐨勭増鏈€傝妯″潡瀹氫箟璁惧鐨勪互澶綉鑳藉姏鍜岄粯璁よ缃紝骞惰绠＄悊鍥轰欢鐢ㄤ綔绠＄悊閾捐矾鍜岃澶囪繛鎺ョ殑涓€閮ㄥ垎銆?    - - `fw.netlist.build`
      - running
      - 0xee16ced7
      - netlist 妯″潡鍐呭鍝堝笇鐨勫墠 4 涓瓧鑺傘€?
## Flash Update


`ixgbe` 椹卞姩瀹炵幇浜嗕娇鐢?`devlink-flash` 鎺ュ彛鐨勯棯瀛樻洿鏂版敮鎸併€傚畠鏀寔浣跨敤鍖呭惈 `fw.mgmt`銆乣fw.undi` 鍜?`fw.netlist` 缁勪欢鐨勫悎骞堕棯瀛樻槧鍍忔潵鏇存柊璁惧闂瓨銆?
   :widths: 5 95

   - - Bits
     - Behavior
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 涓嶄繚鐣欐鍦ㄦ洿鏂扮殑闂瓨缁勪欢涓瓨鍌ㄧ殑璁剧疆銆傝繖鍖呮嫭瑕嗙洊鍐冲畾璁惧灏嗗垵濮嬪寲涓哄灏戠墿鐞嗗姛鑳界殑绔彛閰嶇疆銆?   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS` and `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 鏃笉淇濈暀璁剧疆涔熶笉淇濈暀鏍囪瘑绗︺€傜敤鎵€鎻愪緵鏄犲儚鐨勫唴瀹硅鐩栭棯瀛樹腑鐨勪竴鍒囷紝涓嶈繘琛屼换浣曚繚鐣欍€傝繖鍖呮嫭瑕嗙洊璁惧鏍囪瘑瀛楁锛屼緥濡?MAC 鍦板潃銆侀噸瑕佷骇鍝佹暟鎹紙VPD锛夊尯鍩熷拰璁惧搴忓垪鍙枫€傛缁勫悎棰勬湡鐢ㄤ簬閽堝鐗瑰畾璁惧瀹氬埗鐨勬槧鍍忋€?
## Reload


`ixgbe` 椹卞姩鏀寔鍦ㄩ棯瀛樻洿鏂板悗浣跨敤甯︽湁 `DEVLINK_RELOAD_ACTION_FW_ACTIVATE` 鍔ㄤ綔鐨?`DEVLINK_CMD_RELOAD` 鏉ユ縺娲绘柊鍥轰欢銆?

    $ devlink dev reload pci/0000:01:00.0 reload action fw_activate

鏂板浐浠堕€氳繃鍙戝嚭璁惧鐗瑰畾鐨勫祵鍏ュ紡绠＄悊澶勭悊鍣紙Embedded Management Processor锛夐噸缃潵婵€娲伙紝璇ラ噸缃姹傝澶囬噸缃苟閲嶆柊鍔犺浇 EMP 鍥轰欢鏄犲儚銆?
椹卞姩褰撳墠涓嶆敮鎸侀€氳繃 `DEVLINK_RELOAD_ACTION_DRIVER_REINIT` 閲嶆柊鍔犺浇椹卞姩銆?
## Regions


`ixgbe` 椹卞姩瀹炵幇浜嗕互涓嬬敤浜庤闂唴閮ㄨ澶囨暟鎹殑鍖哄煙銆?
    :widths: 15 85

    - - Name
      - Description
    - - `nvm-flash`
      - 鏁翠釜闂瓨鑺墖鐨勫唴瀹癸紝鏈夋椂琚О涓鸿澶囩殑闈炴槗澶辨€у瓨鍌ㄥ櫒锛圢on Volatile Memory锛夈€?    - - `shadow-ram`
      - Shadow RAM 鐨勫唴瀹癸紝瀹冧粠闂瓨寮€澶村姞杞姐€傚敖绠″唴瀹逛富瑕佹潵鑷棯瀛橈紝浣嗚鍖哄煙杩樺寘鍚澶囧惎鍔ㄦ湡闂寸敓鎴愩€佹湭瀛樺偍鍦ㄩ棯瀛樹腑鐨勬暟鎹€?    - - `device-caps`
      - 璁惧鍥轰欢鑳藉姏缂撳啿鍖虹殑鍐呭銆傛湁鍔╀簬纭畾璁惧鐨勫綋鍓嶇姸鎬佸拰閰嶇疆銆?
`nvm-flash` 鍜?`shadow-ram` 鍖哄煙閮藉彲浠ュ湪涓嶅揩鐓х殑鎯呭喌涓嬭闂€俙device-caps` 鍖哄煙闇€瑕佸揩鐓э紝鍥犱负鍐呭鐢卞浐浠跺彂閫佷笖鏃犳硶鎷嗗垎涓哄崟鐙殑璇诲彇銆?
鐢ㄦ埛鍙互閫氳繃 `DEVLINK_CMD_REGION_NEW` 鍛戒护璇锋眰绔嬪嵆涓烘墍鏈変笁涓尯鍩熸崟鑾峰揩鐓с€?

    $ devlink region show
    pci/0000:01:00.0/nvm-flash: size 10485760 snapshot [] max 1
    pci/0000:01:00.0/device-caps: size 4096 snapshot [] max 10

    $ devlink region new pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    $ devlink region read pci/0000:01:00.0/nvm-flash snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    $ devlink region delete pci/0000:01:00.0/device-caps snapshot 1
