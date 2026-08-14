## Linux 涓婄殑 IBM 鏃ュ織鍨嬫枃浠剁郴缁燂紙JFS, Journaled File System锛?

JFS 涓婚〉锛? http://jfs.sourceforge.net/

鏀寔浠ヤ笅鎸傝浇閫夐」锛?
(*) == 榛樿鍊?
iocharset=name
                鐢ㄤ簬灏?Unicode 杞崲涓?ASCII 鐨勫瓧绗﹂泦銆傞粯璁や笉杩涜杞崲銆備娇鐢?                iocharset=utf8 杩涜 UTF-8 杞崲銆傝繖闇€瑕佸湪鍐呮牳 .config 鏂囦欢涓?                璁剧疆 CONFIG_NLS_UTF8銆俰ocharset=none 鏄惧紡鎸囧畾榛樿琛屼负銆?
resize=value
                灏嗗嵎澶у皬璋冩暣涓?<value> 涓潡銆侸FS 浠呮敮鎸佹墿澶у嵎锛岃€屼笉鏀寔缂╁皬
                瀹冦€傝閫夐」浠呭湪浠ヨ鍐欐柟寮忔寕杞藉嵎鐨勯噸鏂版寕杞斤紙remount锛夋湡闂存湁鏁堛€?                涓嶅甫鍊肩殑 resize 鍏抽敭瀛椾細灏嗗嵎鎵╁ぇ鍒板垎鍖虹殑瀹屾暣澶у皬銆?
nointegrity
                涓嶅啓鍏ユ棩蹇椼€傝閫夐」鐨勪富瑕佺敤閫旀槸鍦ㄤ粠澶囦唤浠嬭川鎭㈠鍗锋椂鑾峰緱鏇撮珮
                鐨勬€ц兘銆傚鏋滅郴缁熷紓甯哥粓姝紝鍗风殑瀹屾暣鎬ф棤娉曞緱鍒颁繚璇併€?
integrity(*)
                灏嗗厓鏁版嵁鍙樻洿鎻愪氦鍒版棩蹇椼€備娇鐢ㄦ閫夐」鍙互閲嶆柊鎸傝浇姝ゅ墠鎸囧畾浜?                nointegrity 閫夐」鐨勫嵎锛屼互鎭㈠姝ｅ父琛屼负銆?
errors=continue
                        鏂囦欢绯荤粺鍑洪敊鏃剁户缁繍琛屻€?errors=remount-ro(*)
                        鍑洪敊鏃跺皢鏂囦欢绯荤粺浠ュ彧璇绘柟寮忛噸鏂版寕杞姐€?errors=panic
                        濡傛灉鍙戠敓閿欒锛岃Е鍙?panic 骞跺仠鏈恒€?
uid=value
                鐢ㄦ寚瀹氱殑鍊艰鐩栫鐩樹笂鐨?uid銆?gid=value
                鐢ㄦ寚瀹氱殑鍊艰鐩栫鐩樹笂鐨?gid銆?umask=value
                鐢ㄦ寚瀹氱殑鍏繘鍒跺€艰鐩栫鐩樹笂鐨?umask銆傚浜庣洰褰曪紝濡傛灉鐩稿簲鐨?                璇讳綅琚缃紝鎵ц浣嶄篃浼氳璁剧疆銆?
discard=minlen, discard/nodiscard(*)
                鍚敤/绂佺敤 discard/TRIM 鍛戒护鐨勪娇鐢ㄣ€傚綋鍧楄閲婃斁鏃讹紝discard/TRIM
                鍛戒护浼氳鍙戦€佺粰搴曞眰鍧楄澶囥€傝繖瀵?SSD 璁惧浠ュ強绋€鐤?绮剧畝閰嶇疆鐨?LUN
                寰堟湁鐢ㄣ€侳ITRIM ioctl 鍛戒护涔熷彲涓?nodiscard 閫夐」涓€璧蜂娇鐢ㄣ€俶inlen
                鐨勫€兼寚瀹氭渶灏忓潡鏁帮紝褰撹揪鍒拌鍊兼椂锛屽悜鍧楄澶囧彂閫?TRIM 鍛戒护鎵嶈璁や负
                鏈夌敤銆傚鏋滄病鏈変负 discard 閫夐」鎻愪緵鍊硷紝瀹冮粯璁や负 64 涓潡锛屽湪 JFS 涓?                鍗?256KiB銆俤iscard 鐨?minlen 鍊间細瑕嗙洊 FITRIM ioctl() 缁欏嚭鐨?minlen
                鍊笺€?
鍙互閫氳繃鎴戜滑缃戦〉 http://jfs.sourceforge.net/ 涓婃爣璁颁负鈥淢ail list Subscribe鈥濈殑閾炬帴
鏉ヨ闃?JFS 閭欢鍒楄〃銆?