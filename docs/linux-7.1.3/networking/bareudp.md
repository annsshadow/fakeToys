
## Bare UDP 闅ч亾妯″潡鏂囨。


褰撳墠鏈夊绉嶅熀浜?UDP 鐨?L3 灏佽鏍囧噯姝ｅ湪琚璁猴紝浠ュ埄鐢ㄤ笉鍚岀綉缁滃熀浜?UDP 鐨勮礋杞藉潎琛¤兘鍔涖€侻PLSoUDP (https://tools.ietf.org/html/rfc7510) 灏辨槸鍏朵腑涔嬩竴銆?
Bareudp 闅ч亾妯″潡涓哄湪 UDP 闅ч亾鍐呭皝瑁?MPLS銆両P銆丯SH 绛変笉鍚?L3 鍗忚鎻愪緵浜嗛€氱敤鐨?L3 灏佽鏀寔銆?
### 鐗规畩澶勭悊


bareudp 璁惧瀵?MPLS 涓?IP 鎻愪緵鐗规畩澶勭悊锛屽洜涓哄畠浠彲浠ユ嫢鏈夊绉?ethertype锛堜互澶被鍨嬶級銆侻PLS 鍗忚鍙互鎷ユ湁 ethertype ETH_P_MPLS_UC锛堝崟鎾級涓?ETH_P_MPLS_MC锛堢粍鎾級銆侷P 鍗忚鍙互鎷ユ湁 ethertype ETH_P_IP锛坴4锛変笌 ETH_P_IPV6锛坴6锛夈€傝繖绉嶇壒娈婂鐞嗗彧鑳介拡瀵?ethertype ETH_P_IP 涓?ETH_P_MPLS_UC 鍚敤锛岄€氳繃涓€涓О涓?multiproto 妯″紡鐨勬爣蹇楁潵瀹炵幇銆?
### 鐢ㄦ硶


1) 璁惧鍒涘缓涓庡垹闄?
    a) ip link add dev bareudp0 type bareudp dstport 6635 ethertype mpls_uc

       杩欏皢鍒涘缓涓€涓?bareudp 闅ч亾璁惧锛岀敤浜庡皝瑁?ethertype 涓?0x8847锛圡PLS 娴侀噺锛夌殑 L3 娴侀噺銆俇DP 澶寸殑鐩殑绔彛灏嗚璁剧疆涓?6635銆傝璁惧灏嗗湪 UDP 绔彛 6635 涓婄洃鍚互鎺ユ敹娴侀噺銆?
    b) ip link delete bareudp0

2) 鍚敤 multiproto 妯″紡鍒涘缓璁惧

multiproto 妯″紡鍏佽 bareudp 闅ч亾澶勭悊鍚屼竴鏃忕殑澶氱鍗忚銆傜洰鍓嶄粎鍙敤浜?IP 涓?MPLS銆傝妯″紡蹇呴』閫氳繃鈥渕ultiproto鈥濇爣蹇楁樉寮忓惎鐢ㄣ€?
    a) ip link add dev bareudp0 type bareudp dstport 6635 ethertype ipv4 multiproto

       瀵逛簬 IPv4 闅ч亾锛宮ultiproto 妯″紡鍏佽璇ラ毀閬撳悓鏃跺鐞?IPv6銆?
    b) ip link add dev bareudp0 type bareudp dstport 6635 ethertype mpls_uc multiproto

       瀵逛簬 MPLS锛宮ultiproto 妯″紡鍏佽璇ラ毀閬撳悓鏃跺鐞嗗崟鎾笌缁勬挱 MPLS 鎶ユ枃銆?
3) 璁惧浣跨敤

bareudp 璁惧鍙笌 OVS 鎴?TC 涓殑 flower 杩囨护鍣ㄤ竴璧蜂娇鐢ㄣ€侽VS 鎴?TC flower 灞傚繀椤诲湪灏嗘姤鏂囩紦鍐插尯鍙戦€佺粰 bareudp 璁惧杩涜鍙戦€佷箣鍓嶏紝鍦?SKB 鐨?dst 瀛楁涓缃毀閬撲俊鎭€傚湪鎺ユ敹鏃讹紝bareUDP 璁惧鎻愬彇闅ч亾淇℃伅骞跺瓨鍌ㄥ湪 SKB 鐨?dst 瀛楁涓紝鍐嶅皢鎶ユ枃缂撳啿鍖轰紶閫掔粰缃戠粶鍗忚鏍堛€?