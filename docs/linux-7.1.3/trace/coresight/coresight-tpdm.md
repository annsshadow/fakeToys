## 璺熻釜鎬ц兘鐩戞帶涓庤瘖鏂洃瑙嗗櫒锛圱PDM锛?

    :Author:   Jinlong Mao <quic_jinlmao@quicinc.com>
    :Date:     January 2023

### 纭欢鎻忚堪

TPDM锛圱race Performance Monitoring and Diagnostics Monitor锛岀畝绉?TPDM锛変綔涓哄悇绉嶆暟鎹泦绫诲瀷鐨勬暟鎹噰闆嗙粍浠躲€俆PDM 鐨勪富瑕佺敤渚嬫槸浠庝笉鍚屾暟鎹簮鏀堕泦鏁版嵁锛屽苟灏嗗叾鍙戦€佺粰 TPDA 杩涜鎵撳寘銆佸姞鏃堕棿鎴充笌姹囪仛銆?
### Sysfs 鏂囦欢涓庣洰褰?
Root: `/sys/bus/coresight/devices/tpdm<N>`

----

:File:            `enable_source`锛圧W锛?:Notes:
    - > 0 : 浣胯兘 TPDM 鐨勬暟鎹泦銆?
    - = 0 : 绂佺敤 TPDM 鐨勬暟鎹泦銆?
:Syntax:
    `echo 1 > enable_source`

----

:File:            `integration_test`锛坵o锛?:Notes:
    闆嗘垚娴嬭瘯灏嗕负 tpdm 鐢熸垚娴嬭瘯鏁版嵁銆?
:Syntax:
    `echo value > integration_test`

    value -  1 鎴?2銆?
----
