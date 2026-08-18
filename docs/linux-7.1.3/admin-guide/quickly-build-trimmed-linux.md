
## 濡備綍蹇€熸瀯寤轰竴涓簿绠€鐨?Linux 鍐呮牳


鏈寚鍗楄瑙ｅ浣曞揩閫熸瀯寤洪潪甯搁€傚悎娴嬭瘯銆佸悓鏃朵篃瀹屽叏鍙互鐢ㄤ簬鏃ュ父浣跨敤鐨?Linux 鍐呮牳銆?
## 杩囩▼绮鹃珦锛堝嵆"澶暱涓嶇湅"鐗堬級


*[濡傛灉浣犲垰鎺ヨЕ缂栬瘧 Linux锛岃蹇界暐杩欐 TL;DR锛岀洿鎺ヨ烦鍒颁笅闈㈢殑涓€鑺傦細閭ｉ噷鏈変竴浠介€愭鎸囧崡锛?瀹冩洿璇︾粏锛屼絾浠嶇劧绠€娲併€佹槗浜庤窡闅忥紱璇ユ寚鍗楀強鍏舵墍闄勭殑鍙傝€冧竴鑺傝繕鎻愬埌浜嗗悇绉嶆浛浠ｆ柟妗堛€侀櫡闃卞拰
琛ュ厖鏂归潰锛岃繖浜涢兘鍙兘涓庝綘鐩稿叧銆俔*

濡傛灉浣犵殑绯荤粺浣跨敤浜?Secure Boot 涔嬬被鐨勬妧鏈紝璇峰厛鍑嗗濂藉厑璁稿惎鍔ㄨ嚜宸辩紪璇戠殑 Linux 鍐呮牳锛?瀹夎缂栬瘧鍣ㄤ互鍙婃瀯寤?Linux 鎵€闇€鐨勫叾浠栦竴鍒囷紱纭繚鍦ㄤ綘鐨?home 鐩綍涓嬫湁 12 GB 鐨勭┖闂茬┖闂淬€?鐜板湪杩愯浠ヤ笅鍛戒护浠ヤ笅杞芥渶鏂扮殑 Linux 涓荤嚎婧愪唬鐮侊細

```
    git clone --depth 1 -b master \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git ~/linux/
    cd ~/linux/
    # 鎻愮ず锛氬鏋滀綘鎯虫墦琛ヤ竵锛岃鍦ㄦ澶勮繘琛屻€傝瑙佷笅鏂囥€?    # 鎻愮ず锛氬缓璁湪姝ゅ涓轰綘鐨勬瀯寤烘墦涓婃爣绛俱€傝瑙佷笅鏂囥€?    yes "" | make localmodconfig
    # 鎻愮ず锛氭鏃朵綘鍙兘闇€瑕佽皟鏁存瀯寤洪厤缃紱濡傛灉浣犺繍琛岀殑鏄?Debian锛屽氨蹇呴』瑕佽皟銆傝瑙佷笅鏂囥€?    make -j $(nproc --all)
    # 娉ㄦ剰锛氬湪璁稿甯歌鍙戣鐗堜笂涓嬩竴鏉″懡浠ゅ氨瓒冲浜嗭紝浣嗗湪 Arch Linux 鍙婂叾琛嶇敓鐗堝拰鏌愪簺
    #   鍏朵粬鍙戣鐗堜笂骞堕潪濡傛銆傝瑙佷笅鏂囥€?    command -v installkernel && sudo make modules_install install
    reboot
```

瑕佷负浠ュ悗鐨勬瀯寤烘洿鏂颁唬鐮侊紝璇蜂娇鐢ㄨ繖浜涘懡浠わ細

```
    cd ~/linux/
    git fetch --depth 1 origin
    # 娉ㄦ剰锛氫笅涓€鏉″懡浠や細涓㈠純浣犲浠ｇ爜鎵€鍋氱殑浠讳綍淇敼锛?    git checkout --force --detach origin/master
    # 鎻愰啋锛氬鏋滀綘鎯筹紙閲嶆柊锛夋墦琛ヤ竵锛岃鍦ㄦ澶勮繘琛屻€?    # 鎻愰啋锛氫綘鍙兘鎯冲湪姝ゅ娣诲姞鎴栦慨鏀逛竴涓瀯寤烘爣绛俱€?    make olddefconfig
    make -j $(nproc --all)
    # 鎻愰啋锛氫笅涓€鏉″懡浠ゅ湪鏌愪簺鍙戣鐗堜笂骞朵笉瓒冲銆?    command -v installkernel && sudo make modules_install install
    reboot
```

## 閫愭鎸囧崡


鑷繁缂栬瘧 Linux 鍐呮牳鍘熷垯涓婂緢绠€鍗曘€傛湁鍚勭涓嶅悓鐨勬柟寮忔潵鍋氳繖浠朵簨銆傚叾涓摢浜涚湡姝ｅ彲琛屻€佸摢涓?鏈€濂斤紝鍙栧喅浜庡叿浣撶幆澧冦€?
鏈寚鍗楁弿杩扮殑鏂规硶闈炲父閫傚悎閭ｄ簺鎯充粠婧愪唬鐮佸揩閫熷畨瑁?Linux銆佽€屼笉鎯宠澶嶆潅缁嗚妭鍥版壈鐨勪汉锛涘叾鐩爣
鏄鐩栧湪鍟嗗搧 PC 鎴栨湇鍔″櫒纭欢涓婅繍琛岀殑涓绘祦 Linux 鍙戣鐗堜笂閫氬父鎵€闇€鐨勪竴鍒囥€?
鎵€鎻忚堪鐨勬柟娉曢潪甯搁€傚悎娴嬭瘯鐩殑锛屼緥濡傚皾璇曚竴涓彁璁殑淇锛屾垨妫€鏌ユ煇涓棶棰樺湪鏈€鏂扮殑浠ｇ爜搴撲腑鏄?鍚﹀凡缁忚淇銆傚敖绠″姝わ紝鐢ㄨ繖绉嶆柟寮忔瀯寤虹殑鍐呮牳涔熷畬鍏ㄥ彲浠ョ敤浜庢棩甯镐娇鐢紝鍚屾椂鍙堟槗浜庝繚鎸佹洿鏂般€?
浠ヤ笅姝ラ鎻忚堪浜嗚杩囩▼鐨勯噸瑕佹柟闈紱鍚庨潰涓€涓叏闈㈢殑鍙傝€冧竴鑺備細鏇磋缁嗗湴瑙ｉ噴鍏朵腑姣忎竴椤广€傚畠鏈夋椂
涔熸弿杩颁簡鏇夸唬鏂规銆侀櫡闃憋紝浠ュ強鍙兘鍦ㄦ煇涓壒瀹氱偣鍙戠敓鐨勯敊璇€斺€斾互鍙婂浣曡浜嬫儏閲嶆柊杩愯浆璧锋潵銆?
..
   Note: if you see this note, you are reading the text's source file. You
   might want to switch to a rendered version, as it makes it a lot easier to
   quickly look something up in the reference section and afterwards jump back
   to where you left off. Find a the latest rendered version here:
   https://docs.kernel.org/admin-guide/quickly-build-trimmed-linux.html


 - 鍒涘缓涓€涓叏鏂扮殑澶囦唤锛屽苟鎶婄郴缁熶慨澶嶅拰鎭㈠宸ュ叿鏀惧湪鎵嬭竟锛屼互闃蹭竾涓€鍑虹幇鎰忓鎯呭喌銆?
   [details<backup>]


 - 鍦ㄤ娇鐢?'Secure Boot' 鎴栫被浼兼妧鏈殑骞冲彴涓婏紝鍑嗗濂戒竴鍒囷紝纭繚绯荤粺浠ュ悗浼氬厑璁镐綘鑷紪璇戠殑
   鍐呮牳鍚姩銆傚湪鍟嗗搧 x86 绯荤粺涓婂疄鐜拌繖涓€鐐规渶蹇渶绠€鍗曠殑鏂规硶鏄湪 BIOS 璁剧疆宸ュ叿涓鐢ㄦ绫绘妧鏈紱
   鎴栬€呴€氳繃鐢?`mokutil --disable-validation` 鍙戣捣鐨勬祦绋嬫潵绉婚櫎瀹冧滑鐨勯檺鍒躲€?
   [details<secureboot>]


 - 瀹夎鏋勫缓 Linux 鍐呮牳鎵€闇€鐨勬墍鏈夎蒋浠躲€傞€氬父浣犻渶瑕侊細'bc'銆?binutils'锛?ld' 绛夛級銆?bison'銆?   'flex'銆?gcc'銆?git'銆?openssl'銆?pahole'銆?perl'锛屼互鍙?'libelf' 鍜?'openssl' 鐨勫紑鍙?   澶存枃浠躲€傚弬鑰冧竴鑺傚睍绀轰簡濡備綍鍦ㄥ悇绉嶆祦琛岀殑 Linux 鍙戣鐗堜笂蹇€熷畨瑁呭畠浠€?
   [details<buildrequires>]


 - 纭繚鏈夎冻澶熺殑绌洪棽绌洪棿鐢ㄤ簬鏋勫缓鍜屽畨瑁?Linux銆傚浜庡悗鑰咃紝/lib/ 涓?150 MB 鍜?/boot/ 涓?100 MB
   鏄竴涓ǔ濡ョ殑浼拌銆傚浜庡瓨鏀炬簮浠ｇ爜鍜屾瀯寤轰骇鐗╋紝浣犵殑 home 鐩綍涓?12 GB 閫氬父瓒冲銆傚鏋滀綘鍙敤
   绌洪棿鏇村皯锛屽姟蹇呮煡闃呭弬鑰冧竴鑺備腑鍏充簬璋冩暣鍐呮牳鏋勫缓閰嶇疆鐨勯偅涓€姝ワ細瀹冩彁鍒颁竴涓兘鎶?/home/ 涓嬫墍闇€
   绌洪棿鍑忓皯鍒扮害 4 GB 鐨勬妧宸с€?
   [details<diskspace>]


 - 鑾峰彇浣犳兂瑕佹瀯寤虹殑 Linux 鐗堟湰鐨勬簮浠ｇ爜锛涚劧鍚庡垏鎹㈠埌淇濆瓨瀹冧滑鐨勭洰褰曪紝鍥犱负鏈寚鍗椾腑鎵€鏈夊悗缁懡浠?   閮芥墦绠椾粠璇ョ洰褰曟墽琛屻€?
   *[Note: the following paragraphs describe how to retrieve the sources by
   partially cloning the Linux stable git repository. This is called a shallow
   clone. The reference section explains two alternatives:* :ref:`packaged
   archives<sources_archive>` **and** a full git clone<sources_full> *;
   prefer the latter, if downloading a lot of data does not bother you, as that
   will avoid some* :ref:`peculiar characteristics of shallow clones the
   reference section explains<sources_shallow>` **.]**

```
     git clone --no-checkout --depth 1 -b master \
       https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git ~/linux/
     cd ~/linux/

   If you want to access recent mainline releases and pre-releases, deepen you
   clone's history to the oldest mainline version you are interested in::

     git fetch --shallow-exclude=v6.0 origin

   In case you want to access a stable/longterm release (say v6.1.5), simply add
   the branch holding that series; afterwards fetch the history at least up to
   the mainline version that started the series (v6.1)::

     git remote set-branches --add origin linux-6.1.y
     git fetch --shallow-exclude=v6.0 origin

   Now checkout the code you are interested in. If you just performed the
   initial clone, you will be able to check out a fresh mainline codebase, which
   is ideal for checking whether developers already fixed an issue::

      git checkout --detach origin/master

   If you deepened your clone, you instead of ``origin/master`` can specify the
   version you deepened to (``v6.0`` above); later releases like ``v6.1`` and
   pre-release like ``v6.2-rc1`` will work, too. Stable or longterm versions
   like ``v6.1.5`` work just the same, if you added the appropriate
   stable/longterm branch as described.

   [:ref:`details<sources>`]
```

 - 濡傛灉浣犳兂搴旂敤涓€涓唴鏍歌ˉ涓侊紝鐜板湪灏卞仛銆傞€氬父涓€鏉¤繖鏍风殑鍛戒护锛?
```
     patch -p1 < ../proposed-fix.patch

   If the ``-p1`` is actually needed, depends on how the patch was created; in
   case it does not apply thus try without it.

   If you cloned the sources with git and anything goes sideways, run ``git
   reset --hard`` to undo any changes to the sources.

   [:ref:`details<patching>`]
```

 - 濡傛灉浣犱负鍐呮牳鎵撲簡琛ヤ竵锛屾垨鑰呭凡缁忓畨瑁呬簡涓€涓浉鍚岀増鏈殑鍐呮牳锛?
```
     echo "-proposed_fix" > localversion

   Running ``uname -r`` under your kernel later will then print something like
   '6.1-rc4-proposed_fix'.

   [:ref:`details<tagging>`]
```

.. _configuration_sbs:

* 鍩轰簬鐜版湁閰嶇疆涓轰綘鐨勫唴鏍稿垱寤烘瀯寤洪厤缃€?
  濡傛灉浣犺嚜宸卞凡缁忓噯澶囧ソ浜嗚繖鏍蜂竴涓?'.config' 鏂囦欢锛屾妸瀹冨鍒跺埌 ~/linux/ 骞惰繍琛?  ``make olddefconfig``銆?
  濡傛灉浣犵殑鍙戣鐗堟垨鍒汉宸茬粡鎶婃鍦ㄨ繍琛岀殑鍐呮牳閽堝浣犳垨浣犵殑纭欢闇€姹傝鍓繃锛氶偅涔?make 鐩爣
  'olddefconfig' 浼氬皾璇曚互璇ュ唴鏍哥殑 .config 浣滀负鍩虹銆?
  杩欎釜 make 鐩爣瀵瑰叾浠栦汉涔熼€傜敤鈥斺€斾絾浣犵粡甯稿彲浠ラ€氳繃鏀圭敤杩欐潯鍛戒护鏉ヨ妭鐪佸ぇ閲忔椂闂达細

```
     yes "" | make localmodconfig
```

  瀹冧細灏濊瘯浠ヤ綘鐨勫彂琛岀増鍐呮牳涓哄熀纭€锛屼絾闅忓悗浼氫负浣犻厤缃腑鏄庢樉澶氫綑鐨勫姛鑳界鐢ㄦā鍧椼€傝繖灏嗘瀬澶у湴
  鍑忓皯缂栬瘧鏃堕棿锛屽挨鍏舵槸褰撲綘杩愯鐨勬槸涓€涓潵鑷晢鍝?Linux 鍙戣鐗堢殑閫氱敤鍐呮牳鏃躲€?
  杩欓噷鏈変竴涓潙锛?localmodconfig' 寰堝彲鑳界鐢ㄤ綘鑷惎鍔?Linux 浠ユ潵娌℃湁浣跨敤杩囩殑鍐呮牳鍔熻兘鈥斺€旀瘮濡?  褰撳墠鏈繛鎺ョ殑澶栭儴璁惧鐨勯┍鍔紝鎴栦綘灏氭湭浣跨敤杩囩殑铏氭嫙鍖栬蒋浠躲€備綘鍙互鐢ㄥ弬鑰冧竴鑺傛杩扮殑鎶€宸ф潵鍑忓皯
  鐢氳嚦鍑犱箮娑堥櫎杩欑椋庨櫓锛涗絾鍦ㄤ粎涓哄揩閫熸祴璇曠洰鐨勬瀯寤哄唴鏍告椂锛岃繖浜涘姛鑳界己澶遍€氬父鏃犱激澶ч泤銆備笉杩囧湪浣跨敤
  鐢ㄨ繖涓?make 鐩爣鏋勫缓鐨勫唴鏍告椂锛屼綘搴斿綋鎶婅繖涓€鐐硅鍦ㄥ績閲岋紝鍥犱负瀹冨彲鑳芥槸浣犲伓灏旀墠鐢ㄧ殑涓滆タ鍋滄
  宸ヤ綔鍘熷洜銆?
   [:ref:`details<configuration>`]

```
 - 妫€鏌ヤ綘鏄惁鎯虫垨蹇呴』璋冩暣涓€浜涘唴鏍搁厤缃€夐」锛?
  - 鑰冭檻濡備綍澶勭悊璋冭瘯绗﹀彿銆傚鏋滀綘浠ュ悗鍙兘闇€瑕佽В鐮佷竴涓緥濡傚湪 'panic'銆?Oops'銆?warning' 鎴?    'BUG' 涓壘鍒扮殑鍫嗘爤璺熻釜锛屽氨鍚敤瀹冧滑锛涘弽涔嬶紝濡傛灉浣犲瓨鍌ㄧ┖闂寸揣寮犳垨鏇村枩娆㈡洿灏忕殑鍐呮牳浜岃繘鍒舵枃浠讹紝
    灏辩鐢ㄥ畠浠€傚叧浜庡浣曞仛杩欎袱鑰呯殑缁嗚妭锛岃鍙傞槄鍙傝€冧竴鑺傘€傚鏋滀袱鑰呴兘涓嶉€傜敤锛岀畝鍗曞湴涓嶅幓绠″畠澶氬崐
    涔熸棤濡ㄣ€俒details<configmods_debugsymbols>]

  - 浣犺繍琛岀殑鏄?Debian 鍚楋紵閭ｄ箞璇锋墽琛屽弬鑰冧竴鑺備腑瑙ｉ噴鐨勯澶栬皟鏁达紝浠ラ伩鍏嶅凡鐭ラ棶棰樸€?    [details<configmods_distros>]銆?
  - 濡傛灉浣犳兂褰卞搷閰嶇疆鐨勫叾浠栨柟闈紝鐜板湪灏遍€氳繃 'menuconfig' 鎴?'xconfig' 涔嬬被鐨?make 鐩爣鏉ュ仛銆?    [details<configmods_individual>]銆?```

 - 缂栬瘧鍐呮牳锛?
```
     make -j $(nproc --all)

   If you want your kernel packaged up as deb, rpm, or tar file, see the
   reference section for alternatives.

   [:ref:`details<build>`]
```

 - 瀹夎鍐呮牳锛?
```
     command -v installkernel && sudo make modules_install install

   Often all left for you to do afterwards is a ``reboot``, as many commodity
   Linux distributions will then create an initramfs (also known as initrd) and
   an entry for your kernel in your bootloader's configuration; but on some
   distributions you have to take care of these two steps manually for reasons
   the reference section explains.

   On a few distributions like Arch Linux and its derivatives the above command
   does nothing at all; in that case you have to manually install your kernel,
   as outlined in the reference section.

   If you are running an immutable Linux distribution, check its documentation
   and the web to find out how to install your own kernel there.

   [:ref:`details<install>`]
```

 - 浠ュ悗瑕佹瀯寤哄彟涓€涓唴鏍革紝浣犻渶瑕佺被浼肩殑姝ラ锛屼絾鏈夋椂鍛戒护鐣ユ湁涓嶅悓銆?
```
      cd ~/linux/

   In case you want to build a version from a stable/longterm series you have
   not used yet (say 6.2.y), tell git to track it::

      git remote set-branches --add origin linux-6.2.y

   Now fetch the latest upstream changes; you again need to specify the earliest
   version you care about, as git otherwise might retrieve the entire commit
   history::

     git fetch --shallow-exclude=v6.0 origin

   Now switch to the version you are interested in -- but be aware the command
   used here will discard any modifications you performed, as they would
   conflict with the sources you want to checkout::

     git checkout --force --detach origin/master

   At this point you might want to patch the sources again or set/modify a build
   tag, as explained earlier. Afterwards adjust the build configuration to the
   new codebase using olddefconfig, which will now adjust the configuration file
   you prepared earlier using localmodconfig  (~/linux/.config) for your next
   kernel::

     # reminder: if you want to apply patches, do it at this point
     # reminder: you might want to update your build tag at this point
     make olddefconfig

   Now build your kernel::

     make -j $(nproc --all)

   Afterwards install the kernel as outlined above::

     command -v installkernel && sudo make modules_install install

   [:ref:`details<another>`]
```

 - 浣犵殑鍐呮牳浠ュ悗寰堝鏄撶Щ闄わ紝鍥犱负瀹冪殑鍚勪釜閮ㄥ垎鍙瓨鏀惧湪涓や釜鍦版柟锛屽苟涓斿彲浠ラ€氳繃鍐呮牳鐨勫彂琛屽悕娓呮櫚
   璇嗗埆銆傚彧瑕佺‘淇濅笉瑕佸垹闄や綘姝ｅ湪杩愯鐨勫唴鏍革紝鍥犱负閭ｅ彲鑳戒娇浣犵殑绯荤粺鏃犳硶鍚姩銆?
   棣栧厛鍒犻櫎淇濆瓨浣犲唴鏍告ā鍧楃殑鐩綍锛屽畠鍛藉悕涓猴細

```
     sudo rm -rf /lib/modules/6.0.1-foobar
```

  鐜板湪璇曚竴涓嬩笅闈㈣繖鏉″懡浠わ紝瀹冨湪涓€浜涘彂琛岀増涓婁細鍒犻櫎瀹夎鐨勬墍鏈夊叾浠栧唴鏍告枃浠讹紝鍚屾椂浠?bootloader
  閰嶇疆涓Щ闄よ鍐呮牳鐨勬潯鐩細

```
     command -v kernel-install && sudo kernel-install -v remove 6.0.1-foobar
```

  濡傛灉閭ｆ潯鍛戒护娌℃湁浠讳綍杈撳嚭鎴栧け璐ワ紝璇峰弬闃呭弬鑰冧竴鑺傦紱濡傛灉鍦?/boot/ 涓粛鐒舵湁浠讳綍鍚嶄负
  '*6.0.1-foobar*' 鐨勬枃浠讹紝涔熻繖涔堝仛銆?
   [:ref:`details<uninstall>`]


鎸夌収閫愭鎸囧崡鎿嶄綔鏃堕亣鍒颁簡鍙傝€冧竴鑺備篃娌¤兘瑙ｅ喅鐨勯夯鐑﹀悧锛熶綘鍙戠幇浜嗛敊璇悧锛熸垨鑰呬綘瀵瑰浣曟敼杩涙湰鎸囧崡
鏈夋兂娉曞悧锛?
濡傛灉浠ヤ笂浠绘剰鎯呭喌閫傜敤锛岃閫氳繃缁?Thorsten Leemhuis <linux@leemhuis.info> 鍙戦€佺畝鐭鏄庢垨琛ヤ竵锛?鏈€濂藉悓鏃舵妱閫佸叕寮€鐨?Linux 鏂囨。閭欢鍒楄〃 <linux-doc@vger.kernel.org>锛岃寮€鍙戣€呯煡閬撱€傝繖鏍风殑鍙嶉
瀵硅繘涓€姝ユ敼杩涙湰鏂囪嚦鍏抽噸瑕侊紝杩欑鍚堟瘡涓汉鐨勫埄鐩婏紝鍥犱负瀹冭兘璁╂洿澶氫汉鎺屾彙姝ゅ鎻忚堪鐨勪换鍔°€?
## 閫愭鎸囧崡鍙傝€冧竴鑺?

鏈妭淇濆瓨浜嗕笂杩版寚鍗椾腑姣忎竴姝ョ殑闄勫姞淇℃伅銆?

### 涓虹揣鎬ユ儏鍐靛仛鍑嗗


   **Create a fresh backup and put system repair and restore tools at hand**
   [... <backup_sbs>]

璁颁綇锛屼綘姝ｅ湪涓庤绠楁満鎵撲氦閬擄紝璁＄畻鏈烘湁鏃朵細鍙戠敓鎰忓鈥斺€斿挨鍏舵槸褰撲綘鎽嗗紕鍍忔搷浣滅郴缁熷唴鏍歌繖鏍峰叧閿殑閮ㄥ垎鏃躲€?鑰岃繖姝ｆ槸浣犲湪姝よ繃绋嬩腑瑕佸仛鐨勪簨鎯呫€傚洜姝わ紝鏈€濂戒负鍑虹幇鎰忓鍋氬ソ鍑嗗锛屽嵆浣垮畠鏈笉搴旇鍙戠敓銆?
[back to step-by-step guide <backup_sbs>]


### 搴斿 Secure Boot 涔嬬被鐨勬妧鏈?

   *On platforms with 'Secure Boot' or similar techniques, prepare everything to
   ensure the system will permit your self-compiled kernel to boot later.*
   [... <secureboot_sbs>]

璁稿鐜颁唬绯荤粺鍙厑璁告煇浜涙搷浣滅郴缁熷惎鍔紱鍥犳榛樿鎯呭喌涓嬪畠浠細鎷掔粷鍚姩鑷紪璇戠殑鍐呮牳銆?
鏈€鐞嗘兂鐨勫仛娉曟槸鍊熷姪璇佷功鍜岀鍚嶈浣犵殑骞冲彴淇′换浣犺嚜鏋勫缓鐨勫唴鏍搞€傚浣曞仛鍒拌繖涓€鐐硅繖閲屼笉鎻忚堪锛屽洜涓鸿繖闇€瑕?澶氫釜姝ラ锛屼細浣挎湰鏂囧亸绂诲叾鐩殑澶繙锛?Documentation/admin-guide/module-signing.rst' 浠ュ強澶氫釜缃戦〉
宸茬粡瀵规鍋氫簡鏇磋缁嗙殑璇存槑銆?
涓存椂绂佺敤 Secure Boot 涔嬬被鐨勬柟妗堟槸璁╀綘鑷繁鐨?Linux 鍚姩鐨勫彟涓€绉嶆柟寮忋€傚湪鍟嗗搧 x86 绯荤粺涓婏紝鍙互鍦?BIOS 璁剧疆宸ュ叿涓仛鍒拌繖涓€鐐癸紱鍏蜂綋姝ラ杩欓噷涓嶆弿杩帮紝鍥犱负瀹冧滑鍦ㄤ笉鍚屾満鍣ㄤ箣闂村樊寮傚緢澶с€?
鍦ㄤ富娴?x86 Linux 鍙戣鐗堜笂锛岃繕鏈夌涓夌閫夋嫨涓旀槸閫氱敤鐨勶細涓轰綘鐨?Linux 鐜绂佺敤鎵€鏈?Secure Boot
闄愬埗銆備綘鍙互閫氳繃杩愯 `mokutil --disable-validation` 鏉ュ彂璧锋娴佺▼锛涘畠浼氭彁绀轰綘鍒涘缓涓€涓竴娆℃€у瘑鐮侊紝
鎶婂畠鍐欎笅鏉ユ槸瀹夊叏鐨勩€傜幇鍦ㄩ噸鍚紱鍦ㄤ綘鐨?BIOS 瀹屾垚鎵€鏈夎嚜妫€涔嬪悗锛宐ootloader Shim 浼氱珛鍗虫樉绀轰竴涓?钃濊壊鏂规锛屼笂闈㈡湁涓€鏉℃秷鎭?Press any key to perform MOK management"銆傚湪鍊掕鏃剁粨鏉熷墠鎸夋煇涓敭銆傝繖浼?鎵撳紑涓€涓彍鍗曪紝鍦ㄥ叾涓€夋嫨"Change Secure Boot state"銆係him 鐨?"MokManager" 鐜板湪浼氳姹備綘杈撳叆涔嬪墠
涓€娆℃€у瘑鐮佷腑闅忔満閫夊嚭鐨勪笁涓瓧绗︺€備竴鏃︽彁渚涳紝纭浣犵‘瀹炴兂瑕佺鐢ㄦ牎楠屻€備箣鍚庯紝鍏佽 MokManager 閲嶅惎鏈哄櫒銆?
[back to step-by-step guide <secureboot_sbs>]


### 瀹夎鏋勫缓闇€姹?

   **Install all software required to build a Linux kernel.**
   [...<buildrequires_sbs>]

鍐呮牳鐩稿綋鐙珛锛屼絾闄や簡缂栬瘧鍣ㄤ箣绫荤殑宸ュ叿涔嬪锛屾湁鏃朵綘杩橀渶瑕佸嚑涓簱鏉ユ瀯寤哄畠銆傚浣曞畨瑁呮墍闇€鐨勪竴鍒囧彇鍐充簬
浣犵殑 Linux 鍙戣鐗堜互鍙婁綘灏嗚鏋勫缓鐨勫唴鏍哥殑閰嶇疆銆?
浠ヤ笅鏄竴浜涗富娴佸彂琛岀増涓婁綘閫氬父闇€瑕佺殑渚嬪瓙锛?
```
     sudo apt install bc binutils bison dwarves flex gcc git make openssl \
       pahole perl-base libssl-dev libelf-dev

 * Fedora and derivatives::

     sudo dnf install binutils /usr/include/{libelf.h,openssl/pkcs7.h} \
       /usr/bin/{bc,bison,flex,gcc,git,openssl,make,perl,pahole}

 * openSUSE and derivatives::

     sudo zypper install bc binutils bison dwarves flex gcc git make perl-base \
       openssl openssl-devel libelf-dev
```

濡傛灉浣犳兂鐭ラ亾涓轰粈涔堣繖浜涘垪琛ㄥ寘鍚?openssl 鍙婂叾寮€鍙戝ご鏂囦欢锛氬畠浠槸 Secure Boot 鏀寔鎵€闇€瑕佺殑锛岃澶?鍙戣鐗堝湪鍏?x86 鏈哄櫒鐨勫唴鏍搁厤缃腑鍚敤浜嗗畠銆?
鏈夋椂浣犱篃闇€瑕佽濡?bzip2銆乬zip銆乴z4銆乴zma銆乴zo銆亁z 鎴?zstd 绛夊帇缂╂牸寮忕殑宸ュ叿銆?
濡傛灉浣犳墽琛屾湰鎸囧崡鏈兜鐩栫殑浠诲姟锛屽彲鑳介渶瑕侀澶栫殑搴撳強鍏跺紑鍙戝ご鏂囦欢銆備緥濡傦紝浠?tools/ 鐩綍鏋勫缓鍐呮牳宸ュ叿
鏃堕渶瑕?zlib锛涚敤 'menuconfig' 鎴?'xconfig' 涔嬬被鐨?make 鐩爣璋冩暣鏋勫缓閰嶇疆灏嗛渶瑕?ncurses 鎴?Qt5
鐨勫紑鍙戝ご鏂囦欢銆?
[back to step-by-step guide <buildrequires_sbs>]


### 绌洪棿闇€姹?

   **Ensure to have enough free space for building and installing Linux.**
   [... <diskspace_sbs>]

鎻愬埌鐨勬暟瀛楁槸鍦ㄧ暀瓒充綑閲忎互淇濊瘉瀹夊叏鍓嶆彁涓嬬殑绮楃暐浼拌锛屾墍浠ラ€氬父浣犻渶瑕佸緱鏇村皯銆?
濡傛灉浣犵┖闂村彈闄愶紝璁板緱鍦ㄥ埌杈惧叧浜庨厤缃皟鏁寸殑 <configmods> 涓€鑺傛椂闃呰鍙傝€冧竴鑺傦紝鍥犱负纭繚绂佺敤璋冭瘯绗﹀彿
浼氭妸娑堣€楃殑纾佺洏绌洪棿鍑忓皯濂藉嚑 GB銆?
[back to step-by-step guide <diskspace_sbs>]


### 涓嬭浇婧愪唬鐮?

  **Retrieve the sources of the Linux version you intend to build.**
  [...<sources_sbs>]

閫愭鎸囧崡姒傝堪浜嗗浣曚娇鐢ㄦ祬鍏嬮殕锛坰hallow clone锛夋潵鑾峰彇 Linux 鐨勬簮浠ｇ爜銆傚叧浜庤繖绉嶆柟娉曡繕鏈夋洿澶氬彲璇寸殑
<sources_shallow>锛屽苟涓旇繕鏈変袱绉嶅€煎緱涓€鎻愮殑鏇夸唬鏂瑰紡锛氭墦鍖呭綊妗?<sources_archive> 鍜屽畬鏁?git 鍏嬮殕
<sources_full>銆備互鍙?鏄惁浣跨敤閫傚綋鐨勯鍙戝竷鐗堟湰鑰岄潪鏈€鏂扮殑涓荤嚎浠ｇ爜鏇存槑鏅?<sources_snapshot>"鍜?濡備綍鑾峰彇鏇存柊椴滅殑涓荤嚎浠ｇ爜搴?<sources_fresher>"杩欎袱涓柟闈篃闇€瑕侀槓杩般€?
娉ㄦ剰锛屼负绠€鍗曡捣瑙侊紝鏈寚鍗椾腑浣跨敤鐨勫懡浠ゆ妸鏋勫缓浜х墿瀛樻斁鍦ㄦ簮浠ｇ爜鏍戜腑銆傚鏋滀綘鍊惧悜浜庢妸瀹冧滑鍒嗗紑锛屽彧闇€鍦?鎵€鏈?make 璋冪敤涓姞涓婄被浼?`O=~/linux-builddir/` 鐨勫唴瀹癸紱鍚屾椂璋冩暣鎵€鏈夋坊鍔犳枃浠舵垨淇敼浠讳綍鐢熸垚鏂囦欢
锛堝浣犵殑 '.config'锛夌殑鍛戒护涓殑璺緞銆?
[back to step-by-step guide <sources_sbs>]


#### 娴呭厠闅嗗€煎緱娉ㄦ剰鐨勭壒鎬?

閫愭鎸囧崡浣跨敤浜嗘祬鍏嬮殕锛屽洜涓哄畠鏄湰鏂囨。鐩爣鍙椾紬澶у鏁版儏鍐典笅鐨勬渶浣虫柟妗堛€傝繖绉嶆柟寮忔湁鍑犱釜鏂归潰鍊煎緱涓€鎻愶細

 - 鏈枃妗ｅ湪澶у鏁板湴鏂逛娇鐢?`git fetch` 閰嶅悎 `--shallow-exclude=` 鏉ユ寚瀹氫綘鍏冲績鐨勬渶鏃╃増鏈紙鏇村噯纭?   鍦拌锛氬畠鐨?git 鏍囩锛夈€備綘涔熷彲浠ユ敼鐢ㄥ弬鏁?`--shallow-since=` 鏉ユ寚瀹氫竴涓粷瀵圭殑锛堟瘮濡?`'2023-07-15'`锛?   鎴栫浉瀵圭殑锛坄'12 months'`锛夋棩鏈燂紝浠ュ畾涔変綘鎯宠涓嬭浇鐨勫巻鍙叉繁搴︺€備綔涓虹浜岀鏇夸唬锛屼綘涔熷彲浠ユ樉寮忔寚瀹?   鏌愪釜娣卞害锛屼娇鐢ㄧ被浼?`--depth=1` 鐨勫弬鏁帮紝闄ら潪浣犱负 stable/longterm 鍐呮牳娣诲姞浜嗗垎鏀€?
 - 杩愯 `git fetch` 鏃讹紝璁板緱濮嬬粓鍍忛€愭鎸囧崡涓偅鏍锋寚瀹氭渶鏃╃殑鐗堟湰銆佷綘鍏冲績鐨勬椂鍒伙紝鎴栨樉寮忕殑娣卞害銆傚惁鍒?   浣犲皢闈复涓嬭浇鍑犱箮鏁翠釜 git 鍘嗗彶鐨勯闄╋紝杩欎細娑堣€楃浉褰撳鐨勬椂闂村拰甯﹀锛屽悓鏃朵篃浼氱粰鏈嶅姟鍣ㄥ甫鏉ュ帇鍔涖€?
   娉ㄦ剰锛屼綘涓嶄竴瀹氳濮嬬粓浣跨敤鐩稿悓鐨勭増鏈垨鏃ユ湡銆備絾褰撲綘闅忕潃鏃堕棿鐨勬帹绉绘敼鍙樺畠鏃讹紝git 浼氭妸鍘嗗彶鍔犳繁鎴?   鍘嬫墎鍒版寚瀹氱殑鐐广€傝繖璁╀綘鑳藉鑾峰彇浣犳渶鍒濅互涓轰笉闇€瑕佺殑鐗堟湰鈥斺€旀垨鑰呭畠浼氫涪寮冭緝鏃х増鏈殑婧愪唬鐮侊紝渚嬪褰撲綘
   鎯抽噴鏀句竴浜涚鐩樼┖闂存椂銆傚悗鑰呭湪浣跨敤浜?`--shallow-since=` 鎴?`--depth=` 鏃朵細鑷姩鍙戠敓銆?
 - 璀﹀憡锛氬綋鍔犳繁浣犵殑鍏嬮殕鏃讹紝浣犲彲鑳戒細閬囧埌绫讳技
   'fatal: error in object: unshallow cafecaca0c0dacafecaca0c0dacafecaca0c0da' 鐨勯敊璇€?   鍦ㄨ繖绉嶆儏鍐典笅杩愯 `git repack -d` 鐒跺悗鍐嶈瘯涓€娆`

 - 濡傛灉浣犳兂鍥為€€鏌愪釜鐗堟湰鐨勬敼鍔紙姣斿 Linux 6.3锛夋垨杩涜浜屽垎锛坴6.2..v6.3锛夛紝鏈€濂借 `git fetch` 鑾峰彇
   鏃╄嚦涓変釜鐗堟湰涔嬪墠锛堟瘮濡?6.0锛夌殑瀵硅薄锛歚git describe` 涔嬪悗灏辫兘鍍忓湪瀹屾暣 git 鍏嬮殕涓竴鏍锋弿杩板ぇ澶氭暟鎻愪氦銆?
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 浣跨敤鎵撳寘褰掓。涓嬭浇婧愪唬鐮?

鍒氭帴瑙︾紪璇?Linux 鐨勪汉甯稿父浠ヤ负閫氳繃 https://kernel.org 鐨勯椤典笅杞藉綊妗ｆ槸鑾峰彇 Linux 婧愪唬鐮佺殑鏈€浣虫柟娉曘€?鍦ㄦ煇浜涙儏鍐典笅纭疄濡傛锛屽鏋滀綘纭畾鍙瀯寤轰竴涓壒瀹氬唴鏍哥増鏈笖涓嶆敼鍔ㄤ换浣曚唬鐮佺殑璇濄€傞棶棰樻槸锛氫綘鍙兘纭俊浼?濡傛锛屼絾鍦ㄥ疄璺典腑杩欏父甯歌璇佹槑鏄竴涓敊璇殑鍋囪銆?
杩欐槸鍥犱负褰撴姤鍛婃垨璋冭瘯闂鏃讹紝寮€鍙戣€呭父甯镐細瑕佹眰灏濊瘯鍙︿竴涓増鏈€備粬浠篃鍙兘寤鸿鐢?`git revert` 涓存椂
鎾ら攢鏌愪釜鎻愪氦锛屾垨鍙兘鎻愪緵鍚勭琛ヤ竵鏉ュ皾璇曘€傛湁鏃舵姤鍛婅€呬篃浼氳瑕佹眰浣跨敤 `git bisect` 鏉ユ壘鍑哄鑷撮棶棰樼殑
鏀瑰姩銆傝繖浜涗簨鎯呴兘渚濊禆 git锛屾垨鑰呮湁浜?git 浼氬鏄撳拰蹇嵎寰楀銆?
娴呭厠闅嗕篃涓嶄細澧炲姞浠讳綍鏄捐憲寮€閿€銆備緥濡傦紝褰撲綘浣跨敤 `git clone --depth=1` 鏉ュ垱寤轰竴涓渶鏂颁富绾夸唬鐮佸簱鐨勬祬鍏嬮殕
鏃讹紝git 鍙細姣旈€氳繃 kernel.org 棣栭〉涓嬭浇鏈€鏂扮殑涓荤嚎棰勫彂甯冪増锛堝嵆 'rc'锛夊鍙栦竴鐐圭偣鏁版嵁銆?
鍥犳娴呭厠闅嗛€氬父鏄洿濂界殑閫夋嫨銆傚敖绠″姝わ紝濡傛灉浣犺繕鏄兂浣跨敤鎵撳寘鐨勬簮浠ｇ爜褰掓。锛岃閫氳繃 kernel.org 涓嬭浇涓€涓紱
涔嬪悗鎶婂叾鍐呭瑙ｅ帇鍒版煇涓洰褰曞苟鍒囨崲鍒拌В鍘嬫椂鍒涘缓鐨勫瓙鐩綍銆傞€愭鎸囧崡鐨勫叾浣欓儴鍒嗛兘鐓у父宸ヤ綔锛岄櫎浜嗕緷璧?git
鐨勯儴鍒嗏€斺€斾絾杩欎富瑕佹秹鍙婅繛缁瀯寤哄叾浠栫増鏈殑閭ｄ竴鑺傘€?
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 浣跨敤瀹屾暣 git 鍏嬮殕涓嬭浇婧愪唬鐮?

濡傛灉浣犱笉鍦ㄦ剰涓嬭浇鍜屽瓨鍌ㄥぇ閲忔暟鎹紙鎴嚦 2023 骞村垵绾?4.4 GB锛夛紝閭ｅ氨鎵ц瀹屾暣 git 鍏嬮殕锛岃€屼笉鏄祬鍏嬮殕銆?杩欐牱浣犱細閬垮厤涓婅堪鐨勭壒娈婁箣澶勶紝骞舵嫢鏈夋墍鏈夛細

```
    curl -L \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/clone.bundle \
      -o linux-stable.git.bundle
    git clone linux-stable.git.bundle ~/linux/
    rm linux-stable.git.bundle
    cd ~/linux/
    git remote set-url origin \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git
    git fetch origin
    git checkout --detach origin/master
```

[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 鎭板綋鐨勯鍙戝竷鐗堟湰锛圧C锛変笌鏈€鏂颁富绾?

褰撲娇鐢?git 鍏嬮殕婧愪唬鐮佸苟妫€鍑?origin/master 鏃讹紝浣犲父甯歌幏鍙栫殑浠ｇ爜搴撳浜庢渶鏂扮増鏈笌涓嬩竴涓彂甯冩垨棰勫彂甯?鐗堟湰涔嬮棿銆傝繖鍑犱箮鎬绘槸浣犵粰涓荤嚎涓€涓満浼氭椂鎯宠鐨勪唬鐮侊細鍍?v6.1-rc5 杩欐牱鐨勯鍙戝竷鐗堝苟涓嶇壒娈婏紝鍥犱负瀹冧滑
鍦ㄥ彂甯冨墠涓嶄細鑾峰緱浠讳綍鏄捐憲鐨勯澶栨祴璇曘€?
鏈変竴涓緥澶栵細浣犲彲鑳芥兂鍦ㄥ叾鍚庣户鑰呯殑绗竴涓鍙戝竷鐗堟湰锛坴6.2-rc1锛夊彂甯冧箣鍓嶏紝鍧氭寔浣跨敤鏈€鏂扮殑涓荤嚎鍙戝竷鐗?锛堟瘮濡?v6.1锛夈€傝繖鏄洜涓哄湪姝ゆ湡闂寸紪璇戦敊璇拰鍏朵粬闂鏇存湁鍙兘鍙戠敓锛屽洜涓烘鏃朵富绾垮浜庡畠鐨?鍚堝苟绐楀彛"
锛坢erge window锛夛細涓€涓€氬父涓烘湡涓ゅ懆鐨勯樁娈碉紝鍏堕棿涓轰笅涓€涓彂甯冪増鏈墍鍋氱殑缁濆ぇ閮ㄥ垎鏀瑰姩浼氳鍚堝苟銆?
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 閬垮厤涓荤嚎婊炲悗


瀵规祬鍏嬮殕鍜屽畬鏁村厠闅嗙殑瑙ｉ噴閮芥槸浠?Linux stable git 浠撳簱鑾峰彇浠ｇ爜銆傝繖瀵规湰鏂囨。鐨勮鑰呮潵璇存洿绠€鍗曪紝鍥犱负瀹?鍏佽杞绘澗璁块棶涓荤嚎鍜?stable/longterm 鍙戝竷鐗堟湰銆傝繖绉嶆柟寮忓彧鏈変竴涓己鐐癸細

鍚堝苟鍒颁富绾夸粨搴撶殑鏀瑰姩鍙瘡闅斿嚑灏忔椂鍚屾鍒?Linux stable 浠撳簱鐨?master 鍒嗘敮銆傝繖绉嶆粸鍚庡湪澶у鏁版椂鍊?涓嶅€煎緱鎷呭績锛涗絾濡傛灉浣犵湡鐨勯渶瑕佹渶鏂颁唬鐮侊紝鍙渶锛?
```
    git remote add mainline \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
    git fetch mainline
    git checkout --detach mainline/master
```

鍦ㄦ祬鍏嬮殕涓婅繖鏍峰仛鏃讹紝璁板緱鐢ㄥ墠闈㈡弿杩扮殑鏌愪釜鍙傛暟璋冪敤 `git fetch` 鏉ラ檺鍒舵繁搴︺€?
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


### 缁欐簮浠ｇ爜鎵撹ˉ涓侊紙鍙€夛級


  **In case you want to apply a kernel patch, do so now.**
  [...<patching_sbs>]

杩欐鏄綘鍙兘鎯充负鍐呮牳鎵撹ˉ涓佺殑鍦版柟鈥斺€斾緥濡傦紝褰撴煇涓紑鍙戣€呮彁鍑轰簡涓€涓慨澶嶏紝骞惰浣犳鏌ュ畠鏄惁鏈夊府鍔╂椂銆?閫愭鎸囧崡宸茬粡瑙ｉ噴浜嗚繖閲屾墍鏈夊叧閿殑鍐呭銆?
[back to step-by-step guide <patching_sbs>]


### 涓烘鍐呮牳鏋勫缓鎵撴爣绛撅紙鍙€夛紝閫氬父鏄庢櫤锛?

  *If you patched your kernel or already have that kernel version installed,
  better tag your kernel by extending its release name:*
  [...<tagging_sbs>]

涓轰綘鐨勫唴鏍告墦鏍囩鏈夊姪浜庨伩鍏嶄互鍚庢贩娣嗭紝灏ゅ叾鏄綋浣犱负鍐呮牳鎵撲簡琛ヤ竵鏃躲€傛坊鍔犱竴涓嫭绔嬬殑鏍囩杩樺皢纭繚鍐呮牳鐨?鏄犲儚鍙婂叾妯″潡涓庝换浣曠幇鏈夊唴鏍稿苟琛屽畨瑁呫€?
鏈夊绉嶆柟寮忔坊鍔犺繖鏍风殑鏍囩銆傞€愭鎸囧崡閫氳繃鍦ㄤ綘鏋勫缓鐩綍涓垱寤轰竴涓?'localversion' 鏂囦欢鏉ュ疄鐜板叾涓竴绉嶏紝
鍐呮牳鏋勫缓鑴氭湰浼氳嚜鍔ㄤ粠涓幏鍙栬鏍囩銆備綘浠ュ悗鍙互淇敼璇ユ枃浠朵互鍦ㄥ悗缁瀯寤轰腑浣跨敤涓嶅悓鐨勬爣绛撅紝鎴栫畝鍗曞湴鍒犻櫎
璇ユ枃浠朵互涓㈠純鏍囩銆?
[back to step-by-step guide <tagging_sbs>]


### 涓轰綘鐨勫唴鏍稿畾涔夋瀯寤洪厤缃?

  *Create the build configuration for your kernel based on an existing
  configuration.* [... <configuration_sbs>]

杩欎竴姝ユ湁鍑犱釜鏂归潰闇€瑕佹洿浠旂粏鐨勮В閲婏細


#### 浣跨敤鍙︿竴涓厤缃枃浠朵綔涓哄熀纭€鏃剁殑闄烽槺


make 鐩爣濡?localmodconfig 鍜?olddefconfig 鏈変竴浜涘叡鍚岀殑闄烽槺锛屼綘搴斿綋浜嗚В锛?
 - 杩欎簺鐩爣浼氬鐢ㄤ綘鏋勫缓鐩綍涓凡鏈夌殑鍐呮牳鏋勫缓閰嶇疆锛堜緥濡?'~/linux/.config'锛夛紝濡傛灉瀛樺湪鐨勮瘽銆傚洜姝?   濡傛灉浣犳兂浠庨浂寮€濮嬶紝灏遍渶瑕佸垹闄ゅ畠銆?
 - make 鐩爣浼氬皾璇曡嚜鍔ㄦ壘鍒颁綘姝ｅ湪杩愯鐨勫唴鏍哥殑閰嶇疆锛屼絾鍙兘閫夊緱涓嶅ソ銆備竴琛屽儚
   '# using defaults found in /boot/config-6.0.7-250.fc36.x86_64' 鎴?   'using config: /boot/config-6.0.7-250.fc36.x86_64' 浼氬憡璇変綘瀹冧滑閫変簡鍝釜鏂囦欢銆傚鏋滈偅涓嶆槸鎯宠鐨?   閭ｄ釜锛屽彧闇€鍦ㄤ娇鐢ㄨ繖浜?make 鐩爣涔嬪墠鎶婂畠瀛樹负 '~/linux/.config'銆?
 - 濡傛灉浣犺瘯鍥炬妸涓€涓负鏌愪釜鍐呮牳锛堟瘮濡?v6.0锛夊噯澶囩殑閰嶇疆鏂囦欢鐢ㄥ湪涓€涓洿鑰佺殑浠ｉ檯锛堟瘮濡?v5.15锛変笂锛屽彲鑳戒細
   鍙戠敓鎰忔兂涓嶅埌鐨勪簨鎯呫€傚湪杩欑鎯呭喌涓嬶紝浣犲彲鑳芥兂浣跨敤涓€涓綘鐨勫彂琛岀増鍦ㄤ娇鐢ㄩ偅涓垨绋嶈€佺殑鍐呮牳鐗堟湰鏃舵墍鐢ㄨ繃鐨?   閰嶇疆浣滀负鍩虹銆?

#### 褰卞搷閰嶇疆


make 鐩爣 olddefconfig 浠ュ強浣跨敤 localmodconfig 鏃剁殑 `yes "" |` 浼氭妸浠讳綍鏈畾涔夌殑鏋勫缓閫夐」璁句负瀹冧滑鐨?榛樿鍊笺€傝繖鍏朵腑鍖呮嫭浼氱鐢ㄨ澶氬湪浣犵殑鍩虹鍐呮牳鍙戝竷涔嬪悗鎵嶅紩鍏ョ殑鍐呮牳鍔熻兘銆?
濡傛灉浣犳兂鎵嬪姩璁剧疆杩欎簺閰嶇疆閫夐」锛岃浣跨敤 `oldconfig` 鑰岄潪 `olddefconfig`锛屾垨鑰呭湪浣跨敤 localmodconfig 鏃?鐪佺暐 `yes "" |`銆傜劧鍚庡浜庢瘡涓湭瀹氫箟鐨勯厤缃€夐」锛屼綘閮戒細琚棶鍒板浣曡繘琛屻€傚鏋滀綘涓嶇‘瀹氳濡備綍鍥炵瓟锛屽彧闇€
鎸?'enter' 搴旂敤榛樿鍊笺€?

#### 浣跨敤 localmodconfig 鏃剁殑澶у潙


姝ｅ閫愭鎸囧崡涓凡缁忕畝瑕佽В閲婄殑锛氫娇鐢?localmodconfig 鏃讹紝寰堝鏄撳彂鐢熶綘鑷瀯寤虹殑鍐呮牳缂哄皯浣犲湪浣跨敤杩欎釜 make
鐩爣涔嬪墠娌℃湁鎵ц杩囩殑浠诲姟鎵€闇€鐨勬ā鍧椼€傝繖鏄洜涓洪偅浜涗换鍔￠渶瑕佺殑鍐呮牳妯″潡閫氬父浼氬湪浣犵涓€娆℃墽琛岃浠诲姟鏃惰嚜鍔?鍔犺浇锛涘鏋滀綘鍦ㄤ娇鐢?localmodconfig 涔嬪墠鑷冲皯鎵ц杩囦竴娆¤浠诲姟锛屽悗鑰呭氨浼氬亣瀹氳繖浜涙ā鍧楁槸澶氫綑鐨勮€岀鐢ㄥ畠浠€?
浣犲彲浠ラ€氳繃鎵ц閭ｄ簺甯稿父浼氳嚜鍔犺浇棰濆鍐呮牳妯″潡鐨勫吀鍨嬩换鍔℃潵灏介噺閬垮厤杩欎竴鐐癸細鍚姩涓€涓櫄鎷熸満銆佸缓绔?VPN 杩炴帴銆?鍥炵幆鎸傝浇涓€涓?CD/DVD 鐨?ISO銆佹寕杞界綉缁滃叡浜紙CIFS銆丯FS鈥︹€︼級锛屼互鍙婅繛鎺ユ墍鏈夊閮ㄨ澶囷紙2FA 瀵嗛挜銆佸ご鎴村紡
鑰虫満銆佺綉缁滄憚鍍忓ご鈥︹€︼級浠ュ強浣犲钩鏃朵笉浣跨敤鐨勬枃浠剁郴缁燂紙btrfs銆乪xt4銆丗AT銆丯TFS銆乆FS鈥︹€︼級鐨勫瓨鍌ㄨ澶囥€備絾寰堥毦
鎯冲埌涓€鍒囧彲鑳介渶瑕佺殑鈥斺€斿嵆渚挎槸鍐呮牳寮€鍙戣€呭湪杩欎釜鐐逛笂涔熷父甯稿繕璁拌繖鎴栭偅銆?
涓嶈璁╄繖绉嶉闄╁洶鎵颁綘锛屽挨鍏舵槸鍦ㄤ粎涓烘祴璇曠洰鐨勭紪璇戝唴鏍告椂锛氭墍鏈夐€氬父鍏抽敭鐨勪笢瑗块兘浼氬湪閭ｉ噷銆傝€屼笖濡傛灉浣犲繕浜?鏌愪簺閲嶈鐨勪笢瑗匡紝浠ュ悗鍙互鎵撳紑缂哄け鐨勫姛鑳斤紝骞跺揩閫熻繍琛屽懡浠ゆ潵缂栬瘧鍜屽畨瑁呬竴涓洿濂界殑鍐呮牳銆?
浣嗗鏋滀綘鎵撶畻瀹氭湡鏋勫缓鍜屼娇鐢ㄨ嚜鏋勫缓鐨勫唴鏍革紝浣犲彲鑳芥兂閫氳繃璁板綍浣犵殑绯荤粺鍦ㄥ嚑鍛ㄨ繃绋嬩腑鍔犺浇浜嗗摢浜涙ā鍧楁潵闄嶄綆椋庨櫓銆?浣犲彲浠ョ敤 `modprobed-db <https://github.com/graysky2/modprobed-db>`_ 鎶婂畠鑷姩鍖栥€備箣鍚庝娇鐢?`LSMOD=<path>` 鏉ワ細

```
    yes "" | make LSMOD="${HOME}"/.config/modprobed.db localmodconfig
```

#### 鐢?localmodconfig 杩涜杩滅▼鏋勫缓


濡傛灉浣犳兂鐢?localmodconfig 涓哄彟涓€鍙版満鍣ㄦ瀯寤哄唴鏍革紝鍦ㄥ畠涓婇潰杩愯 `lsmod > lsmod_foo-machine` 骞舵妸璇ユ枃浠?浼犺緭鍒颁綘鐨勬瀯寤轰富鏈恒€傜幇鍦ㄥ儚杩欐牱鎶婃瀯寤鸿剼鏈寚鍚戣鏂囦欢锛歚`yes "" | make LSMOD=~/lsmod_foo-machine
localmodconfig``銆傛敞鎰忥紝鍦ㄨ繖绉嶆儏鍐典笅浣犲彲鑳戒篃鎯充粠鍙︿竴鍙版満鍣ㄥ鍒朵竴浠藉熀纭€鍐呮牳閰嶇疆杩囨潵锛屽苟鎶婂畠浣滀负 .config
鏀惧湪浣犵殑鏋勫缓鐩綍涓€?
[back to step-by-step guide <configuration_sbs>]


### 璋冩暣鏋勫缓閰嶇疆


   *Check if you might want to or have to adjust some kernel configuration
   options:*

鏍规嵁浣犵殑闇€姹傦紝鍦ㄨ繖涓€鐐逛笂浣犲彲鑳芥兂鎴栧繀椤昏皟鏁翠竴浜涘唴鏍搁厤缃€夐」銆?

#### 璋冭瘯绗﹀彿


   **Evaluate how you want to handle debug symbols.**
   [...<configmods_sbs>]

澶у鏁扮敤鎴蜂笉闇€瑕佸叧蹇冭繖涓紝閫氬父淇濇寔鍘熸牱灏卞ソ锛涗絾濡傛灉浣犲彲鑳介渶瑕佽В鐮佷竴涓爢鏍堣窡韪紝鎴栨兂鍑忓皯绌洪棿鍗犵敤锛屼綘
搴斿綋鏇翠粩缁嗗湴鐪嬩竴涓嬨€?
褰撲綘鐨勫唴鏍镐互鍚庤繍琛屾椂鎶涘嚭 'panic'銆?Oops'銆?warning' 鎴?'BUG' 鏃讹紝鎷ユ湁鍙敤鐨勮皟璇曠鍙峰彲鑳藉緢閲嶈锛?鍥犱负閭ｆ椂浣犲皢鑳藉鎵惧埌闂鍦ㄤ唬鐮佷腑鍙戠敓鐨勭‘鍒囦綅缃€備絾鏀堕泦鍜屽祵鍏ユ墍闇€鐨勮皟璇曚俊鎭渶瑕佹椂闂村苟娑堣€楃浉褰撳鐨?绌洪棿锛氬湪 2022 骞存湯锛岀敤 localmodconfig 閰嶇疆鐨勫吀鍨?x86 鍐呮牳鐨勬瀯寤轰骇鐗╁湪寮€鍚皟璇曠鍙锋椂娑堣€楃害 5 GB 绌洪棿锛?鑰屽湪绂佺敤鏃朵笉鍒?1 GB銆傜敱姝や骇鐢熺殑鍐呮牳鏄犲儚鍜屾ā鍧椾篃鏇村ぇ锛屼粠鑰屽鍔犱簡鍔犺浇鏃堕棿銆?
鍥犳锛屽鏋滀綘鎯宠涓€涓皬鐨勫唴鏍革紝骞朵笖涓嶅お鍙兘瑙ｇ爜鍫嗘爤璺熻釜锛?
```
    ./scripts/config --file .config -d DEBUG_INFO \
      -d DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -d DEBUG_INFO_DWARF4 \
      -d DEBUG_INFO_DWARF5 -e CONFIG_DEBUG_INFO_NONE
    make olddefconfig
```

鍙︿竴鏂归潰锛屽鏋滀綘浠ュ悗寰堝彲鑳介渶瑕佽В鐮佸爢鏍堣窡韪紙濡?Documentation/admin-guide/tainted-kernels.rst 涓殑
"Decode failure messages" 鎵€瑙ｉ噴鐨勶級锛屼綘缁濆鎯冲惎鐢ㄥ畠浠細

```
    ./scripts/config --file .config -d DEBUG_INFO_NONE -e DEBUG_KERNEL
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS -e KALLSYMS_ALL
    make olddefconfig
```

娉ㄦ剰锛岃澶氫富娴佸彂琛岀増鍦ㄥ叾鍐呮牳閰嶇疆涓惎鐢ㄤ簡璋冭瘯绗﹀彿鈥斺€斿洜姝ゅ儚 localmodconfig 鍜?olddefconfig 杩欐牱鐨?make
鐩爣甯稿父浼氶噰鐢ㄨ璁剧疆銆?
[back to step-by-step guide <configmods_sbs>]


#### 鍙戣鐗堢壒瀹氱殑璋冩暣


   **Are you running** [... <configmods_sbs>]

浠ヤ笅鍑犺妭甯姪浣犻伩鍏嶅湪鏈寚鍗楄嫢骞插晢鍝佸彂琛岀増涓婂凡鐭ヤ細鍙戠敓鐨勬瀯寤洪棶棰樸€?

**Debian:**

 - 绉婚櫎瀵逛竴涓瘉涔︽枃浠剁殑闄堟棫寮曠敤锛屽畠浼氳浣犵殑鏋勫缓锛?
```
    ./scripts/config --file .config --set-str SYSTEM_TRUSTED_KEYS ''
```

   鎴栬€咃紝涓嬭浇鎵€闇€璇佷功骞惰璇ラ厤缃€夐」鎸囧悜瀹冿紝姝ｅ `the Debian handbook explains in more detail
   <https://debian-handbook.info/browse/stable/sect.kernel-compilation.html>`_ 鎵€璇﹁堪鈥斺€旀垨鑰呯敓鎴?   浣犺嚜宸辩殑锛屽 Documentation/admin-guide/module-signing.rst 涓墍杩般€?
[back to step-by-step guide <configmods_sbs>]


#### 鍗曠嫭璋冩暣


   *If you want to influence the other aspects of the configuration, do so
   now* [... <configmods_sbs>]

鍦ㄨ繖涓€鐐逛笂浣犲彲浠ヤ娇鐢ㄥ儚 `make menuconfig` 杩欐牱鐨勫懡浠わ紝閫氳繃鍩轰簬鏂囨湰鐨勭晫闈㈡潵鍚敤鎴栫鐢ㄦ煇浜涘姛鑳斤紱瑕佷娇鐢?鍥惧舰鍖栭厤缃紝璇锋敼鐢?make 鐩爣 `xconfig` 鎴?`gconfig`銆傚畠浠兘闇€瑕佸叾鎵€鍩轰簬宸ュ叿鍖咃紙ncurses銆丵t5銆丟tk2锛?鐨勫紑鍙戝簱锛涘鏋滅己澶辨墍闇€涔嬬墿锛屼竴鏉￠敊璇秷鎭細鍛婅瘔浣犮€?
[back to step-by-step guide <configmods_sbs>]


### 鏋勫缓浣犵殑鍐呮牳


  **Build the image and the modules of your kernel** [... <build_sbs>]

杩欎釜闃舵鍙兘鍑洪敊鐨勫湴鏂瑰緢澶氾紝浣嗕笅闈㈢殑璇存槑浼氬府鍔╀綘鑷姪銆傚彟涓€涓皬鑺傝瑙ｅ浣曠洿鎺ユ妸浣犵殑鍐呮牳鎵撳寘鎴?deb銆?rpm 鎴?tar 鏂囦欢銆?

#### 澶勭悊鏋勫缓閿欒


褰撳彂鐢熸瀯寤洪敊璇椂锛屽畠鍙兘鏄敱浣犳満鍣ㄨ缃殑鏌愪簺鏂归潰寮曡捣鐨勶紝閫氬父鍙互蹇€熶慨澶嶏紱浣嗗叾浠栨椂鍊欓棶棰樺嚭鍦ㄤ唬鐮?涓紝鍙兘鐢卞紑鍙戣€呬慨澶嶃€備粩缁嗘鏌ュけ璐ユ秷鎭紝鍐嶅姞涓婁竴浜涗簰鑱旂綉涓婄殑鐮旂┒锛岄€氬父浼氬憡璇変綘杩欎袱鑰呭睘浜庡摢涓€绉嶃€傝
杩涜杩欐牱鐨勮皟鏌ワ紝閲嶆柊鍚姩鏋勫缓锛?
```
    make V=1
```

`V=1` 浼氭縺娲昏缁嗚緭鍑猴紝杩欏彲鑳芥槸鐪嬫竻瀹為檯閿欒鎵€蹇呴渶鐨勩€備负浜嗚瀹冩洿瀹规槗琚敞鎰忓埌锛岃繖鏉″懡浠や篃鐪佺暐浜嗕箣鍓?鐢ㄤ簬鍒╃敤绯荤粺涓瘡涓?CPU 鏍稿績鐨?``-j $(nproc --all)`` 鈥斺€斾絾杩欑骞惰鍖栧湪鍑洪敊鏃朵篃浼氬甫鏉ヤ竴浜涙潅涔便€?
鍑犵閽熷悗锛屾瀯寤鸿繃绋嬪簲璇ヤ細鍐嶆閬囧埌閿欒銆傜幇鍦ㄨ瘯鐫€鎵惧嚭鎻忚堪闂鏈€鍏抽敭鐨勯偅涓€琛屻€傜劧鍚庡湪浜掕仈缃戜笂鎼滅储璇ヨ
涓渶閲嶈銆佹渶涓嶉€氱敤鐨勪竴娈碉紙姣斿 4 鍒?8 涓瘝锛夛紱閬垮厤鎴栧幓鎺変换浣曠湅璧锋潵杩滅▼绯荤粺鐗瑰畾鐨勪笢瑗匡紝姣斿浣犵殑鐢ㄦ埛鍚?鎴栧儚 `/home/username/linux/` 杩欐牱鐨勬湰鍦拌矾寰勫悕銆傞鍏堢敤璇ュ瓧绗︿覆灏濊瘯浣犲父鐢ㄧ殑浜掕仈缃戞悳绱㈠紩鎿庯紝涔嬪悗閫氳繃
`lore.kernel.org/all/ <https://lore.kernel.org/all/>`_ 鎼滅储 Linux 鍐呮牳閭欢鍒楄〃銆?
杩欏ぇ澶氭暟鏃跺€欎細鎵惧埌鑳借В閲婇棶棰樻墍鍦ㄧ殑鍐呭锛涘緢甯歌鐨勬槸鍏朵腑涓€涓懡涓篃浼氫负浣犵殑鎻愪緵涓€涓В鍐虫柟妗堛€傚鏋滀綘
娌℃湁鎵惧埌涓庝綘闂鍖归厤鐨勫唴瀹癸紝鎹竴涓搴︼紝閫氳繃淇敼鎼滅储璇嶆垨浣跨敤閿欒娑堟伅涓殑鍙︿竴琛屽啀璇曚竴娆°€?
褰掓牴缁撳簳锛屼綘灏嗚閬囧埌鐨勫ぇ澶氭暟楹荤儲寰堝彲鑳藉凡缁忚鍒汉閬囧埌骞舵姤鍛婅繃浜嗐€傝繖鍖呮嫭鍘熷洜涓嶅湪浣犵殑绯荤粺銆佽€屽湪浜庝唬鐮佺殑
闂銆傚鏋滀綘閬囧埌浜嗗叾涓箣涓€锛屼綘涔熷彲鑳戒负浣犵殑鎵惧埌瑙ｅ喅鏂规锛堟瘮濡備竴涓ˉ涓侊級鎴栧彉閫氭柟娉曘€?

#### 鎵撳寘浣犵殑鍐呮牳


閫愭鎸囧崡浣跨敤榛樿鐨?make 鐩爣锛坸86 涓婄殑 'bzImage' 鍜?'modules'锛夋潵鏋勫缓鍐呮牳鐨勬槧鍍忓拰妯″潡锛屾寚鍗楀悗闈㈢殑
姝ラ鍐嶅畨瑁呭畠浠€備綘涔熷彲浠ユ敼鐢ㄤ互涓嬬洰鏍囦箣涓€锛岀洿鎺ユ瀯寤轰竴鍒囧苟鐩存帴鎵撳寘锛?
 - `make -j $(nproc --all) bindeb-pkg` 鐢熸垚 deb 鍖?
 - `make -j $(nproc --all) binrpm-pkg` 鐢熸垚 rpm 鍖?
 - `make -j $(nproc --all) tarbz2-pkg` 鐢熸垚 bz2 鍘嬬缉鐨?tar 鍖?
杩欏彧鏄负姝ょ洰鐨勫彲鐢ㄧ殑 make 鐩爣鐨勪竴涓€夋嫨锛屽叾浠栫殑璇峰弬瑙?`make help`銆備綘涔熷彲浠ュ湪杩愯
`make -j $(nproc --all)` 涔嬪悗浣跨敤杩欎簺鐩爣锛屽洜涓哄畠浠細鎺ョ鎵€鏈夊凡缁忔瀯寤哄ソ鐨勪笢瑗裤€?
濡傛灉浣犱娇鐢ㄨ繖浜涚洰鏍囨潵鐢熸垚 deb 鎴?rpm 鍖咃紝璇峰拷鐣ラ€愭鎸囧崡涓叧浜庡畨瑁呭拰绉婚櫎鍐呮牳鐨勮鏄庯紱鑰屾槸浣跨敤璇ユ牸寮忕殑
鍖呭伐鍏凤紙濡?dpkg 鍜?rpm锛夋垨鏋勫缓浜庡叾涓婄殑鍖呯鐞嗗伐鍏凤紙apt銆乤ptitude銆乨nf/yum銆亃ypper鈥︹€︼級鏉ュ畨瑁呭拰绉婚櫎
鍖呫€傛敞鎰忥紝鐢ㄨ繖涓や釜 make 鐩爣鐢熸垚鐨勫寘琚璁′负鍙湪浣跨敤杩欎簺鏍煎紡鐨勫悇绉嶅彂琛岀増涓婂伐浣滐紝鍥犳瀹冧滑鏈夋椂琛ㄧ幇浼?涓庝綘鐨勫彂琛岀増鐨勫唴鏍稿寘鏈夋墍涓嶅悓銆?
[back to step-by-step guide <build_sbs>]


### 瀹夎浣犵殑鍐呮牳


  **Now install your kernel** [... <install_sbs>]

鎵ц閫愭鎸囧崡涓殑鍛戒护涔嬪悗闇€瑕佸仛浠€涔堬紝鍙栧喅浜庢槸鍚﹀瓨鍦ㄤ互鍙?`installkernel` 鍙墽琛屾枃浠跺浣曞疄鐜般€傝澶氬晢鍝?Linux 鍙戣鐗堝湪 `/sbin/` 涓檮甯︿簡杩欐牱涓€涓唴鏍稿畨瑁呭櫒锛屽畠瀹屾垚鎵€闇€鐨勪竴鍒囷紝鍥犳闄や簡閲嶅惎浣犳棤浜嬪彲鍋氥€備絾鏈変簺
鍙戣鐗堝寘鍚殑 installkernel 鍙畬鎴愰儴鍒嗗伐浣溾€斺€斿皯鏁板彂琛岀増鍒欏畬鍏ㄧ己灏戝畠锛屾妸鎵€鏈夊伐浣滅暀缁欎綘銆?
濡傛灉鎵惧埌浜?`installkernel`锛屽唴鏍哥殑鏋勫缓绯荤粺浼氭妸鍐呮牳鏄犲儚鍙婄浉鍏虫枃浠剁殑瀹為檯瀹夎濮旀墭缁欒繖涓彲鎵ц鏂囦欢銆傚湪
鍑犱箮鎵€鏈?Linux 鍙戣鐗堜笂锛屽畠浼氭妸鏄犲儚瀛樹负 '/boot/vmlinuz-<浣犵殑鍐呮牳鍙戣鍚?'锛屽苟鍦ㄦ梺杈规斁涓€涓?'System.map-<浣犵殑鍐呮牳鍙戣鍚?'銆傚洜姝わ紝浣犵殑鍐呮牳浼氫笌鍏朵粬宸插瓨鍦ㄧ殑鍐呮牳骞惰瀹夎锛岄櫎闈炰綘宸茬粡鏈変竴涓彂琛屽悕
瀹屽叏鐩稿悓鐨勫唴鏍搞€?
璁稿鍙戣鐗堜笂鐨?installkernel 涔嬪悗浼氱敓鎴愪竴涓?'initramfs'锛堥€氬父涔熺О涓?'initrd'锛夛紝鍟嗗搧鍙戣鐗堜緷璧栧畠鏉?鍚姩锛涘洜姝ゅ姟蹇呬繚鎸侀€愭鎸囧崡涓娇鐢ㄧ殑涓や釜 make 鐩爣鐨勯『搴忥紝鍥犱负濡傛灉浣犲湪鍐呮牳妯″潡涔嬪墠瀹夎鍐呮牳鏄犲儚锛屼簨鎯?灏变細涔卞銆傞€氬父 installkernel 涔嬪悗涔熶細鎶婁綘鐨勫唴鏍告坊鍔犲埌 bootloader 閰嶇疆涓€傚鏋滀綘鐨勫彂琛岀増鐨?installkernel
涓嶅鐞嗗畠浠紝浣犲氨寰楄嚜宸辫礋璐ｈ繖涓ら」浠诲姟涓殑涓€椤规垨涓ら」銆?
灏戞暟鍙戣鐗堝 Arch Linux 鍙婂叾琛嶇敓鐗堝畬鍏ㄦ病鏈?installkernel 鍙墽琛屾枃浠躲€傚湪杩欎簺鍙戣鐗堜笂鍙渶鐢ㄥ唴鏍哥殑锛?
```
     sudo make modules_install
     sudo install -m 0600 $(make -s image_name) /boot/vmlinuz-$(make -s kernelrelease)
     sudo install -m 0600 System.map /boot/System.map-$(make -s kernelrelease)
```

濡傛灉浣犵殑鍙戣鐗堝€熷姪 initramfs 鍚姩锛岀幇鍦ㄧ敤浣犵殑鍙戣鐗堜负姝よ繃绋嬫彁渚涚殑宸ュ叿涓轰綘鐨勫唴鏍哥敓鎴愪竴涓€備箣鍚庢妸浣犵殑
鍐呮牳娣诲姞鍒颁綘鐨?bootloader 閰嶇疆涓苟閲嶅惎銆?
[back to step-by-step guide <install_sbs>]


### 浠ュ悗鍐嶆潵涓€杞?

  *To later build another kernel you need similar, but sometimes slightly
  different commands* [... <another_sbs>]

鏋勫缓鍚庣画鍐呮牳鐨勮繃绋嬬被浼硷紝浣嗗湪鏌愪簺鐐逛笂鐣ユ湁涓嶅悓銆備緥濡備綘涓嶆兂瀵瑰悗缁殑鍐呮牳鏋勫缓浣跨敤 'localmodconfig'锛屽洜涓?浣犲凡缁忓垱寤轰簡涓€涓綘鎯充粠鐜板湪璧蜂娇鐢ㄧ殑绮剧畝閰嶇疆銆傚洜姝ゆ敼涓哄彧浣跨敤 `oldconfig` 鎴?`olddefconfig` 鏉ユ妸浣犵殑
鏋勫缓閰嶇疆璋冩暣鍒颁綘瑕佹瀯寤虹殑鍐呮牳鐗堟湰鐨勯渶姹傘€?
濡傛灉浣犵敤 git 鍒涘缓浜嗘祬鍏嬮殕锛岃璁颁綇 :ref:`浠ユ洿璇︾粏鏂瑰紡瑙ｉ噴璇ヨ缃殑閭ｄ釜灏忚妭 <sources>`锛氫綘闇€瑕佷娇鐢ㄧ暐寰?涓嶅悓鐨?`git fetch` 鍛戒护锛屽苟涓斿湪鍒囨崲鍒板彟涓€涓郴鍒楁椂闇€瑕佹坊鍔犱竴涓澶栫殑杩滅▼鍒嗘敮銆?
[back to step-by-step guide <another_sbs>]


### 浠ュ悗鍗歌浇鍐呮牳


  *All parts of your installed kernel are identifiable by its release name and
  thus easy to remove later.* [... <uninstall_sbs>]

涓嶈鎷呭績鎵嬪姩瀹夎鍐呮牳浠庤€岀粫杩囦簡浣犲彂琛岀増鐨勬墦鍖呯郴缁熶細鎶婁綘鐨勬満鍣ㄥ交搴曞紕涔憋細浣犲唴鏍哥殑鎵€鏈夐儴鍒嗕互鍚庨兘寰堝鏄?绉婚櫎锛屽洜涓烘枃浠跺彧瀛樻斁鍦ㄤ袱涓湴鏂癸紝骞朵笖閫氬父鍙互閫氳繃鍐呮牳鐨勫彂琛屽悕璇嗗埆銆?
杩欎袱涓湴鏂逛箣涓€鏄?/lib/modules/ 涓殑涓€涓洰褰曪紝瀹冧繚瀛樻瘡涓凡瀹夎鍐呮牳鐨勬ā鍧椼€傝繖涓洰褰曚互鍐呮牳鐨勫彂琛屽悕鍛藉悕锛?鍥犳锛岃绉婚櫎鍏朵腑涓€涓唴鏍哥殑鎵€鏈夋ā鍧楋紝鍙渶绉婚櫎瀹冨湪 /lib/modules/ 涓殑妯″潡鐩綍銆?
鍙︿竴涓湴鏂规槸 /boot/锛屽湪閭ｉ噷瀹夎涓€涓唴鏍告椂閫氬父浼氭斁缃竴鍒颁簲涓枃浠躲€傚畠浠€氬父閮藉寘鍚彂琛屽悕鍦ㄦ枃浠跺悕涓紝浣?鏈夊灏戞枃浠跺強鍏跺悕绉板湪涓€瀹氱▼搴︿笂鍙栧喅浜庝綘鍙戣鐗堢殑 installkernel 鍙墽琛屾枃浠讹紙瑙佷笂鏂?<install>锛夊強鍏?initramfs 鐢熸垚鍣ㄣ€傚湪鏌愪簺鍙戣鐗堜笂锛岄€愭鎸囧崡涓彁鍒扮殑 `kernel-install` 鍛戒护浼氫负浣犵Щ闄ゆ墍鏈夎繖浜涙枃浠垛€斺€斿悓鏃?涔熺Щ闄ゅ畠浠湪鍐呮牳 bootloader 閰嶇疆涓殑鏉＄洰銆傚湪鍏朵粬鍙戣鐗堜笂锛屼綘寰楄嚜宸辫礋璐ｈ繖浜涙楠ゃ€備互涓嬪懡浠ゅ簲褰撲氦浜掑紡鍦?绉婚櫎涓€涓唴鏍哥殑涓や釜涓昏鏂囦欢锛?
```
    rm -i /boot/{System.map,vmlinuz}-6.0.1-foobar
```

鐜板湪绉婚櫎瀵瑰簲鐨?initramfs锛屽畠閫氬父鍚嶄负绫讳技 `/boot/initramfs-6.0.1-foobar.img` 鎴?`/boot/initrd.img-6.0.1-foobar`銆備箣鍚庢鏌?/boot/ 涓枃浠跺悕鍖呭惈 '6.0.1-foobar' 鐨勫叾浠栨枃浠跺苟涓€骞跺垹闄ゃ€?鐜板湪浠庝綘鐨?bootloader 閰嶇疆涓Щ闄よ鍐呮牳銆?
娉ㄦ剰锛屾墜鍔ㄥ垹闄ゅ唴鏍哥殑鏂囦欢鎴栫洰褰曟椂锛屽鍍?'*' 杩欐牱鐨勯€氶厤绗﹁闈炲父灏忓績锛氬綋浣犲彧鎯冲垹闄?6.0 鎴?6.0.1 鏃讹紝
浣犲彲鑳戒細鎰忓鍒犻櫎 6.0.11 鍐呮牳鐨勬枃浠躲€?
[back to step-by-step guide <uninstall_sbs>]


## FAQ


### 涓轰粈涔堣繖涓?how-to"鍦ㄦ垜鐨勭郴缁熶笂涓嶅伐浣滐紵


濡備竴寮€濮嬫墍杩帮紝鏈寚鍗?鏃ㄥ湪瑕嗙洊鍦ㄥ晢鍝?PC 鎴栨湇鍔″櫒纭欢涓婅繍琛岀殑涓绘祦 Linux 鍙戣鐗堜笂鏋勫缓鍐呮牳閫氬父鎵€闇€鐨勪竴鍒?銆?灏界濡傛锛屾墍姒傝堪鐨勬柟娉曞湪寰堝鍏朵粬璁剧疆涓婁篃搴斿綋鑳藉伐浣溿€備絾璇曞浘鍦ㄤ竴浠芥寚鍗椾腑瑕嗙洊姣忎釜鍙兘鐨勭敤渚嬩細杩濊儗鍏剁洰鐨勶紝
鍥犱负娌℃湁杩欐牱鐨勮仛鐒︼紝浣犲皢闇€瑕佸嚑鍗佹潯鎴栧嚑鐧炬潯绫讳技"濡傛灉浣犳湁 <鏌愭満鍣ㄦ垨鍙戣鐗?锛屼綘鍦ㄦ澶勫繀椤诲仛
<杩欎釜鍜岄偅涓? <instead|additionally>"杩欐牱鐨勬瀯閫犮€傚叾涓瘡涓€鏉￠兘浼氳鏂囨湰鏇撮暱銆佹洿澶嶆潅銆佹洿闅捐窡闅忋€?
璇濊櫧濡傛锛氳繖褰撶劧鏄竴涓潈琛°€傚洜姝わ紝濡傛灉浣犺涓轰竴涓澶栫殑鐢ㄤ緥鍊煎緱鎻忚堪锛岃鎸変笂鏂?<submit_improvements_qbtl>
鎵€杩版妸瀹冨缓璁粰鏈枃妗ｇ殑缁存姢鑰呫€?

..
   end-of-content
..
   This document is maintained by Thorsten Leemhuis <linux@leemhuis.info>. If
   you spot a typo or small mistake, feel free to let him know directly and
   he'll fix it. You are free to do the same in a mostly informal way if you
   want to contribute changes to the text -- but for copyright reasons please CC
   linux-doc@vger.kernel.org and 'sign-off' your contribution as
   Documentation/process/submitting-patches.rst explains in the section 'Sign
   your work - the Developer's Certificate of Origin'.
..
   This text is available under GPL-2.0+ or CC-BY-4.0, as stated at the top
   of the file. If you want to distribute this text under CC-BY-4.0 only,
   please use 'The Linux kernel development community' for author attribution
   and link this as source:
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/quickly-build-trimmed-linux.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.
