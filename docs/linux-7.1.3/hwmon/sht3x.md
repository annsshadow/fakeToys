## 鍐呮牳椹卞姩 sht3x


Supported chips:

  - Sensirion SHT3x-DIS

    Prefix: 'sht3x'

    Addresses scanned: none

    Datasheets:
        - https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf
        - https://sensirion.com/media/documents/051DF50B/639C8101/Sensirion_Humidity_and_Temperature_Sensors_Datasheet_SHT33.pdf

  - Sensirion STS3x-DIS

    Prefix: 'sts3x'

    Addresses scanned: none

    Datasheets:
        - https://sensirion.com/media/documents/1DA31AFD/61641F76/Sensirion_Temperature_Sensors_STS3x_Datasheet.pdf
        - https://sensirion.com/media/documents/292A335C/65537BAF/Sensirion_Datasheet_STS32_STS33.pdf

  - Sensirion SHT85

    Prefix: 'sht85'

    Addresses scanned: none

    Datasheet: https://sensirion.com/media/documents/4B40CEF3/640B2346/Sensirion_Humidity_Sensors_SHT85_Datasheet.pdf

Author:

  - David Frey <david.frey@sensirion.com>
  - Pascal Sachs <pascal.sachs@sensirion.com>

### 鎻忚堪


璇ラ┍鍔ㄥ疄鐜颁簡瀵?Sensirion SHT3x-DIS銆丼TS3x-DIS 浠ュ強 SHT85 绯诲垪娓╂箍搴︿紶鎰熷櫒鐨?鏀寔銆傛俯搴︿互鎽勬皬搴︿负鍗曚綅娴嬮噺锛岀浉瀵规箍搴︿互鐧惧垎姣旇〃绀恒€傚湪 sysfs 鎺ュ彛涓紝鎵€鏈?鍊奸兘鏀惧ぇ 1000 鍊嶏紝鍗?31.5 鎽勬皬搴﹀搴旂殑鍊间负 31500銆?
璇ュ櫒浠堕€氳繃 I2C 鍗忚閫氫俊銆係HT3x 浼犳劅鍣ㄦ牴鎹帴绾夸笉鍚岋紝I2C 鍦板潃鍙互鏄?0x44 鎴?0x45锛坰ts3x 涓?0x4a 鎴?0x4b锛夈€係HT85 鐨勫湴鍧€鍥哄畾涓?0x44銆傚疄渚嬪寲璇ュ櫒浠剁殑鏂规硶
璇峰弬闃?Documentation/i2c/instantiating-devices.rst銆?
灏界 sht3x 浼犳劅鍣ㄥ湪鍗曟妯″紡涓嬫敮鎸佹椂閽熷欢灞曪紙闃诲妯″紡锛夊拰闈炲欢灞曪紙闈為樆濉炴ā寮忥級锛?鏈┍鍔ㄤ粎鏀寔鍚庤€呫€?
sht3x 浼犳劅鍣ㄦ敮鎸佸崟娆℃祴閲忔ā寮忎互鍙?5 绉嶅懆鏈熸祴閲忔ā寮忥紝鍙€氳繃 update_interval
sysfs 鎺ュ彛鎺у埗銆傛墍鍏佽鐨?update_interval锛堝崟浣嶆绉掞級濡備笅锛?
    ===== ======= ====================
       0          鍗曟娴嬮噺妯″紡
    2000   0.5 Hz  鍛ㄦ湡娴嬮噺
    1000   1   Hz  鍛ㄦ湡娴嬮噺
     500   2   Hz  鍛ㄦ湡娴嬮噺
     250   4   Hz  鍛ㄦ湡娴嬮噺
     100  10   Hz  鍛ㄦ湡娴嬮噺
    ===== ======= ====================

鍦ㄥ懆鏈熸祴閲忔ā寮忎笅锛屼紶鎰熷櫒浠ヨ姱鐗囦笂閰嶇疆鐨勬洿鏂伴棿闅旇嚜鍔ㄨЕ鍙戞祴閲忋€傚綋娓╁害鎴栨箍搴﹁鏁?瓒呭嚭閰嶇疆鐨勯檺鍊兼椂锛宎lert 灞炴€ц缃负 1锛屼笖浼犳劅鍣ㄤ笂鐨?alert 寮曡剼琚疆涓洪珮鐢靛钩銆?褰撴俯搴﹀拰婀垮害璇绘暟鍥炲埌杩熸粸鍊间箣闂存椂锛宎lert 浣嶈缃负 0锛屼紶鎰熷櫒涓婄殑 alert 寮曡剼琚?缃负浣庣數骞炽€?
鏆撮湶鍒?debugfs 鐨勫簭鍒楀彿鍙敤浜庡浼犳劅鍣ㄨ繘琛屽敮涓€鏍囪瘑銆傚浜?sts32銆乻ts33 鍜?sht33锛?鍒堕€犲晢閫氳繃 API 鎻愪緵鏍″噯璇佷功銆?
### sysfs 鎺ュ彛


=================== ============================================================
temp1_input:        娓╁害杈撳叆鍊?humidity1_input:    婀垮害杈撳叆鍊?temp1_max:          娓╁害鏈€澶у€?temp1_max_hyst:     娓╁害涓婇檺鐨勮繜婊炲€?humidity1_max:      婀垮害鏈€澶у€?humidity1_max_hyst: 婀垮害涓婇檺鐨勮繜婊炲€?temp1_min:          娓╁害鏈€灏忓€?temp1_min_hyst:     娓╁害涓嬮檺鐨勮繜婊炲€?humidity1_min:      婀垮害鏈€灏忓€?humidity1_min_hyst: 婀垮害涓嬮檺鐨勮繜婊炲€?temp1_alarm:        鑻ユ俯搴﹁秴鍑洪厤缃殑闄愬€硷紝鍛婅鏍囧織琚疆涓?1銆傚憡璀︿粎鍦ㄥ懆鏈熸祴閲?		    妯″紡涓嬫湁鏁?humidity1_alarm:    鑻ユ箍搴﹁秴鍑洪厤缃殑闄愬€硷紝鍛婅鏍囧織琚疆涓?1銆傚憡璀︿粎鍦ㄥ懆鏈熸祴閲?		    妯″紡涓嬫湁鏁?heater_enable:      鍔犵儹鍣ㄤ娇鑳斤紝鍔犵儹鍏冧欢鐢ㄤ簬鍘婚櫎浼犳劅鍣ㄤ笂澶氫綑鐨勬箍姘旓細

   - 0: 鍏抽棴
   - 1: 寮€鍚?update_interval:    鏇存柊闂撮殧锛? 琛ㄧず鍗曟妯″紡锛屽懆鏈熸祴閲忔椂鍗曚綅涓烘绉掋€傝嫢浼犳劅鍣?		    涓嶆敮鎸佽闂撮殧锛屽垯閫夋嫨涓嬩竴涓洿蹇殑闂撮殧
repeatability:      鍐欏叆鎴栬鍙栭噸澶嶇簿搴︼紝閲嶅绮惧害瓒婇珮鎰忓懗鐫€娴嬮噺鑰楁椂鏇撮暱銆佸櫔澹?                    鏇翠綆銆佽兘鑰楁洿澶э細

                        - 0: 浣庨噸澶嶇簿搴?                        - 1: 涓噸澶嶇簿搴?                        - 2: 楂橀噸澶嶇簿搴?=================== ============================================================

### debugfs 鎺ュ彛


=================== ============================================================
serial_number:      浼犳劅鍣ㄧ殑鍞竴搴忓垪鍙凤紙鍗佽繘鍒讹級
=================== ============================================================
