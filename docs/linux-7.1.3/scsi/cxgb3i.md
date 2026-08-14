
## Chelsio S3 iSCSI 椹卞姩锛圠inux 鐗堬級


## 绠€浠?

鍩轰簬 Chelsio T3 ASIC 鐨勯€傞厤鍣紙S310銆丼320銆丼302銆丼304銆丮ezz 鍗＄瓑浜у搧绯诲垪锛夋敮鎸?iSCSI 鍔犻€熶互鍙?iSCSI 鐩存帴鏁版嵁鏀剧疆锛圖DP锛夛紝鐢辩‖浠跺鐞嗗紑閿€杈冨ぇ鐨勯€愬瓧鑺傛搷浣滐紝渚嬪
CRC 璁＄畻涓庢牎楠岋紝浠ュ強鐩存帴鍚戞渶缁堜富鏈哄唴瀛樼洰鏍囧湴鍧€鍙戣捣 DMA锛?
 - iSCSI PDU 鎽樿鐢熸垚涓庢牎楠?
	  鍙戦€佹椂锛孋helsio S3 纭欢璁＄畻骞跺皢澶撮儴涓庢暟鎹憳瑕佹彃鍏ュ埌 PDU 涓€?	  鎺ユ敹鏃讹紝Chelsio S3 纭欢璁＄畻骞舵牎楠?PDU 鐨勫ご閮ㄤ笌鏁版嵁鎽樿銆?
 - 鐩存帴鏁版嵁鏀剧疆锛圖DP锛?
	  S3 纭欢鍙互鏍规嵁 Data-In PDU 涓殑鍙戣捣鑰呬换鍔℃爣绛撅紙ITT锛夋垨 Data-Out
	  PDU 涓殑鐩爣浠诲姟鏍囩锛圱TT锛夛紝灏?iSCSI Data-In 鎴?Data-Out PDU 鐨?	  鏈夋晥杞借嵎鐩存帴鏀剧疆鍒伴鍏堟彁浜ょ殑銆佹渶缁堢洰鐨勫湴鐨勪富鏈哄唴瀛樼紦鍐插尯涓€?
 - PDU 鍙戦€佷笌鎭㈠

	  鍙戦€佹椂锛孲3 纭欢浠庝富鏈洪┍鍔ㄦ帴鏀跺畬鏁寸殑 PDU锛堝ご閮?+ 鏁版嵁锛夛紝璁＄畻骞舵彃鍏?	  鎽樿锛屽湪蹇呰鏃跺皢 PDU 鍒嗚В涓哄涓?TCP 娈碉紝骞跺皢鎵€鏈?TCP 娈靛彂閫佸埌缃戠粶涓娿€?	  濡傞渶閲嶄紶锛屽畠浼氬鐞?TCP 閲嶄紶銆?
	  鎺ユ敹鏃讹紝S3 纭欢閫氳繃閲嶇粍 TCP 娈垫潵鎭㈠ iSCSI PDU锛屽垎绂诲ご閮ㄤ笌鏁版嵁锛?	  璁＄畻骞舵牎楠屾憳瑕侊紝鐒跺悗灏嗗ご閮ㄨ浆鍙戠粰涓绘満銆傛湁鏁堣浇鑽锋暟鎹鏈夊彲鑳藉皢鐩存帴
	  鏀剧疆鍒伴鍏堟彁浜ょ殑涓绘満 DDP 缂撳啿鍖轰腑锛屽惁鍒欐湁鏁堣浇鑽锋暟鎹篃浼氬彂閫佺粰涓绘満銆?
cxgb3i 椹卞姩涓?open-iscsi 鍙戣捣鑰呭鎺ワ紝骞跺湪閫傜敤澶勯€氳繃 Chelsio 纭欢鎻愪緵 iSCSI 鍔犻€熴€?
## 浣跨敤 cxgb3i 椹卞姩


瑕佷娇 open-iscsi 鍙戣捣鑰呰幏寰楀姞閫燂紝闇€瑕佹墽琛屼互涓嬫楠わ細

1. 鍔犺浇 cxgb3i 椹卞姩锛?modprobe cxgb3i"

   cxgb3i 妯″潡浼氬悜 open-iscsi 娉ㄥ唽涓€涓柊鐨勪紶杈撶被 "cxgb3i"銆?
```

	Device Drivers
		SCSI device support --->
			[*] SCSI low-level drivers  --->
				<M>   Chelsio S3xx iSCSI support

```
2. 鍦?/etc/iscsi/ifaces/ 涓嬩负鏂扮殑浼犺緭绫?"cxgb3i" 鍒涘缓涓€涓帴鍙ｆ枃浠躲€?
```

	iface.transport_name = cxgb3i
	iface.net_ifacename = <ethX>
	iface.ipaddress = <iscsi ip address>

   * 鑻ユ寚瀹氫簡 iface.ipaddress锛屽垯 <iscsi ip address> 蹇呴』涓?ethX 鐨?IP 鍦板潃鐩稿悓锛?     鎴栦綅浜庡悓涓€瀛愮綉鍐呫€傝纭繚璇?IP 鍦板潃鍦ㄧ綉缁滀腑鍞竴銆?
```
3. 缂栬緫 /etc/iscsi/iscsid.conf
   榛樿璁剧疆 MaxRecvDataSegmentLength锛?31072锛夎繃澶э紱
```

	node.conn[0].iscsi.MaxRecvDataSegmentLength = 8192

   * 鑻?MaxRecvDataSegmentLength 杩囧ぇ锛屾櫘閫氫細璇濈殑鐧诲綍浼氬け璐ャ€傜郴缁熶細鍦?dmesg 涓?     璁板綍鏍煎紡濡備笅鐨勯敊璇秷鎭細
     "cxgb3i: ERR! MaxRecvSegmentLength <X> too big. Need to be <= <Y>."

```
4. 瑕佷娇 open-iscsi 娴侀噺缁忕敱 cxgb3i 鐨勫姞閫熻矾寰勶紝澶у鏁?iscsiadm 鍛戒护閮介渶瑕佹寚瀹?   "-I <iface file name>" 閫夐」銆?iface file name> 涓虹 2 姝ヤ腑鍒涘缓鐨勪紶杈撴帴鍙ｆ枃浠躲€?
