## 鍐呮牳椹卞姩 shtc1


鏀寔鐨勮姱鐗囷細

  - Sensirion SHTC1

    Prefix: 'shtc1'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtc1



  - Sensirion SHTW1

    Prefix: 'shtw1'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtw1



  - Sensirion SHTC3

    Prefix: 'shtc3'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtc3



Author:

  Johannes Winkelmann <johannes.winkelmann@sensirion.com>

### 鎻忚堪


璇ラ┍鍔ㄥ疄鐜颁簡瀵?Sensirion SHTC1銆丼HTW1 鍜?SHTC3 鑺墖鐨勬敮鎸侊紝杩欎簺鑺墖鏄俯婀垮害浼犳劅鍣ㄣ€傛俯搴︿互鎽勬皬搴︿负鍗曚綅娴嬮噺锛岀浉瀵规箍搴︿互鐧惧垎姣旇〃绀恒€?

璇ヨ澶囬€氳繃 I2C 鍗忚閫氫俊銆傛墍鏈変紶鎰熷櫒鐨?I2C 鍦板潃鍧囪涓?0x70銆傚疄渚嬪寲璇ヨ澶囩殑鏂规硶璇峰弬闃?Documentation/i2c/instantiating-devices.rst銆?

鍙€氳繃 shtc1_platform_data 閰嶇疆涓や釜閫夐」锛?

1. 闃诲妯″紡锛堝湪鎵ц娴嬮噺鏃舵媺浣?I2C 鏃堕挓绾匡級鎴?
   闈為樆濉炴ā寮忋€傞樆濉炴ā寮忚兘淇濊瘉鏈€蹇殑缁撴灉锛屼絾
   I2C 鎬荤嚎鍦ㄦ鏈熼棿灏嗗浜庣箒蹇欑姸鎬併€傞粯璁や娇鐢ㄩ潪闃诲妯″紡銆?
   濡傛灉瑕佷娇鐢ㄩ樆濉炴ā寮忥紝璇风‘淇濊澶囦笂鐨勬椂閽熷欢灞曪紙clock-stretching锛夊伐浣滄甯搞€?
2. 楂樼簿搴︽垨浣庣簿搴︺€傞粯璁や娇鐢ㄩ珮绮惧害锛屽己鐑堝缓璁娇鐢ㄩ珮绮惧害銆?

### sysfs 鎺ュ彛


temp1_input
 - 娓╁害杈撳叆
humidity1_input
 - 婀垮害杈撳叆
