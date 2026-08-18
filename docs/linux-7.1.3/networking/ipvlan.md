
## IPVLAN 椹卞姩浣跨敤鎸囧崡


鍒濆鐗堟湰锛?
	Mahesh Bandewar <maheshb AT google.com>

## 1. 绠€浠嬶細

浠庢蹇典笂璁诧紝瀹冧笌 macvlan 椹卞姩闈炲父鐩镐技锛屼富瑕佸尯鍒湪浜庝娇鐢?L3 鍦ㄤ粠璁惧锛坰lave锛変箣闂磋繘琛屽璺鐢?瑙ｅ鐢ㄣ€傝繖涓€鐗规€т娇寰椾富璁惧涓庡叾浠庤澶囧叡浜?L2銆傛垜鏄湪閰嶅悎缃戠粶鍛藉悕绌洪棿寮€鍙戣繖涓┍鍔ㄧ殑锛屼笉纭畾鍦ㄦ涔嬪鏄惁杩樻湁鍏跺畠浣跨敤鍦烘櫙銆?


## 2. 鏋勫缓涓庡畨瑁咃細


涓轰簡鏋勫缓璇ラ┍鍔紝璇烽€夋嫨閰嶇疆椤?CONFIG_IPVLAN銆傝椹卞姩鍙互鍐呭缓鍒板唴鏍镐腑锛圕ONFIG_IPVLAN=y锛夛紝涔熷彲浠ヤ綔涓烘ā鍧楁瀯寤猴紙CONFIG_IPVLAN=m锛夈€?


## 3. 閰嶇疆锛?


璇ラ┍鍔ㄦ病鏈夋ā鍧楀弬鏁帮紝鍙互浣跨敤 IProute2/ip 宸ュ叿杩涜閰嶇疆銆?
```

    ip link add link <master> name <slave> type ipvlan [ mode MODE ] [ FLAGS ]
       where
	 MODE: l3 (default) | l3s | l2
	 FLAGS: bridge (default) | private | vepa

```
渚嬪锛?

    (a) 浠ヤ笅鍛戒护灏嗗垱寤轰竴涓互 eth0 涓轰富璁惧銆佹ā寮忎负
```

	  bash# ip link add link eth0 name ipvl0 type ipvlan
    (b) This command will create IPvlan link in L2 bridge mode::

	  bash# ip link add link eth0 name ipvl0 type ipvlan mode l2 bridge

    (c) This command will create an IPvlan device in L2 private mode::

	  bash# ip link add link eth0 name ipvlan type ipvlan mode l2 private

    (d) This command will create an IPvlan device in L2 vepa mode::

	  bash# ip link add link eth0 name ipvlan type ipvlan mode l2 vepa


```
## 4. 宸ヤ綔妯″紡锛?


IPvlan 鏈変袱绉嶅伐浣滄ā寮忊€斺€擫2 鍜?L3銆傚浜庣粰瀹氱殑涓昏澶囷紝浣犲彲浠ラ€夋嫨杩欎袱绉嶆ā寮忎箣涓€锛岃涓昏澶囦笂鐨勬墍鏈変粠璁惧閮藉皢浠ョ浉鍚岀殑锛堟墍閫夛級妯″紡杩愯銆傞櫎浜嗗湪 L3 妯″紡涓嬩粠璁惧涓嶄細鎺ユ敹浠讳綍澶氭挱/骞挎挱娴侀噺涔嬪锛孯X 妯″紡鍑犱箮鐩稿悓銆侺3 妯″紡闄愬埗鏇村锛屽洜涓鸿矾鐢辨槸浠庡彟涓€涓紙閫氬父鏄粯璁わ級鍛藉悕绌洪棿鎺у埗鐨勩€?

### 4.1 L2 妯″紡锛?


鍦ㄦ妯″紡涓嬶紝TX 澶勭悊鍙戠敓鍦ㄦ寕杞藉埌浠庤澶囩殑鍗忚鏍堝疄渚嬩笂锛屾暟鎹寘琚氦鎹㈠苟鎺掑叆涓昏澶囦互鍙戦€佸嚭鍘汇€傚湪姝ゆā寮忎笅锛屼粠璁惧涔熶細鎺ユ敹/鍙戦€佸鎾拰骞挎挱锛堝閫傜敤锛夈€?

### 4.2 L3 妯″紡锛?


鍦ㄦ妯″紡涓嬶紝鍒?L3 涓烘鐨?TX 澶勭悊鍙戠敓鍦ㄦ寕杞藉埌浠庤澶囩殑鍗忚鏍堝疄渚嬩笂锛屾暟鎹寘琚垏鎹㈠埌涓昏澶囩殑鍗忚鏍堝疄渚嬭繘琛?L2 澶勭悊鍜岃矾鐢憋紝鐒跺悗鍐嶆帓鍏ュ嚭绔欒澶囥€傚湪姝ゆā寮忎笅锛屼粠璁惧鏃笉鑳芥帴鏀朵篃涓嶈兘鍙戦€佸鎾?骞挎挱娴侀噺銆?

### 4.3 L3S 妯″紡锛?


杩欎笌 L3 妯″紡闈炲父鐩镐技锛屽尯鍒湪浜?iptables锛堣繛鎺ヨ窡韪級鍦ㄦ妯″紡涓嬪彲鐢紝鍥犳瀹冩槸 L3 瀵圭О鐨勶紙L3s锛夈€傚叾鎬ц兘浼氱暐浣庝竴浜涳紝浣嗚繖鏃犲叧绱ц锛屽洜涓轰綘閫夋嫨姝ゆā寮忚€岄潪绾?L3 妯″紡鏄负浜嗚杩炴帴璺熻釜姝ｅ父宸ヤ綔銆?

## 5. 妯″紡鏍囧織锛?


鐩墠鎻愪緵浠ヤ笅妯″紡鏍囧織

### 5.1 bridge锛堟ˉ鎺ワ級锛?


杩欐槸榛樿閫夐」銆傝灏?IPvlan 绔彛閰嶇疆涓烘妯″紡锛岀敤鎴峰彲浠ラ€夋嫨鍦ㄥ懡浠よ涓婃坊鍔犺閫夐」锛屾垨鑰呬笉鎸囧畾浠讳綍閫夐」銆傝繖鏄紶缁熸ā寮忥紝浠庤澶囦箣闂村彲浠ヤ簰鐩搁€氫俊锛屼篃鍙互閫氳繃涓昏澶囪繘琛岄€氫俊銆?

### 5.2 private锛堢鏈夛級锛?


濡傛灉鍦ㄥ懡浠よ涓婃坊鍔犳閫夐」锛岀鍙ｅ皢琚缃负绉佹湁妯″紡銆傚嵆绔彛涓嶅厑璁镐粠璁惧涔嬮棿浜掔浉閫氫俊銆?

### 5.3 vepa锛?


濡傛灉鍦ㄥ懡浠よ涓婃坊鍔犳閫夐」锛岀鍙ｅ皢琚缃负 VEPA 妯″紡銆傚嵆绔彛浼氬儚 802.1Qbg 涓弿杩扮殑閭ｆ牱锛屽皢浜ゆ崲鍔熻兘鍗歌浇鍒板閮ㄥ疄浣撱€?
娉ㄦ剰锛欼Pvlan 涓殑 VEPA 妯″紡瀛樺湪闄愬埗銆侷Pvlan 浣跨敤涓昏澶囩殑 MAC 鍦板潃锛屽洜姝ゅ湪姝ゆā寮忎笅涓虹浉閭婚偦灞呭彂鍑虹殑鏁版嵁鍖呯殑婧?MAC 鍜岀洰鐨?MAC 灏嗙浉鍚屻€傝繖灏嗗鑷翠氦鎹㈡満/璺敱鍣ㄥ彂閫侀噸瀹氬悜娑堟伅銆?

## 6. 濡備綍閫夋嫨锛坢acvlan 涓?ipvlan锛夛紵


杩欎袱绉嶈澶囧湪璁稿鏂归潰闈炲父鐩镐技锛屽叿浣撶殑浣跨敤鍦烘櫙寰堝彲鑳藉喅瀹氶€夋嫨鍝竴绉嶈澶囥€傚鏋滀綘鐨勪娇鐢ㄥ満鏅鍚堜互涓嬫煇涓€绉嶆儏鍐碉紝鍒欏彲浠ラ€夋嫨浣跨敤 ipvlan锛?


(a) 杩炴帴鍒板閮ㄤ氦鎹㈡満/璺敱鍣ㄧ殑 Linux 涓绘満閰嶇疆浜嗙瓥鐣ワ紝姣忎釜绔彛鍙厑璁镐竴涓?MAC 鍦板潃銆?
(b) 鍦ㄤ富璁惧涓婂垱寤虹殑铏氭嫙璁惧鏁伴噺瓒呰繃浜?MAC 瀹归噺锛屽鑷寸綉鍗¤繘鍏ユ贩鏉傛ā寮忥紝鎬ц兘涓嬮檷鎴愪负涓€涓棶棰樸€?
(c) 濡傛灉浠庤澶囪琚斁鍏ユ晫瀵?涓嶅彈淇′换鐨勭綉缁滃懡鍚嶇┖闂达紝鍏朵腑浠庤澶囦笂鐨?L2 鍙兘琚洿鏀?婊ョ敤銆?


## 6. 閰嶇疆绀轰緥锛?


```

  +=============================================================+
  |  Host: host1                                                |
  |                                                             |
  |   +----------------------+      +----------------------+    |
  |   |   NS:ns0             |      |  NS:ns1              |    |
  |   |                      |      |                      |    |
  |   |                      |      |                      |    |
  |   |        ipvl0         |      |         ipvl1        |    |
  |   +----------#-----------+      +-----------#----------+    |
  |              #                              #               |
  |              ################################               |
  |                              # eth0                         |
  +==============================#==============================+


```
```

	ip netns add ns0
	ip netns add ns1

```
```

	ip link add link eth0 ipvl0 type ipvlan mode l2
	ip link add link eth0 ipvl1 type ipvlan mode l2

```
```

	ip link set dev ipvl0 netns ns0
	ip link set dev ipvl1 netns ns1

```
(d) 鐜板湪鍒囨崲鍒板懡鍚嶇┖闂达紙ns0 鎴?ns1锛変互閰嶇疆浠庤澶?
```

		(1) ip netns exec ns0 bash
		(2) ip link set dev ipvl0 up
		(3) ip link set dev lo up
		(4) ip -4 addr add 127.0.0.1 dev lo
		(5) ip -4 addr add $IPADDR dev ipvl0
		(6) ip -4 route add default via $ROUTER dev ipvl0

	- For ns1::

		(1) ip netns exec ns1 bash
		(2) ip link set dev ipvl1 up
		(3) ip link set dev lo up
		(4) ip -4 addr add 127.0.0.1 dev lo
		(5) ip -4 addr add $IPADDR dev ipvl1
		(6) ip -4 route add default via $ROUTER dev ipvl1

```