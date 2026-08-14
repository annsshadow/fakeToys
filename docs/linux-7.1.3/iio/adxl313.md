
## ADXL313 椹卞姩

鏈枃浠朵粙缁?Linux IIO 瀛愮郴缁熶腑 ADXL313 涓夎酱鍔犻€熷害璁＄殑椹卞姩瀹炵幇锛岃鏄庡叾 SPI/I2C 杩炴帴銆佹祴閲忚寖鍥翠互鍙婂搴旂殑 sysfs 璁惧鏂囦欢涓庢椿鍔?闈炴椿鍔ㄤ簨浠堕厤缃紝渚涢┍鍔ㄤ娇鐢ㄨ€呭弬鑰冦€?


鏈┍鍔ㄦ敮鎸?Analog Device 鐨?ADXL313锛岄€氳繃 SPI/I2C 鎬荤嚎杩炴帴銆?

## 1. 鏀寔鐨勮澶?


- `ADXL313 <https://www.analog.com/ADXL313>`_

ADXL313 鏄竴娆句綆鍣０瀵嗗害銆佷綆鍔熻€楃殑 3 杞村姞閫熷害璁★紝鍏锋湁鍙€夌殑娴嬮噺鑼冨洿銆侫DXL313 鏀寔 卤0.5 g銆伮? g銆伮? g 涓?卤4 g 鑼冨洿銆?

## 2. 璁惧灞炴€?


鍔犻€熷害璁℃祴閲忓€煎缁堟彁渚涖€?

姣忎釜 IIO 璁惧锛屽湪 `/sys/bus/iio/devices/iio:deviceX` 涓嬮兘鏈変竴涓澶囨枃浠跺す锛屽叾涓?X 鏄璁惧鐨?IIO 绱㈠紩銆傛牴鎹墍璁ㄨ纭欢璁惧鐨勭壒鎬т笌鍔熻兘锛岃繖浜涙枃浠跺す涓嬪瓨鏀剧潃涓€缁勮澶囨枃浠躲€傝繖浜涙枃浠惰涓€鑷村湴娉涘寲锛屽苟璁板綍鍦?IIO ABI 鏂囨。涓€?

涓嬭〃鏄剧ず浜嗕笌 adxl313 鐩稿叧鐨勮澶囨枃浠讹紝瀹冧滑浣嶄簬鐗瑰畾璁惧鏂囦欢澶硅矾寰?`/sys/bus/iio/devices/iio:deviceX` 涓嬨€?

+---------------------------------------------------+----------------------------------------------------------+
| 3 杞村姞閫熷害璁＄浉鍏宠澶囨枃浠?                           | 鎻忚堪                                                     |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_scale                                    | 鍔犻€熷害璁￠€氶亾鐨勬瘮渚嬪洜瀛愶紙scale锛夈€?                       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_calibbias                              | X 杞村姞閫熷害璁￠€氶亾鐨勬牎鍑嗗亸绉汇€?                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_raw                                    | X 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_calibbias                              | Y 杞村姞閫熷害鍋忕Щ鏍℃                                       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_raw                                    | Y 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_calibbias                              | Z 杞村姞閫熷害璁￠€氶亾鐨勬牎鍑嗗亸绉汇€?                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_raw                                    | Z 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+---------------------------------------------------+----------------------------------------------------------+

+---------------------------------------+----------------------------------------------+
| 鏉傞」璁惧鏂囦欢                          | 鎻忚堪                                         |
+---------------------------------------+----------------------------------------------+
| name                                  | IIO 璁惧鐨勫悕绉般€?                            |
+---------------------------------------+----------------------------------------------+
| in_accel_sampling_frequency           | 褰撳墠閫夋嫨鐨勯噰鏍风巼銆?                          |
+---------------------------------------+----------------------------------------------+
| in_accel_sampling_frequency_available | 鍙敤鐨勯噰鏍烽鐜囬厤缃€?                        |
+---------------------------------------+----------------------------------------------+

涓?iio 浜嬩欢鐩稿叧鐨勮缃紝浣嶄簬 `/sys/bus/iio/devices/iio:deviceX/events` 涓嬨€?

+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_falling_period              | AC 鑰﹀悎鐨勯潪娲诲姩鏃堕棿銆?                                  |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_falling_value               | AC 鑰﹀悎鐨勯潪娲诲姩闃堝€笺€?                                  |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_rising_value                | AC 鑰﹀悎鐨勬椿鍔ㄩ槇鍊笺€?                                    |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_falling_period                       | 闈炴椿鍔ㄦ椂闂淬€?                                          |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_falling_value                        | 闈炴椿鍔ㄩ槇鍊笺€?                                          |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_rising_value                         | 娲诲姩闃堝€笺€?                                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\&y\&z_mag_adaptive_falling_en          | 鍚敤鎴栫鐢?AC 鑰﹀悎鐨勯潪娲诲姩浜嬩欢銆?                      |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\|y\|z_mag_adaptive_rising_en           | 鍚敤鎴栫鐢?AC 鑰﹀悎鐨勬椿鍔ㄤ簨浠躲€?                        |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\&y\&z_mag_falling_en                   | 鍚敤鎴栫鐢ㄩ潪娲诲姩浜嬩欢銆?                                |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\|y\|z_mag_rising_en                    | 鍚敤鎴栫鐢ㄦ椿鍔ㄤ簨浠躲€?                                  |
+---------------------------------------------------+----------------------------------------------------------+

榛樿鑰﹀悎涓?DC 鑰﹀悎浜嬩欢銆傚湪杩欑鎯呭喌涓嬮槇鍊煎皢淇濇寔鍘熸牱锛岃€屽浜?AC 鑰﹀悎鐨勬儏鍐碉紝浼犳劅鍣ㄤ細搴旂敤涓€涓嚜閫傚簲闃堝€硷紙鍦?datasheet 涓弿杩帮級銆傞€氬父娲诲姩锛屽嵆 `ACTIVITY` 鎴?`ACTIVITY_AC`锛屼互鍙婇潪娲诲姩锛屽嵆 `INACTIVITY` 鎴?`INACTIVITY_AC`锛屽湪涓よ€呴兘鍚敤鏃跺皢涓庤嚜鍔ㄤ紤鐪犲叧鑱斻€傝繖鎰忓懗鐫€鐗瑰埆鏄?`ACTIVITY` 涔熷彲浠ヤ笌 `INACTIVITY_AC` 鍏宠仈锛屽弽涔嬩害鐒讹紝娌℃湁闂銆?

娉ㄦ剰锛岃繖閲?`ACTIVITY` 涓?`ACTIVITY_AC` 鏄簰鏂ョ殑銆傝繖鎰忓懗鐫€锛屾渶杩戠殑涓€娆￠厤缃皢琚缃€備緥濡傦紝濡傛灉 `ACTIVITY` 宸插惎鐢紝鑰?`ACTIVITY_AC` 灏嗚鍚敤锛岄偅涔堜紶鎰熷櫒椹卞姩灏嗙鐢?`ACTIVITY`锛屼絾鍚敤 `ACTIVITY_AC`銆傚浜庨潪娲诲姩鍚屾牱鎴愮珛銆傚湪鍏抽棴涓€涓簨浠舵椂锛屽畠蹇呴』涓庡疄闄呭惎鐢ㄧ殑鐩稿尮閰嶏紝鍗冲惎鐢?`ACTIVITY_AC` 鐒跺悗绂佺敤 `ACTIVITY` 浼氳绠€鍗曞湴蹇界暐锛屽洜涓哄畠宸茬粡澶勪簬绂佺敤鐘舵€併€傛垨鑰咃紝灏卞儚瀵瑰緟浠讳綍鍏朵粬鏈惎鐢ㄧ殑浜嬩欢涓€鏍枫€?

### 閫氶亾澶勭悊鍚庣殑鍊?


鍙互浠庨€氶亾鐨?_raw 灞炴€ц鍙栦竴涓€氶亾鍊笺€傝繑鍥炵殑鍊兼槸璁惧鎵€鎶ュ憡鐨勫師濮嬪€笺€傝鑾峰緱閫氶亾鐨勫鐞嗗悗鍊硷紝搴旂敤浠ヤ笅鍏紡锛?


        processed value = (_raw + _offset) * _scale

鍏朵腑 _offset 涓?_scale 鏄澶囧睘鎬с€傚鏋滀笉瀛樺湪 _offset 灞炴€э紝鍒欑畝鍗曞湴鍋囧畾鍏跺€间负 0銆?

ADXL313 椹卞姩涓哄崟涓€绫诲瀷鐨勯€氶亾鎻愪緵鏁版嵁锛屼笅琛ㄦ樉绀轰簡澶勭悊鍚庡€肩殑娴嬮噺鍗曚綅锛屽畠浠敱 IIO 妗嗘灦瀹氫箟锛?

+-------------------------------------+---------------------------+
| 閫氶亾绫诲瀷                            | 娴嬮噺鍗曚綅                  |
+-------------------------------------+---------------------------+
| X銆乊 涓?Z 杞翠笂鐨勫姞閫熷害              | 绫虫瘡浜屾鏂圭             |
+-------------------------------------+---------------------------+

### 浣跨敤绀轰緥


鏄剧ず璁惧鍚嶇О锛?


        root:/sys/bus/iio/devices/iio:device0> cat name
        adxl313

鏄剧ず鍔犻€熷害璁￠€氶亾鍊硷細


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_y_raw
        -57
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_z_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_scale
        0.009576806

鍔犻€熷害璁＄殑鍊煎皢鏄細

- X 杞村姞閫熷害 = in_accel_x_raw * in_accel_scale = 0.0191536 m/s^2
- Y 杞村姞閫熷害 = in_accel_y_raw * in_accel_scale = -0.5458779 m/s^2
- Z 杞村姞閫熷害 = in_accel_z_raw * in_accel_scale = 0.0191536 m/s^2

璁剧疆鍔犻€熷害璁￠€氶亾鐨勬牎鍑嗗亸绉汇€傛敞鎰忥紝鏍″噯灏嗘牴鎹?LSB 鍗曚綅鐨勫埢搴﹁繘琛屽洓鑸嶄簲鍏ワ細


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        0

        root:/sys/bus/iio/devices/iio:device0> echo 50 > in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        48

璁剧疆閲囨牱棰戠巼锛?


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency
        100.000000
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency_available
        6.250000 12.500000 25.000000 50.000000 100.000000 200.000000 400.000000 800.000000 1600.000000 3200.000000

        root:/sys/bus/iio/devices/iio:device0> echo 400 > in_accel_sampling_frequency
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency
        400.000000

## 3. 璁惧缂撳啿鍖轰笌瑙﹀彂鍣?


鏈┍鍔ㄦ敮鎸?IIO 缂撳啿鍖恒€?

鎵€鏈夎澶囬兘鏀寔浣跨敤缂撳啿鍖烘绱㈠師濮嬪姞閫熷害娴嬮噺鍊笺€?

### 浣跨敤绀轰緥


涓虹紦鍐插尯璇诲彇閫夋嫨閫氶亾锛?


        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_z_en

璁剧疆鍦ㄧ紦鍐插尯涓瓨鍌ㄧ殑鏍锋湰鏁伴噺锛?


        root:/sys/bus/iio/devices/iio:device0> echo 10 > buffer/length

鍚敤缂撳啿鍖鸿鍙栵細


        root:/sys/bus/iio/devices/iio:device0> echo 1 > buffer/enable

鑾峰彇缂撳啿鏁版嵁锛?


        root:/sys/bus/iio/devices/iio:device0> hexdump -C /dev/iio\:device0
        ...
        000000d0  01 fc 31 00 c7 ff 03 fc  31 00 c7 ff 04 fc 33 00  |..1.....1.....3.|
        000000e0  c8 ff 03 fc 32 00 c5 ff  ff fc 32 00 c7 ff 0a fc  |....2.....2.....|
        000000f0  30 00 c8 ff 06 fc 33 00  c7 ff 01 fc 2f 00 c8 ff  |0.....3...../...|
        00000100  02 fc 32 00 c6 ff 04 fc  33 00 c8 ff 05 fc 33 00  |..2.....3.....3.|
        00000110  ca ff 02 fc 31 00 c7 ff  02 fc 30 00 c9 ff 09 fc  |....1.....0.....|
        00000120  35 00 c9 ff 08 fc 35 00  c8 ff 02 fc 31 00 c5 ff  |5.....5.....1...|
        00000130  03 fc 32 00 c7 ff 04 fc  32 00 c7 ff 02 fc 31 00  |..2.....2.....1.|
        00000140  c7 ff 08 fc 30 00 c7 ff  02 fc 32 00 c5 ff ff fc  |....0.....2.....|
        00000150  31 00 c5 ff 04 fc 31 00  c8 ff 03 fc 32 00 c8 ff  |1.....1.....2...|
        00000160  01 fc 31 00 c7 ff 05 fc  31 00 c3 ff 04 fc 31 00  |..1.....1.....1.|
        00000170  c5 ff 04 fc 30 00 c7 ff  03 fc 31 00 c9 ff 03 fc  |....0.....1.....|
        ...

鍚敤娲诲姩妫€娴嬶細


        root:/sys/bus/iio/devices/iio:device0> echo 1.28125 > ./events/in_accel_mag_rising_value
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\|y\|z_mag_rising_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        <only while moving the sensor>
        Event: time: 1748795762298351281, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        Event: time: 1748795762302653704, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        Event: time: 1748795762304340726, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        ...

绂佺敤娲诲姩妫€娴嬶細


        root:/sys/bus/iio/devices/iio:device0> echo 0 > ./events/in_accel_x\|y\|z_mag_rising_en
        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        <nothing>

鍚敤闈炴椿鍔ㄦ娴嬶細


        root:/sys/bus/iio/devices/iio:device0> echo 1.234375 > ./events/in_accel_mag_falling_value
        root:/sys/bus/iio/devices/iio:device0> echo 5 > ./events/in_accel_mag_falling_period
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\&y\&z_mag_falling_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        Event: time: 1748796324115962975, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1748796329329981772, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1748796334543399706, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...
        <every 5s now indicates inactivity>

鐜板湪锛屽惎鐢ㄦ椿鍔紝渚嬪 AC 鑰﹀悎鐨勫搴旈」 `ACTIVITY_AC`


        root:/sys/bus/iio/devices/iio:device0> echo 1.28125 > ./events/in_accel_mag_rising_value
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\|y\|z_mag_rising_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        <some activity with the sensor>
        Event: time: 1748796880354686777, type: accel(x|y|z), channel: 0, evtype: mag_adaptive, direction: rising
        <5s of inactivity, then>
        Event: time: 1748796885543252017, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        <some other activity detected by accelerating the sensor>
        Event: time: 1748796887756634678, type: accel(x|y|z), channel: 0, evtype: mag_adaptive, direction: rising
        <again, 5s of inactivity>
        Event: time: 1748796892964368352, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        <stays like this until next activity in auto-sleep>

娉ㄦ剰锛屽綋鍚敤 AC 鑰﹀悎鏃讹紝浜嬩欢绫诲瀷灏嗕负 `mag_adaptive`銆侫C 鑰﹀悎鎴?DC 鑰﹀悎锛堥粯璁わ級浜嬩欢鐨勪娇鐢ㄦ柟寮忕被浼笺€?

## 4. IIO 鎺ュ彛宸ュ叿


鏈夊叧鍙敤 IIO 鎺ュ彛宸ュ叿鐨勬弿杩帮紝璇峰弬闃?Documentation/iio/iio_tools.rst銆?
