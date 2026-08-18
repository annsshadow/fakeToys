
## 濡備綍鍦?mac80211 涓娇鐢ㄥ寘娉ㄥ叆


mac80211 鐜板湪鍏佽浠庣敤鎴风┖闂村悜浠绘剰 Monitor Mode锛堢洃鍚ā寮忥級鎺ュ彛娉ㄥ叆浠绘剰鏁版嵁鍖呫€備綘娉ㄥ叆鐨?鏁版嵁鍖呴渶瑕佹寜濡備笅鏂瑰紡缁勭粐锛?
```
 [ radiotap header  ]
 [ ieee80211 header ]
 [ payload ]

```
radiotap 鏍煎紡鍦?./Documentation/networking/radiotap-headers.rst
涓湁璁ㄨ銆?
灏界鐩墠瀹氫箟浜嗕紬澶?radiotap 鍙傛暟锛屼絾鍏朵腑澶ч儴鍒嗗彧鍦ㄦ帴鏀剁殑鏁版嵁鍖呬笂鎵嶆湁鎰忎箟銆備互涓嬩俊鎭細浠?radiotap 澶撮儴瑙ｆ瀽鍑烘潵锛屽苟鐢ㄤ簬鎺у埗娉ㄥ叆杩囩▼锛?
 - IEEE80211_RADIOTAP_FLAGS

   =========================  ===========================================
   IEEE80211_RADIOTAP_F_FCS   FCS 灏嗚绉婚櫎骞堕噸鏂拌绠?   IEEE80211_RADIOTAP_F_WEP   濡傛灉瀛樺湪瀵嗛挜锛屽皢瀵瑰抚杩涜鍔犲瘑
   IEEE80211_RADIOTAP_F_FRAG  濡傛灉甯ч暱浜庡綋鍓嶇殑鍒嗙墖闃堝€硷紝灏嗗鍏惰繘琛屽垎鐗?   =========================  ===========================================

 - IEEE80211_RADIOTAP_TX_FLAGS

   =============================  ========================================
   IEEE80211_RADIOTAP_F_TX_NOACK  鍗充娇鏄潪骞挎挱鐨勫崟鎾抚锛屽彂閫佹椂涔熶笉绛夊緟 ACK
   =============================  ========================================

 - IEEE80211_RADIOTAP_RATE

   浼犺緭鎵€鐢ㄧ殑浼犵粺閫熺巼锛堜粎閫傜敤浜庢病鏈夎嚜韬€熺巼鎺у埗鐨勮澶囷級

 - IEEE80211_RADIOTAP_MCS

   浼犺緭鎵€鐢ㄧ殑 HT 閫熺巼锛堜粎閫傜敤浜庢病鏈夎嚜韬€熺巼鎺у埗鐨勮澶囷級銆傚悓鏃朵篃浼氳В鏋愰儴鍒嗘爣蹇?
   ============================  ========================
   IEEE80211_RADIOTAP_MCS_SGI    浣跨敤鐭繚鎶ら棿闅?   IEEE80211_RADIOTAP_MCS_BW_40  浠?HT40 妯″紡鍙戦€?   ============================  ========================

 - IEEE80211_RADIOTAP_DATA_RETRIES

   褰撲娇鐢ㄤ簡 IEEE80211_RADIOTAP_RATE 鎴?IEEE80211_RADIOTAP_MCS 鏃剁殑閲嶈瘯娆℃暟

 - IEEE80211_RADIOTAP_VHT

   浼犺緭鎵€鐢ㄧ殑 VHT mcs 浠ュ強娴佺殑鏁伴噺锛堜粎閫傜敤浜庢病鏈夎嚜韬€熺巼鎺у埗鐨勮澶囷級銆傚悓鏃朵篃浼氳В鏋愬叾浠栧瓧娈?
   flags 瀛楁
	IEEE80211_RADIOTAP_VHT_FLAG_SGI锛氫娇鐢ㄧ煭淇濇姢闂撮殧

   bandwidth 瀛楁
 - 1锛氫互 40MHz 淇￠亾瀹藉害鍙戦€? - 4锛氫互 80MHz 淇￠亾瀹藉害鍙戦€? - 11锛氫互 160MHz 淇￠亾瀹藉害鍙戦€?
娉ㄥ叆浠ｇ爜杩樺彲浠ヨ烦杩囨墍鏈夊叾瀹冨綋鍓嶅凡瀹氫箟鐨?radiotap 瀛楁锛屼粠鑰屼究浜庣洿鎺ラ噸鏀炬墍鎹曡幏鐨?radiotap
澶撮儴銆?
```

	0x00, 0x00, // <-- radiotap 鐗堟湰
	0x0b, 0x00, // <- radiotap 澶撮儴闀垮害
	0x04, 0x0c, 0x00, 0x00, // <-- 浣嶅浘
	0x6c, // <-- 閫熺巼
	0x0c, //<-- 鍙戝皠鍔熺巼
	0x01 //<-- 澶╃嚎

```
闅忓悗绱ц窡鐫€ ieee80211 澶撮儴锛屼緥濡傦細

```

	0x08, 0x01, 0x00, 0x00,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0x13, 0x22, 0x33, 0x44, 0x55, 0x66,
	0x13, 0x22, 0x33, 0x44, 0x55, 0x66,
	0x10, 0x86

```
鏈€鍚庢槸杞借嵎銆?
鍦ㄦ瀯閫犲ソ鏁版嵁鍖呭唴瀹瑰悗锛岄€氳繃 send() 灏嗗叾鍙戦€佸埌澶勪簬 Monitor 妯″紡鐨?mac80211 閫昏緫鎺ュ彛鍗冲彲銆?涔熷彲浠ヤ娇鐢?Libpcap锛堣繖姣旇嚜琛屽皢濂楁帴瀛楃粦瀹氬埌姝ｇ‘鎺ュ彛瑕佸鏄擄級锛?
```

	ppcap = pcap_open_live(szInterfaceName, 800, 1, 20, szErrbuf);
	...
	r = pcap_inject(ppcap, u8aSendBuffer, nLength);

```
浣犺繕鍙互鍦ㄦ澶勬壘鍒颁竴涓畬鏁存敞鍏ュ簲鐢ㄧ殑閾炬帴锛?
https://wireless.wiki.kernel.org/en/users/Documentation/packetspammer

Andy Green <andy@warmcat.com>
