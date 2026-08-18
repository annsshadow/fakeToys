
## 鍐呮牳椹卞姩 gpd-fan


Author:
    - Cryolitia PukNgae <cryolitia@uniontech.com>

### 鎻忚堪


娣卞湷 GPD 绉戞妧鏈夐檺鍏徃锛圫henzhen GPD Technology Co., Ltd.锛夌殑鎵嬫寔璁惧閫氳繃鍏跺祵鍏ュ紡鎺у埗鍣ㄦ彁渚涢鎵囪鏁板拰椋庢墖鎺у埗銆?
### 鏀寔鐨勮澶?

鐩墠璇ラ┍鍔ㄦ敮鎸佷互涓嬫墜鎸佽澶囷細

 - GPD Win Mini (7840U)
 - GPD Win Mini (8840U)
 - GPD Win Mini (HX370)
 - GPD Pocket 4
 - GPD Duo
 - GPD Win Max 2 (6800U)
 - GPD Win Max 2 2023 (7840U)
 - GPD Win Max 2 2024 (8840U)
 - GPD Win Max 2 2025 (HX370)
 - GPD Win 4 (6800U)
 - GPD Win 4 (7840U)
 - GPD Micro PC 2

### 妯″潡鍙傛暟


gpd_fan_board
  寮哄埗鎸囧畾搴斾娇鐢ㄥ摢涓ā鍧?quirk銆?  浣跨敤鏂瑰紡濡?"gpd_fan_board=wm2"銆?
   - wm2
       - GPD Win 4 (7840U)
       - GPD Win Max 2 (6800U)
       - GPD Win Max 2 2023 (7840U)
       - GPD Win Max 2 2024 (8840U)
       - GPD Win Max 2 2025 (HX370)
   - win4
       - GPD Win 4 (6800U)
   - win_mini
       - GPD Win Mini (7840U)
       - GPD Win Mini (8840U)
       - GPD Win Mini (HX370)
       - GPD Pocket 4
       - GPD Duo
   - mpc2
       - GPD Micro PC 2

### Sysfs 鎺ュ彛


鏀寔浠ヤ笅灞炴€э細

fan1_input
  鍙銆傝鍙栧綋鍓嶉鎵囪浆閫燂紙RPM锛夈€?
pwm1_enable
  璇?鍐欍€傚惎鐢ㄦ墜鍔ㄩ鎵囨帶鍒躲€傚啓鍏?"0" 绂佺敤鎺у埗骞朵互鍏ㄩ€熻繍琛屻€傚啓鍏?"1" 璁句负鎵嬪姩妯″紡锛屽啓鍏?"2" 鐢?EC 鎺у埗鏉ュ喅瀹氶鎵囪浆閫熴€傝鍙栬灞炴€у彲鏌ョ湅褰撳墠鐘舵€併€?
  NB锛氬嚭浜庤澶囧畨鍏ㄨ€冭檻锛屽綋璁剧疆涓烘墜鍔ㄦā寮忔椂锛宲wm 閫熷害榛樿浼氳璁句负鏈€澶у€硷紙255锛夈€備綘鍙互閫氳繃闅忓悗鍐欏叆 pwm1 鏉ヨ缃笉鍚岀殑鍊笺€?
pwm1
  璇?鍐欍€傝鍙栬灞炴€у彲鏌ョ湅褰撳墠鍗犵┖姣旓紝鑼冨洿涓?[0-255]銆傚綋 pwm1_enable 璁句负 "1"锛堟墜鍔級鏃讹紝鍐欏叆 [0-255] 鑼冨洿鍐呯殑浠绘剰鍊兼潵璁剧疆椋庢墖杞€熴€?
  NB锛氳澶氫富鏉匡紙涓婅堪 wm2 鍒楄〃涔嬪鐨勶級涓嶆敮鎸佸湪鑷姩妯″紡涓嬭鍙栧綋鍓?pwm 鍊硷紝閭ｅ皢鍙繑鍥?EOPNOTSUPP銆傚湪鎵嬪姩妯″紡涓嬪垯濮嬬粓杩斿洖鐪熷疄鍊笺€?