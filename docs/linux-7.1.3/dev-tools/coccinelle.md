


## Coccinelle


Coccinelle 鏄竴涓敤浜庢ā寮忓尮閰嶅拰鏂囨湰杞崲鐨勫伐鍏凤紝鍦ㄥ唴鏍稿紑鍙戜腑鏈夎澶氱敤閫旓紝鍖呮嫭搴旂敤澶嶆潅鐨勩€佹爲
鑼冨洿鐨勮ˉ涓侊紝浠ュ強妫€娴嬫湁闂鐨勭紪绋嬫ā寮忋€?
### 鑾峰彇 Coccinelle


鍐呮牳涓寘鍚殑璇箟琛ヤ竵锛坰emantic patch锛変娇鐢ㄤ簡鐢?Coccinelle 1.0.0-rc11 鍙婃洿楂樼増鏈彁渚涚殑鐗规€?鍜岄€夐」銆備娇鐢ㄦ洿鏃╃殑鐗堟湰浼氬け璐ワ紝鍥犱负 Coccinelle 鏂囦欢鍜?coccicheck 鎵€浣跨敤鐨勯€夐」鍚嶇О宸茬粡鏇存柊銆?
Coccinelle 鍙€氳繃璁稿鍙戣鐗堢殑鍖呯鐞嗗櫒鑾峰彇锛屼緥濡傦細

 - Debian
 - Fedora
 - Ubuntu
 - OpenSUSE
 - Arch Linux
 - Gentoo
 - NetBSD
 - FreeBSD

涓€浜涘彂琛岀増鎵撶殑鍖呭凡缁忚繃鏃讹紝寤鸿浣跨敤浠?Coccinelle 涓婚〉鍙戝竷鐨勬渶鏂扮増鏈細

https://coccinelle.gitlabpages.inria.fr/website

鎴栦粠 Github 鑾峰彇锛?
https://github.com/coccinelle/coccinelle

```

        ./autogen
        ./configure
        make

```

```

        sudo make install

```
浠庢簮鐮佹瀯寤虹殑鏇磋缁嗗畨瑁呰鏄庡彲浠ュ湪浠ヤ笅浣嶇疆鎵惧埌锛?
https://github.com/coccinelle/coccinelle/blob/master/install.txt

### 琛ュ厖鏂囨。


鍏充簬琛ュ厖鏂囨。锛岃鍙傞槄 wiki锛?
https://bottest.wiki.kernel.org/coccicheck.html

wiki 鏂囨。濮嬬粓鎸囧悜璇ヨ剼鏈殑 linux-next 鐗堟湰銆?
鍏充簬璇箟琛ヤ竵璇█锛圫mPL锛孲emantic Patch Language锛夎娉曟枃妗ｏ紝璇峰弬闃咃細

https://coccinelle.gitlabpages.inria.fr/website/docs/main_grammar.html

### 鍦?Linux 鍐呮牳涓婁娇鐢?Coccinelle


椤跺眰 Makefile 涓畾涔変簡涓€涓?Coccinelle 涓撶敤鐨勭洰鏍囥€傝鐩爣鍚嶄负 `coccicheck`锛屽畠浼氳皟鐢?`scripts`
鐩綍涓殑 `coccicheck` 鍓嶇銆?
瀹氫箟浜嗗洓绉嶅熀鏈ā寮忥細`patch`銆乣report`銆乣context` 鍜?`org`銆傝浣跨敤鐨勬ā寮忛€氳繃 `MODE=<mode>`
璁剧疆 MODE 鍙橀噺鏉ユ寚瀹氥€?
- `patch` 鍦ㄥ彲鑳界殑鎯呭喌涓嬫彁鍑轰竴涓慨澶嶃€?
- `report` 鐢熸垚濡備笅鏍煎紡鐨勬姤鍛婏細
  file:line:column-column: message锛堟枃浠?琛?鍒?鍒? 娑堟伅锛?
- `context` 浠ョ被浼?diff 鐨勯鏍奸珮浜劅鍏磋叮鐨勮鍙婂叾涓婁笅鏂囥€傛劅鍏磋叮鐨勮鐢?`-` 鏍囩ず銆?
- `org` 鐢熸垚 Emacs 鐨?Org mode 鏍煎紡鐨勬姤鍛娿€?
璇锋敞鎰忥紝骞堕潪鎵€鏈夎涔夎ˉ涓侀兘瀹炵幇浜嗘墍鏈夋ā寮忋€備负浜嗕究浜庝娇鐢?Coccinelle锛岄粯璁ゆā寮忔槸 鈥渞eport鈥濄€?
鍙︽湁涓ょ妯″紡鎻愪緵浜嗚繖浜涙ā寮忕殑甯歌缁勫悎銆?
- `chain` 鎸変笂杩伴『搴忓皾璇曞墠闈㈢殑妯″紡锛岀洿鍒板叾涓竴涓垚鍔熴€?
- `rep+ctxt` 渚濇杩愯 report 妯″紡鍜?context 妯″紡銆傚畠搴斾笌 C 閫夐」锛堜笅鏂囨弿杩帮級涓€璧蜂娇鐢紝璇ラ€夐」浠?  鏂囦欢涓哄崟浣嶆鏌ヤ唬鐮併€?
#### 绀轰緥


```

		make coccicheck MODE=report

```

```

		make coccicheck MODE=patch


```
coccicheck 鐩爣浼氭妸 `scripts/coccinelle` 瀛愮洰褰曚腑鍙敤鐨勬瘡涓涔夎ˉ涓佸簲鐢ㄥ埌鏁翠釜 Linux 鍐呮牳銆?
瀵逛簬姣忎釜璇箟琛ヤ竵锛岄兘浼氭彁鍑轰竴鏉℃彁浜や俊鎭€傚畠鎻忚堪浜嗚璇箟琛ヤ竵鎵€妫€鏌ョ殑闂锛屽苟鍖呭惈瀵?Coccinelle
鐨勫紩鐢ㄣ€?
涓庝换浣曢潤鎬佷唬鐮佸垎鏋愬櫒涓€鏍凤紝Coccinelle 浼氫骇鐢熻鎶ワ紙false positive锛夈€傚洜姝わ紝鎶ュ憡蹇呴』浠旂粏妫€鏌ワ紝
琛ヤ竵涔熷繀椤荤粡杩囧鏌ャ€?
```

   make coccicheck MODE=report V=1

```
榛樿鎯呭喌涓嬶紝coccicheck 浼氭妸璋冭瘯鏃ュ織鎵撳嵃鍒?stdout锛屽苟鎶?stderr 閲嶅畾鍚戝埌 /dev/null銆傝繖鍙兘浣?coccicheck 鐨勮緭鍑洪毦浠ラ槄璇诲拰鐞嗚В銆傝皟璇曞拰閿欒娑堟伅涔熷彲浠ユ敼涓哄啓鍏ヤ竴涓皟璇曟枃浠讹紝閫氳繃

```

    make coccicheck MODE=report DEBUG_FILE="cocci.log"

```
Coccinelle 涓嶈兘瑕嗙洊涓€涓皟璇曟枃浠躲€備笌鍏跺弽澶嶅垹闄ゆ棩蹇楋紝涓嶅

```

    make coccicheck MODE=report DEBUG_FILE="cocci-$(date -Iseconds).log"

```
### Coccinelle 骞惰鍖?

榛樿鎯呭喌涓嬶紝coccicheck 浼氬敖閲忎互骞惰鏂瑰紡杩愯銆傝鏀瑰彉杩欎竴鐐癸紝鍙互浣跨敤

```

   make coccicheck MODE=report J=4

```
浠?Coccinelle 1.0.2 璧凤紝Coccinelle 浣跨敤 Ocaml parmap 杩涜骞惰鍖栵紱濡傛灉妫€娴嬪埌瀵规鐨勬敮鎸侊紝浣犲皢
鍙楃泭浜?parmap 骞惰鍖栥€?
褰撳惎鐢?parmap 鏃讹紝coccicheck 浼氫娇鐢?`--chunksize 1` 鍙傛暟鏉ュ惎鐢ㄥ姩鎬佽礋杞藉潎琛°€傝繖纭繚鎴戜滑涓€涓竴涓?鍦版寔缁悜绾跨▼鍒嗗彂宸ヤ綔锛屼粠鑰岄伩鍏嶅ぇ閮ㄥ垎宸ヤ綔鍙敱灏戞暟鍑犱釜绾跨▼瀹屾垚鐨勬儏鍐点€傞€氳繃鍔ㄦ€佽礋杞藉潎琛★紝濡傛灉鏌愪釜
绾跨▼鎻愬墠瀹屾垚锛屾垜浠細鎸佺画鍚戝畠鍒嗗彂鏇村宸ヤ綔銆?
褰?parmap 鍚敤鏃讹紝濡傛灉 Coccinelle 涓彂鐢熶簡閿欒锛岃閿欒鍊间細琚紶鎾洖鏉ワ紝骞朵笖 `make coccicheck`
鍛戒护鐨勮繑鍥炲€间細鎹曡幏杩欎釜杩斿洖鍊笺€?
### 浣跨敤鍗曚釜璇箟琛ヤ竵杩愯 Coccinelle


鍙€夌殑 make 鍙橀噺 COCCI 鍙敤浜庢鏌ュ崟涓涔夎ˉ涓併€傚湪杩欑鎯呭喌涓嬶紝璇ュ彉閲忓繀椤荤敤瑕佸簲鐢ㄧ殑璇箟琛ヤ竵鐨?鍚嶅瓧鍒濆鍖栥€?
```

	make coccicheck COCCI=<my_SP.cocci> MODE=patch

```

```

	make coccicheck COCCI=<my_SP.cocci> MODE=report


```
### 鎺у埗 Coccinelle 澶勭悊鍝簺鏂囦欢


榛樿浼氭鏌ユ暣涓唴鏍告簮浠ｇ爜鏍戙€?
瑕佸皢 Coccinelle 搴旂敤鍒扮壒瀹氱洰褰曪紝鍙互浣跨敤 `M=`銆?
```

    make coccicheck M=drivers/net/wireless/

```
瑕佷互鏂囦欢涓哄崟浣嶏紙鑰岄潪鐩綍涓哄崟浣嶏級搴旂敤 Coccinelle锛宮akefile 浣跨敤 C 鍙橀噺鏉ラ€夋嫨瑕佸鐞嗙殑鏂囦欢銆傝
鍙橀噺鍙敤浜庝负鏁翠釜鍐呮牳銆佺壒瀹氱洰褰曟垨鍗曚釜鏂囦欢杩愯鑴氭湰銆?
渚嬪锛岃妫€鏌?drivers/bluetooth/bfusb.c锛屽悜 C 鍙橀噺浼犲叆鍊?1 浠ユ鏌?make 璁や负鐩稿叧鐨勬枃浠?
```

    make C=1 CHECK=scripts/coccicheck drivers/bluetooth/bfusb.o

```
鍚?C 鍙橀噺浼犲叆鍊?2 浠ユ鏌ユ枃浠惰€屼笉绠″叾鏄惁

```

    make C=2 CHECK=scripts/coccicheck drivers/bluetooth/bfusb.o

```
鍦ㄨ繖浜涗互鏂囦欢涓哄崟浣嶅伐浣滅殑妯″紡涓嬶紝涓嶄細鏄剧ず鍏充簬璇箟琛ヤ竵鐨勪俊鎭紝涔熶笉浼氭彁鍑烘彁浜や俊鎭€?
杩欓粯璁よ繍琛?scripts/coccinelle 涓殑姣忎釜璇箟琛ヤ竵銆侰OCCI 鍙橀噺涔熷彲棰濆鐢ㄤ簬浠呭簲鐢ㄥ崟涓涔夎ˉ涓侊紝
濡備笂涓€鑺傛墍绀恒€?
榛樿妯″紡鏄?鈥渞eport鈥濄€備綘鍙互鐢ㄤ笂鏂囪В閲婄殑 MODE 鍙橀噺閫夋嫨鍙︿竴绉嶆ā寮忋€?
### 璋冭瘯 Coccinelle SmPL 琛ヤ竵


浣跨敤 coccicheck 鏈€濂斤紝鍥犱负瀹冨湪 spatch 鍛戒护琛屼腑鎻愪緵浜嗕笌鎴戜滑缂栬瘧鍐呮牳鏃舵墍浣跨敤閫夐」鐩稿尮閰嶇殑
鍖呭惈閫夐」銆備綘鍙互閫氳繃浣跨敤 V=1 鏉ヤ簡瑙ｈ繖浜涢€夐」鏄粈涔堬紱鐒跺悗浣犲氨鍙互鍔犱笂璋冭瘯閫夐」鎵嬪姩杩愯
Coccinelle銆?
璋冭瘯閽堝 SmPL 琛ヤ竵杩愯 Coccinelle 鐨勪竴涓洿绠€鍗曠殑鏂规硶锛屾槸璁?coccicheck 鎶?stderr 閲嶅畾鍚戝埌
涓€涓皟璇曟枃浠躲€傚绀轰緥鎵€杩帮紝榛樿 stderr 琚噸瀹氬悜鍒?/dev/null锛涘鏋滀綘鎯虫崟鑾?stderr锛屽彲浠?
```

    rm -f cocci.err
    make coccicheck COCCI=scripts/coccinelle/free/kfree.cocci MODE=report DEBUG_FILE=cocci.err
    cat cocci.err

```
浣犲彲浠ヤ娇鐢?SPFLAGS 娣诲姞璋冭瘯鏍囧織锛涗緥濡傦紝鍦ㄨ皟璇曟椂浣犲彲鑳芥兂鍚?SPFLAGS 鍚屾椂娣诲姞 `--profile
--show-trying`銆備緥濡?
```

    rm -f err.log
    export COCCI=scripts/coccinelle/misc/irqf_oneshot.cocci
    make coccicheck DEBUG_FILE="err.log" MODE=report SPFLAGS="--profile --show-trying" M=./drivers/mfd

```
err.log 鐜板湪灏嗗寘鍚€ц兘鍒嗘瀽锛坧rofiling锛変俊鎭紝鑰?stdout 灏嗛殢鐫€ Coccinelle 鎺ㄨ繘宸ヤ綔鎻愪緵涓€浜?杩涘害淇℃伅銆?
娉ㄦ剰锛?
DEBUG_FILE 鏀寔浠呭湪 coccinelle >= 1.0.2 鏃跺彲鐢ㄣ€?
鐩墠锛孌EBUG_FILE 鏀寔浠呴€傜敤浜庢鏌ユ枃浠跺す锛岃€屼笉閫傜敤浜庡崟涓枃浠躲€傝繖鏄洜涓烘鏌ュ崟涓枃浠堕渶瑕佽皟鐢?spatch 涓ゆ锛屽鑷?DEBUG_FILE 涓ゆ閮借璁剧疆涓虹浉鍚岀殑鍊硷紝浠庤€屼骇鐢熼敊璇€?
### .cocciconfig 鏀寔


Coccinelle 鏀寔璇诲彇 .cocciconfig 浠ヨ幏鍙栨瘡娆＄敓鎴?spatch 鏃堕兘搴斾娇鐢ㄧ殑榛樿 Coccinelle 閫夐」銆?.cocciconfig 涓彉閲忕殑浼樺厛椤哄簭濡備笅锛?
- 棣栧厛澶勭悊褰撳墠鐢ㄦ埛鐨勪富鐩綍
- 鎺ヤ笅鏉ュ鐞嗚皟鐢?spatch 鎵€鍦ㄧ洰褰?- 濡傛灉浣跨敤锛屾渶鍚庡鐞嗛€氳繃 `--dir` 閫夐」鎻愪緵鐨勭洰褰?
`make coccicheck` 涔熸敮鎸佷娇鐢?M= 鐩爣銆傚鏋滀綘娌℃湁鎻愪緵浠讳綍 M= 鐩爣锛屽垯鍋囧畾浣犳兂浠ユ暣涓唴鏍镐负鐩爣銆?
```

    OPTIONS="--dir $srcroot $COCCIINCLUDE"

```
杩欓噷锛?srcroot 鎸囩殑鏄洰鏍囩殑婧愪唬鐮佺洰褰曪細褰撲娇鐢?M= 鏃跺畠鎸囧悜澶栭儴妯″潡鐨勬簮浠ｇ爜鐩綍锛屽惁鍒欐寚鍚戝唴鏍?婧愪唬鐮佺洰褰曘€傜涓夋潯瑙勫垯纭繚 spatch 浠庣洰鏍囩洰褰曡鍙?.cocciconfig锛屼粠鑰屽厑璁稿閮ㄦā鍧楁嫢鏈夎嚜宸辩殑
.cocciconfig 鏂囦欢銆?
濡傛灉涓嶄娇鐢ㄥ唴鏍哥殑 coccicheck 鐩爣锛岃淇濇寔涓婅堪 .cocciconfig 璇诲彇鐨勪紭鍏堥『搴忛€昏緫銆傚鏋滀娇鐢ㄥ唴鏍哥殑
coccicheck 鐩爣锛屽彲閫氳繃 SPFLAGS 瑕嗙洊鍐呮牳 .coccicheck 鐨勪换浣曡缃€?
鎴戜滑鍦ㄩ拡瀵?Linux 浣跨敤 Coccinelle 鏃讹紝閫氳繃鎴戜滑鑷繁鐨?Linux .cocciconfig 鎻愪緵浜嗕竴缁勫悎鐞嗙殑
Linux 榛樿閫夐」锛屼互鎻愮ず Coccinelle 鍙互浣跨敤 git 杩涜 `git grep` 鏌ヨ锛堥€氳繃 coccigrep锛夈€傜洰鍓?200 绉掔殑瓒呮椂搴旇瓒冲浜嗐€?
Coccinelle 鍦ㄨ鍙?.cocciconfig 鏃舵嬀鍙栫殑閫夐」涓嶄細浣滀负杩愯鍦ㄤ綘绯荤粺涓婄殑 spatch 杩涚▼鐨勫弬鏁板嚭鐜般€?瑕佺‘璁ゅ疄闄呬娇鐢ㄤ簡鍝簺閫夐」锛屽彲浠?
```

      spatch --print-options-only

```
浣犲彲浠ラ€氳繃浣跨敤 SPFLAGS 瑕嗙洊涓轰綘鑷繁鍋忓ソ鐨勭储寮曢€夐」銆傝娉ㄦ剰锛屽綋瀛樺湪鍐茬獊閫夐」鏃讹紝Coccinelle 浼氫紭鍏?閲囩敤鏈€鍚庝紶鍏ョ殑閫夐」銆備娇鐢?.cocciconfig 涔熷彲浠ヤ娇鐢?idutils锛屼笉杩囬壌浜?Coccinelle 閬靛惊鐨勪紭鍏堥『搴忥紝
鐢变簬鍐呮牳鐜板湪甯︽湁鑷繁鐨?.cocciconfig锛屽鏋滈渶瑕佷娇鐢?idutils锛屼綘灏嗗繀椤讳娇鐢?SPFLAGS銆傛洿澶氬叧浜庡浣?浣跨敤 idutils 鐨勭粏鑺傦紝璇峰弬闃呬笅鏂?鈥淎dditional flags锛堥檮鍔犳爣蹇楋級鈥?涓€鑺傘€?
### 闄勫姞鏍囧織


鍙互閫氳繃 SPFLAGS 鍙橀噺鍚?spatch 浼犻€掗檮鍔犳爣蹇椼€傝繖鍙互宸ヤ綔锛屽洜涓?Coccinelle 浼氶伒寰渶鍚庝紶鍏ョ殑鏍囧織

```

    make SPFLAGS=--use-glimpse coccicheck

```
Coccinelle 涔熸敮鎸?idutils锛屼絾闇€瑕?coccinelle >= 1.0.6銆傚綋娌℃湁鎸囧畾 ID 鏂囦欢鏃讹紝Coccinelle 鍋囧畾
浣犵殑 ID 鏁版嵁搴撴枃浠朵綅浜庡唴鏍搁《灞傜殑 .id-utils.index 鏂囦欢涓€侰occinelle

```

    mkid -i C --output .id-utils.index

```
濡傛灉浣犳湁鍙︿竴涓暟鎹簱鏂囦欢鍚嶏紝涔熷彲浠ョ洿鎺ラ€氳繃濡備笅鏂瑰紡浣跨敤绗﹀彿閾炬帴

```

    make SPFLAGS=--use-idutils coccicheck

```
鎴栬€呬綘涔熷彲浠ユ樉寮忔寚瀹氭暟鎹簱鏂囦欢鍚?
```

    make SPFLAGS="--use-idutils /full-path/to/ID" coccicheck

```
鍙傝 `spatch --help` 浠ヤ簡瑙ｆ洿澶氬叧浜?spatch 閫夐」鐨勪俊鎭€?
璇锋敞鎰忥紝`--use-glimpse` 鍜?`--use-idutils` 閫夐」闇€瑕佸閮ㄥ伐鍏锋潵涓轰唬鐮佸缓绔嬬储寮曘€傚洜姝ゅ畠浠粯璁ら兘涓?婵€娲汇€傜劧鑰岋紝閫氳繃浣跨敤杩欎簺宸ュ叿涔嬩竴涓轰唬鐮佸缓绔嬬储寮曪紝骞舵牴鎹墍浣跨敤鐨?cocci 鏂囦欢锛宻patch 鍙互鏇村揩鍦?澶勭悊鏁翠釜浠ｇ爜搴撱€?
### SmPL 琛ヤ竵涓撴湁閫夐」


SmPL 琛ヤ竵鍙互瀵硅嚜宸变紶缁?Coccinelle 鐨勯€夐」鏈夎姹傘€係mPL 琛ヤ竵涓撴湁閫夐」鍙互閫氳繃濡備笅鏂瑰紡鎻愪緵

```

	// Options: --no-includes --include-headers

```
### SmPL 琛ヤ竵鐨?Coccinelle 鐗堟湰瑕佹眰


闅忕潃 Coccinelle 鐗规€т笉鏂鍔狅紝涓€浜涙洿楂樼骇鐨?SmPL 琛ヤ竵鍙兘闇€瑕佹洿鏂扮増鏈殑 Coccinelle銆傚鏋滀竴涓?SmPL 琛ヤ竵瑕佹眰鏈€浣庣増鏈殑 Coccinelle锛屽彲浠ュ涓嬫寚瀹?
```

	// Requires: 1.0.5

```
### 鎻愬嚭鏂扮殑璇箟琛ヤ竵


鍐呮牳寮€鍙戣€呭彲浠ユ彁鍑哄苟鎻愪氦鏂扮殑璇箟琛ヤ竵銆備负浜嗘竻鏅拌捣瑙侊紝瀹冧滑搴斿綋缁勭粐鍦?`scripts/coccinelle/` 鐨?瀛愮洰褰曚腑銆?

### ``report`` 妯″紡鐨勮缁嗚鏄?

```

  file:line:column-column: message

```
#### 绀轰緥


```

	make coccicheck MODE=report COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

   <smpl>
   @r depends on !context && !patch && (org || report)@
   expression x;
   position p;
   @@

     ERR_PTR@p(PTR_ERR(x))

   @script:python depends on report@
   p << r.p;
   x << r.x;
   @@

   msg="ERR_CAST can be used with %s" % (x)
   coccilib.report.print_report(p[0], msg)
   </smpl>

```
杩欐 SmPL 鎽樺綍鍦ㄦ爣鍑嗚緭鍑轰笂鐢熸垚濡備笅鏉＄洰

```

    /home/user/linux/crypto/ctr.c:188:9-16: ERR_CAST can be used with alg
    /home/user/linux/crypto/authenc.c:619:9-16: ERR_CAST can be used with auth
    /home/user/linux/crypto/xts.c:227:9-16: ERR_CAST can be used with alg


```
### ``patch`` 妯″紡鐨勮缁嗚鏄?

褰?`patch` 妯″紡鍙敤鏃讹紝瀹冧細涓烘瘡涓瘑鍒嚭鐨勯棶棰樻彁鍑轰竴涓慨澶嶃€?
#### 绀轰緥


```

	make coccicheck MODE=patch COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @ depends on !context && patch && !org && !report @
    expression x;
    @@

    - ERR_PTR(PTR_ERR(x))
    + ERR_CAST(x)
    </smpl>

```
杩欐 SmPL 鎽樺綍鍦ㄦ爣鍑嗚緭鍑轰笂鐢熸垚琛ヤ竵鍧楋紙patch hunk锛夛紝濡備笅鎵€绀?
```

    diff -u -p a/crypto/ctr.c b/crypto/ctr.c
    --- a/crypto/ctr.c 2010-05-26 10:49:38.000000000 +0200
    +++ b/crypto/ctr.c 2010-06-03 23:44:49.000000000 +0200
    @@ -185,7 +185,7 @@ static struct crypto_instance *crypto_ct
 	alg = crypto_attr_alg(tb[1], CRYPTO_ALG_TYPE_CIPHER,
 				  CRYPTO_ALG_TYPE_MASK);
 	if (IS_ERR(alg))
    -		return ERR_PTR(PTR_ERR(alg));
    +		return ERR_CAST(alg);

 	/* Block size must be >= 4 bytes. */
 	err = -EINVAL;

```
### ``context`` 妯″紡鐨勮缁嗚鏄?

`context` 浠ョ被浼?diff 鐨勯鏍奸珮浜劅鍏磋叮鐨勮鍙婂叾涓婁笅鏂囥€?
      **娉ㄦ剰**锛氱敓鎴愮殑绫讳技 diff 鐨勮緭鍑哄苟涓嶆槸涓€涓彲搴旂敤鐨勮ˉ涓併€俙context` 妯″紡鐨勬剰鍥炬槸
      楂樹寒閲嶈鐨勮锛堢敤鍑忓彿 `-` 鏍囨敞锛夛紝骞剁粰鍑哄懆鍥寸殑涓€浜涗笂涓嬫枃琛屻€傝繖涓緭鍑哄彲浠ュ拰
      Emacs 鐨?diff 妯″紡涓€璧风敤鏉ュ鏌ヤ唬鐮併€?
#### 绀轰緥


```

	make coccicheck MODE=context COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @ depends on context && !patch && !org && !report@
    expression x;
    @@

    * ERR_PTR(PTR_ERR(x))
    </smpl>

```
杩欐 SmPL 鎽樺綍鍦ㄦ爣鍑嗚緭鍑轰笂鐢熸垚 diff 鍧楋紙diff hunk锛夛紝濡備笅鎵€绀?
```

    diff -u -p /home/user/linux/crypto/ctr.c /tmp/nothing
    --- /home/user/linux/crypto/ctr.c	2010-05-26 10:49:38.000000000 +0200
    +++ /tmp/nothing
    @@ -185,7 +185,6 @@ static struct crypto_instance *crypto_ct
 	alg = crypto_attr_alg(tb[1], CRYPTO_ALG_TYPE_CIPHER,
 				  CRYPTO_ALG_TYPE_MASK);
 	if (IS_ERR(alg))
    -		return ERR_PTR(PTR_ERR(alg));

 	/* Block size must be >= 4 bytes. */
 	err = -EINVAL;

```
### ``org`` 妯″紡鐨勮缁嗚鏄?

`org` 鐢熸垚 Emacs 鐨?Org mode 鏍煎紡鐨勬姤鍛娿€?
#### 绀轰緥


```

	make coccicheck MODE=org COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @r depends on !context && !patch && (org || report)@
    expression x;
    position p;
    @@

      ERR_PTR@p(PTR_ERR(x))

    @script:python depends on org@
    p << r.p;
    x << r.x;
    @@

    msg="ERR_CAST can be used with %s" % (x)
    msg_safe=msg.replace("[","@(").replace("]",")")
    coccilib.org.print_todo(p[0], msg_safe)
    </smpl>

```
杩欐 SmPL 鎽樺綍鍦ㄦ爣鍑嗚緭鍑轰笂鐢熸垚 Org 鏉＄洰锛屽涓嬫墍绀?
```

    * TODO [[view:/home/user/linux/crypto/ctr.c::face=ovl-face1::linb=188::colb=9::cole=16][ERR_CAST can be used with alg]]
    * TODO [[view:/home/user/linux/crypto/authenc.c::face=ovl-face1::linb=619::colb=9::cole=16][ERR_CAST can be used with auth]]
    * TODO [[view:/home/user/linux/crypto/xts.c::face=ovl-face1::linb=227::colb=9::cole=16][ERR_CAST can be used with alg]]

```
