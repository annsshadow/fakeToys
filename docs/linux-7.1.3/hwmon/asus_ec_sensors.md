
## 鍐呮牳椹卞姩 asus_ec_sensors


Supported boards:
 - MAXIMUS VI HERO
 - PRIME X470-PRO
 - PRIME X570-PRO
 - PRIME X670E-PRO WIFI
 - PRIME Z270-A
 - Pro WS TRX50-SAGE WIFI
 - Pro WS TRX50-SAGE WIFI A
 - Pro WS X570-ACE
 - Pro WS WRX90E-SAGE SE
 - ProArt X570-CREATOR WIFI
 - ProArt X670E-CREATOR WIFI
 - ProArt X870E-CREATOR WIFI
 - ProArt B550-CREATOR
 - ROG CROSSHAIR VIII DARK HERO
 - ROG CROSSHAIR VIII HERO (WI-FI)
 - ROG CROSSHAIR VIII FORMULA
 - ROG CROSSHAIR VIII HERO
 - ROG CROSSHAIR VIII IMPACT
 - ROG CROSSHAIR X670E EXTREME
 - ROG CROSSHAIR X670E HERO
 - ROG CROSSHAIR X670E GENE
 - ROG MAXIMUS X HERO
 - ROG MAXIMUS XI HERO
 - ROG MAXIMUS XI HERO (WI-FI)
 - ROG MAXIMUS Z690 FORMULA
 - ROG STRIX B550-E GAMING
 - ROG STRIX B550-I GAMING
 - ROG STRIX B650E-I GAMING WIFI
 - ROG STRIX B850-I GAMING WIFI
 - ROG STRIX X470-F GAMING
 - ROG STRIX X470-I GAMING
 - ROG STRIX X570-E GAMING
 - ROG STRIX X570-E GAMING WIFI II
 - ROG STRIX X570-F GAMING
 - ROG STRIX X570-I GAMING
 - ROG STRIX X670E-E GAMING WIFI
 - ROG STRIX X670E-I GAMING WIFI
 - ROG STRIX X870-F GAMING WIFI
 - ROG STRIX X870-I GAMING WIFI
 - ROG STRIX X870E-E GAMING WIFI
 - ROG STRIX X870E-H GAMING WIFI7
 - ROG STRIX Z390-F GAMING
 - ROG STRIX Z490-F GAMING
 - ROG STRIX Z690-A GAMING WIFI D4
 - ROG STRIX Z690-E GAMING WIFI
 - ROG STRIX Z790-E GAMING WIFI II
 - ROG STRIX Z790-H GAMING WIFI
 - ROG STRIX Z790-I GAMING WIFI
 - ROG ZENITH II EXTREME
 - ROG ZENITH II EXTREME ALPHA
 - TUF GAMING X670E PLUS
 - TUF GAMING X670E PLUS WIFI

Authors:
    - Eugene Shalygin <eugene.shalygin@gmail.com>

### 鎻忚堪锛?
鍗庣锛圓SUS锛変富鏉块€氳繃 Super I/O 鑺墖浠ュ強 ACPI 宓屽叆寮忔帶鍒跺櫒锛圗C锛夊瘎瀛樺櫒鍙戝竷纭欢鐩戞帶淇℃伅銆傚叾涓儴鍒嗕紶鎰熷櫒鍙兘閫氳繃 EC 鑾峰彇銆?
璇ラ┍鍔ㄨ兘澶熻瘑鍒苟璇诲彇浠ヤ笅浼犳劅鍣細

1. 鑺墖缁勶紙PCH锛夋俯搴?2. CPU 灏佽娓╁害
3. 涓绘澘娓╁害
4. T_Sensor 鎺ュご鐨勮鏁?5. VRM 娓╁害
6. CPU_Opt 椋庢墖杞€燂紙RPM锛?7. VRM 鏁ｇ儹鐗囬鎵囪浆閫燂紙RPM锛?8. 鑺墖缁勯鎵囪浆閫燂紙RPM锛?9. 鈥淲ater flow meter锛堟按娴侀噺璁★級鈥濇帴澶寸殑璇绘暟锛圧PM锛?10. 鈥淲ater In锛堣繘姘达級鈥濅笌鈥淲ater Out锛堝嚭姘达級鈥濇俯搴︽帴澶寸殑璇绘暟
11. CPU 鐢垫祦
12. CPU 鏍稿績鐢靛帇

浼犳劅鍣ㄦ暟鍊间粠 EC 瀵勫瓨鍣ㄨ鍙栵紝涓洪伩鍏嶄笌涓绘澘鍥轰欢绔炰簤锛岄┍鍔ㄤ細鑾峰彇 ACPI 浜掓枼浣擄紙mutex锛夆€斺€斿嵆 WMI 鍦ㄨ闂?EC 鐨勬柟娉曚腑鎵€浣跨敤鐨勯偅涓簰鏂ヤ綋銆?
### 妯″潡鍙傛暟

 - mutex_path锛氬瓧绗︿覆
		椹卞姩涓烘瘡涓富鏉夸繚瀛樹簡 ACPI 浜掓枼浣撶殑璺緞锛堝疄闄呬笂锛岃繖浜涜矾寰勫ぇ浣撶浉鍚岋級銆傚鏋?ASUS 鍦ㄦ湭鏉ョ殑 BIOS 鏇存柊涓敼鍙樹簡璇ヨ矾寰勶紝鍦ㄦ湰椹卞姩鏇存柊涔嬪墠锛屽彲浣跨敤姝ゅ弬鏁版潵瑕嗙洊鎵€淇濆瓨鐨勫€笺€?		涔熷彲浼犲叆鐗规畩瀛楃涓?":GLOBAL_LOCK" 浠ヤ娇鐢?ACPI 鍏ㄥ眬閿侊紝鑰岄潪涓撶敤浜掓枼浣撱€?