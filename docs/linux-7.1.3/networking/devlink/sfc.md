
## sfc devlink 鏀寔


鏈枃妗ｆ弿杩?`sfc` 璁惧椹卞姩涓?ef10 涓?ef100 璁惧瀹炵幇鐨?devlink 鐗规€с€?
## 淇℃伅鐗堟湰


`sfc` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `fw.bundle_id`
     - stored
     - 涓婃鐢ㄤ簬鏇存柊澶氫釜缁勪欢鐨勫浐浠垛€渂undle鈥濋暅鍍忕殑鐗堟湰銆?   - - `fw.mgmt.suc`
     - running
     - 瀵逛簬绠＄悊鍔熻兘琚媶鍒嗗埌澶氫釜鎺у埗鍗曞厓鐨勬澘鍗★紝杩欐槸 SUC 鎺у埗鍗曞厓鐨勫浐浠剁増鏈€?   - - `fw.mgmt.cmc`
     - running
     - 瀵逛簬绠＄悊鍔熻兘琚媶鍒嗗埌澶氫釜鎺у埗鍗曞厓鐨勬澘鍗★紝杩欐槸 CMC 鎺у埗鍗曞厓鐨勫浐浠剁増鏈€?   - - `fpga.rev`
     - running
     - FPGA 璁捐淇鐗堛€?   - - `fpga.app`
     - running
     - 鏁版嵁閫氳矾鍙紪绋嬮€昏緫鐗堟湰銆?   - - `fw.app`
     - running
     - 鏁版嵁閫氳矾杞欢/寰爜/鍥轰欢鐗堟湰銆?   - - `coproc.boot`
     - running
     - SmartNIC 搴旂敤鍗忓鐞嗗櫒锛圓PU锛夌涓€闃舵寮曞鍔犺浇鍣ㄧ増鏈€?   - - `coproc.uboot`
     - running
     - SmartNIC 搴旂敤鍗忓鐞嗗櫒锛圓PU锛夊崗鍚屾搷浣滅郴缁熷姞杞藉櫒鐗堟湰銆?   - - `coproc.main`
     - running
     - SmartNIC 搴旂敤鍗忓鐞嗗櫒锛圓PU锛変富鎿嶄綔绯荤粺鐗堟湰銆?   - - `coproc.recovery`
     - running
     - SmartNIC 搴旂敤鍗忓鐞嗗櫒锛圓PU锛夋仮澶嶆搷浣滅郴缁熺増鏈€?   - - `fw.exprom`
     - running
     - 鎵╁睍 ROM 鐗堟湰銆傚浜庢墿灞?ROM 琚媶鍒嗗埌澶氫釜闀滃儚锛堝 PXE 涓?UEFI锛夌殑鏉垮崱锛岃繖涓撻棬鎸?PXE 寮曞 ROM 鐗堟湰銆?   - - `fw.uefi`
     - running
     - UEFI 椹卞姩鐗堟湰锛堟棤 UNDI 鏀寔锛夈€?
## 闂瓨鏇存柊


`sfc` 椹卞姩瀹炵幇瀵逛娇鐢?`devlink-flash` 鎺ュ彛杩涜闂瓨鏇存柊鐨勬敮鎸併€傚畠鏀寔浣跨敤鍖呭惈澶氫釜缁勪欢鐨勭粍鍚堥棯瀛橀暅鍍忥紙鈥渂undle鈥濓級鏇存柊璁惧闂瓨锛堝湪 ef10 涓婏紝閫氬父涓?`fw.mgmt`銆乣fw.app`銆乣fw.exprom` 涓?`fw.uefi`锛夈€?
璇ラ┍鍔ㄤ笉鏀寔浠讳綍瑕嗗啓鎺╃爜锛坥verwrite mask锛夋爣蹇椼€?