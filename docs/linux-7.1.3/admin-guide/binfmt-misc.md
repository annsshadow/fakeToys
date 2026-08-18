## 鍐呮牳瀵规潅椤逛簩杩涘埗鏍煎紡鐨勬敮鎸侊紙binfmt_misc锛?

璇ュ唴鏍哥壒鎬у厑璁镐綘鍑犱箮锛堥檺鍒惰涓嬫枃锛夊彧闇€鍦?shell 涓緭鍏ョ▼搴忓悕鍗冲彲璋冪敤浠绘剰绋嬪簭銆傝繖鍖呮嫭渚嬪缂栬瘧鍚庣殑 Java(TM)銆丳ython 鎴?Emacs 绋嬪簭銆?
涓烘锛屼綘蹇呴』鍛婅瘔 binfmt_misc 鍝釜瑙ｉ噴鍣ㄥ簲閰嶅悎鍝釜浜岃繘鍒舵枃浠惰璋冪敤銆侭infmt_misc 閫氳繃灏嗘枃浠跺紑澶寸殑鑻ュ共瀛楄妭涓庝綘鎻愪緵鐨勯瓟鏁板瓧鑺傚簭鍒楋紙灞忚斀鎺夋寚瀹氱殑浣嶏級杩涜鍖归厤鏉ヨ瘑鍒簩杩涘埗绫诲瀷銆侭infmt_misc 杩樺彲浠ヨ瘑鍒枃浠舵墿灞曞悕锛屼緥濡?`.com` 鎴?`.exe`銆?
```

	mount binfmt_misc -t binfmt_misc /proc/sys/fs/binfmt_misc

```
瑕佸疄闄呮敞鍐屼竴涓柊鐨勪簩杩涘埗绫诲瀷锛屼綘蹇呴』鏋勯€犱竴涓舰濡?`:name:type:offset:magic:mask:interpreter:flags` 鐨勫瓧绗︿覆锛堝叾涓殑 `:` 鍙互鏍规嵁闇€瑕侀€夋嫨锛夛紝骞跺皢鍏?echo 鍒?`/proc/sys/fs/binfmt_misc/register`銆?
浠ヤ笅涓哄悇瀛楁鐨勫惈涔夛細

- `name`
   鏄竴涓爣璇嗙瀛楃涓层€傚皢鍦?`/proc/sys/fs/binfmt_misc` 涓嬩互璇ュ悕绉板垱寤轰竴涓柊鐨?/proc 鏂囦欢锛涘嚭浜庢樉鑰屾槗瑙佺殑鍘熷洜锛屼笉鑳藉寘鍚枩鏉?`/`銆?- `type`
   鏄瘑鍒被鍨嬨€傞瓟鏁拌瘑鍒敤 `M`锛屾墿灞曞悕璇嗗埆鐢?`E`銆?- `offset`
   鏄枃浠朵腑 magic/mask 鐨勫亸绉婚噺锛屼互瀛楄妭璁°€傚鏋滅渷鐣ュ垯榛樿涓?0锛堝嵆浣犲啓鎴?`:name:type::magic...`锛夈€傚湪浣跨敤鏂囦欢鍚嶆墿灞曞悕鍖归厤鏃惰蹇界暐銆?- `magic`
   鏄?binfmt_misc 瑕佸尮閰嶇殑瀛楄妭搴忓垪銆傞瓟鏁板瓧绗︿覆鍙互鍖呭惈鍗佸叚杩涘埗缂栫爜鐨勫瓧绗︼紝濡?`\x0a` 鎴?`\xA4`銆傛敞鎰忎綘蹇呴』杞箟浠讳綍 NUL 瀛楄妭锛涜В鏋愬湪閬囧埌绗竴涓?NUL 鏃跺仠姝€傚湪 shell 鐜涓紝浣犲彲鑳藉繀椤诲啓鎴?`\\x0a` 浠ラ槻姝?shell 鍚冩帀浣犵殑 `\`銆傚鏋滈€夋嫨浜嗘枃浠跺悕鎵╁睍鍚嶅尮閰嶏紝鍒欐澶勪负瑕佽瘑鍒殑鎵╁睍鍚嶏紙涓嶅惈 `.`锛屼笉鍏佽浣跨敤 `\x0a` 鐗规畩褰㈠紡锛夈€傛墿灞曞悕鍖归厤鍖哄垎澶у皬鍐欙紝涓斾笉鍏佽鏂滄潬 `/`锛?- `mask`
   鏄竴涓紙鍙€夛紝榛樿涓哄叏 0xff锛夋帺鐮併€備綘鍙互鍍?magic 涓€鏍锋彁渚涗竴涓笌 magic 绛夐暱鐨勫瓧绗︿覆鏉ュ睆钄芥煇浜涘尮閰嶄綅銆傝鎺╃爜浼氫笌鏂囦欢鐨勫瓧鑺傚簭鍒楄繘琛屼笌杩愮畻銆傛敞鎰忎綘蹇呴』杞箟浠讳綍 NUL 瀛楄妭锛涜В鏋愬湪閬囧埌绗竴涓?NUL 鏃跺仠姝€傚湪浣跨敤鏂囦欢鍚嶆墿灞曞悕鍖归厤鏃惰蹇界暐銆?- `interpreter`
   鏄浠ヤ簩杩涘埗鏂囦欢浣滀负绗竴涓弬鏁版潵璋冪敤鐨勭▼搴忥紙璇锋寚瀹氬畬鏁磋矾寰勶級銆?- `flags`
   鏄竴涓彲閫夊瓧娈碉紝鎺у埗瑙ｉ噴鍣ㄨ皟鐢ㄧ殑鑻ュ共鏂归潰銆傚畠鏄竴涓ぇ鍐欏瓧姣嶅瓧绗︿覆锛屾瘡涓瓧姣嶆帶鍒朵竴涓柟闈€傛敮鎸佷互涓嬫爣蹇楋細

      `P` - 淇濈暀 argv[^0^]
            浼犵粺琛屼负鏄?binfmt_misc 浼氱敤浜岃繘鍒剁殑瀹屾暣璺緞瑕嗙洊鍘熷鐨?argv[^0^]銆傚寘鍚鏍囧織鏃讹紝binfmt_misc 浼氫负姝ゅ悜鍙傛暟鍚戦噺娣诲姞涓€涓弬鏁帮紝浠庤€屼繚鐣欏師濮嬬殑 `argv[^0^]`銆備緥濡傦紝濡傛灉浣犵殑 interp 璁句负 `/bin/foo` 涓斾綘杩愯 `blah`锛堜綅浜?`/usr/local/bin`锛夛紝鍒欏唴鏍稿皢浠?`argv[]` 璁句负 `["/bin/foo", "/usr/local/bin/blah", "blah"]` 鏉ユ墽琛?`/bin/foo`銆傝В閲婂櫒蹇呴』鎰忚瘑鍒拌繖涓€鐐癸紝鎵嶈兘浠?`argv[]` 璁句负 `["blah"]` 鏉ユ墽琛?`/usr/local/bin/blah`銆?      `O` - 鎵撳紑浜岃繘鍒讹紙open-binary锛?	    浼犵粺琛屼负鏄悜瑙ｉ噴鍣ㄤ紶閫掍簩杩涘埗鐨勫畬鏁磋矾寰勪綔涓哄弬鏁般€傚寘鍚鏍囧織鏃讹紝binfmt_misc 浼氭墦寮€璇ユ枃浠剁敤浜庤鍙栵紝骞跺皢鍏舵弿杩扮锛堣€岄潪瀹屾暣璺緞锛変綔涓哄弬鏁颁紶閫掞紝浠庤€屽厑璁歌В閲婂櫒鎵ц涓嶅彲璇荤殑浜岃繘鍒舵枃浠躲€傚簲璋ㄦ厧浣跨敤姝ょ壒鎬р€斺€斿繀椤讳俊浠昏В閲婂櫒涓嶄細娉勯湶涓嶅彲璇讳簩杩涘埗鏂囦欢鐨勫唴瀹广€?      `C` - 鍑瘉锛坈redentials锛?            褰撳墠锛宐infmt_misc 鐨勮涓烘槸鏍规嵁瑙ｉ噴鍣ㄦ潵璁＄畻鏂拌繘绋嬬殑鍑瘉鍜屽畨鍏ㄤ护鐗屻€傚寘鍚鏍囧織鏃讹紝杩欎簺灞炴€у皢鏍规嵁浜岃繘鍒舵枃浠惰绠椼€傚畠涔熼殣鍚簡 `O` 鏍囧織銆傚簲璋ㄦ厧浣跨敤姝ょ壒鎬э紝鍥犱负褰撲互 binfmt_misc 杩愯鐢?root 鎷ユ湁鐨?setuid 浜岃繘鍒舵枃浠舵椂锛岃В閲婂櫒灏嗕互 root 鏉冮檺杩愯銆?      `F` - 鍥哄畾浜岃繘鍒讹紙fix binary锛?            閫氬父 binfmt_misc 鐨勮涓烘槸鍦ㄨ皟鐢?misc 鏍煎紡鏂囦欢鏃舵墠鎯版€х敓鎴愶紙spawn锛変簩杩涘埗鏂囦欢銆傜劧鑰岋紝鍦ㄦ寕杞藉懡鍚嶇┖闂村拰 changeroots 闈㈠墠杩欑鏂瑰紡琛ㄧ幇涓嶄匠锛屽洜姝?`F` 妯″紡浼氬湪浠跨湡瀹夎瀹屾垚鍚庣珛鍗虫墦寮€浜岃繘鍒舵枃浠讹紝骞朵娇鐢ㄦ墦寮€鐨勬槧鍍忔潵鐢熸垚妯℃嫙鍣紝杩欐剰鍛崇潃涓€鏃﹀畨瑁呭氨濮嬬粓鍙敤锛屼笌鐜濡備綍鍙樺寲鏃犲叧銆?

鏈変竴浜涢檺鍒讹細

 - 鏁翠釜娉ㄥ唽瀛楃涓蹭笉寰楄秴杩?1920 涓瓧绗? - magic 蹇呴』浣嶄簬鏂囦欢鐨勫墠 128 瀛楄妭鍐咃紝鍗?offset+size(magic) 蹇呴』灏忎簬 128
 - 瑙ｉ噴鍣ㄥ瓧绗︿覆涓嶅緱瓒呰繃 127 涓瓧绗?
瑕佷娇鐢?binfmt_misc锛屼綘蹇呴』鍏堟寕杞藉畠銆備綘鍙互浣跨敤 `mount -t binfmt_misc none /proc/sys/fs/binfmt_misc` 鍛戒护鎸傝浇瀹冿紝鎴栬€呭悜浣犵殑 `/etc/fstab` 娣诲姞涓€琛?`none  /proc/sys/fs/binfmt_misc binfmt_misc defaults 0 0`锛屼娇鍏跺湪鍚姩鏃惰嚜鍔ㄦ寕杞姐€?
浣犲彲鑳藉笇鏈涘湪鍚姩鏈熼棿鍦ㄤ綘鐨勬煇涓?`/etc/rc` 鑴氭湰涓坊鍔犱簩杩涘埗鏍煎紡銆傝闃呰浣犵殑 init 绋嬪簭鐨勬墜鍐屼互浜嗚В姝ｇ‘鐨勫仛娉曘€?
娉ㄦ剰娣诲姞鏉＄洰鐨勯『搴忥紒鍚庢坊鍔犵殑鏉＄洰浼氬厛琚尮閰嶏紒


浠ヤ笅鏄竴浜涚ず渚嬶紙鍋囪浣犲湪 `/proc/sys/fs/binfmt_misc` 鐩綍涓嬶級锛?
```

    echo ':i386:M::\x7fELF\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x03:\xff\xff\xff\xff\xff\xfe\xfe\xff\xff\xff\xff\xff\xff\xff\xff\xff\xfb\xff\xff:/bin/em86:' > register
    echo ':i486:M::\x7fELF\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x06:\xff\xff\xff\xff\xff\xfe\xfe\xff\xff\xff\xff\xff\xff\xff\xff\xff\xfb\xff\xff:/bin/em86:' > register

```
```

    echo ':DEXE:M::\x0eDEX::/usr/bin/dosexec:' > register

```
```

    echo ':DOSWin:M::MZ::/usr/local/bin/wine:' > register

```
鏈夊叧 Java 鏀寔锛岃鍙傝 Documentation/admin-guide/java.rst


浣犲彲浠ラ€氳繃鍚?`/proc/sys/fs/binfmt_misc/status` 鎴?`/proc/.../the_name` echo 0锛堢鐢級鎴?1锛堝惎鐢級鏉ュ惎鐢?绂佺敤 binfmt_misc 鎴栨煇涓簩杩涘埗绫诲瀷銆傛煡鐪嬭鏂囦欢鐨勫唴瀹逛細鍛婅瘔浣?`binfmt_misc/the_entry` 鐨勫綋鍓嶇姸鎬併€?
浣犲彲浠ラ€氳繃鍚?`/proc/.../the_name` 鎴?`/proc/sys/fs/binfmt_misc/status` echo -1 鏉ュ垹闄や竴涓潯鐩垨鎵€鏈夋潯鐩€?

### 鎻愮ず


濡傛灉浣犳兂鍚戣В閲婂櫒浼犻€掔壒娈婂弬鏁帮紝鍙互涓哄畠缂栧啓涓€涓寘瑁呰剼鏈€?绀轰緥璇峰弬瑙?[Documentation/admin-guide/java.rst <./java>](Documentation/admin-guide/java.rst <./java>)銆?
浣犵殑瑙ｉ噴鍣ㄤ笉搴斿湪 PATH 涓煡鎵炬枃浠跺悕锛涘唴鏍镐細鍚戝畠浼犻€掕浣跨敤鐨勫畬鏁存枃浠跺悕锛堟垨鏂囦欢鎻忚堪绗︼級銆備娇鐢?`$PATH` 鍙兘瀵艰嚧鎰忓琛屼负锛屽苟鍙兘甯︽潵瀹夊叏闅愭偅銆?

Richard G眉nther <rguenth@tat.physik.uni-tuebingen.de>
