## SNMP 璁℃暟鍣?


鏈枃妗ｈВ閲?SNMP 璁℃暟鍣ㄧ殑鍚箟銆?

## 閫氱敤 IPv4 璁℃暟鍣?

鎵€鏈夌 4 灞傛姤鏂囧拰 ICMP 鎶ユ枃閮戒細鏀瑰彉杩欎簺璁℃暟鍣紝浣嗙 2 灞傛姤鏂囷紙渚嬪 STP锛?
鎴?ARP 鎶ユ枃涓嶄細鏀瑰彉瀹冧滑銆?

- IpInReceives

瀹氫箟浜?`RFC1213 ipInReceives`_


IP 灞傛敹鍒扮殑鎶ユ枃鏁伴噺銆傚畠鍦?ip_rcv 鍑芥暟寮€澶村閫掑锛屽缁堜笌 IpExtInOctets
涓€璧锋洿鏂般€傚嵆浣挎姤鏂囧悗鏉ヨ涓㈠純锛堜緥濡傜敱浜?IP 澶撮儴鏃犳晥鎴栨牎楠屽拰閿欒绛夛級锛?
瀹冧粛浼氬鍔犮€傚畠琛ㄧず GRO/LRO 涔嬪悗鑱氬悎娈电殑鏁伴噺銆?

- IpInDelivers

瀹氫箟浜?`RFC1213 ipInDelivers`_


鎶曢€掔粰涓婂眰鍗忚鐨勬姤鏂囨暟閲忋€備緥濡?TCP銆乁DP銆両CMP 绛夈€傚鏋滄病鏈変汉鍦?raw
socket 涓婄洃鍚紝鍒欏彧鏈夊唴鏍告敮鎸佺殑鍗忚浼氳鎶曢€掞紱濡傛灉鏈変汉鍦?raw socket 涓?
鐩戝惉锛屾墍鏈夊悎娉曠殑 IP 鎶ユ枃閮戒細琚姇閫掋€?

- IpOutRequests

瀹氫箟浜?`RFC1213 ipOutRequests`_


缁忕敱 IP 灞傚彂閫佺殑鎶ユ枃鏁伴噺锛屽寘鎷崟鎾拰澶氭挱鎶ユ枃锛屽苟涓斿缁堜笌 IpExtOutOctets
涓€璧锋洿鏂般€?

- IpExtInOctets 涓?IpExtOutOctets

瀹冧滑鏄?Linux 鍐呮牳鎵╁睍锛屾病鏈?RFC 瀹氫箟銆傝娉ㄦ剰锛孯FC1213 纭疄瀹氫箟浜?
ifInOctets 鍜?ifOutOctets锛屼絾瀹冧滑鏄笉鍚岀殑涓滆タ銆俰fInOctets 鍜?ifOutOctets
鍖呭惈 MAC 灞傚ご閮ㄥぇ灏忥紝鑰?IpExtInOctets 鍜?IpExtOutOctets 涓嶅寘鍚紝瀹冧滑
鍙寘鍚?IP 灞傚ご閮ㄥ拰 IP 灞傛暟鎹€?

- IpExtInNoECTPkts銆両pExtInECT1Pkts銆両pExtInECT0Pkts銆両pExtInCEPkts

瀹冧滑琛ㄧず鍥涚 ECN IP 鎶ユ枃鐨勬暟閲忥紝鏇村缁嗚妭璇峰弬鑰?`Explicit Congestion
Notification`_銆?


杩?4 涓鏁板櫒缁熻姣忕 ECN 鐘舵€佷笅鏀跺埌鐨勬姤鏂囨暟閲忋€傛棤璁?LRO/GRO锛屽畠浠兘
缁熻鐪熷疄鐨勫抚鏁般€傚洜姝ゅ浜庡悓涓€涓姤鏂囷紝浣犲彲鑳戒細鍙戠幇 IpInReceives 璁℃暟涓?1锛?
鑰?IpExtInNoECTPkts 璁℃暟涓?2 鎴栨洿澶氥€?

- IpInHdrErrors

瀹氫箟浜?`RFC1213 ipInHdrErrors`_. 瀹冭〃绀烘姤鏂囧洜 IP 澶撮儴閿欒鑰岃涓㈠純銆傚畠
鍙兘鍙戠敓鍦?IP 杈撳叆璺緞鍜?IP 杞彂璺緞涓€?


- IpInAddrErrors

瀹氫箟浜?`RFC1213 ipInAddrErrors`_. 瀹冧細鍦ㄤ袱绉嶆儏鍐典笅澧炲姞锛?1) IP 鍦板潃鏃犳晥銆?
(2) 鐩殑 IP 鍦板潃涓嶆槸鏈湴鍦板潃锛屼笖鏈惎鐢?IP 杞彂銆?


- IpExtInNoRoutes

璇ヨ鏁板櫒琛ㄧず褰?IP 鍗忚鏍堟敹鍒颁竴涓姤鏂囥€佷笖鏃犳硶浠庤矾鐢辫〃涓负鍏舵壘鍒拌矾鐢辨椂锛?
璇ユ姤鏂囪涓㈠純銆傚畠鍙兘鍙戠敓鍦ㄥ惎鐢ㄤ簡 IP 杞彂銆佺洰鐨?IP 鍦板潃涓嶆槸鏈湴鍦板潃銆佷笖
涓嶅瓨鍦ㄩ€氬線璇ョ洰鐨?IP 鍦板潃鐨勮矾鐢辨椂銆?

- IpInUnknownProtos

瀹氫箟浜?`RFC1213 ipInUnknownProtos`_. 濡傛灉绗?4 灞傚崗璁笉琚唴鏍告敮鎸侊紝瀹冨氨浼?
澧炲姞銆傚鏋滃簲鐢ㄧ▼搴忔鍦ㄤ娇鐢?raw socket锛屽唴鏍告€讳細灏嗘姤鏂囨姇閫掔粰 raw socket锛?
璇ヨ鏁板櫒鍒欎笉浼氬鍔犮€?


- IpExtInTruncatedPkts

瀵逛簬 IPv4 鎶ユ枃锛屽畠琛ㄧず瀹為檯鏁版嵁澶у皬灏忎簬 IPv4 澶撮儴涓殑 "Total Length" 瀛楁銆?

- IpInDiscards

瀹氫箟浜?`RFC1213 ipInDiscards`_. 瀹冭〃绀烘姤鏂囧洜鍐呮牳鍐呴儴鍘熷洜锛堜緥濡傚唴瀛樹笉瓒筹級
鍦?IP 鎺ユ敹璺緞涓涓㈠純銆?


- IpOutDiscards

瀹氫箟浜?`RFC1213 ipOutDiscards`_. 瀹冭〃绀烘姤鏂囧洜鍐呮牳鍐呴儴鍘熷洜鍦?IP 鍙戦€佽矾寰勪腑
琚涪寮冦€?


- IpOutNoRoutes

瀹氫箟浜?`RFC1213 ipOutNoRoutes`_. 瀹冭〃绀烘姤鏂囧湪 IP 鍙戦€佽矾寰勪腑琚涪寮冿紝涓?
鎵句笉鍒伴€氬線瀹冪殑璺敱銆?


## ICMP 璁℃暟鍣?

- IcmpInMsgs 涓?IcmpOutMsgs

鐢?`RFC1213 icmpInMsgs`_ 鍜?`RFC1213 icmpOutMsgs`_ 瀹氫箟


濡?RFC1213 鎵€杩帮紝杩欎袱涓鏁板櫒鍖呭惈閿欒锛屽嵆浣?ICMP 鎶ユ枃绫诲瀷鏃犳晥瀹冧滑涔熶細
澧炲姞銆侷CMP 杈撳嚭璺緞浼氭鏌?raw socket 鐨勫ご閮紝鍥犳鍗充娇 IP 澶撮儴鐢辩敤鎴锋€?
绋嬪簭鏋勯€狅紝IcmpOutMsgs 浠嶄細琚洿鏂般€?

- ICMP 鍛藉悕绫诲瀷

| 杩欎簺璁℃暟鍣ㄥ寘鍚ぇ澶氭暟甯歌鐨?ICMP 绫诲瀷锛屽畠浠槸锛?
| IcmpInDestUnreachs: `RFC1213 icmpInDestUnreachs`_
| IcmpInTimeExcds: `RFC1213 icmpInTimeExcds`_
| IcmpInParmProbs: `RFC1213 icmpInParmProbs`_
| IcmpInSrcQuenchs: `RFC1213 icmpInSrcQuenchs`_
| IcmpInRedirects: `RFC1213 icmpInRedirects`_
| IcmpInEchos: `RFC1213 icmpInEchos`_
| IcmpInEchoReps: `RFC1213 icmpInEchoReps`_
| IcmpInTimestamps: `RFC1213 icmpInTimestamps`_
| IcmpInTimestampReps: `RFC1213 icmpInTimestampReps`_
| IcmpInAddrMasks: `RFC1213 icmpInAddrMasks`_
| IcmpInAddrMaskReps: `RFC1213 icmpInAddrMaskReps`_
| IcmpOutDestUnreachs: `RFC1213 icmpOutDestUnreachs`_
| IcmpOutTimeExcds: `RFC1213 icmpOutTimeExcds`_
| IcmpOutParmProbs: `RFC1213 icmpOutParmProbs`_
| IcmpOutSrcQuenchs: `RFC1213 icmpOutSrcQuenchs`_
| IcmpOutRedirects: `RFC1213 icmpOutRedirects`_
| IcmpOutEchos: `RFC1213 icmpOutEchos`_
| IcmpOutEchoReps: `RFC1213 icmpOutEchoReps`_
| IcmpOutTimestamps: `RFC1213 icmpOutTimestamps`_
| IcmpOutTimestampReps: `RFC1213 icmpOutTimestampReps`_
| IcmpOutAddrMasks: `RFC1213 icmpOutAddrMasks`_
| IcmpOutAddrMaskReps: `RFC1213 icmpOutAddrMaskReps`_



姣忕 ICMP 绫诲瀷閮芥湁涓や釜璁℃暟鍣細'In' 鍜?'Out'銆備緥濡傦紝瀵逛簬 ICMP Echo 鎶ユ枃锛?
瀹冧滑鏄?IcmpInEchos 鍜?IcmpOutEchos銆傚畠浠殑鍚箟寰堢洿瑙傘€?In' 璁℃暟鍣ㄨ〃绀哄唴鏍?
鏀跺埌浜嗚繖鏍风殑鎶ユ枃锛?Out' 璁℃暟鍣ㄨ〃绀哄唴鏍稿彂閫佷簡杩欐牱鐨勬姤鏂囥€?

- ICMP 鏁板瓧绫诲瀷

瀹冧滑鏄?IcmpMsgInType[N] 鍜?IcmpMsgOutType[N]锛孾N] 琛ㄧず ICMP 绫诲瀷鍙枫€傝繖浜?
璁℃暟鍣ㄨ窡韪墍鏈夌绫荤殑 ICMP 鎶ユ枃銆侷CMP 绫诲瀷鍙风殑瀹氫箟鍙互鍦?`ICMP parameters`_
鏂囨。涓壘鍒般€?


渚嬪锛屽鏋?Linux 鍐呮牳鍙戦€佷竴涓?ICMP Echo 鎶ユ枃锛孖cmpMsgOutType8 浼氬鍔?1銆?
濡傛灉鍐呮牳鏀跺埌涓€涓?ICMP Echo Reply 鎶ユ枃锛孖cmpMsgInType0 浼氬鍔?1銆?

- IcmpInCsumErrors

璇ヨ鏁板櫒琛ㄧず ICMP 鎶ユ枃鐨勬牎楠屽拰閿欒銆傚唴鏍镐細鍦ㄦ洿鏂?IcmpInMsgs 涔嬪悗銆佹洿鏂?
IcmpMsgInType[N] 涔嬪墠鏍￠獙鏍￠獙鍜屻€傚鏋滄姤鏂囨牎楠屽拰閿欒锛孖cmpInMsgs 浼氳鏇存柊锛?
浣嗕换浣?IcmpMsgInType[N] 閮戒笉浼氳鏇存柊銆?

- IcmpInErrors 涓?IcmpOutErrors

鐢?`RFC1213 icmpInErrors`_ 鍜?`RFC1213 icmpOutErrors`_ 瀹氫箟


褰?ICMP 鎶ユ枃澶勭悊璺緞涓彂鐢熼敊璇椂锛岃繖涓や釜璁℃暟鍣ㄤ細琚洿鏂般€傛帴鏀舵姤鏂囪矾寰?
浣跨敤 IcmpInErrors锛屽彂閫佹姤鏂囪矾寰勪娇鐢?IcmpOutErrors銆傚綋 IcmpInCsumErrors
澧炲姞鏃讹紝IcmpInErrors 鎬讳細鍚屾椂澧炲姞銆?

### ICMP 璁℃暟鍣ㄤ箣闂寸殑鍏崇郴

IcmpMsgOutType[N] 鐨勬€诲拰濮嬬粓绛変簬 IcmpOutMsgs锛屽洜涓哄畠浠槸鍚屾椂鏇存柊鐨勩€?
IcmpMsgInType[N] 鐨勬€诲拰鍔犱笂 IcmpInErrors 搴斿綋绛変簬鎴栧ぇ浜?IcmpInMsgs銆傚綋
鍐呮牳鏀跺埌涓€涓?ICMP 鎶ユ枃鏃讹紝鍐呮牳閬靛惊浠ヤ笅閫昏緫锛?

1. 澧炲姞 IcmpInMsgs
2. 濡傛灉鏈変换浣曢敊璇紝鏇存柊 IcmpInErrors 骞剁粨鏉熷鐞?
3. 鏇存柊 IcmpMsgOutType[N]
4. 鏍规嵁绫诲瀷澶勭悊鎶ユ枃锛屽鏋滄湁浠讳綍閿欒锛屾洿鏂?
   IcmpInErrors 骞剁粨鏉熷鐞?

鍥犳锛屽鏋滄墍鏈夐敊璇兘鍙戠敓鍦ㄦ楠?(2)锛孖cmpInMsgs 搴旂瓑浜?IcmpMsgOutType[N]
鐨勬€诲拰鍔犱笂 IcmpInErrors銆傚鏋滄墍鏈夐敊璇兘鍙戠敓鍦ㄦ楠?(4)锛孖cmpInMsgs 搴旂瓑浜?
IcmpMsgOutType[N] 鐨勬€诲拰銆傚鏋滈敊璇悓鏃跺彂鐢熷湪姝ラ (2) 鍜屾楠?(4)锛孖cmpInMsgs
搴斿皬浜?IcmpMsgOutType[N] 鐨勬€诲拰鍔犱笂 IcmpInErrors銆?

## 閫氱敤 TCP 璁℃暟鍣?

- TcpInSegs

瀹氫箟浜?`RFC1213 tcpInSegs`_


TCP 灞傛敹鍒扮殑鎶ユ枃鏁伴噺銆傚 RFC1213 鎵€杩帮紝瀹冨寘鍚帴鏀舵椂鍑洪敊鐨勬姤鏂囷紝渚嬪鏍￠獙鍜?
閿欒銆乀CP 澶撮儴鏃犳晥绛夈€傚彧鏈変竴绉嶉敊璇笉浼氳璁″叆锛氬鏋滅 2 灞傜洰鐨勫湴鍧€涓嶆槸
NIC 鐨勭 2 灞傚湴鍧€銆傝繖鍙兘鍙戠敓鍦ㄦ姤鏂囨槸缁勬挱鎴栧箍鎾姤鏂囷紝鎴栬€?NIC 澶勪簬娣锋潅
妯″紡鏃躲€傚湪杩欎簺鎯呭喌涓嬶紝鎶ユ枃浼氳鎶曢€掔粰 TCP 灞傦紝浣?TCP 灞備細鍦ㄥ鍔?TcpInSegs
涔嬪墠涓㈠純杩欎簺鎶ユ枃銆俆cpInSegs 璁℃暟鍣ㄤ笉鎰熺煡 GRO銆傚洜姝ゅ鏋滄湁涓や釜鎶ユ枃琚?GRO
鍚堝苟锛孴cpInSegs 璁℃暟鍣ㄥ彧浼氬鍔?1銆?

- TcpOutSegs

瀹氫箟浜?`RFC1213 tcpOutSegs`_


TCP 灞傚彂閫佺殑鎶ユ枃鏁伴噺銆傚 RFC1213 鎵€杩帮紝瀹冧笉鍖呭惈閲嶄紶鐨勬姤鏂囷紝浣嗗寘鍚?SYN銆?
ACK 鍜?RST 鎶ユ枃銆備笌 TcpInSegs 涓嶅悓锛孴cpOutSegs 鎰熺煡 GSO锛屽洜姝ゅ鏋滀竴涓姤鏂?
琚?GSO 鎷嗗垎涓?2 涓紝TcpOutSegs 浼氬鍔?2銆?

- TcpActiveOpens

瀹氫箟浜?`RFC1213 tcpActiveOpens`_


瀹冭〃绀?TCP 灞傚彂閫佷竴涓?SYN锛屽苟杩涘叆 SYN-SENT 鐘舵€併€傛瘡娆?TcpActiveOpens 澧炲姞
1锛孴cpOutSegs 閮藉簲褰撳悓鏃跺鍔?1銆?

- TcpPassiveOpens

瀹氫箟浜?`RFC1213 tcpPassiveOpens`_


瀹冭〃绀?TCP 灞傛敹鍒颁竴涓?SYN锛屽洖澶?SYN+ACK锛岃繘鍏?SYN-RCVD 鐘舵€併€?

- TcpExtTCPRcvCoalesce

褰?TCP 灞傛敹鍒版姤鏂囦笖搴旂敤绋嬪簭灏氭湭璇诲彇鏃讹紝TCP 灞備細灏濊瘯鍚堝苟瀹冧滑銆傝璁℃暟鍣?
琛ㄧず鍦ㄨ繖绉嶆儏褰笅鍚堝苟浜嗗灏戜釜鎶ユ枃銆傚鏋滃惎鐢ㄤ簡 GRO锛屽ぇ閲忔姤鏂囦細琚?GRO 鍚堝苟锛?
杩欎簺鎶ユ枃涓嶄細琚鍏?TcpExtTCPRcvCoalesce銆?

- TcpExtTCPAutoCorking

鍙戦€佹姤鏂囨椂锛孴CP 灞備細灏濊瘯灏嗗皬鎶ユ枃鍚堝苟涓烘洿澶х殑鎶ユ枃銆傚湪杩欑鎯呭舰涓嬫瘡鍚堝苟涓€涓?
鎶ユ枃锛岃璁℃暟鍣ㄥ鍔?1銆傛洿澶氱粏鑺傝鍙傝€?LWN 鏂囩珷锛?
https://lwn.net/Articles/576263/

- TcpExtTCPOrigDataSent

璇ヨ鏁板櫒鐢卞唴鏍告彁浜?f19c29e3e391 瑙ｉ噴锛屾垜绮樿创浜?
```

  TCPOrigDataSent: number of outgoing packets with original data (excluding
  retransmission but including data-in-SYN). This counter is different from
  TcpOutSegs because TcpOutSegs also tracks pure ACKs. TCPOrigDataSent is
  more useful to track the TCP retransmission rate.

```
- TCPSynRetrans

璇ヨ鏁板櫒鐢卞唴鏍告彁浜?f19c29e3e391 瑙ｉ噴锛屾垜绮樿创浜?
```

  TCPSynRetrans: number of SYN and SYN/ACK retransmits to break down
  retransmissions into SYN, fast-retransmits, timeout retransmits, etc.

```
- TCPFastOpenActiveFail

璇ヨ鏁板櫒鐢卞唴鏍告彁浜?f19c29e3e391 瑙ｉ噴锛屾垜绮樿创浜?
```

  TCPFastOpenActiveFail: Fast Open attempts (SYN/data) failed because
  the remote does not accept it or the attempts timed out.

```
- TcpExtListenOverflows 涓?TcpExtListenDrops

褰撳唴鏍告敹鍒版潵鑷鎴风鐨?SYN锛屼笖 TCP accept 闃熷垪宸叉弧鏃讹紝鍐呮牳浼氫涪寮冭 SYN
骞跺悜 TcpExtListenOverflows 鍔?1銆傚悓鏃讹紝鍐呮牳涔熶細鍚?TcpExtListenDrops 鍔?1銆?
褰撲竴涓?TCP socket 澶勪簬 LISTEN 鐘舵€佹椂锛屽彧瑕佸唴鏍搁渶瑕佷涪寮冧竴涓姤鏂囷紝鍐呮牳鎬讳細
鍚?TcpExtListenDrops 鍔?1銆傚洜姝わ紝TcpExtListenOverflows 鐨勫鍔犱細鍚屾椂瀵艰嚧
TcpExtListenDrops 澧炲姞锛屼絾 TcpExtListenDrops 涔熷彲鑳藉湪 TcpExtListenOverflows
涓嶅鍔犵殑鎯呭喌涓嬪鍔狅紝渚嬪鍐呭瓨鍒嗛厤澶辫触涔熶細璁?TcpExtListenDrops 澧炲姞銆?

娉ㄦ剰锛氫笂杩拌В閲婂熀浜庡唴鏍?4.10 鎴栦互涓婄増鏈紝鍦ㄦ棫鍐呮牳涓婏紝褰?TCP accept 闃熷垪
宸叉弧鏃?TCP 鍗忚鏍堢殑琛屼负涓嶅悓銆傚湪鏃у唴鏍镐笂锛孴CP 鍗忚鏍堜笉浼氫涪寮?SYN锛岃€屾槸浼?
瀹屾垚涓夋鎻℃墜銆傜敱浜?accept 闃熷垪宸叉弧锛孴CP 鍗忚鏍堜細灏嗚 socket 淇濈暀鍦?TCP
鍗婂紑闃熷垪涓€傚洜涓哄畠澶勪簬鍗婂紑闃熷垪涓紝TCP 鍗忚鏍堜細浠ユ寚鏁伴€€閬垮畾鏃跺櫒鍙戦€?
SYN+ACK锛屽湪瀹㈡埛绔洖澶?ACK 鍚庯紝TCP 鍗忚鏍堜細妫€鏌?accept 闃熷垪鏄惁浠嶇劧宸叉弧锛?
濡傛灉鏈弧锛屽垯灏嗚 socket 绉诲叆 accept 闃熷垪锛涘鏋滃凡婊★紝鍒欑户缁皢鍏朵繚鐣欏湪鍗婂紑
闃熷垪涓紝寰呭鎴风涓嬫鍥炲 ACK 鏃讹紝璇?socket 浼氳幏寰楀張涓€娆＄Щ鍏?accept 闃熷垪鐨?
鏈轰細銆?


## TCP Fast Open

- TcpEstabResets

瀹氫箟浜?`RFC1213 tcpEstabResets`_.


- TcpAttemptFails

瀹氫箟浜?`RFC1213 tcpAttemptFails`_.


- TcpOutRsts

瀹氫箟浜?`RFC1213 tcpOutRsts`_. RFC 鎸囧嚭璇ヨ鏁板櫒琛ㄧず鈥滃寘鍚?RST 鏍囧織鐨勬鈥濓紝
浣嗗湪 linux 鍐呮牳涓紝璇ヨ鏁板櫒琛ㄧず鐨勬槸鍐呮牳灏濊瘯鍙戦€佺殑娈点€傚彂閫佽繃绋嬪彲鑳藉洜鏌愪簺
閿欒锛堜緥濡傚唴瀛樺垎閰嶅け璐ワ級鑰屽け璐ャ€?


- TcpExtTCPSpuriousRtxHostQueues

褰?TCP 鍗忚鏍堟兂瑕侀噸浼犱竴涓姤鏂囷紝鍗村彂鐜拌鎶ユ枃骞堕潪鍦ㄧ綉缁滀腑涓㈠け锛岃€屾槸灏氭湭
鍙戦€佸嚭鍘绘椂锛孴CP 鍗忚鏍堜細鏀惧純閲嶄紶骞舵洿鏂拌璁℃暟鍣ㄣ€傚綋鎶ユ枃鍦?qdisc 鎴栭┍鍔?
闃熷垪涓仠鐣欒繃涔呮椂鍙兘浼氬彂鐢熻繖绉嶆儏鍐点€?

- TcpEstabResets

socket 鍦?Establish 鎴?CloseWait 鐘舵€佹敹鍒颁簡涓€涓?RST 鎶ユ枃銆?

- TcpExtTCPKeepAlive

璇ヨ鏁板櫒琛ㄧず宸插彂閫佷簡澶氬皯涓?keepalive 鎶ユ枃銆俴eepalive 榛樿涓嶅惎鐢ㄣ€傜敤鎴锋€?
绋嬪簭鍙互閫氳繃璁剧疆 SO_KEEPALIVE socket 閫夐」鏉ュ惎鐢ㄥ畠銆?

- TcpExtTCPSpuriousRTOs

鐢?`F-RTO`_ 绠楁硶妫€娴嬪埌鐨勮櫄鍋囬噸浼犺秴鏃躲€?


## TCP Fast Path

褰撳唴鏍告敹鍒颁竴涓?TCP 鎶ユ枃鏃讹紝瀹冩湁涓ゆ潯璺緞鏉ュ鐞嗚鎶ユ枃锛屼竴鏉℃槸蹇€熻矾寰勶紝
鍙︿竴鏉℃槸鎱㈤€熻矾寰勩€傚唴鏍镐腑鐨勬敞閲?
```

  It is split into a fast path and a slow path. The fast path is
  disabled when:

  - A zero window was announced from us
  - zero window probing
    is only handled properly on the slow path.
  - Out of order segments arrived.
  - Urgent data is expected.
  - There is no buffer space left
  - Unexpected TCP flags/window values/header lengths are received
    (detected by checking the TCP header against pred_flags)
  - Data is sent in both directions. The fast path only supports pure senders
    or pure receivers (this means either the sequence number or the ack
    value must stay constant)
  - Unexpected TCP option.

```
闄ら潪婊¤冻涓婅堪浠讳綍鏉′欢锛屽唴鏍搁兘浼氬皾璇曚娇鐢ㄥ揩閫熻矾寰勩€傚鏋滄姤鏂囦贡搴忥紝鍐呮牳浼氫互
鎱㈤€熻矾寰勫鐞嗭紝杩欐剰鍛崇潃鎬ц兘鍙兘涓嶅お濂姐€傚鏋滀娇鐢ㄤ簡 "Delayed ack"锛屽唴鏍镐篃浼?
杩涘叆鎱㈤€熻矾寰勶紝鍥犱负浣跨敤 "Delayed ack" 鏃舵暟鎹細鍦ㄤ袱涓柟鍚戜笂鍙戦€併€傚綋鏈娇鐢?
TCP window scale 閫夐」鏃讹紝鍐呮牳浼氬湪杩炴帴杩涘叆 established 鐘舵€佹椂绔嬪嵆灏濊瘯鍚敤
蹇€熻矾寰勶紱浣嗗鏋滀娇鐢ㄤ簡 TCP window scale 閫夐」锛屽唴鏍镐細鍏堢鐢ㄥ揩閫熻矾寰勶紝骞跺湪
鏀跺埌鎶ユ枃鍚庡啀灏濊瘯鍚敤瀹冦€?

- TcpExtTCPPureAcks 涓?TcpExtTCPHPAcks

濡傛灉涓€涓姤鏂囪缃簡 ACK 鏍囧織涓旀病鏈夋暟鎹紝瀹冨氨鏄函 ACK 鎶ユ枃锛涘鏋滃唴鏍镐互
蹇€熻矾寰勫鐞嗗畠锛孴cpExtTCPHPAcks 澧炲姞 1锛涘鏋滃唴鏍镐互鎱㈤€熻矾寰勫鐞嗗畠锛?
TcpExtTCPPureAcks 澧炲姞 1銆?

- TcpExtTCPHPHits

濡傛灉涓€涓?TCP 鎶ユ枃甯︽湁鏁版嵁锛堝嵆瀹冧笉鏄函 ACK 鎶ユ枃锛夛紝骞朵笖璇ユ姤鏂囦互蹇€熻矾寰?
澶勭悊锛孴cpExtTCPHPHits 澧炲姞 1銆?


## TCP 涓锛坅bort锛?

- TcpExtTCPAbortOnData

瀹冭〃绀?TCP 灞傛湁鍦ㄩ€旀暟鎹紝浣嗛渶瑕佸叧闂繛鎺ャ€傚洜姝?TCP 灞傚悜瀵圭鍙戦€佷竴涓?RST锛?
琛ㄦ槑杩炴帴骞堕潪浼橀泤鍦板叧闂€備竴绉嶅鍔犺璁℃暟鍣ㄧ殑绠€鍗曟柟娉曟槸浣跨敤 SO_LINGER 閫夐」銆?
璇峰弬鑰?`socket man page`_ 鐨?SO_LINGER 灏忚妭锛?


榛樿鎯呭喌涓嬶紝褰撳簲鐢ㄧ▼搴忓叧闂竴涓繛鎺ユ椂锛宑lose 鍑芥暟浼氱珛鍗宠繑鍥烇紝鍐呮牳浼氬皾璇?
寮傛鍙戦€佸湪閫旀暟鎹€傚鏋滀娇鐢?SO_LINGER 閫夐」锛屽皢 l_onoff 璁句负 1锛屽苟灏?l_linger
璁句负涓€涓鏁帮紝close 鍑芥暟涓嶄細绔嬪嵆杩斿洖锛岃€屾槸绛夊緟鍦ㄩ€旀暟鎹瀵圭纭锛屾渶闀?
绛夊緟鏃堕棿涓?l_linger 绉掋€傚鏋滃皢 l_onoff 璁句负 1 骞跺皢 l_linger 璁句负 0锛屽綋
搴旂敤绋嬪簭鍏抽棴杩炴帴鏃讹紝鍐呮牳浼氱珛鍗冲彂閫佷竴涓?RST锛屽苟澧炲姞 TcpExtTCPAbortOnData
璁℃暟鍣ㄣ€?

- TcpExtTCPAbortOnClose

璇ヨ鏁板櫒琛ㄧず褰撳簲鐢ㄧ▼搴忔兂瑕佸叧闂?TCP 杩炴帴鏃讹紝TCP 灞備腑瀛樺湪灏氭湭琚鍙栫殑鏁版嵁銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屽唴鏍镐細鍚?TCP 杩炴帴鐨勫绔彂閫佷竴涓?RST銆?

- TcpExtTCPAbortOnMemory

褰撳簲鐢ㄧ▼搴忓叧闂竴涓?TCP 杩炴帴鏃讹紝鍐呮牳浠嶉渶瑕佽窡韪杩炴帴锛岃鍏跺畬鎴?TCP 鏂紑
杩囩▼銆備緥濡傦紝搴旂敤绋嬪簭璋冪敤浜?socket 鐨?close 鏂规硶锛屽唴鏍稿悜杩炴帴瀵圭鍙戦€?fin锛?
涔嬪悗搴旂敤绋嬪簭涓庤 socket 鍐嶆棤鍏宠仈锛屼絾鍐呮牳闇€瑕佷繚鐣欒 socket锛岃繖涓?socket 鍙樻垚
浜嗗鍎?socket锛屽唴鏍哥瓑寰呭绔殑鍥炲锛屾渶缁堜細杩涘叆 TIME_WAIT 鐘舵€併€傚綋鍐呮牳娌℃湁
瓒冲鐨勫唴瀛樻潵淇濈暀璇ュ鍎?socket 鏃讹紝鍐呮牳浼氬悜瀵圭鍙戦€佷竴涓?RST 骞跺垹闄よ socket锛?
鍦ㄨ繖绉嶆儏鍐典笅锛屽唴鏍镐細璁?TcpExtTCPAbortOnMemory 鍔?1銆傛湁涓ょ鎯呭喌浼氳Е鍙?
TcpExtTCPAbortOnMemory锛?

1. TCP 鍗忚浣跨敤鐨勫唴瀛橀珮浜?tcp_mem 鐨勭涓変釜鍊笺€傝鍙傝€?`TCP man page`_ 鐨?
tcp_mem 灏忚妭锛?


2. 瀛ゅ効 socket 鏁伴噺楂樹簬 net.ipv4.tcp_max_orphans


- TcpExtTCPAbortOnTimeout

褰撲换浣曚竴涓?TCP 瀹氭椂鍣ㄨ秴鏃舵椂锛岃璁℃暟鍣ㄤ細澧炲姞銆傚湪杩欑鎯呭喌涓嬶紝鍐呮牳涓嶄細鍙戦€?
RST锛屽彧鏄斁寮冭杩炴帴銆?

- TcpExtTCPAbortOnLinger

褰撲竴涓?TCP 杩炴帴杩涘叆 FIN_WAIT_2 鐘舵€佹椂锛屽唴鏍稿彲浠ヤ笉绛夊緟瀵圭鐨?fin 鎶ユ枃锛?
鑰屾槸绔嬪嵆鍙戦€佷竴涓?RST 骞跺垹闄よ socket銆傝繖涓嶆槸 Linux 鍐呮牳 TCP 鍗忚鏍堢殑榛樿
琛屼负銆傞€氳繃閰嶇疆 TCP_LINGER2 socket 閫夐」锛屽彲浠ヨ鍐呮牳閲囧彇杩欑琛屼负銆?

- TcpExtTCPAbortFailed

濡傛灉婊¤冻 `RFC2525 2.17 section`_锛屽唴鏍?TCP 灞備細鍙戦€?RST銆傚鏋滃湪姝よ繃绋嬩腑
鍙戠敓鍐呴儴閿欒锛孴cpExtTCPAbortFailed 浼氬鍔犮€?


## TCP 娣峰悎鎱㈠惎鍔紙Hybrid Slow Start锛?

娣峰悎鎱㈠惎鍔ㄧ畻娉曟槸瀵逛紶缁?TCP 鎷ュ绐楀彛鎱㈠惎鍔ㄧ畻娉曠殑澧炲己銆傚畠鍒╃敤涓ょ被淇℃伅鏉?
妫€娴嬫槸鍚︽帴杩?TCP 璺緞鐨勬渶澶у甫瀹姐€傝繖涓ょ被淇℃伅鏄?ACK 鍒楄溅锛圓CK train锛夐暱搴?
鍜屾姤鏂囧欢杩熺殑澧炲姞銆傛洿澶氱粏鑺傝鍙傝€?`Hybrid Slow Start paper`_銆傚彧瑕?ACK 鍒楄溅
闀垮害鎴栨姤鏂囧欢杩熻揪鍒版煇涓壒瀹氶槇鍊硷紝鎷ュ鎺у埗绠楁硶灏变細杩涘叆鎷ュ閬垮厤锛圕ongestion
Avoidance锛夌姸鎬併€傜洿鍒?v4.20锛屾湁涓や釜鎷ュ鎺у埗绠楁硶浣跨敤浜嗘贩鍚堟參鍚姩锛屽畠浠槸
cubic锛堥粯璁ょ殑鎷ュ鎺у埗绠楁硶锛夊拰 cdg銆傛湁鍥涗釜 snmp 璁℃暟鍣ㄤ笌娣峰悎鎱㈠惎鍔ㄧ畻娉曠浉鍏炽€?


- TcpExtTCPHystartTrainDetect

妫€娴嬪埌 ACK 鍒楄溅闀垮害闃堝€肩殑娆℃暟

- TcpExtTCPHystartTrainCwnd

鐢?ACK 鍒楄溅闀垮害妫€娴嬪埌鐨?CWND 涔嬪拰銆傚皢璇ュ€奸櫎浠?TcpExtTCPHystartTrainDetect
鍗充负鐢?ACK 鍒楄溅闀垮害妫€娴嬪埌鐨勫钩鍧?CWND銆?

- TcpExtTCPHystartDelayDetect

妫€娴嬪埌鎶ユ枃寤惰繜闃堝€肩殑娆℃暟銆?

- TcpExtTCPHystartDelayCwnd

鐢辨姤鏂囧欢杩熸娴嬪埌鐨?CWND 涔嬪拰銆傚皢璇ュ€奸櫎浠?TcpExtTCPHystartDelayDetect 鍗充负
鐢辨姤鏂囧欢杩熸娴嬪埌鐨勫钩鍧?CWND銆?

## TCP 閲嶄紶涓庢嫢濉炴帶鍒?

TCP 鍗忚鏈変袱绉嶉噸浼犳満鍒讹細SACK 鍜屽揩閫熸仮澶嶏紙fast recovery锛夈€傚畠浠郊姝や簰鏂ャ€?
褰撳惎鐢?SACK 鏃讹紝鍐呮牳 TCP 鍗忚鏍堜細浣跨敤 SACK锛屽惁鍒欏唴鏍镐細浣跨敤蹇€熸仮澶嶃€係ACK
鏄竴涓?TCP 閫夐」锛屽畾涔変簬 `RFC2018`_锛屽揩閫熸仮澶嶅畾涔変簬 `RFC6582`_锛屼篃绉颁负
'Reno'銆?

TCP 鎷ュ鎺у埗鏄竴涓簽澶ц€屽鏉傜殑涓婚銆傝鐞嗚В鐩稿叧鐨?snmp 璁℃暟鍣紝鎴戜滑闇€瑕?
浜嗚В鎷ュ鎺у埗鐘舵€佹満鐨勭姸鎬併€傚叡鏈?5 涓姸鎬侊細Open銆丏isorder銆丆WR銆丷ecovery 鍜?
Loss銆傚叧浜庤繖浜涚姸鎬佺殑缁嗚妭锛岃鍙傝€冭鏂囨。鐨勭 5 椤靛拰绗?6 椤碉細
https://pdfs.semanticscholar.org/0e9c/968d09ab2e53e24c4dca5b2d67c7f7140f8e.pdf


- TcpExtTCPRenoRecovery 涓?TcpExtTCPSackRecovery

褰撴嫢濉炴帶鍒惰繘鍏?Recovery 鐘舵€佹椂锛屽鏋滀娇鐢ㄤ簡 sack锛孴cpExtTCPSackRecovery
澧炲姞 1锛涘鏋滄湭浣跨敤 sack锛孴cpExtTCPRenoRecovery 澧炲姞 1銆傝繖涓や釜璁℃暟鍣ㄨ〃绀?
TCP 鍗忚鏍堝紑濮嬮噸浼犱涪澶辩殑鎶ユ枃銆?

- TcpExtTCPSACKReneging

涓€涓姤鏂囧凡琚?SACK 纭锛屼絾鎺ユ敹鏂逛涪寮冧簡璇ユ姤鏂囷紝鍥犳鍙戦€佹柟闇€瑕侀噸浼犺鎶ユ枃銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屽彂閫佹柟灏?TcpExtTCPSACKReneging 鍔?1銆傛帴鏀舵柟鍙兘涓㈠純涓€涓?
宸茶 SACK 纭鐨勬姤鏂囷紝灏界杩欏緢涓嶅甯革紝浣?TCP 鍗忚鏄厑璁哥殑銆傚彂閫佹柟鍏跺疄
骞朵笉鐭ラ亾鎺ユ敹鏂瑰彂鐢熶簡浠€涔堛€傚彂閫佹柟鍙槸绛夊緟璇ユ姤鏂囩殑 RTO 瓒呮椂锛岀劧鍚庡亣瀹氳
鎶ユ枃宸茶鎺ユ敹鏂逛涪寮冦€?

- TcpExtTCPRenoReorder

涔卞簭鎶ユ枃鐢卞揩閫熸仮澶嶆娴嬨€傚畠浠呭湪绂佺敤 SACK 鏃朵娇鐢ㄣ€傚揩閫熸仮澶嶇畻娉曢€氳繃閲嶅 ACK
鐨勬暟閲忔潵妫€娴嬩贡搴忋€備緥濡傦紝濡傛灉瑙﹀彂浜嗛噸浼狅紝鑰屽師鏈閲嶄紶鐨勬姤鏂囧苟鏈涪澶憋紝鍙槸
涔卞簭锛屾帴鏀舵柟浼氳繘琛屽娆＄‘璁わ紝涓€娆￠拡瀵归噸浼犵殑鎶ユ枃锛屽彟涓€娆￠拡瀵瑰師濮嬩贡搴忔姤鏂囩殑
鍒拌揪銆傚洜姝ゅ彂閫佹柟浼氬彂鐜版敹鍒扮殑 ACK 澶氫簬棰勬湡锛屼粠鑰岀煡閬撳彂鐢熶簡涔卞簭銆?

- TcpExtTCPTSReorder

褰撳～琛ヤ竴涓┖闅欙紙hole锛夋椂妫€娴嬪埌涔卞簭鎶ユ枃銆備緥濡傦紝鍋囪鍙戦€佹柟鍙戦€佷簡鎶ユ枃
1銆?銆?銆?銆?锛岃€屾帴鏀堕『搴忔槸 1銆?銆?銆?銆?銆傚綋鍙戦€佹柟鏀跺埌鎶ユ枃 3 鐨?ACK锛堝皢
濉ˉ绌洪殭锛夋椂锛屼袱绉嶆儏鍐典笅浼氳 TcpExtTCPTSReorder 澧炲姞 1锛?1) 濡傛灉鎶ユ枃 3
灏氭湭琚啀娆￠噸浼犮€?2) 濡傛灉鎶ユ枃 3 宸茶閲嶄紶锛屼絾鍏?ACK 鐨勬椂闂存埑鏃╀簬閲嶄紶鐨?
鏃堕棿鎴炽€?

- TcpExtTCPSACKReorder

鐢?SACK 妫€娴嬪埌鐨勪贡搴忔姤鏂囥€係ACK 鏈変袱绉嶆柟娉曟娴嬩贡搴忥細(1) 鍙戦€佹柟鏀跺埌 DSACK銆?
杩欒〃绀哄彂閫佹柟澶氭鍙戦€佷簡鍚屼竴涓姤鏂囷紝鍞竴鐨勫師鍥犳槸鍙戦€佹柟璁や负涓€涓贡搴忔姤鏂囧凡
涓㈠け锛屼簬鏄啀娆″彂閫佽鎶ユ枃銆?2) 鍋囪鍙戦€佹柟鍙戦€佷簡鎶ユ枃 1銆?銆?銆?銆?锛屼笖
鍙戦€佹柟宸叉敹鍒版姤鏂?2 鍜?5 鐨?SACK锛岀幇鍦ㄥ彂閫佹柟鏀跺埌鎶ユ枃 4 鐨?SACK锛屼笖鍙戦€佹柟
灏氭湭閲嶄紶璇ユ姤鏂囷紝鍙戦€佹柟灏变細鐭ラ亾鎶ユ枃 4 鏄贡搴忕殑銆傚湪涓婅堪涓ょ鎯呭喌涓嬶紝鍐呮牳鐨?
TCP 鍗忚鏍堥兘浼氬鍔?TcpExtTCPSACKReorder銆?

- TcpExtTCPSlowStartRetrans

TCP 鍗忚鏍堟兂瑕侀噸浼犱竴涓姤鏂囷紝涓旀嫢濉炴帶鍒剁姸鎬佷负 'Loss'銆?

- TcpExtTCPFastRetrans

TCP 鍗忚鏍堟兂瑕侀噸浼犱竴涓姤鏂囷紝涓旀嫢濉炴帶鍒剁姸鎬佷笉涓?'Loss'銆?

- TcpExtTCPLostRetransmit

涓€涓?SACK 鎸囧嚭鏌愪釜閲嶄紶鎶ユ枃鍐嶆涓㈠け銆?

- TcpExtTCPRetransFail

TCP 鍗忚鏍堣瘯鍥惧皢涓€涓噸浼犳姤鏂囦氦浠樼粰涓嬪眰锛屼絾涓嬪眰杩斿洖浜嗛敊璇€?

- TcpExtTCPSynRetrans

TCP 鍗忚鏍堥噸浼犱竴涓?SYN 鎶ユ枃銆?

## DSACK

DSACK 瀹氫箟浜?`RFC2883`_銆傛帴鏀舵柟浣跨敤 DSACK 鍚戝彂閫佹柟鎶ュ憡閲嶅鐨勬姤鏂囥€傚瓨鍦ㄤ袱绫?
閲嶅锛?1) 涓€涓凡琚‘璁ょ殑鎶ユ枃鏄噸澶嶇殑銆?2) 涓€涓贡搴忔姤鏂囨槸閲嶅鐨勩€俆CP 鍗忚鏍?
鍦ㄦ帴鏀舵柟鍜屽彂閫佹柟涓や晶閮界粺璁¤繖涓ょ被閲嶅銆?


- TcpExtTCPDSACKOldSent

TCP 鍗忚鏍堟敹鍒颁竴涓凡琚‘璁ょ殑閲嶅鎶ユ枃锛屼簬鏄悜鍙戦€佹柟鍙戦€佷竴涓?DSACK銆?

- TcpExtTCPDSACKOfoSent

TCP 鍗忚鏍堟敹鍒颁竴涓贡搴忕殑閲嶅鎶ユ枃锛屼簬鏄悜鍙戦€佹柟鍙戦€佷竴涓?DSACK銆?

- TcpExtTCPDSACKRecv

TCP 鍗忚鏍堟敹鍒颁竴涓?DSACK锛岃〃绀烘敹鍒颁簡涓€涓凡琚‘璁ょ殑閲嶅鎶ユ枃銆?

- TcpExtTCPDSACKOfoRecv

TCP 鍗忚鏍堟敹鍒颁竴涓?DSACK锛岃〃绀烘敹鍒颁簡涓€涓贡搴忕殑閲嶅鎶ユ枃銆?

## 鏃犳晥鐨?SACK 涓?DSACK

褰撲竴涓?SACK锛堟垨 DSACK锛夊潡鏃犳晥鏃讹紝鐩稿簲鐨勮鏁板櫒浼氳鏇存柊銆傛牎楠屾柟娉曞熀浜?SACK
鍧楃殑璧峰/缁撴潫搴忓垪鍙枫€傛洿澶氱粏鑺傝鍙傝€冨唴鏍告簮鐮佷腑鍑芥暟 tcp_is_sackblock_valid
鐨勬敞閲娿€備竴涓?SACK 閫夐」鏈€澶氬彲浠ユ湁 4 涓潡锛屽畠浠細琚€愪竴妫€鏌ャ€備緥濡傦紝濡傛灉
涓€涓?SACK 鏈?3 涓潡鏃犳晥锛岀浉搴旂殑璁℃暟鍣ㄤ細琚洿鏂?3 娆°€傛彁浜?18f02545a9a1
锛?[TCP] MIB: Add counters for discarded SACK blocks"锛夌殑娉ㄩ噴鏈夐澶栫殑瑙ｉ噴锛?

- TcpExtTCPSACKDiscard

璇ヨ鏁板櫒琛ㄧず鏈夊灏戜釜 SACK 鍧楁棤鏁堛€傚鏋滄棤鏁堢殑 SACK 鍧楁槸鐢?ACK 璁板綍锛圓CK
recording锛夊紩璧风殑锛孴CP 鍗忚鏍堝彧浼氬拷鐣ュ畠锛岃€屼笉浼氭洿鏂拌璁℃暟鍣ㄣ€?

- TcpExtTCPDSACKIgnoredOld 涓?TcpExtTCPDSACKIgnoredNoUndo

褰撲竴涓?DSACK 鍧楁棤鏁堟椂锛岃繖涓や釜璁℃暟鍣ㄤ箣涓€浼氳鏇存柊銆傛洿鏂板摢涓鏁板櫒鍙栧喅浜?
TCP socket 鐨?undo_marker 鏍囧織銆傚鏋?undo_marker 鏈缃紝TCP 鍗忚鏍堜笉澶?
鍙兘閲嶄紶浠讳綍鎶ユ枃锛岃€屾垜浠粛鐒舵敹鍒颁簡涓€涓棤鏁堢殑 DSACK 鍧楋紝鍘熷洜鍙兘鏄姤鏂囧湪
缃戠粶涓棿琚鍒朵簡銆傚湪杩欑鎯呭喌涓嬶紝TcpExtTCPDSACKIgnoredNoUndo 浼氳鏇存柊銆傚鏋?
undo_marker 宸茶缃紝TcpExtTCPDSACKIgnoredOld 浼氳鏇存柊銆傛濡傚叾鍚嶇О鎵€鏆楃ず鐨勶紝
瀹冨彲鑳芥槸涓€涓棫鐨勬姤鏂囥€?

## SACK 绉讳綅锛坰hift锛?

Linux 缃戠粶鍗忚鏍堝皢鏁版嵁瀛樺偍鍦?sk_buff 缁撴瀯浣擄紙绠€绉?skb锛変腑銆傚鏋滀竴涓?SACK
鍧楄法瓒婂涓?skb锛孴CP 鍗忚鏍堜細灏濊瘯閲嶆柊鏁寸悊杩欎簺 skb 涓殑鏁版嵁銆備緥濡傦紝濡傛灉涓€涓?
SACK 鍧楃‘璁や簡 seq 10 鍒?15锛宻kb1 鎷ユ湁 seq 10 鍒?13锛宻kb2 鎷ユ湁 seq 14 鍒?20銆?
skb2 涓殑 seq 14 鍜?15 浼氳绉诲姩鍒?skb1銆傝繖涓搷浣滅О涓?'shift'锛堢Щ浣嶏級銆傚鏋?
涓€涓?SACK 鍧楃‘璁や簡 seq 10 鍒?20锛宻kb1 鎷ユ湁 seq 10 鍒?13锛宻kb2 鎷ユ湁 seq 14
鍒?20銆俿kb2 涓殑鍏ㄩ儴鏁版嵁閮戒細琚Щ鍔ㄥ埌 skb1锛屽苟涓?skb2 浼氳涓㈠純锛岃繖涓搷浣?
绉颁负 'merge'锛堝悎骞讹級銆?

- TcpExtTCPSackShifted

涓€涓?skb 琚Щ浣?

- TcpExtTCPSackMerged

涓€涓?skb 琚悎骞?

- TcpExtTCPSackShiftFallback

涓€涓?skb 鏈簲琚Щ浣嶆垨鍚堝苟锛屼絾 TCP 鍗忚鏍堝嚭浜庢煇浜涘師鍥犳病鏈夎繖涔堝仛銆?

## TCP 涔卞簭锛坥ut of order锛?

- TcpExtTCPOFOQueue

TCP 灞傛敹鍒颁竴涓贡搴忔姤鏂囷紝骞朵笖鏈夊厖瓒崇殑鍐呭瓨灏嗗叾鍏ラ槦銆?

- TcpExtTCPOFODrop

TCP 灞傛敹鍒颁竴涓贡搴忔姤鏂囷紝浣嗘病鏈夎冻澶熺殑鍐呭瓨锛屼簬鏄皢鍏朵涪寮冦€傝繖绫绘姤鏂囦笉浼氳
璁″叆 TcpExtTCPOFOQueue銆?

- TcpExtTCPOFOMerge

鏀跺埌鐨勪贡搴忔姤鏂囦笌鍓嶄竴涓姤鏂囧瓨鍦ㄩ噸鍙犮€傞噸鍙犻儴鍒嗕細琚涪寮冦€傛墍鏈?TcpExtTCPOFOMerge
鎶ユ枃涔熶細琚鍏?TcpExtTCPOFOQueue銆?

## TCP PAWS

PAWS锛圥rotection Against Wrapped Sequence numbers锛岄槻姝㈠簭鍒楀彿鍥炵粫锛夋槸涓€绉?
鐢ㄤ簬涓㈠純鏃ф姤鏂囩殑绠楁硶銆傚畠渚濊禆浜?TCP 鏃堕棿鎴炽€傛洿澶氱粏鑺傝鍙傝€?`timestamp wiki`_
鍜?`RFC of PAWS`_銆?


- TcpExtPAWSActive

鎶ユ枃鍦?Syn-Sent 鐘舵€佷笅琚?PAWS 涓㈠純銆?

- TcpExtPAWSEstab

鎶ユ枃鍦ㄩ櫎 Syn-Sent 涔嬪鐨勪换浣曠姸鎬佷笅琚?PAWS 涓㈠純銆?

## TCP ACK 璺宠繃

鍦ㄦ煇浜涘満鏅笅锛屽唴鏍镐細閬垮厤杩囦簬棰戠箒鍦板彂閫侀噸澶?ACK銆傛洿澶氱粏鑺傝鍙傝€?`sysctl
document`_ 鐨?tcp_invalid_ratelimit 灏忚妭銆傚綋鍐呮牳鐢变簬 tcp_invalid_ratelimit
鍐冲畾璺宠繃涓€涓?ACK 鏃讹紝鍐呮牳浼氭洿鏂颁互涓嬫煇涓鏁板櫒锛屼互琛ㄦ槑璇?ACK 鏄湪鍝鍦烘櫙涓?
琚烦杩囩殑銆傚彧鏈夊綋鏀跺埌鐨勬姤鏂囨槸 SYN 鎶ユ枃鎴栦笉鍚暟鎹椂锛孉CK 鎵嶄細琚烦杩囥€?


- TcpExtTCPACKSkippedSynRecv

ACK 鍦?Syn-Recv 鐘舵€佷笅琚烦杩囥€係yn-Recv 鐘舵€佽〃绀?TCP 鍗忚鏍堟敹鍒颁簡 SYN 骞?
鍥炲浜?SYN+ACK銆傛鏃?TCP 鍗忚鏍堟鍦ㄧ瓑寰呬竴涓?ACK銆傞€氬父锛孴CP 鍗忚鏍堝湪 Syn-Recv
鐘舵€佷笅涓嶉渶瑕佸彂閫?ACK銆備絾鍦ㄥ嚑绉嶅満鏅笅锛孴CP 鍗忚鏍堥渶瑕佸彂閫?ACK銆備緥濡傦紝TCP
鍗忚鏍堥噸澶嶆敹鍒扮浉鍚岀殑 SYN 鎶ユ枃銆佹敹鍒扮殑鎶ユ枃鏈€氳繃 PAWS 妫€鏌ワ紝鎴栨敹鍒扮殑鎶ユ枃
搴忓垪鍙疯秴鍑虹獥鍙ｃ€傚湪杩欎簺鍦烘櫙涓嬶紝TCP 鍗忚鏍堥渶瑕佸彂閫?ACK銆傚鏋滃彂閫?ACK 鐨勯鐜?
楂樹簬 tcp_invalid_ratelimit 鎵€鍏佽鐨勫€硷紝TCP 鍗忚鏍堜細璺宠繃鍙戦€?ACK锛屽苟澧炲姞
TcpExtTCPACKSkippedSynRecv銆?


- TcpExtTCPACKSkippedPAWS

ACK 鍥犱负 PAWS锛圥rotect Against Wrapped Sequence numbers锛岄槻姝㈠簭鍒楀彿鍥炵粫锛?
妫€鏌ュけ璐ヨ€岃璺宠繃銆傚鏋?PAWS 妫€鏌ュ湪 Syn-Recv銆丗in-Wait-2 鎴?Time-Wait 鐘舵€佷笅
澶辫触锛岃璺宠繃鐨?ACK 浼氳璁″叆 TcpExtTCPACKSkippedSynRecv銆乀cpExtTCPACKSkippedFinWait2
鎴?TcpExtTCPACKSkippedTimeWait銆傚湪鎵€鏈夊叾浠栫姸鎬佷笅锛岃璺宠繃鐨?ACK 浼氳璁″叆
TcpExtTCPACKSkippedPAWS銆?

- TcpExtTCPACKSkippedSeq

搴忓垪鍙疯秴鍑虹獥鍙ｏ紝涓旀椂闂存埑閫氳繃浜?PAWS 妫€鏌ワ紝涓?TCP 鐘舵€佷笉鏄?Syn-Recv銆?
Fin-Wait-2 鍜?Time-Wait銆?

- TcpExtTCPACKSkippedFinWait2

ACK 鍦?Fin-Wait-2 鐘舵€佷笅琚烦杩囷紝鍘熷洜鍙兘鏄?PAWS 妫€鏌ュけ璐ユ垨鏀跺埌鐨勫簭鍒楀彿
瓒呭嚭绐楀彛銆?

- TcpExtTCPACKSkippedTimeWait

ACK 鍦?Time-Wait 鐘舵€佷笅琚烦杩囷紝鍘熷洜鍙兘鏄?PAWS 妫€鏌ュけ璐ユ垨鏀跺埌鐨勫簭鍒楀彿
瓒呭嚭绐楀彛銆?

- TcpExtTCPACKSkippedChallenge

濡傛灉璇?ACK 鏄竴涓寫鎴橈紙challenge锛堿CK锛屽垯璺宠繃瀹冦€俁FC 5961 瀹氫箟浜?3 绉?
鎸戞垬 ACK锛岃鍙傝€?`RFC 5961 section 3.2`_銆乣RFC 5961 section 4.2`_ 鍜?
`RFC 5961 section 5.2`_銆傞櫎浜嗚繖涓夌鍦烘櫙澶栵紝鍦ㄦ煇浜?TCP 鐘舵€佷笅锛屽鏋?ACK 鍙?
浣嶄簬绗竴涓湭纭鍙蜂箣鍓嶏紝Linux 鐨?TCP 鍗忚鏍堜篃浼氬彂閫佹寫鎴?ACK锛堟瘮 `RFC 5961
section 5.2`_ 鏇翠弗鏍硷級銆?

## TCP 鎺ユ敹绐楀彛

- TcpExtTCPWantZeroWindowAdv

鏍规嵁褰撳墠鍐呭瓨浣跨敤鎯呭喌锛孴CP 鍗忚鏍堝皾璇曞皢鎺ユ敹绐楀彛璁句负闆躲€備絾鎺ユ敹绐楀彛浠嶅彲鑳芥槸
涓€涓潪闆跺€笺€備緥濡傦紝濡傛灉涔嬪墠鐨勭獥鍙ｅぇ灏忎负 10锛岃€?TCP 鍗忚鏍堟敹鍒颁簡 3 瀛楄妭锛岄偅涔?
褰撳墠绐楀彛澶у皬浼氭槸 7锛屽嵆浣挎寜鍐呭瓨浣跨敤閲忚绠楀嚭鐨勭獥鍙ｅぇ灏忎负闆躲€?

- TcpExtTCPToZeroWindowAdv

TCP 鎺ユ敹绐楀彛浠庝竴涓潪闆跺€艰璁句负闆躲€?

- TcpExtTCPFromZeroWindowAdv

TCP 鎺ユ敹绐楀彛浠庨浂琚涓洪潪闆跺€笺€?


## 寤惰繜纭锛圖elayed ACK锛?

TCP 寤惰繜纭鏄竴绉嶇敤浜庡噺灏戠綉缁滀腑鎶ユ枃鏁伴噺鐨勬妧鏈€傛洿澶氱粏鑺傝鍙傝€?`Delayed ACK
wiki`_


- TcpExtDelayedACKs

涓€涓欢杩熺‘璁ゅ畾鏃跺櫒鍒版湡銆俆CP 鍗忚鏍堜細鍙戦€佷竴涓函 ACK 鎶ユ枃骞堕€€鍑哄欢杩熺‘璁ゆā寮忋€?

- TcpExtDelayedACKLocked

涓€涓欢杩熺‘璁ゅ畾鏃跺櫒鍒版湡锛屼絾鐢变簬 socket 琚敤鎴锋€佺▼搴忛攣瀹氾紝TCP 鍗忚鏍堟棤娉?
绔嬪嵆鍙戦€?ACK銆俆CP 鍗忚鏍堜細鍦ㄧ◢鍚庯紙鍦ㄧ敤鎴锋€佺▼搴忚В閿佽 socket 涔嬪悗锛夊彂閫佷竴涓?
绾?ACK銆傚綋 TCP 鍗忚鏍堢◢鍚庡彂閫佽绾?ACK 鏃讹紝瀹冧篃浼氭洿鏂?TcpExtDelayedACKs 骞?
閫€鍑哄欢杩熺‘璁ゆā寮忋€?

- TcpExtDelayedACKLost

褰?TCP 鍗忚鏍堟敹鍒颁竴涓凡琚‘璁ょ殑鎶ユ枃鏃讹紝瀹冧細琚洿鏂般€傚欢杩熺‘璁や涪澶卞彲鑳戒細
瀵艰嚧姝ら棶棰橈紝浣嗗畠涔熷彲鑳界敱鍏朵粬鍘熷洜瑙﹀彂锛屼緥濡傛姤鏂囧湪缃戠粶涓澶嶅埗銆?

## 灏鹃儴涓㈠け鎺㈡祴锛圱LP锛孴ail Loss Probe锛?

TLP 鏄竴绉嶇敤浜庢娴?TCP 鎶ユ枃涓㈠け鐨勭畻娉曘€傛洿澶氱粏鑺傝鍙傝€?`TLP paper`_銆?


- TcpExtTCPLossProbes

鍙戦€佷簡涓€涓?TLP 鎺㈡祴鎶ユ枃銆?

- TcpExtTCPLossProbeRecovery

妫€娴嬪埌涓€涓姤鏂囦涪澶卞苟鐢?TLP 鎭㈠銆?

## TCP 蹇€熸墦寮€锛圱CP Fast Open锛夎鏄?

TCP 蹇€熸墦寮€鏄竴绉嶅厑璁稿湪涓夋鎻℃墜瀹屾垚涔嬪墠浼犺緭鏁版嵁鐨勬妧鏈€備竴鑸€т粙缁嶈鍙傝€?
`TCP Fast Open wiki`_銆?


- TcpExtTCPFastOpenActive

褰?TCP 鍗忚鏍堝湪 SYN-SENT 鐘舵€佷笅鏀跺埌涓€涓?ACK 鎶ユ枃锛屼笖璇?ACK 鎶ユ枃纭浜?SYN
鎶ユ枃涓殑鏁版嵁鏃讹紝TCP 鍗忚鏍堜究鐭ラ亾 TFO cookie 宸茶瀵圭鎺ュ彈锛屼簬鏄洿鏂拌璁℃暟鍣ㄣ€?

- TcpExtTCPFastOpenActiveFail

璇ヨ鏁板櫒琛ㄧず TCP 鍗忚鏍堝彂璧蜂簡涓€涓?TCP 蹇€熸墦寮€锛屼絾澶辫触浜嗐€傝璁℃暟鍣ㄤ細鍦ㄤ笁绉?
鍦烘櫙涓嬫洿鏂帮細(1) 瀵圭娌℃湁纭 SYN 鎶ユ枃涓殑鏁版嵁銆?2) 甯︽湁 TFO cookie 鐨?SYN
鎶ユ枃鑷冲皯瓒呮椂浜嗕竴娆°€?3) 涓夋鎻℃墜涔嬪悗锛岄噸浼犺秴鏃跺彂鐢熶簡 net.ipv4.tcp_retries1
娆★紝鍥犱负鏌愪簺涓棿璁惧鍙兘浼氬湪鎻℃墜鍚庘€滈粦娲炩€濇帀蹇€熸墦寮€銆?

- TcpExtTCPFastOpenPassive

璇ヨ鏁板櫒琛ㄧず TCP 鍗忚鏍堟帴鍙楀揩閫熸墦寮€璇锋眰鐨勬鏁般€?

- TcpExtTCPFastOpenPassiveFail

璇ヨ鏁板櫒琛ㄧず TCP 鍗忚鏍堟嫆缁濆揩閫熸墦寮€璇锋眰鐨勬鏁般€傚叾鍘熷洜瑕佷箞鏄?TFO cookie
鏃犳晥锛岃涔堟槸 TCP 鍗忚鏍堝湪鍒涘缓 socket 鐨勮繃绋嬩腑鍙戠幇閿欒銆?

- TcpExtTCPFastOpenListenOverflow

褰撳緟澶勭悊鐨勫揩閫熸墦寮€璇锋眰鏁伴噺澶т簬 fastopenq->max_qlen 鏃讹紝TCP 鍗忚鏍堜細鎷掔粷
璇ュ揩閫熸墦寮€璇锋眰骞舵洿鏂拌璁℃暟鍣ㄣ€傚綋璇ヨ鏁板櫒琚洿鏂版椂锛孴CP 鍗忚鏍堜笉浼氭洿鏂?
TcpExtTCPFastOpenPassive 鎴?TcpExtTCPFastOpenPassiveFail銆俧astopenq->max_qlen
鐢?TCP_FASTOPEN socket 鎿嶄綔璁剧疆锛屼笖涓嶈兘澶т簬 net.core.somaxconn銆備緥濡傦細

setsockopt(sfd, SOL_TCP, TCP_FASTOPEN, &qlen, sizeof(qlen));

- TcpExtTCPFastOpenCookieReqd

璇ヨ鏁板櫒琛ㄧず瀹㈡埛绔兂瑕佽姹備竴涓?TFO cookie 鐨勬鏁般€?

## SYN cookie

SYN cookie 鐢ㄤ簬缂撹В SYN flood 鏀诲嚮锛屾洿澶氱粏鑺傝鍙傝€?`SYN cookies wiki`_銆?


- TcpExtSyncookiesSent

琛ㄧず鍙戦€佷簡澶氬皯涓?SYN cookie銆?

- TcpExtSyncookiesRecv

TCP 鍗忚鏍堟敹鍒颁簡澶氬皯涓?SYN cookie 鐨勫洖澶嶆姤鏂囥€?

- TcpExtSyncookiesFailed

浠?SYN cookie 涓В鐮佸嚭鐨?MSS 鏃犳晥銆傚綋璇ヨ鏁板櫒琚洿鏂版椂锛屾敹鍒扮殑鎶ユ枃涓嶄細琚?
褰撲綔 SYN cookie 澶勭悊锛孴cpExtSyncookiesRecv 璁℃暟鍣ㄤ篃涓嶄細琚洿鏂般€?

## 鎸戞垬 ACK锛圕hallenge ACK锛?

鍏充簬鎸戞垬 ACK 鐨勭粏鑺傦紝璇峰弬鑰?TcpExtTCPACKSkippedChallenge 鐨勮鏄庛€?

- TcpExtTCPChallengeACK

鍙戦€佺殑鎸戞垬 ACK 鐨勬暟閲忋€?

- TcpExtTCPSYNChallenge

涓哄搷搴?SYN 鎶ユ枃鑰屽彂閫佺殑鎸戞垬 ACK 鐨勬暟閲忋€傛洿鏂拌璁℃暟鍣ㄥ悗锛孴CP 鍗忚鏍堝彲鑳戒細
鍙戦€佷竴涓寫鎴?ACK 骞舵洿鏂?TcpExtTCPChallengeACK 璁℃暟鍣紝涔熷彲鑳借烦杩囧彂閫佹寫鎴?
ACK 鑰屾洿鏂?TcpExtTCPACKSkippedChallenge銆?

## 淇壀锛坧rune锛?

褰?socket 澶勪簬鍐呭瓨鍘嬪姏涓嬫椂锛孴CP 鍗忚鏍堜細灏濊瘯浠庢帴鏀堕槦鍒楀拰涔卞簭闃熷垪涓洖鏀跺唴瀛樸€?
鍏朵腑涓€绉嶅洖鏀舵柟娉曟槸 'collapse'锛堟姌鍙狅級锛屽嵆鍒嗛厤涓€涓ぇ鐨?skb锛屽皢杩炵画鐨?skb
澶嶅埗鍒拌繖涓ぇ鐨?skb 涓紝骞堕噴鏀捐繖浜涜繛缁殑 skb銆?

- TcpExtPruneCalled

TCP 鍗忚鏍堝皾璇曚负涓€涓?socket 鍥炴敹鍐呭瓨銆傛洿鏂拌璁℃暟鍣ㄥ悗锛孴CP 鍗忚鏍堜細灏濊瘯鎶樺彔
涔卞簭闃熷垪鍜屾帴鏀堕槦鍒椼€傚鏋滃唴瀛樹粛鐒朵笉瓒筹紝TCP 鍗忚鏍堜細灏濊瘯浠庝贡搴忛槦鍒椾腑涓㈠純鎶ユ枃
锛堝苟鏇存柊 TcpExtOfoPruned 璁℃暟鍣級銆?

- TcpExtOfoPruned

TCP 鍗忚鏍堝皾璇曚粠涔卞簭闃熷垪涓涪寮冩姤鏂囥€?

- TcpExtRcvPruned

缁忚繃 'collapse' 骞朵粠涔卞簭闃熷垪涓涪寮冩姤鏂囧悗锛屽鏋滃疄闄呬娇鐢ㄧ殑鍐呭瓨浠嶇劧澶т簬鍏佽鐨?
鏈€澶у唴瀛橈紝璇ヨ鏁板櫒浼氳鏇存柊銆傝繖鎰忓懗鐫€ 'prune' 澶辫触銆?

- TcpExtTCPRcvCollapsed

璇ヨ鏁板櫒琛ㄧず鍦?'collapse' 杩囩▼涓噴鏀句簡澶氬皯涓?skb銆?

## 绀轰緥


### ping 娴嬭瘯
```

  nstatuser@nstat-a:~$ ping 8.8.8.8 -c 1
  PING 8.8.8.8 (8.8.8.8) 56(84) bytes of data.
  64 bytes from 8.8.8.8: icmp_seq=1 ttl=119 time=17.8 ms

  --- 8.8.8.8 ping statistics ---
  1 packets transmitted, 1 received, 0% packet loss, time 0ms
  rtt min/avg/max/mdev = 17.875/17.875/17.875/0.000 ms

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  IcmpInMsgs                      1                  0.0
  IcmpInEchoReps                  1                  0.0
  IcmpOutMsgs                     1                  0.0
  IcmpOutEchos                    1                  0.0
  IcmpMsgInType0                  1                  0.0
  IcmpMsgOutType8                 1                  0.0
  IpExtInOctets                   84                 0.0
  IpExtOutOctets                  84                 0.0
  IpExtInNoECTPkts                1                  0.0

```
Linux 鏈嶅姟鍣ㄥ彂閫佷簡涓€涓?ICMP Echo 鎶ユ枃锛屽洜姝?IpOutRequests銆?
IcmpOutMsgs銆両cmpOutEchos 鍜?IcmpMsgOutType8 鍚勫鍔?1銆傛湇鍔″櫒浠?
8.8.8.8 鏀跺埌 ICMP Echo Reply锛屽洜姝?IpInReceives銆両cmpInMsgs銆?
IcmpInEchoReps 鍜?IcmpMsgInType0 鍚勫鍔?1銆傝 ICMP Echo Reply 缁?
IP 灞備紶閫掔粰 ICMP 灞傦紝鍥犳 IpInDelivers 澧炲姞 1銆俻ing 鐨勯粯璁ゆ暟鎹ぇ灏?
涓?48锛屽洜姝や竴涓?ICMP Echo 鎶ユ枃鍙婂叾瀵瑰簲鐨?Echo Reply 鎶ユ枃鐢变互涓嬮儴鍒?
鏋勬垚锛?

- 14 瀛楄妭 MAC 澶撮儴
- 20 瀛楄妭 IP 澶撮儴
- 16 瀛楄妭 ICMP 澶撮儴
- 48 瀛楄妭鏁版嵁锛坧ing 鍛戒护鐨勯粯璁ゅ€硷級

鍥犳 IpExtInOctets 鍜?IpExtOutOctets 鍧囦负 20+16+48=84銆?

### TCP 涓夋鎻℃墜
```

  nstatuser@nstat-b:~$ nc -lknv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -nv 192.168.122.251 9000
  Connection to 192.168.122.251 9000 port [tcp/*] succeeded!

```
鏈嶅姟鍣ㄧ洃鍚?tcp 9000 绔彛锛屽鎴风杩炴帴鍒板畠锛屽弻鏂瑰畬鎴愪簡涓夋鎻℃墜銆?
```

  nstatuser@nstat-b:~$ nstat | grep -i tcp
  TcpPassiveOpens                 1                  0.0
  TcpInSegs                       2                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPPureAcks               1                  0.0

```
```

  nstatuser@nstat-a:~$ nstat | grep -i tcp
  TcpActiveOpens                  1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      2                  0.0

```
褰撴湇鍔″櫒鏀跺埌绗竴涓?SYN 鏃讹紝瀹冨洖澶?SYN+ACK锛屽苟杩涘叆 SYN-RCVD 鐘舵€侊紝
鍥犳 TcpPassiveOpens 澧炲姞 1銆傛湇鍔″櫒鏀跺埌 SYN銆佸彂閫?SYN+ACK銆佹敹鍒?
ACK锛屽洜姝ゆ湇鍔″櫒鍙戦€?1 涓姤鏂囥€佹帴鏀?2 涓姤鏂囷紝TcpInSegs 澧炲姞 2锛?
TcpOutSegs 澧炲姞 1銆備笁娆℃彙鎵嬬殑鏈€鍚庝竴涓?ACK 鏄笉甯︽暟鎹殑绾?ACK锛屽洜姝?
TcpExtTCPPureAcks 澧炲姞 1銆?

褰撳鎴风鍙戦€?SYN 鏃讹紝瀹㈡埛绔繘鍏?SYN-SENT 鐘舵€侊紝鍥犳 TcpActiveOpens
澧炲姞 1锛涘鎴风鍙戦€?SYN銆佹敹鍒?SYN+ACK銆佸彂閫?ACK锛屽洜姝ゅ鎴风鍙戦€?2 涓?
鎶ユ枃銆佹帴鏀?1 涓姤鏂囷紝TcpInSegs 澧炲姞 1锛孴cpOutSegs 澧炲姞 2銆?

### TCP 姝ｅ父娴侀噺
```

  nstatuser@nstat-b:~$ nc -lkv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!
  hello

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPPureAcks               1                  0.0
  TcpExtTCPOrigDataSent           1                  0.0
  IpExtInOctets                   52                 0.0
  IpExtOutOctets                  58                 0.0
  IpExtInNoECTPkts                1                  0.0

```
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  IpExtInOctets                   58                 0.0
  IpExtOutOctets                  52                 0.0
  IpExtInNoECTPkts                1                  0.0

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!
  hello
  world

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPHPAcks                 1                  0.0
  TcpExtTCPOrigDataSent           1                  0.0
  IpExtInOctets                   52                 0.0
  IpExtOutOctets                  58                 0.0
  IpExtInNoECTPkts                1                  0.0


```
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPHPHits                 1                  0.0
  IpExtInOctets                   58                 0.0
  IpExtOutOctets                  52                 0.0
  IpExtInNoECTPkts                1                  0.0

```
瀵规瘮绗竴娆″鎴风 nstat 涓庣浜屾瀹㈡埛绔?nstat锛屾垜浠彲浠ュ彂鐜颁竴涓樊寮傦細
绗竴娆℃湁 'TcpExtTCPPureAcks'锛岃€岀浜屾鏈?'TcpExtTCPHPAcks'銆傜涓€娆?
鏈嶅姟鍣ㄧ nstat 涓庣浜屾鏈嶅姟鍣ㄧ nstat 涔熸湁宸紓锛氱浜屾鏈嶅姟鍣ㄧ nstat
鏈?TcpExtTCPHPHits锛岃€岀涓€娆℃湇鍔″櫒绔?nstat 娌℃湁銆傜綉缁滄祦閲忔ā寮忓畬鍏?
鐩稿悓锛氬鎴风鍚戞湇鍔″櫒鍙戦€佷竴涓姤鏂囷紝鏈嶅姟鍣ㄥ洖澶嶄竴涓?ACK銆備絾鍐呮牳浠ヤ笉鍚岀殑
鏂瑰紡澶勭悊瀹冧滑銆傚綋鏈娇鐢?TCP window scale 閫夐」鏃讹紝鍐呮牳浼氬湪杩炴帴杩涘叆
established 鐘舵€佹椂绔嬪嵆灏濊瘯鍚敤蹇€熻矾寰勶紱浣嗗鏋滀娇鐢ㄤ簡 TCP window scale
閫夐」锛屽唴鏍镐細鍏堢鐢ㄥ揩閫熻矾寰勶紝骞跺湪鏀跺埌鎶ユ枃鍚庡啀灏濊瘯鍚敤瀹冦€傛垜浠彲浠ヤ娇鐢?
'ss' 鍛戒护鏉ラ獙璇佹槸鍚︿娇鐢ㄤ簡 window scale 閫夐」銆備緥濡傦紝鍦ㄦ湇鍔″櫒鎴栧鎴风
涓婅繍琛屼互涓嬪懡浠?
```

  nstatuser@nstat-a:~$ ss -o state established -i '( dport = :9000 or sport = :9000 )
  Netid    Recv-Q     Send-Q            Local Address:Port             Peer Address:Port
  tcp      0          0               192.168.122.250:40654         192.168.122.251:9000
             ts sack cubic wscale:7,7 rto:204 rtt:0.98/0.49 mss:1448 pmtu:1500 rcvmss:536 advmss:1448 cwnd:10 bytes_acked:1 segs_out:2 segs_in:1 send 118.2Mbps lastsnd:46572 lastrcv:46572 lastack:46572 pacing_rate 236.4Mbps rcv_space:29200 rcv_ssthresh:29200 minrtt:0.98

```
'wscale:7,7' 琛ㄧず鏈嶅姟鍣ㄥ拰瀹㈡埛绔兘灏?window scale 閫夐」璁句负 7銆傜幇鍦ㄦ垜浠彲浠?
瑙ｉ噴娴嬭瘯涓?nstat 鐨勮緭鍑猴細

鍦ㄥ鎴风绗竴娆?nstat 杈撳嚭涓紝瀹㈡埛绔彂閫佷簡涓€涓姤鏂囷紝鏈嶅姟鍣ㄥ洖澶嶄簡涓€涓?
ACK锛屽綋鍐呮牳澶勭悊杩欎釜 ACK 鏃讹紝蹇€熻矾寰勫皻鏈惎鐢紝鍥犳璇?ACK 琚鍏?
'TcpExtTCPPureAcks'銆?

鍦ㄥ鎴风绗簩娆?nstat 杈撳嚭涓紝瀹㈡埛绔啀娆″彂閫佷簡涓€涓姤鏂囷紝骞舵敹鍒版湇鍔″櫒
鐨勫彟涓€涓?ACK锛屾鏃跺揩閫熻矾寰勫凡鍚敤锛屼笖璇?ACK 绗﹀悎蹇€熻矾寰勬潯浠讹紝鍥犳
鐢卞揩閫熻矾寰勫鐞嗭紝璇?ACK 琚鍏?TcpExtTCPHPAcks銆?

鍦ㄦ湇鍔″櫒绔涓€娆?nstat 杈撳嚭涓紝蹇€熻矾寰勬湭鍚敤锛屽洜姝ゆ病鏈?
'TcpExtTCPHPHits'銆?

鍦ㄦ湇鍔″櫒绔浜屾 nstat 杈撳嚭涓紝蹇€熻矾寰勫凡鍚敤锛屽苟涓斾粠瀹㈡埛绔敹鍒扮殑
鎶ユ枃绗﹀悎蹇€熻矾寰勬潯浠讹紝鍥犳瀹冭璁″叆 'TcpExtTCPHPHits'銆?

### TcpExtTCPAbortOnClose
```

  import socket
  import time

  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
璇?python 鑴氭湰鐩戝惉 9000 绔彛锛屼絾涓嶄細浠庤繛鎺ヤ腑璇诲彇浠讳綍鏁版嵁銆?
```

  nstatuser@nstat-a:~$ echo "hello" | nc nstat-b 9000

```
鐒跺悗锛屾垜浠洖鍒版湇鍔″櫒绔紝鏈嶅姟鍣ㄥ凡缁忔敹鍒颁簡 "hello" 鎶ユ枃锛屽苟涓?TCP 灞?
宸茬粡瀵硅鎶ユ枃杩涜浜嗙‘璁わ紙ack锛夛紝浣嗗簲鐢ㄧ▼搴忓皻鏈鍙栧畠銆傛垜浠緭鍏?
Ctrl-C 鏉ョ粓姝㈡湇鍔″櫒鑴氭湰銆傜劧鍚庢垜浠?
```

  nstatuser@nstat-b:~$ nstat | grep -i abort
  TcpExtTCPAbortOnClose           1                  0.0

```
濡傛灉鎴戜滑鍦ㄦ湇鍔″櫒绔繍琛?tcpdump锛屽彲浠ュ彂鐜版湇鍔″櫒鍦ㄦ垜浠緭鍏?Ctrl-C 鍚?
鍙戦€佷簡涓€涓?RST銆?

### TcpExtTCPAbortOnMemory 涓?TcpExtTCPAbortOnTimeout

涓嬮潰鏄竴涓瀛ゅ効 socket 鏁伴噺瓒呰繃 net.ipv4.tcp_max_orphans 鐨勭ず渚嬨€?
```

  sudo bash -c "echo 10 > /proc/sys/net/ipv4/tcp_max_orphans"

```
```

  nstatuser@nstat-a:~$ cat client_orphan.py
  import socket
  import time

  server = 'nstat-b' # server address
  port = 9000

  count = 64

  connection_list = []

  for i in range(64):
      s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
      s.connect((server, port))
      connection_list.append(s)
      print("connection_count: %d" % len(connection_list))

  while True:
      time.sleep(99999)

```
```

  nstatuser@nstat-b:~$ cat server_orphan.py
  import socket
  import time

  port = 9000
  count = 64

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(count)
  connection_list = []
  while True:
      sock, addr = s.accept()
      connection_list.append((sock, addr))
      print("connection_count: %d" % len(connection_list))

```
鍦ㄦ湇鍔″櫒鍜屽鎴风涓婅繍琛?python 鑴氭湰銆?
```

  python3 server_orphan.py

```
```

  python3 client_orphan.py

```
```

  sudo iptables -A INPUT -i ens3 -p tcp --destination-port 9000 -j DROP

```
鍦ㄥ鎴风杈撳叆 Ctrl-C锛屽仠姝?client_orphan.py銆?
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnMemory          54                 0.0

```
```

  nstatuser@nstat-a:~$ ss -s
  Total: 131 (kernel 0)
  TCP:   14 (estab 1, closed 0, orphaned 10, synrecv 0, timewait 0/0), ports 0

  Transport Total     IP        IPv6
  *         0         -         -
  RAW       1         0         1
  UDP       1         1         0
  TCP       14        13        1
  INET      16        14        2
  FRAG      0         0         0

```
璇ユ祴璇曠殑瑙ｉ噴锛氬湪杩愯 server_orphan.py 鍜?client_orphan.py 涔嬪悗锛屾垜浠湪
鏈嶅姟鍣ㄥ拰瀹㈡埛绔箣闂村缓绔嬩簡 64 涓繛鎺ャ€傝繍琛?iptables 鍛戒护鍚庯紝鏈嶅姟鍣ㄤ細涓㈠純
鏉ヨ嚜瀹㈡埛绔殑鎵€鏈夋姤鏂囷紱鍦?client_orphan.py 涓婅緭鍏?Ctrl-C锛屽鎴风绯荤粺浼?
灏濊瘯鍏抽棴杩欎簺杩炴帴锛屽湪瀹冧滑琚甯稿叧闂箣鍓嶏紝杩欎簺杩炴帴鍙樻垚浜嗗鍎?socket銆傜敱浜?
鏈嶅姟鍣ㄧ殑 iptables 闃绘柇浜嗘潵鑷鎴风鐨勬姤鏂囷紝鏈嶅姟鍣ㄤ笉浼氭敹鍒版潵鑷鎴风鐨?
fin锛屽洜姝ゅ鎴风涓婄殑鎵€鏈夎繛鎺ラ兘浼氬崱鍦?FIN_WAIT_1 闃舵锛屼粠鑰屼綔涓哄鍎?socket
涓€鐩翠繚鎸佸埌瓒呮椂銆傛垜浠皢 10 鍐欏叆 /proc/sys/net/ipv4/tcp_max_orphans锛屽洜姝?
瀹㈡埛绔郴缁熷彧浼氫繚鐣?10 涓鍎?socket锛屽浜庡叾浣欐墍鏈夊鍎?socket锛屽鎴风绯荤粺
浼氬悜瀹冧滑鍙戦€?RST 骞跺皢鍏跺垹闄ゃ€傛垜浠缓绔嬩簡 64 涓繛鎺ワ紝鍥犳 'ss -s' 鍛戒护鏄剧ず
绯荤粺鏈?10 涓鍎?socket锛岃€?TcpExtTCPAbortOnMemory 鐨勫€间负 54銆?

鍏充簬瀛ゅ効 socket 鏁伴噺鐨勮ˉ鍏呰鏄庯細浣犲彲浠ラ€氳繃 'ss -s' 鍛戒护鎵惧埌绮剧‘鐨勫鍎?
socket 鏁伴噺锛屼絾褰撳唴鏍稿喅瀹氭槸鍚﹀鍔?TcpExtTCPAbortOnMemory 骞跺彂閫?RST 鏃讹紝
鍐呮牳骞朵笉鎬绘槸妫€鏌ョ簿纭殑瀛ゅ効 socket 鏁伴噺銆備负浜嗘彁楂樻€ц兘锛屽唴鏍镐細鍏堟鏌ヤ竴涓?
杩戜技璁℃暟锛屽鏋滆繎浼艰鏁板ぇ浜?tcp_max_orphans锛屽唴鏍告墠浼氬啀娆℃鏌ョ簿纭鏁般€?
鍥犳锛屽鏋滆繎浼艰鏁板皬浜?tcp_max_orphans锛屼絾绮剧‘璁℃暟澶т簬 tcp_max_orphans锛?
浣犱細鍙戠幇 TcpExtTCPAbortOnMemory 鏍规湰涓嶄細澧炲姞銆傚鏋?tcp_max_orphans 瓒冲
澶э紝杩欑鎯呭喌涓嶄細鍙戠敓锛涗絾濡傛灉浣犲儚鎴戜滑鐨勬祴璇曢偅鏍锋妸 tcp_max_orphans 璋冨皬锛?
灏卞彲鑳戒細閬囧埌杩欎釜闂銆傛墍浠ュ湪鎴戜滑鐨勬祴璇曚腑锛屽敖绠?tcp_max_orphans 鏄?10锛?
瀹㈡埛绔粛寤虹珛浜?64 涓繛鎺ャ€傚鏋滃鎴风鍙缓绔?11 涓繛鎺ワ紝鎴戜滑灏辫瀵熶笉鍒?
TcpExtTCPAbortOnMemory 鐨勫彉鍖栥€?

缁х画鍓嶉潰鐨勬祴璇曪紝鎴戜滑绛夊緟鍑犲垎閽熴€傜敱浜庢湇鍔″櫒涓婄殑 iptables 闃绘柇浜嗘祦閲忥紝
鏈嶅姟鍣ㄤ笉浼氭敹鍒?fin锛屽鎴风鐨勫叏閮ㄥ鍎?socket 鏈€缁堥兘浼氬湪 FIN_WAIT_1
鐘舵€佽秴鏃躲€傛墍浠ユ垜浠瓑寰呭嚑鍒嗛挓鍚庯紝鍙互鍙戠幇
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnTimeout         10                 0.0

```
### TcpExtTCPAbortOnLinger
```

  nstatuser@nstat-b:~$ cat server_linger.py
  import socket
  import time

  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
```

  nstatuser@nstat-a:~$ cat client_linger.py
  import socket
  import struct

  server = 'nstat-b' # server address
  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack('ii', 1, 10))
  s.setsockopt(socket.SOL_TCP, socket.TCP_LINGER2, struct.pack('i', -1))
  s.connect((server, port))
  s.close()

```
```

  nstatuser@nstat-b:~$ python3 server_linger.py

```
```

  nstatuser@nstat-a:~$ python3 client_linger.py

```
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnLinger          1                  0.0

```
### TcpExtTCPRcvCoalesce

鍦ㄦ湇鍔″櫒绔紝鎴戜滑杩愯涓€涓洃鍚?TCP 9000 绔彛鐨勭▼搴忥紝浣?
```

  import socket
  import time
  port = 9000
  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
```

  python3 server_coalesce.py

```
```

  import socket
  server = 'nstat-b'
  port = 9000
  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.connect((server, port))

```
```

  nstatuser@nstat-a:~$ python3 -i client_coalesce.py

```
```

  >>> s.send(b'foo')
  3

```
```

  >>> s.send(b'bar')
  3

```
```

  ubuntu@nstat-b:~$ nstat
  #kernel
  IpInReceives                    2                  0.0
  IpInDelivers                    2                  0.0
  IpOutRequests                   2                  0.0
  TcpInSegs                       2                  0.0
  TcpOutSegs                      2                  0.0
  TcpExtTCPRcvCoalesce            1                  0.0
  IpExtInOctets                   110                0.0
  IpExtOutOctets                  104                0.0
  IpExtInNoECTPkts                2                  0.0

```
瀹㈡埛绔彂閫佷簡涓や釜鎶ユ枃锛屾湇鍔″櫒娌℃湁璇诲彇浠讳綍鏁版嵁銆傚綋绗簩涓姤鏂囧埌杈炬湇鍔″櫒鏃讹紝
绗竴涓姤鏂囦粛鍦ㄦ帴鏀堕槦鍒椾腑銆傚洜姝?TCP 灞傚悎骞朵簡杩欎袱涓姤鏂囷紝鎴戜滑鍙互鐪嬪埌
TcpExtTCPRcvCoalesce 澧炲姞浜?1銆?

### TcpExtListenOverflows 涓?TcpExtListenDrops
```

  nstatuser@nstat-b:~$ nc -lkv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
nc 鍛戒护鍙帴鍙?1 涓繛鎺ワ紝涓?accept 闃熷垪闀垮害涓?1銆傚湪褰撳墠鐨?linux 瀹炵幇涓紝
灏嗛槦鍒楅暱搴﹁涓?n 鎰忓懗鐫€瀹為檯闃熷垪闀垮害涓?n+1銆傜幇鍦ㄦ垜浠垱寤?3 涓繛鎺ワ紝鍏朵腑
1 涓 nc 鎺ュ彈锛? 涓湪 accept 闃熷垪涓紝鍥犳 accept 闃熷垪宸叉弧銆?
```

  nstatuser@nstat-b:~$ nstat -n

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000

```
濡傛灉 nc 鏈嶅姟鍣ㄨ繍琛屽湪鍐呮牳 4.10 鎴栨洿楂樼増鏈笂锛屼綘涓嶄細鐪嬪埌 "Connection to ... succeeded!"
瀛楃涓诧紝鍥犱负褰?accept 闃熷垪宸叉弧鏃跺唴鏍镐細涓㈠純 SYN銆傚鏋?nc 瀹㈡埛绔繍琛屽湪鏃у唴鏍镐笂锛?
浣犱細鐪嬪埌杩炴帴鎴愬姛浜嗭紝鍥犱负鍐呮牳浼氬畬鎴愪笁娆℃彙鎵嬪苟灏?socket 淇濈暀鍦ㄥ崐寮€闃熷垪涓€?
鎴戝湪鍐呮牳 4.15 涓婅繘琛岀殑娴嬭瘯銆備笅闈㈡槸 nstat
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    4                  0.0
  IpInDelivers                    4                  0.0
  TcpInSegs                       4                  0.0
  TcpExtListenOverflows           4                  0.0
  TcpExtListenDrops               4                  0.0
  IpExtInOctets                   240                0.0
  IpExtInNoECTPkts                4                  0.0

```
TcpExtListenOverflows 鍜?TcpExtListenDrops 鍧囦负 4銆傚鏋滅 4 涓?nc 涓?
nstat 涔嬮棿鐨勬椂闂撮棿闅旀洿闀匡紝TcpExtListenOverflows 鍜?TcpExtListenDrops 鐨?
鍊间細鏇村ぇ锛屽洜涓虹 4 涓?nc 鐨?SYN 琚涪寮冧簡锛屽鎴风姝ｅ湪閲嶈瘯銆?

### IpInAddrErrors銆両pExtInNoRoutes 涓?IpOutNoRoutes

server A IP address: 192.168.122.250
server B IP address: 192.168.122.251
```

  $ sudo ip route add 8.8.8.8/32 via 192.168.122.251

```
```

  $ sudo sysctl -w net.ipv4.conf.all.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.ens3.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.lo.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.default.send_redirects=0

```
鎴戜滑甯屾湜璁?server A 鍚?8.8.8.8 鍙戦€佷竴涓姤鏂囷紝骞跺皢璇ユ姤鏂囪矾鐢卞埌 server B銆?
褰?server B 鏀跺埌杩欐牱鐨勬姤鏂囨椂锛屽畠鍙兘浼氬悜 server A 鍙戦€佷竴涓?ICMP Redirect
娑堟伅锛屽皢 send_redirects 璁句负 0 鍙互绂佺敤姝よ涓恒€?
```

  $ sudo sysctl -w net.ipv4.conf.all.forwarding=0

```
```

  $ nc -v 8.8.8.8 53

```
```

  $ nstat
  #kernel
  IpInReceives                    3                  0.0
  IpInAddrErrors                  3                  0.0
  IpExtInOctets                   180                0.0
  IpExtInNoECTPkts                3                  0.0

```
鐢变簬鎴戜滑璁?server A 灏?8.8.8.8 璺敱鍒?server B锛屽苟涓旀垜浠湪 server B 涓?
绂佺敤浜?IP 杞彂锛宻erver A 鍚?server B 鍙戦€佹姤鏂囧悗锛宻erver B 浼氫涪寮冭繖浜涙姤鏂?
骞跺鍔?IpInAddrErrors銆傜敱浜?nc 鍛戒护濡傛灉鏈敹鍒?SYN+ACK 浼氶噸鏂板彂閫?SYN 鎶ユ枃锛?
鎴戜滑鍙互鍙戠幇澶氫釜 IpInAddrErrors銆?

鍏舵锛岀敓鎴?IpExtInNoRoutes銆傚湪 server B 涓婏紝鎴戜滑鍚敤 IP
```

  $ sudo sysctl -w net.ipv4.conf.all.forwarding=1

```
```

  $ ip route show
  default via 192.168.122.1 dev ens3 proto static
  192.168.122.0/24 dev ens3 proto kernel scope link src 192.168.122.251
  $ sudo ip route delete default via 192.168.122.1 dev ens3 proto static

```
```

  $ nc -v 8.8.8.8 53
  nc: connect to 8.8.8.8 port 53 (tcp) failed: Network is unreachable

```
```

  $ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpOutRequests                   1                  0.0
  IcmpOutMsgs                     1                  0.0
  IcmpOutDestUnreachs             1                  0.0
  IcmpMsgOutType3                 1                  0.0
  IpExtInNoRoutes                 1                  0.0
  IpExtInOctets                   60                 0.0
  IpExtOutOctets                  88                 0.0
  IpExtInNoECTPkts                1                  0.0

```
鎴戜滑鍦?server B 涓婂惎鐢ㄤ簡 IP 杞彂锛屽綋 server B 鏀跺埌鐩殑 IP 鍦板潃涓?8.8.8.8 鐨?
鎶ユ枃鏃讹紝server B 浼氬皾璇曡浆鍙戣鎶ユ枃銆傜敱浜庢垜浠凡缁忓垹闄や簡榛樿璺敱锛屾病鏈夐€氬線
8.8.8.8 鐨勮矾鐢憋紝鍥犳 server B 澧炲姞 IpExtInNoRoutes锛屽苟鍚?server A 鍙戦€?
"ICMP Destination Unreachable" 娑堟伅銆?
```

  $ ping -c 1 8.8.8.8
  connect: Network is unreachable

```
```

  $ nstat
  #kernel
  IpOutNoRoutes                   1                  0.0

```
鎴戜滑宸插湪 server B 涓婂垹闄や簡榛樿璺敱銆俿erver B 鎵句笉鍒伴€氬線 8.8.8.8 鐨勮矾鐢憋紝
鍥犳 server B 澧炲姞浜?IpOutNoRoutes銆?

### TcpExtTCPACKSkippedSynRecv

鍦ㄦ湰娴嬭瘯涓紝鎴戜滑浠庡鎴风鍚戞湇鍔″櫒鍙戦€?3 涓浉鍚岀殑 SYN 鎶ユ枃銆傜涓€涓?SYN
浼氳鏈嶅姟鍣ㄥ垱寤轰竴涓?socket锛屽皢鍏剁疆涓?Syn-Recv 鐘舵€侊紝骞跺洖澶?SYN/ACK銆傜浜屼釜
SYN 浼氳鏈嶅姟鍣ㄥ啀娆″洖澶?SYN/ACK锛屽苟璁板綍鍥炲鏃堕棿锛堥噸澶?ACK 鐨勫洖澶嶆椂闂达級銆?
绗笁涓?SYN 浼氳鏈嶅姟鍣ㄦ鏌ヤ箣鍓嶉噸澶?ACK 鐨勫洖澶嶆椂闂达紝骞跺喅瀹氳烦杩囪閲嶅 ACK锛?
鐒跺悗澧炲姞 TcpExtTCPACKSkippedSynRecv 璁℃暟鍣ㄣ€?
```

  nstatuser@nstat-a:~$ sudo tcpdump -c 1 -w /tmp/syn.pcap port 9000
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
```

  nstatuser@nstat-a:~$ nc nstat-b 9000

```
鐢变簬 nstat-b 娌℃湁鐩戝惉 9000 绔彛锛屽畠搴斿綋鍥炲涓€涓?RST锛宯c 鍛戒护闅忓嵆閫€鍑恒€?
杩欒冻浠ヨ tcpdump 鍛戒护鎹曡幏鍒颁竴涓?SYN 鎶ユ枃銆俵inux 鏈嶅姟鍣ㄥ彲鑳戒細瀵?TCP 鏍￠獙鍜?
浣跨敤纭欢鍗歌浇锛坔ardware offload锛夛紝鍥犳 /tmp/syn.pcap 涓殑鏍￠獙鍜?
```

  nstatuser@nstat-a:~$ tcprewrite --infile=/tmp/syn.pcap --outfile=/tmp/syn_fixcsum.pcap --fixcsum

```
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
鍦?nstat-a 涓婏紝鎴戜滑闃绘柇浜嗘潵鑷?9000 绔彛鐨勬姤鏂囷紝鍚﹀垯 nstat-a 浼氬彂閫?
```

  nstatuser@nstat-a:~$ sudo iptables -A INPUT -p tcp --sport 9000 -j DROP

```
```

  nstatuser@nstat-a:~$ for i in {1..3}; do sudo tcpreplay -i ens3 /tmp/syn_fixcsum.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedSynRecv      1                  0.0

```
姝ｅ棰勬湡锛孴cpExtTCPACKSkippedSynRecv 涓?1銆?

### TcpExtTCPACKSkippedPAWS

瑕佽Е鍙?PAWS锛屾垜浠彲浠ュ彂閫佷竴涓棫鐨?SYN銆?
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ sudo tcpdump -w /tmp/paws_pre.pcap -c 1 port 9000
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
鐜板湪 tcpdump 宸茬粡鎹曡幏鍒?SYN 骞堕€€鍑恒€傛垜浠簲褰撲慨澶?
```

  nstatuser@nstat-a:~$ tcprewrite --infile /tmp/paws_pre.pcap --outfile /tmp/paws.pcap --fixcsum

```
```

  nstatuser@nstat-a:~$ for i in {1..2}; do sudo tcpreplay -i ens3 /tmp/paws.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedPAWS         1                  0.0

```
鎴戜滑閫氳繃 tcpreplay 鍙戦€佷簡涓や釜 SYN锛屽畠浠兘浼氫娇 PAWS 妫€鏌ュけ璐ワ紝nstat-b 涓?
绗竴涓?SYN 鍥炲浜嗕竴涓?ACK锛岃烦杩囦簡绗簩涓?SYN 鐨?ACK锛屽苟鏇存柊浜?
TcpExtTCPACKSkippedPAWS銆?

### TcpExtTCPACKSkippedSeq

瑕佽Е鍙?TcpExtTCPACKSkippedSeq锛屾垜浠彂閫佸甫鏈夋湁鏁堟椂闂存埑锛堜互閫氳繃 PAWS 妫€鏌ワ級
浣嗗簭鍒楀彿瓒呭嚭绐楀彛鐨勬姤鏂囥€俵inux 鐨?TCP 鍗忚鏍堜細鍦ㄦ姤鏂囧甫鏁版嵁鏃堕伩鍏嶈烦杩囷紝
鍥犳鎴戜滑闇€瑕佷竴涓函 ACK 鎶ユ枃銆傝鐢熸垚杩欐牱鐨勬姤鏂囷紝鎴戜滑鍙互鍒涘缓涓や釜 socket锛?
涓€涓湪 9000 绔彛锛屽彟涓€涓湪 9001 绔彛銆傜劧鍚庡湪 9001 绔彛涓婃崟鑾蜂竴涓?ACK锛?
灏嗘簮/鐩殑绔彛鍙锋敼涓哄尮閰?9000 绔彛鐨?socket銆傛帴鐫€鎴戜滑灏卞彲浠ラ€氳繃璇ユ姤鏂?
瑙﹀彂 TcpExtTCPACKSkippedSeq銆?

鍦?nstat-b 涓婏紝鎵撳紑涓や釜缁堢锛岃繍琛屼袱涓?nc 鍛戒护鍒嗗埆鐩戝惉
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

  nstatuser@nstat-b:~$ nc -lkv 9001
  Listening on [0.0.0.0] (family 0, port 9001)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

  nstatuser@nstat-a:~$ nc -v nstat-b 9001
  Connection to nstat-b 9001 port [tcp/*] succeeded!

```
```

  nstatuser@nstat-a:~$ sudo tcpdump -w /tmp/seq_pre.pcap -c 1 dst port 9001
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
鍦?nstat-b 涓婏紝閫氳繃 9001 绔彛鐨?socket 鍙戦€佷竴涓姤鏂囥€備緥濡傛垜浠彂閫佷簡涓€涓?
```

  nstatuser@nstat-b:~$ nc -lkv 9001
  Listening on [0.0.0.0] (family 0, port 9001)
  Connection from nstat-a 42132 received!
  foo

```
鍦?nstat-a 涓婏紝tcpdump 搴斿綋宸茬粡鎹曡幏鍒拌 ACK銆傛垜浠簲褰撴鏌?
```

  nstatuser@nstat-a:~$ ss -ta '( dport = :9000 || dport = :9001 )' | tee
  State  Recv-Q   Send-Q         Local Address:Port           Peer Address:Port
  ESTAB  0        0            192.168.122.250:50208       192.168.122.251:9000
  ESTAB  0        0            192.168.122.250:42132       192.168.122.251:9001

```
杩愯 tcprewrite锛屽皢 9001 绔彛鏀逛负 9000 绔彛锛屽皢 42132 绔彛鏀逛负
```

  nstatuser@nstat-a:~$ tcprewrite --infile /tmp/seq_pre.pcap --outfile /tmp/seq.pcap -r 9001:9000 -r 42132:50208 --fixcsum

```
```

  nstatuser@nstat-a:~$ for i in {1..2}; do sudo tcpreplay -i ens3 /tmp/seq.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedSeq          1                  0.0

```