
## 璺熻釜鎬ц兘鐩戞帶涓庤瘖鏂仛鍚堝櫒锛圱PDA锛?

    :Author:   Jinlong Mao <quic_jinlmao@quicinc.com>
    :Date:     January 2023

### 纭欢鎻忚堪


TPDA锛堣窡韪€ц兘鐩戞帶涓庤瘖鏂仛鍚堝櫒锛宼race performance monitoring and diagnostics aggregator锛夌畝鑰岃█涔嬶紝鍏呭綋鎬ц兘鐩戞帶涓庤瘖鏂綉缁滆鑼冪殑浠茶涓庢墦鍖呭紩鎿庛€?TPDA 鐨勪富瑕佺敤閫旀槸瀵?Monitor 鏁版嵁杩涜鎵撳寘銆佹眹鑱氾紙funneling锛変笌鏃堕棿鎴虫爣璁般€?
### Sysfs 鏂囦欢涓庣洰褰?

鏍圭洰褰曪細`/sys/bus/coresight/devices/tpda<N>`

### 閰嶇疆缁嗚妭


tpdm 涓?tpda 鑺傜偣搴斿湪 coresight 璺緞 "/sys/bus/coresight/devices" 涓嬫煡鐪嬨€?渚嬪锛?/sys/bus/coresight/devices # ls -l | grep tpd
tpda0 -> ../../../devices/platform/soc@0/6004000.tpda/tpda0
tpdm0 -> ../../../devices/platform/soc@0/6c08000.mm.tpdm/tpdm0

鎴戜滑鍙互浣跨敤绫讳技浜庝笅闈㈢殑鍛戒护鏉ラ獙璇?TPDM銆?鍏堝惎鐢?coresight sink銆傛墽琛屼互涓嬪懡浠ゅ悗锛岃繛鎺ュ埌 tpdm 鐨?tpda 绔彛灏嗚鍚敤銆?
echo 1 > /sys/bus/coresight/devices/tmc_etf0/enable_sink
echo 1 > /sys/bus/coresight/devices/tpdm0/enable_source
echo 1 > /sys/bus/coresight/devices/tpdm0/integration_test
echo 2 > /sys/bus/coresight/devices/tpdm0/integration_test

娴嬭瘯鏁版嵁灏嗚鏀堕泦鍒板凡鍚敤鐨?coresight sink 涓€?濡傛灉鍦ㄦ墽琛?integration_test 鏃?sink 鐨?rwp 瀵勫瓨鍣ㄦ寔缁洿鏂帮紙閫氳繃 cat tmc_etf0/mgmt/rwp锛夛紝鍒欐剰鍛崇潃鏈変粠 TPDM 鍒?sink 鐨勬暟鎹敓鎴愩€?
鍦?tpdm 涓?sink 涔嬮棿蹇呴』瀛樺湪涓€涓?tpda銆傚綋鍚屼竴 HW 鍧椾腑瀛樺湪鍏朵粬璺熻釜浜嬩欢纭欢缁勪欢涓?tpdm 涓€璧锋椂锛宼pdm 涓庤繖浜涚‖浠剁粍浠跺皢杩炴帴鍒?coresight funnel銆傚綋 HW 鍧椾腑鍙湁 tpdm 璺熻釜纭欢鏃讹紝tpdm 灏嗙洿鎺ヨ繛鎺ュ埌 tpda銆?