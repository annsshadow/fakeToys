
## Devlink E-Switch 灞炴€?


Devlink E-Switch 鏀寔涓ょ鎿嶄綔妯″紡锛歭egacy 涓?switchdev銆侺egacy 妯″紡鍩轰簬浼犵粺鐨?MAC/VLAN 瀵煎悜瑙勫垯杩愯銆傚垏鎹㈠喅绛栧熀浜?MAC 鍦板潃銆乂LAN 绛夊仛鍑恒€傚皢鍒囨崲瑙勫垯鍗歌浇鍒扮‖浠剁殑鑳藉姏鏈夐檺銆?

鍙︿竴鏂归潰锛宻witchdev 妯″紡鍏佽灏?E-Switch 鏇村鍦伴珮绾у嵏杞借兘鍔涗氦缁欑‖浠躲€傚湪 switchdev 妯″紡涓嬶紝鏇村鐨勫垏鎹㈣鍒欎笌閫昏緫鍙互琚嵏杞藉埌纭欢浜ゆ崲 ASIC 涓娿€傚畠鍚敤浜嗕唬琛ㄨ澶囪櫄鎷熷姛鑳斤紙VF锛夋垨鍙墿灞曞姛鑳斤紙SF锛夋參閫熻矾寰勭殑 representor netdevices銆傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃?Documentation/networking/switchdev.rst <switchdev> 涓?Documentation/networking/representors.rst <representors>銆?

姝ゅ锛宒evlink E-Switch 杩橀檮甯︿簡涓嬩竴鑺傚垪鍑虹殑鍏朵粬灞炴€с€?

## 灞炴€ф弿杩?


浠ヤ笅鏄?E-Switch 灞炴€х殑鍒楄〃銆?

   :widths: 8 5 45

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `mode`
     - enum
     - 璁惧鐨勬ā寮忋€傛ā寮忓彲浠ユ槸浠ヤ笅涔嬩竴锛?

       - `legacy` 鍩轰簬浼犵粺 MAC/VLAN 瀵煎悜瑙勫垯杩愯銆?
       - `switchdev` 鍏佽灏?E-Switch 鏇村鍦伴珮绾у嵏杞借兘鍔涗氦缁欑‖浠躲€?
       - `switchdev_inactive` switchdev 妯″紡浣嗗惎鍔ㄦ椂澶勪簬闈炴縺娲荤姸鎬侊紝鍦ㄦ樉寮忔縺娲诲墠涓嶅厑璁告祦閲忛€氳繃銆傛妯″紡瀵逛簬甯屾湜浠?switchdev 妯″紡鍑嗗璁惧銆佷絾浠呭湪鎵€鏈夐厤缃畬鎴愬悗鎵嶆縺娲诲畠鐨勭紪鎺掑櫒寰堟湁鐢ㄣ€?
   - - `inline-mode`
     - enum
     - 鏌愪簺纭欢闇€瑕?VF 椹卞姩灏嗛儴鍒嗘暟鎹寘澶撮儴鏀惧叆 TX 鎻忚堪绗︼紝浠ヤ究 e-switch 鑳藉杩涜姝ｇ‘鐨勫尮閰嶄笌瀵煎悜銆俿witchdev 妯″紡涓?legacy 妯″紡鍧囨敮鎸併€?

       - `none` 鏃犮€?
       - `link` L2 妯″紡銆?
       - `network` L3 妯″紡銆?
       - `transport` L4 妯″紡銆?
   - - `encap-mode`
     - enum
     - 璁惧鐨勫皝瑁呮ā寮忋€俿witchdev 妯″紡涓?legacy 妯″紡鍧囨敮鎸併€傛ā寮忓彲浠ユ槸浠ヤ笅涔嬩竴锛?

       - `none` 绂佺敤灏佽鏀寔銆?
       - `basic` 鍚敤灏佽鏀寔銆?

## 浣跨敤绀轰緥


    # 鍚敤 switchdev 妯″紡
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev

    # 璁剧疆 inline-mode 涓?encap-mode
    $ devlink dev eswitch set pci/0000:08:00.0 inline-mode none encap-mode basic

    # 鏄剧ず devlink 璁惧鐨?eswitch 灞炴€?
    $ devlink dev eswitch show pci/0000:08:00.0
      pci/0000:08:00.0: mode switchdev inline-mode none encap-mode basic

    # 鍦?legacy 妯″紡涓嬪惎鐢?encap-mode
    $ devlink dev eswitch set pci/0000:08:00.0 mode legacy inline-mode none encap-mode basic

    # 浠ラ潪婵€娲荤姸鎬佸惎鍔?switchdev 妯″紡
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev_inactive

    # 閰嶇疆 switchdev 鐨勮缃€乺epresentors銆丗DB 鏉＄洰绛?.
    ...

    # 婵€娲?switchdev 妯″紡浠ュ厑璁告祦閲忛€氳繃
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev
