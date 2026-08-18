## ALSA PCM 鏃堕棿鎴?


ALSA API 鍙互鎻愪緵涓ょ涓嶅悓鐨勭郴缁熸椂闂存埑锛?

- Trigger_tstamp锛堣Е鍙戞椂闂存埑锛夋槸鍦?.trigger 鍥炶皟琚皟鐢ㄦ椂鑾峰彇鐨勭郴缁熸椂闂村揩鐓с€傚湪涓€鑸儏鍐典笅锛岃蹇収鐢?ALSA 鏍稿績鑾峰彇锛屼絾鐗瑰畾纭欢鍙兘鍏峰鍚屾鑳藉姏锛屾垨鑰呯浉鍙嶅湴锛屽彧鑳藉欢杩熶竴娈垫椂闂村悗鎻愪緵姝ｇ‘鐨勪及璁″€笺€傚湪鍚庝袱绉嶆儏鍐典笅锛屽簳灞傞┍鍔ㄨ礋璐ｅ湪鏈€鍚堥€傘€佹渶绮剧‘鐨勬椂鍒绘洿鏂?trigger_tstamp銆傚簲鐢ㄧ▼搴忎笉搴斾粎渚濊禆绗竴涓?trigger_tstamp锛岃€屽簲鍦ㄩ┍鍔ㄥ甫鏈夊欢杩熷湴鎻愪緵缁忚繃鎻愮偧鐨勪及璁″€兼椂锛屾洿鏂板叾鍐呴儴璁＄畻銆?

- tstamp锛堟椂闂存埑锛夋槸涓婁竴娆′簨浠舵垨搴旂敤绋嬪簭鏌ヨ鏈熼棿鏇存柊鐨勫綋鍓嶇郴缁熸椂闂存埑銆?
  宸€硷紙tstamp - trigger_tstamp锛夊畾涔変簡缁忚繃鐨勬椂闂淬€?

ALSA API 鎻愪緵涓ゆ潯鍩烘湰淇℃伅锛歛vail锛堝彲鐢ㄧ┖闂达級鍜?delay锛堝欢杩燂級锛屽畠浠笌瑙﹀彂鏃堕棿鎴冲拰褰撳墠绯荤粺鏃堕棿鎴崇浉缁撳悎锛屼娇搴旂敤绋嬪簭鑳藉璺熻釜鐜舰缂撳啿鍖虹殑鈥滃～鍏呭害鈥濅互鍙婂凡鎺掗槦鏍锋湰鐨勬暟閲忋€?

浣跨敤杩欎簺涓嶅悓鎸囬拡鍜屾椂闂翠俊鎭殑鏂瑰紡鍙栧喅浜庡簲鐢ㄧ▼搴忕殑闇€姹傦細

- `avail` 鎶ュ憡鐜舰缂撳啿鍖轰腑杩樺彲浠ュ啓鍏ュ灏戞暟鎹?
- `delay` 鎶ュ憡鍦ㄦ挱鏀惧畬鎵€鏈夊凡鎺掗槦鏍锋湰鍚庯紝鍚埌涓€涓柊鏍锋湰鎵€闇€鐨勬椂闂淬€?

褰撳惎鐢ㄦ椂闂存埑鏃讹紝avail/delay 淇℃伅浼氶殢绯荤粺鏃堕棿蹇収涓€璧锋姤鍛娿€傚簲鐢ㄧ▼搴忓彲浠ヤ粠 `CLOCK_REALTIME`锛堝寘鍚?NTP 鏍℃锛屽寘鎷洖閫€锛夈€乣CLOCK_MONOTONIC`锛堝寘鍚?NTP 鏍℃浣嗕粠涓嶅洖閫€锛夈€乣CLOCK_MONOTIC_RAW`锛堜笉鍖呭惈 NTP 鏍℃锛変腑閫夋嫨锛屽苟閫氳繃 sw_params 鍔ㄦ€佹洿鏀规ā寮忋€?


ALSA API 杩樻彁渚?audio_tstamp锛堥煶棰戞椂闂存埑锛夛紝瀹冨弽鏄犵敱闊抽纭欢涓嶅悓缁勪欢娴嬪緱鐨勬椂闂存祦閫濄€傜敤 ascii 绀烘剰鍥捐〃绀哄涓嬶紙浠ユ挱鏀句负渚嬶級锛?
```

  --------------------------------------------------------------> time
    ^               ^              ^                ^           ^
    |               |              |                |           |
   analog         link            dma              app       FullBuffer
   time           time           time              time        time
    |               |              |                |           |
    |< codec delay >|<--hw delay-->|<queued samples>|<---avail->|
    |<----------------- delay---------------------->|           |
                                   |<----ring buffer length---->|


```
妯℃嫙鏃堕棿锛坅nalog time锛夊湪鎾斁鐨勬渶鍚庝竴绾ц幏鍙栵紝灏藉彲鑳芥帴杩戝疄闄呯殑鎹㈣兘鍣紙transducer锛夈€?

閾捐矾鏃堕棿锛坙ink time锛夊湪 SoC/鑺墖缁勭殑杈撳嚭澶勮幏鍙栵紝姝ゆ椂鏍锋湰姝ｈ鎺ㄩ€佸埌閾捐矾涓娿€傚鏋滅‖浠舵敮鎸侊紝閾捐矾鏃堕棿鍙互閫氳繃鏍锋湰璁℃暟鍣ㄦ垨澧欎笂鏃堕挓锛堜緥濡?HDAudio 鐨?24MHz 鏃堕挓锛屾垨缃戠粶鍖栨柟妗堢殑 PTP 鏃堕挓锛夌洿鎺ユ祴閲忥紝涔熷彲浠ラ€氳繃闂存帴鏂瑰紡浼拌锛堜緥濡備娇鐢?USB 涓殑甯ц鏁板櫒锛夈€?

DMA 鏃堕棿閫氳繃璁℃暟鍣ㄦ祴閲忊€斺€旂敱浜?DMA 浼犺緭鐨勭獊鍙戠壒鎬э紝瀹冮€氬父鏄墍鏈夋祴閲忎腑鏈€涓嶅彲闈犵殑銆?

搴旂敤鏃堕棿锛坅pp time锛夊搴斿簲鐢ㄧ▼搴忓啓鍏ョ幆褰㈢紦鍐插尯鍚庢墍璺熻釜鐨勬椂闂淬€?

搴旂敤绋嬪簭鍙互鏌ヨ纭欢鑳藉姏锛岄€氳繃閫夋嫨 audio_tstamp_config 瀛楁涓殑鐩稿叧璁剧疆鏉ュ畾涔夊笇鏈涙姤鍛婄殑闊抽鏃堕棿锛屼粠鑰屼及绠楁椂闂存埑鐨勭簿搴︺€傚畠杩樺彲浠ヨ姹傚湪娴嬮噺涓寘鍚埌妯℃嫙绔殑寤惰繜銆傚湪鎻愪緵宓屽叆寮?DSP 鐨勫钩鍙颁笂锛岀洿鎺ヨ闂摼璺椂闂撮潪甯告湁鎰忎箟锛涗娇鐢ㄤ笓鐢ㄧ‖浠剁洿鎺ユ祴閲忛摼璺椂闂达紙鍙兘涓庣郴缁熸椂闂村悓姝ワ級锛屽氨鏃犻渶鍐嶈窡韪唴閮?DSP 鐨勫鐞嗘椂闂村拰寤惰繜銆?

濡傛灉搴旂敤绋嬪簭璇锋眰鐨勯煶棰戞椂闂存埑鍦ㄧ‖浠?搴曞眰椹卞姩涓笉鍙楁敮鎸侊紝鍒欒绫诲瀷浼氳瑕嗙洊涓?DEFAULT锛屾椂闂存埑灏嗗熀浜?hw_pointer 鍊兼姤鍛?DMA 鏃堕棿銆?

涓轰簡涓庢湭鎻愪緵鏃堕棿鎴抽€夋嫨鐨勬棭鏈熷疄鐜颁繚鎸佸悜鍚庡吋瀹癸紝褰撲娇鐢ㄩ浂鍊肩殑 COMPAT 鏃堕棿鎴崇被鍨嬫椂锛屾挱鏀炬祦鐨勭粨鏋滃皢榛樿浣跨敤 HDAudio 澧欎笂鏃堕挓锛屽叾浠栨墍鏈夋儏鍐典笅鍒欎娇鐢?DMA 鏃堕棿锛坔w_ptr锛夈€?

闊抽鏃堕棿鎴崇殑绮惧害鍙互杩斿洖缁欑敤鎴风┖闂达紝浠ヤ究鍋氬嚭閫傚綋鐨勫喅绛栵細

- 瀵逛簬 DMA 鏃堕棿锛堥粯璁わ級锛屼紶杈撶殑绮掑害鍙互浠庢洿鏂颁箣闂寸殑闂撮殧鎺ㄦ柇鍑烘潵锛岃繘鑰屾彁渚涘叧浜庡簲鐢ㄧ▼搴忔寚閽堝彲浠ュ畨鍏ㄥ洖閫€澶氬皯鐨勪俊鎭€?

- 閾捐矾鏃堕棿鍙敤浜庨€氳繃 (tstamp-trigger_tstamp)/audio_tstamp 姣斿€兼潵璺熻釜闊抽鏃堕棿涓庣郴缁熸椂闂翠箣闂寸殑闀挎湡婕傜Щ锛屽叾绮惧害鏈夊姪浜庣‘瀹氶渶瑕佸灏戝钩婊?浣庨€氭护娉€傞摼璺椂闂村彲浠ュ湪鍚姩鏃跺浣嶏紝涔熷彲浠ユ寜鍘熸牱鎶ュ憡锛堝悗鑰呭浜庢瘮杈冧笉鍚屾祦鐨勮繘搴﹀緢鏈夌敤鈥斺€斾絾鍙兘瑕佹眰澧欎笂鏃堕挓濮嬬粓杩愯锛屼笖鍦ㄧ┖闂叉湡闂翠笉浼氬洖缁曪級銆傚鏋滅‖浠舵敮鎸侊紝缁濆閾捐矾鏃堕棿涔熷彲浠ョ敤浜庡畾涔夌簿纭殑鍚姩鏃堕棿锛堣ˉ涓佸紑鍙戜腑锛夈€?

- 鍦ㄩ煶棰戞椂闂存埑涓寘鍚欢杩熷彲鑳戒細鍙嶇洿瑙夊湴涓嶄細鎻愰珮鏃堕棿鎴崇殑绮惧害锛屼緥濡傦紝濡傛灉缂栬В鐮佸櫒鍖呭惈鍙彉寤惰繜鐨?DSP 澶勭悊锛屾垨鑰呯敱涓€涓茬‖浠剁粍浠剁粍鎴愶紝鍒欏欢杩熼€氬父鏃犳硶绮剧‘鑾风煡銆?

绮惧害浠ョ撼绉掍负鍗曚綅鎶ュ憡锛堜娇鐢ㄤ竴涓棤绗﹀彿 32 浣嶅瓧锛夛紝鏈€澶х簿搴︿负 4.29 绉掞紝瀵归煶棰戝簲鐢ㄦ潵璇寸话缁版湁浣欌€︹€?

鐢变簬鏃堕棿鎴抽渶姹傜殑澶氭牱鎬э紝鍗充究鏄浜庡崟涓簲鐢ㄧ▼搴忥紝audio_tstamp_config 涔熷彲浠ュ姩鎬佹洿鏀广€傚湪 `STATUS` ioctl 涓紝鍙傛暟鏄彧璇荤殑锛屼笉鍏佽浠讳綍搴旂敤绋嬪簭閫夋嫨銆備负浜嗗湪涓嶅奖鍝嶉仐鐣欏簲鐢ㄧ▼搴忕殑鎯呭喌涓嬭閬胯繖涓€闄愬埗锛屽紩鍏ヤ簡涓€涓柊鐨?`STATUS_EXT` ioctl锛屽叾鍙傛暟涓哄彲璇诲啓銆侫LSA-lib 灏嗚淇敼浠ヤ娇鐢?`STATUS_EXT`锛屼粠鑰屽疄闄呬笂寮冪敤 `STATUS`銆?

ALSA API 涓€娆″彧鍏佽鎶ュ憡鍗曚釜闊抽鏃堕棿鎴炽€傝繖鏄竴涓湁鎰忕殑璁捐鍐冲畾锛屽洜涓轰粠纭欢瀵勫瓨鍣ㄦ垨 IPC 璇诲彇闊抽鏃堕棿鎴抽渶瑕佹椂闂达紝璇诲彇鐨勬椂闂存埑瓒婂锛屽悎骞舵祴閲忕殑绮惧害灏辫秺浣庛€備负閬垮厤浠讳綍瑙ｉ噴涓婄殑闂锛屽彧鎶ュ憡涓€涓紙绯荤粺锛岄煶棰戯級鏃堕棿鎴炽€傞渶瑕佷笉鍚屾椂闂存埑鐨勫簲鐢ㄧ▼搴忓繀椤诲彂鍑哄娆℃煡璇㈠苟瀵圭粨鏋滆繘琛屾彃鍊笺€?

鍦ㄦ煇浜涚壒瀹氱‖浠堕厤缃腑锛岀郴缁熸椂闂存埑鐢卞簳灞傞煶棰戝瓙绯荤粺閿佸瓨锛屽苟灏嗕俊鎭彁渚涘洖椹卞姩銆傜敱浜庝笌纭欢閫氫俊鍙兘瀛樺湪寤惰繜锛屽瓨鍦ㄤ笌 avail 鍜?delay 淇℃伅閿欎綅鐨勯闄┿€備负纭繚搴旂敤绋嬪簭涓嶈娣锋穯锛屽湪 snd_pcm_status 缁撴瀯浣撲腑澧炲姞浜嗕竴涓?driver_timestamp 瀛楁锛涜鏃堕棿鎴虫樉绀轰簡椹卞姩鍦ㄤ粠 `STATUS` 鍜?`STATUS_EXT` ioctl 杩斿洖涔嬪墠灏嗕俊鎭眹鎬诲湪涓€璧风殑鏃堕棿銆傚湪澶у鏁版儏鍐典笅锛岃繖涓?driver_timestamp 涓庡父瑙勭殑绯荤粺 tstamp 鐩稿悓銆?

浣跨敤 HDAudio 鐨勬椂闂存埑绀轰緥锛?

1. DMA 鏃堕棿鎴筹紝涓嶈ˉ鍋?DMA+妯℃嫙寤惰繜```

  $ ./audio_time  -p --ts_type=1
  playback: systime: 341121338 nsec, audio time 342000000 nsec, 	systime delta -878662
  playback: systime: 426236663 nsec, audio time 427187500 nsec, 	systime delta -950837
  playback: systime: 597080580 nsec, audio time 598000000 nsec, 	systime delta -919420
  playback: systime: 682059782 nsec, audio time 683020833 nsec, 	systime delta -961051
  playback: systime: 852896415 nsec, audio time 853854166 nsec, 	systime delta -957751
  playback: systime: 937903344 nsec, audio time 938854166 nsec, 	systime delta -950822

```
2. DMA 鏃堕棿鎴筹紝琛ュ伩 DMA+妯℃嫙寤惰繜
```

  $ ./audio_time  -p --ts_type=1 -d
  playback: systime: 341053347 nsec, audio time 341062500 nsec, 	systime delta -9153
  playback: systime: 426072447 nsec, audio time 426062500 nsec, 	systime delta 9947
  playback: systime: 596899518 nsec, audio time 596895833 nsec, 	systime delta 3685
  playback: systime: 681915317 nsec, audio time 681916666 nsec, 	systime delta -1349
  playback: systime: 852741306 nsec, audio time 852750000 nsec, 	systime delta -8694

```
3. 閾捐矾鏃堕棿鎴筹紝琛ュ伩 DMA+妯℃嫙寤惰繜
```

  $ ./audio_time  -p --ts_type=2 -d
  playback: systime: 341060004 nsec, audio time 341062791 nsec, 	systime delta -2787
  playback: systime: 426242074 nsec, audio time 426244875 nsec, 	systime delta -2801
  playback: systime: 597080992 nsec, audio time 597084583 nsec, 	systime delta -3591
  playback: systime: 682084512 nsec, audio time 682088291 nsec, 	systime delta -3779
  playback: systime: 852936229 nsec, audio time 852940916 nsec, 	systime delta -4687
  playback: systime: 938107562 nsec, audio time 938112708 nsec, 	systime delta -5146

```
绀轰緥 1 琛ㄦ槑锛孌MA 绾у埆鐨勬椂闂存埑姣斿疄闄呮挱鏀炬椂闂磋秴鍓嶇害 1ms锛堥『渚胯涓€鍙ワ紝杩欑被娴嬮噺鏈夊姪浜庡畾涔夊洖閫€淇濇姢鎺柦锛夈€傚湪绀轰緥 2 涓ˉ鍋?DMA-閾捐矾寤惰繜鏈夊姪浜庢秷闄ょ‖浠剁紦鍐诧紝浣嗕俊鎭粛鐒堕潪甯告姈鍔紝璇樊鏈€澶氬彲杈句竴涓牱鏈€傚湪绀轰緥 3 涓紝鏃堕棿鎴虫槸鐢ㄩ摼璺涓婃椂閽熸祴閲忕殑锛屾樉绀哄嚭鍗曡皟鐨勮涓哄拰鏇翠綆鐨勭鏁ｅ害銆?

绀轰緥 3 鍜?4 閽堝 USB 闊抽绫汇€傜ず渚?3 鐢变簬缂撳啿鑰屾樉绀哄嚭闊抽鏃堕棿涓庣郴缁熸椂闂翠箣闂村瓨鍦ㄨ緝澶х殑鍋忕Щ銆傜ず渚?4 灞曠ず浜嗚ˉ鍋垮欢杩熷浣曟毚闇插嚭 1ms 鐨勭簿搴︼紙寰楃泭浜庨┍鍔ㄤ娇鐢ㄤ簡甯ц鏁板櫒锛夈€?

绀轰緥 3锛欴MA 鏃堕棿鎴筹紝涓嶈ˉ鍋垮欢杩燂紝delta 绾?5ms
```

  $ ./audio_time -p -Dhw:1 -t1
  playback: systime: 120174019 nsec, audio time 125000000 nsec, 	systime delta -4825981
  playback: systime: 245041136 nsec, audio time 250000000 nsec, 	systime delta -4958864
  playback: systime: 370106088 nsec, audio time 375000000 nsec, 	systime delta -4893912
  playback: systime: 495040065 nsec, audio time 500000000 nsec, 	systime delta -4959935
  playback: systime: 620038179 nsec, audio time 625000000 nsec, 	systime delta -4961821
  playback: systime: 745087741 nsec, audio time 750000000 nsec, 	systime delta -4912259
  playback: systime: 870037336 nsec, audio time 875000000 nsec, 	systime delta -4962664

```
绀轰緥 4锛欴MA 鏃堕棿鎴筹紝琛ュ伩寤惰繜锛屽欢杩熺害 1ms
```

  $ ./audio_time -p -Dhw:1 -t1 -d
  playback: systime: 120190520 nsec, audio time 120000000 nsec, 	systime delta 190520
  playback: systime: 245036740 nsec, audio time 244000000 nsec, 	systime delta 1036740
  playback: systime: 370034081 nsec, audio time 369000000 nsec, 	systime delta 1034081
  playback: systime: 495159907 nsec, audio time 494000000 nsec, 	systime delta 1159907
  playback: systime: 620098824 nsec, audio time 619000000 nsec, 	systime delta 1098824
  playback: systime: 745031847 nsec, audio time 744000000 nsec, 	systime delta 1031847

