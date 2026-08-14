## SELinux


鏈夊叧 SELinux 鍐呮牳瀛愮郴缁熺殑淇℃伅鍙湪浠ヤ笅閾炬帴鎵惧埌锛?

	https://git.kernel.org/pub/scm/linux/kernel/git/pcmoore/selinux.git/tree/README.md

	https://github.com/selinuxproject/selinux-kernel/wiki

鏈夊叧 SELinux 鐢ㄦ埛绌洪棿鐨勪俊鎭彲鍦ㄤ互涓嬩綅缃壘鍒帮細

	https://github.com/SELinuxProject/selinux/wiki

濡傛灉浣犳兂瑕佷娇鐢?SELinux锛屼綘寰堝彲鑳戒細鎯充娇鐢ㄥ彂琛岀増鎻愪緵鐨勭瓥鐣ワ紝鎴栦粠浠ヤ笅浣嶇疆瀹夎鏈€鏂扮殑鍙傝€冪瓥鐣ョ増鏈?

	https://github.com/SELinuxProject/refpolicy

浣嗘槸锛屽鏋滀綘鎯冲畨瑁呬竴涓敤浜庢祴璇曠殑铏氭嫙锛坉ummy锛夌瓥鐣ワ紝鍙互浣跨敤 scripts/selinux 涓嬫彁渚涚殑 `mdp` 鏉ュ畬鎴愩€傛敞鎰忚繖闇€瑕佸畨瑁?selinux 鐢ㄦ埛绌洪棿鈥斺€旂壒鍒槸浣犻渶瑕?checkpolicy 鏉ョ紪璇戝唴鏍革紝浠ュ強 setfiles 鍜?fixfiles 鏉ユ爣璁版枃浠剁郴缁熴€?

 1. 缂栬瘧鍚敤 selinux 鐨勫唴鏍搞€?
 2. 杈撳叆 `make` 缂栬瘧 `mdp`銆?
 3. 纭繚浣犳病鏈夊湪鍚敤 SELinux 涓斾娇鐢ㄧ湡瀹炵瓥鐣ョ殑鎯呭喌涓嬭繍琛屻€傚鏋滄槸锛岃鍦ㄧ户缁箣鍓嶄互绂佺敤 selinux 鐨勬柟寮忛噸鍚€?
```
		cd scripts/selinux
		sh install_policy.sh
```
绗?4 姝ュ皢鍒涘缓涓€涓浣犵殑鍐呮牳鏈夋晥鐨勬柊铏氭嫙绛栫暐锛屽叾涓彧鏈変竴涓?selinux 鐢ㄦ埛銆佽鑹插拰绫诲瀷銆傚畠灏嗙紪璇戣绛栫暐锛屽皢浣犵殑 `SELINUXTYPE` 鍦?`/etc/selinux/config` 涓涓?`dummy`锛屽畨瑁呯紪璇戝悗鐨勭瓥鐣ヤ綔涓?`dummy`锛屽苟閲嶆柊鏍囪浣犵殑鏂囦欢绯荤粺銆?
