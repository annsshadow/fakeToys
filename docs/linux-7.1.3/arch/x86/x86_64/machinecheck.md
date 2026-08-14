## x86-64 鏈哄櫒妫€鏌ワ紙machine check锛変唬鐮佺殑鍙厤缃?sysfs 鍙傛暟


鏈哄櫒妫€鏌ワ紙Machine check锛夋姤鍛婄敱 CPU 妫€娴嬪埌鐨勫唴閮ㄧ‖浠堕敊璇姸鎬併€傛湭绾犳锛圲ncorrected锛夌殑閿欒閫氬父浼氬鑷翠竴娆℃満鍣ㄦ鏌ワ紙閫氬父浼撮殢 panic锛夛紝宸茬籂姝ｇ殑閿欒鍒欎細鐢熸垚涓€鏉℃満鍣ㄦ鏌ユ棩蹇楁潯鐩€?
鏈哄櫒妫€鏌ユ寜 bank锛堥€氬父鍏宠仈鏌愪釜纭欢瀛愮郴缁燂級浠ュ強 bank 鍐呯殑瀛愪簨浠讹紙subevent锛夌粍缁囥€俠ank 涓庡瓙浜嬩欢鐨勭‘鍒囧惈涔夋槸 CPU 鐩稿叧鐨勩€?
mcelog 鐭ラ亾濡備綍瀵瑰畠浠繘琛岃В鐮併€?
褰撲綘鍦ㄧ郴缁熸棩蹇椾腑鐪嬪埌 "Machine check errors logged" 娑堟伅鏃讹紝搴斿綋杩愯 mcelog 鏉ヤ粠 /dev/mcelog 鏀堕泦骞惰В鐮佹満鍣ㄦ鏌ユ潯鐩€傞€氬父锛宮celog 搴旈€氳繃 cron 浠诲姟瀹氭湡杩愯銆?
姣忎釜 CPU 鍦?/sys/devices/system/machinecheck/machinecheckN 涓嬮兘鏈変竴涓洰褰曪紙N = CPU 缂栧彿锛夈€?
璇ョ洰褰曞寘鍚竴浜涘彲閰嶇疆椤广€傛洿澶氱粏鑺傝鍙傞槄 Documentation/ABI/testing/sysfs-mce銆?
寰呭畾锛圱BD锛夛細璁板綍 AMD 闃堝€间腑鏂紙threshold interrupt锛夐厤缃浉鍏虫潯鐩€?
鏈夊叧 x86 鏈哄櫒妫€鏌ユ灦鏋勭殑鏇村缁嗚妭锛岃鍙傞槄 Intel 鍜?AMD 寮€鍙戣€呯綉绔欎笂鐨勬灦鏋勬墜鍐屻€?
鏈夊叧璇ユ灦鏋勭殑鏇村缁嗚妭锛岃鍙傞槄 http://one.firstfloor.org/~andi/mce.pdf
