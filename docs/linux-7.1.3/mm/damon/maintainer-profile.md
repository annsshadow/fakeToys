
## DAMON 缁存姢鑰呮潯鐩。妗?
DAMON 瀛愮郴缁熻鐩?`MAINTAINERS` 鏂囦欢涓?`DAMON` 绔犺妭鎵€鍒楃殑鏂囦欢銆?
璇ュ瓙绯荤粺鐨勯偖浠跺垪琛ㄤ负 damon@lists.linux.dev 涓?linux-mm@kvack.org銆傝ˉ涓佸簲灏藉彲鑳藉熀浜?`mm-new tree
<https://git.kernel.org/akpm/mm/h/mm-new>`_ 鍒朵綔锛屽苟鍙戝竷鍒伴偖浠跺垪琛ㄣ€?
### SCM 鏍?
DAMON 寮€鍙戞湁澶氫釜 Linux 鏍戙€傚浜庡紑鍙戞垨娴嬭瘯涓殑琛ヤ竵鐢?DAMON 缁存姢鑰呮帓鍏?`damon/next
<https://git.kernel.org/sj/h/damon/next>`_銆傜粡杩囧厖鍒嗚瘎瀹＄殑琛ヤ竵鐢卞唴瀛樼鐞嗗瓙绯荤粺缁存姢鑰呮帓鍏?`mm-new
<https://git.kernel.org/akpm/mm/h/mm-new>`_銆傞殢鐫€娴嬭瘯鏇村姞鍏呭垎锛岃ˉ涓佷細绉诲姩鍒?`mm-unstable <https://git.kernel.org/akpm/mm/h/mm-unstable>`_锛屽啀绉诲姩鍒?`mm-stable <https://git.kernel.org/akpm/mm/h/mm-stable>`_銆傛渶缁堣繖浜涜ˉ涓佷細鐢卞唴瀛樼鐞嗗瓙绯荤粺缁存姢鑰呬互鎷夊彇璇锋眰鐨勫舰寮忔彁浜ゅ埌涓荤嚎銆?
鍐嶆鎻愰啋锛岄拡瀵?`mm-new tree
<https://git.kernel.org/akpm/mm/h/mm-new>`_ 鐨勮ˉ涓佺敱鍐呭瓨绠＄悊瀛愮郴缁熺淮鎶よ€呮帓闃熴€傚鏋滆ˉ涓侀渶瑕?`damon/next tree
<https://git.kernel.org/sj/h/damon/next>`_ 涓皻鏈悎骞惰繘 mm-new 鐨勬煇浜涜ˉ涓侊紝璇峰姟蹇呮竻妤氳鏄庤渚濊禆鍏崇郴銆?
### 鎻愪氦妫€鏌ユ竻鍗曡ˉ鍏?
杩涜 DAMON 鏀瑰姩鏃讹紝搴斿仛鍒颁互涓嬪嚑鐐广€?
- 鏋勫缓鍙樻洿鐩稿叧鐨勪骇鐗╋紝鍖呮嫭鍐呮牳涓庢枃妗ｃ€?- 纭繚鏋勫缓涓嶅紩鍏ユ柊鐨勯敊璇垨璀﹀憡銆?- 杩愯 DAMON `selftests
  <https://github.com/damonitor/damon-tests/blob/master/corr/run.sh#L49>`_ 涓?`kunittests
  <https://github.com/damonitor/damon-tests/blob/master/corr/tests/kunit.sh>`_ 骞剁‘淇濇棤鏂板け璐ャ€?
杩涗竴姝ュ仛鍒颁互涓嬪嚑鐐瑰苟鎶婄粨鏋滈檮涓婁細寰堟湁甯姪銆?
- 閽堝鏅€氭敼鍔ㄨ繍琛?`damon-tests/corr
  <https://github.com/damonitor/damon-tests/tree/master/corr>`_銆?- 閽堝鎬ц兘鏀瑰姩锛屾祴閲忓鍩哄噯娴嬭瘯鎴栫湡瀹炰笘鐣屽伐浣滆礋杞界殑褰卞搷銆?
### 鍏抽敭鍛ㄦ湡鏃ユ湡

琛ヤ竵鍙殢鏃跺彂閫併€俙mm-new
<https://git.kernel.org/akpm/mm/h/mm-new>`_銆乣mm-unstable
<https://git.kernel.org/akpm/mm/h/mm-unstable>`_ 涓?`mm-stable
<https://git.kernel.org/akpm/mm/h/mm-stable>`_ 鏍戠殑鍏抽敭鍛ㄦ湡鏃ユ湡鍙栧喅浜庡唴瀛樼鐞嗗瓙绯荤粺缁存姢鑰呫€?
### 璇勫鑺傚

DAMON 缁存姢鑰呴€氬父宸ヤ綔鏂瑰紡鐏垫椿锛屼絾澶钩娲嬫椂闂达紙PT锛夋竻鏅ㄩ櫎澶栥€傚琛ヤ竵鐨勫洖澶嶅伓灏斾細杈冩參銆傚鏋滃湪鍙戦€佽ˉ涓佸悗涓€鍛ㄥ唴娌℃湁鏀跺埌鍥炲锛岃鍕跨姽璞紝鍙戞秷鎭彁閱掍竴涓嬨€?
### 閭欢宸ュ叿

涓庤澶氬叾浠?Linux 鍐呮牳瀛愮郴缁熶竴鏍凤紝DAMON 浣跨敤閭欢鍒楄〃锛坉amon@lists.linux.dev 涓?linux-mm@kvack.org锛変綔涓轰富瑕佹矡閫氭笭閬撱€傛湁涓€涓悕涓?`HacKerMaiL
<https://github.com/damonitor/hackermail>`_锛坄hkml`锛夌殑绠€鍗曞伐鍏凤紝闈㈠悜涓嶅お鐔熸倝鍩轰簬閭欢鍒楄〃娌熼€氱殑浜恒€傝宸ュ叿瀵?DAMON 绀惧尯鎴愬憳灏ゅ叾鏈夌敤锛屽洜涓哄畠鐢?DAMON 缁存姢鑰呭紑鍙戝苟缁存姢銆傝宸ュ叿涔熷凡姝ｅ紡瀹ｅ竷鏀寔 DAMON 鍙婇€氱敤鐨?Linux 鍐呮牳寮€鍙戞祦绋嬨€?
鎹㈣█涔嬶紝`hkml <https://github.com/damonitor/hackermail>`_ 鏄潰鍚?DAMON 绀惧尯鐨勯偖浠跺伐鍏凤紝DAMON 缁存姢鑰呮壙璇轰簣浠ユ敮鎸併€傝闅忔剰璇曠敤锛屽苟鍚戠淮鎶よ€呮姤鍛婇棶棰樻垨鍔熻兘璇锋眰銆?
### 绀惧尯鑱氫細

DAMON 绀惧尯涓烘洿鍠滄鍚屾瀵硅瘽鑰岄潪閭欢寰€鏉ョ殑鎴愬憳涓惧姙鍙屽懆鑱氫細绯诲垪銆傚畠鐢ㄤ簬鍖呮嫭缁存姢鑰呭湪鍐呯殑涓€缇ゆ垚鍛樹箣闂村氨鐗瑰畾涓婚灞曞紑璁ㄨ銆傜淮鎶よ€呬細鍒嗕韩鍙敤鐨勬椂娈碉紝鍙備細鑰呭簲鍦ㄦ椂娈靛紑濮嬪墠鑷冲皯 24 灏忔椂閫氳繃鑱旂郴缁存姢鑰呮潵棰勭害鍏朵腑涓€涓椂娈点€?
鏃ョ▼涓庨绾︾姸鎬佸彲鍦?Google `doc
<https://docs.google.com/document/d/1v43Kcj3ly4CYqmAkMaZzLiM2GEnWfgdGbZAH3mi2vpM/edit?usp=sharing>`_ 鏌ョ湅銆傝繕鏈変竴涓叕寮€鐨?Google `calendar
<https://calendar.google.com/calendar/u/0?cid=ZDIwOTA4YTMxNjc2MDQ3NTIyMmUzYTM5ZmQyM2U4NDA0ZGIwZjBiYmJlZGQxNDM0MmY4ZTRjOTE0NjdhZDRiY0Bncm91cC5jYWxlbmRhci5nb29nbGUuY29t>`_
鍖呭惈鐩稿叧浜嬩欢銆備换浣曚汉閮藉彲浠ヨ闃呫€侱AMON 缁存姢鑰呬篃浼氬畾鏈熷悜閭欢鍒楄〃锛坉amon@lists.linux.dev锛夊彂閫佹彁閱掋€?