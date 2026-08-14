## 浜屽垎瀹氫綅鍥炲綊锛圔isecting regression锛?

鏈枃妗ｄ粙缁嶅浣曚娇鐢?`git bisect` 鎵惧嚭瀵艰嚧鏌愬姛鑳藉け鏁堢殑婧愮爜鏀瑰姩鈥斺€斾緥濡傚湪灏?Linux 浠?6.0 鍗囩骇鍒?6.1 鍚庯紝鏌愰」鍔熻兘鍋滄宸ヤ綔銆備笅鏂囪仛鐒︿簬璇ヨ繃绋嬬殑鏍稿績瑕佺偣銆傝嫢瑕佷粠澶村紑濮嬪鍐呮牳鍋氫簩鍒嗭紝鏇村缓璁敼璇?Documentation/admin-guide/verify-bugs-bisect-regressions.rst锛氬畠瀵规暣涓繃绋嬩粠澶村埌灏鹃兘鏈夋弿杩帮紝骞舵兜鐩栦簡澶氫釜杩炲唴鏍稿紑鍙戣€呭伓灏斾篃浼氶仐蹇樼殑缁嗚妭銆傚叾涓繕鍖呮嫭灏芥棭璇嗗埆鈥滀簩鍒嗗彧浼氭氮璐规椂闂淬€佸叾缁撴灉鏃犱汉鍏冲績鈥濈殑鎯呭舰鈥斺€斾緥濡傞棶棰樺彂鐢熷湪琚唴鏍告爣璁颁负鈥滃彈姹℃煋锛坱ainted锛夆€濈殑鍐呮牳涓€佸嚭鐜板湪宸插簾寮冪殑鐗堟湰閲屻€佸凡缁忚淇锛屾垨鏄敱 Linux 鍙戣鏂规墍鍋氱殑 .config 鍙樻洿寮曡捣鐨勩€?

## 浣跨敤浜屽垎鏌ユ壘瀵艰嚧鍐呮牳闂鐨勬敼鍔?

*璇存槑锛氫互涓嬭繃绋嬪亣璁句綘宸茬粡涓轰簩鍒嗗仛濂戒簡鎵€鏈夊噯澶囥€傝繖鍖呮嫭锛氭嫢鏈夌浉搴旀簮鐮佺殑 Git 鍏嬮殕銆佸畨瑁呬簡鏋勫缓骞跺畨瑁呭唴鏍告墍闇€鐨勮蒋浠讹紝浠ュ強灏嗕竴浠?.config 鏂囦欢淇濆瓨鍦ㄥ畨鍏ㄧ殑浣嶇疆锛堜笅渚嬪亣璁句负 '~/prepared_kernel_.config'锛夛紝浠ヤ究浣滀负姣忎竴姝ヤ簩鍒嗙殑骞插噣鍩哄噯锛涚悊鎯虫儏鍐典笅锛屼綘杩樺簲鎵惧埌涓€涓畬鍏ㄥ彲闈犮€佺洿鎴簡褰撶殑鏂瑰紡鏉ュ鐜拌鍥炲綊銆?

- 鍑嗗锛氬紑濮嬩簩鍒嗭紝骞跺憡璇?Git 鍘嗗彶涓殑涓や釜绔偣锛?
```
git bisect start
git bisect good v6.0
git bisect bad v6.1
```
闄や簡鍍?'v6.0' 鍜?'v6.1' 杩欐牱鐨?Git 鏍囩澶栵紝浣犱篃鍙互鎸囧畾鎻愪氦 ID銆?

1. 灏嗗噯澶囧ソ鐨?.config 澶嶅埗鍒版瀯寤虹洰褰曞苟閫傞厤锛?
```
cp ~/prepared_kernel_.config .config
make olddefconfig
```
2. 鐜板湪鏋勫缓銆佸畨瑁呭苟鍚姩鍐呮牳銆傚畠鍙兘鍥犳棤鍏冲師鍥犺€屽け璐ワ紝渚嬪鍦ㄤ簩鍒嗗綋鍓嶉樁娈靛嚭鐜颁簡涓€涓紪璇戦敊璇紝鑰岃閿欒浼氬湪鍚庣画鐨勬煇娆℃敼鍔ㄤ腑琚В鍐炽€傝繖绉嶆儏鍐典笅璇疯繍琛?`git bisect skip` 骞惰繑鍥炵 1 姝ャ€?
3. 妫€鏌ュ垰鍒氭瀯寤虹殑鍐呮牳涓紝鍙戠敓鍥炲綊鐨勯偅椤瑰姛鑳芥槸鍚︽甯稿伐浣溿€?
```
git bisect good
```
濡傛灉瀹冨潖浜嗭紝鍒欒繍琛岋細
```
git bisect bad
```
璇锋敞鎰忥紝鍙鎼為敊涓€娆★紝灏变細璁╀綑涓嬬殑浜屽垎褰诲簳璺戝亸銆備负浜嗛伩鍏嶆棩鍚庝笉寰椾笉浠庡ご鍐嶆潵锛屼綘瑕佺‘淇濆憡璇?Git 鐨勭粨璁烘槸姝ｇ‘鐨勶紱鍥犳锛屽綋浣犵殑澶嶇幇鎵嬫骞朵笉鍙潬鏃讹紝澶氳姳鍑犲垎閽熷仛娴嬭瘯寰€寰€鏄槑鏅虹殑銆?
鍦ㄥ彂鍑轰笂杩颁袱鏉″懡浠や箣涓€鍚庯紝Git 閫氬父浼氭鍑哄彟涓€涓簩鍒嗙偣骞舵墦鍗扮被浼尖€淏isecting: 675 revisions left to test after this (roughly 10 steps)鈥濈殑淇℃伅銆傛鏃惰鍥炲埌绗?1 姝ャ€?
濡傛灉 Git 鎵撳嵃鐨勫垯鏄被浼尖€渃afecaca0c0dacafecaca0c0dacafecaca0c0da is the first bad commit鈥濈殑淇℃伅锛岄偅涔堜簩鍒嗗氨瀹屾垚浜嗐€傛鏃惰杞埌涓嬮潰鐨勪笅涓€涓鐐广€傛敞鎰忥紝鍦ㄦ樉绀鸿琛屽悗锛孏it 浼氱珛鍗冲睍绀哄叧浜庘€滅姜榄佺ジ棣栵紙culprit锛夆€濈殑涓€浜涚粏鑺傦紝鍖呮嫭鍏惰ˉ涓佽鏄庯紱杩欏緢瀹规槗鍗犳弧浣犵殑缁堢锛屽洜姝や綘鍙兘闇€瑕佸悜涓婃粴鍔ㄦ墠鑳界湅鍒版彁鍙婅鎻愪氦 ID 鐨勯偅鏉℃秷鎭€?
濡傛灉浣犻敊杩囦簡 Git 鐨勮緭鍑猴紝闅忔椂鍙互杩愯 ``git bisect log`` 鏉ユ墦鍗扮姸鎬侊細瀹冧細鏄剧ず杩樺墿澶氬皯姝ワ紝鎴栬€呯粰鍑轰簩鍒嗙殑缁撴灉銆?

- 鎺ㄨ崘鐨勮緟鍔╂楠わ細灏嗕簩鍒嗘棩蹇楀拰褰撳墠鐨?.config 鏂囦欢鐣欎綔缂洪櫡鎶ュ憡涔嬬敤锛涙澶栬 Git 閲嶇疆婧愮爜锛?
```
git bisect log > ~/bisection-log
cp .config ~/bisection-config-culprit
git bisect reset
```
- 鎺ㄨ崘鐨勫閫夋楠わ細灏濊瘯鍦ㄦ渶鏂扮殑浠ｇ爜鍩轰箣涓婅繕鍘熲€滅姜榄佺ジ棣栤€濓紝浠ユ鏌ユ槸鍚﹁兘淇璇ョ己闄凤紱鑻ュ彲浠ワ紝鍒欓獙璇佷簡浜屽垎鐨勬纭€э紝骞惰寮€鍙戣€呰兘澶熼€氳繃杩樺師鏉ヨВ鍐宠鍥炲綊銆?
```
git revert --no-edit cafec0cacaca0
```
Git 鍙兘浼氭嫆缁濊繖涓€鎿嶄綔锛屼緥濡傚綋浜屽垎钀藉湪浜嗕竴涓悎骞舵彁浜や笂鏃躲€傛鏃惰鏀惧純灏濊瘯銆傚鏋?Git 鍥犲悗缁敼鍔ㄤ緷璧栦簬璇ユ彁浜よ€岃嚜韬棤娉曞畬鎴愯繕鍘燂紝涔熷簲鍚屾牱鏀惧純鈥斺€旈櫎闈炰綘浜屽垎鐨勬槸 stable 鎴?longterm 鍐呮牳绯诲垪锛岃繖绉嶆儏鍐典笅浣犲簲妫€鍑哄叾鏈€鏂颁唬鐮佸熀骞跺湪閭ｉ噷灏濊瘯杩樺師銆?
濡傛灉杩樺師鎴愬姛锛岃鍐嶆瀯寤哄苟娴嬭瘯涓€涓唴鏍革紝浠ョ‘璁よ繕鍘熸槸鍚﹁В鍐充簡浣犵殑鍥炲綊銆?

杩囩▼鑷虫瀹屾垚銆傜幇鍦ㄨ鎸?Documentation/admin-guide/reporting-issues.rst 鎵€杩扮殑鏂瑰紡鎶ュ憡璇ュ洖褰掋€?

### 瀵?linux-next 鍋氫簩鍒嗭紙Bisecting linux-next锛?

濡傛灉闂鍑虹幇鍦?linux-next 涓紝璇峰 linux-next 鐨?'stable' 涓?'master' 鍒嗘敮鍋氫簩鍒嗐€備互涓嬪懡浠ょ敤浜庡紑濮嬶細
```
git bisect start
git bisect good next/stable
git bisect bad next/master
```
'stable' 鍒嗘敮瀵瑰簲鐨勬槸褰撳墠 linux-next 鍙戝竷锛堜綅浜?'master' 鍒嗘敮锛夋墍鍩轰簬鐨?linux-mainline 鐘舵€佲€斺€斿洜姝ゅ墠鑰呬笉鍚湁鍦?-next銆佸嵆 Linus 鐨勬爲涓墠浼氬嚭鐜扮殑闂銆?
褰撹法瓒婂緢澶ц寖鍥寸殑鏀瑰姩鍋氫簩鍒嗘椂锛屼綘鍙兘浼氭兂浣跨敤鏇存棭鐨?linux-next 鍙戝竷鏉ヨ閬块棶棰樸€傞仐鎲剧殑鏄紝骞舵病鏈夌畝鍗曠殑鏂规硶鍙互鍏嶅幓鏍稿锛氬皢涓€涓?linux-next 鍙戝竷涓庢洿鏅氱殑涓€涓紙渚嬪 'next-20241020' 涓?'next-20241021'锛夌浉浜掍簩鍒嗘槸涓嶅彲鑳界殑锛屽洜涓哄畠浠病鏈夊叡鍚岀殑鍘嗗彶銆?

### 寤朵几闃呰锛圓dditional reading material锛?

- `git bisect 鐨勬墜鍐岄〉 <https://git-scm.com/docs/git-bisect>`_
- `鐢?'git bisect' 瀵规姉鍥炲綊 <https://git-scm.com/docs/git-bisect-lk2009.html>`_锛孏it 鏂囨。銆?
- `浣跨敤 git bisect 宸ヤ綔 <https://nathanchance.dev/posts/working-with-git-bisect/>`_锛屽唴鏍稿紑鍙戣€?Nathan Chancellor銆?
- `鐢?Git bisect 寮勬竻闂鏄湪浣曟椂寮曞叆鐨?<http://webchick.net/node/99>`_銆?
- `鐢?'git bisect run' 瀹炵幇瀹屽叏鑷姩鍖栫殑浜屽垎 <https://lwn.net/Articles/317154>`_銆?

..
end-content
..
鏈枃妗ｇ敱 Thorsten Leemhuis <linux@leemhuis.info> 缁存姢銆傚鍙戠幇閿欏埆瀛楁垨灏忕殑鐤忔紡锛屾杩庣洿鎺ュ憡鐭ヤ粬锛屼粬浼氫簣浠ヤ慨姝ｃ€傝嫢浣犳兂浠ュ悓鏍凤紙澶у涓洪潪姝ｅ紡锛夌殑鏂瑰紡璐＄尞瀵规鏂囩殑鏀瑰姩锛屽嚭浜庣増鏉冨師鍥犺鎶勯€侊紙CC锛塴inux-doc@vger.kernel.org 骞堕檮涓娾€?sign-off鈥濓紙寮€鍙戣€呭師浜у湴璇佷功锛夎鏄庯紝瑙?Documentation/process/submitting-patches.rst 涓殑鐩稿叧绔犺妭銆?
..
鏈枃鏈彲鎸?GPL-2.0+ 涓?CC-BY-4.0 鍙岄噸璁稿彲鍙戝竷锛屾枃浠堕《閮ㄥ凡娉ㄦ槑銆傝嫢浣犳兂浠?CC-BY-4.0 鍒嗗彂鏈枃锛岃浣跨敤鈥淟inux 鍐呮牳寮€鍙戠ぞ鍖衡€濅綔涓轰綔鑰呯讲鍚嶏紝骞堕檮涓婃潵婧愰摼鎺ワ細
https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/bug-bisect.rst

..
璇存槑锛氭湰 RST 鏂囦欢鐨勫唴瀹瑰彇鑷?Linux 鍐呮牳婧愮爜锛屽彲鎸?CC-BY-4.0 浣跨敤锛涗絾缁忚繃澶勭悊锛堜緥濡傚唴鏍哥殑鏋勫缓绯荤粺锛夊悗鐨勬枃鏈増鏈彲鑳藉寘鍚互鏇翠弗鏍艰鍙瘉鍙戝竷鐨勫唴瀹广€?
