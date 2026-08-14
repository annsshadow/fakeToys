
澶勭悊鍥炲綊闂
++++++++++++

**鎴戜滑涓嶄細寮曞叆鍥炲綊锛坮egression锛?* 鈥斺€?鏈枃妗ｆ弿杩颁簡杩欐潯鈥淟inux 鍐呮牳寮€鍙戠涓€鍑嗗垯鈥濆寮€鍙戣€呭湪瀹炶返涓剰鍛崇潃浠€涔堛€傚畠涓?Documentation/admin-guide/reporting-regressions.rst 浜掍负琛ュ厖锛屽悗鑰呬粠鐢ㄦ埛瑙嗚闃愯堪璇ヤ富棰橈紱濡傛灉浣犱粠鏈杩囬偅绡囨枃妗ｏ紝璇疯嚦灏戝厛娴忚涓€閬嶅啀缁х画寰€涓嬬湅銆?

## 閲嶇偣閫熻锛坅ka "The TL;DR"锛?


#. 纭繚 `regression mailing list <https://lore.kernel.org/regressions/>`_
   (regressions@lists.linux.dev) 鐨勮闃呰€呰兘杩呴€熷緱鐭ヤ换浣曟柊鎻愪氦鐨勫洖褰掓姤鍛婏細

    - 褰撴敹鍒扮殑閭欢鎶ュ憡娌℃湁鎶勯€侊紙CC锛夎鍒楄〃鏃讹紝绔嬪嵆鍙戦€佽嚦灏戜竴灏佺畝鐭殑鈥淩eply-all鈥濓紙鍏ㄩ儴鍥炲锛夛紝骞跺湪鎶勯€佷腑鍔犲叆璇ュ垪琛紝浣垮叾杩涘叆澶勭悊娴佺▼銆?

    - 灏嗙己闄疯窡韪櫒涓彁浜ょ殑浠讳綍鎶ュ憡杞彂鎴栧脊鍥烇紙bounce锛夊埌璇ュ垪琛ㄣ€?

#. 璁?Linux 鍐呮牳鍥炲綊璺熻釜鏈哄櫒浜衡€渞egzbot鈥濊窡韪闂锛堣繖涓€姝ュ彲閫夛紝浣嗗己鐑堟帹鑽愶級锛?

    - 瀵逛簬閭欢鎶ュ憡锛屾鏌ユ姤鍛婅€呮槸鍚﹀寘鍚被浼?``#regzbot
      introduced: v5.13..v5.14-rc1`` 鐨勮銆傚鏋滄病鏈夛紝鍒欏彂閫佷竴灏佸洖澶嶏紙鎶勯€佸洖褰掑垪琛級锛屽叾涓寘鍚涓嬫钀斤紝鐢ㄤ簬鍛婅瘔 regzbot
```
       #regzbot ^introduced: 1f2e3d4c5b6a

    * When forwarding reports from a bug tracker to the regressions list (see
      above), include a paragraph like the following::

       #regzbot introduced: v5.13..v5.14-rc1
       #regzbot from: Some N. Ice Human <some.human@example.com>
       #regzbot monitor: http://some.bugtracker.example.com/ticket?id=123456789

```
#. 鍦ㄦ彁浜ゅ洖褰掍慨澶嶈ˉ涓佹椂锛屽悜琛ヤ竵鎻忚堪涓坊鍔犫€淐loses:鈥濇爣绛撅紝鎸囧悜鎶ュ憡璇ラ棶棰樼殑鎵€鏈変綅缃紝姝ｅ
   Documentation/process/submitting-patches.rst 鍜?
   Documentation/process/5.Posting.rst <development_posting> 鎵€瑕佹眰鐨勯偅鏍枫€傚鏋滀綘鍙慨澶嶄簡瀵艰嚧鍥炲綊鐨勯棶棰樹腑鐨勪竴閮ㄥ垎锛屽彲浠ヤ娇鐢ㄢ€淟ink:鈥濇爣绛句唬鏇裤€俽egzbot 鐩墠瀵逛袱鑰呬笉鍋氬尯鍒嗐€?

#. 涓€鏃︾‘瀹氫簡缃瓉绁搁锛坈ulprit锛夛紝搴斿敖蹇慨澶嶅洖褰掞紱澶у鏁板洖褰掔殑淇搴斿湪涓ゅ懆鍐呭悎鍏ワ紝浣嗘湁浜涢渶瑕佸湪涓や笁澶╁唴瑙ｅ喅銆?


## 涓庡紑鍙戣€呯浉鍏崇殑 Linux 鍐呮牳鍥炲綊鐨勬墍鏈夌粏鑺?


### 閲嶈鍩虹锛屾洿澶氱粏鑺?


#### 鏀跺埌鍥炲綊鎶ュ憡鏃惰鎬庝箞鍋?


纭繚 Linux 鍐呮牳鐨勫洖褰掕窡韪€呬互鍙婂叾浠?`regression mailing list <https://lore.kernel.org/regressions/>`_
(regressions@lists.linux.dev) 鐨勮闃呰€呰兘寰楃煡浠讳綍鏂版姤鍛婄殑鍥炲綊锛?

 - 褰撲綘閫氳繃閭欢鏀跺埌涓€浠芥湭鎶勯€佽鍒楄〃鐨勬姤鍛婃椂锛岀珛鍗冲彂閫佽嚦灏戜竴灏佺畝鐭殑鈥淩eply-all鈥濓紙鍏ㄩ儴鍥炲锛夛紝骞跺湪鎶勯€佷腑鍔犲叆璇ュ垪琛紝浣垮叾杩涘叆澶勭悊娴佺▼锛涘鏋滀綘鍥炲鐨勬槸涓€灏佺渷鐣ヤ簡鍒楄〃鐨勫洖澶嶏紝璇峰敖閲忕‘淇濆垪琛ㄥ啀娆¤鎶勯€併€?

 - 濡傛灉鎻愪氦鍒扮己闄疯窡韪櫒鐨勬姤鍛婅繘鍏ヤ簡浣犵殑鏀朵欢绠憋紝璇峰皢鍏惰浆鍙戞垨寮瑰洖缁欒鍒楄〃銆傚鏋滄姤鍛婅€呭凡鎸夌収
   Documentation/admin-guide/reporting-issues.rst 鐨勬寚绀鸿浆鍙戜簡鎶ュ憡锛屽彲鑰冭檻浜嬪厛鏌ョ湅涓€涓嬪垪琛ㄥ瓨妗ｃ€?

鏃犺閲囩敤涓婅堪鍝鏂瑰紡锛岄兘搴旇€冭檻璁?Linux 鍐呮牳鍥炲綊璺熻釜鏈哄櫒浜衡€渞egzbot鈥濈珛鍗冲紑濮嬭窡韪闂锛?

 - 瀵逛簬閭欢鎶ュ憡锛屾鏌ユ姤鍛婅€呮槸鍚﹀寘鍚簡绫讳技
   `#regzbot introduced: 1f2e3d4c5b6a` 鐨勨€渞egzbot 鍛戒护鈥濄€傚鏋滄病鏈夛紝鍒欏彂閫佷竴灏佸洖澶嶏紙鎶勯€?
```
       #regzbot ^introduced: v5.13..v5.14-rc1

   This tells regzbot the version range in which the issue started to happen;
   you can specify a range using commit-ids as well or state a single commit-id
   in case the reporter bisected the culprit.

   Note the caret (^) before the "introduced": it tells regzbot to treat the
   parent mail (the one you reply to) as the initial report for the regression
   you want to see tracked; that's important, as regzbot will later look out
   for patches with "Closes:" tags pointing to the report in the archives on
   lore.kernel.org.

 * When forwarding a regression reported to a bug tracker, include a paragraph
   with these regzbot commands::

       #regzbot introduced: 1f2e3d4c5b6a
       #regzbot from: Some N. Ice Human <some.human@example.com>
       #regzbot monitor: http://some.bugtracker.example.com/ticket?id=123456789

   Regzbot will then automatically associate patches with the report that
   contain "Closes:" tags pointing to your mail or the mentioned ticket.

```
#### 淇鍥炲綊鏃剁殑閲嶈浜嬮」


鍦ㄦ彁浜ゅ洖褰掍慨澶嶆椂锛屼綘涓嶉渶瑕佸仛浠讳綍鐗规畩鐨勪簨鎯咃紝鍙渶璁板緱閬靛惊
Documentation/process/submitting-patches.rst銆?
Documentation/process/5.Posting.rst <development_posting> 浠ュ強
Documentation/process/stable-kernel-rules.rst 涓凡缁忚缁嗚В閲婄殑閭ｄ簺瑕佹眰锛?

```
       Closes: https://lore.kernel.org/r/30th.anniversary.repost@klaava.Helsinki.FI/
       Closes: https://bugzilla.kernel.org/show_bug.cgi?id=1234567890

   If you are only fixing part of the issue, you may use "Link:" instead as
   described in the first document mentioned above. regzbot currently treats
   both of these equivalently and considers the linked reports as resolved.

 * Add a "Fixes:" tag to specify the commit causing the regression.

 * If the culprit was merged in an earlier development cycle, explicitly mark
   the fix for backporting using the ``Cc: stable@vger.kernel.org`` tag.

```
鎵€鏈夎繖浜涢兘鏄浣犵殑鏈€浣庤姹傦紝骞朵笖鍦ㄥ洖褰掗棶棰樹笂闈炲父閲嶈锛屽洜涓鸿繖浜涙爣绛惧浜庢暟鍛ㄣ€佹暟鏈堟垨鏁板勾鍚庡彲鑳借繕鍦ㄦ煡鐪嬭闂鐨勬墍鏈変汉锛堝寘鎷綘鑷繁锛夐兘鏋佹湁浠峰€笺€傝繖浜涙爣绛惧浜庡叾浠栧唴鏍稿紑鍙戣€呮垨 Linux 鍙戣鐗堟墍浣跨敤鐨勫伐鍏峰拰鑴氭湰涔熻嚦鍏抽噸瑕侊紱鍏朵腑涔嬩竴灏辨槸 regzbot锛屽畠楂樺害渚濊禆鈥淐loses:鈥濇爣绛炬潵灏嗗洖褰掓姤鍛婁笌瑙ｅ喅瀹冧滑鐨勫彉鏇村叧鑱旇捣鏉ャ€?

#### 淇鍥炲綊鐨勬湡鏈涗笌鏈€浣冲疄璺?


浣滀负涓€鍚?Linux 鍐呮牳寮€鍙戣€咃紝浣犲簲褰撳敖鍔涢伩鍏嶈繖鏍蜂竴绉嶅眬闈細鐢变綘鏈€杩戠殑鏀瑰姩寮曡捣鐨勫洖褰掞紝璁╃敤鎴峰彧鍓╀笅濡備笅鍑犵閫夋嫨锛?

 - 杩愯涓€涓甫鏈夊奖鍝嶄娇鐢ㄧ殑鍥炲綊鐨勫唴鏍搞€?

 - 鍒囨崲鍒版洿鏃ф垨鏇存柊鐨勫唴鏍哥郴鍒椼€?

 - 鍦ㄥ洖褰掔殑缃瓉绁搁琚‘璁ゅ悗锛岀户缁繍琛屼竴涓繃鏃跺洜鑰屾綔鍦ㄤ笉瀹夊叏鐨勫唴鏍歌秴杩囦笁鍛ㄣ€傜悊鎯虫儏鍐典笅搴斿綋灏戜簬涓ゅ懆銆傝€屽鏋滈棶棰樹弗閲嶆垨褰卞搷澶ч噺鐢ㄦ埛鈥斺€旀棤璁烘槸鏅亶鎯呭喌杩樻槸鍦ㄧ壒瀹氱幆澧冧腑鈥斺€斿垯搴斿綋鍙湁鍑犲ぉ銆?

濡備綍鍦ㄥ疄璺典腑瀹炵幇杩欎竴鐐癸紝鍙栧喅浜庡绉嶅洜绱犮€傝浣跨敤浠ヤ笅缁忛獙娉曞垯浣滀负鎸囧銆?

涓€鑸€岃█锛?

 - 灏嗗洖褰掔浉鍏冲伐浣滅殑浼樺厛绾х疆浜庢墍鏈夊叾浠?Linux 鍐呮牳宸ヤ綔涔嬩笂锛岄櫎闈炲悗鑰呮秹鍙婁弗閲嶉棶棰橈紙渚嬪鎬ヨ揩鐨勫畨鍏ㄦ紡娲炪€佹暟鎹涪澶便€佺‖浠跺彉鐮栫瓑锛夈€?

 - 鍔犲揩淇鏈€杩戣繘鍏ユ煇涓寮?mainline銆乻table 鎴?longterm 鐗堟湰鐨?mainline 鍥炲綊锛堟棤璁烘槸鐩存帴杩涘叆杩樻槸閫氳繃 backport锛夈€?

 - 涓嶈灏嗗綋鍓嶅紑鍙戝懆鏈熷唴鐨勫洖褰掕涓哄彲浠ユ嫋寤跺埌鍛ㄦ湡鏈殑闂锛屽洜涓鸿闂鍙兘鍔濋樆鎴栭樆姝㈢敤鎴峰拰 CI 绯荤粺鐜板湪鎴栨€讳綋涓婂 mainline 杩涜娴嬭瘯銆?

 - 浠ユ墍闇€鐨勮皑鎱庡紑灞曞伐浣滐紝閬垮厤閫犳垚棰濆鎴栨洿澶х殑鎹熷锛屽嵆渚胯繖鏍疯В鍐抽棶棰樺彲鑳芥瘮涓嬫枃鎵€杩拌€楁椂鏇撮暱銆?

鍦ㄥ洖褰掔殑缃瓉绁搁宸茬煡鍚庯紝鍏充簬鏃堕棿瀹夋帓锛?

 - 濡傛灉闂涓ラ噸鎴栧洶鎵板ぇ閲忕敤鎴封€斺€旀棤璁烘槸鏅亶鎯呭喌杩樻槸鍦ㄦ櫘閬嶇幆澧冧腑锛堝鐗瑰畾鐨勭‖浠剁幆澧冦€佸彂琛岀増鎴?stable/longterm 绯诲垪锛夆€斺€旂洰鏍囨槸鍦ㄤ袱涓夊ぉ鍐呭皢淇鍚堝叆 mainline銆?

 - 濡傛灉缃瓉绁搁杩涘叆浜嗘煇涓渶杩戠殑 mainline銆乻table 鎴?longterm 鐗堟湰锛堟棤璁烘槸鐩存帴杩涘叆杩樻槸閫氳繃 backport锛夛紝鐩爣鏄湪涓嬩竴涓懆鏃ヤ箣鍓嶅皢淇鍚堝叆 mainline锛涘鏋滅姜榄佺ジ棣栧湪涓€鍛ㄥ垵灏辫鍙戠幇涓旇В鍐宠捣鏉ュ緢绠€鍗曪紝灏介噺鍦ㄥ悓涓€鍛ㄥ唴灏嗕慨澶嶅悎鍏?mainline銆?

 - 瀵逛簬鍏朵粬鍥炲綊锛岀洰鏍囨槸鍦ㄦ帴涓嬫潵涓夊懆鍐呮渶闈犲悗鐨勯偅涓懆鏃ヤ箣鍓嶅皢淇鍚堝叆 mainline銆傚鏋滃洖褰掓槸浜轰滑鍙互杞绘槗蹇嶅彈涓€娈垫椂闂寸殑鈥斺€斾緥濡傝交寰殑鎬ц兘鍥炲綊鈥斺€旀櫄涓€涓や釜鍛ㄦ棩涔熸槸鍙互鎺ュ彈鐨勩€?

 - 寮虹儓涓嶅缓璁皢鍥炲綊淇鐨勫悎鍏ユ嫋寤跺埌涓嬩竴涓悎骞剁獥鍙ｏ紙merge window锛夛紝闄ら潪璇ヤ慨澶嶉闄╂瀬楂橈紝鎴栬€呯姜榄佺ジ棣栧湪涓€骞村浠ュ墠灏卞凡鍚堝叆 mainline銆?

鍏充簬娴佺▼锛?

 - 濮嬬粓鑰冭檻鍥為€€锛坮evert锛夌姜榄佺ジ棣栵紝鍥犱负杩欓€氬父鏄渶蹇€佹渶瀹夊叏鐨勪慨澶嶅洖褰掔殑鏂瑰紡銆備笉瑕佹媴蹇冪◢鍚庡悎鍏ヤ竴涓慨澶嶈繃鐨勫彉浣擄細閭ｅ簲褰撳緢鐩存帴锛屽洜涓哄ぇ閮ㄥ垎浠ｇ爜宸茬粡缁忚繃涓€杞瘎瀹′簡銆?

 - 鍔姏鍦ㄥ綋鍓嶅紑鍙戝懆鏈熺粨鏉熷墠锛岃В鍐?mainline 鍦ㄨ繃鍘诲崄浜屼釜鏈堝唴寮曞叆鐨勫洖褰掞細Linus 甯屾湜杩欑被鍥炲綊鍍忓綋鍓嶅懆鏈熺殑鍥炲綊涓€鏍疯澶勭悊锛岄櫎闈炰慨澶嶅甫鏉ヤ笉瀵诲父鐨勯闄┿€?

 - 濡傛灉鏌愪釜鍥炲綊鐪嬭捣鏉ュ緢妫樻墜锛岃€冭檻鍦ㄨ璁烘垨琛ヤ竵璇勫鏃舵妱閫侊紙CC锛塋inus銆傚湪妫樻墜鎴栫揣鎬ョ殑鎯呭喌涓嬪悓鏍峰姝も€斺€斿挨鍏舵槸褰撳瓙绯荤粺缁存姢鑰呭彲鑳借仈绯讳笉涓婃椂銆傚綋浣犵煡閬撹繖鏍风殑鍥炲綊宸茶繘鍏ユ煇涓?mainline銆乻table 鎴?longterm 鐗堟湰鏃讹紝涔熻鎶勯€?stable 鍥㈤槦銆?

 - 瀵逛簬绱ф€ュ洖褰掞紝鑰冭檻璇锋眰 Linus 鐩存帴浠庨偖浠跺垪琛ㄤ腑鎷惧彇锛坧ick up锛変慨澶嶏細瀵逛簬娌℃湁浜夎鐨勪慨澶嶏紝浠栧畬鍏ㄦ病闂杩欐牱鍋氥€備笉杩囩悊鎯虫儏鍐典笅锛屾绫昏姹傚簲褰撳緱鍒板瓙绯荤粺缁存姢鑰呯殑鍚屾剰锛屾垨鑰呯敱浠栦滑鐩存帴鎻愬嚭銆?

 - 濡傛灉浣犱笉纭畾鏌愪釜淇鏄惁鍊煎緱鍦ㄨ窛鏂?mainline 鍙戝竷浠呭墿鍑犲ぉ鏃跺啋闄╁簲鐢紝璇风粰 Linus 鍙戜竴灏侀偖浠讹紝鐓т緥鎶勯€佺浉鍏冲垪琛ㄥ拰浜哄憳锛涘湪閭欢涓€荤粨鎯呭喌锛屽悓鏃惰浠栬€冭檻鐩存帴浠庡垪琛ㄤ腑鎷惧彇璇ヤ慨澶嶃€傜劧鍚庣敱浠栬嚜宸卞仛鍐冲畾锛屽繀瑕佹椂鐢氳嚦鍙互鎺ㄨ繜鍙戝竷銆傛绫昏姹傚悓鏍风悊鎯虫儏鍐典笅搴斿綋寰楀埌瀛愮郴缁熺淮鎶よ€呯殑鍚屾剰锛屾垨鑰呯敱浠栦滑鐩存帴鎻愬嚭銆?

鍏充簬 stable 鍜?longterm 鍐呮牳锛?

 - 濡傛灉鏌愬洖褰掑湪浠讳綍鏃堕棿鐐归兘娌℃湁鍦?mainline 涓嚭鐜拌繃锛屾垨鑰呭凡缁忓湪 mainline 涓淇锛屼綘鍙互灏嗗叾鐣欑粰 stable 鍥㈤槦澶勭悊銆?

 - 濡傛灉鏌愪釜鍥炲綊鍦ㄨ繃鍘诲崄浜屼釜鏈堝唴杩涘叆浜嗘煇涓寮忕殑 mainline 鐗堟湰锛岃纭繚缁欎慨澶嶅姞涓娾€淐c: stable@vger.kernel.org鈥濇爣绛撅紝鍥犱负浠呴潬鈥淔ixes:鈥濇爣绛惧苟涓嶈兘淇濊瘉琚?backport銆傚鏋滀綘鐭ラ亾缃瓉绁搁宸茶 backport 鍒?stable 鎴?longterm 鍐呮牳锛屼篃璇峰姞涓婂悓鏍风殑鏍囩銆?

 - 褰撴敹鍒板叧浜庤繎鏈?stable 鎴?longterm 鍐呮牳绯诲垪鐨勫洖褰掓姤鍛婃椂锛岃鑷冲皯绠€瑕佽瘎浼颁竴涓嬭闂鏄惁涔熷彲鑳藉嚭鐜板湪褰撳墠 mainline 涓€斺€斿鏋滅湅璧锋潵寰堟湁鍙兘锛岃鎺ユ墜璇ユ姤鍛娿€傚鏋滄湁鐤戦棶锛岃鎶ュ憡鑰呮鏌?mainline銆?

 - 褰撲綘鎯宠繀閫熻В鍐充竴涓渶杩戜篃杩涘叆浜嗘煇涓寮?mainline銆乻table 鎴?longterm 鐗堟湰鐨勫洖褰掓椂锛岃鍦?mainline 涓揩閫熶慨澶嶅畠锛涘湪閫傚綋鏃跺洜姝よ Linus 鍔犲揩璇ヤ慨澶嶇殑鍚堝叆锛堣涓婃枃锛夈€傝繖鏄洜涓?stable 鍥㈤槦閫氬父涓嶄細鍥為€€鎴栦慨澶嶉偅浜涘湪 mainline 涓悓鏍峰紩璧烽棶棰樼殑鏀瑰姩銆?

 - 鍦ㄧ揣鎬ュ洖褰掍慨澶嶇殑鎯呭喌涓嬶紝浣犲彲鑳藉笇鏈涘湪淇鍚堝叆 mainline 鍚庣粰 stable 鍥㈤槦鍙戜釜鎻愮ず锛屼互纭繚鍙婃椂 backport锛涜繖鍦ㄥ悎骞剁獥鍙ｆ湡闂村強鍏跺垰缁撴潫鍚庡挨鍏跺彲鍙栵紝鍚﹀垯璇ヤ慨澶嶅彲鑳戒細鎺掑湪涓€闀夸覆琛ヤ竵闃熷垪鐨勬湯灏俱€?

鍏充簬琛ヤ竵娴佺▼锛?

 - 寮€鍙戣€咃紝鍦ㄥ姫鍔涜揪鍒颁笂杩版椂闂磋姹傛椂锛岃璁板緱涓轰慨澶嶈娴嬭瘯銆佽瘎瀹′互鍙婅 Linus 鍚堝叆锛堢悊鎯虫儏鍐典笅鑷冲皯鐭殏鍦拌繘鍏ヨ繃 linux-next锛夋墍鑺辩殑鏃堕棿鐣欏嚭浣欓噺銆傚洜姝わ紝濡傛灉鏌愪釜淇寰堢揣鎬ワ紝璇疯鍏剁揣杩€ф樉鑰屾槗瑙侊紝浠ョ‘淇濅粬浜哄Ε鍠勫鐞嗐€?

 - 璇勫鑰咃紝鎭宠浣犱滑鍙婃椂璇勫鍥炲綊淇锛屼互鍗忓姪寮€鍙戣€呰揪鍒颁笂杩版椂闂磋姹傘€?

 - 瀛愮郴缁熺淮鎶よ€咃紝鍚屾牱榧撳姳浣犱滑鍔犲揩鍥炲綊淇鐨勫鐞嗐€傚洜姝よ璇勪及瀵硅鐗瑰畾淇璺宠繃 linux-next 鏄惁鍙銆傚湪闇€瑕佹椂锛屼篃鑰冭檻姣斿钩甯告洿棰戠箒鍦板彂閫?git pull 璇锋眰銆傚苟灏介噺閬垮厤灏嗗洖褰掍慨澶嶆嫋鍒板懆鏈€斺€斿挨鍏舵槸褰撹淇琚爣璁颁负闇€瑕?backport 鏃躲€?


### 寮€鍙戣€呭簲褰撲簡瑙ｇ殑鏈夊叧鍥炲綊鐨勬洿澶氭柟闈?


#### 濡備綍澶勭悊宸茬煡瀛樺湪鍥炲綊椋庨櫓鐨勫彉鍖?


璇勪及鍥炲綊鐨勯闄╂湁澶氬ぇ锛屼緥濡傞€氳繃鍦?Linux 鍙戣鐗堝拰 Git 鎵樼骞冲彴涓繘琛屼唬鐮佹悳绱€備篃鑰冭檻璇峰叾浠栧彲鑳藉彈褰卞搷鐨勫紑鍙戣€呮垨椤圭洰鏉ヨ瘎浼扮敋鑷虫祴璇曟墍鎻愯鐨勬敼鍔紱濡傛灉鍑虹幇闂锛屼篃璁歌兘鎵惧埌鍚勬柟閮藉彲鎺ュ彈鐨勬柟妗堛€?

濡傛灉鏈€缁堢湅鏉ュ洖褰掔殑椋庨櫓鐩稿杈冨皬锛屽垯鍙互缁х画鏀瑰姩锛屼絾瑕佽鎵€鏈夌浉鍏虫柟鐭ユ檽璇ラ闄┿€傚洜姝わ紝璇风‘淇濅綘鐨勮ˉ涓佹弿杩拌杩欎竴鏂归潰鏄捐€屾槗瑙併€備竴鏃︽敼鍔ㄨ鍚堝叆锛岃灏嗛闄╁憡鐭?Linux 鍐呮牳鐨勫洖褰掕窡韪€呬互鍙婂洖褰掗偖浠跺垪琛紝杩欐牱鑻ユ姤鍛婇檰缁嚭鐜帮紝鎵€鏈変汉閮藉皢鎶婅鏀瑰姩鏀惧湪鍏虫敞鑼冨洿鍐呫€傛牴鎹闄╁ぇ灏忥紝浣犲彲鑳借繕鎯宠瀛愮郴缁熺淮鎶よ€呭湪鍏?mainline pull 璇锋眰涓彁鍙婅闂銆?

#### 鍏充簬鍥炲綊杩樻湁鍝簺闇€瑕佷簡瑙ｏ紵


璇锋煡鐪?Documentation/admin-guide/reporting-regressions.rst锛屽畠娑电洊浜嗚澶氫綘鍙兘鎯宠浜嗚В鐨勫叾浠栨柟闈細

 - 鈥渘o regressions鈥濓紙涓嶅紩鍏ュ洖褰掞級瑙勫垯鐨勭洰鐨?

 - 鍝簺闂鎵嶇湡姝ｇ畻寰椾笂鏄洖褰?

 - 璋佽礋璐ｅ鎵惧洖褰掔殑鏍规湰鍘熷洜

 - 濡備綍澶勭悊妫樻墜鎯呭喌锛屼緥濡傚綋鍥炲綊鏄敱鏌愪釜瀹夊叏淇寮曡捣锛屾垨鑰呬慨澶嶄竴涓洖褰掑彲鑳藉鑷村彟涓€涓洖褰掓椂

#### 鍏充簬鍥炲綊璇ュ悜璋佸緛姹傛剰瑙?


鍙戦€侀偖浠跺埌鍥炲綊閭欢鍒楄〃锛坮egressions@lists.linux.dev锛夛紝鍚屾椂鎶勯€?Linux 鍐呮牳鐨勫洖褰掕窡韪€咃紙regressions@leemhuis.info锛夛紱濡傛灉璇ラ棶棰樻洿閫傚悎绉佷笅澶勭悊锛屽彲浠ヤ笉鎶勯€佸垪琛ㄣ€?


### 鏇村鍏充簬鍥炲綊璺熻釜涓?regzbot


#### 涓轰粈涔?Linux 鍐呮牳鏈変竴涓洖褰掕窡韪€咃紝鍙堜负浠€涔堣浣跨敤 regzbot锛?


鍍忊€渘o regressions鈥濊繖鏍风殑瑙勫垯闇€瑕佹湁浜烘潵纭繚瀹冧滑琚伒瀹堬紝鍚﹀垯瀹冧滑浼氳鏃犳剰鎴栨湁鎰忓湴鐮村潖銆傚巻鍙茶〃鏄庯紝瀵?Linux 鍐呮牳鑰岃█鍚屾牱濡傛銆傝繖灏辨槸涓轰粈涔?Thorsten Leemhuis 鑷効浠?Linux 鍐呮牳鍥炲綊璺熻釜鑰呯殑韬唤鏉ョ暀鎰忓悇绉嶆儏鍐碉紝鍋跺皵浼氭湁鍏朵粬浜哄崗鍔┿€備粬浠兘涓嶄负姝よ幏寰楁姤閰紝鍥犳鍥炲綊璺熻釜鏄敖鍔涜€屼负锛坆est effort锛夌殑銆?

鏃╂湡鎵嬪姩璺熻釜鍥炲綊鐨勫皾璇曡〃鏄庤繖鏄竴椤逛护浜虹瓔鐤插姏灏戒笖娌抚鐨勫伐浣滐紝鍥犳瀹冧滑涓€娈垫椂闂村悗灏辫鏀惧純浜嗐€備负浜嗛槻姝㈣繖绉嶆儏鍐靛啀娆″彂鐢燂紝Thorsten 寮€鍙戜簡 regzbot 鏉ョ畝鍖栬繖椤瑰伐浣滐紝闀胯繙鐩爣鏄负鎵€鏈夌浉鍏充汉鍛樺敖鍙兘鑷姩鍖栧洖褰掕窡韪€?

#### regzbot 鐨勫洖褰掕窡韪槸濡備綍宸ヤ綔鐨勶紵


璇ユ満鍣ㄤ汉浼氱洃瑙嗗宸茶窡韪洖褰掓姤鍛婄殑鍥炲銆傛澶栵紝瀹冭繕浼氬鎵惧紩鐢ㄤ簡姝ょ被鎶ュ憡锛堝甫鏈夆€淐loses:鈥濇爣绛撅級鐨勫凡鍙戝竷鎴栧凡鎻愪氦琛ヤ竵锛涘姝ょ被琛ヤ竵鍙戝竷鐨勫洖澶嶄篃浼氳璺熻釜銆傜粨鍚堣捣鏉ョ殑鏁版嵁鎻愪緵浜嗗叧浜庝慨澶嶈繃绋嬪綋鍓嶇姸鎬佺殑鑹ソ娲炲療銆?

regzbot 璇曞浘浠ュ敖鍙兘灏戠殑寮€閿€涓烘姤鍛婅€呭拰寮€鍙戣€呭畬鎴愬伐浣溿€備簨瀹炰笂锛屽彧鏈夋姤鍛婅€呰棰濆澧炲姞浜嗕竴椤硅亴璐ｏ細浠栦滑闇€瑕佷娇鐢ㄤ笂鏂囨杩扮殑 ``#regzbot
introduced`` 鍛戒护鏉ュ憡鐭?regzbot 鍥炲綊鎶ュ憡锛涘鏋滀粬浠笉杩欐牱鍋氾紝鍏朵粬浜哄彲浠ヤ娇鐢?`#regzbot ^introduced` 鏉ュ鐞嗐€?

瀵瑰紑鍙戣€呮潵璇达紝閫氬父涓嶆秹鍙婇澶栧伐浣滐紝浠栦滑鍙渶瑕佺‘淇濆幓鍋氬湪 regzbot 鍑虹幇涔嬪墠灏辨棭璇ュ仛鐨勪簨锛氬湪琛ヤ竵鎻忚堪涓坊鍔犳寚鍚戞墍鏈夊叧浜庢墍淇闂鐨勬姤鍛婄殑閾炬帴銆?

#### 鎴戝繀椤讳娇鐢?regzbot 鍚楋紵


濡傛灉浣犱娇鐢ㄥ畠锛岃繖绗﹀悎姣忎釜浜虹殑鍒╃泭锛屽洜涓哄唴鏍哥淮鎶よ€咃紙濡?Linus
Torvalds锛夊湪宸ヤ綔涓儴鍒嗕緷璧?regzbot 鐨勮窡韪€斺€斾緥濡傚湪鍐冲畾鏄惁鍙戝竷鏂扮増鏈垨寤堕暱寮€鍙戦樁娈垫椂銆備负姝わ紝浠栦滑闇€瑕佺煡鏅撴墍鏈夋湭淇鐨勫洖褰掞紱浼楁墍鍛ㄧ煡锛孡inus 浼氭煡鐪?regzbot 鍙戦€佺殑姣忓懆鎶ュ憡銆?

#### 鎴戝繀椤绘妸鎴戦亣鍒扮殑姣忎竴涓洖褰掗兘鍛婅瘔 regzbot 鍚楋紵


鐞嗘兂鎯呭喌涓嬫槸鐨勶細鎴戜滑閮芥槸浜猴紝褰撴洿閲嶈鐨勪簨鎯呮剰澶栧嚭鐜版椂鈥斺€斾緥濡?Linux 鍐呮牳涓殑涓€涓洿澶ч棶棰橈紝鎴栬€呯幇瀹炵敓娲讳腑璁╂垜浠殏鏃剁寮€閿洏鐨勪簨鎯呪€斺€旀垜浠緢瀹规槗蹇樿闂銆傚洜姝わ紝鏈€濂芥妸姣忎竴涓洖褰掗兘鍛婅瘔 regzbot锛岄櫎闈炰綘绔嬪嵆鍐欎簡涓€涓慨澶嶅苟灏嗗叾鎻愪氦鍒颁竴涓畾鏈熷悎鍏ュ彈褰卞搷鍐呮牳绯诲垪鐨勬爲涓€?

#### 濡備綍鏌ョ湅 regzbot 褰撳墠璺熻釜鍝簺鍥炲綊锛?


璇锋煡鐪?`regzbot's web-interface <https://linux-regtracking.leemhuis.info/regzbot/>`_
鑾峰彇鏈€鏂颁俊鎭紱鎴栬€咃紝`search for the latest regression report
<https://lore.kernel.org/lkml/?q=%22Linux+regressions+report%22+f%3Aregzbot>`_锛?
regzbot 閫氬父浼氬湪鍛ㄦ棩鍌嶆櫄锛圲TC锛夊彂閫佷竴娆★紝涔熷氨鏄?Linus 閫氬父鍙戝竷鏂帮紙棰勶級鍙戝竷鐗堟湰鍓嶅嚑涓皬鏃躲€?

#### regzbot 鍦ㄧ洃鎺у摢浜涘湴鏂癸紵


regzbot 姝ｅ湪鐩戣鏈€閲嶈鐨?Linux 閭欢鍒楄〃锛屼互鍙?linux-next銆乵ainline 鍜?stable/longterm 鐨?git 浠撳簱銆?

#### 鍝簺绫诲瀷鐨勯棶棰樺簲褰撶敱 regzbot 璺熻釜锛?


璇ユ満鍣ㄤ汉鏃ㄥ湪璺熻釜鍥炲綊锛屽洜姝よ涓嶈涓哄父瑙勯棶棰樺紩鍏?regzbot銆備絾濡傛灉浣犱娇鐢?regzbot 鏉ヨ窡韪弗閲嶉棶棰橈紙濡傚叧浜庢寕璧枫€佹暟鎹崯鍧忔垨鍐呴儴閿欒锛圥anic銆丱ops銆丅UG()銆亀arning 绛夛級鐨勬姤鍛婏級锛孡inux 鍐呮牳鐨勫洖褰掕窡韪€呮槸鍙互鎺ュ彈鐨勩€?

#### 鎴戝彲浠ユ妸 CI 绯荤粺鍙戠幇鐨勫洖褰掑姞鍏?regzbot 鐨勮窡韪悧锛?


濡傛灉鐗瑰畾鍥炲綊寰堝彲鑳藉瀹為檯浣跨敤鍦烘櫙浜х敓褰卞搷锛屼粠鑰屽彲鑳借鐢ㄦ埛娉ㄦ剰鍒帮紝璇烽殢鎰忚繖鏍峰仛锛涘洜姝わ紝璇蜂笉瑕佷负涓嶅お鍙兘鍦ㄧ湡瀹炰笘鐣屼娇鐢ㄤ腑鏄剧幇鐨勭悊璁烘€у洖褰掑紩鍏?regzbot銆?

#### 濡備綍涓?regzbot 浜や簰锛?


閫氳繃鍦ㄥ甫鏈夊洖褰掓姤鍛婄殑閭欢鐨勭洿鎺ユ垨闂存帴鍥炲涓娇鐢ㄢ€渞egzbot 鍛戒护鈥濄€傝繖浜涘懡浠ら渶瑕佷綅浜庡悇鑷嫭绔嬬殑娈佃惤涓紙IOW锛氶渶瑕佺敤绌鸿涓庨偖浠跺叾浣欓儴鍒嗗垎闅旓級銆?

鍏朵腑涓€涓懡浠ゆ槸 `#regzbot introduced: <version or commit>`锛屽畠浼氳 regzbot 灏嗕綘鐨勯偖浠惰涓哄姞鍏ヨ窡韪殑鍥炲綊鎶ュ憡锛屽涓婃枃鎵€杩帮紱`#regzbot ^introduced: <version or commit>` 鏄彟涓€涓绫诲懡浠わ紝瀹冧細璁?regzbot 灏嗙埗閭欢瑙嗕负瀹冨紑濮嬭窡韪殑鍥炲綊鐨勬姤鍛娿€?

涓€鏃︿娇鐢ㄤ簡杩欎袱涓懡浠や箣涓€锛屽氨鍙互鍦ㄥ璇ユ姤鍛婄殑鐩存帴鎴栭棿鎺ュ洖澶嶄腑浣跨敤鍏朵粬 regzbot 鍛戒护銆備綘鍙互灏嗗畠浠啓鍦ㄦ煇涓?`introduced` 鍛戒护涓嬫柟锛屾垨鑰呭湪浣跨敤浜嗗叾涓竴涓懡浠ょ殑閭欢鐨勫洖澶嶄腑锛屾垨鑰呮湰韬槸瀵硅閭欢鐨勫洖澶嶄腑锛?

```
       #regzbot title: foo

 * Monitor a discussion or bugzilla.kernel.org ticket where additions aspects of
   the issue or a fix are discussed -- for example the posting of a patch fixing
   the regression::

       #regzbot monitor: https://lore.kernel.org/all/30th.anniversary.repost@klaava.Helsinki.FI/

   Monitoring only works for lore.kernel.org and bugzilla.kernel.org; regzbot
   will consider all messages in that thread or ticket as related to the fixing
   process.

 * Point to a place with further details of interest, like a mailing list post
   or a ticket in a bug tracker that are slightly related, but about a different
   topic::

       #regzbot link: https://bugzilla.kernel.org/show_bug.cgi?id=123456789

 * Mark a regression as fixed by a commit that is heading upstream or already
   landed::

       #regzbot fix: 1f2e3d4c5d

 * Mark a regression as a duplicate of another one already tracked by regzbot::

       #regzbot dup-of: https://lore.kernel.org/all/30th.anniversary.repost@klaava.Helsinki.FI/

 * Mark a regression as invalid::

       #regzbot invalid: wasn't a regression, problem has always existed

```
#### 鍏充簬 regzbot 鍙婂叾鍛戒护杩樻湁浠€涔堝彲璇寸殑鍚楋紵


鍏充簬 Linux 鍐呮牳鍥炲綊璺熻釜鏈哄櫒浜虹殑鏇磋缁嗐€佹洿鍙婃椂鐨勪俊鎭彲浠ュ湪鍏?
`project page <https://gitlab.com/knurd42/regzbot>`_ 涓婃壘鍒帮紝鍏朵腑鍖呮嫭
`getting started guide <https://gitlab.com/knurd42/regzbot/-/blob/main/docs/getting_started.md>`_
鍜?`reference documentation <https://gitlab.com/knurd42/regzbot/-/blob/main/docs/reference.md>`_锛?
涓よ€呮兜鐩栫殑缁嗚妭閮藉浜庝笂闈㈣繖涓€鑺傘€?

### Linus 鍏充簬鍥炲綊鐨勮褰?


浠ヤ笅 Linus Torvalds 鐨勮█璁烘彁渚涗簡涓€浜涘叧浜?Linux
鈥渘o regressions鈥濓紙涓嶅紩鍏ュ洖褰掞級瑙勫垯浠ュ強浠栨湡鏈涘洖褰掑浣曡澶勭悊鐨勬礊瑙侊細

#### 鍏充簬鍥炲綊搴斿綋澶氬揩琚慨澶?


```
    But a user complaining should basically result in an immediate fix -
    possibly a "revert and rethink".

  With a later clarification on `2026-01-28 <https://lore.kernel.org/all/CAHk-%3Dwi86AosXs66-yi54%2BmpQjPu0upxB8ZAfG%2BLsMyJmcuMSA@mail.gmail.com/>`_::

    It's also worth noting that "immediate" obviously doesn't mean "right
    this *second* when the problem has been reported".

    But if it's a regression with a known commit that caused it, I think
    the rule of thumb should generally be "within a week", preferably
    before the next rc.

```
```
    Known-broken commits either
     (a) get a timely fix that doesn't have other questions
    or
     (b) get reverted

```
```
    [...] review shouldn't hold up reported regressions of existing code. That's
    just basic _testing_ - either the fix should be applied, or - if the fix is
    too invasive or too ugly - the problematic source of the regression should
    be reverted.

    Review should be about new code, it shouldn't be holding up "there's a
    bug report, here's the obvious fix".

```
```
    If something doesn't even build, it should damn well be fixed ASAP.

```
#### 鍏充簬鐢ㄥ洖閫€鏉ヤ慨澶嶅洖褰掑浣曟湁鍔╀簬闃叉缁存姢鑰呭€︽€?


```
    > So how can I/we make "immediate fixes" happen more often without
    > contributing to maintainer burnout?

    [...] the "revert and rethink" model [...] often a good idea in general [...]

    Exactly so that maintainers don't get stressed out over having a pending
    problem report that people keep pestering them about.

    I think people are sometimes a bit too bought into whatever changes
    they made, and reverting is seen as "too drastic", but I think it's
    often the quick and easy solution for when there isn't some obvious
    response to a regression report.

```
#### 鍏充簬鍦ㄦ渶鍚庝竴涓?-rc 鎴栨柊鐗堟湰涓磋繎鏃跺悎鍏ヤ慨澶?


```
    So I think I'd rather see them hit rc8 (later today) and have a week
    of testing in my tree and be reverted if they cause problems, than
    have them go in after rc8 and then cause problems in the 6.19 release
    instead.

```
```
    But something like this, where the regression was in the previous release
    and it's just a clear fix with no semantic subtlety, I consider to be just a
    regular regression that should be expedited - partly to make it into stable,
    and partly to avoid having to put the fix into _another_ stable kernel.

```
#### 鍏充簬鍙彁浜や竴涓慨澶嶇殑鍚堝苟璇锋眰


```
    If the issue is just that there's nothing else happening, I think people
    should just point me to the patch and say "can you apply this single fix?"

```
```
    I'm always open to direct fixes when there is no controversy about the fix.
    No problem. I still happily deal with individual patches.

```
#### 鍏充簬浣跨敤 Link:/Closes: 鏍囩鎸囧悜缂洪櫡鎶ュ憡鐨勯噸瑕佹€?


```
    [...] revert like this, it really would be good to link to the problems, so
    that when people try to re-enable it, they have the history for why it
    didn't work the first time.

```
```
    So I have to once more complain [...]

    [...] There's no link to the actual problem the patch fixes.

```
```
    See, *that* link [to the report] would have been useful in the commit.

```
#### 鍏充簬鈥渘o regressions鈥濊鍒欎负浣曞瓨鍦?


```
    But the basic rule is: be so good about backwards compatibility that
    users never have to worry about upgrading. They should absolutely feel
    confident that any kernel-reported problem will either be solved, or
    have an easy solution that is appropriate for *them* (ie a
    non-technical user shouldn't be expected to be able to do a lot).

    Because the last thing we want is people holding back from trying new
    kernels.

```
```
    I introduced that "no regressions" rule something like two decades
    ago, because people need to be able to update their kernel without
    fear of something they relied on suddenly stopping to work.

```
```
    The whole point of "we do not regress" is so that people can upgrade
    the kernel and never have to worry about it.

    [...]

    Because the only thing that matters IS THE USER.

```
```
    If the kernel used to work for you, the rule is that it continues to work
    for you.

    [...]

    People should basically always feel like they can update their kernel
    and simply not have to worry about it.

    I refuse to introduce "you can only update the kernel if you also
    update that other program" kind of limitations. If the kernel used to
    work for you, the rule is that it continues to work for you.

```
#### 鍏充簬鈥渘o regressions鈥濊鍒欑殑渚嬪


```
    There are _very_ few exceptions to that rule, the main one being "the
    problem was a fundamental huge and gaping security issue and we *had* to
    make that change, and we couldn't even make your limited use-case just
    continue to work".

    The other exception is "the problem was reported years after it was
    introduced, and now most people rely on the new behavior".

    [...]

    Now, if it's one or two users and you can just get them to recompile,
    that's one thing. Niche hardware and odd use-cases can sometimes be
    solved that way, and regressions can sometimes be fixed by handholding
    every single reporter if the reporter is willing and able to change
    his or her workflow.

```
```
    And yes, I do consider "regression in an earlier release" to be a
    regression that needs fixing.

    There's obviously a time limit: if that "regression in an earlier
    release" was a year or more ago, and just took forever for people to
    notice, and it had semantic changes that now mean that fixing the
    regression could cause a _new_ regression, then that can cause me to
    go "Oh, now the new semantics are what we have to live with".

```
```
    There have been exceptions, but they are few and far between, and they
    generally have some major and fundamental reasons for having happened,
    that were basically entirely unavoidable, and people _tried_hard_ to
    avoid them. Maybe we can't practically support the hardware any more
    after it is decades old and nobody uses it with modern kernels any
    more. Maybe there's a serious security issue with how we did things,
    and people actually depended on that fundamentally broken model. Maybe
    there was some fundamental other breakage that just _had_ to have a
    flag day for very core and fundamental reasons.

```
#### 鍏充簬鏇存柊鐢ㄦ埛绌洪棿涓殑鏌愪簺涓滆タ鍙互瑙ｅ喅鍥炲綊鐨勬儏鍐?


```
    And dammit, we upgrade the kernel ALL THE TIME without upgrading any
    other programs at all. It is absolutely required, because flag-days
    and dependencies are horribly bad.

    And it is also required simply because I as a kernel developer do not
    upgrade random other tools that I don't even care about as I develop the
    kernel, and I want any of my users to feel safe doing the same time.

```
```
    But if something actually breaks, then the change must get fixed or
    reverted. And it gets fixed in the *kernel*. Not by saying "well, fix your
    user space then". It was a kernel change that exposed the problem, it needs
    to be the kernel that corrects for it, because we have a "upgrade in place"
    model. We don't have a "upgrade with new user space".

    And I seriously will refuse to take code from people who do not understand
    and honor this very simple rule.

    This rule is also not going to change.

    And yes, I realize that the kernel is "special" in this respect. I'm proud
    of it.

```
```
    If you break existing user space setups THAT IS A REGRESSION.

    It's not ok to say "but we'll fix the user space setup".

    Really. NOT OK.

```
#### 鍏充簬浠€涔堢畻浣滅敤鎴风┖闂存帴鍙ｃ€丄BI銆丄PI銆佸凡鏂囨。鍖栫殑鎺ュ彛绛?


```
    So I absolutely detest the whole notion of "ABI changes". It's a
    meaningless concept, and I hate it with a passion, [...]

    The Linux rule for regressions is basically based on the philosophical
    question of "If a tree falls in the forest, and nobody is around to
    hear it, does it make a sound?".

    So the only thing that matters is if something breaks user-*conscious*
    behavior.

    And when that happens, the distinction between "bug fix" and "new
    feature" and "ABI change" matters not one whit, and the change needs
    to be done differently.

    [...]

    I just wanted to point out that the argument about whether it's an ABI
    change or not is irrelevant. If it turns out that some program - not a test
    script, but something with relevance to conscious user expectations ~
    depended on the old broken behavior, then it needs to be done some other
    way.

```
```
    > [...] this should not fall under the don't break user space rule [...]

    Note that the rule is about breaking *users*, not breaking user space per
    se. [...]

    If some user setup breaks, things need fixing.

    [...] but I want to make it very clear that there are no excuses about "user
    space applications".

```
```
    [...] a regression is a bit like Schr枚dinger's cat - if nobody is around
    to notice it and it doesn't actually affect any real workload, then you
    can treat the regression as if it doesn't exist.

```
```
    The rules about regressions have never been about any kind of documented
    behavior, or where the code lives.

    The rules about regressions are always about "breaks user workflow".

    Users are literally the _only_ thing that matters.

```
```
    One _particularly_ last-minute revert is the top-most commit (ignoring
    the version change itself) done just before the release, and while
    it's very annoying, it's perhaps also instructive.

    What's instructive about it is that I reverted a commit that wasn't
    actually buggy. In fact, it was doing exactly what it set out to do,
    and did it very well. In fact it did it _so_ well that the much
    improved IO patterns it caused then ended up revealing a user-visible
    regression due to a real bug in a completely unrelated area.

    The actual details of that regression are not the reason I point that
    revert out as instructive, though. It's more that it's an instructive
    example of what counts as a regression, and what the whole "no
    regressions" kernel rule means.

    [...] The reverted commit didn't change any API's, and it didn't introduce
    any new bugs. But it ended up exposing another problem, and as such caused
    a kernel upgrade to fail for a user. So it got reverted.

    The point here being that we revert based on user-reported _behavior_, not
    based on some "it changes the ABI" or "it caused a bug" concept. The problem
    was really pre-existing, and it just didn't happen to trigger before. [...]

    Take-away from the whole thing: it's not about whether you change the
    kernel-userspace ABI, or fix a bug, or about whether the old code
    "should never have worked in the first place". It's about whether
    something breaks existing users' workflow.

```
```
    And our regression rule has never been "behavior doesn't change".
    That would mean that we could never make any changes at all.

```
```
    No amount of "you shouldn't have used this" or "that behavior was
    undefined, it's your own fault your app broke" or "that used to work
    simply because of a kernel bug" is at all relevant.

```
```
    But no, "that was documented to be broken" (whether it's because the code
    was in staging or because the man-page said something else) is irrelevant.
    If staging code is so useful that people end up using it, that means that
    it's basically regular kernel code with a flag saying "please clean this
    up".

    [...]

    The other side of the coin is that people who talk about "API stability" are
    entirely wrong. API's don't matter either. You can make any changes to an
    API you like - as long as nobody notices.

    Again, the regression rule is not about documentation, not about API's, and
    not about the phase of the moon.

```
```
    > Now this got me wondering if Debian _unstable_ actually qualifies as a
    > standard distro userspace.

    Oh, if the kernel breaks some standard user space, that counts. Tons
    of people run Debian unstable

```
```
    It's clearly NOT an internal tracepoint. By definition. It's being
    used by powertop.

```
#### 鍏充簬鐢ㄦ埛鎴栨祴璇曞浠?CI 娉ㄦ剰鍒扮殑鍥炲綊


```
    Users complaining is the only real line in the end.

    [...] a test-suite complaining is then often a *very* good indication that
    maybe users will hit some problem, and test suite issues should be taken
    very seriously [...]

    But a test-suite error isn't necessarily where you have to draw the
    line - it's a big red flag [...]

```
```
    The "no regressions" rule is not about made-up "if I do this, behavior
    changes".

    The "no regressions" rule is about *users*.

    If you have an actual user that has been doing insane things, and we
    change something, and now the insane thing no longer works, at that
    point it's a regression, and we'll sigh, and go "Users are insane" and
    have to fix it.

    But if you have some random test that now behaves differently, it's
    not a regression. It's a *warning* sign, sure: tests are useful.

```
#### 鍏充簬鎵胯鍥炲綊宸茬粡鍙戠敓


```
    But starting to argue about users reporting breaking changes is
    basically the final line for me. I have a couple of people that I have
    in my spam block-list and refuse to have anything to do with, and they
    have generally been about exactly that.

    Note how it's not about making mistakes and _causing_ the regression.
    That's normal. That's development. But then arguing about it is a
    no-no.

```
```
    We don't introduce regressions and then blame others.

    There's a very clear rule in kernel development: things that break
    other things ARE NOT FIXES.

    EVER.

    They get reverted, or the thing they broke gets fixed.

```
```
    THERE ARE NO VALID ARGUMENTS FOR REGRESSIONS.

    Honestly, security people need to understand that "not working" is not
    a success case of security. It's a failure case.

    Yes, "not working" may be secure. But security in that case is *pointless*.

```
```
    [...] when regressions *do* occur, we admit to them and fix them, instead of
    blaming user space.

    The fact that you have apparently been denying the regression now for
    three weeks means that I will revert, and I will stop pulling apparmor
    requests until the people involved understand how kernel development
    is done.

```
#### 鍏充簬鏉ュ洖鎷夐敮


```
    The "no regressions" rule is that we do not introduce NEW bugs.

    It *literally* came about because we had an endless dance of "fix two
    bugs, introduce one new one", and that then resulted in a system that
    you cannot TRUST.

```
```
    And the thing that makes regressions special is that back when I
    wasn't so strict about these things, we'd end up in endless "seesaw
    situations" where somebody would fix something, it would break
    something else, then that something else would break, and it would
    never actually converge on anything reliable at all.

```
```
    The strict policy of no regressions actually originally started mainly wrt
    suspend/resume issues, where the "fix one machine, break another" kind of
    back-and-forth caused endless problems, and meant that we didn't actually
    necessarily make any forward progress, just moving a problem around.

```
#### 鍏充簬鏈夊紩鍙戝洖褰掗闄╃殑鍙樺寲


```
    So what I think you should do is to fix the bug right, with a clean
    patch, and no crazy hacks. That is something we can then apply and
    test. All the while knowing full well that "uhhuh, this is a visible
    change, we may have to revert it".

    If then some *real* load ends up showing a regression, we may just be
    screwed. Our current behavior may be buggy, but we have the rule that
    once user space depends on kernel bugs, they become features pretty
    much by definition, however much we might dislike it.

```
#### 鍏充簬鍐呮牳鍐呯殑鍙橀€氭柟妗堜互閬垮厤鍥炲綊


```
    Behavioral changes happen, and maybe we don't even support some
    feature any more. There's a number of fields in /proc/<pid>/stat that
    are printed out as zeroes, simply because they don't even *exist* in
    the kernel any more, or because showing them was a mistake (typically
    an information leak). But the numbers got replaced by zeroes, so that
    the code that used to parse the fields still works. The user might not
    see everything they used to see, and so behavior is clearly different,
    but things still _work_, even if they might no longer show sensitive
    (or no longer relevant) information.

```
#### 鍏充簬鐢辩己闄蜂慨澶嶅紩璧风殑鍥炲綊


```
    > Kernel had a bug which has been fixed

    That is *ENTIRELY* immaterial.

    Guys, whether something was buggy or not DOES NOT MATTER.

    [...]

    It's basically saying "I took something that worked, and I broke it,
    but now it's better". Do you not see how f*cking insane that statement
    is?

```
#### 鍏充簬鍐呴儴 API 鍙樻洿


```
    We do API breakage _inside_ the kernel all the time. We will fix
    internal problems by saying "you now need to do XYZ", but then it's
    about internal kernel API's, and the people who do that then also
    obviously have to fix up all the in-kernel users of that API. Nobody
    can say "I now broke the API you used, and now _you_ need to fix it
    up". Whoever broke something gets to fix it too.

```
#### 鍏充簬寰堜箙浠ュ悗鎵嶈鍙戠幇鐨勫洖褰?


```
    I'm definitely not reverting a patch from almost a decade ago as a
    regression.

    If it took that long to find, it can't be that critical of a regression.

    So yes, let's treat it as a regular bug.

```
#### 鍏充簬鍦?linux-next 涓祴璇曞洖褰掍慨澶?


```
   So running fixes though linux-next is just a waste of time.

```
#### 鍏充簬涓庡洖褰掔浉鍏崇殑鍏朵粬鍑犱釜鏂归潰


- From `2025-07-29(2) <https://lore.kernel.org/all/CAHk-=wjj9DvOZtmTkoLtyfHmy5mNKy6q_96d9=4FUEDXre=cww@mail.gmail.com/>`_
```
    I no longer have sound.

    I also suspect that it's purely because "make oldconfig" doesn't work,
    and probably turned off my old Intel HDA settings. Or something.

    Renaming config parameters is *bad*. I've harped on the Kconfig phase
    of the kernel build probably being our nastiest point, and a real pain
    point to people getting involved with development simply because
    building your own kernel can be so daunting with hundreds of fairly
    esoteric questions.

```
..
   end-of-content
..
   This text is available under GPL-2.0+ or CC-BY-4.0, as stated at the top
   of the file. If you want to distribute this text under CC-BY-4.0 only,
   please use "The Linux kernel developers" for author attribution and link
   this as source:
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/process/handling-regressions.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.
