

## Devlink Flash


`devlink-flash` API 鍏佽鏇存柊璁惧鍥轰欢銆傚畠鍙栦唬浜嗚緝鏃х殑 `ethtool-flash` 鏈哄埗锛屽苟涓斾笉闇€瑕佸彇涓嬩换浣?```

  $ devlink dev flash pci/0000:05:00.0 file flash-boot.bin

```
娉ㄦ剰锛屾枃浠跺悕鏄浉瀵逛簬鍥轰欢鍔犺浇璺緞锛堥€氬父鏄?`/lib/firmware/`锛夌殑璺緞銆傞┍鍔ㄥ彲鑳藉彂閫佺姸鎬佹洿鏂帮紝浠ラ€氱煡鐢ㄦ埛绌洪棿
鏇存柊鎿嶄綔鐨勮繘搴︺€?
## Overwrite Mask锛堣鐩栨帺鐮侊級


`devlink-flash` 鍛戒护鍏佽鍙€夊湴鎸囧畾涓€涓帺鐮侊紝鎸囩ず璁惧鍦ㄦ洿鏂版椂搴斿浣曞鐞嗛棯瀛樼粍浠剁殑瀛愭銆傛鎺╃爜鎸囩ず鍏佽
琚鐩栫殑娈甸泦鍚堛€?
   :widths: 5 95

   - - 鍚嶇О
     - 鎻忚堪
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 鎸囩ず璁惧搴斾娇鐢ㄦ墍鎻愪緵鏄犲儚涓殑璁剧疆瑕嗙洊姝ｅ湪鏇存柊鐨勭粍浠朵腑鐨勮缃€?   - - `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 鎸囩ず璁惧搴斾娇鐢ㄦ墍鎻愪緵鏄犲儚涓殑鏍囪瘑绗﹁鐩栨鍦ㄦ洿鏂扮殑缁勪欢涓殑鏍囪瘑绗︺€傝繖鍖呮嫭 MAC 鍦板潃銆佸簭鍒?ID 浠ュ強
       绫讳技鐨勮澶囨爣璇嗙銆?
鍙互缁勫悎骞朵竴璧疯姹傚涓鐩栦綅銆傚鏋滄湭鎻愪緵浠讳綍浣嶏紝鍒欐湡鏈涜澶囧彧鏇存柊姝ｅ湪鏇存柊鐨勭粍浠朵腑鐨勫浐浠朵簩杩涘埗銆傝缃拰
鏍囪瘑绗﹀簲璺ㄦ洿鏂拌淇濈暀銆傝澶囧彲鑳戒笉鏀寔姣忕缁勫悎锛屾绫昏澶囩殑椹卞姩蹇呴』鎷掔粷浠讳綍鏃犳硶蹇犲疄瀹炵幇鐨勭粍鍚堛€?
## 鍥轰欢鍔犺浇


闇€瑕佸浐浠舵墠鑳借繍琛岀殑璁惧閫氬父灏嗗叾瀛樺偍鍦ㄦ澘涓婄殑闈炴槗澶辨€у瓨鍌ㄥ櫒涓紝渚嬪闂瓨銆傛湁浜涜澶囧彧鍦ㄦ澘涓婂瓨鍌ㄥ熀鏈浐浠讹紝
椹卞姩鍦ㄦ帰娴嬫湡闂翠粠纾佺洏鍔犺浇鍏朵綑閮ㄥ垎銆俙devlink-info` 鍏佽鐢ㄦ埛鏌ヨ鍥轰欢淇℃伅锛堝凡鍔犺浇鐨勭粍浠跺拰鐗堟湰锛夈€?
鍦ㄥ叾浠栨儏鍐典笅锛岃澶囨棦鍙互灏嗘槧鍍忓瓨鍌ㄥ湪鏉夸笂銆佷粠纾佺洏鍔犺浇锛屼篃鍙互鑷姩浠庣鐩樺埛鍐欐柊鏄犲儚銆俙fw_load_policy`
devlink 鍙傛暟鍙敤浜庢帶鍒舵琛屼负
锛圖ocumentation/networking/devlink/devlink-params.rst <devlink_params_generic>锛夈€?
纾佺洏涓婄殑鍥轰欢鏂囦欢閫氬父瀛樺偍鍦?`/lib/firmware/`銆?
## 鍥轰欢鐗堟湰绠＄悊


鏈熸湜椹卞姩瀹炵幇 `devlink-flash` 鍜?`devlink-info` 鍔熻兘锛屽畠浠叡鍚屽厑璁稿疄鐜颁笌渚涘簲鍟嗘棤鍏崇殑鑷姩鍖栧浐浠舵洿鏂拌鏂姐€?
`devlink-info` 鏆撮湶 `driver` 鍚嶇О浠ュ強涓変釜鐗堟湰缁勶紙`fixed`銆乣running`銆乣stored`锛夈€?
`driver` 灞炴€у拰 `fixed` 缁勬爣璇嗙壒瀹氱殑璁惧璁捐锛屼緥濡傜敤浜庢煡鎵鹃€傜敤鐨勫浐浠舵洿鏂般€傝繖灏辨槸涓轰粈涔?`serial_number`
涓嶆槸 `fixed` 鐗堟湰鐨勪竴閮ㄥ垎锛堝嵆浣垮畠鏄浐瀹氱殑锛夆€斺€擿fixed` 鐗堟湰搴旀爣璇嗚璁★紝鑰岄潪鍗曚釜璁惧銆?
`running` 鍜?`stored` 鍥轰欢鐗堟湰鏍囪瘑璁惧涓婅繍琛岀殑鍥轰欢锛屼互鍙婂皢鍦ㄩ噸鍚垨璁惧閲嶇疆鍚庢縺娲荤殑鍥轰欢銆?
鍥轰欢鏇存柊浠ｇ悊搴旇鑳藉閬靛惊杩欎釜绠€鍗曠畻娉曟潵鏇存柊鍥轰欢鍐呭锛岃€屼笌璁惧渚涘簲鍟嗘棤鍏筹細


  # 鑾峰彇鍞竴鐨勭‖浠惰璁℃爣璇嗙
  $hw_id = devlink-dev-info['fixed']

  # 鏌ユ槑鎴戜滑鎯充负姝?NIC 浣跨敤鍝釜 FW 闂瓨
  $want_flash_vers = some-db-backed.lookup($hw_id, 'flash')

  # 蹇呰鏃舵洿鏂伴棯瀛?  if $want_flash_vers != devlink-dev-info['stored']:
      $file = some-db-backed.download($hw_id, 'flash')
      devlink-dev-flash($file)

  # 鏌ユ槑棰勬湡鐨勬暣浣撳浐浠剁増鏈?  $want_fw_vers = some-db-backed.lookup($hw_id, 'all')

  # 蹇呰鏃舵洿鏂扮鐩樹笂鐨勬枃浠?  if $want_fw_vers != devlink-dev-info['running']:
      $file = some-db-backed.download($hw_id, 'disk')
      write($file, '/lib/firmware/')

  # 灏濊瘯璁惧閲嶇疆锛堝鏋滃彲鐢級
  if $want_fw_vers != devlink-dev-info['running']:
     devlink-reset()

  # 閲嶅惎锛堝鏋滈噸缃笉澶燂級
  if $want_fw_vers != devlink-dev-info['running']:
     reboot()

娉ㄦ剰锛屾浼唬鐮佷腑姣忔瀵?`devlink-dev-info` 鐨勫紩鐢ㄩ兘鏈熸湜浠庡唴鏍歌幏鍙栨渶鏂颁俊鎭€?
涓烘柟渚胯瘑鍒浐浠舵枃浠讹紝涓€浜涗緵搴斿晢鍦ㄥ浐浠剁増鏈腑娣诲姞浜?`bundle_id` 淇℃伅銆傛鍏冪増鏈鐩栧涓€愮粍浠剁増鏈紝鍙敤浜?渚嬪鍥轰欢鏂囦欢鍚嶄腑锛堟墍鏈夌粍浠剁増鏈彲鑳戒細鐩稿綋闀匡級銆?