## OMAP 3 鍥惧儚淇″彿澶勭悊鍣紙ISP锛夐┍鍔?

Copyright |copy| 2010 Nokia Corporation

Copyright |copy| 2009 Texas Instruments, Inc.

Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>,
Sakari Ailus <sakari.ailus@iki.fi>, David Cohen <dacohen@gmail.com>


### 绠€浠?

鏈枃妗ｆ弿杩颁簡浣嶄簬 drivers/media/platform/ti/omap3isp 鐨?Texas Instruments OMAP 3 鍥惧儚淇″彿
澶勭悊鍣紙ISP锛夐┍鍔ㄣ€傚師濮嬮┍鍔ㄧ敱 Texas Instruments 缂栧啓锛屼絾姝ゅ悗鍦?Nokia 琚噸鍐欙紙涓ゆ锛夈€?
璇ラ┍鍔ㄥ凡鎴愬姛鐢ㄤ簬浠ヤ笅鐗堟湰鐨?OMAP 3锛?
- 3430
- 3530
- 3630

璇ラ┍鍔ㄥ疄鐜颁簡 V4L2銆丮edia controller 鍜?v4l2_subdev 鎺ュ彛銆傛敮鎸佸唴鏍镐腑浣跨敤 v4l2_subdev 鎺ュ彛鐨?浼犳劅鍣ㄣ€侀暅澶村拰闂厜鐏┍鍔ㄣ€?

### 鎷嗗垎涓?subdev


OMAP 3 ISP 琚媶鍒嗕负 V4L2 subdev锛孖SP 鍐呴儴鐨勬瘡涓ā鍧楅兘鏈変竴涓?subdev 鏉ヨ〃绀哄畠銆傛瘡涓?subdev
閮藉悜鐢ㄦ埛绌洪棿鎻愪緵涓€涓?V4L2 subdev 鎺ュ彛銆?
- OMAP3 ISP CCP2
- OMAP3 ISP CSI2a
- OMAP3 ISP CCDC
- OMAP3 ISP preview
- OMAP3 ISP resizer
- OMAP3 ISP AEWB
- OMAP3 ISP AF
- OMAP3 ISP histogram

ISP 涓瘡涓彲鑳界殑閾炬帴閮界敱 Media controller 鎺ュ彛涓殑涓€涓摼鎺ユ潵寤烘ā銆傜ず渚嬬▼搴忚 [#]_銆?

### 鎺у埗 OMAP 3 ISP


涓€鑸€岃█锛屾彁渚涚粰 OMAP 3 ISP 鐨勮缃細鍦ㄤ笅涓€甯у紑濮嬫椂鐢熸晥銆傝繖鍙戠敓鍦ㄦā鍧楀湪浼犳劅鍣ㄧ殑鍨傜洿娑堥殣
鏈熼棿鍙樹负绌洪棽鏃躲€傚湪鍐呭瓨鍒板唴瀛樻搷浣滀腑锛屾祦姘寸嚎涓€娆¤繍琛屼竴甯с€傝缃殑搴旂敤鍦ㄥ抚涔嬮棿杩涜銆?
ISP 涓殑鎵€鏈夋ā鍧楋紝闄?CSI-2 浠ュ強鍙兘杩樻湁 CCP2 鎺ユ敹鍣ㄥ锛岄兘鍧氭寔鎺ユ敹瀹屾暣鐨勫抚銆傚洜姝や紶鎰熷櫒
缁濅笉鑳藉悜 ISP 鍙戦€佷笉瀹屾暣鐨勫抚銆?
鑷冲皯鍦?3430 涓婏紝autoidle 涓庢煇浜?ISP 妯″潡瀛樺湪闂銆俛utoidle 浠呭湪 3630 涓娿€佷笖 omap3isp 妯″潡
鍙傛暟 autoidle 闈為浂鏃跺惎鐢ㄣ€?
### 鎶€鏈弬鑰冩墜鍐岋紙TRM锛夊強鍏朵粬鏂囨。


OMAP 3430 TRM:
<URL:http://focus.ti.com/pdfs/wtbu/OMAP34xx_ES3.1.x_PUBLIC_TRM_vZM.zip>
Referenced 2011-03-05.

OMAP 35xx TRM:
<URL:http://www.ti.com/litv/pdf/spruf98o> Referenced 2011-03-05.

OMAP 3630 TRM:
<URL:http://focus.ti.com/pdfs/wtbu/OMAP36xx_ES1.x_PUBLIC_TRM_vQ.zip>
Referenced 2011-03-05.

DM 3730 TRM:
<URL:http://www.ti.com/litv/pdf/sprugn4h> Referenced 2011-03-06.


### 鍙傝€冩枃鐚?