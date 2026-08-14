


######## Introduction


LIRC 浠ｈ〃 Linux Infrared Remote Control锛圠inux 绾㈠閬ユ帶锛夈€侺IRC 璁惧鎺ュ彛鏄竴涓弻鍚戞帴鍙ｏ紝鐢ㄤ簬鍦ㄧ敤鎴风┖闂翠笌鍐呮牳绌洪棿涔嬮棿浼犺緭鍘熷 IR 鍜岃В鐮佸悗鐨勬壂鎻忕爜鏁版嵁銆備粠鏍规湰涓婅锛屽畠鍙槸涓€涓瓧绗﹁澶囷紙/dev/lircX锛屽叾涓?X = 0, 1, 2, ...锛夛紝鍦ㄥ叾涓婂畾涔変簡涓€浜涙爣鍑嗙殑 struct file_operations銆傚氨鏉ュ洖浼犺緭鍘熷 IR 鍜岃В鐮佸悗鐨勬壂鎻忕爜鑰岃█锛屽叧閿殑 fops 鏄?read銆亀rite 鍜?ioctl銆?
涔熷彲浠ュ悜 LIRC 璁惧闄勫姞涓€涓?BPF 绋嬪簭锛屽皢鍘熷 IR 瑙ｇ爜涓烘壂鎻忕爜銆?
椹卞姩娉ㄥ唽甯︽湁 LIRC 鏃?dmesg 杈撳嚭绀轰緥锛?

    $ dmesg |grep lirc_dev
    rc rc0: lirc_dev: driver mceusb registered at minor = 0, raw IR receiver, raw IR transmitter


浣犲簲璇ョ湅鍒扮殑瀛楃璁惧锛?

    $ ls -l /dev/lirc*
    crw-rw---- 1 root root 248, 0 Jul 2 22:20 /dev/lirc0


娉ㄦ剰 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_ 杞欢鍖呭寘鍚敤浜庡鐞?LIRC 璁惧鐨勫伐鍏凤細

 - ir-ctl: 鍙互鎺ユ敹鍘熷 IR 骞跺彂閫?IR锛屼互鍙婃煡璇?LIRC 璁惧鐗规€с€?
 - ir-keytable: 鍙互鍔犺浇閿槧灏勶紱鍏佽浣犺缃?IR 鍐呮牳鍗忚锛涘姞杞?BPF IR 瑙ｇ爜鍣ㄥ苟娴嬭瘯 IR 瑙ｇ爜銆備篃鎻愪緵浜嗕竴浜?BPF IR 瑙ｇ爜鍣ㄣ€?

######## LIRC modes


LIRC 鏀寔鍑犵鎺ユ敹鍜屽彂閫?IR 鐮佺殑妯″紡锛屽涓嬭〃鎵€绀恒€?

`LIRC_MODE_SCANCODE`

    璇ユā寮忕敤浜庡彂閫佸拰鎺ユ敹 IR銆?
    瀵逛簬鍙戦€侊紙鍗?transmitting锛夛紝鍒涘缓涓€涓?struct lirc_scancode锛屽湪 `scancode` 鎴愬憳涓缃湡鏈涚殑鎵弿鐮侊紝`rc_proto` 璁剧疆涓?IR 鍗忚 <Remote_controllers_Protocols>锛屽叾浠栨墍鏈夋垚鍛樿涓?0銆傛妸杩欎釜缁撴瀯浣撳啓鍏?lirc 璁惧銆?
    瀵逛簬鎺ユ敹锛屼綘浠?LIRC 璁惧璇诲彇 struct lirc_scancode銆俙scancode` 瀛楁琚涓烘帴鏀跺埌鐨勬壂鎻忕爜锛孖R 鍗忚 <Remote_controllers_Protocols> 琚鍦?`rc_proto` 涓€傚鏋滄壂鎻忕爜鏄犲皠鍒颁竴涓湁鏁堢殑閿爜锛屽垯瀹冧細琚鍦?`keycode` 瀛楁涓紝鍚﹀垯璁句负 `KEY_RESERVED`銆?
    `flags` 鍙互鍦ㄦ敮鎸?toggle 浣嶇殑鍗忚涓缃?`LIRC_SCANCODE_FLAG_TOGGLE`锛堜緥濡?rc-5 鍜?rc-6锛夛紝鎴栬€呭湪鏀寔 repeat 鐨勫崗璁腑鏀跺埌 repeat 鏃惰缃?`LIRC_SCANCODE_FLAG_REPEAT`锛堜緥濡?nec锛夈€?
    鍦?Sanyo 鍜?NEC 鍗忚涓紝濡傛灉浣犳寜浣忛仴鎺у櫒涓婄殑鎸夐挳锛岄仴鎺у櫒涓嶆槸閲嶅鏁翠釜鎵弿鐮侊紝鑰屾槸鍙戦€佷竴鏉′笉鍚壂鎻忕爜鐨勬洿鐭秷鎭紝浠呰〃绀烘寜閽鎸変綇锛屽嵆鈥渞epeat鈥濄€傚綋鏀跺埌杩欎釜鏃讹紝`LIRC_SCANCODE_FLAG_REPEAT` 琚缃紝骞朵笖鎵弿鐮佸拰閿爜琚噸澶嶃€?
    瀵逛簬 nec锛屾棤娉曞尯鍒嗏€滄寜浣忔寜閽€濅笌鈥滃弽澶嶆寜鍚屼竴涓寜閽€濄€俽c-5 鍜?rc-6 鍗忚鏈変竴涓?toggle 浣嶃€傚綋鎸夐挳琚噴鏀惧苟鍐嶆鎸変笅鏃讹紝toggle 浣嶈鍙栧弽銆傚鏋滆缃簡 toggle 浣嶏紝鍒?`LIRC_SCANCODE_FLAG_TOGGLE` 琚缃€?
    `timestamp` 瀛楁琚～鍏ユ壂鎻忕爜琚В鐮佹椂鐨勬椂闂达紝鍗曚綅涓虹撼绉掞紙鍦?`CLOCK_MONOTONIC` 涓嬶級銆?

`LIRC_MODE_MODE2`

    椹卞姩鍚戠敤鎴风┖闂磋繑鍥炰竴涓茶剦鍐诧紙pulse锛夊拰闂撮殧锛坰pace锛夌爜锛屽舰寮忎负涓€绯诲垪 u32 鍊笺€?
    璇ユā寮忎粎鐢ㄤ簬 IR 鎺ユ敹銆?
    楂?8 浣嶅喅瀹氬寘绫诲瀷锛屼綆 24 浣嶄负璐熻浇銆備娇鐢?`LIRC_VALUE()` 瀹忚幏鍙栬礋杞斤紝`LIRC_MODE2()` 瀹忕粰鍑虹被鍨嬶紝绫诲瀷涔嬩竴鏄細

    `LIRC_MODE2_PULSE`

        琛ㄧず瀛樺湪 IR锛屽崟浣嶄负寰锛屼篃绉颁负 **flash**銆?
    `LIRC_MODE2_SPACE`

        琛ㄧず涓嶅瓨鍦?IR锛屽崟浣嶄负寰锛屼篃绉颁负 **gap**銆?
    `LIRC_MODE2_FREQUENCY`

        濡傛灉宸蹭娇鐢?lirc_set_measure_carrier_mode 鍚敤杞芥尝棰戠巼娴嬮噺锛屽垯璇ュ寘缁欏嚭浠ヨ但鍏逛负鍗曚綅鐨勮浇娉㈤鐜囥€?
    `LIRC_MODE2_TIMEOUT`

        褰撲娇鐢?lirc_set_rec_timeout 璁剧疆鐨勮秴鏃剁敱浜庢湭妫€娴嬪埌 IR 鑰屽埌鏈熸椂锛屼細鍙戦€佽鍖咃紝鍖呬腑鏄病鏈?IR 鐨勫井绉掓暟銆?
    `LIRC_MODE2_OVERFLOW`

        琛ㄧず IR 鎺ユ敹鍣ㄩ亣鍒颁簡婧㈠嚭锛岄儴鍒?IR 缂哄け銆傛鍚庣殑 IR 鏁版嵁搴斿綋鍐嶆姝ｇ‘銆傚疄闄呭€间笉閲嶈锛屼絾涓轰笌 lircd 鍏煎锛屽唴鏍稿皢鍏惰涓?0xffffff銆?

`LIRC_MODE_PULSE`

    鍦ㄨ剦鍐叉ā寮忎笅锛屼娇鐢?lirc-write 鎶婁竴涓茶剦鍐?闂撮殧鏁存暟鍊煎啓鍏?lirc 璁惧銆?
    杩欎簺鍊兼槸浜ゆ浛鐨勮剦鍐插拰闂撮殧闀垮害锛屽崟浣嶄负寰銆傜涓€涓拰鏈€鍚庝竴涓潯鐩繀椤绘槸鑴夊啿锛屽洜姝ゆ潯鐩暟蹇呴』涓哄鏁般€?
    璇ユā寮忎粎鐢ㄤ簬 IR 鍙戦€併€?
######## Data types used by LIRC_MODE_SCANCODE


    :identifiers: lirc_scancode rc_proto

######## BPF based IR decoder


鍐呮牳鏀寔瑙ｇ爜鏈€甯歌鐨?IR 鍗忚 <Remote_controllers_Protocols>锛屼絾杩樻湁璁稿鍗忚涓嶅彈鏀寔銆備负浜嗘敮鎸佽繖浜涘崗璁紝鍙互鍔犺浇涓€涓墽琛岃В鐮佺殑 BPF 绋嬪簭銆傝繖鍙兘鍦ㄦ敮鎸佽鍙栧師濮?IR 鐨?LIRC 璁惧涓婂畬鎴愩€?
棣栧厛锛屼娇鐢ㄥ甫鏈?`BPF_LOAD_PROG` 鍙傛暟鐨?`bpf(2)`_ 绯荤粺璋冪敤锛屽繀椤诲姞杞界被鍨嬩负 `BPF_PROG_TYPE_LIRC_MODE2` 鐨勭▼搴忋€備竴鏃﹂檮鍔犲埌 LIRC 璁惧锛岃绋嬪簭灏嗗湪 LIRC 璁惧涓婄殑姣忎釜鑴夊啿銆侀棿闅旀垨瓒呮椂浜嬩欢鏃惰璋冪敤銆侭PF 绋嬪簭鐨勪笂涓嬫枃鏄竴涓寚鍚?unsigned int 鐨勬寚閽堬紝鍗充竴涓?LIRC_MODE_MODE2 <lirc-mode-mode2> 鍊笺€傚綋绋嬪簭瑙ｇ爜鍑烘壂鎻忕爜鍚庯紝鍙互浣跨敤 BPF 鍑芥暟 `bpf_rc_keydown()` 鎴?`bpf_rc_repeat()` 鎻愪氦銆傞紶鏍囨垨鎸囬拡绉诲姩鍙互浣跨敤 `bpf_rc_pointer_rel()` 鎶ュ憡銆?
涓€鏃︿綘鏈変簡 `BPF_PROG_TYPE_LIRC_MODE2` BPF 绋嬪簭鐨勬枃浠舵弿杩扮锛屽氨鍙互浣跨敤 `bpf(2)`_ 绯荤粺璋冪敤灏嗗叾闄勫姞鍒?LIRC 璁惧銆傜洰鏍囧繀椤绘槸 LIRC 璁惧鐨勬枃浠舵弿杩扮锛岄檮鍔犵被鍨嬪繀椤绘槸 `BPF_LIRC_MODE2`銆備竴涓?LIRC 璁惧涓婁竴娆℃渶澶氬彲闄勫姞 64 涓?BPF 绋嬪簭銆?