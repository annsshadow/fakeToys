## 鍒涘缓鎷夊彇璇锋眰


鏈珷鎻忚堪缁存姢鑰呭浣曞垱寤哄苟鍚戝叾瀹冪淮鎶よ€呮彁浜ゆ媺鍙栬姹傘€傝繖瀵逛簬灏嗕竴涓淮鎶よ€呮爲涓殑鏇存敼
杞Щ鍒板彟涓€涓淮鎶よ€呮爲涓緢鏈夌敤銆?
鏈枃妗ｄ富瑕佺敱 Tobin C. Harding锛堝郊鏃朵粬杩樹笉鏄竴浣嶇粡楠屼赴瀵岀殑缁存姢鑰咃級鏍规嵁 Greg
Kroah-Hartman 鍜?Linus Torvalds 鍦?LKML 涓婄殑璇勮鎾板啓銆傜敱 Jonathan Corbet 鍜?Mauro Carvalho Chehab 鎻愪緵寤鸿鍜屼慨姝ｃ€傝瑙ｅ苟闈炴湁鎰忎絾涓嶅彲閬垮厤锛岃灏嗘寚璐ｆ寚鍚?Tobin C. Harding <me@tobin.cc>銆?
```

	https://lore.kernel.org/r/20171114110500.GA21175@kroah.com


```
### 鍒涘缓鍒嗘敮


棣栧厛锛屼綘闇€瑕佸皢甯屾湜鍖呭惈鍦ㄦ媺鍙栬姹備腑鐨勬墍鏈夋洿鏀规斁鍦ㄤ竴涓崟鐙殑鍒嗘敮涓娿€傞€氬父浣犱細鍩轰簬
浣犳墦绠楀彂閫佹媺鍙栬姹傜殑寮€鍙戣€呮爲涓殑鏌愪釜鍒嗘敮鏉ュ垱寤烘鍒嗘敮銆?
涓轰簡鍒涘缓鎷夊彇璇锋眰锛屼綘蹇呴』棣栧厛涓轰綘鍒氬垰鍒涘缓鐨勫垎鏀墦涓婃爣绛俱€傚缓璁綘閫夋嫨涓€涓湁鎰忎箟鐨?鏍囩鍚嶏紝浠ヤ竴绉嶄綘鍜屼粬浜哄嵆浣胯繃涓€娈垫椂闂翠篃鑳界悊瑙ｇ殑鏂瑰紡銆備竴涓ソ鐨勫仛娉曟槸锛屽湪鍚嶇О涓寘鍚?鏉ユ簮瀛愮郴缁熺殑鎸囩ず绗︿互鍙婄洰鏍囧唴鏍哥増鏈€?
Greg 鎻愪緵浜嗗涓嬪缓璁€備竴涓寘鍚?drivers/char 鍚勭鏉傞」鍐呭銆佽鍦ㄥ唴鏍哥増鏈?4.15-rc1
搴旂敤鐨勬媺鍙栬姹傚彲浠ュ懡鍚嶄负 `char-misc-4.15-rc1`銆傚鏋滆繖鏍蜂竴涓爣绛炬槸浠庝竴涓垎鏀骇鐢熺殑
```

        git tag -s char-misc-4.15-rc1 char-misc-next

```
杩欏皢鍒涘缓涓€涓悕涓?`char-misc-4.15-rc1`銆佸熀浜?`char-misc-next` 鍒嗘敮鏈€鍚庝竴涓彁浜ょ殑
甯︾鍚嶆爣绛撅紝骞剁敤浣犵殑 gpg 瀵嗛挜绛惧悕锛堝弬瑙?Documentation/maintainer/configure-git.rst锛夈€?
Linus 鍙帴鍙楀熀浜庡甫绛惧悕鏍囩鐨勬媺鍙栬姹傘€傚叾瀹冪淮鎶よ€呭彲鑳芥湁鎵€涓嶅悓銆?
褰撲綘杩愯涓婅堪鍛戒护鏃讹紝`git` 浼氳浣犺繘鍏ヤ竴涓紪杈戝櫒锛屽苟瑕佹眰浣犳弿杩拌鏍囩銆傚湪杩欑鎯呭喌涓嬶紝
浣犳槸鍦ㄦ弿杩颁竴涓媺鍙栬姹傦紝鎵€浠ユ杩拌繖閲屽寘鍚粈涔堛€佷负浠€涔堝簲璇ュ悎骞讹紝浠ュ強锛堝鏋滄湁鐨勮瘽锛?鍋氫簡浠€涔堟祴璇曘€傛墍鏈夎繖浜涗俊閮藉皢鏈€缁堣繘鍏ユ爣绛炬湰韬紝鐒跺悗杩涘叆缁存姢鑰呭湪鍚堝苟鎷夊彇璇锋眰鏃讹紙濡傛灉/
褰撲粬浠悎骞舵椂锛夋墍鍋氱殑鍚堝苟鎻愪氦涓€傛墍浠ヨ鎶婂畠鍐欏ソ锛屽洜涓哄畠灏嗘案杩滅暀鍦ㄥ唴鏍告爲涓€?
```

	Anyway, at least to me, the important part is the *message*. I want
	to understand what I'm pulling, and why I should pull it. I also
	want to use that message as the message for the merge, so it should
	not just make sense to me, but make sense as a historical record
	too.

	Note that if there is something odd about the pull request, that
	should very much be in the explanation. If you're touching files
	that you don't maintain, explain _why_. I will see it in the
	diffstat anyway, and if you didn't mention it, I'll just be extra
	suspicious.  And when you send me new stuff after the merge window
	(or even bug-fixes, but ones that look scary), explain not just
	what they do and why they do it, but explain the _timing_. What
	happened that this didn't go through the merge window..

	I will take both what you write in the email pull request _and_ in
	the signed tag, so depending on your workflow, you can either
	describe your work in the signed tag (which will also automatically
	make it into the pull request email), or you can make the signed
	tag just a placeholder with nothing interesting in it, and describe
	the work later when you actually send me the pull request.

	And yes, I will edit the message. Partly because I tend to do just
	trivial formatting (the whole indentation and quoting etc), but
	partly because part of the message may make sense for me at pull
	time (describing the conflicts and your personal issues for sending
	it right now), but may not make sense in the context of a merge
	commit message, so I will try to make it all make sense. I will
	also fix any speeling mistaeks and bad grammar I notice,
	particularly for non-native speakers (but also for native ones
	;^). But I may miss some, or even add some.

			Linus

```
```

	Char/Misc patches for 4.15-rc1

	Here is the big char/misc patch set for the 4.15-rc1 merge window.
	Contained in here is the normal set of new functions added to all
	of these crazy drivers, as well as the following brand new
	subsystems:
		- time_travel_controller: Finally a set of drivers for the
		  latest time travel bus architecture that provides i/o to
		  the CPU before it asked for it, allowing uninterrupted
		  processing
		- relativity_shifters: due to the affect that the
		  time_travel_controllers have on the overall system, there
		  was a need for a new set of relativity shifter drivers to
		  accommodate the newly formed black holes that would
		  threaten to suck CPUs into them.  This subsystem handles
		  this in a way to successfully neutralize the problems.
		  There is a Kconfig option to force these to be enabled
		  when needed, so problems should not occur.

	All of these patches have been successfully tested in the latest
	linux-next releases, and the original problems that it found have
	all been resolved (apologies to anyone living near Canberra for the
	lack of the Kconfig options in the earlier versions of the
	linux-next tree creations.)

	Signed-off-by: Your-name-here <your_email@domain>


```
鏍囩娑堟伅鏍煎紡灏卞儚 git 鎻愪氦 id 涓€鏍枫€傞《閮ㄤ竴琛屼綔涓衡€滄憳瑕佷富棰樷€濓紝骞剁‘淇濆湪搴曢儴绛惧悕銆?
鐜板湪浣犳湁浜嗕竴涓湰鍦板甫绛惧悕鏍囩锛屼綘闇€瑕佸皢鍏舵帹閫佸埌涓€涓叕寮€浣嶇疆
```

	git push origin char-misc-4.15-rc1


```
### 鍒涘缓鎷夊彇璇锋眰


鏈€鍚庤鍋氱殑鏄垱寤烘媺鍙栬姹傛秷鎭€俙git` 鍙互寰堟柟渚垮湴閫氳繃 `git request-pull` 鍛戒护涓轰綘
瀹屾垚锛屼絾瀹冮渶瑕佷竴鐐瑰府鍔╂潵纭畾浣犳兂鎷夊彇浠€涔堬紝浠ュ強鍩轰簬浠€涔堣繘琛屾媺鍙栵紙浠ユ樉绀烘纭殑寰呮媺鍙?鏇存敼鍜?diffstat锛夈€?```

	git request-pull master git://git.kernel.org/pub/scm/linux/kernel/git/gregkh/char-misc.git/ char-misc-4.15-rc1

```
```

	This is asking git to compare the difference from the
	'char-misc-4.15-rc1' tag location, to the head of the 'master'
	branch (which in my case points to the last location in Linus's
	tree that I diverged from, usually a -rc release) and to use the
	git:// protocol to pull from.  If you wish to use https://, that
	can be used here instead as well (but note that some people behind
	firewalls will have problems with https git pulls).

	If the char-misc-4.15-rc1 tag is not present in the repo that I am
	asking to be pulled from, git will complain saying it is not there,
	a handy way to remember to actually push it to a public location.

	The output of 'git request-pull' will contain the location of the
	git tree and specific tag to pull from, and the full text
	description of that tag (which is why you need to provide good
	information in that tag).  It will also create a diffstat of the
	pull request, and a shortlog of the individual commits that the
	pull request will provide.

```
Linus 鍥炲簲璇翠粬鍊惧悜浜庡亸濂?`git://` 鍗忚銆傚叾瀹冪淮鎶よ€呭彲鑳芥湁涓嶅悓鐨勫亸濂姐€傛澶栵紝璇锋敞鎰忥紝
濡傛灉浣犲湪娌℃湁甯︾鍚嶆爣绛剧殑鎯呭喌涓嬪垱寤烘媺鍙栬姹傦紝閭ｄ箞 `https://` 鍙兘鏄洿濂界殑閫夋嫨銆傚畬鏁?璁ㄨ璇峰弬闃呭師濮嬮偖浠剁嚎绋嬨€?

### 鎻愪氦鎷夊彇璇锋眰


鎷夊彇璇锋眰鐨勬彁浜ゆ柟寮忎笌鏅€氱殑琛ヤ竵鐩稿悓銆備綔涓哄唴鑱旈偖浠跺彂閫佺粰缁存姢鑰咃紝骞舵妱閫?LKML 浠ュ強浠讳綍
瀛愮郴缁熺壒瀹氱殑鍒楄〃锛堝鏋滈渶瑕侊級銆傚悜 Linus 鎻愪氦鐨勬媺鍙栬姹傞€氬父鍏锋湁浠ヤ笅涓婚琛?```

	[GIT PULL] <subsystem> changes for v4.15-rc1

```
