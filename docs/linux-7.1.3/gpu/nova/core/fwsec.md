
## FWSEC锛堝浐浠跺畨鍏?/ Firmware Security锛?

鏈枃妗ｄ粠姒傚康涓婄畝瑕佹弿杩?FWSEC锛團irmware Security锛屽浐浠跺畨鍏級闀滃儚鍙婂叾鍦?GPU 鍚姩搴忓垪涓殑浣滅敤銆傚洜姝わ紝杩欎簺淇℃伅灏嗘潵鍙兘浼氬彂鐢熷彉鍖栵紝涓斾粎浠呮槸鎴嚦瀹夊煿锛圓mpere锛塆PU 绯诲垪鏃剁殑鎯呭喌銆備笉杩囷紝甯屾湜鍏朵腑鎻忚堪鐨勬蹇佃兘甯姪鐞嗚В鍐呮牳涓鐞嗗畠鐨勭浉鍏充唬鐮併€傛墍鏈変俊鎭潎鏉ヨ嚜鍏紑鍙敤鐨勮祫鏂欙紝渚嬪鍏紑鐨勯┍鍔ㄥ拰鏂囨。銆?
FWSEC 鐨勪綔鐢ㄦ槸鎻愪緵涓€涓畨鍏ㄥ惎鍔ㄨ繃绋嬨€傚畠杩愯鍦ㄢ€淗eavy-secure锛堥珮瀹夊叏锛夆€濇ā寮忎笅锛屽苟鍦?GPU 澶嶄綅鍚庛€佸皢鍚勭 ucode锛堝井鐮侊級闀滃儚鍔犺浇鍒板叾浠?GPU 寰帶鍒跺櫒锛堝 PMU 鍜?GSP锛変箣鍓嶏紝鎵ц鍥轰欢楠岃瘉銆?
FWSEC 鏈韩鏄竴涓瓨鍌ㄥ湪 VBIOS ROM 涓?ROM 鐨?FWSEC 鍒嗗尯閲岀殑搴旂敤绋嬪簭锛堣瑙?vbios.rst锛夈€傚畠鍖呭惈涓嶅悓鐨勫懡浠わ紝濡?FRTS锛團irmware Runtime Services锛屽浐浠惰繍琛屾椂鏈嶅姟锛夊拰 SB锛圫ecure Booting锛屽浣嶅悗瀹夊叏鍚姩鍏朵粬寰帶鍒跺櫒骞朵负瀹冧滑鍔犺浇闈?FWSEC 鐨勫叾浠?ucode锛夈€傚唴鏍搁┍鍔ㄥ彧闇€瑕佹墽琛?FRTS锛屽洜涓哄畨鍏ㄥ惎鍔紙SB锛夊湪椹卞姩鍔犺浇鏃跺凡缁忓畬鎴愩€?
FRTS 鍛戒护鍒掑垎鍑?WPR2 鍖哄煙锛堝啓淇濇姢鍖哄煙锛夛紝鍏朵腑鍖呭惈鐢垫簮绠＄悊鎵€闇€鐨勬暟鎹€備竴鏃﹁缃畬鎴愶紝鍙湁 HS锛圚igh Secure锛岄珮瀹夊叏锛夋ā寮忕殑 ucode 鎵嶈兘璁块棶瀹冿紙鐗规潈绾у埆璇﹁ falcon.rst锛夈€?
FWSEC 闀滃儚浣嶄簬 VBIOS ROM 涓寘鍚悇绉?ucode 闀滃儚锛堜篃绉颁负搴旂敤绋嬪簭锛夌殑鍒嗗尯涓€斺€斿叾涓箣涓€渚挎槸 FWSEC銆傚叧浜庡畠濡備綍琚彁鍙栵紝璇峰弬闃?vbios.rst 鍜?vbios.rs 婧愪唬鐮併€?
姣忎釜 ucode 闀滃儚锛堝寘鎷?FWSEC 闀滃儚锛夌殑 Falcon 鏁版嵁鐢卞ご閮ㄣ€佹暟鎹锛圖MEM锛夊拰鎸囦护浠ｇ爜娈碉紙IMEM锛夌粍鍚堣€屾垚銆傛墍鏈夎繖浜?ucode 闀滃儚閮藉瓨鍌ㄥ湪鍚屼竴涓?ROM 鍒嗗尯涓紝骞堕€氳繃 PMU 琛ㄦ牴鎹叾搴旂敤 ID锛坅pplication ID锛夋潵鏌ユ壘瑕佸姞杞界殑搴旂敤绋嬪簭锛堣瑙?vbios.rs锛夈€?
瀵逛簬 nova-core 椹卞姩锛孎WSEC 鍖呭惈涓€涓悕涓?DMEMMAPPER 鐨勨€滃簲鐢ㄧ▼搴忔帴鍙ｂ€濓紙application interface锛夈€傝鎺ュ彛闄や簡鍏朵粬鐢ㄩ€斿锛岃繕鐢ㄤ簬鎵ц鈥淔WSEC-FRTS鈥濆懡浠ゃ€傚浜庡畨鍩规灦鏋勶紝FWSEC 杩愯鍦?GSP 涓婄殑 Heavy-secure 妯″紡骞舵墽琛?FRTS銆?
### FWSEC 鍐呭瓨甯冨眬

```

   +---------------------------------------------------------------+
   |                         FWSEC ROM image (type 0xE0)           |
   |                                                               |
   |  +---------------------------------+                          |
   |  |     PMU Falcon Ucode Table      |                          |
   |  |     (PmuLookupTable)            |                          |
   |  |  +-------------------------+    |                          |
   |  |  | Table Header            |    |                          |
   |  |  | - version: 0x01         |    |                          |
   |  |  | - header_size: 6        |    |                          |
   |  |  | - entry_size: 6         |    |                          |
   |  |  | - entry_count: N        |    |                          |
   |  |  | - desc_version:3(unused)|    |                          |
   |  |  +-------------------------+    |                          |
   |  |         ...                     |                          |
   |  |  +-------------------------+    |                          |
   |  |  | Entry for FWSEC (0x85)  |    |                          |
   |  |  | (PmuLookupTableEntry)   |    |                          |
   |  |  | - app_id: 0x85 (FWSEC)  |----|----+                     |
   |  |  | - target_id: 0x01 (PMU) |    |    |                     |
   |  |  | - data: offset ---------|----|----|---+ look up FWSEC   |
   |  |  +-------------------------+    |    |   |                 |
   |  +---------------------------------+    |   |                 |
   |                                         |   |                 |
   |                                         |   |                 |
   |  +---------------------------------+    |   |                 |
   |  |     FWSEC Ucode Component       |<---+   |                 |
   |  |     (aka Falcon data)           |        |                 |
   |  |  +-------------------------+    |        |                 |
   |  |  | FalconUCodeDescV3       |<---|--------+                 |
   |  |  | - hdr                   |    |                          |
   |  |  | - stored_size           |    |                          |
   |  |  | - pkc_data_offset       |    |                          |
   |  |  | - interface_offset -----|----|----------------+         |
   |  |  | - imem_phys_base        |    |                |         |
   |  |  | - imem_load_size        |    |                |         |
   |  |  | - imem_virt_base        |    |                |         |
   |  |  | - dmem_phys_base        |    |                |         |
   |  |  | - dmem_load_size        |    |                |         |
   |  |  | - engine_id_mask        |    |                |         |
   |  |  | - ucode_id              |    |                |         |
   |  |  | - signature_count       |    |    look up sig |         |
   |  |  | - signature_versions --------------+          |         |
   |  |  +-------------------------+    |     |          |         |
   |  |         (no gap)                |     |          |         |
   |  |  +-------------------------+    |     |          |         |
   |  |  | Signatures Section      |<---|-----+          |         |
   |  |  | (384 bytes per sig)     |    |                |         |
   |  |  | - RSA-3K Signature 1    |    |                |         |
   |  |  | - RSA-3K Signature 2    |    |                |         |
   |  |  |   ...                   |    |                |         |
   |  |  +-------------------------+    |                |         |
   |  |                                 |                |         |
   |  |  +-------------------------+    |                |         |
   |  |  | IMEM Section (Code)     |    |                |         |
   |  |  |                         |    |                |         |
   |  |  | Contains instruction    |    |                |         |
   |  |  | code etc.               |    |                |         |
   |  |  +-------------------------+    |                |         |
   |  |                                 |                |         |
   |  |  +-------------------------+    |                |         |
   |  |  | DMEM Section (Data)     |    |                |         |
   |  |  |                         |    |                |         |
   |  |  | +---------------------+ |    |                |         |
   |  |  | | Application         | |<---|----------------+         |
   |  |  | | Interface Table     | |    |                          |
   |  |  | | (FalconAppifHdrV1)  | |    |                          |
   |  |  | | Header:             | |    |                          |
   |  |  | | - version: 0x01     | |    |                          |
   |  |  | | - header_size: 4    | |    |                          |
   |  |  | | - entry_size: 8     | |    |                          |
   |  |  | | - entry_count: N    | |    |                          |
   |  |  | |                     | |    |                          |
   |  |  | | Entries:            | |    |                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | | DEVINIT (ID 1)  | | |    |                          |
   |  |  | | | - id: 0x01      | | |    |                          |
   |  |  | | | - dmemOffset X -|-|-|----+                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | | DMEMMAPPER(ID 4)| | |    |                          |
   |  |  | | | - id: 0x04      | | |    | Used only for DevInit    |
   |  |  | | |  (NVFW_FALCON_  | | |    | application (not FWSEC)  |
   |  |  | | |   APPIF_ID_DMEMMAPPER)   |                          |
   |  |  | | - dmemOffset Y -|-|-|----|-----+                    |
   |  |  | +-----------------+ | |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |                         |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |  | DEVINIT Engine      |<|----+     | Used by FWSEC      |
   |  |  | Interface           | |    |     |         app.       |
   |  |  +---------------------+ |    |     |                    |
   |  |                         |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |  | DMEM Mapper (ID 4)  |<|----+-----+                    |
   |  |  | (FalconAppifDmemmapperV3)  |                          |
   |  |  | - signature: "DMAP" | |    |                          |
   |  |  | - version: 0x0003   | |    |                          |
   |  |  | - Size: 64 bytes    | |    |                          |
   |  |  | - cmd_in_buffer_off | |----|------------+             |
   |  |  | - cmd_in_buffer_size| |    |            |             |
   |  |  | - cmd_out_buffer_off| |----|------------|-----+       |
   |  |  | - cmd_out_buffer_sz | |    |            |     |       |
   |  |  | - init_cmd          | |    |            |     |       |
   |  |  | - features          | |    |            |     |       |
   |  |  | - cmd_mask0/1       | |    |            |     |       |
   |  |  +---------------------+ |    |            |     |       |
   |  |                         |    |            |     |       |
   |  |  +---------------------+ |    |            |     |       |
   |  |  | Command Input Buffer|<|----|------------+     |       |
   |  |  | - Command data      | |    |                  |       |
   |  |  | - Arguments         | |    |                  |       |
   |  |  +---------------------+ |    |                  |       |
   |  |                         |    |                  |       |
   |  |  +---------------------+ |    |                  |       |
   |  |  | Command Output      |<|----|------------------+       |
   |  |  | Buffer              | |    |                          |
   |  |  | - Results           | |    |                          |
   |  |  | - Status            | |    |                          |
   |  |  +---------------------+ |    |                          |
   |  +-------------------------+    |                          |
   |  +---------------------------------+                          |
   |                                                               |
   +---------------------------------------------------------------+

```
   浠ヤ笂浠?GA-102 瀹夊煿 GPU 涓轰緥锛屾湭鏉ョ殑 GPU 鍙兘浼氭湁鎵€涓嶅悓銆?
   FWSEC 闀滃儚杩樺湪鍐呭瓨鎿﹂櫎锛圗CC 鍒濆鍖栵級鍜?VPR锛圴ideo Protected Region锛岃棰戜繚鎶ゅ尯锛夊垵濮嬪寲涓彂鎸ヤ綔鐢ㄣ€傚湪 nova-core 椹卞姩鍔犺浇涔嬪墠锛孎WSEC 闀滃儚灏卞凡缁忚繍琛屽湪 GSP 涓婄殑 heavy-secure 妯″紡銆俤evinit 搴忓垪瀹屾垚鍚庯紝瀹冧細杩涜 VRAM 鍐呭瓨鎿﹂櫎锛圗CC 鍒濆鍖栵級銆傚湪娑堣垂绾?GPU 涓婏紝瀹冨彧鎿﹂櫎閮ㄥ垎鍐呭瓨锛岀劧鍚庡彂璧封€滃紓姝ユ摝闄も€濓紙async scrubbing锛夈€傚湪璇ュ紓姝ユ摝闄ゅ畬鎴愪箣鍓嶏紝鏈摝闄ょ殑 VRAM 涓嶈兘鐢ㄤ簬鍒嗛厤锛堝洜姝?DRM 鍐呭瓨鍒嗛厤鍣ㄩ渶瑕佺瓑寰呰鎿﹂櫎瀹屾垚锛夈€?