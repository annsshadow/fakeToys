## 寰呭姙浜嬮」锛圱ODO LIST锛?

灏氭湭瀹炵幇鐨勫嚱鏁板涓嬶紙褰撳墠鐢辩紪璇戝櫒鍙戝嚭锛屽苟鐢?libc 涓殑渚嬬▼澶勭悊锛夈€傝繖浜涘嚱鏁板凡鍦?FPA11 纭欢涓婂疄鐜帮紝鐢辨诞鐐规敮鎸佷唬鐮佸鐞嗐€傛湭鏉ョ増鏈皢瀹炵幇鍏朵綑閮ㄥ垎銆?

```
  POW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - power
  RPW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse power
  POL{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - polar angle (arctan2)

  LOG{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base 10
  LGN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base e
  EXP{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - exponent
  SIN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - sine
  COS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - cosine
  TAN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - tangent
  ASN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arcsine
  ACS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arccosine
  ATN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arctangent

```

鍙互閫氳繃鍑犵閫斿緞鏉ュ疄鐜拌繖浜涜秴瓒婂嚱鏁般€傚叾涓竴绉嶆柟娉曟槸浣跨敤鍩轰簬鏌ユ壘琛ㄧ殑绮剧‘鏂规硶鏉ョ紪鍐欒繖浜涗緥绋嬨€傛垜鏈?S. Gal 鍦?IBM 浠ヨ壊鍒楁捣娉曪紙Haifa锛夌爺绌跺疄楠屽鍙戣〃鐨勫嚑绡囪鏂囷紝浼间箮鏈夋湜鍦ㄥ悎鐞嗙殑閫熷害涓嬭揪鍒版瀬楂樼殑绮惧害锛堢害 99.8%锛夈€傝鏂规硶浣跨敤 GLIBC 鐨勮秴瓒婂嚱鏁般€?

鍙︿竴绉嶉€斿緞鏄垜鐭ヤ箣鐢氬皯鐨?CORDIC锛圕oordinate Rotation Digital Computer锛屽潗鏍囨棆杞暟瀛楄绠楁満锛夋柟娉曪紝瀹冮€氳繃绉讳綅浠ュ強灏戦噺鐨勪箻娉曞拰闄ゆ硶鏉ヨ绠楄秴瓒婂嚱鏁般€侫RM 鍦ㄧЩ浣嶄笌鍔犳硶鏂归潰琛ㄧ幇鍑鸿壊锛屽洜姝よ鏂规硶鍙兘寰堟湁鍓嶆櫙锛屼絾闇€瑕佹洿澶氱殑鐮旂┒鏉ョ‘瀹氬叾鍙鎬с€?

### 鑸嶅叆妯″紡锛圧ounding Methods锛?

IEEE 鏍囧噯瀹氫箟浜?4 绉嶈垗鍏ユā寮忋€傞粯璁ゆ槸鍚戞渶鎺ヨ繎鍊艰垗鍏ワ紙round to nearest锛夛紝涔熷厑璁稿悜姝ｆ棤绌枫€佽礋鏃犵┓浠ュ強鍚戦浂鑸嶅叆銆傝澶氫綋绯荤粨鏋勫厑璁搁€氳繃淇敼鎺у埗瀵勫瓨鍣ㄤ腑鐨勪綅鏉ユ寚瀹氳垗鍏ユā寮忋€侫RM FPA11 浣撶郴缁撴瀯鍒欓€氳繃涓€鏉′笓闂ㄧ殑鎸囦护鏉ユ敼鍙樿垗鍏ユā寮忋€?

杩欎竴鐐逛娇寰楃Щ妞嶅熀鍑嗘祴璇曞彉寰楀洶闅俱€傛湁鍙兘鍦ㄦā鎷熷櫒涓紩鍏ョ浉搴旂殑鑳藉姏銆侳PCR 涓寘鍚弿杩拌垗鍏ユā寮忕殑浣嶃€傛ā鎷熷櫒鍙互淇敼杩欎簺浣嶃€佹鏌ユ爣蹇楋紝骞跺己鍒跺拷鐣ユ寚浠や腑鎸囧畾鐨勮垗鍏ユā寮忥紝杞€屼娇鐢?FPCR 涓綅鎵€鎸囧畾鐨勬ā寮忋€?

杩欓渶瑕佷竴绉嶈幏鍙?璁剧疆鏍囧織涓?FPCR 涓綅鐨勬柟娉曘€傝繖闇€瑕佸唴鏍歌皟鐢?ArmLinux 鐨?WFC/RFC 鐩戠鑰咃紙supervisor锛夋寚浠ゃ€傚鏋滄湁浜烘湁鎯虫硶鎴栨剰瑙侊紝鎴戝笇鏈涜兘鍚惉銆?

娉ㄦ剰锛?

浠ヤ笅鍐呭鎽樿嚜 ARM 娴偣鏂囨。锛堢壒鍒槸 Acorn FPE锛夛紝浣嗘湁鎵€鍒犺妭锛?

娴偣鎺у埗瀵勫瓨鍣紙FPCR锛夊湪鐜版湁瀹炵幇涓苟涓嶅瓨鍦細瀹冪敤浜庢帶鍒剁‖浠朵互鐗瑰畾鏂瑰紡瀹炵幇鈥斺€斾緥濡傜鐢ㄦ诞鐐圭郴缁熴€傚湪鐢ㄦ埛妯″紡涓嬶紝ARM 涓嶅厑璁镐娇鐢ㄨ瀵勫瓨鍣紙鍥犱负淇濈暀缁欏疄鐜版柟鏇存敼锛夛紝鑰?WFC/RFC 鎸囦护鑻ュ湪鐢ㄦ埛妯″紡涓嬪皾璇曟墽琛屼細瑙﹀彂寮傚父锛坱rap锛夈€?

鍥犳锛岀瓟妗堟槸锛氬彲浠ワ紝浣嗚繍琛屽畠鐨勯闄╁緢楂橈紝鍥犱负褰撶‖浠舵诞鐐规ā鎷熷嚭鐜版椂锛屽畠浼氬彉寰楀绔嬫棤鎻淬€?

-- Russell.
