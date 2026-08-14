## 鏅鸿兘閿洏锛坕kbd锛夊崗璁?
## 绠€浠?
Atari 鍏徃鐨勬櫤鑳介敭鐩橈紙Intelligent Keyboard锛宨kbd锛夋槸涓€涓€氱敤鐨勯敭鐩樻帶鍒跺櫒锛屽叾
鐏垫椿鎬ц冻浠ュ湪澶氱浜у搧涓笉缁忎慨鏀瑰嵆鍙娇鐢ㄣ€傝閿洏杩炲悓鍏跺井鎺у埗鍣紝涓洪紶鏍囧拰寮€鍏冲瀷
锛坰witch-type锛夋父鎴忔潌锛坖oystick锛夋彁渚涗簡涓€涓柟渚跨殑杩炴帴鐐广€俰kbd 澶勭悊鍣ㄨ繕缁存姢涓€涓?鍒嗚鲸鐜囦负涓€绉掔殑鏃ユ湡鏃堕挓锛坱ime-of-day clock锛夈€?ikbd 鐨勮璁″叿鏈夎冻澶熺殑閫氱敤鎬э紝鍙敤浜庡绉嶆柊鐨勮绠楁満浜у搧銆傛寜閿紑鍏虫暟閲忋€侀紶鏍囧垎杈ㄧ巼
绛夋柟闈㈢殑浜у搧宸紓閮藉彲浠ヨ瀹圭撼銆?ikbd 閫氳繃涓€涓珮閫熷弻鍚戜覆琛屾帴鍙ｄ笌涓诲鐞嗗櫒閫氫俊銆傚畠鍙互宸ヤ綔浜庡绉嶆ā寮忥紝浠ユ柟渚块敭鐩樸€?娓告垙鏉嗘垨榧犳爣鐨勪笉鍚屽簲鐢ㄣ€傞€氳繃绮惧績璁捐榛樿妯″紡锛屽嵆浣垮湪鍙湁鍗曞悜閫氫俊浠嬭川鍙敤鐨勫簲鐢?涓紝涔熷彲浠ユ湁闄愬害鍦颁娇鐢ㄨ鎺у埗鍣ㄣ€?
## 閿洏

閿洏濮嬬粓杩斿洖鎸夐敭鎸変笅/閲婃斁锛坢ake/break锛夌殑鎵弿鐮併€俰kbd 涓烘瘡涓寜閿殑鎸変笅鍜岄噴鏀?鐢熸垚閿洏鎵弿鐮併€傛寜閿壂鎻忔寜涓嬶紙鎸夐敭闂悎锛夌爜浠?1 寮€濮嬶紝骞跺湪闄勫綍 A 涓畾涔夈€備緥濡傦紝
鎵弿鐮佽〃涓?ISO 閿殑浣嶇疆搴斿綋瀛樺湪锛屽嵆浣挎煇涓壒瀹氶敭鐩樹笂璇ヤ綅缃病鏈夋寜閿紑鍏炽€傛瘡涓寜閿?鐨勯噴鏀撅紙break锛夌爜閫氳繃瀵规寜涓嬶紙make锛夌爜杩涜 OR 0x80 鎿嶄綔寰楀埌銆?
鐗规畩鐮?0xF6 鍒?0xFF 淇濈暀鐢ㄤ簬浠ヤ笅鐢ㄩ€旓細

=================== ====================================================
    Code            Command
=================== ====================================================
    0xF6            status report
    0xF7            absolute mouse position record
    0xF8-0xFB       relative mouse position records (lsbs determined by
                    mouse button states)
    0xFC            time-of-day
    0xFD            joystick report (both sticks)
    0xFE            joystick 0 event
    0xFF            joystick 1 event
=================== ====================================================

鍦ㄦ妯″紡涓嬶紝涓や釜 Shift 閿繑鍥炰笉鍚岀殑鎵弿鐮併€侲NTER 閿拰 RETurn 閿篃鏄笉鍚岀殑銆?
## 榧犳爣

榧犳爣绔彛搴斿綋鑳藉鏀寔鍒嗚鲸鐜囩害涓烘瘡鑻卞琛岀▼ 200 涓鏁帮紙鐩镐綅鍙樺寲鎴栤€滅偣鍑伙紙click锛夆€濓級鐨?榧犳爣銆傞紶鏍囧簲浠ヨ兘澶熷厑璁稿湪楂樿揪姣忕 10 鑻卞鐨勯€熷害涓嬪噯纭窡韪殑閫熺巼杩涜鎵弿銆?ikbd 鍙互閫氳繃涓夌鏄庢樉涓嶅悓鐨勬柟寮忔姤鍛婇紶鏍囪繍鍔ㄣ€傚畠鍙互鎶ュ憡鐩稿杩愬姩銆佸湪 ikbd 鍐呴儴
缁存姢鐨勫潗鏍囩郴涓殑缁濆杩愬姩锛屾垨鑰呭皢榧犳爣杩愬姩杞崲涓洪敭鐩樺厜鏍囨帶鍒堕敭绛夋晥閿€?榧犳爣鎸夐敭鍙互瑙嗕负榧犳爣鐨勪竴閮ㄥ垎锛屼篃鍙互瑙嗕负棰濆鐨勯敭鐩樻寜閿€?
### 鐩稿浣嶇疆鎶ュ憡

鍦ㄧ浉瀵逛綅缃ā寮忎笅锛屾瘡褰撳彂鐢熼紶鏍囦簨浠舵椂锛宨kbd 閮戒細杩斿洖鐩稿榧犳爣浣嶇疆璁板綍銆傞紶鏍囦簨浠?鍖呮嫭榧犳爣鎸夐敭琚寜涓嬫垨閲婃斁锛屾垨鑰呬换涓€杞寸殑杩愬姩瓒呰繃鍙缃殑闃堝€笺€傛棤璁洪槇鍊煎浣曪紝鎵€鏈?鍒嗚鲸鐜囦綅閮戒細杩斿洖缁欎富鏈猴紙host锛夎绠楁満銆?娉ㄦ剰锛宨kbd 杩斿洖鐨勭浉瀵归紶鏍囦綅缃姤鍛婁腑鐨?delta x 鎴?y 鍙兘鏄捐憲澶т簬闃堝€笺€傝繖鍙兘鍙戠敓锛?鍥犱负涓嶄細鐢熸垚鐩稿榧犳爣杩愬姩浜嬩欢鐨勬儏鍐垫槸锛?a) 褰撻敭鐩樺凡琚€滄殏鍋溾€濓紙浜嬩欢灏嗚瀛樺偍鐩村埌閿洏
閫氫俊鎭㈠锛夋椂锛?b) 褰撲换浣曚簨浠舵鍦ㄤ紶杈撴椂銆?
鐩稿榧犳爣浣嶇疆璁板綍鏄竴涓笁瀛楄妭璁板綍锛屽舰寮忎负锛?```

    %111110xy           ; mouse position record flag
                        ; where y is the right button state
                        ; and x is the left button state
    X                   ; delta x as twos complement integer
    Y                   ; delta y as twos complement integer

```
娉ㄦ剰锛屽嵆浣?MOUSE BUTTON ACTION 宸插皢鎸夐敭璁剧疆涓哄儚閿洏鐨勪竴閮ㄥ垎閭ｆ牱宸ヤ綔锛屾寜閿姸鎬佷綅
鐨勫€间篃搴斿綋鏄湁鏁堢殑銆?濡傛灉鍦ㄧ敓鎴愭姤鍛婃暟鎹寘涔嬪墠绱Н鐨勮繍鍔ㄨ秴鍑轰簡 +127...-128 鑼冨洿锛屽垯璇ヨ繍鍔ㄤ細鍒嗚В涓哄涓?鏁版嵁鍖呫€?娉ㄦ剰锛屾墍鎶ュ憡鐨?delta y 鐨勭鍙锋槸鎵€閫?Y 鍘熺偣鐨勫嚱鏁般€?
### 缁濆浣嶇疆鎶ュ憡

ikbd 涔熷彲浠ョ淮鎶ょ粷瀵归紶鏍囦綅缃€傚瓨鍦ㄧ敤浜庨噸缃紶鏍囦綅缃€佽缃?X/Y 缂╂斁姣斾緥浠ュ強鏌ヨ褰撳墠
榧犳爣浣嶇疆鐨勫懡浠ゃ€?
### 榧犳爣鍏夋爣閿ā寮?
ikbd 鍙互灏嗛紶鏍囪繍鍔ㄨ浆鎹负绛夋晥鐨勫厜鏍囨寜閿€傛瘡涓酱姣忔寜閿竴娆＄殑榧犳爣鐐瑰嚮娆℃暟鏄彲鐙珛
缂栫▼鐨勩€俰kbd 鍐呴儴浠ュ彲鐢ㄧ殑鏈€楂樺垎杈ㄧ巼缁存姢榧犳爣杩愬姩淇℃伅锛屽苟涓斾粎涓烘瘮渚嬪洜瀛愮殑姣忎釜鍊嶆暟
鐢熸垚涓€瀵瑰厜鏍囬敭浜嬩欢銆?榧犳爣杩愬姩浜х敓鍏夋爣閿殑鎸変笅锛坢ake锛夌爜锛岀揣闅忓叾鍚庣殑鏄浉搴斿厜鏍囬敭鐨勯噴鏀撅紙break锛夌爜銆傞紶鏍?鎸夐敭浜х敓鐨勬壂鎻忕爜楂樹簬閫氬父涓烘渶澶ц鎯抽敭鐩樻墍鍒嗛厤鐨勮寖鍥达紙鍗?LEFT=0x74 涓?RIGHT=0x75锛夈€?
## 娓告垙鏉嗭紙Joystick锛?
### 娓告垙鏉嗕簨浠舵姤鍛?
鍦ㄦ妯″紡涓嬶紝姣忓綋娓告垙鏉嗕綅缃敼鍙樻椂锛堝嵆姣忓綋娓告垙鏉嗗紑鍏虫垨鎵虫満锛坱rigger锛夐棴鍚堟垨鏂紑
鏃讹級锛宨kbd 閮戒細鐢熸垚涓€涓褰曘€?
```

    %1111111x           ; Joystick event marker
                        ; where x is Joystick 0 or 1
    %x000yyyy           ; where yyyy is the stick position
                        ; and x is the trigger

```

### 娓告垙鏉嗘煡璇?
鍦ㄦ妯″紡涓嬶紝鍙殢鏃堕€氳繃鍚?ikbd 鍙戦€佲€淚nterrogate Joystick锛堟煡璇㈡父鎴忔潌锛夆€濆懡浠ゆ潵
鏌ヨ娓告垙鏉嗙鍙ｇ殑褰撳墠鐘舵€併€?
```

    0xFD                ; joystick report header
    %x000yyyy           ; Joystick 0
    %x000yyyy           ; Joystick 1
                        ; where x is the trigger
                        ; and yyy is the stick position

```

### 娓告垙鏉嗙洃瑙?
鎻愪緵涓€绉嶆ā寮忥紝璇ユā寮忓嚑涔庡皢鎵€鏈夐敭鐩橀€氫俊鏃堕棿閮界敤浜庡湪鐢ㄦ埛鍙寚瀹氶€熺巼涓嬫姤鍛婃父鎴忔潌
绔彛鐨勭姸鎬併€傚畠浼氫竴鐩翠繚鎸佹妯″紡锛岀洿鍒拌閲嶇疆鎴栬鍛戒护杩涘叆鍙︿竴绉嶆ā寮忋€傛妯″紡涓嬬殑
PAUSE 鍛戒护涓嶄粎鍋滄杈撳嚭锛岃繕浼氭殏鏃跺仠姝㈡壂鎻忔父鎴忔潌锛堟牱鏈笉琚帓闃燂級銆?
### 鎵虫満鎸夐挳鐩戣

鎻愪緵涓€绉嶆ā寮忥紝鐢ㄤ簬浠ラ珮閫熺巼鐩戣鍗曚釜杈撳叆浣嶃€傚湪姝ゆā寮忎笅锛宨kbd 浠ヤ覆琛岄€氫俊閫氶亾鍏佽鐨?鏈€澶ч€熺巼鐩戣娓告垙鏉?1 鐨勬壋鏈猴紙fire锛夋寜閽殑鐘舵€併€傛暟鎹鎵撳寘涓烘瘡瀛楄妭 8 浣嶄紶杈撶粰涓绘満銆?ikbd 涓€鐩翠繚鎸佹妯″紡锛岀洿鍒拌閲嶇疆鎴栬鍛戒护杩涘叆鍙︿竴绉嶆ā寮忋€傛妯″紡涓嬬殑 PAUSE 鍛戒护涓嶄粎
鍋滄杈撳嚭锛岃繕浼氭殏鏃跺仠姝㈡壂鎻忔寜閽紙鏍锋湰涓嶈鎺掗槦锛夈€?
### 娓告垙鏉嗛敭鐮佹ā寮?
鍙互鍛戒护 ikbd 灏嗕换涓€娓告垙鏉嗙殑浣跨敤杞崲涓虹瓑鏁堢殑鍏夋爣鎺у埗鎸夐敭銆俰kbd 鎻愪緵涓€涓崟涓€
鏂偣锛坆reakpoint锛夐€熷害娓告垙鏉嗗厜鏍囥€?娓告垙鏉嗕簨浠朵骇鐢熸寜涓嬶紙make锛夌爜锛岀揣闅忓叾鍚庣殑鏄浉搴斿厜鏍囪繍鍔ㄩ敭鐨勯噴鏀撅紙break锛夌爜銆傛父鎴忔潌鐨?鎵虫満鎴栧紑鐏紙fire锛夋寜閽骇鐢熺殑浼寜閿壂鎻忕爜楂樹簬鎵€璁炬兂鐨勬渶澶ф寜閿煩闃垫墍浣跨敤鐨勮寖鍥?锛堝嵆 JOYSTICK0=0x74锛孞OYSTICK1=0x75锛夈€?
## 鏃ユ湡鏃堕挓锛圱ime-of-Day Clock锛?
ikbd 杩樹负绯荤粺缁存姢涓€涓棩鏈熸椂閽熴€傛彁渚涙湁鐢ㄤ簬璁剧疆鍜屾煡璇㈣鏃ユ湡鏃堕挓鐨勫懡浠ゃ€傝鏃讹紙Time-keeping锛?鐨勭淮鎶ゅ垎杈ㄧ巼鍙揪涓€绉掋€?
## 鐘舵€佹煡璇?
鍙互閫氳繃鍙戦€佷笌 ikbd 璁剧疆鍛戒护鐩稿搴旂殑鐘舵€佹煡璇㈠懡浠わ紝鏉ヨ幏鐭?ikbd 妯″紡鍜屽弬鏁扮殑
褰撳墠鐘舵€併€?
## 涓婄數妯″紡锛圥ower-Up Mode锛?
閿洏鎺у埗鍣ㄥ湪涓婄數鏃跺皢鎵ц涓€涓畝鍗曠殑鑷锛屼互妫€娴嬩富瑕佺殑鎺у埗鍣ㄦ晠闅滐紙ROM 鏍￠獙鍜屼笌
RAM 娴嬭瘯锛変互鍙婅濡傚崱閿紙stuck keys锛変箣绫荤殑闂銆備笂鐢垫椂鎸変笅鐨勪换浣曢敭閮借鍋囧畾涓哄崱浣忥紝
骞惰繑鍥炲叾 BREAK锛堝師鏂囧姝わ級鐮侊紙鍦ㄦ病鏈夊墠缃?MAKE 鐮佺殑鎯呭喌涓嬶紝杩欐槸涓€涓敭鐩橀敊璇殑鏍囧織锛夈€?濡傛灉鎺у埗鍣ㄨ嚜妫€鏃犺瀹屾垚锛屽垯杩斿洖浠ｇ爜 0xF0銆傦紙璇ヤ唬鐮佸皢鐢ㄤ簬鎸囩ず ikbd 鎺у埗鍣ㄧ殑鐗堟湰/鍙戝竷銆?ikbd 鐨勯涓彂甯冪増鏈负 0xF0锛岃嫢鍙戝竷绗簩涓増鏈垯涓?0xF1锛屼緷姝ょ被鎺ㄣ€傦級
ikbd 榛樿涓洪紶鏍囦綅缃姤鍛婃ā寮忥紝涓や釜杞寸殑闃堝€煎潎涓?1 涓崟浣嶏紝Y=0 鍘熺偣浣嶄簬灞忓箷椤堕儴锛屽苟
瀵规父鎴忔潌 1 閲囩敤娓告垙鏉嗕簨浠舵姤鍛婃ā寮忥紝涓や釜鎸夐敭鍦ㄩ€昏緫涓婇兘鍒嗛厤缁欓紶鏍囥€傚湪浠讳綍娓告垙鏉嗗懡浠?涔嬪悗锛宨kbd 鍋囧畾娓告垙鏉?0 鍜屾父鎴忔潌 1 鍧囧凡杩炴帴銆傜劧鍚庝换浣曢紶鏍囧懡浠わ紙MOUSE DISABLE 闄ゅ锛?浼氫娇绔彛 0 鍐嶆琚綋浣滈紶鏍囨壂鎻忥紝骞朵笖涓や釜鎸夐敭鍦ㄩ€昏緫涓婇兘杩炴帴鍒板畠銆傚鏋滃湪鍋囧畾绔彛 0 涓?榧犳爣鏃舵敹鍒伴紶鏍囩鐢ㄥ懡浠わ紝鍒欒鎸夐敭鍦ㄩ€昏緫涓婅鍒嗛厤缁欐父鎴忔潌 1锛堢洿鍒伴紶鏍囪鍙︿竴涓紶鏍囧懡浠?閲嶆柊鍚敤锛夈€?
## ikbd 鍛戒护闆?
鏈妭鍖呭惈鍙彂閫佺粰 ikbd 鐨勫懡浠ゅ垪琛ㄣ€傛湭鎸囧畾鐨勫懡浠ょ爜锛堝 0x00锛夊簲褰撲笉鎵ц浠讳綍鎿嶄綔
锛圢OPs锛夈€?
### RESET

```

    0x80
    0x01

```

娉ㄦ剰锛歊ESET 鍛戒护鏄?ikbd 鎵€鑳界悊瑙ｇ殑鍞竴涓€涓弻瀛楄妭鍛戒护銆備换浣曡窡鍦?0x80 鍛戒护瀛楄妭涔嬪悗銆?闄?0x01 浠ュ鐨勫瓧鑺傞兘灏嗚蹇界暐锛堝苟瀵艰嚧 0x80 琚拷鐣ワ級銆?涔熷彲浠ラ€氳繃鍚?ikbd 鍙戦€佹寔缁嚦灏?200mS 鐨?break 鏉ュ紩璧峰浣嶃€?鎵ц RESET 鍛戒护浼氫娇閿洏杩斿洖鍏堕粯璁わ紙涓婄數锛夋ā寮忓拰鍙傛暟璁剧疆銆傚畠涓嶅奖鍝嶆棩鏈熸椂閽熴€?RESET 鍛戒护鎴栧姛鑳戒細浣?ikbd 鎵ц涓€涓畝鍗曠殑鑷銆傚鏋滄祴璇曟垚鍔燂紝ikbd 灏嗗湪鏀跺埌 RESET
鍛戒护锛堟垨 break 缁撴潫锛屾垨涓婄數锛夊悗鐨?300mS 鍐呭彂閫佷唬鐮?0xF0銆傜劧鍚?ikbd 浼氭壂鎻忔寜閿煩闃?浠ユ煡鎵句换浣曞崱浣忥紙闂悎锛夌殑閿€傚彂鐜扮殑浠讳綍闂悎閿兘浼氬鑷寸敓鎴愰噴鏀撅紙break锛夋壂鎻忕爜锛堥噴鏀?鐮佸湪娌℃湁鍓嶇疆鎸変笅锛坢ake锛夌爜鐨勬儏鍐典笅鍒拌揪锛屽氨鏄寜閿煩闃甸敊璇殑鏍囧織锛夈€?
### SET MOUSE BUTTON ACTION

```

    0x07
    %00000mss           ; mouse button action
                        ;       (m is presumed = 1 when in MOUSE KEYCODE mode)
                        ; mss=0xy, mouse button press or release causes mouse
                        ;  position report
                        ;  where y=1, mouse key press causes absolute report
                        ;  and x=1, mouse key release causes absolute report
                        ; mss=100, mouse buttons act like keys

```

姝ゅ懡浠よ缃?ikbd 搴斿浣曞鐞嗛紶鏍囦笂鐨勬寜閿€傞粯璁ょ殑榧犳爣鎸夐敭鍔ㄤ綔妯″紡涓?%00000000锛屾寜閿?鍦ㄩ€昏緫涓婅瑙嗕负榧犳爣鐨勪竴閮ㄥ垎銆?褰撴寜閿〃鐜板緱鍍忔寜閿椂锛孡EFT=0x74 涓?RIGHT=0x75銆?
### SET RELATIVE MOUSE POSITION REPORTING

```

    0x08

```

璁剧疆鐩稿榧犳爣浣嶇疆鎶ュ憡銆傦紙榛樿锛夋瘡褰撲换涓€杞寸殑杩愬姩瓒呰繃鍙缃殑闃堝€兼椂锛岄紶鏍囦綅缃暟鎹寘
鐢?ikbd 寮傛鐢熸垚锛堝弬瑙?SET MOUSE THRESHOLD锛夈€傛牴鎹紶鏍囬敭妯″紡鐨勪笉鍚岋紝榧犳爣浣嶇疆鎶ュ憡
涔熷彲鑳藉湪涓や釜榧犳爣鎸夐敭涓殑浠讳綍涓€涓鎸変笅鎴栭噴鏀炬椂鐢熸垚銆傚惁鍒欓紶鏍囨寜閿殑琛屼负灏卞儚閿洏
鎸夐敭涓€鏍枫€?
### SET ABSOLUTE MOUSE POSITIONING

```

    0x09
    XMSB                ; X maximum (in scaled mouse clicks)
    XLSB
    YMSB                ; Y maximum (in scaled mouse clicks)
    YLSB

```

璁剧疆缁濆榧犳爣浣嶇疆缁存姢銆傞噸缃?ikbd 缁存姢鐨?X 鍜?Y 鍧愭爣銆?鍦ㄦ妯″紡涓嬶紝鍐呴儴缁存姢鐨勫潗鏍囧€间笉浼氬湪 0 鍜岃緝澶х殑姝ｆ暟涔嬮棿鍥炵粫锛坵rap锛夈€備綆浜?0 鐨勮繃閲?杩愬姩琚拷鐣ャ€傝鍛戒护璁剧疆鍙湪缂╂斁鍧愭爣绯荤粺涓揪鍒扮殑鏈€澶ф鍊笺€傝秴鍑鸿鍊肩殑杩愬姩涔熻蹇界暐銆?
### SET MOUSE KEYCODE MODE

```

    0x0A
    deltax              ; distance in X clicks to return (LEFT) or (RIGHT)
    deltay              ; distance in Y clicks to return (UP) or (DOWN)

```

璁剧疆榧犳爣鐩戣渚嬬▼浠ヨ繑鍥炲厜鏍囪繍鍔ㄩ敭鐮侊紝鑰屼笉鏄浉瀵规垨缁濆杩愬姩璁板綍銆俰kbd 鍦ㄤ换涓€杞寸殑榧犳爣
绉诲姩閲忚秴杩囩敤鎴锋寚瀹氱殑 delta 鍚庤繑鍥炵浉搴旂殑鍏夋爣閿爜銆傚綋閿洏澶勪簬閿壂鎻忕爜妯″紡鏃讹紝榧犳爣杩愬姩
浼氬鑷存寜涓嬶紙make锛夌爜涔嬪悗绱ц窡鐫€閲婃斁锛坆reak锛夌爜銆傛敞鎰忥紝姝ゅ懡浠や笉鍙楅紶鏍囪繍鍔ㄥ師鐐圭殑褰卞搷銆?
### SET MOUSE THRESHOLD

```

    0x0B
    X                   ; x threshold in mouse ticks (positive integers)
    Y                   ; y threshold in mouse ticks (positive integers)

```

姝ゅ懡浠よ缃湪鐢熸垚榧犳爣浜嬩欢涔嬪墠鐨勯槇鍊笺€傛敞鎰忥紝瀹冧笉褰卞搷杩斿洖缁欎富鏈虹殑鏁版嵁鐨勫垎杈ㄧ巼銆傛
鍛戒护浠呭湪 RELATIVE MOUSE POSITIONING 妯″紡涓嬫湁鏁堛€傞槇鍊煎湪 RESET锛堟垨涓婄數锛夋椂榛樿涓?1銆?
### SET MOUSE SCALE

```

    0x0C
    X                   ; horizontal mouse ticks per internal X
    Y                   ; vertical mouse ticks per internal Y

```

姝ゅ懡浠よ缃?ABSOLUTE MOUSE POSITIONING 妯″紡鐨勬瘮渚嬪洜瀛愩€傚湪姝ゆā寮忎笅锛屽繀椤诲彂鐢熸寚瀹氭暟閲?鐨勯紶鏍囩浉浣嶅彉鍖栵紙鈥滅偣鍑伙紙click锛夆€濓級锛屽唴閮ㄧ殑缁存姢鍧愭爣鎵嶄細鏀瑰彉 1锛堟瘡涓酱鐙珛缂╂斁锛夈€?璇疯浣忥紝闄ら潪宸插懡浠?ikbd 鍦ㄦ寜閿寜涓嬫垨閲婃斁鏃惰繘琛屾姤鍛婏紙鍙傝 SET MOUSE BUTTON ACTION锛夛紝
鍚﹀垯榧犳爣浣嶇疆淇℃伅浠呰兘閫氳繃鏌ヨ ABSOLUTE MOUSE POSITIONING 妯″紡涓嬬殑 ikbd 鑾峰緱銆?
### INTERROGATE MOUSE POSITION

```

    0x0D
    Returns:
            0xF7       ; absolute mouse position header
    BUTTONS
            0000dcba   ; where a is right button down since last interrogation
                       ; b is right button up since last
                       ; c is left button down since last
                       ; d is left button up since last
            XMSB       ; X coordinate
            XLSB
            YMSB       ; Y coordinate
            YLSB

```

INTERROGATE MOUSE POSITION 鍛戒护鍦?ABSOLUTE MOUSE POSITIONING 妯″紡涓嬫湁鏁堬紝鏃犺
MOUSE BUTTON ACTION 鐨勮缃浣曘€?
### LOAD MOUSE POSITION

```

    0x0E
    0x00                ; filler
    XMSB                ; X coordinate
    XLSB                ; (in scaled coordinate system)
    YMSB                ; Y coordinate
    YLSB

```

姝ゅ懡浠ゅ厑璁哥敤鎴烽璁惧唴閮ㄧ淮鎶ょ殑缁濆榧犳爣浣嶇疆銆?
### SET Y=0 AT BOTTOM

```

    0x0F

```

姝ゅ懡浠や娇 Y 杞寸殑鍘熺偣浣嶄簬 ikbd 鍐呴儴鎵€鏈夌浉瀵规垨缁濆榧犳爣杩愬姩閫昏緫鍧愭爣绯荤殑搴曢儴銆傝繖浣?鏈濆悜鐢ㄦ埛鐨勯紶鏍囪繍鍔ㄧ鍙蜂负璐燂紝杩滅鐢ㄦ埛鐨勯紶鏍囪繍鍔ㄧ鍙蜂负姝ｃ€?
### SET Y=0 AT TOP

```

    0x10

```

浣?Y 杞寸殑鍘熺偣浣嶄簬 ikbd 鍐呴儴鎵€鏈夌浉瀵规垨缁濆榧犳爣杩愬姩閫昏緫鍧愭爣绯荤殑椤堕儴銆傦紙榛樿锛?杩欎娇鏈濆悜鐢ㄦ埛鐨勯紶鏍囪繍鍔ㄧ鍙蜂负姝ｏ紝杩滅鐢ㄦ埛鐨勯紶鏍囪繍鍔ㄧ鍙蜂负璐熴€?
### RESUME

```

    0x11

```

鎭㈠鍚戜富鏈哄彂閫佹暟鎹€傜敱浜?ikbd 鍦ㄨ緭鍑鸿鏆傚仠鍚庢敹鍒扮殑浠讳綍鍛戒护涔熶細瀵艰嚧闅愬紡 RESUME锛屽洜姝?姝ゅ懡浠ゅ彲琚涓轰竴涓棤鎿嶄綔锛圢O OPERATION锛夊懡浠ゃ€傚鏋?ikbd 鏀跺埌姝ゅ懡浠よ€屽畠骞舵湭澶勪簬 PAUSED
鐘舵€侊紝鍒欑畝鍗曞湴灏嗗叾蹇界暐銆?
### DISABLE MOUSE

```

    0x12

```

绂佺敤鎵€鏈夐紶鏍囦簨浠舵姤鍛婏紙骞朵笖鎵弿鍙兘鍦ㄥ唴閮ㄨ绂佺敤锛夈€備换浣曟湁鏁堢殑榧犳爣妯″紡鍛戒护閮戒細鎭㈠
榧犳爣杩愬姩鐩戣銆傦紙鏈夋晥鐨勯紶鏍囨ā寮忓懡浠ゆ湁 SET RELATIVE MOUSE POSITION REPORTING銆丼ET
ABSOLUTE MOUSE POSITIONING 浠ュ強 SET MOUSE KEYCODE MODE銆傦級
娉ㄦ剰锛氬鏋滈紶鏍囨寜閿凡琚懡浠よ〃鐜板緱鍍忛敭鐩樻寜閿紝姝ゅ懡浠?*纭疄**浼氬奖鍝嶅畠浠殑鍔ㄤ綔銆?
### PAUSE OUTPUT

```

    0x13

```

鍋滄鍚戜富鏈哄彂閫佹暟鎹紝鐩村埌鏀跺埌鍙︿竴涓湁鏁堝懡浠ゃ€傛寜閿煩闃垫椿鍔ㄤ粛琚洃瑙嗭紝鎵弿鐮佹垨 ASCII
瀛楃琚帓闃燂紙鏈€澶氬彈寰帶鍒跺櫒鏀寔鐨勬暟閲忥級锛屼互渚垮湪涓绘満鍏佽鎭㈠杈撳嚭鏃跺彂閫併€傚鏋滃浜?JOYSTICK EVENT REPORTING 妯″紡锛屾父鎴忔潌浜嬩欢涔熶細琚帓闃熴€?鍦ㄨ緭鍑烘殏鍋滄湡闂达紝榧犳爣杩愬姩搴斿綋琚疮绉€傚鏋?ikbd 澶勪簬 RELATIVE MOUSE POSITIONING
REPORTING 妯″紡锛岃繍鍔ㄤ細鍦ㄦ甯搁槇鍊奸檺鍒朵箣澶栫疮绉紝浠ュ湪杈撳嚭鎭㈠鏃朵骇鐢熶紶杈撴墍闇€鐨勬渶灏?鏁版嵁鍖呮暟閲忋€傚鏋滈紶鏍囧浜?RELATIVE MOUSE POSITION REPORTING 妯″紡锛屾寜涓嬫垨閲婃斁浠讳竴
榧犳爣鎸夐敭浼氬鑷翠换浣曠疮绉殑杩愬姩绔嬪嵆浣滀负鏁版嵁鍖呮帓闃熴€?鐢变簬寰帶鍒跺櫒鍐呭瓨鐨勯檺鍒讹紝姝ゅ懡浠ゅ簲褰撹皑鎱庝娇鐢紝骞朵笖姣忔鍏抽棴杈撳嚭鐨勬椂闂翠笉搴旇秴杩?<tbd> 姣銆?杈撳嚭浠呭湪褰撳墠鈥滀簨浠讹紙event锛夆€濈粨鏉熸椂鎵嶅仠姝€傚鏋滃湪澶氬瓧鑺傛姤鍛婄殑涓€旀敹鍒?PAUSE OUTPUT
鍛戒护锛岃鏁版嵁鍖呬粛灏嗚浼犺緭瀹屾瘯锛岀劧鍚?PAUSE 鎵嶄細鐢熸晥銆?褰?ikbd 澶勪簬 JOYSTICK MONITORING 妯″紡鎴?FIRE BUTTON MONITORING 妯″紡鏃讹紝PAUSE OUTPUT
鍛戒护涔熶細鏆傛椂鍋滄鐩戣杩囩▼锛堝嵆鏍锋湰涓嶈鎺掗槦浠ヤ紶杈擄級銆?
### SET JOYSTICK EVENT REPORTING

```

    0x14

```

杩涘叆 JOYSTICK EVENT REPORTING 妯″紡锛堥粯璁わ級銆傛父鎴忔潌寮€鍏虫垨鎵虫満鐨勬瘡娆℃柇寮€鎴栭棴鍚堥兘浼?瀵艰嚧鐢熸垚涓€涓父鎴忔潌浜嬩欢璁板綍銆?
### SET JOYSTICK INTERROGATION MODE

```

    0x15

```

绂佺敤 JOYSTICK EVENT REPORTING銆備富鏈哄繀椤诲彂閫佸崟鐙殑 JOYSTICK INTERROGATE 鍛戒护鏉ユ劅鐭?娓告垙鏉嗙姸鎬併€?
### JOYSTICK INTERROGATE

```

    0x16

```

杩斿洖涓€涓寚绀烘父鎴忔潌褰撳墠鐘舵€佺殑璁板綍銆傛鍛戒护鍦?JOYSTICK EVENT REPORTING 妯″紡鎴?JOYSTICK INTERROGATION MODE 涓嬪潎鏈夋晥銆?
### SET JOYSTICK MONITORING

```

    0x17
    rate                ; time between samples in hundredths of a second
    Returns: (in packets of two as long as in mode)
            %000000xy   ; where y is JOYSTICK1 Fire button
                        ; and x is JOYSTICK0 Fire button
            %nnnnmmmm   ; where m is JOYSTICK1 state
                        ; and n is JOYSTICK0 state

```

璁剧疆 ikbd 鍙洃瑙嗕覆琛屽懡浠ょ嚎銆佺淮鎶ゆ棩鏈熸椂閽熷苟鐩戣娓告垙鏉嗐€俽ate 璁剧疆娓告垙鏉嗛噰鏍蜂箣闂寸殑
闂撮殧銆?娉ㄦ剰锛氱敤鎴蜂笉搴斿皢 rate 璁剧疆寰楅珮浜庝覆琛岄€氫俊閫氶亾鎵€鑳藉厑璁镐紶杈撹繖 2 瀛楄妭鏁版嵁鍖呯殑閫熺巼銆?
### SET FIRE BUTTON MONITORING

```

    0x18
    Returns: (as long as in mode)
            %bbbbbbbb   ; state of the JOYSTICK1 fire button packed
                        ; 8 bits per byte, the first sample if the MSB

```

璁剧疆 ikbd 鍙洃瑙嗕覆琛屽懡浠ょ嚎銆佺淮鎶ゆ棩鏈熸椂閽熷苟鐩戣娓告垙鏉?1 涓婄殑寮€鐏紙fire锛夋寜閽€傚紑鐏?鎸夐挳鐨勬壂鎻忛€熺巼浣垮緱鍦ㄥ墠涓€涓瓧鑺傚彂閫佺粰涓绘満鎵€闇€鐨勬椂闂村唴杩涜 8 娆￠噰鏍凤紙鍗虫壂鎻忛€熺巼 =
8/10 脳 娉㈢壒鐜囷級銆傞噰鏍烽棿闅斿簲灏藉彲鑳芥亽瀹氥€?
### SET JOYSTICK KEYCODE MODE

```

    0x19
    RX                  ; length of time (in tenths of seconds) until
                        ; horizontal velocity breakpoint is reached
    RY                  ; length of time (in tenths of seconds) until
                        ; vertical velocity breakpoint is reached
    TX                  ; length (in tenths of seconds) of joystick closure
                        ; until horizontal cursor key is generated before RX
                        ; has elapsed
    TY                  ; length (in tenths of seconds) of joystick closure
                        ; until vertical cursor key is generated before RY
                        ; has elapsed
    VX                  ; length (in tenths of seconds) of joystick closure
                        ; until horizontal cursor keystrokes are generated
                        ; after RX has elapsed
    VY                  ; length (in tenths of seconds) of joystick closure
                        ; until vertical cursor keystrokes are generated
                        ; after RY has elapsed

```

鍦ㄦ妯″紡涓嬶紝娓告垙鏉?0 浠ヤ竴绉嶆ā鎷熷厜鏍囨寜閿殑鏂瑰紡琚壂鎻忋€傚湪鍒濆闂悎鏃讹紝鐢熸垚涓€瀵规寜閿?锛堟寜涓?閲婃斁锛夈€傜劧鍚庡湪鏈€澶?Rn 涓崄鍒嗕箣涓€绉掍箣鍚庯紝姣?Tn 涓崄鍒嗕箣涓€绉掔敓鎴愪竴瀵规寜閿€傚湪
杈惧埌 Rn 鏂偣鍚庯紝姣?Vn 涓崄鍒嗕箣涓€绉掔敓鎴愪竴瀵规寜閿€傝繖鎻愪緵浜嗕竴涓€熷害锛堣嚜鍔ㄩ噸澶嶏級鏂偣
鐗规€с€?娉ㄦ剰锛岄€氳繃灏?RX 鍜?鎴?RY 璁剧疆涓洪浂锛屽彲浠ョ鐢ㄩ€熷害鐗规€с€傛鏃?TX 鍜?TY 鐨勫€煎彉寰楁棤鎰忎箟锛?鑰屽厜鏍団€滄寜閿€濈殑鐢熸垚鐢?VX 鍜?VY 鍐冲畾銆?
### DISABLE JOYSTICKS

```

    0x1A

```

绂佺敤浠讳綍娓告垙鏉嗕簨浠剁殑鐢熸垚锛堝苟涓旀壂鎻忓彲鑳藉湪鍐呴儴琚鐢級銆備换浣曟湁鏁堢殑娓告垙鏉嗘ā寮忓懡浠ら兘浼?鎭㈠娓告垙鏉嗙洃瑙嗐€傦紙娓告垙鏉嗘ā寮忓懡浠ゅ寘鎷?SET JOYSTICK EVENT REPORTING銆丼ET JOYSTICK
INTERROGATION MODE銆丼ET JOYSTICK MONITORING銆丼ET FIRE BUTTON MONITORING 浠ュ強
SET JOYSTICK KEYCODE MODE銆傦級

### TIME-OF-DAY CLOCK SET

```

    0x1B
    YY                  ; year (2 least significant digits)
    MM                  ; month
    DD                  ; day
    hh                  ; hour
    mm                  ; minute
    ss                  ; second

```

鎵€鏈夋棩鏈熸椂閽熸暟鎹兘搴斾互鍘嬬缉 BCD 鏍煎紡鍙戦€佺粰 ikbd銆?浠讳綍涓嶆槸鏈夋晥 BCD 浣嶇殑鏁板瓧搴旇瑙嗕负鈥滀笉鍏冲績锛坉on't care锛夆€濓紝骞朵笖涓嶆敼鍙樻棩鏈熸垨鏃堕棿鐨?璇ョ壒瀹氬瓧娈点€傝繖鍏佽鍙缃棩鏈熸椂閽熺殑鏌愪簺瀛愬瓧娈点€?
### INTERROGATE TIME-OF-DAT CLOCK

```

    0x1C
    Returns:
            0xFC        ; time-of-day event header
            YY          ; year (2 least significant digits)
            MM          ; month
            DD          ; day
            hh          ; hour
            mm          ; minute
            ss          ; second

    All time-of-day is sent in packed BCD format.

```

### MEMORY LOAD

```

    0x20
    ADRMSB              ; address in controller
    ADRLSB              ; memory to be loaded
    NUM                 ; number of bytes (0-128)
    { data }

```

姝ゅ懡浠ゅ厑璁镐富鏈哄皢浠绘剰鍊煎姞杞藉埌 ikbd 鎺у埗鍣ㄥ唴瀛樹腑銆傛暟鎹瓧鑺備箣闂寸殑鏃堕棿闂撮殧蹇呴』灏忎簬 20ms銆?
### MEMORY READ

```

    0x21
    ADRMSB              ; address in controller
    ADRLSB              ; memory to be read
    Returns:
            0xF6        ; status header
            0x20        ; memory access
            { data }    ; 6 data bytes starting at ADR

```

姝ゅ懡浠ゅ厑璁镐富鏈轰粠 ikbd 鎺у埗鍣ㄥ唴瀛樹腑璇诲彇銆?
### CONTROLLER EXECUTE

```

    0x22
    ADRMSB              ; address of subroutine in
    ADRLSB              ; controller memory to be called

```

姝ゅ懡浠ゅ厑璁镐富鏈哄懡浠ゆ墽琛?ikbd 鎺у埗鍣ㄥ唴瀛樹腑鐨勪竴涓瓙渚嬬▼銆?
### STATUS INQUIRIES

```

    Status commands are formed by inclusively ORing 0x80 with the
    relevant SET command.

    Example:
    0x88 (or 0x89 or 0x8A)  ; request mouse mode
    Returns:
            0xF6        ; status response header
            mode        ; 0x08 is RELATIVE
                        ; 0x09 is ABSOLUTE
                        ; 0x0A is KEYCODE
            param1      ; 0 is RELATIVE
                        ; XMSB maximum if ABSOLUTE
                        ; DELTA X is KEYCODE
            param2      ; 0 is RELATIVE
                        ; YMSB maximum if ABSOLUTE
                        ; DELTA Y is KEYCODE
            param3      ; 0 if RELATIVE
                        ; or KEYCODE
                        ; YMSB is ABSOLUTE
            param4      ; 0 if RELATIVE
                        ; or KEYCODE
                        ; YLSB is ABSOLUTE
            0           ; pad
            0

```

STATUS INQUIRY 鍛戒护璇锋眰 ikbd 杩斿洖褰撳墠妯″紡鎴栦笌缁欏畾鍛戒护鍏宠仈鐨勫弬鏁般€傛墍鏈夌姸鎬佹姤鍛婇兘琚?濉厖涓哄舰鎴?8 瀛楄妭闀跨殑杩斿洖鏁版嵁鍖呫€傚鐘舵€佽姹傜殑鍝嶅簲琚璁℃垚杩欐牱锛氫富鏈哄彲浠ュ皢瀹冧滑瀛樺偍
璧锋潵锛堝湪鍓ョ鐘舵€佹姤鍛婂ご瀛楄妭涔嬪悗锛夛紝骞跺湪浠ュ悗浣滀负鍛戒护鍙戝洖缁?ikbd 浠ユ仮澶嶅叾鐘舵€併€? 濉厖
瀛楄妭浼氳 ikbd 瑙嗕负 NOP銆?
```

            0x87    mouse button action
            0x88    mouse mode
            0x89
            0x8A
            0x8B    mnouse threshold
            0x8C    mouse scale
            0x8F    mouse vertical coordinates
            0x90    ( returns       0x0F Y=0 at bottom
                            0x10 Y=0 at top )
            0x92    mouse enable/disable
                    ( returns       0x00 enabled)
                            0x12 disabled )
            0x94    joystick mode
            0x95
            0x96
            0x9A    joystick enable/disable
                    ( returns       0x00 enabled
                            0x1A disabled )

```

鍦ㄥ悓涓€鏃堕棿鍙湁涓€涓湭绛斿鐨勬煡璇㈠浜庤繘琛屼腑锛岃繖鏄紙涓绘満锛夌▼搴忓憳鐨勮矗浠汇€?濡傛灉 ikbd 澶勪簬 JOYSTICK MONITORING 妯″紡鎴?FIRE BUTTON MONITORING 妯″紡锛屽垯 STATUS
INQUIRY 鍛戒护鏃犳晥銆?
## 鎵弿鐮侊紙SCAN CODES锛?
ikbd 杩斿洖鐨勯敭鎵弿鐮佽閫夋嫨涓虹畝鍖?GSX 鐨勫疄鐜般€?
GSX Standard Keyboard Mapping

======= ============
Hex	Keytop
======= ============
01	Esc
02	1
03	2
04	3
05	4
06	5
07	6
08	7
09	8
0A	9
0B	0
0C	\-
0D	\=
0E	BS
0F	TAB
10	Q
11	W
12	E
13	R
14	T
15	Y
16	U
17	I
18	O
19	P
1A	[
1B	]
1C	RET
1D	CTRL
1E	A
1F	S
20	D
21	F
22	G
23	H
24	J
25	K
26	L
27	;
28	'
29	\`
2A	(LEFT) SHIFT
2B	\\
2C	Z
2D	X
2E	C
2F	V
30	B
31	N
32	M
33	,
34	.
35	/
36	(RIGHT) SHIFT
37	{ NOT USED }
38	ALT
39	SPACE BAR
3A	CAPS LOCK
3B	F1
3C	F2
3D	F3
3E	F4
3F	F5
40	F6
41	F7
42	F8
43	F9
44	F10
45	{ NOT USED }
46	{ NOT USED }
47	HOME
48	UP ARROW
49	{ NOT USED }
4A	KEYPAD -
4B	LEFT ARROW
4C	{ NOT USED }
4D	RIGHT ARROW
4E	KEYPAD +
4F	{ NOT USED }
50	DOWN ARROW
51	{ NOT USED }
52	INSERT
53	DEL
54	{ NOT USED }
5F	{ NOT USED }
60	ISO KEY
61	UNDO
62	HELP
63	KEYPAD (
64	KEYPAD /
65	KEYPAD *
66	KEYPAD *
67	KEYPAD 7
68	KEYPAD 8
69	KEYPAD 9
6A	KEYPAD 4
6B	KEYPAD 5
6C	KEYPAD 6
6D	KEYPAD 1
6E	KEYPAD 2
6F	KEYPAD 3
70	KEYPAD 0
71	KEYPAD .
72	KEYPAD ENTER
======= ============
