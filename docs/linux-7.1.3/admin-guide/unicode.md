## Unicode 鏀寔


		 鏈€鍚庢洿鏂帮細2005-01-17锛岀増鏈?1.4

娉ㄦ剰锛氭湰鏂囨。鐨勫師濮嬬増鏈敱 lanana.org 浣滀负 Linux 宸插垎閰嶅悕绉颁笌缂栧彿绠＄悊灞€
锛圠ANANA锛夐」鐩殑涓€閮ㄥ垎缁存姢锛岀幇宸蹭笉澶嶅瓨鍦ㄣ€傚洜姝わ紝涓荤嚎 Linux 鍐呮牳涓殑
杩欎釜鐗堟湰鐜板湪鎴愪负鍙楃淮鎶ょ殑涓绘枃妗ｃ€?
### 绠€浠?

Linux 鍐呮牳浠ｇ爜宸茶閲嶅啓锛屼娇鐢?Unicode 灏嗗瓧绗︽槧灏勫埌瀛椾綋銆傞€氳繃涓嬭浇涓€浠?Unicode 鍒板瓧浣撶殑琛ㄦ牸锛屽叓浣嶅瓧绗﹂泦鍜?UTF-8 妯″紡閮戒細琚敼涓轰娇鐢ㄦ墍鎸囩ず鐨勫瓧浣撱€?
杩欏井濡欏湴鏀瑰彉浜嗗叓浣嶅瓧绗﹁〃鐨勮涔夈€傜幇鍦ㄥ洓涓瓧绗﹁〃濡備笅锛?
=============== =============================== ================
鏄犲皠绗﹀彿	鏄犲皠鍚嶇О		杞箟鐮?(G0)
=============== =============================== ================
LAT1_MAP	Latin-1 (ISO 8859-1)		ESC ( B
GRAF_MAP	DEC VT100 浼浘褰?	ESC ( 0
IBMPC_MAP	IBM 浠ｇ爜椤?437		ESC ( U
USER_MAP	鐢ㄦ埛瀹氫箟			ESC ( K
=============== =============================== ================

灏ゅ叾鏄紝ESC ( U 涓嶅啀鏄?鐩存帴閫佸瓧浣?锛屽洜涓哄瓧浣撳彲鑳戒笌 IBM 瀛楃闆嗗畬鍏ㄤ笉鍚屻€?杩欐牱渚嬪鍗充究鍔犺浇浜?Latin-1 瀛椾綋涔熻兘浣跨敤鍧楀浘褰€?
娉ㄦ剰锛屽敖绠¤繖浜涗唬鐮佷笌 ISO 2022 绫讳技锛屼絾鏃犺鏄唬鐮佹湰韬繕鏄叾浣跨敤鏂瑰紡閮戒笌
ISO 2022 涓嶅尮閰嶏紱Linux 鏈変袱涓?8 浣嶄唬鐮侊紙G0 鍜?G1锛夛紝鑰?ISO 2022 鏈夊洓涓?7 浣嶄唬鐮侊紙G0-G3锛夈€?
鏍规嵁 Unicode 鏍囧噯/ISO 10646锛岃寖鍥?U+F000 鍒?U+F8FF 琚繚鐣欑敤浜庢搷浣滅郴缁?鑼冨洿鐨勫垎閰嶏紙Unicode 鏍囧噯绉颁箣涓?浼佷笟鍖?锛岀敱浜庤繖瀵?Linux 涓嶅噯纭紝鎴戜滑绉颁箣涓?"Linux 鍖?锛夈€傞€夋嫨 U+F000 浣滀负璧风偣锛屾槸鍥犱负瀹冨彲浠ヨ鐩存帴鏄犲皠鍖轰粠涓€涓ぇ鐨?2 鐨勫箓寮€濮嬶紙浠ラ槻鏃ュ悗纭疄闇€瑕?1024 鎴?2048 瀛楃鐨勫瓧浣擄級銆傝繖灏卞皢 U+E000 鍒?U+EFFF 鐣欎綔鏈€缁堢敤鎴峰尯銆?
[v1.2]锛氫粠 U+F000 鍒?U+F7FF 鐨?Unicode 鑼冨洿宸茶纭紪鐮佷负鐩存帴鏄犲皠鍒版墍鍔犺浇鐨?瀛椾綋锛岀粫杩囩炕璇戣〃銆傜敤鎴疯嚜瀹氫箟鏄犲皠鐜板湪榛樿鎸囧悜 U+F000 鍒?U+F0FF锛屾ā鎷熶簡涔嬪墠
鐨勮涓恒€傚疄闄呬笂璇ヨ寖鍥村彲鑳芥洿鐭紱渚嬪 vgacon 鍙兘澶勭悊 256 瀛楃
锛圲+F000..U+F0FF锛夋垨 512 瀛楃锛圲+F000..U+F1FF锛夌殑瀛椾綋銆?

### 鍦?Linux 鍖轰腑瀹為檯鍒嗛厤鐨勫瓧绗?

姝ゅ锛岃繕瀹氫箟浜嗕互涓?Unicode 1.1.4 涓笉瀛樺湪鐨勫瓧绗︼紱杩欎簺鐢?DEC VT 鍥惧舰鏄犲皠
浣跨敤銆俒v1.2] 姝ょ敤娉曞凡杩囨椂锛屼笉搴斿啀浣跨敤锛涜鍙傞槄涓嬫枃銆?
====== ======================================
U+F800 DEC VT 鍥惧舰 姘村钩绾?鎵弿 1
U+F801 DEC VT 鍥惧舰 姘村钩绾?鎵弿 3
U+F803 DEC VT 鍥惧舰 姘村钩绾?鎵弿 7
U+F804 DEC VT 鍥惧舰 姘村钩绾?鎵弿 9
====== ======================================

DEC VT220 浣跨敤 6x10 瀛楃鐭╅樀锛岃繖浜涘瓧绗﹀湪 DEC VT 鍥惧舰瀛楃闆嗕腑褰㈡垚骞虫粦鐨?閫掕繘銆傛垜鐣ュ幓浜嗘壂鎻?5 鐨勭嚎锛屽洜涓哄畠涔熻鐢ㄤ綔鍧楀浘瀛楃锛屽洜姝よ缂栫爜涓?U+2500 缁嗘按骞崇嚎銆?
[v1.3]锛氳繖浜涘瓧绗﹀凡琚寮忓姞鍏?Unicode 3.2.0锛涘畠浠娣诲姞鍦?U+23BA銆乁+23BB銆?U+23BC銆乁+23BD銆侺inux 鐜板湪浣跨敤鏂扮殑鍊笺€?
[v1.2]锛氭坊鍔犱簡浠ヤ笅瀛楃浠ヨ〃绀哄父瑙佺殑閿洏绗﹀彿锛岃繖浜涚鍙蜂笉澶彲鑳借姝ｅ紡鍔犲叆
Unicode锛屽洜涓哄畠浠槸鏋佸己鐨勫巶鍟嗕笓鏈夊唴瀹广€傝繖褰撶劧鏄竴涓碂绯曡璁＄殑缁濅匠鑼冧緥銆?
====== ======================================
U+F810 閿洏绗﹀彿 椋炶鏃楀笢
U+F811 閿洏绗﹀彿 涓嬫媺鑿滃崟
U+F812 閿洏绗﹀彿 寮€鑻规灉
U+F813 閿洏绗﹀彿 瀹炲績鑻规灉
====== ======================================

### 鍏嬫灄璐¤鏀寔


1996 骞达紝Linux 鎴愪负涓栫晫涓婄涓€涓坊鍔犲浜洪€犺瑷€鍏嬫灄璐¤鏀寔鐨勬搷浣滅郴缁燂紝璇ヨ瑷€
鐢?Marc Okrand 涓恒€婃槦闄呰糠鑸€嬬數瑙嗗墽鍒涢€犮€傝繖绉嶇紪鐮佸悗鏉ヨ ConScript Unicode
娉ㄥ唽琛ㄩ噰绾筹紝骞惰鎻愯锛堜絾鏈€缁堣鎷掔粷锛夌撼鍏?Unicode 骞抽潰 1銆傚洜姝わ紝瀹冧綔涓?Linux/CSUR 鐨勭鏈夊垎閰嶄繚鐣欏湪 Linux 鍖轰腑銆?
姝ょ紪鐮佸凡寰楀埌鍏嬫灄璐¤瑷€鐮旂┒鎵€鐨勮鍙€傛洿澶氫俊鎭鑱旂郴浠栦滑锛?
	http://www.kli.org/

鐢变簬 Linux CZ 寮€澶寸殑瀛楃澶氫负瑁呴グ绗﹀彿/绗﹀彿/琛ㄥ崟绫伙紝鑰岃繖鏄竴绉嶈瑷€锛屾垜灏嗗叾
鏀惧湪鏈熬锛屼綅浜庝竴涓?16 鍗曞厓鐨勮竟鐣屼笂锛屼互绗﹀悎鏍囧噯 Unicode 鎯緥銆?
  姝よ寖鍥寸幇鍦ㄧ敱 ConScript Unicode 娉ㄥ唽琛ㄦ寮忕鐞嗐€傝鑼冩€у弬鑰冧綅浜庯細

	https://www.evertype.com/standards/csur/klingon.html

鍏嬫灄璐¤鏈?26 涓瓧绗︾殑瀛楁瘝琛ㄣ€佷竴涓甫 10 涓暟瀛楃殑浣嶇疆鏁板瓧涔﹀啓绯荤粺锛屼功鍐欐柟鍚戜负
浠庡乏鍒板彸銆佷粠涓婂埌涓嬨€?
宸茬粡鎻愬嚭浜嗗嚑绉嶅厠鏋楄础瀛楁瘝鐨勫瓧褰㈠舰寮忋€傜劧鑰岋紝鐢变簬绗﹀彿闆嗕技涔庢暣浣撲竴鑷达紝鍙湁瀹為檯
褰㈢姸涓嶅悓锛屾寜鐓ф爣鍑?Unicode 鎯緥锛岃繖浜涘樊寮傝瑙嗕负瀛椾綋鍙樹綋銆?
======	=======================================================
U+F8D0	KLINGON LETTER A
U+F8D1	KLINGON LETTER B
U+F8D2	KLINGON LETTER CH
U+F8D3	KLINGON LETTER D
U+F8D4	KLINGON LETTER E
U+F8D5	KLINGON LETTER GH
U+F8D6	KLINGON LETTER H
U+F8D7	KLINGON LETTER I
U+F8D8	KLINGON LETTER J
U+F8D9	KLINGON LETTER L
U+F8DA	KLINGON LETTER M
U+F8DB	KLINGON LETTER N
U+F8DC	KLINGON LETTER NG
U+F8DD	KLINGON LETTER O
U+F8DE	KLINGON LETTER P
U+F8DF	KLINGON LETTER Q
 - 鍦ㄦ爣鍑?Okrand 鎷変竵杞啓涓啓浣?<q>
U+F8E0	KLINGON LETTER QH
 - 鍦ㄦ爣鍑?Okrand 鎷変竵杞啓涓啓浣?<Q>
U+F8E1	KLINGON LETTER R
U+F8E2	KLINGON LETTER S
U+F8E3	KLINGON LETTER T
U+F8E4	KLINGON LETTER TLH
U+F8E5	KLINGON LETTER U
U+F8E6	KLINGON LETTER V
U+F8E7	KLINGON LETTER W
U+F8E8	KLINGON LETTER Y
U+F8E9	KLINGON LETTER 澹伴棬鍋滈】绗?
U+F8F0	KLINGON DIGIT ZERO
U+F8F1	KLINGON DIGIT ONE
U+F8F2	KLINGON DIGIT TWO
U+F8F3	KLINGON DIGIT THREE
U+F8F4	KLINGON DIGIT FOUR
U+F8F5	KLINGON DIGIT FIVE
U+F8F6	KLINGON DIGIT SIX
U+F8F7	KLINGON DIGIT SEVEN
U+F8F8	KLINGON DIGIT EIGHT
U+F8F9	KLINGON DIGIT NINE

U+F8FD	KLINGON COMMA
U+F8FE	KLINGON FULL STOP
U+F8FF	KLINGON SYMBOL FOR EMPIRE
======	=======================================================

### 鍏跺畠铏氭瀯涓庝汉宸ユ枃瀛?

鑷粠鍒嗛厤浜嗗厠鏋楄础 Linux Unicode 鍧椾互鏉ワ紝涓€涓櫄鏋勪笌浜哄伐鏂囧瓧鐨勬敞鍐岃〃宸茬敱
John Cowan <jcowan@reutershealth.com> 鍜?Michael Everson <everson@evertype.com>
寤虹珛銆侰onScript Unicode 娉ㄥ唽琛ㄤ綅浜庯細

	  https://www.evertype.com/standards/csur/

鎵€浣跨敤鐨勮寖鍥翠綅浜庢渶缁堢敤鎴峰尯鐨勪綆绔紝鍥犳涓嶈兘琚鑼冩€у湴鍒嗛厤锛屼絾寤鸿甯屾湜缂栫爜
铏氭瀯鏂囧瓧鐨勪汉鍑轰簬浜掓搷浣滄€х殑鑰冭檻浣跨敤杩欎簺浠ｇ爜銆傚浜庡厠鏋楄础璇紝CSUR 宸查噰绾?Linux 缂栫爜銆侰SUR 鏂归潰姝ｅ湪鎺ㄥ姩灏?Tengwar 鍜?Cirth 鍔犲叆 Unicode 骞抽潰 1锛涘皢鍏嬫灄璐¤
鍔犲叆 Unicode 骞抽潰 1 宸茶鎷掔粷锛屽洜姝や笂杩扮紪鐮佷粛鐒朵负瀹樻柟缂栫爜銆?