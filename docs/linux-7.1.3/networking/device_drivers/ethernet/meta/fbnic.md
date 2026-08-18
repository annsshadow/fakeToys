## Meta 骞冲彴涓绘満缃戠粶鎺ュ彛


### 鍥轰欢鐗堟湰


fbnic 鍦ㄩ棯瀛樹笂瀛樺偍浜嗕笁涓敱鍗曚釜 PLDM 闀滃儚鎻愪緵鐨勭粍浠讹細

1. fw - 鐢ㄤ簬鎺у埗鍥轰欢锛岀敤浜庢煡鐪嬪拰淇敼鍥轰欢璁剧疆銆佽姹傚浐浠跺姩浣滐紝浠ュ強鍦ㄦ暟鎹矾寰勪箣澶栬幏鍙栧浐浠惰鏁板櫒銆傝繖鏄?`fbnic_fw.c` 鎵€浜や簰鐨勫浐浠躲€?2. bootloader - 鐢ㄤ簬鏍￠獙鍥轰欢瀹夊叏鎬у苟鎺у埗鍩烘湰鎿嶄綔锛堝寘鎷姞杞藉拰鏇存柊鍥轰欢锛夌殑鍥轰欢銆傝繖涔熻绉颁负 cmrt 鍥轰欢銆?3. undi - 鍩轰簬 Linux 椹卞姩鏋勫缓鐨?UEFI 椹卞姩銆?
fbnic 鍦ㄩ棯瀛樹笂涓鸿繖涓変釜缁勪欢鍚勫瓨鍌ㄤ袱浠藉壇鏈€傝繖浣垮緱 fbnic 鍦ㄥ浐浠跺惎鍔ㄥけ璐ユ椂鑳借嚜鍔ㄥ洖閫€鍒版棫鐗堟湰鐨勫浐浠躲€備袱鑰呯殑鐗堟湰淇℃伅閮戒互 running 鍜?stored 鐨勫舰寮忔彁渚涖€倁ndi 浠呬互 stored 褰㈠紡鎻愪緵锛屽洜涓轰竴鏃?Linux 椹卞姩鎺ョ锛屽畠灏变笉鍐嶄富鍔ㄨ繍琛屻€?
`devlink dev info` 鎻愪緵鍏ㄩ儴涓変釜缁勪欢鐨勭増鏈俊鎭€傞櫎浜嗙増鏈箣澶栵紝鏋勫缓鐨?hg commit hash 涔熶綔涓哄崟鐙殑鏉＄洰涓€骞跺寘鍚€?
### 閰嶇疆


#### Ring 鍙傛暟锛坋thtool -g / -G锛?

fbnic 涓烘瘡涓畬鎴愶紙璁惧 -> 涓绘満锛夌幆閰嶅涓や釜鎻愪氦锛堜富鏈?-> 璁惧锛夌幆銆傝繖涓変釜鐜璞″叡鍚岀粍鎴愪笂灞傝蒋浠朵娇鐢ㄧ殑涓€涓€滈槦鍒椻€濓紙涓€涓?Rx 闃熷垪鎴?Tx 闃熷垪锛夈€?
瀵逛簬 Rx锛屼袱涓彁浜ょ幆鐢ㄤ簬灏嗙┖椤典紶閫掔粰 NIC銆傜幆 0 鏄?Header Page Queue锛圚PQ锛屽ご椤甸槦鍒楋級锛孨IC 灏嗕娇鐢ㄥ叾椤甸潰鏉ユ斁缃?L2-L4 澶撮儴锛堝鏋滃抚涓嶆槸澶?鏁版嵁鍒嗙锛屽垯鏀剧疆鏁村抚锛夈€傜幆 1 鏄?Payload Page Queue锛圥PQ锛岃浇鑽烽〉闃熷垪锛夛紝鐢ㄤ簬鍖呰浇鑽枫€傚畬鎴愮幆鐢ㄤ簬鎺ユ敹鍖呴€氱煡/鍏冩暟鎹€俥thtool 鐨?`rx` ring 鍙傛暟瀵瑰簲瀹屾垚鐜殑澶у皬锛宍rx-mini` 瀵瑰簲 HPQ锛宍rx-jumbo` 瀵瑰簲 PPQ銆?
瀵逛簬 Tx锛屼袱涓彁浜ょ幆閮藉彲鐢ㄤ簬鎻愪氦鍖咃紝瀹屾垚鐜惡甯︿袱鑰呯殑閫氱煡銆俧bnic 浣跨敤涓€涓彁浜ょ幆鏉ュ鐞嗘潵鑷崗璁爤鐨勬櫘閫氭祦閲忥紝绗簩涓敤浜庡鐞?XDP 甯с€俥thtool 鐨?`tx` ring 鍙傛暟鍚屾椂鎺у埗鎻愪氦鐜拰瀹屾垚鐜殑澶у皬銆?
HPQ 鍜?PPQ锛坄rx-mini`銆乣rx-jumbo`锛変笂鐨勬瘡涓€涓〃椤瑰搴?4kB 鐨勫凡鍒嗛厤鍐呭瓨锛岃€屽叾浣欑幆涓婄殑琛ㄩ」浠ユ弿杩扮锛?B锛変负鍗曚綅銆傛彁浜ょ幆涓庡畬鎴愮幆澶у皬涔嬮棿鐨勭悊鎯虫瘮渚嬪彇鍐充簬宸ヤ綔璐熻浇锛屽洜涓哄浜庡皬鍖咃紝澶氫釜鍖呭彲浠ユ斁鍏ュ崟涓〉闈€?
### 鍗囩骇鍥轰欢


fbnic 鏀寔浣跨敤甯︽湁绛惧悕鐨?PLDM 闀滃儚閫氳繃 `devlink dev flash` 鏉ユ洿鏂板浐浠躲€侾LDM 闀滃儚琚啓鍏ラ棯瀛樸€傚埛鍐欒繃绋嬩笉浼氫腑鏂澶囩殑杩愯銆?
涓绘満鍚姩鏃跺缁堜娇鐢ㄦ渶鏂扮殑 UEFI 椹卞姩锛屾棤闇€鏄惧紡婵€娲汇€傝繍琛屾柊鐨勬帶鍒跺浐浠堕渶瑕佹縺娲诲浐浠躲€俢mrt 鍥轰欢鍙兘閫氳繃缁?NIC 鏂數鍐嶄笂鐢碉紙power cycle锛夋潵婵€娲汇€?
### 鍋ュ悍鎶ュ憡鍣紙Health reporters锛?

#### fw 鎶ュ憡鍣?

`fw` 鍋ュ悍鎶ュ憡鍣ㄨ窡韪?FW 宕╂簝銆傝浆鍌ㄨ鎶ュ憡鍣ㄥ皢鏄剧ず鏈€杩戜竴娆?FW 宕╂簝鐨勬牳蹇冭浆鍌紱濡傛灉鑷柇鐢典互鏉ユ湭鍙戠敓 FW 宕╂簝锛屽垯鏄剧ず涓€浠?FW 鍐呭瓨蹇収銆傝瘖鏂洖璋冩牴鎹渶杩戞帴鏀跺埌鐨勫績璺虫秷鎭樉绀?FW 宸茶繍琛屾椂闂达紙宕╂簝閫氳繃妫€鏌ヨ繍琛屾椂闂存槸鍚︿笅闄嶆潵妫€娴嬶級銆?
#### otp 鎶ュ憡鍣?

OTP 鍐呭瓨锛堚€滅啍涓濃€濓級鐢ㄤ簬瀹夊叏鍚姩鍜岄槻鍥炴粴淇濇姢銆侽TP 鍐呭瓨鍙?ECC 淇濇姢锛孍CC 閿欒琛ㄦ槑瀛樺湪鍒堕€犵己闄锋垨閮ㄤ欢闅忚€佸寲鑰岄€€鍖栥€?
### 缁熻淇℃伅


#### TX MAC 鎺ュ彛


 - `ptp_illegal_req`锛氳缃簡 PTP 璇锋眰浣嶄絾琚矾鐢卞埌 BMC/FW 鐨勩€佸彂寰€ NIC 鐨勫寘
 - `ptp_good_ts`锛氭垚鍔熻矾鐢卞埌 MAC 涓旇缃簡 PTP 璇锋眰浣嶇殑鍖? - `ptp_bad_ts`锛氱洰鐨勪负 MAC 涓旇缃簡 PTP 璇锋眰浣嶃€佷絾鍥犳煇绉嶉敊璇紙渚嬪 DMA 璇婚敊璇級鑰屼腑姝㈢殑鍖?
#### TX Extension锛圱EI锛夋帴鍙ｏ紙TTI锛?

 - `tti_cm_drop`锛氬洜淇＄敤锛坈redit锛夎€楀敖鑰屽湪 TX Extension锛圱EI锛夋帴鍙ｅ涓㈠純鐨勬帶鍒舵秷鎭? - `tti_frame_drop`锛氬洜淇＄敤鑰楀敖鑰屽湪 TX Extension锛圱EI锛夋帴鍙ｅ涓㈠純鐨勫寘
 - `tti_tbi_drop`锛氬洜淇＄敤鑰楀敖鑰屽湪 TX BMC 鎺ュ彛锛圱BI锛夊涓㈠純鐨勫寘

#### RXB锛圧X Buffer锛夊叆闃?

 - `rxb_integrity_err[i]`锛氬湪 RXB 杈撳叆 i 涓婁互瀹屾暣鎬ч敊璇紙渚嬪澶氫綅 ECC 閿欒锛夊叆闃熺殑甯? - `rxb_mac_err[i]`锛氬湪 RXB 杈撳叆 i 涓婁互 MAC 甯у熬閿欒锛堜緥濡傚潖 FCS锛夊叆闃熺殑甯? - `rxb_parser_err[i]`锛氱粡鍘嗕簡 RPC 瑙ｆ瀽鍣ㄩ敊璇殑甯? - `rxb_frm_err[i]`锛氬湪 RXB 杈撳叆 i 涓婄粡鍘嗕簡淇″彿閿欒锛堜緥濡傜己灏戝寘灏?鍖呴锛夌殑甯? - `rxb_drbo[i]_frames`锛氬湪 RXB 杈撳叆 i 涓婃帴鏀跺埌鐨勫抚
 - `rxb_drbo[i]_bytes`锛氬湪 RXB 杈撳叆 i 涓婃帴鏀跺埌鐨勫瓧鑺?
#### RXB锛圧X Buffer锛塅IFO


 - `rxb_fifo[i]_drop`锛氳繘鍏?RXB 姹?i 涓㈠純鐘舵€佺殑娆℃暟
 - `rxb_fifo[i]_dropped_frames`锛氬湪 RXB 姹?i 涓婅涓㈠純鐨勫抚
 - `rxb_fifo[i]_ecn`锛氳繘鍏?RXB 姹?i ECN 鏍囪鐘舵€佺殑娆℃暟
 - `rxb_fifo[i]_level`锛歊XB 姹?i 鐨勫綋鍓嶅崰鐢?
#### RXB锛圧X Buffer锛夊嚭闃?

   - `rxb_intf[i]_frames`锛氬彂寰€杈撳嚭 i 鐨勫抚
   - `rxb_intf[i]_bytes`锛氬彂寰€杈撳嚭 i 鐨勫瓧鑺?   - `rxb_pbuf[i]_frames`锛氫粠鍐呴儴鍖呯紦鍐茶瑙掑彂寰€杈撳嚭 i 鐨勫抚
   - `rxb_pbuf[i]_bytes`锛氫粠鍐呴儴鍖呯紦鍐茶瑙掑彂寰€杈撳嚭 i 鐨勫瓧鑺?
#### RPC锛圧x 瑙ｆ瀽鍣級


 - `rpc_unkn_etype`锛氬寘鍚湭鐭?EtherType 鐨勫抚
 - `rpc_unkn_ext_hdr`锛氬寘鍚湭鐭?IPv6 鎵╁睍澶寸殑甯? - `rpc_ipv4_frag`锛氬寘鍚?IPv4 鍒嗙墖鐨勫抚
 - `rpc_ipv6_frag`锛氬寘鍚?IPv6 鍒嗙墖鐨勫抚
 - `rpc_ipv4_esp`锛氬甫鏈?IPv4 ESP 灏佽鐨勫抚
 - `rpc_ipv6_esp`锛氬甫鏈?IPv6 ESP 灏佽鐨勫抚
 - `rpc_tcp_opt_err`锛氶亣鍒?TCP 閫夐」瑙ｆ瀽閿欒鐨勫抚
 - `rpc_out_of_hdr_err`锛氬ご閮ㄥぇ浜庡彲瑙ｆ瀽鍖哄煙鐨勫抚
 - `ovr_size_err`锛氳秴闀垮抚

#### 纭欢闃熷垪


1. RX DMA 寮曟搸锛?
 - `rde_[i]_pkt_err`锛氬甫鏈?MAC EOP銆丷PC 瑙ｆ瀽鍣ㄣ€丷XB 鎴柇鎴?RDE 甯ф埅鏂敊璇殑鍖呫€傝繖浜涢敊璇湪鍖呭厓鏁版嵁涓爣璁帮紝鍥犱负鏀寔 cut-through锛屼絾瀹為檯涓㈠純鍙戠敓鍦ㄥ埌杈?PCIE/RDE 鏃躲€? - `rde_[i]_pkt_cq_drop`锛氬洜 RCQ 婊¤€岃涓㈠純鐨勫寘
 - `rde_[i]_pkt_bdq_drop`锛氬洜 HPQ 鎴?PPQ 鑰楀敖涓绘満缂撳啿鍖鸿€岃涓㈠純鐨勫寘

#### PCIe


fbnic 椹卞姩閫氳繃 debugfs锛坄pcie_stats`锛夋毚闇?PCIe 纭欢鎬ц兘缁熻淇℃伅銆傝繖浜涚粺璁′俊鎭湁鍔╀簬浜嗚В PCIe 浜嬪姟琛屼负鍜屾綔鍦ㄧ殑鎬ц兘鐡堕銆?
1. PCIe 浜嬪姟璁℃暟鍣細

   杩欎簺璁℃暟鍣ㄨ窡韪?PCIe 浜嬪姟娲诲姩锛?        - `pcie_ob_rd_tlp`锛氬嚭绔欒 TLP锛圱ransaction Layer Packets锛夎鏁?        - `pcie_ob_rd_dword`锛氬嚭绔欒浜嬪姟涓紶杈撶殑 DWORD 鏁?        - `pcie_ob_wr_tlp`锛氬嚭绔欏啓 TLP 璁℃暟
        - `pcie_ob_wr_dword`锛氬嚭绔欏啓浜嬪姟涓紶杈撶殑 DWORD 鏁?	  transactions
        - `pcie_ob_cpl_tlp`锛氬嚭绔欏畬鎴?TLP 璁℃暟
        - `pcie_ob_cpl_dword`锛氬嚭绔欏畬鎴?TLP 涓紶杈撶殑 DWORD 鏁?
2. PCIe 璧勬簮鐩戞帶锛?
   杩欎簺璁℃暟鍣ㄨ〃绀?PCIe 璧勬簮鑰楀敖浜嬩欢锛?        - `pcie_ob_rd_no_tag`锛氬洜 tag 涓嶅彲鐢ㄨ€屼涪寮冪殑璇昏姹?        - `pcie_ob_rd_no_cpl_cred`锛氬洜瀹屾垚淇＄敤锛坈ompletion
	  credit锛夎€楀敖鑰屼涪寮冪殑璇昏姹?        - `pcie_ob_rd_no_np_cred`锛氬洜闈炲彂甯冿紙non-posted锛変俊鐢ㄨ€楀敖
	  鑰屼涪寮冪殑璇昏姹?
#### XDP 闀垮害閿欒锛?

瀵逛簬涓嶆敮鎸?frags 鐨?XDP 绋嬪簭锛宖bnic 浼氬皾璇曠‘淇?MTU 鑳芥斁鍏ュ崟涓紦鍐插尯銆傚鏋滄敹鍒拌秴澶у抚骞惰鍒嗙墖锛屽畠灏嗚涓㈠純锛屽苟鏇存柊浠ヤ笅 netlink 璁℃暟鍣細

   - `rx-length`锛氱敱浜庢墍闄勫姞鐨?XDP 绋嬪簭缂哄皯鍒嗙墖鏀寔鑰岃涓㈠純鐨勫抚鏁?   - `rx-errors`锛氭帴鍙ｄ笂鏀跺埌鐨勯敊璇寘鎬绘暟
