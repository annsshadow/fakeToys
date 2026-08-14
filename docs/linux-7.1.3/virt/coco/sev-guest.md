
## 鏉冨▉鐨?SEV 瀹綋 API 鏂囨。


## 1. 鎬讳綋鎻忚堪


SEV API 鏄竴缁?ioctl锛岀敱瀹綋鎴栬櫄鎷熸満鐩戞帶鍣ㄧ敤鏉ヨ幏鍙栨垨璁剧疆 SEV 铏氭嫙鏈虹殑鏌愪釜鏂归潰銆傝繖浜?ioctl 灞炰簬浠ヤ笅绫诲埆锛?
 - 铏氭嫙鏈虹洃鎺у櫒 ioctl锛氳繖浜涙煡璇㈠苟璁剧疆褰卞搷鏁翠釜 SEV 鍥轰欢鐨勫叏灞€灞炴€с€傝繖浜?ioctl 鐢卞钩鍙?   閰嶇疆宸ュ叿浣跨敤銆?
 - 瀹綋 ioctl锛氳繖浜涙煡璇㈠苟璁剧疆 SEV 铏氭嫙鏈虹殑灞炴€с€?
## 2. API 鎻忚堪


鏈妭鎻忚堪鐢ㄤ簬浠?SEV 鍥轰欢鏌ヨ SEV 瀹綋鎶ュ憡鐨?ioctl銆傚浜庢瘡涓?ioctl锛岄櫎浜嗘弿杩板杩樻彁渚涗互涓?淇℃伅锛?
  Technology锛堟妧鏈級锛?      鐢卞摢涓?SEV 鎶€鏈彁渚涙 ioctl銆係EV銆丼EV-ES銆丼EV-SNP 鎴栧叏閮ㄣ€?
  Type锛堢被鍨嬶級锛?      铏氭嫙鏈虹洃鎺у櫒鎴栧浣撱€傝 ioctl 鍙互鍦ㄥ浣撴垨铏氭嫙鏈虹洃鎺у櫒鍐呴儴浣跨敤銆?
  Parameters锛堝弬鏁帮級锛?      璇?ioctl 鎺ュ彈鍝簺鍙傛暟銆?
  Returns锛堣繑鍥烇級锛?      杩斿洖鍊笺€備竴鑸殑閿欒鍙凤紙-ENOMEM銆?EINVAL锛変笉灞曞紑璇存槑锛屼絾鏈夌壒瀹氬惈涔夌殑閿欒浼氳鏄庛€?
瀹綋 ioctl 搴斿綋鍦ㄤ竴涓?/dev/sev-guest 璁惧鐨勬枃浠舵弿杩扮涓婂彂鍑恒€傝 ioctl 鎺ュ彈
struct snp_user_guest_request銆傝緭鍏ュ拰杈撳嚭缁撴瀯鍒嗗埆閫氳繃 req_data 鍜?resp_data 瀛楁鎸囧畾銆?濡傛灉 ioctl 鐢变簬鍥轰欢閿欒鑰屾墽琛屽け璐ワ紝鍒?fw_error 浠ｇ爜浼氳璁剧疆锛屽惁鍒?fw_error 浼氳璁句负 -1銆?
鍥轰欢浼氭鏌ユ秷鎭簭鍒楄鏁板櫒姣斿浣撶殑娑堟伅搴忓垪璁℃暟鍣ㄥぇ 1銆傚鏋滃浣撻┍鍔ㄦ湭鑳介€掑娑堟伅璁℃暟鍣?锛堜緥濡傝鏁板櫒婧㈠嚭锛夛紝鍒欒繑鍥?-EIO銆?```

        struct snp_guest_request_ioctl {
                /* 娑堟伅鐗堟湰鍙?*/
                __u32 msg_version;

                /* 璇锋眰鍜屽搷搴旂粨鏋勭殑鍦板潃 */
                __u64 req_data;
                __u64 resp_data;

                /* bits[63:32]: VMM 閿欒鐮? bits[31:0] 鍥轰欢閿欒鐮?(瑙?psp-sev.h) */
                union {
                        __u64 exitinfo2;
                        struct {
                                __u32 fw_error;
                                __u32 vmm_error;
                        };
                };
        };

```
涓绘満 ioctl 琚彂鍑哄埌涓€涓?/dev/sev 璁惧鐨勬枃浠舵弿杩扮銆傝 ioctl 鎺ュ彈濡備笅鎵€杩扮殑鍛戒护
ID/杈撳叆缁撴瀯銆?```

        struct sev_issue_cmd {
                /* 鍛戒护 ID */
                __u32 cmd;

                /* 鍛戒护璇锋眰缁撴瀯 */
                __u64 data;

                /* 澶辫触鏃剁殑鍥轰欢閿欒鐮?(瑙?psp-sev.h) */
                __u32 error;
        };


```
### 2.1 SNP_GET_REPORT

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in): struct snp_report_req
:Returns (out): struct snp_report_resp on success, -negative on error

SNP_GET_REPORT ioctl 鍙敤浜庝粠 SEV-SNP 鍥轰欢鏌ヨ璇佹槑锛坅ttestation锛夋姤鍛娿€傝 ioctl 浣跨敤
SEV-SNP 鍥轰欢鎻愪緵鐨?SNP_GUEST_REQUEST锛圡SG_REPORT_REQ锛夊懡浠ゆ潵鏌ヨ璇佹槑鎶ュ憡銆?
鎴愬姛鏃讹紝snp_report_resp.data 灏嗗寘鍚姤鍛娿€傛姤鍛婂寘鍚殑鏍煎紡鍦?SEV-SNP 瑙勮寖涓弿杩般€傛洿澶?缁嗚妭璇峰弬闃?SEV-SNP 瑙勮寖銆?
### 2.2 SNP_GET_DERIVED_KEY

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in): struct snp_derived_key_req
:Returns (out): struct snp_derived_key_resp on success, -negative on error

SNP_GET_DERIVED_KEY ioctl 鍙敤浜庤幏鍙栦粠涓€涓牴瀵嗛挜娲剧敓鐨勫瘑閽ャ€傛淳鐢熺殑瀵嗛挜鍙互琚浣撶敤浜?浠讳綍鐩殑锛屼緥濡傚瘑灏佸瘑閽ワ紙sealing keys锛夋垨涓庡閮ㄥ疄浣撻€氫俊銆?
璇?ioctl 浣跨敤 SEV-SNP 鍥轰欢鎻愪緵鐨?SNP_GUEST_REQUEST锛圡SG_KEY_REQ锛夊懡浠ゆ潵娲剧敓瀵嗛挜銆傚叧浜?瀵嗛挜娲剧敓璇锋眰涓紶鍏ョ殑鍚勪釜瀛楁鐨勬洿澶氱粏鑺傦紝璇峰弬闃?SEV-SNP 瑙勮寖銆?
鎴愬姛鏃讹紝snp_derived_key_resp.data 鍖呭惈娲剧敓鐨勫瘑閽ュ€笺€傛洿澶氱粏鑺傝鍙傞槄 SEV-SNP 瑙勮寖銆?
### 2.3 SNP_GET_EXT_REPORT

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in/out): struct snp_ext_report_req
:Returns (out): struct snp_report_resp on success, -negative on error

SNP_GET_EXT_REPORT ioctl 涓?SNP_GET_REPORT 绫讳技銆傚尯鍒湪浜庨殢鎶ュ憡涓€璧疯繑鍥炵殑棰濆璇佷功
鏁版嵁銆傝繑鍥炵殑璇佷功鏁版嵁鐢辫櫄鎷熸満鐩戞帶鍣ㄩ€氳繃 SNP_SET_EXT_CONFIG 鎻愪緵銆?
璇?ioctl 浣跨敤 SEV-SNP 鍥轰欢鎻愪緵鐨?SNP_GUEST_REQUEST锛圡SG_REPORT_REQ锛夊懡浠ゆ潵鑾峰彇璇佹槑鎶ュ憡銆?
鎴愬姛鏃讹紝snp_ext_report_resp.data 灏嗗寘鍚瘉鏄庢姤鍛婏紝snp_ext_report_req.certs_address 灏?鍖呭惈璇佷功 blob銆傚鏋?blob 鐨勯暱搴﹀皬浜庨鏈燂紝鍒?snp_ext_report_req.certs_len 浼氳鏇存柊涓?棰勬湡鍊笺€?
鍏充簬濡備綍瑙ｆ瀽璇佷功 blob 鐨勬洿澶氱粏鑺傦紝璇峰弬闃?GHCB 瑙勮寖銆?
### 2.4 SNP_PLATFORM_STATUS

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (out): struct sev_user_data_snp_status
:Returns (out): 0 on success, -negative on error

SNP_PLATFORM_STATUS 鍛戒护鐢ㄤ簬鏌ヨ SNP 骞冲彴鐘舵€併€傜姸鎬佸寘鎷?API 涓汇€佹鐗堟湰鍙风瓑銆傛洿澶氱粏鑺?璇峰弬闃?SEV-SNP 瑙勮寖銆?
### 2.5 SNP_COMMIT

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Returns (out): 0 on success, -negative on error

SNP_COMMIT 鐢ㄤ簬浣跨敤 SEV-SNP 鍥轰欢鐨?SNP_COMMIT 鍛戒护鎻愪氦褰撳墠宸插畨瑁呯殑鍥轰欢銆傝繖闃叉鍥炴粴鍒?涔嬪墠宸叉彁浜ょ殑鍥轰欢鐗堟湰銆傝繖涔熶細灏嗘姤鍛婄殑 TCB 鏇存柊涓轰笌褰撳墠宸插畨瑁呭浐浠剁浉鍖归厤銆?
### 2.6 SNP_SET_CONFIG

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (in): struct sev_user_data_snp_config
:Returns (out): 0 on success, -negative on error

SNP_SET_CONFIG 鐢ㄤ簬璁剧疆绯荤粺鑼冨洿鐨勯厤缃紝渚嬪璇佹槑鎶ュ憡涓姤鍛婄殑 TCB 鐗堟湰銆傝鍛戒护绫讳技浜?SEV-SNP 瑙勮寖涓畾涔夌殑 SNP_CONFIG 鍛戒护銆傚彈姝ゅ懡浠ゅ奖鍝嶇殑鍥轰欢鍙傛暟鐨勫綋鍓嶅€煎彲浠ラ€氳繃
SNP_PLATFORM_STATUS 鏌ヨ銆?
### 2.7 SNP_VLEK_LOAD

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (in): struct sev_user_data_snp_vlek_load
:Returns (out): 0 on success, -negative on error

鍦ㄨ姹傝瘉鏄庢姤鍛婃椂锛屽浣撹兘澶熸寚瀹氬畠鏄笇鏈?SNP 鍥轰欢浣跨敤鐢辫姱鐗囧敮涓€鏈哄瘑娲剧敓鐨勭増鏈寲鑺墖
绛炬敞瀵嗛挜锛圴CEK锛夋潵绛剧讲鎶ュ憡锛岃繕鏄娇鐢ㄤ粠 AMD 瀵嗛挜娲剧敓鏈嶅姟锛圞DS锛夎幏鍙栥€佸苟鐢卞垎閰嶇粰宸叉敞鍐?浜戞湇鍔℃彁渚涘晢鐨勭瀛愭淳鐢熺殑鐗堟湰鍖栧姞杞界娉ㄥ瘑閽ワ紙VLEK锛夈€?
瀵逛簬 VLEK 瀵嗛挜锛孲NP_VLEK_LOAD SNP 鍛戒护鐢ㄤ簬鍦ㄤ粠 KDS 鑾峰彇瀹冧滑涔嬪悗灏嗗叾鍔犺浇鍒扮郴缁熶腑锛屽苟涓?涓?SEV-SNP 瑙勮寖涓寚瀹氱殑 SNP_VLEK_LOAD 鍥轰欢鍛戒护瀵嗗垏鐩稿叧銆?
## 3. SEV-SNP CPUID 寮哄埗鎵ц


SEV-SNP 瀹綋鍙互璁块棶涓€涓壒娈婇〉锛屽叾涓寘鍚竴寮犵敱 PSP 鍦?SNP_LAUNCH_UPDATE 鍥轰欢鍛戒护杩囩▼涓?楠岃瘉杩囩殑 CPUID 鍊艰〃銆傚畠閽堝 CPUID 鍊肩殑鏈夋晥鎬ф彁渚涗互涓嬩繚璇侊細

 - 瀹冪殑鍦板潃閫氳繃寮曞鍔犺浇绋嬪簭/鍥轰欢锛堢粡鐢?CC blob锛夎幏寰楋紝閭ｄ簺浜岃繘鍒舵枃浠跺皢浣滀负 SEV-SNP
   璇佹槑鎶ュ憡鐨勪竴閮ㄥ垎琚害閲忋€? - 瀹冪殑鍒濆鐘舵€佷細琚姞瀵?pvalidated锛屽洜姝ゅ湪杩愯鏈熼棿璇曞浘淇敼瀹冧細瀵艰嚧鍐欏叆鍨冨溇鏁版嵁锛屾垨鑰?   濡傛灉铏氭嫙鏈虹洃鎺у櫒璇曞浘鏇挎崲鍚庡彴椤碉紝浼氬洜楠岃瘉鐘舵€佸彉鍖栬€屼骇鐢?#VC 寮傚父銆? - 铏氭嫙鏈虹洃鎺у櫒閫氳繃浣跨敤鏅€氶〉鎴栭潪 CPUID 鍔犲瘑椤垫潵缁曡繃 PSP 妫€鏌ョ殑灏濊瘯锛屼細鏀瑰彉 SEV-SNP
   璇佹槑鎶ュ憡鎻愪緵鐨勫害閲忋€? - CPUID 椤电殑鍐呭**涓?*琚害閲忥紝浣嗕綔涓哄浣撳垵濮嬪寲鐨勪竴閮ㄥ垎璇曞浘淇敼 CPUID 椤电殑棰勬湡鍐呭锛?   浼氳 PSP 鍦?SNP_LAUNCH_UPDATE 鏈熼棿瀵硅椤垫墽琛岀殑 PSP CPUID 寮哄埗鎵ц绛栫暐妫€鏌ユ墍鎷︽埅锛屽苟鍦?   涔嬪悗锛堝鏋滃浣撴墍鏈夎€呭疄鐜颁簡鑷繁瀵?CPUID 鍊肩殑妫€鏌ワ級鍙樺緱鏄庢樉銆?
闇€瑕佹敞鎰忕殑鏄紝鏈€鍚庤繖鏉′繚璇佸彧鏈夊湪鍐呮牳鍦ㄥ紩瀵肩殑鎵€鏈夐樁娈甸兘娉ㄦ剰浣跨敤 SEV-SNP CPUID 鏃舵墠鏈夌敤銆?鍚﹀垯锛屽浣撴墍鏈夎€呰瘉鏄庢棤娉曟彁渚涘唴鏍稿湪寮曞杩囩▼涓煇涓椂鍒绘病鏈夎鍠傚叆閿欒鍊肩殑淇濊瘉銆?
## 4. SEV 瀹綋椹卞姩閫氫俊瀵嗛挜


SEV 瀹綋涓?AMD 瀹夊叏澶勭悊鍣紙ASP锛屽嵆 PSP锛変腑鐨?SEV 鍥轰欢涔嬮棿鐨勯€氫俊鍙?VM 骞冲彴閫氫俊瀵嗛挜
锛圴MPCK锛変繚鎶ゃ€傞粯璁ゆ儏鍐典笅锛宻ev-guest 椹卞姩浣跨敤瀹綋杩愯鎵€鍦ㄧ殑 VM 鐗规潈绾э紙VMPL锛夊叧鑱旂殑
VMPCK銆傚鏋滆瀵嗛挜琚?sev-guest 椹卞姩鎿﹂櫎锛堝叧浜?VMPCK 鍙兘琚摝闄ょ殑鍘熷洜锛岃鍙傝椹卞姩锛夛紝鍙互
閫氳繃閲嶆柊鍔犺浇 sev-guest 椹卞姩骞朵娇鐢?vmpck_id 妯″潡鍙傛暟鎸囧畾鎵€闇€瀵嗛挜鏉ヤ娇鐢ㄤ笉鍚岀殑瀵嗛挜銆?
### 鍙傝€?

SEV-SNP 鍜?GHCB 瑙勮寖锛歞eveloper.amd.com/sev

璇ラ┍鍔ㄥ熀浜?SEV-SNP 鍥轰欢瑙勮寖 0.9 鍜?GHCB 瑙勮寖鐗堟湰 2.0銆?