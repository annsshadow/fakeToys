## 浣滆€?

### 鍘熷浣滆€?

Steve French (smfrench@gmail.com, sfrench@samba.org)

浣滆€呭笇鏈涜〃杈句粬鐨勬劅婵€涓庤阿鎰忥細鎰熻阿 Andrew Tridgell锛圫amba 鍥㈤槦锛夊叧浜?SMB/CIFS VFS 鏀硅繘鐨勬棭鏈熷缓璁€傛劅璋?IBM 缁欎簣鎴戞椂闂翠笌娴嬭瘯璧勬簮鏉ユ帹杩涙椤圭洰锛屾劅璋?IBM 鐨?Jim McDonough锛堜互鍙?Samba 鍥㈤槦锛夌殑甯姪锛屾劅璋?IBM Linux JFS 鍥㈤槦瀵硅澶氭繁濂?Linux 鏂囦欢绯荤粺鐗规€х殑璁茶В銆係amba 鍥㈤槦鐨?Jeremy Allison 鍦ㄥ畬鎴愬師濮?CIFS Unix 鎵╁睍鐨勬湇鍔＄浠ュ強灏嗚緝鏂扮殑 CIFS POSIX 鎵╁睍鐨勯儴鍒嗗唴瀹瑰鏌ュ苟瀹炵幇鍒?Samba 3 鏂囦欢鏈嶅姟鍣ㄤ腑鍋氬嚭浜嗗疂璐靛伐浣溿€傛劅璋?IBM Rochester 鐨?Dave Boutcher锛圤S/400 smb/cifs 鏂囦欢绯荤粺瀹㈡埛绔殑浣滆€咃級澶氬勾鍓嶇殑璇佹槑锛氬湪绫?Unix 鎿嶄綔绯荤粺涓婂彲浠ュ疄鐜伴潪甯稿ソ鐨?smb/cifs 瀹㈡埛绔€俈olker Lendecke銆丄ndrew Tridgell銆乁rban Widmark銆丣ohn Newbigin 鍙婂叾浠栦汉鎰熻阿浠栦滑鍦?Linux smbfs 妯″潡涓婄殑宸ヤ綔銆傛劅璋㈠瓨鍌ㄧ綉缁滃伐涓氬崗浼氾紙SNIA锛塁IFS 鎶€鏈伐浣滅粍鍏朵粬鎴愬憳鍦ㄨ鑼冭繖涓€楂樺害澶嶆潅鍗忚涓婄殑宸ヤ綔锛屾渶鍚庢劅璋?Samba 鍥㈤槦鐨勬妧鏈缓璁笌榧撳姳銆?
### 琛ヤ竵璐＄尞鑰?

- Zwane Mwaikambo
- Andi Kleen
- Amrut Joshi
- Shobhit Dayal
- Sergey Vlasov
- Richard Hughes
- Yury Umanets
- Mark Hamzy锛堥儴鍒嗘棭鏈?cifs IPv6 宸ヤ綔锛?- Domen Puncer
- Jesper Juhl锛堝挨鍏惰础鐚簡澶ч噺绌虹櫧/鏍煎紡娓呯悊锛?- Vince Negri 鍜?Dave Stahl锛堝彂鐜颁簡閲嶈鐨勭紦瀛?bug锛?- Adrian Bunk锛坘calloc 娓呯悊锛?- Miklos Szeredi
- Kazeon 鍥㈤槦锛屽悇绉嶄慨澶嶏紝灏ゅ叾鏄?2.4 鐗堟湰銆?- Asser Ferno锛圕hange Notify 鏀寔锛?- Shaggy锛圖ave Kleikamp锛夛紝鏃犳暟灏忕殑鏂囦欢绯荤粺寤鸿鍜屼竴浜涜壇濂界殑娓呯悊
- Gunter Kukkukk锛堥拡瀵硅€佹棫鏈嶅姟鍣ㄦ敮鎸佺殑娴嬭瘯涓庡缓璁級
- Igor Mammedov锛圖FS 鏀寔锛?- Jeff Layton锛堣澶氥€佽澶氫慨澶嶏紝浠ュ強 cifs Kerberos 浠ｇ爜鐨勫嚭鑹插伐浣滐級
- Scott Lovenberg
- Pavel Shilovsky锛堝湪娣诲姞 SMB2 鏀寔浠ュ強鍚勭 SMB3 鐗规€т笂鐨勫嚭鑹插伐浣滐級
- Aurelien Aptel锛圖FS SMB3 宸ヤ綔浠ュ強涓€浜涘叧閿?bug 淇锛?- Ronnie Sahlberg锛圫MB3 xattr 宸ヤ綔銆乥ug 淇浠ュ強澶ч噺鍦ㄥ鍚堬紙compounding锛変笂鐨勫嚭鑹插伐浣滐級
- Shirish Pargaonkar锛堝骞存潵璁稿 ACL 琛ヤ竵锛?- Sachin Prabhu锛堣澶?bug 淇锛屽寘鎷噸杩炪€佸鍒跺嵏杞藉拰瀹夊叏鎬э級
- Paulo Alcantara锛堝湪 DFS 浠ュ強浠?SMB3 鍚姩涓婄殑鍑鸿壊宸ヤ綔锛?- Long Li锛堝湪 RDMA銆丼MB Direct 涓婄殑涓€浜涘嚭鑹插伐浣滐級


### 娴嬭瘯鐢ㄤ緥涓庣己闄锋姤鍛婅础鐚€?

鎰熻阿绀惧尯涓彁浜よ缁嗙己闄锋姤鍛婂苟璋冭瘯鎵€鍙戠幇闂鐨勪汉锛欽ochen Dolze銆丏avid Blaine銆丷ene Scharfe銆丮artin Josefsson銆丄lexander Wild銆丄nthony Liguori銆丩ars Muller銆乁rban Widmark銆丮assimiliano Ferrero銆丠oward Owen銆丱laf Kirch銆並ieron Briggs銆丯ick Millington 绛夈€備篃鐗瑰埆鎻愬強 Stanford Checker锛圫WAT锛夛紝瀹冩寚鍑轰簡閿欒璺緞涓殑璁稿灏?bug銆侫l Viro 鍜?Dave Miller 涔熺粰鍑轰簡瀹濊吹鐨勫缓璁€?
骞舵劅璋?IBM LTC 鍜?Power 娴嬭瘯鍥㈤槦锛屼互鍙?SuSE銆丆itrix 鍜?RedHat 鐨勬祴璇曚汉鍛樺湪浼樼鐨勫帇鍔涙祴璇曡繍琛屼腑鍙戠幇浜嗗涓?bug銆?