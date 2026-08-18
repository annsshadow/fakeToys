
## RCU 姒傚康


RCU锛坮ead-copy update锛岃-澶嶅埗-鏇存柊锛夎儗鍚庣殑鍩烘湰鎬濇兂鏄皢鐮村潖鎬ф搷浣滄媶鍒嗕负涓ら儴鍒嗭紝涓€閮ㄥ垎闃绘浠讳綍浜虹湅鍒版鍦ㄨ閿€姣佺殑鏁版嵁椤癸紝鍙︿竴閮ㄥ垎瀹為檯鎵ц閿€姣併€傝繖涓ら儴鍒嗕箣闂村繀椤荤粡鍘嗕竴涓€滃闄愭湡锛坓race period锛夆€濓紝涓旇瀹介檺鏈熷繀椤昏冻澶熼暱锛屼娇寰椾换浣曟鍦ㄨ闂鍒犻櫎椤圭殑璇昏€呮鍚庨兘宸叉斁寮冨叾寮曠敤銆備緥濡傦紝瀵?RCU 淇濇姢鐨勯摼琛ㄨ繘琛屽垹闄わ紝浼氬厛灏嗚椤逛粠閾捐〃涓Щ闄わ紝绛夊緟瀹介檺鏈熻繃鍘伙紝鐒跺悗閲婃斁璇ュ厓绱犮€傚叧浜庡湪閾捐〃涓婁娇鐢?RCU 鐨勬洿澶氫俊鎭紝璇峰弬瑙?listRCU.rst銆?
### 甯歌闂


- 涓轰粈涔堜細鏈変汉鎯宠浣跨敤 RCU锛?
  RCU 涓ゅ垎娉曟柟娉曠殑浼樺娍鍦ㄤ簬 RCU 璇昏€呮棤闇€鑾峰彇浠讳綍閿併€佹墽琛屼换浣曞師瀛愭寚浠ゃ€佸啓鍏ュ叡浜唴瀛橈紝鎴栧湪锛圓lpha 浠ュ鐨勶級CPU 涓婃墽琛屼换浣曞唴瀛樺睆闅溿€傝繖浜涙搷浣滃湪鐜颁唬 CPU 涓婄浉褰撴槀璐碉紝杩欐鏄?RCU 鍦ㄨ澶氬満鏅腑鍏锋湁鎬ц兘浼樺娍鐨勫師鍥犮€俁CU 璇昏€呮棤闇€鑾峰彇閿佷篃鏋佸ぇ绠€鍖栦簡閬垮厤姝婚攣鐨勪唬鐮併€?
- 濡傛灉 RCU 璇昏€呭湪瀹屾垚鍚庢病鏈変换浣曟寚绀猴紝鏇存柊鑰呭浣曞垽鏂闄愭湡宸茬粡瀹屾垚锛?
  涓庤嚜鏃嬮攣涓€鏍凤紝RCU 璇昏€呬笉鍏佽闃诲銆佸垏鎹㈠埌鐢ㄦ埛鎬佹墽琛屾垨杩涘叆绌洪棽寰幆銆傚洜姝わ紝涓€鏃︾湅鍒版煇涓?CPU 缁忓巻浜嗚繖涓夌鐘舵€佷箣涓€锛屾垜浠氨鐭ラ亾璇?CPU 宸茬粡閫€鍑轰簡浠讳綍鍏堝墠鐨?RCU 璇讳晶涓寸晫鍖恒€傛墍浠ワ紝濡傛灉鎴戜滑浠庨摼琛ㄤ腑绉婚櫎涓€椤癸紝鐒跺悗绛夊緟鎵€鏈?CPU 閮借繘琛屼簡涓婁笅鏂囧垏鎹€佸湪鐢ㄦ埛鎬佹墽琛屾垨杩涘叆浜嗙┖闂插惊鐜紝灏卞彲浠ュ畨鍏ㄥ湴閲婃斁璇ラ」銆?
  RCU 鐨勫彲鎶㈠崰鍙樹綋锛圕ONFIG_PREEMPT_RCU锛夎揪鍒扮浉鍚屾晥鏋滐紝浣嗚姹傝鑰呮搷浣?CPU 鏈湴鐨勮鏁板櫒銆傝繖浜涜鏁板櫒鍏佽鍦?RCU 璇讳晶涓寸晫鍖轰腑杩涜鏈夐檺绫诲瀷鐨勯樆濉炪€係RCU 涔熶娇鐢?CPU 鏈湴璁℃暟鍣紝骞跺厑璁稿湪 RCU 璇讳晶涓寸晫鍖轰腑杩涜涓€鑸樆濉炪€傝繖浜?RCU 鍙樹綋閫氳繃閲囨牱杩欎簺璁℃暟鍣ㄦ潵妫€娴嬪闄愭湡銆?
- 濡傛灉鎴戣繍琛屽湪鍙兘涓€娆″仛涓€浠朵簨鐨勫崟澶勭悊鍣紙uniprocessor锛夊唴鏍镐笂锛屼负浠€涔堣繕瑕佺瓑寰呭闄愭湡锛?
  鏇村淇℃伅璇峰弬瑙?UP.rst銆?
- 濡備綍鏌ョ湅 RCU 褰撳墠鍦?Linux 鍐呮牳涓殑浣跨敤浣嶇疆锛?
  鎼滅储 "rcu_read_lock"銆?rcu_read_unlock"銆?call_rcu"銆?rcu_read_lock_bh"銆?rcu_read_unlock_bh"銆?srcu_read_lock"銆?srcu_read_unlock"銆?synchronize_rcu"銆?synchronize_net"銆?synchronize_srcu" 浠ュ強鍏朵粬 RCU 鍘熻銆傛垨鑰呬粠浠ヤ笅鍦板潃鑾峰彇鏌愪釜 cscope 鏁版嵁搴擄細

  (http://www.rdrop.com/users/paulmck/RCU/linuxusage/rculocktab.html)銆?
- 缂栧啓浣跨敤 RCU 鐨勪唬鐮佹椂搴旈伒寰摢浜涘噯鍒欙紵

  璇峰弬瑙?checklist.rst銆?
- 涓轰粈涔堝彨 "RCU"锛?
  "RCU" 浠ｈ〃 "read-copy update"锛堣-澶嶅埗-鏇存柊锛夈€俵istRCU.rst 涓湁鍏充簬璇ュ悕绉扮敱鏉ョ殑鏇村淇℃伅锛屾悳绱?"read-copy update" 鍗冲彲鎵惧埌銆?
- 鎴戝惉璇?RCU 鏈変笓鍒╋紵杩欐槸鎬庝箞鍥炰簨锛?
  鏄殑锛屽畠鏈変笓鍒┿€傛湁澶氫釜宸茬煡鐨勪笌 RCU 鐩稿叧鐨勪笓鍒╋紝鍦?Documentation/RCU/RTFP.txt 涓悳绱㈠瓧绗︿覆 "Patent" 鍗冲彲鎵惧埌瀹冧滑銆傚叾涓竴椤瑰凡琚彈璁╀汉鏀惧純锛屽叾浣欏凡鏍规嵁 GPL 璐＄尞缁?Linux 鍐呮牳銆傝澶氾紙浣嗗苟闈炲叏閮級鏃╁凡杩囨湡銆傜幇鍦ㄤ篃鏈?LGPL 鐨勫疄鐜帮紙鐢ㄦ埛鎬?RCU锛夊彲鐢紙https://liburcu.org/锛夈€?
- 鎴戝惉璇?RCU 闇€瑕佽繘琛屽伐浣滀互鏀寔瀹炴椂锛坮ealtime锛夊唴鏍革紵

  瀹炴椂鍙嬪ソ鐨?RCU 閫氳繃 CONFIG_PREEMPTION 鍐呮牳閰嶇疆鍙傛暟鍚敤銆?
- 鍦ㄥ摢閲屽彲浠ユ壘鍒板叧浜?RCU 鐨勬洿澶氫俊鎭紵

  璇峰弬瑙?Documentation/RCU/RTFP.txt 鏂囦欢銆?  鎴栧皢娴忚鍣ㄦ寚鍚?(https://docs.google.com/document/d/1X0lThx8OK0ZgLMqVoXiR4ZrGURHrXK6NyLRbeXe3Xac/edit)
  鎴?(https://docs.google.com/document/d/1GCdQC8SDbb54W1shjEXqGZ0Rq8a6kIeYutdSIajfpLA/edit?usp=sharing)銆?