
## Marvell OcteonTx2 RVU 鍐呮牳椹卞姩


Copyright (c) 2020 Marvell International Ltd.

## 鐩綍


- `Overview`_
- `Drivers`_
- `Basic packet flow`_
- `Devlink health reporters`_
- `Quality of service`_
- `RVU representors`_

## 姒傝堪


Marvell 鐨?OcteonTX2 SOC 涓婄殑璧勬簮铏氭嫙鍖栧崟鍏冿紙RVU锛夊皢鏉ヨ嚜缃戠粶銆佸姞瀵嗕互鍙婂叾浠?鍔熻兘鍧楃殑纭欢璧勬簮鏄犲皠涓?PCI 鍏煎鐨勭墿鐞嗕笌铏氭嫙鍔熻兘銆傛瘡涓姛鑳藉潡鍙堟嫢鏈夊涓湰鍦?鍔熻兘锛圠Fs锛夛紝渚涘垎閰嶇粰 PCI 璁惧浣跨敤銆俁VU 鏀寔澶氫釜 PCIe SRIOV 鐗╃悊鍔熻兘锛圥Fs锛変笌
铏氭嫙鍔熻兘锛圴Fs锛夈€侾F0 琚О涓虹鐞?绠＄悊鍔熻兘锛圓F锛夛紝骞舵嫢鏈夊皢 RVU 鍔熻兘鍧楃殑 LFs 鍒嗛厤缁?鍚勪釜 PF/VF 鐨勭壒鏉冦€?
RVU 绠＄悊鐨勭綉缁滃姛鑳藉潡
 - 缃戠粶姹犳垨缂撳啿鍖哄垎閰嶅櫒锛圢PA锛? - 缃戠粶鎺ュ彛鎺у埗鍣紙NIX锛? - 缃戠粶瑙ｆ瀽鍣?CAM锛圢PC锛? - 璋冨害/鍚屾/鎺掑簭鍗曞厓锛圫SO锛? - 鍥炵幆鎺ュ彛锛圠BK锛?
RVU 绠＄悊鐨勯潪缃戠粶鍔熻兘鍧? - 鍔犲瘑鍔犻€熷櫒锛圕PT锛? - 璋冨害瀹氭椂鍣ㄥ崟鍏冿紙TIM锛? - 璋冨害/鍚屾/鎺掑簭鍗曞厓锛圫SO锛?   鍚屾椂鐢ㄤ簬缃戠粶涓庨潪缃戠粶鍦烘櫙

璧勬簮鍒嗛厤绀轰緥
 - 甯︽湁 NIX-LF 涓?NPA-LF 璧勬簮鐨?PF/VF 浣滀负绾綉缁滆澶囧伐浣? - 甯︽湁 CPT-LF 璧勬簮鐨?PF/VF 浣滀负绾姞瀵嗗嵏杞借澶囧伐浣?
RVU 鍔熻兘鍧楀彲鏍规嵁杞欢闇€姹傞珮搴﹂厤缃€?
鍥轰欢鍦ㄥ唴鏍稿惎鍔ㄥ墠瀹屾垚浠ヤ笅璁剧疆
 - 鏍规嵁鐗╃悊閾捐矾鐨勬暟閲忓惎鐢ㄦ墍闇€鏁伴噺鐨?RVU PF銆? - 姣忎釜 PF 鐨?VF 鏁伴噺鍦ㄧ紪璇戞椂鏄潤鎬佹垨鍙皟鐨勩€傛牴鎹厤缃紝鍥轰欢灏?VF 鍒嗛厤缁欏悇涓?   PF銆? - 鍚屾椂涓烘瘡涓?PF 涓?VF 鍒嗛厤 MSIX 鍚戦噺銆? - 杩欎簺鍦ㄥ唴鏍稿惎鍔ㄥ悗涓嶅啀鏀瑰彉銆?
## 椹卞姩


Linux 鍐呮牳灏嗕細鏈夊涓┍鍔ㄦ敞鍐屽埌 RVU 鐨勪笉鍚?PF 涓?VF銆傚氨缃戠粶鑰岃█锛屽皢浼氭湁 3 绉?椋庢牸鐨勯┍鍔ㄣ€?
### 绠＄悊鍔熻兘椹卞姩


濡備笂鎵€杩帮紝RVU PF0 琚О涓虹鐞嗗姛鑳斤紙AF锛夛紝璇ラ┍鍔ㄦ敮鎸佸姛鑳藉潡鐨勮祫婧愬垎閰嶄笌閰嶇疆銆?瀹冧笉澶勭悊浠讳綍 I/O銆傚畠璁剧疆灏戦噺鍩虹浜嬮」锛屼絾澶ч儴鍒嗗姛鑳芥槸閫氳繃鏉ヨ嚜 PF 涓?VF 鐨勯厤缃?璇锋眰鏉ュ疄鐜扮殑銆?
PF/VF 閫氳繃涓€娈靛叡浜唴瀛樺尯鍩燂紙閭锛変笌 AF 閫氫俊銆傛敹鍒拌姹傚悗锛孉F 杩涜璧勬簮鍒嗛厤浠ュ強
鍏朵粬纭欢閰嶇疆銆侫F 濮嬬粓鎸傛帴鍦ㄤ富鏈哄唴鏍镐笂锛屼絾 PF 鍙婂叾 VF 鍙兘鐢变富鏈哄唴鏍歌嚜韬娇鐢紝
鎴栬€呰鎸傛帴鍒?VM 鎴?DPDK 绛夌敤鎴风┖闂村簲鐢ㄧ▼搴忋€傚洜姝?AF 蹇呴』澶勭悊鏉ヨ嚜浠讳綍鍩熶腑浠讳綍
璁惧鍙戦€佺殑璧勬簮鍒嗛厤/閰嶇疆璇锋眰銆?
AF 椹卞姩杩樹笌搴曞眰鍥轰欢浜や簰浠? - 绠＄悊鐗╃悊浠ュお缃戦摼璺紝鍗?CGX LMAC銆? - 鑾峰彇閫熷害銆佸弻宸ャ€佽嚜鍗忓晢绛変俊鎭? - 鑾峰彇 PHY EEPROM 涓庣粺璁′俊鎭€? - 閰嶇疆 FEC銆丳AM 妯″紡
 - 绛夌瓑

浠庣函缃戠粶瑙掑害鐪嬶紝AF 椹卞姩鏀寔浠ヤ笅鍔熻兘銆? - 灏嗙墿鐞嗛摼璺槧灏勫埌娉ㄥ唽浜?netdev 鐨?RVU PF銆? - 灏?NIX 涓?NPA 鍧楃殑 LFs 鎸傛帴鍒?RVU PF/VF锛屼互鎻愪緵鐢ㄤ簬甯歌缃戠粶鍔熻兘鐨勭紦鍐插尯姹犮€?   RQ銆丼Q銆? - 娴佹帶锛堟殏鍋滃抚锛夌殑鍚敤/绂佺敤/閰嶇疆銆? - 涓庣‖浠?PTP 鏃堕棿鎴崇浉鍏崇殑閰嶇疆銆? - NPC 瑙ｆ瀽鍣ㄩ厤缃枃浠堕厤缃紝鍗冲浣曡В鏋愭暟鎹寘浠ュ強鎻愬彇浠€涔堜俊鎭€? - NPC 鎻愬彇閰嶇疆鏂囦欢閰嶇疆锛屽嵆浠庢暟鎹寘涓彁鍙栦粈涔堝唴瀹逛互鍖归厤 MCAM 琛ㄩ」涓殑鏁版嵁銆? - 绠＄悊 NPC MCAM 琛ㄩ」锛屽湪鏀跺埌璇锋眰鏃跺彲浠ヤ负璇锋眰鐨勫寘杞彂瑙勫垯鏋勫缓骞跺畨瑁呫€? - 瀹氫箟鎺ユ敹绔缉鏀撅紙RSS锛夌畻娉曘€? - 瀹氫箟鍒嗘鍗歌浇绠楁硶锛堝 TSO锛? - VLAN 鍓ョ銆佹崟鑾蜂笌鎻掑叆閰嶇疆銆? - SSO 涓?TIM 鍧楅厤缃紝鎻愪緵鍖呰皟搴︽敮鎸併€? - Debugfs 鏀寔锛岀敤浜庢鏌ュ綋鍓嶈祫婧愬垎閰嶃€丯PA 姹犮€丯IX RQ銆丼Q 涓?CQ 鐨勫綋鍓嶇姸鎬併€?   鍚勭缁熻淇℃伅绛夛紝浠ュ府鍔╄皟璇曢棶棰樸€? - 浠ュ強鏇村銆?
### 鐗╃悊鍔熻兘椹卞姩


璇?RVU PF 澶勭悊 IO锛岃鏄犲皠鍒颁竴涓墿鐞嗕互澶綉閾捐矾锛屽苟涓旇椹卞姩娉ㄥ唽涓€涓?netdev銆傚畠
鏀寔 SR-IOV銆傚涓婃墍杩帮紝璇ラ┍鍔ㄩ€氳繃閭涓?AF 閫氫俊銆備负浜嗕粠鐗╃悊閾捐矾鑾峰彇淇℃伅锛岃
椹卞姩涓?AF 浜よ皥锛孉F 鍐嶄粠鍥轰欢鑾峰彇淇℃伅骞跺洖搴斿洖鏉ワ紝鍗冲畠涓嶈兘鐩存帴涓庡浐浠朵氦璋堛€?
鏀寔 ethtool 鐢ㄤ簬閰嶇疆閾捐矾銆丷SS銆侀槦鍒楁暟閲忋€侀槦鍒楀ぇ灏忋€佹祦鎺с€乶tuple 杩囨护鍣ㄣ€佽浆鍌?PHY EEPROM銆侀厤缃?FEC 绛夈€?
### 铏氭嫙鍔熻兘椹卞姩


鏈変袱绉嶇被鍨嬬殑 VF锛屼笌鍏剁埗 SR-IOV PF 鍏变韩鐗╃悊閾捐矾鐨?VF锛屼互鍙婁娇鐢ㄥ唴閮ㄧ‖浠跺洖鐜€氶亾
锛圠BK锛夋垚瀵瑰伐浣滅殑 VF銆?
绫诲瀷 1锛? - 杩欎簺 VF 鍙婂叾鐖?PF 鍏变韩涓€鏉＄墿鐞嗛摼璺紝鐢ㄤ簬涓庡閮ㄩ€氫俊銆? - VF 涓嶈兘鐩存帴涓?AF 閫氫俊锛屽畠浠皢 mbox 娑堟伅鍙戦€佺粰 PF锛孭F 鍐嶅皢鍏惰浆鍙戠粰 AF銆侫F 澶勭悊
   涔嬪悗锛屽皢鍥炲簲杩斿洖缁?PF锛孭F 鍐嶅皢鍥炲杞彂缁?VF銆? - 浠庡姛鑳借搴︾湅锛孭F 涓?VF 涔嬮棿娌℃湁鍖哄埆锛屽洜涓虹浉鍚岀殑纭欢璧勬簮琚寕鎺ュ埌涓よ€呫€備絾鐢ㄦ埛
   鍙兘浠?PF 閰嶇疆灏戦噺鍐呭锛屽洜涓?PF 琚涓洪摼璺殑鎵€鏈夎€?绠＄悊鍛樸€?
绫诲瀷 2锛? - RVU PF0锛屽嵆绠＄悊鍔熻兘锛屽垱寤鸿繖浜?VF 骞跺皢瀹冧滑鏄犲皠鍒板洖鐜潡鐨勯€氶亾銆? - 涓€缁勪袱涓?VF锛圴F0 涓?VF1銆乂F2 涓?VF3鈥︹€︿緷姝ょ被鎺級鎴愬宸ヤ綔锛屽嵆浠?VF0 鍙戝嚭鐨勫寘
   浼氳 VF1 鎺ユ敹锛屽弽涔嬩害鐒躲€? - 杩欎簺 VF 鍙搴旂敤绋嬪簭鎴栬櫄鎷熸満鐢ㄦ潵鍦ㄥ畠浠箣闂撮€氫俊鑰屾棤闇€灏嗘祦閲忓彂寰€澶栭儴銆傜‖浠朵腑
   涓嶅瓨鍦ㄤ氦鎹㈡満锛屽洜姝ゆ彁渚涗簡瀵瑰洖鐜?VF 鐨勬敮鎸併€? - 杩欎簺 VF 閫氳繃 mbox 鐩存帴涓?AF锛圥F0锛夐€氫俊銆?
闄や簡鐢ㄤ簬鍖呮敹鍙戞墍鐢ㄧ殑 IO 閫氶亾鎴栭摼璺箣澶栵紝杩欎簺 VF 绫诲瀷涔嬮棿娌℃湁鍏朵粬鍖哄埆銆侫F 椹卞姩
璐熻矗 IO 閫氶亾鏄犲皠锛屽洜姝ゅ悓涓€涓?VF 椹卞姩瀵逛袱绫昏澶囬兘鑳藉伐浣溿€?
## 鍩烘湰鍖呮祦


### 鍏ュ悜


1. CGX LMAC 鎺ユ敹鏁版嵁鍖呫€?2. 灏嗘暟鎹寘杞彂缁?NIX 鍧椼€?3. 闅忓悗鎻愪氦缁?NPC 鍧楄繘琛岃В鏋愶紝鍐嶈繘琛?MCAM 鏌ユ壘浠ヨ幏寰楃洰鏍?RVU 璁惧銆?4. 鎸傛帴鍒扮洰鏍?RVU 璁惧鐨?NIX LF 浠?NPA 鍧?LF 鐨?RQ 鏄犲皠缂撳啿鍖烘睜涓垎閰嶄竴涓紦鍐插尯銆?5. RQ 鍙敱 RSS 閫夋嫨锛屾垨閫氳繃閰嶇疆甯?RQ 鍙风殑 MCAM 瑙勫垯鏉ラ€夋嫨銆?6. 鏁版嵁鍖呰 DMA锛屽苟閫氱煡椹卞姩銆?
### 鍑哄悜


1. 椹卞姩鍑嗗涓€涓彂閫佹弿杩扮骞舵彁浜ょ粰 SQ 浠ヨ繘琛屼紶杈撱€?2. 璇?SQ 宸茶锛圓F锛夐厤缃负鍦ㄧ壒瀹氶摼璺?閫氶亾涓婁紶杈撱€?3. SQ 鎻忚堪绗︾幆鐢变粠 NPA 鍧?LF 鐨?SQ 鏄犲皠姹犱腑鍒嗛厤鐨勭紦鍐插尯缁存姢銆?4. NIX 鍧楀湪鎸囧畾閫氶亾涓婁紶杈撹鍖呫€?5. 鍙互瀹夎 NPC MCAM 琛ㄩ」浠ュ皢鍖呮敼閬撳埌涓嶅悓鐨勯€氶亾銆?
## Devlink 鍋ュ悍鎶ュ憡鍣?

### NPA 鎶ュ憡鍣?

NPA 鎶ュ憡鍣ㄨ礋璐ｆ姤鍛婂苟鎭㈠浠ヤ笅涓€缁勯敊璇細

1. GENERAL 浜嬩欢

   - 鍥犳湭鏄犲皠 PF 鐨勬搷浣滃鑷寸殑閿欒銆?   - 鍥犲叾浠?HW 鍧楋紙NIX銆丼SO銆乀IM銆丏PI 涓?AURA锛夌殑鍒嗛厤/閲婃斁琚鐢ㄥ鑷寸殑閿欒銆?
2. ERROR 浜嬩欢

   - 鍥?NPA_AQ_INST_S 璇绘垨 NPA_AQ_RES_S 鍐欏鑷寸殑鏁呴殰銆?   - AQ Doorbell 閿欒銆?
3. RAS 浜嬩欢

   - 閽堝 NPA_AQ_INST_S/NPA_AQ_RES_S 鐨?RAS 閿欒鎶ュ憡銆?
4. RVU 浜嬩欢

   - 鍥犳湭鏄犲皠妲戒綅瀵艰嚧鐨勯敊璇€?
```

	~# devlink health
	pci/0002:01:00.0:
	  reporter hw_npa_intr
	      state healthy error 2872 recover 2872 last_dump_date 2020-12-10 last_dump_time 09:39:09 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_gen
	      state healthy error 2872 recover 2872 last_dump_date 2020-12-11 last_dump_time 04:43:04 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_err
	      state healthy error 2871 recover 2871 last_dump_date 2020-12-10 last_dump_time 09:39:17 grace_period 0 auto_recover true auto_dump true
	   reporter hw_npa_ras
	      state healthy error 0 recover 0 last_dump_date 2020-12-10 last_dump_time 09:32:40 grace_period 0 auto_recover true auto_dump true

```
姣忎釜鎶ュ憡鍣ㄨ浆鍌ㄥ嚭

 - 閿欒绫诲瀷
 - 閿欒瀵勫瓨鍣ㄥ€? - 鏂囧瓧褰㈠紡鐨勭紭鐢?
```

	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_gen
	 NPA_AF_GENERAL:
	         NPA General Interrupt Reg : 1
	         NIX0: free disabled RX
	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_intr
	 NPA_AF_RVU:
	         NPA RVU Interrupt Reg : 1
	         Unmap Slot Error
	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_err
	 NPA_AF_ERR:
	        NPA Error Interrupt Reg : 4096
	        AQ Doorbell Error


```
### NIX 鎶ュ憡鍣?

NIX 鎶ュ憡鍣ㄨ礋璐ｆ姤鍛婂苟鎭㈠浠ヤ笅涓€缁勯敊璇細

1. GENERAL 浜嬩欢

   - 鍥犵紦鍐插尯涓嶈冻瀵艰嚧鐨勬帴鏀堕暅鍍?缁勬挱鍖呬涪寮冦€?   - SMQ Flush 鎿嶄綔銆?
2. ERROR 浜嬩欢

   - 鍥犱粠缁勬挱/闀滃儚缂撳啿鍖鸿鍐?WQE 瀵艰嚧鐨勫唴瀛橀敊璇€?   - 鎺ユ敹缁勬挱/闀滃儚澶嶅埗鍒楄〃閿欒銆?   - 鍦ㄦ湭鏄犲皠鐨?PF 涓婃帴鏀舵暟鎹寘銆?   - 鍥?NIX_AQ_INST_S 璇绘垨 NIX_AQ_RES_S 鍐欏鑷寸殑鏁呴殰銆?   - AQ Doorbell 閿欒銆?
3. RAS 浜嬩欢

   - 閽堝 NIX 鎺ユ敹缁勬挱/闀滃儚鏉＄洰缁撴瀯鐨?RAS 閿欒鎶ュ憡銆?   - 閽堝浠庣粍鎾?闀滃儚缂撳啿鍖鸿鍑虹殑 WQE/鍖呮暟鎹殑 RAS 閿欒鎶ュ憡銆?   - 閽堝 NIX_AQ_INST_S/NIX_AQ_RES_S 鐨?RAS 閿欒鎶ュ憡銆?
4. RVU 浜嬩欢

   - 鍥犳湭鏄犲皠妲戒綅瀵艰嚧鐨勯敊璇€?
```

	~# ./devlink health
	pci/0002:01:00.0:
	  reporter hw_npa_intr
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_gen
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_err
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_ras
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_intr
	    state healthy error 1121 recover 1121 last_dump_date 2021-01-19 last_dump_time 05:42:26 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_gen
	    state healthy error 949 recover 949 last_dump_date 2021-01-19 last_dump_time 05:42:43 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_err
	    state healthy error 1147 recover 1147 last_dump_date 2021-01-19 last_dump_time 05:42:59 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_ras
	    state healthy error 409 recover 409 last_dump_date 2021-01-19 last_dump_time 05:43:16 grace_period 0 auto_recover true auto_dump true

```
姣忎釜鎶ュ憡鍣ㄨ浆鍌ㄥ嚭

 - 閿欒绫诲瀷
 - 閿欒瀵勫瓨鍣ㄥ€? - 鏂囧瓧褰㈠紡鐨勭紭鐢?
```

	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_intr
	 NIX_AF_RVU:
	        NIX RVU Interrupt Reg : 1
	        Unmap Slot Error
	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_gen
	 NIX_AF_GENERAL:
	        NIX General Interrupt Reg : 1
	        Rx multicast pkt drop
	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_err
	 NIX_AF_ERR:
	        NIX Error Interrupt Reg : 64
	        Rx on unmapped PF_FUNC


```
## 鏈嶅姟璐ㄩ噺


### 璋冨害涓娇鐢ㄧ殑纭欢绠楁硶


octeontx2 纭呯墖涓?CN10K 浼犺緭鎺ュ彛鐢变簲涓紶杈撳眰绾х粍鎴愶紝浠?SMQ/MDQ銆乀L4 鍒?TL1銆傛瘡涓?鏁版嵁鍖呬細閬嶅巻 MDQ銆乀L4 鍒?TL1 鍚勫眰绾с€傛瘡涓眰绾у寘鍚竴涓槦鍒楁暟缁勪互鏀寔璋冨害涓庢暣褰€?纭欢鏍规嵁璋冨害鍣ㄩ槦鍒楃殑浼樺厛绾т娇鐢ㄤ笅杩扮畻娉曘€備竴鏃︾敤鎴峰垱寤轰簡鍏锋湁涓嶅悓浼樺厛绾х殑 tc 绫伙紝
椹卞姩灏辩敤鎸囧畾鐨勪紭鍏堢骇浠ュ強閫熺巼闄愬埗閰嶇疆鏉ラ厤缃垎閰嶇粰璇ョ被鐨勮皟搴﹀櫒銆?
1. 涓ユ牸浼樺厛绾?
      - 涓€鏃﹀寘琚彁浜ょ粰 MDQ锛岀‖浠朵細浣跨敤涓ユ牸浼樺厛绾ч€夊彇鎵€鏈夊叿鏈変笉鍚屼紭鍏堢骇鐨勬椿璺?MDQ銆?
2. 杞锛圧ound Robin锛?
      - 鍏锋湁鐩稿悓浼樺厛绾х殑娲昏穬 MDQ 浣跨敤杞鏂瑰紡閫夊彇銆?

### 閰嶇疆 HTB 鍗歌浇


```

        # ethtool -K <interface> hw-tc-offload on

```
```

        # tc qdisc add dev <interface> clsact
        # tc qdisc replace dev <interface> root handle 1: htb offload

```
```

        # tc class add dev <interface> parent 1: classid 1:1 htb rate 10Gbit prio 1

        # tc class add dev <interface> parent 1: classid 1:2 htb rate 10Gbit prio 7

```
```

        # tc class add dev <interface> parent 1: classid 1:1 htb rate 10Gbit prio 2 quantum 409600

        # tc class add dev <interface> parent 1: classid 1:2 htb rate 10Gbit prio 2 quantum 188416

        # tc class add dev <interface> parent 1: classid 1:3 htb rate 10Gbit prio 2 quantum 32768


```
## RVU Representors


RVU representor 椹卞姩娣诲姞浜嗗绯荤粺涓负 RVU PF 鐨?VF 鍒涘缓 representor 璁惧鐨勬敮鎸併€?褰撶敤鎴峰惎鐢?switchdev 妯″紡鏃讹紝浼氬垱寤?representor 璁惧銆傚湪璁剧疆 SRIOV numVFs 涔嬪墠鎴?涔嬪悗閮藉彲浠ュ惎鐢?switchdev 妯″紡銆傛墍鏈?representor 璁惧鍏变韩鍗曚釜 NIXLF锛屼絾姣忎釜閮芥嫢鏈?涓撶敤鐨?Rx/Tx 闃熷垪銆俁VU PF representor 椹卞姩涓烘瘡涓?Rx/Tx 闃熷垪瀵规敞鍐屼竴涓嫭绔嬬殑 netdev銆?
褰撳墠纭欢涓嶆敮鎸佽兘澶熻繘琛?L2 瀛︿範涓庡湪 representee 涓?representor 涔嬮棿杞彂鏁版嵁鍖呯殑
鍐呯疆浜ゆ崲鏈恒€傚洜姝わ紝representee 涓庡叾 representor 涔嬮棿鐨勫寘璺緞鏄€氳繃璁剧疆鍚堥€傜殑 NPC
MCAM 杩囨护鍣ㄥ疄鐜扮殑銆傚尮閰嶈繖浜涜繃婊ゅ櫒鐨勪紶杈撴暟鎹寘浼氶€氳繃纭欢鍥炵幆閫氶亾/鎺ュ彛锛堝嵆锛岃€岄潪
浠?MAC 鎺ュ彛鍙戝線澶栭儴锛夎鍥炵幆銆傝繖浼氬啀娆″尮閰嶅凡瀹夎鐨勮繃婊ゅ櫒骞惰杞彂銆備互姝ゆ柟寮忓疄鐜?representee => representor 浠ュ強 representor => representee 鐨勫寘璺緞銆傝繖浜涜鍒欏湪
representor 琚垱寤烘椂瀹夎锛屽苟鏍规嵁 representor/representee 鎺ュ彛鐘舵€佽€屾縺娲?鍋滅敤銆?
浣跨敤绀轰緥锛?
```

	# devlink dev eswitch set pci/0002:1c:00.0 mode switchdev

 - List of representor devices on the system::

	# ip link show
	Rpf1vf0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether f6:43:83:ee:26:21 brd ff:ff:ff:ff:ff:ff
	Rpf1vf1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 12:b2:54:0e:24:54 brd ff:ff:ff:ff:ff:ff
	Rpf1vf2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 4a:12:c4:4c:32:62 brd ff:ff:ff:ff:ff:ff
	Rpf1vf3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether ca:cb:68:0e:e2:6e brd ff:ff:ff:ff:ff:ff
	Rpf2vf0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 06:cc:ad:b4:f0:93 brd ff:ff:ff:ff:ff:ff


```
瑕佷粠绯荤粺涓垹闄?representor 璁惧锛屽皢璁惧鍒囨崲涓?legacy 妯″紡銆?
```

	# devlink dev eswitch set pci/0002:1c:00.0 mode legacy

```
RVU representors 鍙互浣跨敤 devlink 绔彛
锛堝弬瑙?Documentation/networking/devlink/devlink-port.rst <devlink_port>锛夋帴鍙ｈ繘琛岀鐞嗐€?
```

	# devlink port
	pci/0002:1c:00.0/0: type eth netdev Rpf1vf0 flavour physical port 0 splittable false
	pci/0002:1c:00.0/1: type eth netdev Rpf1vf1 flavour pcivf controller 0 pfnum 1 vfnum 1 external false splittable false
	pci/0002:1c:00.0/2: type eth netdev Rpf1vf2 flavour pcivf controller 0 pfnum 1 vfnum 2 external false splittable false
	pci/0002:1c:00.0/3: type eth netdev Rpf1vf3 flavour pcivf controller 0 pfnum 1 vfnum 3 external false splittable false

```
## 鍔熻兘灞炴€?

RVU representor 鏀寔 representor 鐨勫姛鑳藉睘鎬с€俽epresentor 鐨勭鍙ｅ姛鑳介厤缃€氳繃 devlink
eswitch 绔彛鏀寔銆?
### MAC 鍦板潃閰嶇疆


RVU representor 椹卞姩鏀寔閫氳繃 devlink 绔彛鍔熻兘灞炴€ф満鍒舵潵閰嶇疆 MAC 鍦板潃銆傦紙鍙傝
Documentation/networking/devlink/devlink-port.rst锛?
```

	# devlink port function set pci/0002:1c:00.0/2 hw_addr 5c:a1:1b:5e:43:11
	# devlink port show pci/0002:1c:00.0/2
	pci/0002:1c:00.0/2: type eth netdev Rpf1vf2 flavour pcivf controller 0 pfnum 1 vfnum 2 external false splittable false
	function:
		hw_addr 5c:a1:1b:5e:43:11


```
## TC 鍗歌浇


rvu representor 椹卞姩瀹炵幇浜嗕娇鐢ㄧ鍙?representor 鍗歌浇 tc 瑙勫垯鐨勬敮鎸併€?
```

	# tc filter add dev Rpf1vf0 protocol 802.1Q parent ffff: flower vlan_id 3 vlan_ethtype ipv4 skip_sw action drop

 - Redirect packets with vlan id 5 and IPv4 packets to eth1, after stripping vlan header.::

	# tc filter add dev Rpf1vf0 ingress protocol 802.1Q flower vlan_id 5 vlan_ethtype ipv4 skip_sw action vlan pop action mirred ingress redirect dev eth1

```
