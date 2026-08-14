## Metronomefb


Maintained by Jaya Kumar <jayakumar.lkml.gmail.com>

Last revised: Mar 10, 2008

Metronomefb 鏄竴涓敤浜?Metronome 鏄剧ず鎺у埗鍣ㄧ殑椹卞姩銆傝鎺у埗鍣ㄦ潵鑷?E-Ink Corporation銆傚畠鏃ㄥ湪鐢ㄤ簬椹卞姩 E-Ink Vizplex 鏄剧ず濯掍綋銆侲-Ink 鍦ㄥ叾缃戠珯涓婃墭绠′簡璇ユ帶鍒跺櫒鍜屾樉绀哄獟浣撶殑涓€浜涚粏鑺傦細http://www.e-ink.com/products/matrix/metronome.html 銆?
Metronome 閫氳繃 AMLCD 鎺ュ彛涓庝富鏈?CPU 杩炴帴銆備富鏈?CPU 鐢熸垚鎺у埗淇℃伅鍜屽浘鍍忥紝鏀惧湪涓€涓?framebuffer 涓紝闅忓悗閫氳繃鏌愮涓绘満鐗瑰畾鐨勬柟寮忎紶閫佸埌 AMLCD 鎺ュ彛銆傛樉绀虹姸鎬佸拰閿欒鐘舵€佸悇鑷€氳繃鐙珛鐨?GPIO 鎷夊彇銆?
Metronomefb 鏄笌骞冲彴鏃犲叧鐨勶紝渚濊禆浜庝竴涓澘绾х壒瀹氱殑椹卞姩鏉ュ畬鎴愭墍鏈夌墿鐞?IO 宸ヤ綔銆傜洰鍓嶏紝閽堝 AM-200 EPD 寮€鍙戝浠朵腑浣跨敤鐨?PXA 鏉垮疄鐜颁簡涓€涓ず渚嬨€傝绀轰緥鏄?am200epd.c銆?
Metronomefb 闇€瑕佹尝褰俊鎭紝璇ヤ俊鎭€氳繃 AMLCD 鎺ュ彛浼犻€佺粰 metronome 鎺у埗鍣ㄣ€傛尝褰俊鎭鏈熼€氳繃鍥轰欢绫伙紙firmware class锛夋帴鍙ｄ粠鐢ㄦ埛绌洪棿浼犻€併€傚彧瑕佷綘鐨?udev 鎴?hotplug 鑴氭湰鐭ラ亾鍦ㄤ紶閫佷箣鍓嶉渶瑕佽В鍘嬬缉锛屾尝褰㈡枃浠跺氨鍙互琚帇缂┿€俶etronomefb 浼氳姹?metronome.wbf锛屽畠閫氬父浼氭牴鎹?udev/hotplug 閰嶇疆鏀惧叆 /lib/firmware/metronome.wbf銆傛垜鍙敤杩囦竴涓渶鍒濇爣璁颁负 23P01201_60_WT0107_MTC 鐨勬尝褰㈡枃浠舵祴璇曡繃銆傛垜涓嶇煡閬撳畠浠ｈ〃浠€涔堝惈涔夈€傛搷浣滄尝褰㈡椂搴旇皑鎱庯紝鍥犱负瀹冨彲鑳藉鏄剧ず濯掍綋浜х敓鏌愪簺姘镐箙鎬х殑褰卞搷銆傛垜鏃㈡棤娉曡闂篃涓嶇‘鍒囩煡閬撹娉㈠舰瀵逛簬鐗╃悊濯掍綋鍏蜂綋璧蜂粈涔堜綔鐢ㄣ€?
Metronomefb 浣跨敤 deferred IO 鎺ュ彛锛屼互渚挎彁渚涗竴涓彲鍐呭瓨鏄犲皠鐨勫抚缂撳啿銆傚畠宸茬敤 tinyx锛圶fbdev锛夋祴璇曡繃銆傜洰鍓嶅凡鐭ュ畠鍙笌 xeyes銆亁clock銆亁loadimage銆亁pdf 涓€璧峰伐浣溿€?