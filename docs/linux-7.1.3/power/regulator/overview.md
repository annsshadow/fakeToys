## Linux 鐢靛帇涓庣數娴佽皟鑺傚櫒妗嗘灦


## 姒傝堪


璇ユ鏋舵棬鍦ㄦ彁渚涗竴涓爣鍑嗙殑鍐呮牳鎺ュ彛鏉ユ帶鍒剁數鍘嬪拰鐢垫祦璋冭妭鍣ㄣ€?
鍏舵剰鍥炬槸璁╃郴缁熻兘澶熷姩鎬佹帶鍒惰皟鑺傚櫒鐨勫姛鐜囪緭鍑烘潵鑺傜渷鍔熻€椼€佸欢闀跨數姹犲鍛姐€傝繖鏃㈤€傜敤浜?鐢靛帇璋冭妭鍣紙鐢靛帇杈撳嚭鍙帶锛夛紝涔熼€傜敤浜庣數娴侀槺锛堢數娴侀檺鍒跺彲鎺э級銆?
(C) 2008  Wolfson Microelectronics PLC.

Author: Liam Girdwood <lrg@slimlogic.co.uk>


## 鏈


鏈枃妗ｄ娇鐢ㄧ殑涓€浜涙湳璇細

  - Regulator锛堣皟鑺傚櫒锛?                 - 涓哄叾浠栬澶囦緵鐢电殑鐢靛瓙璁惧銆傚ぇ澶氭暟璋冭妭鍣ㄥ彲浠ュ惎鐢ㄥ拰绂佺敤鍏?                   杈撳嚭锛屾湁浜涘彲浠ユ帶鍒跺叾杈撳嚭鐢靛帇鍜?鎴栫數娴併€?
                   杈撳叆鐢靛帇 -> 璋冭妭鍣?-> 杈撳嚭鐢靛帇


  - PMIC
                 - 鐢垫簮绠＄悊 IC锛圥ower Management IC锛夈€備竴绉嶅寘鍚紬澶氳皟鑺傚櫒銆?                   閫氬父杩樺寘鍚叾浠栧瓙绯荤粺鐨?IC銆?

  - Consumer锛堟秷璐硅€咃級
                 - 鐢辫皟鑺傚櫒渚涚數鐨勭數瀛愯澶囥€傛秷璐硅€呭彲鍒嗕负涓ょ被锛?

                   闈欐€侊紙Static锛夛細娑堣垂鑰呬笉鏀瑰彉鍏朵緵鐢电數鍘嬫垨鐢垫祦闄愬埗銆傚畠鍙渶
                   鍚敤鎴栫鐢ㄥ叾鐢垫簮銆傚叾渚涚數鐢靛帇鐢辩‖浠躲€乥ootloader銆佸浐浠舵垨鍐呮牳
                   鏉跨骇鍒濆鍖栦唬鐮佽缃€?
                   鍔ㄦ€侊紙Dynamic锛夛細娑堣垂鑰呴渶瑕佹敼鍙樺叾渚涚數鐢靛帇鎴栫數娴侀檺鍒朵互婊¤冻
                   杩愯闇€姹傘€?

  - Power Domain锛堢數婧愬煙锛?                 - 鍏惰緭鍏ュ姛鐜囩敱璋冭妭鍣ㄣ€佸紑鍏虫垨鍙︿竴涓數婧愬煙鐨勮緭鍑哄姛鐜囨彁渚涚殑
                   鐢靛瓙鐢佃矾銆?
```
                     Regulator -+-> Switch-1 -+-> Switch-2 --> [Consumer A]
                                |             |
                                |             +-> [Consumer B], [Consumer C]
                                |
                                +-> [Consumer D], [Consumer E]

                   That is one regulator and three power domains:

                   - Domain 1: Switch-1, Consumers D & E.
                   - Domain 2: Switch-2, Consumers B & C.
                   - Domain 3: Consumer A.

                   and this represents a "supplies" relationship:

                   Domain-1 --> Domain-2 --> Domain-3.

                   A power domain may have regulators that are supplied power
                   by other regulators. i.e.::

                     Regulator-1 -+-> Regulator-2 -+-> [Consumer A]
                                  |
                                  +-> [Consumer B]

                   This gives us two regulators and two power domains:

                   - Domain 1: Regulator-2, Consumer B.
                   - Domain 2: Consumer A.

                   and a "supplies" relationship:

                   Domain-1 --> Domain-2


  - Constraints锛堢害鏉燂級
                 - 绾︽潫鐢ㄤ簬瀹氫箟鎬ц兘涓庣‖浠朵繚鎶ょ殑鍔熺巼绾у埆銆傜害鏉熷瓨鍦ㄤ簬涓変釜灞傜骇锛?
                   Regulator Level锛堣皟鑺傚櫒灞傜骇锛夛細鐢辫皟鑺傚櫒纭欢宸ヤ綔鍙傛暟瀹氫箟锛?                   骞跺湪璋冭妭鍣ㄦ暟鎹墜鍐屼腑鎸囧畾銆傚嵆

                     - 鐢靛帇杈撳嚭鑼冨洿涓?800mV -> 3500mV銆?                     - 璋冭妭鍣ㄧ數娴佽緭鍑洪檺鍒跺湪 5V 鏃朵负 20mA锛屼絾鍦?10V 鏃朵负 10mA銆?
                   Power Domain Level锛堢數婧愬煙灞傜骇锛夛細鐢卞唴鏍哥骇鏉跨骇鍒濆鍖栦唬鐮佸湪
                   杞欢涓畾涔夈€傚畠鐢ㄤ簬灏嗕竴涓數婧愬煙绾︽潫鍒扮壒瀹氱殑鍔熺巼鑼冨洿銆傚嵆

                     - Domain-1 鐢靛帇涓?3300mV
                     - Domain-2 鐢靛帇涓?1400mV -> 1600mV
                     - Domain-3 鐢垫祦闄愬埗涓?0mA -> 20mA銆?
                   Consumer Level锛堟秷璐硅€呭眰绾э級锛氱敱娑堣垂鑰呴┍鍔ㄥ姩鎬佽缃數鍘嬫垨
                   鐢垫祦闄愬埗绾у埆銆?
                   渚嬪锛屼竴涓秷璐硅€呰儗鍏夐┍鍔ㄨ姹傚皢鐢垫祦浠?5mA 澧炲姞鍒?10mA 浠ユ彁鍗?                   LCD 浜害銆傝繖浼氭寜濡備笅鏂瑰紡绌胯繃鍚勫眰绾э細-

                   娑堣垂鑰咃紙Consumer锛夛細闇€瑕佹彁鍗?LCD 浜害銆傛煡鎵惧苟璇锋眰浜害琛?                   涓殑涓嬩竴涓數娴?mA 鍊硷紙鍚屼竴鍙傝€冭澶囧彲浠ユ湁涓嶅悓鐨?personality锛?                   娑堣垂鑰呴┍鍔ㄥ彲鎹澶嶇敤锛夈€?
                   鐢垫簮鍩燂紙Power Domain锛夛細鏂扮殑鐢垫祦闄愬埗鏄惁鍦ㄨ鍩熷強绯荤粺鐘舵€?                   锛堜緥濡傜數姹犱緵鐢点€乁SB 渚涚數锛夌殑杩愯闄愬埗鍐呫€?
                   璋冭妭鍣ㄥ煙锛圧egulator Domains锛夛細鏂扮殑鐢垫祦闄愬埗鏄惁鍦ㄨ緭鍏?杈撳嚭
                   鐢靛帇鐨勮皟鑺傚櫒宸ヤ綔鍙傛暟鍐呫€?
                   濡傛灉璇ヨ皟鑺傚櫒璇锋眰閫氳繃浜嗘墍鏈夌害鏉熸祴璇曪紝鍒欏簲鐢ㄦ柊鐨勮皟鑺傚櫒鍊笺€?

```
## 璁捐


璇ユ鏋堕拡瀵瑰熀浜?SoC 鐨勮澶囪璁″拰鎵撻€狅紝浣嗕篃鍙兘涓庨潪 SoC 璁惧鐩稿叧锛屽苟琚媶鍒嗕负浠ヤ笅
鍥涗釜鎺ュ彛锛?

   1. 娑堣垂鑰呴┍鍔ㄦ帴鍙ｏ紙Consumer driver interface锛夈€?
      瀹冧娇鐢ㄧ殑 API 涓庡唴鏍告椂閽熸帴鍙ｇ被浼硷紝娑堣垂鑰呴┍鍔ㄥ彲浠ヨ幏鍙栧拰閲婃斁涓€涓皟鑺傚櫒
      锛堝氨鍍忕幇鍦ㄥ鏃堕挓鎵€鍋氱殑閭ｆ牱锛夛紝骞惰幏鍙?璁剧疆鐢靛帇銆佺數娴侀檺鍒躲€佹ā寮忋€佸惎鐢ㄥ拰
      绂佺敤銆傝繖搴旇兘璁╂秷璐硅€呭畬鍏ㄦ帶鍒跺叾渚涚數鐢靛帇鍜岀數娴侀檺鍒躲€傚鏋滄湭浣跨敤锛屽畠涔熶細琚?      缂栬瘧鎺夛紝浠ヤ究椹卞姩鍙互鍦ㄦ病鏈夊熀浜庤皟鑺傚櫒鐨勭數婧愭帶鍒剁殑绯荤粺涓鐢ㄣ€?
        See Documentation/power/regulator/consumer.rst

   2. 璋冭妭鍣ㄩ┍鍔ㄦ帴鍙ｏ紙Regulator driver interface锛夈€?
      杩欏厑璁歌皟鑺傚櫒椹卞姩娉ㄥ唽鍏惰皟鑺傚櫒骞跺悜鏍稿績鎻愪緵鎿嶄綔銆傚畠杩樻湁涓€涓€氱煡璋冪敤閾撅紝
      鐢ㄤ簬灏嗚皟鑺傚櫒浜嬩欢浼犳挱缁欏鎴风銆?
        See Documentation/power/regulator/regulator.rst

   3. 鏈哄櫒鎺ュ彛锛圡achine interface锛夈€?
      璇ユ帴鍙ｇ敤浜庢満鍣ㄧ壒瀹氱殑浠ｇ爜锛屽厑璁镐负姣忎釜璋冭妭鍣ㄥ垱寤虹數鍘?鐢垫祦鍩燂紙甯︾害鏉燂級銆?      瀹冨彲浠ユ彁渚涜皟鑺傚櫒绾︽潫锛岄槻姝㈡湁缂洪櫡鐨勫鎴风椹卞姩閫氳繃杩囧帇鎴栬繃娴佹崯鍧忚澶囥€?      瀹冭繕鍏佽鍒涘缓璋冭妭鍣ㄦ爲锛屽叾涓煇浜涜皟鑺傚櫒鐢卞叾浠栬皟鑺傚櫒渚涚數锛堢被浼间簬鏃堕挓鏍戯級銆?
        See Documentation/power/regulator/machine.rst

   4. 鐢ㄦ埛绌洪棿 ABI锛圲serspace ABI锛夈€?
      璇ユ鏋惰繕閫氳繃 sysfs 鍚戠敤鎴风┖闂村鍑哄ぇ閲忔湁鐢ㄧ殑鐢靛帇/鐢垫祦/鎿嶄綔妯″紡鏁版嵁銆?      杩欏彲鐢ㄤ簬甯姪鐩戞帶璁惧鍔熻€楀拰鐘舵€併€?
        See Documentation/ABI/testing/sysfs-class-regulator
