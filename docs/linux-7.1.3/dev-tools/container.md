
# 瀹瑰櫒鍖栨瀯寤?


`container` 宸ュ叿鍙敤浜庡湪鍐呮牳婧愮爜鏍戜腑锛屼粠瀹瑰櫒鍐呴儴杩愯浠绘剰鍛戒护銆傝繖鏍峰仛鏈夊姪浜庤法鍚勭骞冲彴澶嶇幇鏋勫缓杩囩▼锛屼緥濡傚綋鏌愪釜娴嬭瘯鏈哄櫒浜烘姤鍛婁簡涓€涓渶瑕佺壒瀹氱増鏈紪璇戝櫒鎴栧閮ㄦ祴璇曞浠舵墠鑳藉鐜扮殑闂鏃躲€傝櫧鐒剁啛鎮夊鍣ㄧ殑鐢ㄦ埛宸茬粡鍙互鍋氬埌杩欎竴鐐癸紝浣嗗湪鍐呮牳鏍戜腑鎻愪緵涓€涓笓鐢ㄥ伐鍏凤紝鍙互閫氳繃涓€鍔虫案閫稿湴瑙ｅ喅甯歌闂锛堜緥濡傜敤鎴?id 绠＄悊锛夋潵闄嶄綆浣跨敤闂ㄦ銆傚畠杩樹娇寰楀叡浜兘澶熶骇鐢熺壒瀹氱粨鏋滅殑绮剧‘鍛戒护琛屽彉寰楁洿鍔犲鏄撱€備富瑕佺敤渚嬪緢鍙兘鏄唴鏍告瀯寤猴紝浣嗗嚑涔庝换浣曚笢瑗块兘鍙互杩愯锛欿Unit銆乧heckpatch 绛夛紝鍙瀛樺湪鍚堥€傜殑闀滃儚鍗冲彲銆?


## 閫夐」


```

  scripts/container -i IMAGE [OPTION]... CMD...

```
鍙敤閫夐」锛?

## `-e, --env-file ENV_FILE`

    瑕佸湪瀹瑰櫒鍐呭姞杞界殑鐜鏂囦欢璺緞銆?

## `-g, --gid GID`

    鍦ㄥ鍣ㄥ唴浣跨敤鐨勭粍 id銆?

## `-i, --image IMAGE`

    瀹瑰櫒闀滃儚鍚嶇О锛堝繀濉級銆?

## `-r, --runtime RUNTIME`

    瀹瑰櫒杩愯鏃跺悕绉般€傛敮鎸佺殑杩愯鏃讹細`docker`銆乣podman`銆?

    濡傛灉鏈寚瀹氾紝灏嗕娇鐢ㄧ郴缁熶腑鎵惧埌鐨勭涓€涓紝鍗充紭鍏堜娇鐢?Podman锛屽惁鍒欎娇鐢?Docker銆?

## `-s, --shell`

    浠ヤ氦浜掑紡 shell 杩愯瀹瑰櫒銆?

## `-u, --uid UID`

    鍦ㄥ鍣ㄥ唴浣跨敤鐨勭敤鎴?id銆?

    濡傛灉鏈寚瀹?`-g` 閫夐」锛屽垯璇ョ敤鎴?id 涔熷皢鐢ㄤ簬缁?id銆?

## `-v, --verbose`

    鍚敤璇︾粏杈撳嚭銆?

## `-h, --help`

    鏄剧ず甯姪淇℃伅骞堕€€鍑恒€?


## 鐢ㄦ硶


閫夋嫨浣跨敤鍝釜闀滃儚瀹屽叏鐢辩敤鎴峰喅瀹氾紝鑰?`CMD` 鍙傛暟浼氫綔涓鸿鍦ㄥ鍣ㄥ唴杩愯鐨勪换鎰忓懡浠よ鐩存帴浼犲叆銆傝宸ュ叿璐熻矗灏嗘簮鐮佹爲鎸傝浇涓哄綋鍓嶅伐浣滅洰褰曪紝骞舵牴鎹渶瑕佽皟鏁寸敤鎴峰拰缁?id銆?

閫氬父鐢辩敤鎴锋彁渚涚殑銆佸寘鍚紪璇戝櫒宸ュ叿閾剧殑瀹瑰櫒闀滃儚閫氳繃 `-i` 閫夐」閫夋嫨銆傚鍣ㄨ繍琛屾椂鍙互閫氳繃 `-r` 閫夐」閫夋嫨锛屽彲浠ユ槸 `docker` 鎴?`podman`銆傚鏋滄湭鎸囧畾锛屽皢浣跨敤绯荤粺涓壘鍒扮殑绗竴涓紝骞朵紭鍏堜娇鐢?Podman銆傚鍏朵粬杩愯鏃剁殑鏀寔鍙兘浼氭牴鎹敤鎴蜂腑鐨勬祦琛岀▼搴﹀湪鏃ュ悗鍔犲叆銆?

榛樿鎯呭喌涓嬶紝鍛戒护浠ラ潪浜や簰鏂瑰紡杩愯銆傜敤鎴峰彲浠ラ€氳繃 SIGINT锛圕trl-C锛変腑姝㈡鍦ㄨ繍琛岀殑瀹瑰櫒銆傝浠ュ甫 TTY 鐨勪氦浜掓柟寮忚繍琛屽懡浠わ紝鍙互浣跨敤 `--shell` 鎴?`-s` 閫夐」銆傛鏃朵俊鍙峰皢鐢?shell 鐩存帴鎺ユ敹锛岃€屼笉鏄敱鐖?`container` 杩涚▼鎺ユ敹銆傝閫€鍑轰氦浜掑紡 shell锛岃浣跨敤 Ctrl-D 鎴?`exit`銆?


   闄ゅ鍣ㄨ繍琛屾椂澶栵紝鍞竴鐨勫涓绘満瑕佹眰鏄?Python 3.10 鎴栨洿楂樼増鏈€?


   鏍戝鏋勫缓灏氭湭瀹屽叏鏀寔銆備笉杩囷紝`O=` 閫夐」宸茬粡鍙互涓庢簮鐮佹爲鍐呯殑鐩稿璺緞涓€璧蜂娇鐢紝浠ヤ繚鐣欑浉浜掔嫭绔嬬殑鏋勫缓杈撳嚭銆傚湪鏍戝鏋勫缓鐨勪竴绉嶅彉閫氭柟娉曟槸浣跨敤 `mount --bind`锛岃鍙傞槄涓嬫枃鐨勭ず渚嬮儴鍒嗐€?


## 鐜鍙橀噺


鐜鍙橀噺涓嶄細浼犳挱鍒板鍣ㄤ腑锛屽洜姝ゅ繀椤婚€氳繃闀滃儚鏈韩瀹氫箟锛屾垨閫氳繃 `-e` 閫夐」浣跨敤鐜鏂囦欢鏉ュ畾涔夈€傚湪鏌愪簺鎯呭喌涓嬶紝灏嗗畠浠畾涔夊湪鐢ㄤ簬鍒涘缓闀滃儚鐨?Containerfile 涓洿鏈夋剰涔夈€備緥濡傦紝浠呭惈 Clang 鐨勭紪璇戝櫒宸ュ叿閾鹃暅鍍忓彲鑳戒細瀹氫箟 `LLVM=1`銆?

鏈湴鐜鏂囦欢鏇撮€傚悎浜庡湪寮€鍙戣繃绋嬩腑娣诲姞鐨勭敤鎴风壒瀹氬彉閲忋€傚畠浼氭寜鍘熸牱浼犻€掔粰瀹瑰櫒杩愯鏃讹紝鍥犳鍏舵牸寮?
```

  INSTALL_MOD_STRIP=1
  SOME_RANDOM_TEXT=One upon a time

```
鍙﹁娉ㄦ剰锛宍make` 閫夐」浠嶇劧鍙互浼犻€掔粰鍛戒护琛岋紝鍥犳铏界劧杩欐棤娉曞畬鎴愶紝鍥犱负绗竴涓弬鏁板繀椤绘槸
```

  scripts/container -i docker.io/tuxmake/korg-clang LLVM=1 make  # won't work

```
```

  scripts/container -i docker.io/tuxmake/korg-clang make LLVM=1


```
鐢ㄦ埛 ID


杩欎竴棰嗗煙鐨勮涓轰細鍥犲鍣ㄨ繍琛屾椂鑰岀暐鏈変笉鍚屻€傜洰鏍囨槸浣滀负璋冪敤璇ュ伐鍏风殑鐢ㄦ埛鏉ヨ繍琛屽懡浠ゃ€傚湪 Podman 涓嬶紝浼氬垱寤轰竴涓懡鍚嶇┖闂达紝灏嗗綋鍓嶇敤鎴?id 鏄犲皠涓哄鍣ㄥ唴鐨勫彟涓€涓?id锛堥粯璁や负 1000锛夈€傚湪 Docker 涓嬶紝铏界劧杩戞湡鐗堟湰涔熷彲浠ュ仛鍒拌繖涓€鐐癸紝浣嗗畠闇€瑕佸畧鎶よ繘绋嬩腑鍚敤涓€椤圭壒娈婄壒鎬э紝鍥犳涓虹畝鍗曡捣瑙佽繖閲屽苟鏈娇鐢ㄣ€傜浉鍙嶏紝瀹瑰櫒鐩存帴浣跨敤褰撳墠鐢ㄦ埛 id 杩愯銆傚湪杩欎袱绉嶆儏鍐典笅锛岃繖閮戒細涓轰互鍗峰舰寮忔寕杞界殑鍐呮牳婧愮爜鏍戞彁渚涚浉鍚岀殑鏂囦欢鏉冮檺銆傚敮涓€鐨勫尯鍒槸锛屽湪浣跨敤涓嶅甫鍛藉悕绌洪棿鐨?Docker 鏃讹紝鐢ㄦ埛 id 鍙兘涓庨暅鍍忎腑璁剧疆鐨勯粯璁?id 涓嶅悓銆?

鍋囪鎴戜滑浣跨敤涓€涓缃簡榛樿鐢ㄦ埛 id 1000 鐨勯暅鍍忥紝鑰屽綋鍓嶈皟鐢?`container` 宸ュ叿鐨勭敤鎴?id 涓?1234銆傚唴鏍告簮鐮佹爲鐢卞悓涓€鐢ㄦ埛妫€鍑猴紝鍥犳杩欎簺鏂囦欢灞炰簬鐢ㄦ埛 1234銆傚湪 Podman 涓嬶紝瀹瑰櫒灏嗕互鐢ㄦ埛 id 1000 杩愯锛屽苟鏄犲皠鍥?id 1234锛屼娇寰楁寕杞藉嵎涓殑鏂囦欢鍦ㄥ鍣ㄥ唴鐪嬭捣鏉ュ睘浜?id 1000銆傚湪浣跨敤涓嶅甫鍛藉悕绌洪棿鐨?Docker 鏃讹紝瀹瑰櫒灏嗕互鐢ㄦ埛 id 1234 杩愯锛屽畠鍙互璁块棶鍗蜂腑鐨勬枃浠讹紝浣嗘棤娉曡闂敤鎴?1000 鐨勪富鐩綍銆傚綋鍙湪鍐呮牳鏍戜腑杩愯鍛戒护鏃惰繖涓嶅簲鎴愪负闂锛屼絾杩欓噷鍊煎緱寮鸿皟锛屽洜涓哄畠鍙兘瀵圭壒娈婅竟缂樻儏鍐典骇鐢熷奖鍝嶃€?


   Podman 鍦?Podman 鍚庣涔嬩笂杩愯 `docker` 鍛戒护鐨?`Docker 鍏煎鎬?<https://podman-desktop.io/docs/migrating-from-docker/managing-docker-compatibility>`__ 妯″紡鏇翠负澶嶆潅锛屼笖灏氭湭瀹屽叏鏀寔銆傚洜姝わ紝濡傛灉绯荤粺涓袱绉嶈繍琛屾椂閮藉彲鐢紝Podman 灏嗕紭鍏堛€?


## 绀轰緥


TuxMake 椤圭洰鍦?`Docker Hub <https://hub.docker.com/u/tuxmake>`__ 涓婃彁渚涗簡涓€绯诲垪鍙敤鐨勯鏋勫缓瀹瑰櫒闀滃儚銆備互涓嬫槸鏈€鐭殑褰㈠紡
```

  scripts/container -i docker.io/tuxmake/korg-clang -- make LLVM=1 defconfig
  scripts/container -i docker.io/tuxmake/korg-clang -- make LLVM=1 -j$(nproc)

```

   鍦ㄥ鍣ㄥ唴杩愯甯﹂€夐」鐨勫懡浠ゆ椂锛屽簲褰撶敤鍙岀牬鎶樺彿 `--` 灏嗗叾涓?`container` 宸ュ叿閫夐」鍒嗛殧寮€锛屼互閬垮厤灏嗗畠浠贩娣嗐€備笉甯﹂€夐」鐨勬櫘閫氬懡浠ゅ苟涓嶄弗鏍?
```

     scripts/container -i docker.io/tuxmake/korg-clang make mrproper

```
```

  scripts/container -i perl:slim-trixie scripts/checkpatch.pl patches/*

```
浣滀负 TuxMake 闀滃儚鐨勬浛浠ｏ紝涓嬮潰鐨勭ず渚嬪紩鐢ㄤ簡 `kernel.org` 闀滃儚锛屽畠浠熀浜?`kernel.org 缂栬瘧鍣ㄥ伐鍏烽摼 <https://mirrors.edge.kernel.org/pub/tools/>`__銆傝繖浜涢暅鍍忓皻鏈紙鐩墠锛夊湪浠讳綍鍏叡娉ㄥ唽琛ㄤ腑姝ｅ紡鎻愪緵锛屼絾鐢ㄦ埛鍙互鏀逛负浣跨敤姝?`瀹為獙鎬т粨搴?<https://gitlab.com/gtucker/korg-containers>`__锛岄€氳繃杩愯 ``make PREFIX=kernel.org/`` 鍦ㄦ湰鍦拌嚜琛屾瀯寤恒€?

```

  scripts/container -i kernel.org/clang -- make bzImage -j$(nproc)

```
```

  scripts/container -i kernel.org/gcc:15 -- make bzImage -j$(nproc)

```
瀵逛簬鏍戝鏋勫缓锛屼竴涓妧宸ф槸灏嗙洰鏍囩洰褰曠粦瀹氭寕杞藉埌
```

  mkdir -p $HOME/tmp/my-kernel-build
  mkdir -p build
  sudo mount --bind $HOME/tmp/my-kernel-build build
  scripts/container -i kernel.org/gcc -- make mrproper
  scripts/container -i kernel.org/gcc -- make O=build defconfig
  scripts/container -i kernel.org/gcc -- make O=build -j$(nproc)

```
```

  scripts/container -s -i kernel.org/gcc:kunit -- \
      tools/testing/kunit/kunit.py \
          run \
          --arch=x86_64 \
          --cross_compile=x86_64-linux-

```
```

  scripts/container -si kernel.org/gcc bash

```
瑕佹瀯寤洪渶瑕?`kdocs` 闀滃儚鐨?HTML 鏂囨。锛?

```

  scripts/container -i kernel.org/kdocs make htmldocs

```