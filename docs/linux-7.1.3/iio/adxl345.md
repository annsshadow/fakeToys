
## ADXL345 椹卞姩

鏈┍鍔ㄦ敮鎸?Analog Device 鐨?ADXL345/375锛屽伐浣滀簬 SPI/I2C 鎬荤嚎銆?

## 1. 鏀寔鐨勮澶?

- `ADXL345 <https://www.analog.com/ADXL345>`_
- `ADXL375 <https://www.analog.com/ADXL375>`_

ADXL345 鏄竴娆鹃€氱敤銆佷綆鍔熻€楃殑 3 杞村姞閫熷害璁★紝鏀寔鍙€夌殑娴嬮噺閲忕▼銆侫DXL345 鏀寔浠ヤ笅閲忕▼锛?

- 卤2g  (绾?卤19.61 m/s^2)
- 卤4g  (绾?卤39.23 m/s^2)
- 卤8g  (绾?卤78.45 m/s^2)
- 卤16g (绾?卤156.91 m/s^2)

## 2. 璁惧灞炴€?

姣忎釜 IIO 璁惧鍦?`/sys/bus/iio/devices/iio:deviceX` 涓嬮兘鏈変竴涓澶囨枃浠跺す锛屽叾涓?X 鏄璁惧鐨?IIO 绱㈠紩銆傝繖浜涙枃浠跺す涓嬪寘鍚竴缁勮澶囨枃浠讹紝鍏蜂綋鍙栧喅浜庣浉鍏崇‖浠惰澶囩殑鐗规€т笌鍔熻兘銆傝繖浜涙枃浠舵槸缁熶竴娉涘寲鐨勶紝骞跺湪 IIO ABI 鏂囨。涓湁璇存槑銆?

涓嬭〃灞曠ず浜嗕綅浜庣壒瀹氳澶囨枃浠跺す璺緞 `/sys/bus/iio/devices/iio:deviceX` 涓嬬殑 ADXL345 鐩稿叧璁惧鏂囦欢銆?

+-------------------------------------------+----------------------------------------------------------+
| 3 杞村姞閫熷害璁＄浉鍏宠澶囨枃浠?                  | 璇存槑                                                     |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_sampling_frequency               | 褰撳墠閫夊畾鐨勯噰鏍风巼銆?                                      |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_sampling_frequency_available     | 鍙敤鐨勯噰鏍烽鐜囬厤缃€?                                    |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_scale                            | 鍔犻€熷害璁″悇閫氶亾鐨勯噺绋?鑼冨洿銆?                             |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_scale_available                  | 鍔犻€熷害璁￠€氶亾鍙敤鐨勯噺绋嬭寖鍥淬€?                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_x_calibbias                      | X 杞村姞閫熷害璁￠€氶亾鐨勬牎鍑嗗亸缃€?                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_x_raw                            | X 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_y_calibbias                      | Y 杞村姞閫熷害鍋忕Щ鏍℃銆?                                    |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_y_raw                            | Y 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_z_calibbias                      | Z 杞村姞閫熷害璁￠€氶亾鐨勬牎鍑嗗亸缃€?                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_z_raw                            | Z 杞村姞閫熷害璁￠€氶亾鐨勫師濮嬪€笺€?                              |
+-------------------------------------------+----------------------------------------------------------+

### 閫氶亾澶勭悊鍚庣殑鍊?

閫氶亾鐨勫€煎彲浠庡叾 _raw 灞炴€ц鍙栥€傝繑鍥炵殑鍊兼槸璁惧鎵€鎶ュ憡鐨勫師鍊笺€傝寰楀埌璇ラ€氶亾鐨勫鐞嗗悗鍊硷紝璇峰簲鐢ㄤ互涓嬪叕寮忥細


        processed value = (_raw + _offset) * _scale

鍏朵腑 _offset 涓?_scale 鏄澶囧睘鎬с€傚鏋滀笉瀛樺湪 _offset 灞炴€э紝鍒欑洿鎺ュ亣瀹氬叾鍊间负 0銆?

+-------------------------------------+---------------------------+
| 閫氶亾绫诲瀷                            | 娴嬮噺鍗曚綅                  |
+-------------------------------------+---------------------------+
| X銆乊銆乑 涓夎酱涓婄殑鍔犻€熷害              | 绫虫瘡浜屾鏂圭              |
+-------------------------------------+---------------------------+

### 浼犳劅鍣ㄤ簨浠?

鐗瑰畾鐨?IIO 浜嬩欢鐢卞叾瀵瑰簲鐨勪腑鏂Е鍙戙€備紶鎰熷櫒椹卞姩鏀寔銆屾棤銆嶆垨銆屽崟涓€嶆湁鏁堜腑鏂紙INT锛夌嚎锛屽彲浠?INT1 鎴?INT2 涓や釜鍙敤閫夐」涓€夋嫨銆傛湁鏁堢殑 INT 绾垮簲鍦ㄨ澶囨爲涓寚瀹氥€傚鏋滄湭閰嶇疆 INT 绾匡紝浼犳劅鍣ㄩ粯璁よ繘鍏?FIFO 鏃佽矾妯″紡锛屾鏃朵簨浠舵娴嬭绂佺敤锛屼粎鑳借幏鍙栧崟鐙殑 X銆乊銆乑 杞存祴閲忓€笺€?

涓嬭〃鍒楀嚭浜嗕綅浜庤澶囩壒瀹氳矾寰?`/sys/bus/iio/devices/iio:deviceX/events` 涓嬬殑 ADXL345 鐩稿叧璁惧鏂囦欢銆傛敞鎰忥紝娲诲姩锛坅ctivity锛変笌闈欐锛坕nactivity锛夋娴嬮粯璁ゆ槸鐩存祦锛圖C锛夎€﹀悎鐨勶紱鍥犳锛屾澶勪粎鏄惧紡鍒楀嚭浜ゆ祦锛圓C锛夎€﹀悎鐨勬椿鍔ㄤ笌闈欐浜嬩欢銆?

+---------------------------------------------+---------------------------------------------+
| 浜嬩欢鍙ユ焺                                    | 璇存槑                                        |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_en               | 鍦ㄦ墍鏈夎酱涓婂惎鐢ㄥ弻鍑绘娴?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_reset_timeout    | 鍙屽嚮绐楀彛锛屽崟浣?[us]                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_scale            | 鍙屽嚮鎵嬪娍闃堝€兼瘮渚嬨€?                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_tap2_min_delay   | 鍙屽嚮寤惰繜锛屽崟浣?[us]                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_value            | 鍙屽嚮闃堝€?                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_scale            | 鍗曞嚮鎵嬪娍闃堝€兼瘮渚嬨€?                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_timeout          | 鍗曞嚮鎸佺画鏃堕棿锛屽崟浣?[us]                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_value            | 鍗曞嚮闃堝€?                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_period        | 浜ゆ祦鑰﹀悎闈欐鏃堕棿锛屽崟浣嶇                    |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_scale         | 浜ゆ祦鑰﹀悎闈欐闃堝€兼瘮渚嬨€?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_value         | 浜ゆ祦鑰﹀悎闈欐闃堝€?                           |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_en             | 鍦?X 杞翠笂鍚敤浜ゆ祦鑰﹀悎娲诲姩妫€娴?              |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_scale          | 浜ゆ祦鑰﹀悎娲诲姩闃堝€兼瘮渚嬨€?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_value          | 浜ゆ祦鑰﹀悎娲诲姩闃堝€?                           |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_period                 | 闈欐鏃堕棿锛屽崟浣嶇                            |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_scale                  | 鐩存祦鑰﹀悎闈欐闃堝€兼瘮渚嬨€?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_value                  | 闈欐闃堝€?                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_en                      | 鍦?X 杞翠笂鍚敤娲诲姩妫€娴?                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_scale                   | 鐩存祦鑰﹀悎娲诲姩闃堝€兼瘮渚嬨€?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_value                   | 娲诲姩闃堝€?                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_x&y&z_mag_adaptive_falling_en      | 鍦ㄦ墍鏈夎酱涓婂惎鐢ㄤ氦娴佽€﹀悎闈欐妫€娴?             |
+---------------------------------------------+---------------------------------------------+
| in_accel_x&y&z_mag_falling_en               | 鍦ㄦ墍鏈夎酱涓婂惎鐢ㄩ潤姝㈡娴?                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_x_gesture_singletap_en             | 鍦?X 杞翠笂鍚敤鍗曞嚮妫€娴?                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_y_gesture_singletap_en             | 鍦?Y 杞翠笂鍚敤鍗曞嚮妫€娴?                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_z_gesture_singletap_en             | 鍦?Z 杞翠笂鍚敤鍗曞嚮妫€娴?                      |
+---------------------------------------------+---------------------------------------------+

鏈夊叧璇ュ姛鑳界殑鍏蜂綋璇存槑锛岃鍙傞槄浼犳劅鍣ㄧ殑鏁版嵁鎵嬪唽锛坉atasheet锛夈€?

鎵嬪姩璁剧疆 **ODR** 浼氫娇椹卞姩涓洪潤姝㈡娴嬫椂搴忎及绠楅粯璁ゅ€硷紝鍏朵腑杈冮珮鐨?ODR 鍊煎搴旇緝闀跨殑榛樿绛夊緟鏃堕棿锛岃緝浣庣殑 ODR 鍊煎搴旇緝鐭殑绛夊緟鏃堕棿銆傚鏋滆繖浜涢粯璁ゅ€间笉鑳芥弧瓒充綘鐨勫簲鐢ㄩ渶姹傦紝浣犲彲浠ユ樉寮忛厤缃潤姝㈢瓑寰呮椂闂淬€傚皢璇ュ€艰涓?0 浼氭仮澶嶉粯璁よ涓恒€?

鏇存敼 **g 閲忕▼** 閰嶇疆鏃讹紝椹卞姩浼氫緷鎹棫閲忕▼涓庢柊閲忕▼涔嬫瘮瀵归粯璁ゅ€艰繘琛岀缉鏀撅紝浠庤€屼及绠楀嚭鍚堥€傜殑娲诲姩涓庨潤姝㈤槇鍊笺€傛墍寰楅槇鍊兼案杩滀笉浼氫负闆讹紝涓斿缁堣惤鍦?1 鍒?255 涔嬮棿锛屽搴旀暟鎹墜鍐屼腑瑙勫畾鐨勪笂闄?62.5鈥痬g/LSB锛?.612915 m/s^2/LSB锛夈€備笉杩囷紝浣犱篃鍙互閫氳繃璁剧疆鏄惧紡鍊兼潵瑕嗙洊杩欎簺浼扮畻闃堝€笺€?

褰?**activity** 涓?**inactivity** 浜嬩欢琚惎鐢ㄦ椂锛岄┍鍔ㄤ細閫氳繃璁剧疆 **link** 涓?**auto-sleep** 浣嶈嚜鍔ㄧ鐞嗚繜婊炶涓恒€俵ink 浣嶅皢娲诲姩涓庨潤姝㈠姛鑳界浉杩烇紝浣夸簩鑰呯浉浜掕窡闅忋€俛uto-sleep 鍔熻兘鍦ㄦ娴嬪埌闈欐鏃朵娇浼犳劅鍣ㄨ繘鍏ョ潯鐪犳ā寮忥紝灏嗗姛鑰楅檷鑷?12.5鈥疕z 浠ヤ笅鐨勯€熺巼銆?

闈欐鏃堕棿鍙湪 1 鍒?255 绉掍箣闂撮厤缃€傞櫎闈欐妫€娴嬪锛屼紶鎰熷櫒杩樻敮鎸佽嚜鐢辫惤浣擄紙free-fall锛夋娴嬶紱浠?IIO 鐨勮搴︾湅锛岃嚜鐢辫惤浣撹瑙嗕负鎵€鏈夎酱涓婂箙鍊肩殑涓嬮檷銆傚氨浼犳劅鍣ㄨ€岃█锛岃嚜鐢辫惤浣撶敱涓€涓?0.000 鍒?1.000 绉掔殑闈欐鍛ㄦ湡鏉ュ畾涔夈€?

椹卞姩鐨勮涓哄涓嬶細

- 鑻ラ厤缃殑闈欐鍛ㄦ湡涓?1 绉掓垨浠ヤ笂锛岄┍鍔ㄤ娇鐢ㄤ紶鎰熷櫒鐨勯潤姝㈠瘎瀛樺櫒銆傝繖浣垮緱璇ヤ簨浠惰兘澶熶笌娲诲姩妫€娴嬪叧鑱斻€佷娇鐢?auto-sleep锛屽苟鍙噰鐢ㄤ氦娴侊紙AC锛夋垨鐩存祦锛圖C锛夎€﹀悎銆?

- 鑻ラ潤姝㈠懆鏈熷皬浜?1 绉掞紝鍒欒浜嬩欢琚涓烘櫘閫氶潤姝㈡垨鑷敱钀戒綋妫€娴嬨€傛鏃朵笉搴旂敤 auto-sleep 涓庤€﹀悎锛圓C/DC锛夈€?

- 鑻ラ厤缃?0 绉掔殑闈欐鏃堕棿锛岄┍鍔ㄤ細閫夋嫨涓€涓惎鍙戝紡纭畾鐨勯粯璁ゅ懆鏈燂紙澶т簬 1 绉掞級浠ヤ紭鍖栧姛鑰椼€傝繖鍚屾牱浣跨敤闈欐瀵勫瓨鍣ㄣ€?

娉ㄦ剰锛氭牴鎹暟鎹墜鍐岋紝鐢ㄤ簬妫€娴嬫椿鍔ㄣ€侀潤姝紙鎴栧湪浣跨敤鑷敱钀戒綋瀵勫瓨鍣ㄦ椂锛夌殑鏈€浣?ODR 搴旇惤鍦?12.5 Hz 鍒?400 Hz 涔嬮棿銆傛帹鑽愮殑鑷敱钀戒綋闃堝€间负 300 mg 鍒?600 mg锛堝瘎瀛樺櫒鍊?0x05 鍒?0x09锛夈€?

鍦ㄧ洿娴侊紙DC锛夎€﹀悎妯″紡涓嬶紝褰撳墠鍔犻€熷害骞呭€肩洿鎺ヤ笌 THRESH_ACT 涓?THRESH_INACT 瀵勫瓨鍣ㄤ腑鐨勫€兼瘮杈冿紝浠ュ垽瀹氭椿鍔ㄦ垨闈欐銆傜浉姣斾箣涓嬶紝浜ゆ祦锛圓C锛夎€﹀悎鐨勬椿鍔ㄦ娴嬩互妫€娴嬪紑濮嬫椂鐨勫姞閫熷害鍊间綔涓哄弬鑰冪偣锛屽悗缁噰鏍蜂笌璇ュ弬鑰冭繘琛屾瘮杈冦€傜洿娴佽€﹀悎鏄粯璁ゆā寮忊€斺€斿皢瀹炴椂鍊间笌鍥哄畾闃堝€兼瘮杈冿紱鑰屼氦娴佽€﹀悎鍒欎緷璧栫浉瀵逛簬鎵€閰嶇疆闃堝€肩殑鍐呴儴婊ゆ尝鍣ㄣ€?

浜ゆ祦涓庣洿娴侊紙DC锛夎€﹀悎妯″紡鍒嗗埆閽堝娲诲姩涓庨潤姝㈡娴嬭繘琛岄厤缃紝浣嗘瘡绉嶆娴嬪悓涓€鏃跺埢鍙兘鏈変竴绉嶆ā寮忕敓鏁堛€備緥濡傦紝鑻ュ厛鍚敤浜ゆ祦鑰﹀悎鐨勬椿鍔ㄦ娴嬶紝鍐嶈缃负鐩存祦鑰﹀悎妯″紡锛屽垯鍙湁鐩存祦鑰﹀悎鐨勬椿鍔ㄦ娴嬩細鐢熸晥銆傛崲瑷€涔嬶紝浠呭簲鐢ㄦ渶杩戜竴娆＄殑閰嶇疆銆?

**Single tap**锛堝崟鍑伙級妫€娴嬪彲鎸夌収鏁版嵁鎵嬪唽锛岄€氳繃璁剧疆闃堝€间笌鎸佺画鏃堕棿鍙傛暟鏉ラ厤缃€傚綋浠呭惎鐢ㄥ崟鍑绘娴嬫椂锛屽彧瑕佸姞閫熷害瓒呰繃闃堝€硷紙鏍囧織鐫€鎸佺画鏃堕棿鐨勫紑濮嬶級闅忓悗鍙堜綆浜庨槇鍊硷紙涓旀湭瓒呰繃鎸佺画鏃堕棿涓婇檺锛夛紝灏变細瑙﹀彂鍗曞嚮涓柇銆傝嫢鍚屾椂鍚敤浜嗗崟鍑讳笌鍙屽嚮妫€娴嬶紝鍒欏崟鍑讳腑鏂粎鍦ㄥ弻鍑讳簨浠惰纭鎴栧彇娑堝悗鎵嶄細瑙﹀彂銆?

瑕侀厤缃?**double tap**锛堝弻鍑伙級妫€娴嬶紝杩樺繀椤昏缃獥鍙ｄ笌寤惰繜鍙傛暟锛屽崟浣嶄负寰锛埪祍锛夈€傚欢杩熸湡浠庡崟鍑讳俊鍙蜂綆浜庨槇鍊兼椂寮€濮嬶紝浣滀负涓€娈电瓑寰呮椂闂达紝鍦ㄦ鏈熼棿鍙屽嚮妫€娴嬩細蹇界暐浠讳綍灏栧嘲銆傚欢杩熸湡缁撴潫鍚庯紝妫€娴嬬獥鍙ｅ紑濮嬨€傝嫢鍔犻€熷害鍦ㄨ绐楀彛鍐呭厛鍗囪繃闃堝€笺€佸啀闄嶅洖闃堝€间互涓嬶紝鍒欏湪闄嶈嚦闃堝€间互涓嬫椂瑙﹀彂鍙屽嚮浜嬩欢銆?

鍙屽嚮浜嬩欢妫€娴嬪湪鏁版嵁鎵嬪唽涓湁璇﹀敖璇存槑銆傚湪妫€娴嬪埌鍗曞嚮浜嬩欢鍚庯紝鑻ヤ俊鍙锋弧瓒崇壒瀹氭潯浠讹紝鍙兘浼氳窡闅忎竴涓弻鍑讳簨浠躲€備笉杩囷紝鍙屽嚮妫€娴嬪彲鑳藉洜浠ヤ笅涓変釜鍘熷洜鑰屽け鏁堬細

- 鑻ヨ缃簡 **suppress bit**锛屽垯鍦ㄧ偣鍑诲欢杩熸湡鍐呬换浣曡秴杩囩偣鍑婚槇鍊肩殑鍔犻€熷害灏栧嘲閮戒細绔嬪嵆浣垮弻鍑绘娴嬪け鏁堛€傛崲瑷€涔嬶紝褰?suppress 浣嶆縺娲绘椂锛屽欢杩熸湡鍐呬笉鍏佽鍑虹幇浠讳綍灏栧嘲銆?

- 鑻ュ弻鍑荤獥鍙ｅ紑濮嬫椂鍔犻€熷害楂樹簬闃堝€硷紝鍒欏弻鍑讳簨浠舵棤鏁堛€?

- 鑻ュ姞閫熷害鎸佺画鏃堕棿瓒呰繃 duration 瀵勫瓨鍣ㄨ瀹氱殑涓婇檺锛屽弻鍑绘娴嬪悓鏍蜂細澶辨晥銆?

瀵逛簬鍙屽嚮妫€娴嬶紝閫傜敤鐨勬寔缁椂闂翠笌鍗曞嚮鐩稿悓锛氬姞閫熷害蹇呴』鍏堝崌杩囬槇鍊笺€佸啀鍦ㄦ寚瀹氭寔缁椂闂村唴闄嶅洖闃堝€间互涓嬨€傛敞鎰忥紝褰撳弻鍑绘娴嬪浜庢椿鍔ㄧ姸鎬佹椂锛岄€氬父浼氬惎鐢?suppress 浣嶃€?

### 浣跨敤绀轰緥

鏄剧ず璁惧鍚嶏細


        root:/sys/bus/iio/devices/iio:device0> cat name
        adxl345

鏄剧ず鍔犻€熷害璁￠€氶亾鍊硷細


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_raw
        -1
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_y_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_z_raw
        -253

璁剧疆鍔犻€熷害璁￠€氶亾鐨勬牎鍑嗗亸缃細


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        0

        root:/sys/bus/iio/devices/iio:device0> echo 50 > in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        50

缁欏畾 13 浣嶅叏鍒嗚鲸鐜囷紝鍙敤閲忕▼鐢变互涓嬪叕寮忚绠楋細


        (g ** 2 ** 9.80665) / (2^(resolution) - 1) * 100; for g := 2|4|8|16

閲忕▼閰嶇疆锛?


        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale
        0.004789
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale_available
        0.004789 0.009578 0.019156 0.038312

        root:/sys/bus/iio/devices/iio:device0> echo 0.019156 > ./in_accel_scale
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale
        0.019156

璁剧疆杈撳嚭鏁版嵁閫熺巼锛圤DR锛夛細


        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency
        200.000000

        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency_available
        0.097000 0.195000 0.390000 0.781000 1.562000 3.125000 6.250000 12.500000 25.000000 50.000000 100.000000 200.000000 400.000000 800.000000 1600.000000 3200.000000

        root:/sys/bus/iio/devices/iio:device0> echo 1.562000 > ./in_accel_sampling_frequency
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency
        1.562000

閰嶇疆涓€涓垨澶氫釜浜嬩欢锛?


        root:> cd /sys/bus/iio/devices/iio:device0

        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_z_en

        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_z_en

        root:/sys/bus/iio/devices/iio:device0> echo 14   > ./in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> echo 2    > ./in_accel_y_calibbias
        root:/sys/bus/iio/devices/iio:device0> echo -250 > ./in_accel_z_calibbias

        root:/sys/bus/iio/devices/iio:device0> echo 24 > ./buffer0/length

        ## Check the event scale factor (0.0625 * 9.80665)
        root:/sys/bus/iio/devices/iio:device0> cat ./events/in_accel_gesture_doubletap_scale
        0.612915

        ## AC coupled activity, threshold [0.612915 m/s^2/LSB]
        root:/sys/bus/iio/devices/iio:device0> echo 6 > ./events/in_accel_mag_adaptive_rising_value

        ## AC coupled inactivity, threshold, [0.612915 m/s^2/LSB]
        root:/sys/bus/iio/devices/iio:device0> echo 4 > ./events/in_accel_mag_adaptive_falling_value

        ## AC coupled inactivity, time [s]
        root:/sys/bus/iio/devices/iio:device0> echo 3 > ./events/in_accel_mag_adaptive_falling_period

        ## singletap, threshold
        root:/sys/bus/iio/devices/iio:device0> echo 35 > ./events/in_accel_gesture_singletap_value

        ## singletap, duration [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.001875  > ./events/in_accel_gesture_singletap_timeout

        ## doubletap, window [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.025 > ./events/in_accel_gesture_doubletap_reset_timeout

        ## doubletap, latency [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.025 > ./events/in_accel_gesture_doubletap_tap2_min_delay

        ## AC coupled activity, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_mag_adaptive_rising_en

        ## AC coupled inactivity, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\&y\&z_mag_adaptive_falling_en

        ## singletap, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x_gesture_singletap_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_y_gesture_singletap_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_z_gesture_singletap_en

        ## doubletap, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_gesture_doubletap_en

楠岃瘉鎺ユ敹鍒扮殑浜嬩欢锛?


        root:# iio_event_monitor adxl345
        Found IIO device with name adxl345 with device number 0
        Event: time: 1739063415957073383, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063415963770218, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063416002563061, type: accel(z), channel: 0, evtype: gesture, direction: singletap
        Event: time: 1739063426271128739, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1739063436539080713, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1739063438357970381, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446726161586, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446727892670, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446743019768, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446744650696, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446763559386, type: accel(z), channel: 0, evtype: gesture, direction: singletap
        Event: time: 1739063448818126480, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...

娲诲姩涓庨潤姝㈢浉浜掑叧鑱旓紝骞舵寜涓嬭堪鏂瑰紡鎸囩ず鐘舵€佸彉鍖栵細


        root:# iio_event_monitor adxl345
        Found IIO device with name adxl345 with device number 0
        Event: time: 1744648001133946293, type: accel(x), channel: 0, evtype: mag, direction: rising
          <after inactivity time elapsed>
        Event: time: 1744648057724775499, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...

## 3. 璁惧缂撳啿鍖?

鏈┍鍔ㄦ敮鎸?IIO 缂撳啿鍖恒€傛墍鏈夎澶囬兘鏀寔閫氳繃缂撳啿鍖鸿幏鍙栧師濮嬪姞閫熷害涓庢俯搴︽祴閲忓€笺€?

### 浣跨敤绀轰緥

涓虹紦鍐茶鍙栭€夋嫨閫氶亾锛?


        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_z_en

璁剧疆缂撳啿鍖轰腑瀛樺偍鐨勬牱鏈暟閲忥細


        root:/sys/bus/iio/devices/iio:device0> echo 10 > buffer/length

鍚敤缂撳啿璇诲彇锛?


        root:/sys/bus/iio/devices/iio:device0> echo 1 > buffer/enable

鑾峰彇缂撳啿鏁版嵁锛?


        root:> iio_readdev -b 16 -s 1024 adxl345 | hexdump -d
        WARNING: High-speed mode not enabled
        0000000   00003   00012   00013   00005   00010   00011   00005   00011
        0000010   00013   00004   00012   00011   00003   00012   00014   00007
        0000020   00011   00013   00004   00013   00014   00003   00012   00013
        0000030   00004   00012   00013   00005   00011   00011   00005   00012
        0000040   00014   00005   00012   00014   00004   00010   00012   00004
        0000050   00013   00011   00003   00011   00012   00005   00011   00013
        0000060   00003   00012   00012   00003   00012   00012   00004   00012
        0000070   00012   00003   00013   00013   00003   00013   00012   00005
        0000080   00012   00013   00003   00011   00012   00005   00012   00013
        0000090   00003   00013   00011   00005   00013   00014   00003   00012
        00000a0   00012   00003   00012   00013   00004   00012   00015   00004
        00000b0   00014   00011   00003   00014   00013   00004   00012   00011
        00000c0   00004   00012   00013   00004   00014   00011   00004   00013
        00000d0   00012   00002   00014   00012   00005   00012   00013   00005
        00000e0   00013   00013   00003   00013   00013   00005   00012   00013
        00000f0   00004   00014   00015   00005   00012   00011   00005   00012
        ...

鏈夊叧缂撳啿鏁版嵁鐨勭粨鏋勶紝璇峰弬闃?Documentation/iio/iio_devbuf.rst銆?

## 4. IIO 鎺ュ彛宸ュ叿

鏈夊叧鍙敤 IIO 鎺ュ彛宸ュ叿鐨勮鏄庯紝璇峰弬闃?Documentation/iio/iio_tools.rst銆?
