
## Configfs GPIO 妯℃嫙鍣?

configfs GPIO 妯℃嫙鍣紙gpio-sim锛夋彁渚涗簡涓€绉嶅垱寤虹敤浜庢祴璇曠殑妯℃嫙 GPIO 鑺墖鐨勬柟娉曘€傝繖浜涜姱鐗?鏆撮湶鐨勭嚎璺棦鍙互浣跨敤鏍囧噯 GPIO 瀛楃璁惧鎺ュ彛璁块棶锛屼篃鍙互浣跨敤 sysfs 灞炴€ц繘琛屾搷浣溿€?
### 鍒涘缓妯℃嫙鑺墖


gpio-sim 妯″潡娉ㄥ唽浜嗕竴涓悕涓?`'gpio-sim'` 鐨?configfs 瀛愮郴缁熴€傚叧浜?configfs 鏂囦欢绯荤粺鐨?缁嗚妭锛岃鍙傞槄 configfs 鏂囨。銆?
鐢ㄦ埛鍙互鍒涘缓 configfs 缁勫拰鏉＄洰鐨勫眰绾х粨鏋勶紝骞朵慨鏀规墍鏆撮湶灞炴€х殑鍊笺€備竴鏃﹁姱鐗囪瀹炰緥鍖栵紝杩欎釜
灞傜骇缁撴瀯灏嗚杞崲涓虹浉搴旂殑璁惧灞炴€с€傛€讳綋缁撴瀯濡備笅锛?
**缁勶細** `/config/gpio-sim`

杩欐槸 gpio-sim configfs 鏍戠殑椤跺眰鐩綍銆?
**缁勶細** `/config/gpio-sim/gpio-device`

**灞炴€э細** `/config/gpio-sim/gpio-device/dev_name`

**灞炴€э細** `/config/gpio-sim/gpio-device/live`

杩欐槸涓€涓〃绀?GPIO 骞冲彴璁惧鐨勭洰褰曘€俙'dev_name'` 灞炴€ф槸鍙鐨勶紝鍏佽鐢ㄦ埛绌洪棿璇诲彇骞冲彴璁惧
鍚嶏紙渚嬪 `'gpio-sim.0'`锛夈€俙'live'` 灞炴€х敤浜庡湪璁惧瀹屽叏閰嶇疆濂藉悗瑙﹀彂鍏跺疄闄呭垱寤恒€傚彲鎺ュ彈鐨勫€?涓猴細`'1'` 鍚敤妯℃嫙璁惧锛宍'0'` 绂佺敤骞舵媶闄ゅ畠銆?
**缁勶細** `/config/gpio-sim/gpio-device/gpio-bankX`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/chip_name`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/num_lines`

璇ョ粍琛ㄧず椤跺眰骞冲彴璁惧涓嬬殑涓€涓?GPIO bank銆俙'chip_name'` 灞炴€ф槸鍙鐨勶紝鍏佽鐢ㄦ埛绌洪棿璇诲彇
璇?bank 璁惧鐨勮澶囧悕銆俙'num_lines'` 灞炴€х敤浜庢寚瀹氳 bank 鏆撮湶鐨勭嚎璺暟閲忋€?
**缁勶細** `/config/gpio-sim/gpio-device/gpio-bankX/lineY`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/name`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/valid`

璇ョ粍琛ㄧず鍋忕Щ涓?Y 鐨勫崟鏉＄嚎璺€俙valid` 灞炴€ф寚绀鸿绾胯矾鏄惁鍙敤浣?GPIO銆俙name` 灞炴€х敤浜庤缃?鐢?'gpio-line-names' 灞炴€ф墍琛ㄧず鐨勭嚎璺悕銆?
**鏉＄洰锛?* `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog/name`

**灞炴€э細** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog/direction`

璇ユ潯鐩 gpio-sim 妯″潡鐙崰锛坔og锛夊叧鑱旂殑绾胯矾銆俙'name'` 灞炴€ф寚瀹氳浣跨敤鐨勫唴鏍稿唴娑堣垂鑰呭悕銆?`'direction'` 灞炴€ф寚瀹氱嫭鍗犳柟鍚戯紝涓斿繀椤讳负浠ヤ笅涔嬩竴锛歚'input'`銆乣'output-high'` 鍜?`'output-low'`銆?
鍦ㄦ瘡涓?bank 鐩綍鍐呴儴锛屾湁涓€缁勫彲鐢ㄤ簬閰嶇疆鏂拌姱鐗囩殑灞炴€с€傛澶栵紝鐢ㄦ埛鍙互鍦ㄨ姱鐗囩洰褰曞唴 `mkdir()`
瀛愮洰褰曪紝鐢ㄤ簬浼犻€掔壒瀹氱嚎璺殑棰濆閰嶇疆銆傝繖浜涘瓙鐩綍鐨勫悕绉板繀椤婚噰鐢?`'line<offset>'` 鐨勫舰寮?锛堜緥濡?`'line0'`銆乣'line20'` 绛夛級锛屽洜涓鸿鍚嶇О浼氳妯″潡鐢ㄦ潵鎶婇厤缃垎閰嶇粰缁欏畾鍋忕Щ澶勭殑鐗瑰畾
绾胯矾銆?
閰嶇疆瀹屾垚鍚庯紝蹇呴』灏?`'live'` 灞炴€ц涓?1 浠ュ疄渚嬪寲鑺墖銆備篃鍙互灏嗗叾璁惧洖 0 鏉ラ攢姣佹ā鎷熻姱鐗囥€?妯″潡浼氬悓姝ョ瓑寰呮柊鐨勬ā鎷熻澶囪鎴愬姛鎺㈡祴锛屽鏋滄湭鍙戠敓锛屽啓鍏?`'live'` 灏嗗鑷撮敊璇€?
妯℃嫙 GPIO 鑺墖涔熷彲浠ュ湪璁惧鏍戜腑瀹氫箟銆俢ompatible 瀛楃涓插繀椤讳负锛歚"gpio-simulator"`銆傛敮鎸佺殑
灞炴€ф湁锛?
  `"gpio-sim,label"` - 鑺墖鏍囩

鍏跺畠鏍囧噯 GPIO 灞炴€э紙濡?`"gpio-line-names"`銆乣"ngpios"` 鎴?`"gpio-hog"`锛夊悓鏍峰彈鏀寔銆?璇︽儏璇峰弬闃?GPIO 鏂囨。銆?
涓€涓畾涔?GPIO 妯℃嫙鍣ㄧ殑璁惧鏍戜唬鐮佺ず渚嬶細


    gpio-sim {
        compatible = "gpio-simulator";

        bank0 {
            gpio-controller;
            #gpio-cells = <2>;
            ngpios = <16>;
            gpio-sim,label = "dt-bank0";
            gpio-line-names = "", "sim-foo", "", "sim-bar";
        };

        bank1 {
            gpio-controller;
            #gpio-cells = <2>;
            ngpios = <8>;
            gpio-sim,label = "dt-bank1";

            line3 {
                gpio-hog;
                gpios = <3 0>;
                output-high;
                line-name = "sim-hog-from-dt";
            };
        };
    };

### 鎿嶄綔妯℃嫙绾胯矾


姣忎釜妯℃嫙 GPIO 鑺墖鍦ㄥ叾璁惧鐩綍涓嬩负姣忔潯鏆撮湶鐨勭嚎璺垱寤轰竴涓嫭绔嬬殑 sysfs 缁?锛堜緥濡?`/sys/devices/platform/gpio-sim.X/gpiochipY/`锛夈€傛瘡涓粍鐨勫悕绉颁负 `'sim_gpioX'` 褰㈠紡锛?鍏朵腑 X 鏄嚎璺殑鍋忕Щ銆傛瘡涓粍鍐呴儴鏈変袱涓睘鎬э細

    `pull` - 鍏佽璇诲彇鍜岃缃瘡鏉＄嚎璺殑褰撳墠妯℃嫙涓婃媺/涓嬫媺璁剧疆锛屽啓鍏ユ椂鍏跺€煎繀椤讳负浠ヤ笅涔嬩竴锛?               `'pull-up'`銆乣'pull-down'`

    `value` - 鍏佽璇诲彇绾胯矾鐨勫綋鍓嶅€硷紝濡傛灉璇ョ嚎璺琚敤鎴风┖闂撮┍鍔紝鍒欒鍊煎彲鑳戒笌涓婃媺/涓嬫媺璁剧疆涓嶅悓
