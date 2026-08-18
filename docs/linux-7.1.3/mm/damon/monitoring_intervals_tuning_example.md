
## DAMON 鐩戞帶闂撮殧鍙傛暟璋冧紭绀轰緥

DAMON 鐨勭洃鎺у弬鏁伴渶瑕佹牴鎹粰瀹氱殑宸ヤ綔璐熻浇鍜岀洃鎺х洰鐨勮繘琛岃皟浼樸€傚姝ゆ湁涓€浠?:ref:`璋冧紭鎸囧崡 <damon_design_monitoring_params_tuning_guide>`銆傛湰鏂囨。鏍规嵁璇ユ寚鍗楁彁渚涗竴涓皟浼樼ず渚嬨€?
## 鐜鎼缓

鍦ㄤ笅闈㈢殑绀轰緥涓紝浣跨敤浜?Linux 鍐呮牳 v6.11 鐨?DAMON 浠ュ強 `damo <https://github.com/damonitor/damo>`_锛圖AMON 鐢ㄦ埛绌洪棿宸ュ叿锛塿2.5.9锛屾潵鐩戞帶骞跺彲瑙嗗寲杩愯鐪熷疄鏈嶅姟鍣ㄥ伐浣滆礋杞界殑绯荤粺鐗╃悊鍦板潃绌洪棿涓婄殑璁块棶妯″紡銆?
## 5ms/100ms 闂撮殧锛氶棿闅旇繃鐭?
璁╂垜浠粠浣跨敤 DAMON 鎹曡幏绯荤粺鐗╃悊鍦板潃绌洪棿涓婄殑璁块棶妯″紡蹇収寮€濮嬶紝浣跨敤榛樿鐨勯棿闅斿弬鏁帮紙閲囨牱闂撮殧鍜岃仛鍚堥棿闅斿垎鍒负 5 姣鍜?100 姣锛夈€傚湪 DAMON 鍚姩涓庢崟鑾峰揩鐓т箣闂寸瓑寰呭崄鍒嗛挓锛屼互灞曠ず鏈夋剰涔夌殑鏃堕棿缁村害璁块棶妯″紡銆?
```
    # damo start
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop

```
鐒跺悗锛屽垪鍑?DAMON 鍙戠幇鐨勫叿鏈変笉鍚岃闂ā寮忕殑鍖哄煙锛屾寜 鈥滆闂俯搴︹€濓紙access temperature锛夋帓搴忋€傗€滆闂俯搴︹€?鏄竴涓〃绀哄尯鍩熻闂儹搴︼紙hotness锛夌殑鎸囨爣銆傚畠璁＄畻涓鸿闂鐜囧拰鍖哄煙瀛樺湪鏃堕棿锛坅ge锛夌殑鍔犳潈鍜屻€傚鏋滆闂鐜囦负 0%锛屽垯娓╁害涔樹互璐熶竴銆備篃灏辨槸璇达紝濡傛灉涓€涓尯鍩熸湭琚闂紝瀹冧細寰楀埌璐熸俯搴︼紝涓旈殢鐫€鏈璁块棶鐨勬椂闂村彉闀胯€屽彉寰楁洿浣庛€傛帓搴忔寜娓╁害鍗囧簭鎺掑垪锛屽洜姝や綅浜庡垪琛ㄩ《閮ㄧ殑鍖哄煙鏄?```
    # damo report access --sort_regions_by temperature
    0   addr 16.052 GiB   size 5.985 GiB   access 0 %   age 5.900 s    # coldest
    1   addr 22.037 GiB   size 6.029 GiB   access 0 %   age 5.300 s
    2   addr 28.065 GiB   size 6.045 GiB   access 0 %   age 5.200 s
    3   addr 10.069 GiB   size 5.983 GiB   access 0 %   age 4.500 s
    4   addr 4.000 GiB    size 6.069 GiB   access 0 %   age 4.400 s
    5   addr 62.008 GiB   size 3.992 GiB   access 0 %   age 3.700 s
    6   addr 56.795 GiB   size 5.213 GiB   access 0 %   age 3.300 s
    7   addr 39.393 GiB   size 6.096 GiB   access 0 %   age 2.800 s
    8   addr 50.782 GiB   size 6.012 GiB   access 0 %   age 2.800 s
    9   addr 34.111 GiB   size 5.282 GiB   access 0 %   age 2.300 s
    10  addr 45.489 GiB   size 5.293 GiB   access 0 %   age 1.800 s    # hottest
    total size: 62.000 GiB

```
璇ュ垪琛ㄦ樉绀烘病鏈夋槑鏄剧殑鐑尯鍩燂紝骞朵笖鍙湁鏈€灏忕殑璁块棶妯″紡澶氭牱鎬с€傛瘡涓尯鍩熺殑璁块棶棰戠巼閮芥槸闆躲€傚尯鍩熸暟閲忔槸 10锛屼篃灏辨槸榛樿鐨?`min_nr_regions` 鍊笺€傛瘡涓尯鍩熺殑澶у皬涔熷嚑涔庣浉鍚屻€傛垜浠彲浠ユ€€鐤戣繖鏄洜涓?鈥滆嚜閫傚簲鍖哄煙璋冩暣鈥濓紙adaptive regions adjustment锛夋満鍒舵病鑳藉緢濂藉湴宸ヤ綔銆傛濡傛寚鍗楁墍寤鸿鐨勶紝鎴戜滑鍙互浣跨敤 `age` 浣滀负杩戞湡鎬э紙recency锛変俊鎭潵鑾峰彇鍖哄煙鐨勭浉瀵圭儹搴︺€傝繖鎬绘瘮娌℃湁濂斤紝浣嗛壌浜庢渶闀垮瓨鍦ㄦ椂闂村彧鏈夌害 6 绉掞紝鑰屾垜浠瓑寰呬簡绾﹀崄鍒嗛挓锛屽皻涓嶆竻妤氳繖鏈夊澶х敤澶勩€?
鎸夋俯搴﹁寖鍥寸殑鍖哄煙鎬诲ぇ灏忕洿鏂瑰浘
```
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-,590,000,000, -,549,000,000) 5.985 GiB  |**********          |
    [-,549,000,000, -,508,000,000) 12.074 GiB |********************|
    [-,508,000,000, -,467,000,000) 0 B        |                    |
    [-,467,000,000, -,426,000,000) 12.052 GiB |********************|
    [-,426,000,000, -,385,000,000) 0 B        |                    |
    [-,385,000,000, -,344,000,000) 3.992 GiB  |*******             |
    [-,344,000,000, -,303,000,000) 5.213 GiB  |*********           |
    [-,303,000,000, -,262,000,000) 12.109 GiB |********************|
    [-,262,000,000, -,221,000,000) 5.282 GiB  |*********           |
    [-,221,000,000, -,180,000,000) 0 B        |                    |
    [-,180,000,000, -,139,000,000) 5.293 GiB  |*********           |
    total size: 62.000 GiB

```
绠€鑰岃█涔嬶紝杩欎簺鍙傛暟鍦ㄦ娴嬬儹鍖哄煙鏂归潰鎻愪緵鐨勭洃鎺х粨鏋滆川閲忓緢宸€傛牴鎹?:ref:`鎸囧崡 <damon_design_monitoring_params_tuning_guide>`锛岃繖鏄敱浜庤仛鍚堥棿闅旇繃鐭€?
## 100ms/2s 闂撮殧锛氬紑濮嬫樉鐜板嚭灏忕殑鐑尯鍩?
閬靛惊鎸囧崡锛屽皢闂撮殧澧炲ぇ 20 鍊嶏紙100 姣鍜?2 绉掞級

```
    # damo start -s 100ms -a 2s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 10.180 GiB   size 6.117 GiB   access 0 %   age 7 m 8 s    # coldest
    1   addr 49.275 GiB   size 6.195 GiB   access 0 %   age 6 m 14 s
    2   addr 62.421 GiB   size 3.579 GiB   access 0 %   age 6 m 4 s
    3   addr 40.154 GiB   size 6.127 GiB   access 0 %   age 5 m 40 s
    4   addr 16.296 GiB   size 6.182 GiB   access 0 %   age 5 m 32 s
    5   addr 34.254 GiB   size 5.899 GiB   access 0 %   age 5 m 24 s
    6   addr 46.281 GiB   size 2.995 GiB   access 0 %   age 5 m 20 s
    7   addr 28.420 GiB   size 5.835 GiB   access 0 %   age 5 m 6 s
    8   addr 4.000 GiB    size 6.180 GiB   access 0 %   age 4 m 16 s
    9   addr 22.478 GiB   size 5.942 GiB   access 0 %   age 3 m 58 s
    10  addr 55.470 GiB   size 915.645 MiB access 0 %   age 3 m 6 s
    11  addr 56.364 GiB   size 6.056 GiB   access 0 %   age 2 m 8 s
    12  addr 56.364 GiB   size 4.000 KiB   access 95 %  age 16 s
    13  addr 49.275 GiB   size 4.000 KiB   access 100 % age 8 m 24 s   # hottest
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-42,800,000,000, -33,479,999,000) 22.018 GiB |*****************   |
    [-33,479,999,000, -24,159,998,000) 27.090 GiB |********************|
    [-24,159,998,000, -14,839,997,000) 6.836 GiB  |******              |
    [-14,839,997,000, -5,519,996,000)  6.056 GiB  |*****               |
    [-5,519,996,000, 3,800,005,000)    4.000 KiB  |*                   |
    [3,800,005,000, 13,120,006,000)    0 B        |                    |
    [13,120,006,000, 22,440,007,000)   0 B        |                    |
    [22,440,007,000, 31,760,008,000)   0 B        |                    |
    [31,760,008,000, 41,080,009,000)   0 B        |                    |
    [41,080,009,000, 50,400,010,000)   0 B        |                    |
    [50,400,010,000, 59,720,011,000)   4.000 KiB  |*                   |
    total size: 62.000 GiB

```
DAMON 鍙戠幇浜嗕袱涓埅鐒朵笉鍚岀殑銆佺浉褰撶儹鐨?4 KiB 鍖哄煙銆傝繖浜涘尯鍩熺殑瀛樺湪鏃堕棿涔熷緢濂姐€傛渶鐑殑 4 KiB 鍖哄煙淇濇寔璁块棶棰戠巼绾?8 鍒嗛挓锛屾渶鍐风殑鍖哄煙淇濇寔鏃犺闂害 7 鍒嗛挓銆傜洿鏂瑰浘涓婄殑鍒嗗竷鐪嬭捣鏉ヤ篃鍍忔槸鏈夋煇绉嶆ā寮忋€?
鐗瑰埆鏄紝鍦ㄦ€诲叡 62 GiB 鍐呭瓨涓彂鐜?4 KiB 鍖哄煙锛岃〃鏄?DAMON 鐨勮嚜閫傚簲鍖哄煙璋冩暣姝ｆ寜璁捐宸ヤ綔銆?
涓嶈繃锛屽尯鍩熸暟閲忎粛鐒舵帴杩?`min_nr_regions`锛屼笖鍐峰尯鍩熺殑澶у皬涔熺浉浼笺€傛樉鐒跺畠鏈夋墍鏀瑰杽锛屼絾浠嶆湁鏀硅繘绌洪棿銆?
## 400ms/8s 闂撮殧锛氱浉褰撲笉閿欑殑缁撴灉

灏嗛棿闅斿啀澧炲ぇ鍥涘€嶏紙400 姣鍜?8 绉掞級

```
    # damo start -s 400ms -a 8s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 64.492 GiB   size 1.508 GiB   access 0 %   age 6 m 48 s    # coldest
    1   addr 21.749 GiB   size 5.674 GiB   access 0 %   age 6 m 8 s
    2   addr 27.422 GiB   size 5.801 GiB   access 0 %   age 6 m
    3   addr 49.431 GiB   size 8.675 GiB   access 0 %   age 5 m 28 s
    4   addr 33.223 GiB   size 5.645 GiB   access 0 %   age 5 m 12 s
    5   addr 58.321 GiB   size 6.170 GiB   access 0 %   age 5 m 4 s
    [...]
    25  addr 6.615 GiB    size 297.531 MiB access 15 %  age 0 ns
    26  addr 9.513 GiB    size 12.000 KiB  access 20 %  age 0 ns
    27  addr 9.511 GiB    size 108.000 KiB access 25 %  age 0 ns
    28  addr 9.513 GiB    size 20.000 KiB  access 25 %  age 0 ns
    29  addr 9.511 GiB    size 12.000 KiB  access 30 %  age 0 ns
    30  addr 9.520 GiB    size 4.000 KiB   access 40 %  age 0 ns
    [...]
    41  addr 9.520 GiB    size 4.000 KiB   access 80 %  age 56 s
    42  addr 9.511 GiB    size 12.000 KiB  access 100 % age 6 m 16 s
    43  addr 58.321 GiB   size 4.000 KiB   access 100 % age 6 m 24 s
    44  addr 9.512 GiB    size 4.000 KiB   access 100 % age 6 m 48 s
    45  addr 58.106 GiB   size 4.000 KiB   access 100 % age 6 m 48 s    # hottest
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-40,800,000,000, -32,639,999,000) 21.657 GiB  |********************|
    [-32,639,999,000, -24,479,998,000) 17.938 GiB  |*****************   |
    [-24,479,998,000, -16,319,997,000) 16.885 GiB  |****************    |
    [-16,319,997,000, -8,159,996,000)  586.879 MiB |*                   |
    [-8,159,996,000, 5,000)            4.946 GiB   |*****               |
    [5,000, 8,160,006,000)             260.000 KiB |*                   |
    [8,160,006,000, 16,320,007,000)    0 B         |                    |
    [16,320,007,000, 24,480,008,000)   0 B         |                    |
    [24,480,008,000, 32,640,009,000)   0 B         |                    |
    [32,640,009,000, 40,800,010,000)   16.000 KiB  |*                   |
    [40,800,010,000, 48,960,011,000)   8.000 KiB   |*                   |
    total size: 62.000 GiB

```
鍏锋湁涓嶅悓璁块棶妯″紡鐨勫尯鍩熸暟閲忔樉钁楀鍔犮€傛瘡涓尯鍩熺殑澶у皬涔熸洿鍏峰鏍锋€с€傞潪闆惰闂鐜囧尯鍩熺殑鎬诲ぇ灏忎篃鏄捐憲澧炲姞銆備篃璁歌繖宸茬粡瓒冲濂斤紝鍙互甯︽潵涓€浜涙湁鎰忎箟鐨勫瓨鍌ㄥ櫒绠＄悊鏁堢巼鍙樺寲銆?
## 800ms/16s 闂撮殧锛氬彟涓€绉嶅亸鍚?
杩涗竴姝ュ皢闂撮殧缈诲€嶏紙閲囨牱鍜岃仛鍚堥棿闅斿垎鍒负 800 姣鍜?16 绉掞級銆傜粨鏋滃
```
    # damo start -s 800ms -a 16s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 64.781 GiB   size 1.219 GiB   access 0 %   age 4 m 48 s
    1   addr 24.505 GiB   size 2.475 GiB   access 0 %   age 4 m 16 s
    2   addr 26.980 GiB   size 504.273 MiB access 0 %   age 4 m
    3   addr 29.443 GiB   size 2.462 GiB   access 0 %   age 4 m
    4   addr 37.264 GiB   size 5.645 GiB   access 0 %   age 4 m
    5   addr 31.905 GiB   size 5.359 GiB   access 0 %   age 3 m 44 s
    [...]
    20  addr 8.711 GiB    size 40.000 KiB  access 5 %   age 2 m 40 s
    21  addr 27.473 GiB   size 1.970 GiB   access 5 %   age 4 m
    22  addr 48.185 GiB   size 4.625 GiB   access 5 %   age 4 m
    23  addr 47.304 GiB   size 902.117 MiB access 10 %  age 4 m
    24  addr 8.711 GiB    size 4.000 KiB   access 100 % age 4 m
    25  addr 20.793 GiB   size 3.713 GiB   access 5 %   age 4 m 16 s
    26  addr 8.773 GiB    size 4.000 KiB   access 100 % age 4 m 16 s
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-28,800,000,000, -23,359,999,000) 12.294 GiB  |*****************   |
    [-23,359,999,000, -17,919,998,000) 9.753 GiB   |*************       |
    [-17,919,998,000, -12,479,997,000) 15.131 GiB  |********************|
    [-12,479,997,000, -7,039,996,000)  0 B         |                    |
    [-7,039,996,000, -1,599,995,000)   7.506 GiB   |**********          |
    [-1,599,995,000, 3,840,006,000)    6.127 GiB   |*********           |
    [3,840,006,000, 9,280,007,000)     0 B         |                    |
    [9,280,007,000, 14,720,008,000)    136.000 KiB |*                   |
    [14,720,008,000, 20,160,009,000)   40.000 KiB  |*                   |
    [20,160,009,000, 25,600,010,000)   11.188 GiB  |***************     |
    [25,600,010,000, 31,040,011,000)   4.000 KiB   |*                   |
    total size: 62.000 GiB

```
瀹冨彂鐜颁簡鏇村闈為浂璁块棶棰戠巼鐨勫尯鍩熴€傚尯鍩熸暟閲忎粛杩滈珮浜?`min_nr_regions`锛屼絾姣斾箣鍓嶉厤缃殑鏈夋墍鍑忓皯銆傝€屼笖鏄剧劧鍒嗗竷鐪嬭捣鏉ユ湁鐐瑰亸鍚戠儹鍖哄煙銆?
## 缁撹

鏍规嵁涓婅堪瀹為獙鎬ц皟浼樼粨鏋滐紝鎴戜滑鍙互寰楀嚭缁撹锛氳鐞嗚鍜屾寚鍗楄嚦灏戝杩欑宸ヤ綔璐熻浇鏄悎鐞嗙殑锛屽苟涓斿彲浠ュ簲鐢ㄤ簬绫讳技鎯呭喌銆?