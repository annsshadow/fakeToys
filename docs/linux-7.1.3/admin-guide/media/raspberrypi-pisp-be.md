## Raspberry Pi PiSP 鍚庣鍐呭瓨鍒板唴瀛?ISP锛坧isp-be锛?


## PiSP 鍚庣


PiSP 鍚庣鏄竴涓唴瀛樺埌鍐呭瓨鐨勫浘鍍忎俊鍙峰鐞嗗櫒锛圛SP锛夛紝瀹冧粠 DRAM 鍐呭瓨璇诲彇鍥惧儚鏁版嵁锛屽苟鏍规嵁搴旂敤绋嬪簭閫氳繃閰嶇疆缂撳啿鍖轰腑鐨勫弬鏁版墍鎸囧畾鐨勬柟寮忔墽琛屽浘鍍忓鐞嗭紝鐒跺悗閫氳繃涓や釜鐙珛鐨勮緭鍑洪€氶亾灏嗗儚绱犳暟鎹啓鍥炲唴瀛樸€?

ISP 瀵勫瓨鍣ㄤ笌缂栫▼妯″瀷璁板綍鍦?`Raspberry Pi Image Signal Processor (PiSP) Specification document`_ 涓€?

PiSP 鍚庣 ISP 浠ュ浘鍧楋紙tile锛夋柟寮忓鐞嗗浘鍍忋€傚浘鍍忓垎鍧楋紙tessellation锛夌殑澶勭悊浠ュ強搴曞眰閰嶇疆鍙傛暟鐨勮绠楋紝鐢变竴涓悕涓?`libpisp <https://github.com/raspberrypi/libpisp>`_ 鐨勮嚜鐢辫蒋浠跺簱瀹炵幇銆?

瀹屾暣鐨勫浘鍍忓鐞嗘祦姘寸嚎锛堝寘鎷€氳繃鍏煎 MIPI CSI-2 鐨勯噰闆嗘帴鍙ｄ粠鍥惧儚浼犳劅鍣ㄩ噰闆?RAW Bayer 鏁版嵁銆佸皢鍏跺瓨鍏?DRAM 鍐呭瓨锛屽苟鍦?PiSP 鍚庣涓繘琛屽鐞嗕互寰楀埌搴旂敤绋嬪簭鍙敤鐨勫浘鍍忥級鍦?`libcamera <https://libcamera.org>`_ 涓綔涓?Raspberry Pi 骞冲彴鏀寔鐨勪竴閮ㄥ垎瀹炵幇銆?

## pisp-be 椹卞姩


Raspberry Pi PiSP 鍚庣锛坧isp-be锛夐┍鍔ㄤ綅浜?drivers/media/platform/raspberrypi/pisp-be銆傚畠浣跨敤 `V4L2 API` 娉ㄥ唽鑻ュ共瑙嗛閲囬泦涓庤緭鍑鸿澶囷紝浣跨敤 `V4L2 subdev API` 娉ㄥ唽涓€涓繛鎺ヨ繖浜涜棰戣澶囩殑 ISP 瀛愯澶囷紝浠庤€屽舰鎴愮敱 `Media Controller (MC) API` 瀹炵幇鐨勫崟涓€濯掍綋鍥撅紙media graph锛夈€?

`pisp-be` 椹卞姩娉ㄥ唽鐨勫獟浣撴嫇鎵戝涓嬪浘鎵€绀猴細

    :alt:   榛樿濯掍綋娴佹按绾挎嫇鎵戝浘
    :align: center


濯掍綋鍥炬敞鍐屼簡浠ヤ笅瑙嗛璁惧鑺傜偣锛?

- pispbe-input锛氭彁浜ょ粰 ISP 杩涜澶勭悊鐨勫浘鍍忕殑杈撳嚭璁惧銆?
- pispbe-tdn_input锛氱敤浜庢椂鍩熷幓鍣紙temporal denoise锛夌殑杈撳嚭璁惧銆?
- pispbe-stitch_input锛氱敤浜庡浘鍍忔嫾鎺ワ紙HDR锛夌殑杈撳嚭璁惧銆?
- pispbe-output0锛氬鐞嗗悗鍥惧儚鐨勭涓€涓噰闆嗚澶囥€?
- pispbe-output1锛氬鐞嗗悗鍥惧儚鐨勭浜屼釜閲囬泦璁惧銆?
- pispbe-tdn_output锛氱敤浜庢椂鍩熷幓鍣殑閲囬泦璁惧銆?
- pispbe-stitch_output锛氱敤浜庡浘鍍忔嫾鎺ワ紙HDR锛夌殑閲囬泦璁惧銆?
- pispbe-config锛氱敤浜?ISP 閰嶇疆鍙傛暟鐨勮緭鍑鸿澶囥€?

### pispbe-input


寰?ISP 澶勭悊鐨勫浘鍍忚鎺掑叆 `pispbe-input` 杈撳嚭璁惧鑺傜偣銆傛湁鍏?ISP 杈撳叆鎵€鏀寔鐨勫浘鍍忔牸寮忓垪琛紝璇峰弬闃?`Raspberry Pi Image Signal Processor (PiSP) Specification document`_銆?

### pispbe-tdn_input, pispbe-tdn_output


`pispbe-tdn_input` 杈撳嚭瑙嗛璁惧鎺ユ敹寰呮椂鍩熷幓鍣潡澶勭悊鐨勫浘鍍忥紝杩欎簺鍥惧儚浠?`pispbe-tdn_output` 閲囬泦瑙嗛璁惧鑾峰彇銆傜敤鎴风┖闂磋礋璐ｇ淮鎶よ繖涓や釜璁惧涓婄殑闃熷垪锛屽苟纭繚杈撳嚭璁惧涓婂畬鎴愮殑缂撳啿鍖鸿鎺掑叆杈撳叆璁惧銆?

### pispbe-stitch_input, pispbe-stitch_output


涓哄疄鐜?HDR锛堥珮鍔ㄦ€佽寖鍥达級鍥惧儚澶勭悊锛屼娇鐢ㄥ浘鍍忔嫾鎺ヤ笌鑹茶皟鏄犲皠锛坱onemapping锛夊潡銆俙pispbe-stitch_output` 灏嗗浘鍍忓啓鍏ュ唴瀛橈紝鑰?`pispbe-stitch_input` 鎺ユ敹鍏堝墠鍐欏叆鐨勫抚锛屽皢鍏朵笌褰撳墠杈撳叆鍥惧儚涓€璧峰鐞嗐€傜敤鎴风┖闂磋礋璐ｇ淮鎶よ繖涓や釜璁惧涓婄殑闃熷垪锛屽苟纭繚杈撳嚭璁惧涓婂畬鎴愮殑缂撳啿鍖鸿鎺掑叆杈撳叆璁惧銆?

### pispbe-output0, pispbe-output1


杩欎袱涓噰闆嗚澶囧皢缁?ISP 澶勭悊鍚庣殑鍍忕礌鏁版嵁鍐欏叆鍐呭瓨銆?

### pispbe-config


`pispbe-config` 杈撳嚭瑙嗛璁惧鎺ユ敹涓€涓厤缃弬鏁板瓧娈碉紝璇ュ瓧娈靛畾涔変簡 ISP 寰呮墽琛岀殑鍥惧儚澶勭悊銆?

ISP 閰嶇疆鍙傛暟鐨勬牸寮忕敱 `pisp_be_tiles_config` C 缁撴瀯浣撳畾涔夛紝鍚勫弬鏁扮殑鍚箟鍦?`Raspberry Pi Image Signal Processor (PiSP) Specification document`_ 涓弿杩般€?

## ISP 閰嶇疆


ISP 閰嶇疆浠呯敱鍙傛暟缂撳啿鍖虹殑鍐呭鎻忚堪銆傜敤鎴风┖闂撮渶瑕佷娇鐢?V4L2 API 閰嶇疆鐨勫敮涓€鍙傛暟锛屾槸杈撳嚭涓庨噰闆嗚棰戣澶囦笂鐨勫浘鍍忔牸寮忥紝鐢ㄤ簬鏍￠獙鍙傛暟缂撳啿鍖哄唴瀹圭殑鍚堟硶鎬с€?
