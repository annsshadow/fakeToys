## 甯х紦鍐茶澶囧唴閮ㄦ満鍒?

杩欐槸鍏充簬甯х紦鍐茶澶囧唴閮ㄦ満鍒剁殑閮ㄥ垎鏂囨。鐨勫垵姝ヨ捣鐐广€?
Authors:

- Geert Uytterhoeven <geert@linux-m68k.org>, 1998 骞?7 鏈?21 鏃?- James Simmons <jsimmons@user.sf.net>, 2002 骞?11 鏈?26 鏃?
--------------------------------------------------------------------------------

## 甯х紦鍐茶澶?API 浣跨敤鐨勭粨鏋?

浠ヤ笅缁撴瀯鍦ㄥ抚缂撳啿璁惧鐨勮繍浣滀腑鍙戞尌浣滅敤銆傚畠浠畾涔変簬 <linux/fb.h>銆?
1. 鍐呮牳涔嬪锛堢敤鎴风┖闂达級

  - struct fb_fix_screeninfo

    鍏充簬甯х紦鍐茶澶囦笌鐗瑰畾瑙嗛妯″紡鐨勩€佽澶囨棤鍏充笖涓嶅彲鏀瑰彉鐨勪俊鎭€傚彲浠ラ€氳繃
    FBIOGET_FSCREENINFO ioctl 鑾峰彇銆?
  - struct fb_var_screeninfo

    鍏充簬甯х紦鍐茶澶囦笌鐗瑰畾瑙嗛妯″紡鐨勩€佽澶囨棤鍏充笖鍙敼鍙樼殑淇℃伅銆傚彲浠ラ€氳繃
    FBIOGET_VSCREENINFO ioctl 鑾峰彇锛屽苟閫氳繃 FBIOPUT_VSCREENINFO ioctl 鏇存柊銆傚鏋滃彧鎯?    骞崇Щ锛坧an锛夊睆骞曪紝鍙互浣跨敤 FBIOPAN_DISPLAY ioctl銆?
  - struct fb_cmap

    璁惧鏃犲叧鐨勮皟鑹叉澘锛坈olormap锛変俊鎭€傚彲浠ヤ娇鐢?FBIOGETCMAP 涓?FBIOPUTCMAP ioctl
    鑾峰彇鍜岃缃皟鑹叉澘銆?

2. 鍐呮牳涔嬪唴

  - struct fb_info

    鍏充簬鏌愪釜鐗瑰畾甯х紦鍐茶澶囧疄渚嬶紙鎻掓Ы鍙枫€佹澘鍗″湴鍧€绛夛級鐨勯€氱敤淇℃伅銆丄PI 涓庡簳灞備俊鎭€?
  - struct `par`

    璁惧鐩稿叧鐨勪俊鎭紝鍞竴瀹氫箟浜嗚繖鍧楃壒瀹氱‖浠剁殑瑙嗛妯″紡銆?

## 甯х紦鍐茶澶?API 浣跨敤鐨勮瑙夌被鍨嬶紙Visuals锛?

### 鍗曡壊锛團B_VISUAL_MONO01 涓?FB_VISUAL_MONO10锛?

姣忎釜鍍忕礌闈為粦鍗崇櫧銆?

### 浼僵鑹诧紙FB_VISUAL_PSEUDOCOLOR 涓?FB_VISUAL_STATIC_PSEUDOCOLOR锛?

鏁翠釜鍍忕礌鍊艰閫佸叆涓€涓彲缂栫▼鏌ユ壘琛紝璇ヨ〃涓烘瘡涓彲鑳界殑鍍忕礌鍊兼彁渚涗竴涓鑹诧紙鍖呮嫭绾€佺豢銆?钃濆己搴︼級锛屽苟鏄剧ず璇ラ鑹层€?

### 鐪熷僵鑹诧紙FB_VISUAL_TRUECOLOR锛?

鍍忕礌鍊艰鎷嗗垎涓虹孩銆佺豢銆佽摑瀛楁銆?

### 鐩存帴褰╄壊锛團B_VISUAL_DIRECTCOLOR锛?

鍍忕礌鍊艰鎷嗗垎涓虹孩銆佺豢銆佽摑瀛楁锛屾瘡涓瓧娈靛垎鍒湪鐙珛鐨勭孩銆佺豢銆佽摑鏌ユ壘琛ㄤ腑鏌ユ壘銆?

### 鐏板害鏄剧ず


鐏板害涓庨潤鎬佺伆搴︽槸浼僵鑹蹭笌闈欐€佷吉褰╄壊鐨勭壒娈婂彉浣擄紝鍏朵腑绾€佺豢銆佽摑鍒嗛噺濮嬬粓褰兼鐩哥瓑銆?