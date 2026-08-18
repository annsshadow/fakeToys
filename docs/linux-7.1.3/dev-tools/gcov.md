# 灏?gcov 鐢ㄤ簬 Linux 鍐呮牳


gcov 鎬ц兘鍒嗘瀽锛坧rofiling锛夊唴鏍告敮鎸佷娇寰楀彲浠ュ皢 GCC 鐨勮鐩栫巼娴嬭瘯宸ュ叿 gcov_ 鐢ㄤ簬 Linux 鍐呮牳銆傝繍琛屽唴鏍哥殑瑕嗙洊鐜囨暟鎹互 gcov 鍏煎鐨勬牸寮忛€氳繃 鈥済cov鈥?debugfs 鐩綍瀵煎嚭銆傝鑾峰彇鐗瑰畾鏂囦欢鐨勮鐩栫巼鏁版嵁锛岃鍒囨崲鍒板唴鏍告瀯寤?
```

    # cd /tmp/linux-out
    # gcov -o /sys/kernel/debug/gcov/tmp/linux-out/kernel spinlock.c

```
杩欏皢鍦ㄥ綋鍓嶇洰褰曚腑鍒涘缓甯︽湁鎵ц娆℃暟鏍囨敞鐨勬簮浠ｇ爜鏂囦欢銆傛澶栵紝涔熷彲浠ヤ娇鐢?lcov_ 绛夊浘褰㈠寲 gcov 鍓嶇鏉ヨ嚜鍔ㄥ寲鏀堕泦鏁翠釜鍐呮牳鏁版嵁鐨勮繃绋嬶紝骞舵彁渚?HTML 鏍煎紡鐨勮鐩栫巼姒傝銆?

## 鍙兘鐨勭敤閫旓細

- 璋冭瘯锛堣繖涓€琛屾槸鍚﹁鎵ц杩囷紵锛?
- 鏀硅繘娴嬭瘯锛堝浣曚慨鏀规祴璇曚互瑕嗙洊杩欎簺琛岋紵锛?
- 绮剧畝鍐呮牳閰嶇疆锛堝鏋滅浉鍏充唬鐮佷粠鏈繍琛岋紝鎴戞槸鍚﹁繕闇€瑕佽閫夐」锛燂級



## 鍑嗗宸ヤ綔


```

        CONFIG_DEBUG_FS=y
        CONFIG_GCOV_KERNEL=y

```
```

        CONFIG_GCOV_PROFILE_ALL=y

```
璇锋敞鎰忥紝浣跨敤鎬ц兘鍒嗘瀽鏍囧織缂栬瘧鐨勫唴鏍镐細鏄庢樉鏇村ぇ涓旇繍琛屾洿鎱€傛澶栵紝骞堕潪鎵€鏈夋灦鏋勯兘鏀寔 CONFIG_GCOV_PROFILE_ALL銆?

鍙湁鍦?debugfs 琚寕杞藉悗锛屾€ц兘鍒嗘瀽鏁版嵁鎵嶄細鍙樺緱鍙闂€?
```

        mount -t debugfs none /sys/kernel/debug


```
瀹氬埗


瑕侀拡瀵圭壒瀹氭枃浠舵垨鐩綍鍚敤鎬ц兘鍒嗘瀽锛岃鍦ㄧ浉搴旂殑鍐呮牳 Makefile 涓坊鍔犱竴琛岀被浼间簬浠ヤ笅鍐呭锛?

```

	GCOV_PROFILE_main.o := y

```
```

	GCOV_PROFILE := y

```
鍗充娇鍚敤浜?CONFIG_GCOV_PROFILE_ALL锛屼篃瑕佸皢鏌愪簺鏂囦欢鎺掗櫎鍦ㄦ€ц兘鍒嗘瀽涔嬪
```

	GCOV_PROFILE_main.o := n

```
```

	GCOV_PROFILE := n

```
璇ユ満鍒朵粎鏀寔閾炬帴鍒颁富鍐呮牳鏄犲儚鎴栫紪璇戜负鍐呮牳妯″潡鐨勯偅浜涙枃浠躲€?


## 妯″潡鐗瑰畾鐨勯厤缃?


## 涓嬮潰鎻忚堪浜嗛拡瀵圭壒瀹氭ā鍧楃殑 gcov 鍐呮牳閰嶇疆锛?

CONFIG_GCOV_PROFILE_RDS锛?
        鍦?RDS 涓婂惎鐢?GCOV 鎬ц兘鍒嗘瀽锛岀敤浜庢鏌ュ摢浜涘嚱鏁版垨琛岃鎵ц銆傝閰嶇疆琚?rds 鑷祴璇曠敤浜庣敓鎴愯鐩栫巼鎶ュ憡銆傚鏋滄湭璁剧疆锛屽垯鐪佺暐璇ユ姤鍛娿€?


## 鏂囦欢


## gcov 鍐呮牳鏀寔鍦?debugfs 涓垱寤轰互涓嬫枃浠讹細

`/sys/kernel/debug/gcov`
	鎵€鏈?gcov 鐩稿叧鏂囦欢鐨勭埗鐩綍銆?

`/sys/kernel/debug/gcov/reset`
	鍏ㄥ眬閲嶇疆鏂囦欢锛氬悜鍏跺啓鍏ユ椂浼氬皢鎵€鏈夎鐩栫巼鏁版嵁閲嶇疆涓洪浂銆?

`/sys/kernel/debug/gcov/path/to/compile/dir/file.gcda`
	gcov 宸ュ叿鎵€鑳借瘑鍒殑瀹為檯 gcov 鏁版嵁鏂囦欢銆傚悜鍏跺啓鍏ユ椂浼氬皢璇ユ枃浠剁殑瑕嗙洊鐜囨暟鎹噸缃负闆躲€?

`/sys/kernel/debug/gcov/path/to/compile/dir/file.gcno`
	gcov 宸ュ叿鎵€闇€鐨勯潤鎬佹暟鎹枃浠剁殑绗﹀彿閾炬帴銆傝鏂囦欢鐢?gcc 鍦ㄩ厤鍚?`-ftest-coverage` 閫夐」缂栬瘧鏃剁敓鎴愩€?


## 妯″潡


鍐呮牳妯″潡鍙兘鍖呭惈浠呭湪妯″潡鍗歌浇鏃惰繍琛岀殑娓呯悊浠ｇ爜銆俫cov 鏈哄埗閫氳繃淇濈暀涓庡凡鍗歌浇妯″潡鐩稿叧鑱旂殑鏁版嵁鍓湰锛屾彁渚涗簡涓€绉嶆敹闆嗘绫讳唬鐮佽鐩栫巼鏁版嵁鐨勬墜娈点€傝繖浜涙暟鎹€氳繃 debugfs 淇濇寔鍙敤銆備竴鏃︽ā鍧楀啀娆″姞杞斤紝鐩稿叧鐨勮鐩栫巼璁℃暟鍣ㄤ細鐢ㄥ叾涓婁竴娆″疄渚嬪寲鐨勬暟鎹繘琛屽垵濮嬪寲銆?

閫氳繃鍦ㄨ繍琛屾椂鎸囧畾 gcov_persist 鍙傛暟鍙互鍋滅敤姝よ涓恒€?
```

        gcov_persist=0

```
鍦ㄨ繍琛屾椂锛岀敤鎴蜂篃鍙互閫氳繃鍐欏叆鍏舵暟鎹枃浠舵垨鍏ㄥ眬閲嶇疆鏂囦欢锛屾潵閫夋嫨涓㈠純鏌愪釜宸插嵏杞芥ā鍧楃殑鏁版嵁銆?


## 鏋勫缓鏈轰笌娴嬭瘯鏈哄垎绂荤殑鎯呭喌


gcov 鍐呮牳鎬ц兘鍒嗘瀽鍩虹璁炬柦鐨勮璁″垵琛锋槸璁╁唴鏍稿湪鍚屼竴鍙版満鍣ㄤ笂鏋勫缓鍜岃繍琛岀殑寮€绠卞嵆鐢ㄥ満鏅€傚鏋滃唴鏍歌繍琛屽湪鍙︿竴鍙扮嫭绔嬬殑鏈哄櫒涓婏紝鍒欏繀椤绘牴鎹?gcov 宸ュ叿鐨勪娇鐢ㄤ綅缃仛鍑虹壒娈婂噯澶囷細


## a) gcov 鍦ㄦ祴璇曟満涓婅繍琛?

    The gcov tool version on the test machine must be compatible with the
    gcc version used for kernel build. Also the following files need to be
    copied from build to test machine:

    鏉ヨ嚜婧愮爜鏍戯細
      - 鎵€鏈?C 婧愭枃浠朵笌澶存枃浠?

    鏉ヨ嚜鏋勫缓鏍戯細
      - 鎵€鏈?C 婧愭枃浠朵笌澶存枃浠?
      - 鎵€鏈?.gcda 涓?.gcno 鏂囦欢
      - 鎵€鏈夋寚鍚戠洰褰曠殑绗﹀彿閾炬帴

    闇€瑕佹敞鎰忕殑鏄紝杩欎簺鏂囦欢蹇呴』鏀剧疆鍦ㄦ祴璇曟満涓婁笌鏋勫缓鏈哄畬鍏ㄧ浉鍚岀殑鏂囦欢绯荤粺浣嶇疆銆傚鏋滀换浣曡矾寰勭粍浠舵槸绗﹀彿閾炬帴锛屽垯蹇呴』浣跨敤瀹為檯鐨勭洰褰曪紙杩欐槸鐢变簬 make 瀵?CURDIR 鐨勫鐞嗘柟寮忥級銆?


## b) gcov 鍦ㄦ瀯寤烘満涓婅繍琛?

## 姣忔娴嬭瘯鐢ㄤ緥鎵ц鍚庯紝闇€瑕佷互涓嬫枃浠朵粠娴嬭瘯鏈哄鍒跺埌鏋勫缓鏈猴細

    鏉ヨ嚜 sysfs 涓殑 gcov 鐩綍锛?
      - 鎵€鏈?.gcda 鏂囦欢
      - 鎵€鏈夋寚鍚?.gcno 鏂囦欢鐨勯摼鎺?

    杩欎簺鏂囦欢鍙互澶嶅埗鍒版瀯寤烘満涓婄殑浠绘剰浣嶇疆銆傞殢鍚庡繀椤讳娇鐢?-o 閫夐」鎸囧悜璇ョ洰褰曟潵璋冪敤 gcov銆?

```

      /tmp/linux:    kernel source tree
      /tmp/out:      kernel build directory as specified by make O=
      /tmp/coverage: location of the files copied from the test machine

      [user@build] cd /tmp/out
      [user@build] gcov -o /tmp/coverage/tmp/out/init main.c


```
鍏充簬缂栬瘧鍣ㄧ殑璇存槑


GCC 涓?LLVM 鐨?gcov 宸ュ叿涓嶄竴瀹氬吋瀹广€傝浣跨敤 gcov_ 鏉ュ鐞?GCC 鐢熸垚鐨?.gcno 涓?.gcda 鏂囦欢锛屼娇鐢?llvm-cov_ 鏉ュ鐞?Clang銆?


GCC 涓?Clang 鐨?gcov 鍦ㄦ瀯寤轰笂鐨勫樊寮傜敱 Kconfig 澶勭悊銆傚畠浼氭牴鎹娴嬪埌鐨勫伐鍏烽摼鑷姩閫夋嫨鍚堥€傜殑 gcov 鏍煎紡銆?


## 鏁呴殰鎺掓煡


闂
    缂栬瘧鍦ㄩ摼鎺ュ櫒姝ラ鏈熼棿涓銆?

鍘熷洜
    涓洪偅浜涙湭閾炬帴鍒颁富鍐呮牳銆佹垨閫氳繃鑷畾涔夐摼鎺ヨ繃绋嬮摼鎺ョ殑婧愭枃浠舵寚瀹氫簡鎬ц兘鍒嗘瀽鏍囧織銆?

瑙ｅ喅鏂规
    閫氳繃鍦ㄧ浉搴旂殑 Makefile 涓寚瀹?`GCOV_PROFILE := n` 鎴?`GCOV_PROFILE_basename.o := n`锛屽皢鍙楀奖鍝嶇殑婧愭枃浠舵帓闄ゅ湪鎬ц兘鍒嗘瀽涔嬪銆?

闂
    浠?sysfs 澶嶅埗鐨勬枃浠舵樉绀轰负绌烘垨涓嶅畬鏁淬€?

鍘熷洜
    鐢变簬 seq_file 鐨勫伐浣滄柟寮忥紝鏌愪簺宸ュ叿锛堝 cp 鎴?tar锛夊彲鑳芥棤娉曟纭鍒?sysfs 涓殑鏂囦欢銆?

瑙ｅ喅鏂规
    浣跨敤 `cat` 璇诲彇 `.gcda` 鏂囦欢锛屼娇鐢?`cp -d` 澶嶅埗閾炬帴銆備篃鍙互浣跨敤闄勫綍 B 鎵€绀虹殑鏈哄埗銆?


## 闄勫綍 A锛歡ather_on_build.sh


鐢ㄤ簬鍦ㄦ瀯寤烘満涓婃敹闆嗚鐩栫巼鍏冩枃浠剁殑绀轰緥鑴氭湰
锛堝弬瑙佲€滄瀯寤烘満涓庢祴璇曟満鍒嗙鐨勬儏鍐碘€?a. <gcov-test>锛夛細


    #!/bin/bash

    KSRC=$1
    KOBJ=$2
    DEST=$3

    if [ -z "$KSRC" ] || [ -z "$KOBJ" ] || [ -z "$DEST" ]; then
      echo "Usage: $0 <ksrc directory> <kobj directory> <output.tar.gz>" >&2
      exit 1
    fi

    KSRC=$(cd $KSRC; printf "all:\n\t@echo \${CURDIR}\n" | make -f -)
    KOBJ=$(cd $KOBJ; printf "all:\n\t@echo \${CURDIR}\n" | make -f -)

    find $KSRC $KOBJ \( -name '**.gcno' -o -name '**.[ch]' -o -type l \) -a \
                     -perm /u+r,g+r | tar cfz $DEST -P -T -

    if [ $? -eq 0 ] ; then
      echo "$DEST successfully created, copy to test system and unpack with:"
      echo "  tar xfz $DEST -P"
    else
      echo "Could not create file $DEST"
    fi


## 闄勫綍 B锛歡ather_on_test.sh


鐢ㄤ簬鍦ㄦ祴璇曟満涓婃敹闆嗚鐩栫巼鏁版嵁鏂囦欢鐨勭ず渚嬭剼鏈?
锛堝弬瑙佲€滄瀯寤烘満涓庢祴璇曟満鍒嗙鐨勬儏鍐碘€?b. <gcov-build>锛夛細



    #!/bin/bash -e

    DEST=$1
    GCDA=/sys/kernel/debug/gcov

    if [ -z "$DEST" ] ; then
      echo "Usage: $0 <output.tar.gz>" >&2
      exit 1
    fi

    TEMPDIR=$(mktemp -d)
    echo Collecting data..
    find $GCDA -type d -exec mkdir -p $TEMPDIR/\{\} \;
    find $GCDA -name '*.gcda' -exec sh -c 'cat < $0 > '$TEMPDIR'/$0' {} \;
    find $GCDA -name '*.gcno' -exec sh -c 'cp -d $0 '$TEMPDIR'/$0' {} \;
    tar czf $DEST -C $TEMPDIR sys
    rm -rf $TEMPDIR

    echo "$DEST successfully created, copy to build system and unpack with:"
    echo "  tar xfz $DEST"
