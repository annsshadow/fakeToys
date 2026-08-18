
## Linux 涓嬬殑 BeOS 鏂囦欢绯荤粺


鏂囨。鏈€鍚庢洿鏂帮細2001 骞?12 鏈?6 鏃?
## 璀﹀憡


璇峰姟蹇呮槑鐧借繖鏄?alpha 闃舵鐨勮蒋浠躲€傝繖鎰忓懗鐫€璇ュ疄鐜版棦涓嶅畬鏁达紝涔熸湭缁忚繃鍏呭垎娴嬭瘯銆?
瀵逛簬姝や唬鐮佸彲鑳介€犳垚鐨勪换浣曚笉鑹悗鏋滐紝鏈汉涓嶆壙鎷呬换浣曡矗浠伙紒

## 璁稿彲璇?

鏈蒋浠跺彈 GNU 閫氱敤鍏叡璁稿彲璇佷繚鎶ゃ€傚畬鏁磋鍙瘉鏂囨湰璇疯 COPYING 鏂囦欢銆?鎴栬闂?GNU 缃戠珯锛?http://www.gnu.org/licenses/licenses.html>

## 浣滆€?

浠ｇ爜鐨勫ぇ閮ㄥ垎鐢?Will Dyson <will_dyson@pobox.com> 缂栧啓銆備粬鑷?2001 骞?8 鏈?13 鏃ヨ捣涓€鐩?浠庝簨璇ヤ唬鐮佺殑宸ヤ綔銆傝瑙?changelog銆?
鍘熷浣滆€咃細Makoto Kato <m_kato@ga2.so-net.ne.jp>

浠栫殑鍘熷浠ｇ爜浠嶅彲鍦ㄤ互涓嬩綅缃壘鍒帮細
<http://hp.vector.co.jp/authors/VA008030/bfs/>

鏈変汉鐭ラ亾 Makoto 鏇磋繎鏈熺殑鐢靛瓙閭欢鍦板潃鍚楋紵浠栧涓婅堪鍦板潃宸蹭笉鍐嶅洖澶嶁€︹€?
璇ユ枃浠剁郴缁熺洰鍓嶆病鏈夌淮鎶よ€呫€?
## 杩欎釜椹卞姩鏄粈涔堬紵


璇ユā鍧椾负 Linux 2.4.1 鍙婃洿鏂扮増鏈殑鍐呮牳瀹炵幇浜?BeOS锛坔ttp://www.beincorporated.com/锛夌殑鍘熺敓
鏂囦欢绯荤粺銆傜洰鍓嶅畠鏄竴涓彧璇诲疄鐜般€?
## 鍒板簳鍙?BFS 杩樻槸 BEFS锛?

Be, Inc. 鏇捐〃绀猴紝鈥淏eOS 鏂囦欢绯荤粺鍦ㄥ畼鏂逛笂绉颁负 BFS锛岃€岄潪 BeFS鈥濄€備絾 Unixware 鐨?Boot Filesystem
涔熷彨 bfs锛岃€屼笖瀹冧滑宸茬粡鍦ㄥ唴鏍镐腑浜嗐€傜敱浜庤繖涓€鍛藉悕鍐茬獊锛屽湪 Linux 涓?BeOS 鏂囦欢绯荤粺琚О涓?befs銆?
## 濡備綍瀹夎


姝ラ 1. 灏?BeFS 琛ヤ竵瀹夎鍒?linux 婧愮爜鏍戜腑銆?
灏嗚ˉ涓佹枃浠跺簲鐢ㄥ埌浣犵殑鍐呮牳婧愮爜鏍戙€傚亣璁句綘鐨勫唴鏍告簮鐮佷綅浜?/foo/bar/linux锛岃ˉ涓佹枃浠跺悕涓?patch-befs-xxx锛屽垯搴旀墽琛屽涓嬫搷浣滐細

	cd /foo/bar/linux
	patch -p1 < /path/to/patch-befs-xxx

濡傛灉鎵撹ˉ涓佹楠ゅけ璐ワ紙鍗冲嚭鐜拌鎷掔粷鐨?hunk锛夛紝浣犲彲浠ュ皾璇曡嚜宸辫В鍐筹紙杩欏苟涓嶉毦锛夛紝鎴栧彂閭欢鍚戠淮鎶よ€?锛圵ill Dyson <will_dyson@pobox.com>锛夋眰鍔┿€?
姝ラ 2. 閰嶇疆骞剁紪璇戝唴鏍?
Linux 鍐呮牳鏈夎澶氱紪璇戞湡閫夐」锛屽叾涓ぇ澶氭暟瓒呭嚭浜嗘湰鏂囨。鐨勮寖鍥淬€傛垜鎺ㄨ崘灏?Kernel-HOWTO 鏂囨。浣滀负
杩欎竴涓婚鐨勮壇濂介€氱敤鍙傝€冦€俬ttp://www.linuxdocs.org/HOWTOs/Kernel-HOWTO-4.html

```

	cd /foo/bar/linux
	make menuconfig (鎴?xconfig)

```
BefS 妯″潡骞堕潪 Linux 鍐呮牳鐨勬爣鍑嗙粍鎴愰儴鍒嗭紝鍥犳浣犲繀椤诲厛鍦ㄢ€淐ode maturity level鈥濊彍鍗曚笅鍚敤瀵?瀹為獙鎬т唬鐮佺殑鏀寔銆?
鐒跺悗锛屽湪鈥淔ilesystems鈥濊彍鍗曚笅浼氬嚭鐜颁竴涓悕涓衡€淏eFS filesystem (experimental)鈥濇垨绫讳技鍚嶇О鐨勯€夐」銆?鍚敤璇ラ€夐」锛堝皢鍏剁紪璇戜负妯″潡浜﹀彲锛夈€?
淇濆瓨浣犵殑鍐呮牳閰嶇疆锛岀劧鍚庣紪璇戝唴鏍搞€?
姝ラ 3. 瀹夎

鍏充簬杩欎竴鍏抽敭姝ラ鐨勮鏄庯紝璇峰弬瑙佸唴鏍?howto <http://www.linux.com/howto/Kernel-HOWTO.html>銆?
## 浣跨敤 BFS


瑕佷娇鐢?BeOS 鏂囦欢绯荤粺锛岃浣跨敤鏂囦欢绯荤粺绫诲瀷 鈥榖efs鈥欍€?
```

    mount -t befs /dev/fd0 /beos

```
## 鎸傝浇閫夐」


=============  ===========================================================
uid=nnn        All files in the partition will be owned by user id nnn.
gid=nnn	       All files in the partition will be in group nnn.
iocharset=xxx  Use xxx as the name of the NLS translation table.
debug          The driver will output debugging information to the syslog.
=============  ===========================================================

## 濡備綍鑾峰彇鏈€鏂扮増鏈?

鏈€鏂扮増鏈洰鍓嶅彲鍦ㄤ互涓嬩綅缃幏鍙栵細
<http://befs-driver.sourceforge.net/>

## 宸茬煡缂洪櫡锛?

鎴嚦 2002 骞?1 鏈?20 鏃ワ細

	None

## 鐗瑰埆鑷磋阿


Dominic Giampalo 鈥︹€?鎾板啓浜嗐€奝ractical file system design with Be filesystem銆?
Hiroyuki Yamada 鈥︹€?娴嬭瘯浜?LinuxPPC銆?