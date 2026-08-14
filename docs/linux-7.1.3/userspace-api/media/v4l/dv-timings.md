


######## 鏁板瓧瑙嗛锛圖V锛夋椂搴忥紙Digital Video Timings锛?

鍒扮洰鍓嶄负姝㈡墍璁ㄨ鐨勮棰戞爣鍑嗕竴鐩撮拡瀵规ā鎷熺數瑙嗭紙Analog TV锛夊強鍏剁浉搴旂殑瑙嗛鏃跺簭銆?濡備粖鏈変紬澶氫笉鍚岀殑纭欢鎺ュ彛锛屼緥濡傞珮娓呯數瑙嗘帴鍙ｏ紙HDMI锛夈€乂GA銆丏VI 杩炴帴鍣ㄧ瓑锛屽畠浠?鎵胯浇瑙嗛淇″彿锛屽洜姝ら渶瑕佹墿灞?API 鏉ヤ负杩欎簺鎺ュ彛閫夋嫨瑙嗛鏃跺簭銆傜敱浜庡彈闄愪簬鍙敤鐨勪綅鏁帮紝
鏃犳硶鎵╁睍 v4l2_std_id <v4l2-std-id>锛屽洜姝ゆ柊澧炰簡涓€缁?ioctl 鐢ㄤ簬鍦ㄨ緭鍏ヤ笌杈撳嚭
绔缃?鑾峰彇瑙嗛鏃跺簭銆?
杩欎簺 ioctl 澶勭悊瀹氫箟姣忕瑙嗛鏍煎紡鐨勫叿浣撴暟瀛楄棰戞椂搴忥紝鍖呮嫭娲诲姩瑙嗛瀹藉害涓庨珮搴︺€?淇″彿鏋佹€с€佸墠鑲╋紙frontporch锛夈€佸悗鑲╋紙backporch锛夈€佸悓姝ュ搴︾瓑鍙傛暟銆?`linux/v4l2-dv-timings.h` 澶存枃浠跺彲鐢ㄤ簬鑾峰彇 cea861 涓?vesadmt 鏍囧噯涓悇绉嶆牸寮忕殑
鏃跺簭銆?
涓轰簡鏋氫妇骞舵煡璇㈣澶囨墍鏀寔鐨?DV 鏃跺簭灞炴€э紝搴旂敤绋嬪簭浣跨敤 VIDIOC_ENUM_DV_TIMINGS 涓?VIDIOC_DV_TIMINGS_CAP ioctl銆傝璁剧疆璁惧鐨?DV 鏃跺簭锛屽簲鐢ㄧ▼搴忎娇鐢?VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl锛涜鑾峰彇褰撳墠鐨?DV 鏃跺簭锛屽垯浣跨敤
VIDIOC_G_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl銆傝妫€娴嬭棰戞帴鏀剁鐪嬪埌鐨?DV
鏃跺簭锛屽簲鐢ㄧ▼搴忎娇鐢?VIDIOC_QUERY_DV_TIMINGS ioctl銆?
褰撶‖浠舵娴嬪埌瑙嗛婧愬彂鐢熷彉鍖栵紙渚嬪瑙嗛淇″彿鍑虹幇鎴栨秷澶憋紝鎴栬棰戝垎杈ㄧ巼鏀瑰彉锛夋椂锛屽畠浼?鍙戝嚭涓€涓?`V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢銆備娇鐢?ioctl
VIDIOC_SUBSCRIBE_EVENT <VIDIOC_SUBSCRIBE_EVENT> 涓?VIDIOC_DQEVENT 鏉ユ鏌ヨ浜嬩欢
鏄惁宸茶涓婃姤銆?
濡傛灉瑙嗛淇″彿鍙戠敓鍙樺寲锛岄偅涔堝簲鐢ㄧ▼搴忓繀椤诲仠姝㈡祦浼犺緭銆侀噴鏀炬墍鏈夌紦鍐插尯锛屽苟璋冪敤
VIDIOC_QUERY_DV_TIMINGS 浠ヨ幏鍙栨柊鐨勮棰戞椂搴忥紱濡傛灉瀹冧滑鏈夋晥锛屽垯鍙互閫氳繃璋冪敤 ioctl
VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> 鏉ヨ缃畠浠€傝繖鍚屾椂涔熶細鏇存柊鏍煎紡锛屽洜姝?浣跨敤 ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 鏉ヨ幏鍙栨柊鏍煎紡銆傜幇鍦ㄥ簲鐢ㄧ▼搴忓彲浠ュ垎閰嶆柊鐨?缂撳啿鍖哄苟鍐嶆寮€濮嬫祦浼犺緭銆?
VIDIOC_QUERY_DV_TIMINGS 鍙細鎶ュ憡纭欢妫€娴嬪埌鐨勫唴瀹癸紝瀹冩案杩滀笉浼氭洿鏀归厤缃€傚鏋滃綋鍓?璁剧疆鐨勬椂搴忎笌瀹為檯妫€娴嬪埌鐨勬椂搴忎笉鍚岋紝閫氬父杩欐剰鍛崇潃浣犲皢鏃犳硶閲囬泦鍒颁换浣曡棰戙€傛纭殑
鍋氭硶鏄緷璧?`V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢锛屼互渚跨煡閬撲綍鏃跺彂鐢熶簡鍙樺寲銆?
搴旂敤绋嬪簭鍙互鍒╃敤 input-capabilities 涓?output-capabilities 鏍囧織鏉ュ垽鏂暟瀛楄棰?ioctl 鏄惁鍙敤浜庣粰瀹氱殑杈撳叆鎴栬緭鍑恒€?