## MD 闆嗙兢


闆嗙兢 MD 鏄竴绉嶇敤浜庨泦缇ょ殑鍏变韩璁惧 RAID锛屽畠鏀寔涓や釜绾у埆锛歳aid1 鍜?raid10锛堟敮鎸佹湁闄愶級銆?

## 1. 纾佺洏鏍煎紡


姣忎釜闆嗙兢鑺傜偣浣跨敤鐙珛鐨勫啓鎰忓浘浣嶅浘锛坵rite-intent-bitmap锛夈€傝繖浜涗綅鍥捐褰曚簡璇ヨ妭鐐逛笂鍙兘宸茬粡鍚姩鐨勬墍鏈夊啓鍏ワ紝

```

  0                    4k                     8k                    12k
  -------------------------------------------------------------------
  | idle                | md super            | bm super [0] + bits |
  | bm bits[0, contd]   | bm super[1] + bits  | bm bits[1, contd]   |
  | bm super[2] + bits  | bm bits [2, contd]  | bm super[3] + bits |
  | bm bits [3, contd]  |                     |                     |

```
鍦ㄢ€滄甯糕€濊繍琛岃繃绋嬩腑锛屾垜浠亣璁炬枃浠剁郴缁熺‘淇濅换鎰忔椂鍒诲彧鏈変竴涓妭鐐瑰啓鍏ョ粰瀹氱殑鍧楋紝鍥犳涓€娆″啓鍏ヨ姹備細锛?
 - 璁剧疆鐩稿簲鐨勪綅锛堝鏋滃皻鏈缃級
 - 灏嗗啓鍏ユ彁浜ゅ埌鎵€鏈夐暅鍍? - 瀹夋帓鍦ㄨ秴鏃跺悗娓呴櫎璇ヤ綅銆?
璇诲彇鎸夋甯告柟寮忓鐞嗐€傜敱鏂囦欢绯荤粺璐熻矗纭繚鏌愪釜鑺傜偣涓嶄細浠庡彟涓€涓妭鐐癸紙鎴栧悓涓€鑺傜偣锛夋鍦ㄥ啓鍏ョ殑浣嶇疆璇诲彇銆?

## 2. 鐢ㄤ簬绠＄悊鐨?DLM 閿?

鏈変笁缁勯攣鐢ㄤ簬绠＄悊璁惧锛?
### 2.1 浣嶅浘閿佽祫婧愶紙bm_lockres锛?

 bm_lockres 淇濇姢鍚勪釜鑺傜偣鐨勪綅鍥俱€傚叾鍛藉悕褰㈠紡涓猴細node 1 瀵瑰簲 bitmap000锛宯ode 2 瀵瑰簲 bitmap001锛屼緷姝ょ被鎺ㄣ€傚綋鑺傜偣鍔犲叆闆嗙兢鏃讹紝瀹冧互 PW 妯″紡鑾峰彇璇ラ攣锛屽苟鍦ㄨ妭鐐逛綔涓洪泦缇ゆ垚鍛樼殑鏁翠釜鐢熷懡鍛ㄦ湡鍐呬竴鐩存寔鏈夈€傞攣璧勬簮缂栧彿鍩轰簬 DLM 瀛愮郴缁熻繑鍥炵殑妲藉彿銆傜敱浜?DLM 鐨勮妭鐐硅鏁颁粠 1 寮€濮嬶紝鑰屼綅鍥炬Ы浣嶄粠 0 寮€濮嬶紝鍥犳鐢?DLM 妲藉彿鍑忓幓 1 寰楀埌浣嶅浘妲藉彿銆?
 鏌愪釜鑺傜偣浣嶅浘閿佺殑 LVB 璁板綍浜嗚鑺傜偣姝ｅ湪閲嶆柊鍚屾鐨勬墖鍖鸿寖鍥淬€傚叾浠栬妭鐐逛笉寰楀啓鍏ヨ繖浜涙墖鍖恒€傝繖鍦ㄦ湁鏂拌妭鐐瑰姞鍏ラ泦缇ゆ椂浣跨敤銆?
### 2.2 娑堟伅浼犻€掗攣


 姣忎釜鑺傜偣鍦ㄥ惎鍔ㄦ垨缁撴潫閲嶆柊鍚屾锛屼互鍙婅繘琛屽厓鏁版嵁瓒呯骇鍧楁洿鏂版椂锛屽繀椤讳笌鍏朵粬鑺傜偣閫氫俊銆傝閫氫俊閫氳繃涓変釜閿佺鐞嗭細鈥渢oken鈥濄€佲€渕essage鈥?鍜?鈥渁ck鈥濓紝浠ュ強鍏朵腑涓€涓?鈥渕essage鈥?閿佺殑閿佸€煎潡锛圠VB锛夈€?
### 2.3 鏂拌澶囩鐞?

 浣跨敤鍗曚釜閿?鈥渘o-new-dev鈥?鏉ュ崗璋冩柊璁惧鐨勬坊鍔犫€斺€旇繖蹇呴』鍦ㄦ暣涓樀鍒椾腑鍚屾銆傞€氬父鎵€鏈夎妭鐐归兘瀵硅璁惧鎸佹湁骞跺彂璇婚攣銆?
## 3. 閫氫俊


 娑堟伅鍙互骞挎挱鍒版墍鏈夎妭鐐癸紝鍙戦€佹柟鍦ㄧ户缁箣鍓嶇瓑寰呮墍鏈夊叾浠栬妭鐐圭‘璁よ娑堟伅銆備换鎰忔椂鍒诲彧鑳藉鐞嗕竴鏉℃秷鎭€?
### 3.1 娑堟伅绫诲瀷


 鍏辨湁鍏浼犻€掔殑娑堟伅绫诲瀷锛?
##### 3.1.1 METADATA_UPDATED


   閫氱煡鍏朵粬鑺傜偣鍏冩暟鎹凡鏇存柊锛岃鑺傜偣蹇呴』閲嶆柊璇诲彇 md 瓒呯骇鍧椼€傝繖鏄悓姝ユ墽琛岀殑銆傚畠涓昏鐢ㄤ簬鍙戝嚭璁惧鏁呴殰淇″彿銆?
##### 3.1.2 RESYNCING

   閫氱煡鍏朵粬鑺傜偣閲嶆柊鍚屾宸插惎鍔ㄦ垨缁撴潫锛屼互渚垮悇鑺傜偣鍙互鎸傝捣鎴栨仮澶嶈鍖哄煙銆傛瘡鏉?RESYNCING 娑堟伅鏍囪瘑鍙戦€佽妭鐐瑰嵆灏嗛噸鏂板悓姝ョ殑璁惧鑼冨洿銆傝繖浼氳鐩栬鑺傜偣涔嬪墠鐨勪换浣曢€氱煡锛氭瘡涓妭鐐逛竴娆″彧鑳介噸鏂板悓姝ヤ竴涓寖鍥淬€?
##### 3.1.3 NEWDISK


   閫氱煡鍏朵粬鑺傜偣姝ｅ湪鍚戦樀鍒楁坊鍔犺澶囥€傛秷鎭寘鍚璁惧鐨勬爣璇嗙銆傛洿澶氱粏鑺傝涓嬫枃銆?
##### 3.1.4 REMOVE


   涓€涓晠闅滆澶囨垨澶囩敤璁惧姝ｄ粠闃靛垪涓Щ闄ゃ€傛秷鎭腑鍖呭惈璇ヨ澶囩殑妲藉彿銆?
 3.1.5 RE_ADD:

   姝ｉ噸鏂版縺娲讳竴涓晠闅滆澶団€斺€斿叾鍓嶆彁鏄凡纭璇ヨ澶囨仮澶嶆甯稿伐浣溿€?
 3.1.6 BITMAP_NEEDS_SYNC:

   濡傛灉涓€涓妭鐐瑰湪鏈湴鍋滄浣嗕綅鍥句笉骞插噣锛屽垯閫氱煡鍙︿竴涓妭鐐规帴绠￠噸鏂板悓姝ョ殑鎵€鏈夋潈銆?
### 3.2 閫氫俊鏈哄埗


 璇?DLM 鐨?LVB 鐢ㄤ簬鍦ㄩ泦缇よ妭鐐逛箣闂撮€氫俊銆傜敤浜庢鐩殑鐨勬湁涓変釜璧勬簮锛?
##### 3.2.1 token

   淇濇姢鏁翠釜閫氫俊绯荤粺鐨勮祫婧愩€傛寔鏈?token 璧勬簮鐨勮妭鐐规墠鍏佽閫氫俊銆?
##### 3.2.2 message

   鎼哄甫寰呴€氫俊鏁版嵁鐨勯攣璧勬簮銆?
##### 3.2.3 ack


   鑾峰彇璇ヨ祫婧愭剰鍛崇潃娑堟伅宸茶闆嗙兢涓墍鏈夎妭鐐圭‘璁ゃ€傝璧勬簮鐨?BAST 鐢ㄤ簬閫氱煡鎺ユ敹鑺傜偣鏈夎妭鐐规兂瑕侀€氫俊銆?
璇ョ畻娉曚负锛?
```

	sender                         receiver                 receiver
	"ack":CR                       "ack":CR                 "ack":CR

 2. sender get EX on "token",
    sender get EX on "message"::

	sender                        receiver                 receiver
	"token":EX                    "ack":CR                 "ack":CR
	"message":EX
	"ack":CR

    Sender checks that it still needs to send a message. Messages
    received or other events that happened while waiting for the
    "token" may have made this message inappropriate or redundant.

 3. sender writes LVB

    sender down-convert "message" from EX to CW

    sender try to get EX of "ack"

    ::

      [ wait until all receivers have *processed* the "message" ]

                                       [ triggered by bast of "ack" ]
                                       receiver get CR on "message"
                                       receiver read LVB
                                       receiver processes the message
                                       [ wait finish ]
                                       receiver releases "ack"
                                       receiver tries to get PR on "message"

     sender                         receiver                  receiver
     "token":EX                     "message":CR              "message":CR
     "message":CW
     "ack":EX

 4. triggered by grant of EX on "ack" (indicating all receivers
    have processed message)

    sender down-converts "ack" from EX to CR

    sender releases "message"

    sender releases "token"

    ::

                                 receiver upconvert to PR on "message"
                                 receiver get CR of "ack"
                                 receiver release "message"

     sender                      receiver                   receiver
     "ack":CR                    "ack":CR                   "ack":CR


```
## 4. 鏁呴殰澶勭悊


### 4.1 鑺傜偣鏁呴殰


 褰撹妭鐐瑰彂鐢熸晠闅滄椂锛孌LM 浼氶€氳繃妲藉彿閫氱煡闆嗙兢銆傝鑺傜偣鍚姩涓€涓泦缇ゆ仮澶嶇嚎绋嬨€傞泦缇ゆ仮澶嶇嚎绋嬩細锛?
 - 鑾峰彇鏁呴殰鑺傜偣鐨?bitmap<number> 閿? - 鎵撳紑浣嶅浘
 - 璇诲彇鏁呴殰鑺傜偣鐨勪綅鍥? - 灏嗗凡缃綅鐨勪綅鍥惧鍒跺埌鏈湴鑺傜偣
 - 娓呯┖鏁呴殰鑺傜偣鐨勪綅鍥? - 閲婃斁鏁呴殰鑺傜偣鐨?bitmap<number> 閿? - 鍦ㄥ綋鍓嶈妭鐐逛笂鍚姩浣嶅浘鐨勯噸鏂板悓姝?	  recover_bitmaps 鍐呴儴璋冪敤 md_check_recovery锛?	  鐒跺悗 md_check_recovery -> metadata_update_start/finish锛?	  瀹冧細閫氳繃 lock_comm 閿佸畾閫氫俊銆?	  杩欐剰鍛崇潃褰撲竴涓妭鐐规鍦ㄩ噸鏂板悓姝ユ椂锛屼細闃绘鎵€鏈?	  鍏朵粬鑺傜偣瀵归樀鍒楃殑浠讳綍浣嶇疆杩涜鍐欏叆銆?
 閲嶆柊鍚屾杩囩▼鏄父瑙勭殑 md 閲嶆柊鍚屾銆傜劧鑰岋紝鍦ㄩ泦缇ょ幆澧冧腑鎵ц閲嶆柊鍚屾鏃讹紝闇€瑕佹妸琚寕璧风殑鍖哄煙鍛婄煡鍏朵粬鑺傜偣銆傚湪閲嶆柊鍚屾寮€濮嬪墠锛岃妭鐐逛細鍙戝嚭甯︽湁闇€瑕佹寕璧峰尯鍩?(lo,hi) 鑼冨洿鐨?RESYNCING銆傛瘡涓妭鐐圭淮鎶や竴涓?suspend_list锛屽叾涓寘鍚綋鍓嶈鎸傝捣鐨勮寖鍥村垪琛ㄣ€傛敹鍒?RESYNCING 鍚庯紝鑺傜偣灏嗚鑼冨洿鍔犲叆 suspend_list銆傜被浼煎湴锛屽綋鎵ц閲嶆柊鍚屾鐨勮妭鐐瑰畬鎴愭椂锛屽畠浼氬悜鍏朵粬鑺傜偣鍙戦€佸甫鏈夌┖鑼冨洿鐨?RESYNCING锛屽叾浠栬妭鐐瑰垯浠?suspend_list 涓Щ闄ょ浉搴旀潯鐩€?
 杈呭姪鍑芥暟 ->area_resyncing() 鍙敤浜庢鏌ユ煇涓壒瀹氱殑 I/O 鑼冨洿鏄惁搴旇鎸傝捣銆?
## 4.2 璁惧鏁呴殰


 璁惧鏁呴殰閫氳繃鍏冩暟鎹洿鏂颁緥绋嬭繘琛屽鐞嗗拰閫氭姤銆傚綋鑺傜偣妫€娴嬪埌璁惧鏁呴殰鏃讹紝鍦ㄦ晠闅滆鎵€鏈夊叾浠栬妭鐐圭‘璁や箣鍓嶏紝涓嶅厑璁稿璇ヨ澶囪繘琛屼换浣曡繘涓€姝ュ啓鍏ャ€?
### 5. 娣诲姞鏂拌澶?

 瑕佹坊鍔犳柊璁惧锛屽繀椤昏鎵€鏈夎妭鐐归兘鑳解€滅湅鍒扳€濊娣诲姞鐨勬柊璁惧銆備负姝や娇鐢ㄤ互涓嬬畻娉曪細

   1. Node 1 鎵ц mdadm --manage /dev/mdX --add /dev/sdYY锛屽叾鍙戝嚭 ioctl(ADD_NEW_DISK锛屽叾涓?disc.state 璁句负 MD_DISK_CLUSTER_ADD)
   2. Node 1 鍙戦€佸甫鏈?uuid 鍜屾Ы鍙风殑 NEWDISK 娑堟伅
   3. 鍏朵粬鑺傜偣鍙戝嚭甯︽湁 uuid 鍜屾Ы鍙风殑 kobject_uevent_env锛堟楠?4銆? 鍙兘鏄竴鏉?udev 瑙勫垯锛?   4. 鍦ㄧ敤鎴风┖闂达紝鑺傜偣鎼滅储纾佺洏锛屽彲鑳戒娇鐢?blkid -t SUB_UUID=""
   5. 鍏朵粬鑺傜偣鏍规嵁鏄惁鎵惧埌纾佺洏锛屽彂鍑轰互涓嬩换涓€鎿嶄綔锛?       ioctl(ADD_NEW_DISK锛屽叾涓?disc.state 璁句负 MD_DISK_CANDIDATE锛屼笖
       disc.number 璁句负妲藉彿)
       ioctl(CLUSTERED_DISK_NACK)
   6. 濡傛灉鎵惧埌璁惧锛屽叾浠栬妭鐐归噴鏀?"no-new-devs" 涓婄殑閿侊紙CR锛?   7. Node 1 灏濊瘯鑾峰彇 "no-new-dev" 鐨?EX 閿?   8. 濡傛灉 Node 1 鑾峰彇鍒伴攣锛屽垯鍙栨秷璇ョ鐩樼殑 SpareLocal 鏍囪鍚庡彂閫?METADATA_UPDATED
   9. 鍚﹀垯锛堟湭鑾峰彇鍒?"no-new-dev" 閿侊級锛屽垯鎿嶄綔澶辫触骞跺彂閫?METADATA_UPDATED
   10. 鍏朵粬鑺傜偣閫氳繃鍚庣画鐨?METADATA_UPDATED 鑾风煡纾佺洏鏄惁琚坊鍔犮€?
## 6. 妯″潡鎺ュ彛


 鏈?17 涓洖璋冩槸 md 鏍稿績鍙互鍚戦泦缇ゆā鍧楀彂璧风殑銆傜悊瑙ｈ繖浜涘洖璋冨彲浠ュ緢濂藉湴浠庢暣浣撲笂浜嗚В鏁翠釜杩囩▼銆?
### 6.1 join(nodes) 鍜?leave()


 褰撲互闆嗙兢浣嶅浘鍚姩闃靛垪浠ュ強鍋滄闃靛垪鏃惰皟鐢ㄥ畠浠€俲oin() 纭繚闆嗙兢鍙敤骞跺垵濮嬪寲鍚勭璧勬簮銆傞泦缇や腑鍙湁鍓?'nodes' 涓妭鐐瑰彲浠ヤ娇鐢ㄨ闃靛垪銆?
### 6.2 slot_number()


 鎶ュ憡闆嗙兢鍩虹璁炬柦寤鸿鐨勬Ы鍙枫€傝寖鍥翠负 0 鍒?nodes-1銆?
### 6.3 resync_info_update()


 杩欐洿鏂板瓨鍌ㄥ湪浣嶅浘閿佷腑鐨勯噸鏂板悓姝ヨ寖鍥淬€傝捣鐐归殢閲嶆柊鍚屾鐨勬帹杩涜€屾洿鏂般€傜粓鐐瑰缁堜负闃靛垪鐨勬湯灏俱€傚畠**涓?*鍙戦€?RESYNCING 娑堟伅銆?
### 6.4 resync_start()銆乺esync_finish()


 褰撻噸鏂板悓姝?鎭㈠/閲嶅鍚姩鎴栧仠姝㈡椂璋冪敤瀹冧滑銆傚畠浠洿鏂颁綅鍥鹃攣涓殑閲嶆柊鍚屾鑼冨洿锛屽苟鍙戦€?RESYNCING 娑堟伅銆俽esync_start 灏嗘暣涓樀鍒楁姤鍛婁负姝ｅ湪閲嶆柊鍚屾锛宺esync_finish 鍒欎笉鎶ュ憡浠讳綍閮ㄥ垎銆?
 resync_finish() 杩樹細鍙戦€?BITMAP_NEEDS_SYNC 娑堟伅锛屼娇鍏朵粬鑺傜偣鍙互鎺ョ銆?
### 6.5 metadata_update_start()銆乵etadata_update_finish()銆乵etadata_update_cancel()


 metadata_update_start 鐢ㄤ簬鑾峰彇瀵瑰厓鏁版嵁鐨勭嫭鍗犺闂€備竴鏃﹁幏寰楄璁块棶鍚庝粛鏈夊彉鏇撮渶瑕佹椂锛宮etadata_update_finish() 浼氬悜鎵€鏈夊叾浠栬妭鐐瑰彂閫?METADATA_UPDATE 娑堟伅锛涘惁鍒欏彲浣跨敤 metadata_update_cancel() 閲婃斁璇ラ攣銆?
### 6.6 area_resyncing()


 瀹冪粨鍚堜簡涓ら儴鍒嗗姛鑳姐€?
 棣栧厛锛屽畠浼氭鏌ユ槸鍚︽湁鑺傜偣褰撳墠姝ｅ湪缁欏畾鎵囧尯鑼冨洿鍐呴噸鏂板悓姝ャ€傚鏋滃彂鐜颁换浣曢噸鏂板悓姝ワ紝璋冪敤鏂瑰皢閬垮厤鍦ㄨ鑼冨洿鍐呭啓鍏ユ垨杩涜璇诲潎琛°€?
 鍏舵锛屽湪鑺傜偣鎭㈠鏈熼棿锛屽畠浼氭姤鍛婃墍鏈夊尯鍩熷 READ 璇锋眰閮藉浜庨噸鏂板悓姝ョ姸鎬併€傝繖閬垮厤浜嗛泦缇ゆ枃浠剁郴缁熶笌闆嗙兢 RAID 鍦ㄥ鐞嗚妭鐐规晠闅滄椂鍑虹幇绔炴€併€?
### 6.7 add_new_disk_start()銆乤dd_new_disk_finish()銆乶ew_disk_ack()


 杩欎簺鐢ㄤ簬绠＄悊涓婅堪鏂扮鐩樺崗璁€傛坊鍔犳柊璁惧鏃讹紝鍦ㄨ澶囩粦瀹氬埌闃靛垪涔嬪墠璋冪敤 add_new_disk_start()锛屽鏋滄垚鍔燂紝鍒欒皟鐢?add_new_disk_finish() 瀹屾垚璁惧鐨勫畬鏁存坊鍔犮€?
 褰撹澶囦綔涓哄鍏堝墠璇锋眰鐨勭‘璁よ€岃娣诲姞锛屾垨褰撹澶囪澹版槑涓衡€滀笉鍙敤鈥濇椂锛岃皟鐢?new_disk_ack()銆?
### 6.8 remove_disk()


 褰撳鐢ㄨ澶囨垨鏁呴殰璁惧浠庨樀鍒椾腑绉婚櫎鏃惰皟鐢ㄣ€傚畠浼氬悜鍏朵粬鑺傜偣鍙戦€佷竴鏉?REMOVE 娑堟伅銆?
### 6.9 gather_bitmaps()


 杩欎細鍚戞墍鏈夊叾浠栬妭鐐瑰彂閫佷竴鏉?RE_ADD 娑堟伅锛岀劧鍚庝粠鎵€鏈変綅鍥炬敹闆嗕綅鍥句俊鎭€傝鍚堝苟鍚庣殑浣嶅浘闅忓悗鐢ㄤ簬鎭㈠琚噸鏂版坊鍔犵殑璁惧銆?
### 6.10 lock_all_bitmaps() 鍜?unlock_all_bitmaps()


 褰撴妸浣嶅浘鏀逛负 none 鏃惰皟鐢ㄥ畠浠€傚鏋滄煇涓妭鐐硅鍒掓竻闄ら泦缇?RAID 鐨勪綅鍥撅紝闇€瑕佺‘淇濇病鏈夊叾浠栬妭鐐规鍦ㄤ娇鐢ㄨ RAID锛岃繖閫氳繃閿佸畾闆嗙兢鍐呮墍鏈変綅鍥鹃攣鏉ュ疄鐜帮紝杩欎簺閿佷篃浼氱浉搴斿湴琚В閿併€?
## 7. 涓嶆敮鎸佺殑鐗规€?

闆嗙兢 MD 鐩墠灏氫笉鏀寔浠ヤ笅鍔熻兘銆?
- 鏇存敼 array_sectors銆?