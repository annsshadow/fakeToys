
## 鍐呮牳椹卞姩 sht4x


鏀寔鐨勮澶囷細

  - Sensirion SHT4X

    Prefix: 'sht4x'

    Addresses scanned: None

    Datasheet:

      English: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/2_Humidity_Sensors/Datasheets/Sensirion_Humidity_Sensors_SHT4x_Datasheet.pdf

Author: Navin Sankar Velliangiri <navin@linumiz.com>


### 鎻忚堪


鏈┍鍔ㄥ疄鐜板 Sensirion SHT4x 鑺墖锛堜竴娆炬箍搴︿笌娓╁害浼犳劅鍣級鐨勬敮鎸併€傛俯搴︿互鎽勬皬搴﹀害閲忥紝鐩稿婀垮害浠ョ櫨鍒嗘瘮琛ㄧず銆傚湪 sysfs 鎺ュ彛涓紝鎵€鏈夋暟鍊煎潎涔樹互 1000锛屽嵆 31.5 鎽勬皬搴﹀搴旂殑鍊间负 31500銆?
### 浣跨敤璇存槑


璇ュ櫒浠堕€氳繃 I2C 鍗忚閫氫俊銆備紶鎰熷櫒鍙娇鐢?I2C 鍦板潃 0x44銆傚叧浜庡疄渚嬪寲璇ュ櫒浠剁殑鏂规硶锛岃鍙傞槄 Documentation/i2c/instantiating-devices.rst銆?
### Sysfs 鏉＄洰


=============== ============================================
temp1_input     娴嬮噺鐨勬俯搴︼紝鍗曚綅姣憚姘忓害
humidity1_input 娴嬮噺鐨勬箍搴︼紝鍗曚綅 %H
update_interval 杞浼犳劅鍣ㄧ殑鏈€灏忛棿闅旓紝鍗曚綅姣銆傚彲鍐欍€傚繀椤昏嚦灏戜负 2000銆?heater_power	璇锋眰鐨勫姞鐑櫒鍔熺巼锛屽崟浣嶆鐡︺€?		鍙敤鍊硷細20銆?10銆?00锛堥粯璁わ細200锛夈€?heater_time	璇锋眰鐨勫姞鐑櫒宸ヤ綔鏃堕棿锛屽崟浣嶆绉掋€?		鍙敤鍊硷細100銆?000锛堥粯璁?1000锛夈€?heater_enable	浠ユ墍閫夊姛鐜囥€佸湪鎵€閫夋椂闂村唴鍚敤鍔犵儹鍣紝浠ュ幓闄や紶鎰熷櫒琛ㄩ潰鐨勫喎鍑濇按銆備竴鏃﹀惎鐢ㄤ究鏃犳硶鎵嬪姩鍏抽棴锛堝畬鎴愭搷浣滃悗鑷姩鍏抽棴锛夈€?
   - 0: 鍏抽棴锛堝彧璇诲€硷級
   - 1: 寮€鍚?=============== ============================================
