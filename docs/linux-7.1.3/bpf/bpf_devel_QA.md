## 濡備綍涓?BPF 瀛愮郴缁熶氦浜?

鏈枃妗ｄ负 BPF 瀛愮郴缁熸彁渚涗笌鎶ュ憡缂洪櫡銆佹彁浜よˉ涓佷互鍙婁负绋冲畾鐗堝唴鏍告帓闃熻ˉ涓佺浉鍏崇殑鍚勭宸ヤ綔娴佷俊鎭€?
鍏充簬鎻愪氦琛ヤ竵鐨勪竴鑸俊鎭紝璇峰弬闃?Documentation/process/submitting-patches.rst銆傛湰鏂囨。浠呮弿杩颁笌 BPF 鐩稿叧鐨勯澶栫粏鑺傘€?
    :local:
    :depth: 2

## 鎶ュ憡缂洪櫡


### 闂細濡備綍鎶ュ憡 BPF 鍐呮牳浠ｇ爜鐨勭己闄凤紵


绛旓細鐢变簬鎵€鏈?BPF 鍐呮牳寮€鍙戜互鍙?bpftool 鍜?iproute2 BPF 鍔犺浇鍣ㄧ殑寮€鍙戦兘閫氳繃 bpf 鍐呮牳閭欢鍒楄〃杩涜锛岃灏嗗彂鐜扮殑浠讳綍 BPF 鐩稿叧闂鎶ュ憡鍒颁互涓嬮偖浠跺垪琛細

 bpf@vger.kernel.org

杩欎篃鍙兘鍖呮嫭涓?XDP銆丅PF 璺熻釜绛夌浉鍏崇殑璁銆?
閴翠簬 netdev 娴侀噺寰堥珮锛岃鍚屾椂涔熸妸 BPF 缁存姢鑰呭姞鍒?Cc锛堟潵鑷唴鏍?`MAINTAINERS` 鏂囦欢锛夛細

- Alexei Starovoitov <ast@kernel.org>
- Daniel Borkmann <daniel@iogearbox.net>

濡傛灉宸茬粡瀹氫綅鍒版湁闂鐨勬彁浜わ紝璇风‘淇濇妸瀹為檯鐨勬彁浜や綔鑰呬篃淇濈暀鍦ㄦ姤鍛婄殑 Cc 涓€傞€氬父鍙互閫氳繃鍐呮牳鐨?git 鏍戞潵璇嗗埆浠栦滑銆?
**璇蜂笉瑕佸皢 BPF 闂鎶ュ憡鍒?bugzilla.kernel.org锛屽洜涓洪偅鍑犱箮鍙互淇濊瘉鎵€鎶ュ憡鐨勯棶棰樿蹇界暐銆?*

## 鎻愪氦琛ヤ竵


### 闂細鍦ㄩ€佸嚭瀹℃煡涔嬪墠锛屽浣曞湪鎴戠殑鏀瑰姩涓婅繍琛?BPF CI锛?

绛旓細BPF CI 鍩轰簬 GitHub锛屾墭绠″湪 https://github.com/kernel-patches/bpf銆傝櫧鐒?GitHub 涔熸彁渚涘彲浠ヨ揪鍒扮浉鍚屾晥鏋滅殑 CLI锛屼絾杩欓噷鎴戜滑鍏虫敞鍩轰簬 UI 鐨勫伐浣滄祦銆?
浠ヤ笅姝ラ璇存槑浜嗗浣曚负浣犵殑琛ヤ竵鍚姩涓€娆?CI 杩愯锛?
- 鍦ㄤ綘鑷繁鐨勮处鎴蜂腑鍒涘缓涓婅堪浠撳簱鐨?fork锛堜竴娆℃€ф搷浣滐級

- 鍦ㄦ湰鍦板厠闅嗚 fork锛屾鍑轰竴涓窡韪?bpf-next 鎴?bpf 鍒嗘敮鐨勬柊鍒嗘敮锛屽苟鎶婁綘寰呮祴璇曠殑琛ヤ竵搴旂敤鍒板畠涔嬩笂

- 灏嗘湰鍦板垎鏀帹閫佸埌浣犵殑 fork锛屽苟鍒嗗埆閽堝 kernel-patches/bpf 鐨?bpf-next_base 鎴?bpf_base 鍒嗘敮鍒涘缓 pull request

pull request 鍒涘缓鍚庝笉涔咃紝CI 宸ヤ綔娴佸氨浼氳繍琛屻€傛敞鎰忚绠楀閲忎笌姝ｅ湪琚鏌ョ殑涓婃父鎻愪氦鐨勮ˉ涓佹槸鍏变韩鐨勶紝鍥犳鏍规嵁鍒╃敤鐜囷紝杩愯鍙兘闇€瑕佷竴娈垫椂闂存墠鑳藉畬鎴愩€?
鍙﹁娉ㄦ剰锛屼袱涓熀纭€鍒嗘敮锛坆pf-next_base 鍜?bpf_base锛変細闅忕潃琛ヤ竵琚帹閫佸埌瀹冧滑鎵€璺熻釜鐨勭浉搴斾笂娓稿垎鏀€屾洿鏂般€傚洜姝わ紝浣犵殑琛ヤ竵闆嗕篃浼氳嚜鍔紙灏濊瘯锛夎鍙樺熀銆傝繖绉嶈涓哄彲鑳藉鑷翠竴娆?CI 杩愯琚腑姝㈠苟浠ユ柊鐨勫熀绾块噸鏂板惎鍔ㄣ€?
### 闂細鎴戦渶瑕佹妸 BPF 琛ヤ竵鎻愪氦鍒板摢涓偖浠跺垪琛紵


绛旓細璇峰皢浣犵殑 BPF 琛ヤ竵鎻愪氦鍒?bpf 鍐呮牳閭欢鍒楄〃锛?
 bpf@vger.kernel.org

濡傛灉浣犵殑琛ヤ竵娑夊強鍚勭涓嶅悓鐨勫瓙绯荤粺锛堜緥濡傜綉缁溿€佽窡韪€佸畨鍏ㄧ瓑锛夛紝璇风‘淇濅篃鎶婄浉鍏崇殑鍐呮牳閭欢鍒楄〃鍜岄偅閲岀殑缁存姢鑰呭姞鍒?Cc锛屼互渚夸粬浠兘澶熷鏌ヨ繖浜涙洿鏀瑰苟缁欏嚭浠栦滑鐨?Acked-by銆?
### 闂細鍦ㄥ摢閲屽彲浠ユ壘鍒?BPF 瀛愮郴缁熷綋鍓嶆鍦ㄨ璁虹殑琛ヤ竵锛?

绛旓細鎵€鏈夋妱閫侊紙Cc锛夊埌 netdev 鐨勮ˉ涓侀兘鍦?netdev patchwork 椤圭洰涓嬫帓闃熺瓑寰呭鏌ワ細

  https://patchwork.kernel.org/project/netdevbpf/list/

閭ｄ簺浠?BPF 涓虹洰鏍囩殑琛ヤ竵浼氳鍒嗛厤缁欎竴涓?'bpf' 浠ｇ悊锛坉elegate锛夛紝鐢?BPF 缁存姢鑰呰繘涓€姝ュ鐞嗐€傚綋鍓嶆鍦ㄥ鏌ョ殑琛ヤ竵闃熷垪鍙互鍦ㄤ互涓嬩綅缃壘鍒帮細

  https://patchwork.kernel.org/project/netdevbpf/list/?delegate=121173

涓€鏃﹁ˉ涓佺敱鏁翠釜 BPF 绀惧尯瀹℃煡骞剁敱 BPF 缁存姢鑰呮壒鍑嗭紝瀹冧滑鍦?patchwork 涓殑鐘舵€佷細琚敼涓?'Accepted'锛屾彁浜よ€呬細閫氳繃閭欢鏀跺埌閫氱煡銆傝繖鎰忓懗鐫€浠?BPF 鐨勮搴︾湅杩欎簺琛ヤ竵娌￠棶棰橈紝骞朵笖宸茬粡琚簲鐢ㄥ埌涓や釜 BPF 鍐呮牳鏍戜箣涓€銆?
濡傛灉鏉ヨ嚜绀惧尯鐨勫弽棣堣姹傞噸鏂版彁浜わ紙respin锛夎ˉ涓侊紝瀹冧滑鍦?patchwork 涓殑鐘舵€佷細琚涓?'Changes Requested'锛屽苟浠庡綋鍓嶅鏌ラ槦鍒椾腑娓呴櫎銆傚浜庤ˉ涓佽鎷掔粷鎴栦笉閫傜敤浜?BPF 鏍戯紙浣嗗垎閰嶇粰浜?'bpf' 浠ｇ悊锛夌殑鎯呭喌涔熷悓鏍峰姝ゃ€?
### 闂細杩欎簺鏇存敼鏄浣曡繘鍏?Linux 鐨勶紵


绛旓細鏈変袱涓?BPF 鍐呮牳鏍戯紙git 浠撳簱锛夈€備竴鏃﹁ˉ涓佽 BPF 缁存姢鑰呮帴鍙楋紝瀹冧滑灏变細琚簲鐢ㄥ埌涓や釜 BPF 鏍戜箣涓€锛?
 - https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf.git/
 - https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/

bpf 鏍戞湰韬粎鐢ㄤ簬淇锛岃€?bpf-next 鐢ㄤ簬鐗规€с€佹竻鐞嗘垨鍏朵粬绫诲瀷鐨勬敼杩涳紙鈥滅被浼?next 鐨勫唴瀹光€濓級銆傝繖绫讳技浜庣綉缁滃瓙绯荤粺鐨?net 鍜?net-next 鏍戙€俠pf 鍜?bpf-next 閮藉彧浼氭湁 master 鍒嗘敮锛屼互绠€鍖栬ˉ涓佸簲褰撳彉鍩哄埌鍝釜鍒嗘敮鐨勯棶棰樸€?
bpf 鏍戜腑绱Н鐨?BPF 琛ヤ竵浼氬畾鏈熻鎷夊叆 net 鍐呮牳鏍戙€傚悓鏍峰湴锛岃鎺ュ彈杩涘叆 bpf-next 鏍戠殑绱Н BPF 琛ヤ竵浼氳繘鍏?net-next 鏍戙€俷et 鍜?net-next 閮界敱 David S. Miller 缁存姢銆備粠閭ｉ噷锛屽畠浠細杩涘叆鐢?Linus Torvalds 缁存姢鐨勫唴鏍镐富绾挎爲銆傝浜嗚В net 鍜?net-next 鍚堝苟鍒颁富绾挎爲鐨勮繃绋嬶紝璇峰弬闃?netdev 瀛愮郴缁熺殑鏂囨。 Documentation/process/maintainer-netdev.rst銆?
鍋跺皵锛屼负浜嗛槻姝㈠悎骞跺啿绐侊紝鎴戜滑鍙兘浼氬悜鍏朵粬鏍戯紙渚嬪 tracing锛夊彂閫佸寘鍚竴灏忛儴鍒嗚ˉ涓佺殑 pull request锛屼絾 net 鍜?net-next 濮嬬粓鏄闆嗘垚鐨勭洰鏍囦富鏍戙€?
pull request 浼氬寘鍚疮绉ˉ涓佺殑楂樺眰鎽樿锛屽苟鍙互閫氳繃浠ヤ笅涓婚琛屽湪 netdev 鍐呮牳閭欢鍒楄〃涓婃悳绱紙`yyyy-mm-dd` 鏄?pull 鐨勬棩鏈燂級锛?
```

  pull-request: bpf yyyy-mm-dd
  pull-request: bpf-next yyyy-mm-dd

```
### 闂細濡備綍鎸囨槑鎴戠殑琛ヤ竵搴旇搴旂敤鍒板摢涓爲锛坆pf 杩樻槸 bpf-next锛夛紵


绛旓細杩囩▼涓?netdev 瀛愮郴缁熸枃妗?Documentation/process/maintainer-netdev.rst 涓弿杩扮殑瀹屽叏鐩稿悓锛屽洜姝よ闃呰浜嗚В銆備富棰樿蹇呴』鎸囨槑璇ヨˉ涓佹槸涓€涓慨澶嶈繕鏄€滅被浼?next鈥濈殑鍐呭锛屼互渚胯缁存姢鑰呯煡閬撳畠鏄互 bpf 杩樻槸 bpf-next 涓虹洰鏍囥€?
瀵逛簬鏈€缁堣繘鍏?bpf -> net 鏍戠殑淇锛屼富棰樺繀椤?
```

  git format-patch --subject-prefix='PATCH bpf' start..finish

```
瀵逛簬鏈€缁堝簲杩涘叆

```

  git format-patch --subject-prefix='PATCH bpf-next' start..finish

```
濡傛灉浣犱笉纭畾琛ヤ竵鎴栬ˉ涓佺郴鍒楁槸鍚﹀簲璇ョ洿鎺ヨ繘鍏?bpf 鎴?net锛屾垨鑰呯洿鎺ヨ繘鍏?bpf-next 鎴?net-next锛岄偅涔堜富棰樿浠?net 鎴?net-next 涓虹洰鏍囦篃娌￠棶棰樸€傛渶缁堢敱缁存姢鑰呮潵鍐冲畾琛ヤ竵鐨勫娲俱€?
濡傛灉鏄庣‘琛ヤ竵搴斿綋杩涘叆 bpf 鎴?bpf-next 鏍戯紝璇风‘淇濋拡瀵归偅浜涙爲瀵硅ˉ涓佽繘琛屽彉鍩猴紝浠ュ噺灏戞綔鍦ㄧ殑鍐茬獊銆?
濡傛灉琛ヤ竵鎴栬ˉ涓佺郴鍒楅渶瑕佽繑宸ュ苟鍦ㄧ浜岀増鎴栨洿鏅氱殑淇涓啀娆″彂鍑猴紝鍒欒繕闇€瑕佹坊鍔?
```

  git format-patch --subject-prefix='PATCH bpf-next v2' start..finish

```
褰撹ˉ涓佺郴鍒楄瑕佹眰淇敼鏃讹紝璇峰缁堝皢鏁翠釜琛ヤ竵绯诲垪杩炲悓鍙嶉涓€璧峰啀娆″彂閫侊紙缁濅笉瑕佸湪鍘熺郴鍒椾箣涓婂崟鐙彂閫?diff锛夈€?
### 闂細褰撲竴涓ˉ涓佽搴旂敤鍒?bpf 鎴?bpf-next 鏍戞椂鎰忓懗鐫€浠€涔堬紵


绛旓細杩欐剰鍛崇潃浠?BPF 鐨勮搴︾湅锛岃琛ヤ竵鐪嬭捣鏉ラ€傚悎杩涘叆涓荤嚎銆?
浣嗚娉ㄦ剰锛岃繖骞朵笉绛変簬琛ヤ竵鏈€缁堜細鑷姩琚?net 鎴?net-next 鏍戞帴鍙楃殑瀹氳锛?
鍦?bpf 鍐呮牳閭欢鍒楄〃涓婏紝瀹℃煡鍙互闅忔椂鍒版潵銆傚鏋滃洿缁曟煇涓ˉ涓佺殑璁ㄨ寰楀嚭缁撹璁や负瀹冧笉鑳芥寜鍘熸牱琚帴鍙楋紝鎴戜滑瑕佷箞浼氬簲鐢ㄤ竴涓悗缁殑淇锛岃涔堜細灏嗗叾浠庢爲涓畬鍏ㄤ涪寮冦€傚洜姝わ紝鎴戜滑涔熶繚鐣欏湪璁や负鏈夊繀瑕佹椂瀵规爲杩涜鍙樺熀鐨勬潈鍒┿€傛瘯绔燂紝璇ユ爲鐨勭洰鐨勬槸锛?
i) 绱Н骞舵殏瀛?BPF 琛ヤ竵锛屼互渚块泦鎴愬埌璇稿 net 鍜?net-next 涔嬬被鐨勬爲涓紝浠ュ強

ii) 鍦ㄨˉ涓佽繘涓€姝ュ墠杩涗箣鍓嶏紝瀵瑰叾杩愯骞挎硾鐨?BPF 娴嬭瘯濂椾欢鍜屽伐浣滆礋杞姐€?
涓€鏃?BPF pull request 琚?David S. Miller 鎺ュ彈锛岃ˉ涓佸氨浼氬垎鍒繘鍏?net 鎴?net-next 鏍戯紝骞朵粠閭ｉ噷杩涗竴姝ヨ繘鍏ヤ富绾裤€傚悓鏍凤紝鍏充簬瀹冧滑澶氫箙鍚堝苟鍒颁富绾跨殑鏇村淇℃伅锛岃鍙傞槄 netdev 瀛愮郴缁熺殑鏂囨。 Documentation/process/maintainer-netdev.rst銆?
### 闂細鎴戦渶瑕佺瓑寰呭闀挎椂闂存墠鑳芥敹鍒板叧浜?BPF 琛ヤ竵鐨勫弽棣堬紵


绛旓細鎴戜滑灏介噺淇濇寔杈冧綆鐨勫欢杩熴€傞€氬父缁欏嚭鍙嶉鐨勬椂闂寸害涓?2 鎴?3 涓伐浣滄棩銆傚畠鍙兘浼氭牴鎹洿鏀圭殑澶嶆潅鎬у拰褰撳墠鐨勮ˉ涓佽礋杞借€屽彉鍖栥€?
### 闂細浣犱滑澶氫箙鍚?net 鎴?net-next 涔嬬被鐨勪富瑕佸唴鏍告爲鍙戦€佷竴娆?pull request锛?

绛旓細涓轰簡涓嶈 bpf 鎴?bpf-next 涓疮绉繃澶氳ˉ涓侊紝浼氱浉褰撻绻佸湴鍙戦€?pull request銆?
浣滀负缁忛獙娉曞垯锛岄璁℃瘡涓爲閮戒細瀹氭湡鍦ㄥ懆鏈彂閫?pull request銆傚湪鏌愪簺鎯呭喌涓嬶紝鏍规嵁褰撳墠鐨勮ˉ涓佽礋杞芥垨绱ф€ョ▼搴︼紝pull request 涔熷彲鑳藉湪鍛ㄤ腑棰濆鍙戝嚭銆?
### 闂細鍦ㄥ悎骞剁獥鍙ｅ紑鍚椂锛岃ˉ涓佷細琚簲鐢ㄥ埌 bpf-next 鍚楋紵


绛旓細鍦ㄥ悎骞剁獥鍙ｅ紑鍚湡闂达紝bpf-next 涓嶄細琚鐞嗐€傝繖澶ц嚧绫讳技浜?net-next 琛ヤ竵鐨勫鐞嗘柟寮忥紝鍥犳璇烽殢鎰忛槄璇?netdev 鏂囨。 Documentation/process/maintainer-netdev.rst 浠ヤ簡瑙ｈ繘涓€姝ョ殑缁嗚妭銆?
鍦ㄩ偅涓ゅ懆鐨勫悎骞剁獥鍙ｆ湡闂达紝鎴戜滑鍙兘浼氳姹備綘鍦?bpf-next 鍐嶆寮€鍚悗閲嶆柊鍙戦€佷綘鐨勮ˉ涓佺郴鍒椼€備竴鏃?Linus 鍦ㄥ悎骞剁獥鍙ｄ箣鍚庡彂甯冧簡 `v*-rc1`锛屾垜浠氨缁х画澶勭悊 bpf-next銆?
瀵逛簬娌℃湁璁㈤槄鍐呮牳閭欢鍒楄〃鐨勪汉锛孌avid S. Miller 杩樼淮鎶や簡涓€涓叧浜?net-next 鐨勭姸鎬侀〉闈㈡彁渚涙寚瀵硷細

  http://vger.kernel.org/~davem/net-next.html

### 闂細楠岃瘉鍣ㄦ洿鏀逛笌娴嬭瘯鐢ㄤ緥


闂細鎴戝仛浜?BPF 楠岃瘉鍣ㄧ殑鏇存敼锛岄渶瑕佷负 BPF 鍐呮牳 selftests_ 娣诲姞娴嬭瘯鐢ㄤ緥鍚楋紵

绛旓細濡傛灉琛ヤ竵鏀瑰彉浜嗛獙璇佸櫒鐨勮涓猴紝閭ｄ箞鏄殑锛岀粷瀵规湁蹇呰鍚?BPF 鍐呮牳 selftests_ 濂椾欢娣诲姞娴嬭瘯鐢ㄤ緥銆傚鏋滃畠浠笉瀛樺湪鑰屾垜浠涓洪渶瑕侊紝鎴戜滑鍙兘浼氬湪鎺ュ彈浠讳綍鏇存敼涔嬪墠瑕佹眰鎻愪緵瀹冧滑銆?
鐗瑰埆鏄紝test_verifier.c 璺熻釜鐫€澶ч噺鐨?BPF 娴嬭瘯鐢ㄤ緥锛屽寘鎷?LLVM BPF 鍚庣鍙兘浠庡彈闄?C 浠ｇ爜鐢熸垚鐨勮澶氳竟鐣屾儏鍐点€傚洜姝わ紝娣诲姞娴嬭瘯鐢ㄤ緥瀵逛簬纭繚鏈潵鐨勬洿鏀逛笉浼氭剰澶栧奖鍝嶅厛鍓嶇殑鐢ㄤ緥缁濆鑷冲叧閲嶈銆傚洜姝わ紝璇疯繖鏍风湅寰呰繖浜涙祴璇曠敤渚嬶細鏈敱 test_verifier.c 璺熻釜鐨勯獙璇佸櫒琛屼负鏈夊彲鑳藉彂鐢熷彉鍖栥€?
### 闂細samples/bpf 涓?selftests 鐨勫彇鑸嶏紵


闂細鎴戝簲璇ヤ綍鏃跺悜 `samples/bpf/` 娣诲姞浠ｇ爜锛屽張浣曟椂鍚?BPF 鍐呮牳 selftests_ 娣诲姞浠ｇ爜锛?
绛旓細涓€鑸潵璇达紝鎴戜滑鏇村€惧悜浜庡悜 BPF 鍐呮牳 selftests_ 娣诲姞鍐呭锛岃€屼笉鏄?`samples/bpf/`銆傜悊鐢卞緢绠€鍗曪細鍐呮牳 selftests 浼氳鍚勭鏈哄櫒浜哄畾鏈熻繍琛岋紝浠ユ祴璇曞唴鏍稿洖褰掋€?
鎴戜滑鍚?BPF selftests 娣诲姞鐨勬祴璇曠敤渚嬭秺澶氾紝瑕嗙洊鐜囧氨瓒婂ソ锛屽畠浠鎰忓鐮村潖鐨勫彲鑳芥€у氨瓒婂皬銆傚苟涓嶆槸璇?BPF 鍐呮牳 selftests 涓嶈兘婕旂ず鏌愪釜鐗瑰畾鐗规€у浣曚娇鐢ㄣ€?
璇濊櫧濡傛锛宍samples/bpf/` 鍙兘鏄汉浠叆闂ㄧ殑濂藉湴鏂癸紝鍥犳鎶婄畝鍗曠殑鐗规€ф紨绀烘斁鍏?`samples/bpf/`锛岃€屾妸楂樼骇鍔熻兘鎬у拰杈圭晫鎯呭喌娴嬭瘯鏀惧叆鍐呮牳 selftests 鍙兘鏄伆褰撶殑鍋氭硶銆?
濡傛灉浣犵殑绀轰緥鐪嬭捣鏉ュ儚涓€涓祴璇曠敤渚嬶紝閭ｅ氨鏀圭敤 BPF 鍐呮牳 selftests锛?
### 闂細鎴戝簲璇ヤ綍鏃跺悜 bpftool 娣诲姞浠ｇ爜锛?

绛旓細bpftool锛堜綅浜?tools/bpf/bpftool/ 涓嬶級鐨勪富瑕佺洰鐨勬槸鎻愪緵涓€涓泦涓殑鐢ㄦ埛绌洪棿宸ュ叿锛岀敤浜庤皟璇曞拰鑷渷鍐呮牳涓椿璺冪殑 BPF 绋嬪簭鍜屾槧灏勩€傚鏋滀笌 BPF 鐩稿叧鐨?UAPI 鏇存敼浣垮緱鍙互 dump 绋嬪簭鎴栨槧灏勭殑闄勫姞淇℃伅锛岄偅涔?bpftool 涔熷簲琚墿灞曚互鏀寔 dump 瀹冧滑銆?
### 闂細鎴戝簲璇ヤ綍鏃跺悜 iproute2 鐨?BPF 鍔犺浇鍣ㄦ坊鍔犱唬鐮侊紵


绛旓細瀵逛簬涓?XDP 鎴?tc 灞傦紙渚嬪 `cls_bpf`锛夌浉鍏崇殑 UAPI 鏇存敼锛岀害瀹氭槸杩欎簺鎺у埗璺緞鐩稿叧鐨勬洿鏀逛篃瑕佷粠鐢ㄦ埛绌洪棿涓€渚ф坊鍔犲埌 iproute2 鐨?BPF 鍔犺浇鍣ㄤ腑銆傝繖涓嶄粎鏈夊姪浜庤 UAPI 鏇存敼琚纭璁′负鍙敤锛屼篃鑳借杩欎簺鏇存敼瀵逛富瑕佷笅娓稿彂琛岀増鐨勬洿骞挎硾鐢ㄦ埛缇ゅ彲鐢ㄣ€?
### 闂細浣犱滑涔熸帴鍙楅拡瀵?iproute2 鐨?BPF 鍔犺浇鍣ㄧ殑琛ヤ竵鍚楋紵


绛旓細閽堝 iproute2 鐨?BPF 鍔犺浇鍣ㄧ殑琛ヤ竵蹇呴』鍙戦€佸埌锛?
  netdev@vger.kernel.org

铏界劧杩欎簺琛ヤ竵涓嶇敱 BPF 鍐呮牳缁存姢鑰呭鐞嗭紝浣嗚鎶婁粬浠篃淇濈暀鍦?Cc 涓紝浠ヤ究鑳藉瀹℃煡銆?
iproute2 鐨勫畼鏂?git 浠撳簱鐢?Stephen Hemminger 缁存姢锛屽彲浠ュ湪浠ヤ笅浣嶇疆鎵惧埌锛?
  https://git.kernel.org/pub/scm/linux/kernel/git/shemminger/iproute2.git/

琛ヤ竵闇€瑕佸甫鏈?'``[PATCH iproute2 master]`' 鎴?'`[PATCH iproute2 net-next]`' 鐨勪富棰樺墠缂€銆?`master``' 鎴?'`net-next`' 鎻忚堪琛ヤ竵搴斿綋琚簲鐢ㄥ埌鐨勭洰鏍囧垎鏀€備篃灏辨槸璇达紝濡傛灉鍐呮牳鏇存敼杩涘叆浜?net-next 鍐呮牳鏍戯紝閭ｄ箞鐩稿叧鐨?iproute2 鏇存敼闇€瑕佽繘鍏?iproute2 鐨?net-next 鍒嗘敮锛屽惁鍒欏彲浠ュ皢鐩爣瀹氫负 master 鍒嗘敮銆俰proute2 鐨?net-next 鍒嗘敮浼氬湪褰撳墠鏉ヨ嚜 master 鐨?iproute2 鐗堟湰鍙戝竷鍚庡悎骞跺埌 master 鍒嗘敮銆?
涓?BPF 涓€鏍凤紝杩欎簺琛ヤ竵鏈€缁堜細鍑虹幇鍦?patchwork 鐨?netdev 椤圭洰涓嬶紝骞惰濮旀淳缁?'shemminger' 杩涜杩涗竴姝ュ鐞嗭細

  http://patchwork.ozlabs.org/project/netdev/list/?delegate=389

### 闂細鎻愪氦 BPF 琛ヤ竵涔嬪墠鐨勬渶浣庤姹傛槸浠€涔堬紵


绛旓細鎻愪氦琛ヤ竵鏃讹紝鍔″繀鑺辨椂闂村苟鍦ㄦ彁浜?*涔嬪墠** properly 娴嬭瘯浣犵殑琛ヤ竵銆傚崈涓囦笉瑕佸寙蹇欐彁浜わ紒濡傛灉缁存姢鑰呭彂鐜颁綘鐨勮ˉ涓佹病鏈夌粡杩?proper 娴嬭瘯锛岃繖寰堝鏄撹浠栦滑涓嶆偊銆傛祴璇曡ˉ涓佹彁浜ゆ槸纭€ц姹傦紒

璇锋敞鎰忥紝杩涘叆 bpf 鏍戠殑淇**蹇呴』**鍖呭惈 `Fixes:` 鏍囩銆傞拡瀵?bpf-next 鐨勪慨澶嶅悓鏍峰姝わ紝鍏朵腑鍙楀奖鍝嶇殑鎻愪氦浣嶄簬 net-next锛堟垨鏌愪簺鎯呭喌涓嬬殑 bpf-next锛変腑銆俙Fixes:` 鏍囩瀵逛簬璇嗗埆鍚庣画鎻愪氦鑷冲叧閲嶈锛屽苟涓斿闇€瑕佸仛鍚戝悗绉绘鐨勪汉甯姪鏋佸ぇ锛屽洜姝ゅ畠鏄繀澶囩殑锛?
鎴戜滑涔熶笉鎺ュ彈甯︽湁绌烘彁浜や俊鎭殑琛ヤ竵銆傝姳鏃堕棿 proper 鍦版挵鍐欓珮璐ㄩ噺鐨勬彁浜や俊鎭紝杩欒嚦鍏抽噸瑕侊紒

涓嶅Θ杩欐牱鎯筹細涓€涓湀鍚庢煡鐪嬩綘浠ｇ爜鐨勫叾浠栧紑鍙戣€呴渶瑕佺悊瑙?*涓轰粈涔?*鏌愰」鏇存敼浠ラ偅绉嶆柟寮忓畬鎴愶紝浠ュ強鍘熶綔鑰呭湪鍒嗘瀽鎴栧亣璁句腑鏄惁瀛樺湪缂洪櫡銆傚洜姝わ紝鎻愪緵 proper 鐨勭悊鐢卞苟鎻忚堪鏇存敼鐨勭敤渚嬫槸蹇呴』鐨勩€?
鍖呭惈澶氫簬 1 涓ˉ涓佺殑鎻愪氦蹇呴』鏈変竴灏佸皝闈俊锛屽叾涓寘鍚绯诲垪鐨勯珮灞傛弿杩般€傝繖涓珮灞傛憳瑕侀殢鍚庝細鐢?BPF 缁存姢鑰呮斁鍏ュ悎骞舵彁浜や腑锛屼互渚垮皢鏉ヤ篃鑳戒粠 git 鏃ュ織涓煡闃呫€?
### 闂細鏀瑰彉 BPF JIT 鍜?鎴?LLVM 鐨勭壒鎬?

闂細褰撴柊澧炰竴鏉￠渶瑕?BPF JIT 鍜?鎴?LLVM 闆嗘垚鐨勬寚浠ゆ垨鐗规€ф椂锛屾垜闇€瑕佽€冭檻浠€涔堬紵

绛旓細鎴戜滑鍔姏璁╂墍鏈?BPF JIT 淇濇寔鏈€鏂帮紝浠ヤ究鍦ㄤ笉鍚屾灦鏋勪笂杩愯 BPF 绋嬪簭鏃惰兘澶熶繚璇佺浉鍚岀殑鐢ㄦ埛浣撻獙锛岃€屼笉浼氬湪鍚敤鍐呮牳鍐?BPF JIT 鏃惰绋嬪簭閫€鍥炲埌鏁堢巼杈冧綆鐨勮В閲婂櫒銆?
濡傛灉浣犳棤娉曞疄鐜版垨娴嬭瘯鏌愪簺鏋舵瀯鎵€闇€鐨?JIT 鏇存敼锛岃涓庣浉鍏?BPF JIT 寮€鍙戣€呭悎浣滐紝浠ヤ究鍙婃椂瀹炵幇璇ョ壒鎬с€傝鍙傝€?git 鏃ュ織锛坄arch/*/net/`锛夋潵瀹氫綅鍙互鎻愪緵甯姪鐨勭浉鍏充汉鍛樸€?
鍚屾椂濮嬬粓纭繚涓烘柊鎸囦护娣诲姞 BPF 娴嬭瘯鐢ㄤ緥锛堜緥濡?test_bpf.c 鍜?test_verifier.c锛夛紝浠ヤ究瀹冧滑鑳借幏寰楀箍娉涚殑娴嬭瘯瑕嗙洊锛屽苟甯姪瀵瑰悇涓?BPF JIT 杩涜杩愯鏃舵祴璇曘€?
瀵逛簬鏂扮殑 BPF 鎸囦护锛屼竴鏃︽洿鏀硅鎺ュ彈杩涘叆 Linux 鍐呮牳锛岃鍒?LLVM 鐨?BPF 鍚庣涓疄鐜版敮鎸併€傛洿澶氫俊鎭鍙傞槄涓嬮潰鐨?LLVM_ 涓€鑺傘€?
### 闂細鈥淏PF_INTERNAL鈥?绗﹀彿鍛藉悕绌洪棿鏄仛浠€涔堢敤鐨勶紵


绛旓細浠?BPF_INTERNAL 瀵煎嚭鐨勭鍙峰彧鑳借 BPF 鍩虹璁炬柦浣跨敤锛屼緥濡傚甫鏈?light skeleton 鐨勯鍔犺浇鍐呮牳妯″潡銆侭PF_INTERNAL 涔嬪鐨勫ぇ澶氭暟绗﹀彿涔熶笉鏈熸湜琚?BPF 涔嬪鐨勪唬鐮佷娇鐢ㄣ€傜鍙峰彲鑳界己灏戣鏍囪瘑锛屽洜涓哄畠浠棭浜庡懡鍚嶇┖闂寸殑瀛樺湪锛屾垨鑰呯敱浜庣枏蹇姐€?
## 绋冲畾鐗堟彁浜?

### 闂細鎴戦渶瑕佸湪绋冲畾鐗堝唴鏍镐腑浣跨敤鏌愪釜鐗瑰畾鐨?BPF 鎻愪氦銆傛垜璇ユ€庝箞鍋氾紵


绛旓細濡傛灉浣犻渶瑕佸湪绋冲畾鐗堝唴鏍镐腑浣跨敤鏌愪釜鐗瑰畾鐨勪慨澶嶏紝璇峰厛妫€鏌ヨ鎻愪氦鏄惁宸茬粡搴旂敤鍦ㄧ浉鍏崇殑 `linux-*.y` 鍒嗘敮涓細

  https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux-stable.git/

濡傛灉娌℃湁锛屽垯缁?BPF 缁存姢鑰呭彂涓€灏侀偖浠讹紝骞舵妱閫侊紙Cc锛塶etdev 鍐呮牳閭欢鍒楄〃锛岃姹傚皢璇ヤ慨澶嶆帓闃燂細

  netdev@vger.kernel.org

杩欎釜杩囩▼鎬讳綋涓婁笌 netdev 鏈韩鐩稿悓锛屽彟璇峰弬闃呯綉缁滃瓙绯荤粺鐨勬枃妗?Documentation/process/maintainer-netdev.rst銆?
### 闂細浣犱滑涔熶細鍚戝悗绉绘鍒板綋鍓嶆湭琚綔涓虹ǔ瀹氱増缁存姢鐨勫唴鏍稿悧锛?

绛旓細涓嶄細銆傚鏋滀綘闇€瑕佸湪褰撳墠鏈绋冲畾鐗堢淮鎶よ€呯淮鎶ょ殑鍐呮牳涓娇鐢ㄦ煇涓壒瀹氱殑 BPF 鎻愪氦锛岄偅灏卞彧鑳介潬浣犺嚜宸变簡銆?
褰撳墠鐨勭ǔ瀹氱増鍜岄暱鏈熺ǔ瀹氱増鍐呮牳閮藉垪鍦ㄨ繖閲岋細

  https://www.kernel.org/

### 闂細鎴戝嵆灏嗘彁浜ょ殑 BPF 琛ヤ竵涔熼渶瑕佽繘鍏ョǔ瀹氱増


鎴戣鎬庝箞鍋氾紵

绛旓細瑙勫垯涓?netdev 琛ヤ竵鎻愪氦鐨勪竴鑸鍒欑浉鍚岋紝璇峰弬闃?netdev 鏂囨。 Documentation/process/maintainer-netdev.rst銆?
缁濅笉瑕佸皢 "`Cc: stable@vger.kernel.org`" 娣诲姞鍒拌ˉ涓佹弿杩颁腑锛岃€屾槸璇?BPF 缁存姢鑰呮帓闃熻繖浜涜ˉ涓併€傝繖鍙互鐢ㄤ竴涓敞閲婃潵瀹屾垚锛屼緥濡傛斁鍦ㄨˉ涓佺殑 `---` 閮ㄥ垎涔嬩笅锛堣閮ㄥ垎涓嶄細杩涘叆 git 鏃ュ織锛夈€傛垨鑰咃紝涔熷彲浠ラ€氳繃閭欢鍋氫竴涓畝鍗曠殑璇锋眰鏉ヤ唬鏇裤€?
### 闂細鎺掗槦绋冲畾鐗堣ˉ涓?

闂細鎴戝湪鍝噷鍙互鎵惧埌褰撳墠宸叉帓闃熴€佸皢琚彁浜ゅ埌绋冲畾鐗堢殑 BPF 琛ヤ竵锛?
绛旓細涓€鏃︿慨澶嶄弗閲嶇己闄风殑琛ヤ竵琚簲鐢ㄥ埌 bpf 鏍戯紝瀹冧滑灏变細鍦ㄤ互涓嬩綅缃帓闃熺瓑寰呮彁浜ゅ埌绋冲畾鐗堬細

  http://patchwork.ozlabs.org/bundle/bpf/stable/?state=*

瀹冧滑鑷冲皯浼氫竴鐩村湪閭ｉ噷鎼佺疆锛岀洿鍒扮浉鍏崇殑鎻愪氦杩涘叆涓荤嚎鍐呮牳鏍戙€?
鍦ㄧ粡鍘嗕簡鏇村箍娉涚殑鏇濆厜涔嬪悗锛屾帓闃熺殑琛ヤ竵浼氱敱 BPF 缁存姢鑰呮彁浜ょ粰绋冲畾鐗堢淮鎶よ€呫€?
## 娴嬭瘯琛ヤ竵


### 闂細濡備綍杩愯 BPF selftests


绛旓細鍦ㄤ綘鍚姩杩涘叆鏂扮紪璇戠殑鍐呮牳涔嬪悗锛岃繘鍏?BPF selftests_ 濂椾欢浠ユ祴璇?BPF 鍔熻兘锛堝綋鍓?
```

  $ cd tools/testing/selftests/bpf/
  $ make

```
```

  $ sudo ./test_verifier

```
楠岃瘉鍣ㄦ祴璇曚細鎵撳嵃鍑哄綋鍓嶆鍦ㄦ墽琛岀殑鎵€鏈夋鏌ャ€傝繍琛屾墍鏈夋祴璇曠粨鏉熸椂鐨勬憳瑕佷細 dump

```

  Summary: 418 PASSED, 0 FAILED

```
涓轰簡杩愯鍏ㄩ儴 BPF selftests锛屼互涓嬪懡浠や负

```

  $ sudo make run_tests

```
鏈夊叧璇︾粏淇℃伅锛岃鍙傞槄 [kernel selftest documentation </dev-tools/kselftest>](kernel selftest documentation </dev-tools/kselftest>)銆?
涓轰簡浣块€氳繃鐨勬祴璇曟暟閲忔渶澶у寲锛岃娴嬪唴鏍哥殑 .config 搴斿敖鍙兘涓?tools/testing/selftests/bpf 涓殑閰嶇疆鏂囦欢鐗囨鍖归厤銆?
鏈€鍚庯紝涓虹‘淇濇敮鎸佹渶鏂扮殑 BPF Type Format 鐗规€э紙鍦?Documentation/bpf/btf.rst 涓璁猴級锛屽浜庝互 CONFIG_DEBUG_INFO_BTF=y 鏋勫缓鐨勫唴鏍革紝闇€瑕?pahole 1.16 鐗堟湰銆俻ahole 鐢?dwarves 鍖呮彁渚涳紝涔熷彲浠ヤ粠浠ヤ笅浣嶇疆浠庢簮鐮佹瀯寤猴細

https://github.com/acmel/dwarves

pahole 浠?v1.13 璧枫€佸湪鎻愪氦 21507cd3e97b锛堚€減ahole: add libbpf as submodule under lib/bpf鈥濓級涔嬪悗寮€濮嬩娇鐢?libbpf 鐨勫畾涔夊拰 API銆傚畠涓?git 浠撳簱閰嶅悎鑹ソ锛屽洜涓?libbpf 瀛愭ā鍧椾細浣跨敤 鈥済it submodule update --init --recursive鈥?鏉ユ洿鏂般€?
涓嶅垢鐨勬槸锛実ithub 榛樿鐨勫彂甯冩簮浠ｇ爜涓嶅寘鍚?libbpf 瀛愭ā鍧楁簮浠ｇ爜锛岃繖浼氬鑷存瀯寤洪棶棰橈紱鏉ヨ嚜 https://git.kernel.org/pub/scm/devel/pahole/pahole.git/ 鐨?tarball 涓?github 鐩稿悓锛屼綘鍙互浠庝互涓嬩綅缃幏鍙栧甫鏈夌浉搴?libbpf 瀛愭ā鍧椾唬鐮佺殑婧愮爜 tarball锛?
https://fedorapeople.org/~acme/dwarves

鏌愪簺鍙戣鐗堝凡缁忔墦鍖呬簡 pahole 1.16 鐗堟湰锛屼緥濡?Fedora銆丟entoo銆?
### 闂細鎴戝簲璇ラ拡瀵瑰摢涓?BPF 鍐呮牳 selftests 鐗堟湰鏉ヨ繍琛屾垜鐨勫唴鏍革紵


绛旓細濡傛灉浣犺繍琛岀殑鏄唴鏍?`xyz`锛岄偅涔堜篃濮嬬粓杩愯鏉ヨ嚜璇ュ唴鏍?`xyz` 鐨?BPF 鍐呮牳 selftests銆備笉瑕佹寚鏈涙潵鑷渶鏂颁富绾挎爲鐨?BPF selftest 浼氫竴鐩村叏閮ㄩ€氳繃銆?
鐗瑰埆鏄紝test_bpf.c 鍜?test_verifier.c 鏈夊ぇ閲忔祴璇曠敤渚嬶紝骞朵笖浼氶殢鏂扮殑 BPF 娴嬭瘯搴忓垪涓嶆柇鏇存柊锛屾垨鑰呯幇鏈夌敤渚嬩細閫傚簲鎬у湴淇敼浠ラ厤鍚堥獙璇佸櫒鐨勬洿鏀癸紙渚嬪鐢变簬楠岃瘉鍣ㄥ彉寰楁洿鏅鸿兘銆佽兘澶熸洿濂藉湴璺熻釜鏌愪簺涓滆タ锛夈€?
## LLVM


### 闂細鎴戝湪鍝噷鍙互鎵惧埌鏀寔 BPF 鐨?LLVM锛?

绛旓細LLVM 鐨?BPF 鍚庣鑷増鏈?3.7.1 璧峰氨鏄?LLVM 鐨勪笂娓镐唬鐮併€?
濡備粖鎵€鏈変富瑕佺殑鍙戣鐗堥兘鍙戝竷浜嗗惎鐢ㄤ簡 BPF 鍚庣鐨?LLVM锛屽洜姝ゅ浜庣粷澶у鏁扮敤渚嬶紝涓嶅啀闇€瑕佹墜宸ョ紪璇?LLVM锛屽彧闇€瀹夎鍙戣鐗堟彁渚涚殑鍖呭嵆鍙€?
LLVM 鐨勯潤鎬佺紪璇戝櫒閫氳繃浠ヤ笅鏂瑰紡鍒楀嚭鍙楁敮鎸佺殑鐩爣

```

     $ llc --version
     LLVM (http://llvm.org/):
       LLVM version 10.0.0
       Optimized build.
       Default target: x86_64-unknown-linux-gnu
       Host CPU: skylake

       Registered Targets:
         aarch64    - AArch64 (little endian)
         bpf        - BPF (host endian)
         bpfeb      - BPF (big endian)
         bpfel      - BPF (little endian)
         x86        - 32-bit X86: Pentium-Pro and above
         x86-64     - 64-bit X86: EM64T and AMD64

```
涓轰簡璁╁紑鍙戣€呰兘澶熷埄鐢ㄦ坊鍔犲埌 LLVM BPF 鍚庣鐨勬渶鏂扮壒鎬э紝寤鸿杩愯鏈€鏂扮殑 LLVM 鐗堟湰銆傚鏂?BPF 鍐呮牳鐗规€э紙渚嬪瀵?BPF 鎸囦护闆嗙殑澧炶ˉ锛夌殑鏀寔閫氬父鏄竴鍚屽紑鍙戠殑銆?
鎵€鏈?LLVM 鐗堟湰閮藉彲浠ュ湪浠ヤ笅浣嶇疆鎵惧埌锛歨ttp://releases.llvm.org/

### 闂細鏄庣櫧浜嗭紝閭ｆ垜鍒板簳璇ュ浣曟墜鍔ㄦ瀯寤?LLVM锛?

绛旓細鎴戜滑寤鸿甯屾湜鑾峰緱鏈€蹇閲忔瀯寤虹殑寮€鍙戣€呬娇鐢?Ninja 鏋勫缓绯荤粺锛屼綘鍙互鍦ㄧ郴缁熺殑鍖呯鐞嗗櫒涓壘鍒板畠锛岄€氬父鍖呭悕鏄?ninja 鎴?ninja-build銆?
浣犻渶瑕?ninja銆乧make 鍜?gcc-c++ 浣滀负 LLVM 鐨勬瀯寤哄厛鍐虫潯浠躲€備竴鏃﹁缃ソ锛屽氨鐫€鎵嬫瀯寤烘渶鏂扮殑 LLVM 鍜?clang 鐗堟湰

```

     $ git clone https://github.com/llvm/llvm-project.git
     $ mkdir -p llvm-project/llvm/build
     $ cd llvm-project/llvm/build
     $ cmake .. -G "Ninja" -DLLVM_TARGETS_TO_BUILD="BPF;X86" \
                -DLLVM_ENABLE_PROJECTS="clang"    \
                -DCMAKE_BUILD_TYPE=Release        \
                -DLLVM_BUILD_RUNTIME=OFF
     $ ninja

```
鏋勫缓濂界殑浜岃繘鍒舵枃浠堕殢鍚庡彲浠ュ湪 build/bin/ 鐩綍涓壘鍒帮紝浣犲彲浠ュ皢 PATH 鍙橀噺鎸囧悜閭ｉ噷銆?
灏?`-DLLVM_TARGETS_TO_BUILD` 璁剧疆涓轰綘甯屾湜鏋勫缓鐨勭洰鏍囷紝浣犲彲浠ュ湪 llvm-project/llvm/lib/Target 鐩綍涓壘鍒板畬鏁寸殑鐩爣鍒楄〃銆?
### 闂細鎶ュ憡 LLVM BPF 闂


闂細鎴戞槸鍚﹀簲璇ュ氨 LLVM 鐨?BPF 浠ｇ爜鐢熸垚鍚庣鐨勯棶棰橈紝鎴栬€呭叧浜庨獙璇佸櫒鎷掔粷鎺ュ彈鐨?LLVM 鐢熸垚浠ｇ爜锛岄€氱煡 BPF 鍐呮牳缁存姢鑰咃紵

绛旓細鏄殑锛岃鍔″繀閫氱煡锛?
LLVM 鐨?BPF 鍚庣鏄暣涓?BPF 鍩虹璁炬柦鐨勫叧閿儴鍒嗭紝骞朵笖涓庢潵鑷唴鏍镐竴渚х殑绋嬪簭楠岃瘉娣卞害缁戝畾銆傚洜姝わ紝浠讳綍涓€渚х殑闂閮介渶瑕佸湪蹇呰鏃惰繘琛岃皟鏌ュ拰淇銆?
鍥犳锛岃纭繚鍦?netdev 鍐呮牳閭欢鍒楄〃涓婃彁鍑鸿繖浜涢棶棰橈紝骞舵妸璐熻矗 LLVM 鍜屽唴鏍搁儴鍒嗙殑 BPF 缁存姢鑰呭姞鍒?Cc锛?
- Yonghong Song <yhs@fb.com>
- Alexei Starovoitov <ast@kernel.org>
- Daniel Borkmann <daniel@iogearbox.net>

LLVM 涔熸湁涓€涓?issue 璺熻釜鍣紝鍙互鍦ㄥ叾涓壘鍒?BPF 鐩稿叧鐨勭己闄凤細

  https://bugs.llvm.org/buglist.cgi?quicksearch=bpf

涓嶈繃锛屾渶濂借繕鏄€氳繃閭欢鍒楄〃鑱旂郴锛屽苟鎶婄淮鎶よ€呭姞鍦?Cc 涓€?
### 闂細鍐呮牳涓?LLVM 鐨勬柊 BPF 鎸囦护


闂細鎴戝悜鍐呮牳娣诲姞浜嗕竴鏉℃柊鐨?BPF 鎸囦护锛屽浣曞皢鍏堕泦鎴愬埌 LLVM 涓紵

绛旓細LLVM 涓?BPF 鍚庣鎻愪緵浜嗕竴涓?`-mcpu` 閫夋嫨鍣紝浠ヤ究鍏佽閫夋嫨 BPF 鎸囦护闆嗘墿灞曘€傚湪 llvm 20 鐗堟湰涔嬪墠锛屼娇鐢?`generic` 澶勭悊鍣ㄧ洰鏍囷紝鍗?BPF 鐨勫熀纭€鎸囦护闆嗭紙v1锛夈€備粠 llvm 20 璧凤紝榛樿澶勭悊鍣ㄧ洰鏍囧凡鏇存敼涓烘寚浠ら泦 v3銆?
LLVM 鏈変竴涓€夐」鍙互閫夋嫨 `-mcpu=probe`锛屽畠浼氭帰娴嬪涓绘満鍐呮牳浠ヨ幏寰楀彈鏀寔鐨?BPF 鎸囦护闆嗘墿灞曪紝骞惰嚜鍔ㄩ€夋嫨鏈€浼橀泦鍚堛€?
```

     $ llc -march bpf -mcpu=help
     Available CPUs for this target:

       generic - Select the generic processor.
       probe   - Select the probe processor.
       v1      - Select the v1 processor.
       v2      - Select the v2 processor.
     [...]

```
鍚?Linux 鍐呮牳鏂版坊鍔犵殑 BPF 鎸囦护闇€瑕侀伒寰浉鍚岀殑鏂规锛屾彁鍗囨寚浠ら泦鐗堟湰骞朵负杩欎簺鎵╁睍瀹炵幇鎺㈡祴锛屼互渚?`-mcpu=probe` 鐢ㄦ埛鑳藉湪鍗囩骇鍐呮牳鏃堕€忔槑鍦板彈鐩婁簬璇ヤ紭鍖栥€?
濡傛灉浣犳棤娉曞疄鐜板鏂版坊鍔犵殑 BPF 鎸囦护鐨勬敮鎸侊紝璇峰悜 BPF 寮€鍙戣€呭姹傚府鍔┿€?
椤轰究涓€鎻愶紝BPF 鍐呮牳 selftests 浠?`-mcpu=probe` 杩愯浠ヨ幏寰楁洿濂界殑娴嬭瘯瑕嗙洊銆?
### 闂細閽堝 bpf 鐩爣鐨?clang 鏍囧織锛?

闂細鏌愪簺鎯呭喌涓嬩娇鐢?clang 鏍囧織 `--target=bpf`锛岃€屽湪鍏朵粬鎯呭喌涓嬩娇鐢ㄤ笌搴曞眰鏋舵瀯鍖归厤鐨勯粯璁?clang 鐩爣銆傚尯鍒槸浠€涔堬紝鎴戝簲璇ヤ綍鏃朵娇鐢ㄥ摢涓€涓紵

绛旓細灏界 LLVM IR 鐢熸垚鍜屼紭鍖栧敖閲忎繚鎸佹灦鏋勬棤鍏筹紝浣?`--target=<arch>` 浠嶇劧瀵圭敓鎴愮殑浠ｇ爜鏈変竴瀹氬奖鍝嶏細

- BPF 绋嬪簭鍙兘浼氶€掑綊鍖呭惈甯︽湁鏂囦欢浣滅敤鍩熷唴鑱旀眹缂栦唬鐮佺殑澶存枃浠躲€傞粯璁ょ洰鏍囧彲浠ュ緢濂藉湴澶勭悊杩欎竴鐐癸紝鑰?`bpf` 鐩爣鍙兘浼氬け璐ワ紝濡傛灉 bpf 鍚庣姹囩紪鍣ㄤ笉鐞嗚В杩欎簺姹囩紪浠ｇ爜锛堝ぇ澶氭暟鎯呭喌涓嬬‘瀹炲姝わ級銆?
- 褰撲笉浣跨敤 `-g` 缂栬瘧鏃讹紝榛樿鐩爣鐨勫璞℃枃浠朵腑鍙兘浼氬瓨鍦ㄩ澶栫殑 elf 鑺傦紝渚嬪 .eh_frame 鍜?.rela.eh_frame锛岃€?`bpf` 鐩爣鍒欎笉浼氥€?
- 榛樿鐩爣鍙兘浼氭妸 C 鐨?switch 璇彞杞崲鎴?switch 琛ㄦ煡鎵惧拰璺宠浆鎿嶄綔銆傜敱浜?switch 琛ㄨ鏀惧湪鍏ㄥ眬鍙鑺備腑锛宐pf 绋嬪簭灏嗘棤娉曞姞杞姐€俙bpf` 鐩爣涓嶆敮鎸?switch 琛ㄤ紭鍖栥€傚彲浠ヤ娇鐢?clang 閫夐」 `-fno-jump-tables` 鏉ョ鐢?switch 琛ㄧ殑鐢熸垚銆?
- 瀵逛簬 clang `--target=bpf`锛屾棤璁哄簳灞?clang 浜岃繘鍒舵垨榛樿鐩爣锛堟垨鍐呮牳锛夋槸鍚︿负 32 浣嶏紝閮戒繚璇佹寚閽堟垨 long / unsigned long 绫诲瀷濮嬬粓鍏锋湁 64 浣嶅搴︺€傜劧鑰岋紝褰撲娇鐢ㄥ師鐢?clang 鐩爣鏃讹紝瀹冧細鏍规嵁杩欎簺绫诲瀷鍩轰簬搴曞眰鏋舵瀯鐨勭害瀹氳繘琛岀紪璇戯紝涔熷氨鏄鍦?32 浣嶆灦鏋勭殑鎯呭喌涓嬶紝鎸囬拡鎴?long / unsigned long 绫诲瀷锛堜緥濡傚湪 BPF 涓婁笅鏂囩粨鏋勪腑锛夊皢鍏锋湁 32 浣嶅搴︼紝鑰?BPF LLVM 鍚庣浠嶄互 64 浣嶈繍琛屻€傚師鐢熺洰鏍囦富瑕佸湪璺熻釜涓渶瑕侀亶鍘?`pt_regs` 鎴栧叾浠?CPU 瀵勫瓨鍣ㄥ搴︾浉鍏崇殑鍐呮牳缁撴瀯鏃朵娇鐢ㄣ€傚惁鍒欙紝閫氬父鎺ㄨ崘浣跨敤 `clang --target=bpf`銆?
鍦ㄤ互涓嬫儏鍐典笅锛屼綘搴旇浣跨敤榛樿鐩爣锛?
- 浣犵殑绋嬪簭鍖呭惈鏌愪釜澶存枃浠讹紙渚嬪 ptrace.h锛夛紝瀹冩渶缁堝紩鍏ヤ簡鏌愪簺鍖呭惈鏂囦欢浣滅敤鍩熶富鏈烘眹缂栦唬鐮佺殑澶存枃浠躲€?
- 浣犲彲浠ユ坊鍔?`-fno-jump-tables` 鏉ヨВ鍐?switch 琛ㄩ棶棰樸€?
鍚﹀垯锛屼綘鍙互浣跨敤 `bpf` 鐩爣銆傛澶栵紝鍦ㄤ互涓嬫儏鍐典笅浣?*蹇呴』**浣跨敤 bpf 鐩爣锛?
- 浣犵殑绋嬪簭浣跨敤浜嗗甫鏈夋寚閽堟垨 long / unsigned long 绫诲瀷銆佸苟涓?BPF 杈呭姪鍑芥暟鎴栦笂涓嬫枃鏁版嵁缁撴瀯浜や簰鐨勬暟鎹粨鏋勩€傚杩欎簺缁撴瀯鐨勮闂敱 BPF 楠岃瘉鍣ㄩ獙璇侊紝濡傛灉鍘熺敓鏋舵瀯涓?BPF 鏋舵瀯锛堜緥濡?64 浣嶏級涓嶄竴鑷达紝鍙兘浼氬鑷撮獙璇佸け璐ャ€傝繖鏂归潰鐨勪竴涓緥瀛愭槸 BPF_PROG_TYPE_SK_MSG 闇€瑕?`--target=bpf`銆?

   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/testing/selftests/bpf/

BPF 寮€鍙戞剦蹇紒
