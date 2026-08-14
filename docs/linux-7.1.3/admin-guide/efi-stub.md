## EFI 寮曞瀛樻牴锛圗FI Boot Stub锛?

鍦?x86 涓?ARM 骞冲彴涓婏紝鍐呮牳 zImage/bzImage 鍙互浼鎴?PE/COFF 闀滃儚锛屼粠鑰岃鏈?EFI 鍥轰欢
鍔犺浇鍣ㄥ皢鍏朵綔涓?EFI 鍙墽琛屾枃浠跺姞杞姐€備慨鏀?bzImage 澶撮儴鐨勪唬鐮侊紝杩炲悓鍥轰欢鍔犺浇鍣ㄨ烦杞埌鐨?EFI 鐗瑰畾鍏ュ彛鐐癸紝缁熺О涓衡€淓FI 寮曞瀛樻牴锛圗FI boot stub锛夆€濓紝鍒嗗埆浣嶄簬
arch/x86/boot/header.S 涓?drivers/firmware/efi/libstub/x86-stub.c銆傚浜?ARM锛孍FI 瀛樻牴
瀹炵幇浜?arch/arm/boot/compressed/efi-header.S 涓?drivers/firmware/efi/libstub/arm32-stub.c銆傚悇鏋舵瀯闂村叡浜殑 EFI 瀛樻牴浠ｇ爜浣嶄簬
drivers/firmware/efi/libstub銆?
瀵逛簬 arm64锛屾病鏈夊帇缂╁唴鏍告敮鎸侊紝鍥犳 Image 鑷韩浼鎴?PE/COFF 闀滃儚锛孍FI 瀛樻牴琚摼鎺ヨ繘
鍐呮牳銆俛rm64 鐨?EFI 瀛樻牴浣嶄簬 drivers/firmware/efi/libstub/arm64.c 涓?drivers/firmware/efi/libstub/arm64-stub.c銆?
閫氳繃浣跨敤 EFI 寮曞瀛樻牴锛屽彲浠ュ湪涓嶄娇鐢ㄤ紶缁?EFI 寮曞鍔犺浇鍣紙濡?grub 鎴?elilo锛夌殑鎯呭喌涓?寮曞 Linux 鍐呮牳銆傜敱浜?EFI 寮曞瀛樻牴鎵挎媴浜嗗紩瀵煎姞杞藉櫒鐨勫伐浣滐紝鍦ㄦ煇绉嶆剰涔変笂瀹?*灏辨槸**寮曞
鍔犺浇鍣ㄣ€?
EFI 寮曞瀛樻牴閫氳繃 CONFIG_EFI_STUB 鍐呮牳閫夐」鍚敤銆?

### 濡備綍瀹夎 bzImage.efi


浣嶄簬 arch/x86/boot/bzImage 鐨?bzImage 蹇呴』澶嶅埗鍒?EFI 绯荤粺鍒嗗尯锛圗SP锛夛紝骞舵敼鍚嶄负鎵╁睍鍚?鈥?efi鈥濄€傛病鏈夎鎵╁睍鍚嶏紝EFI 鍥轰欢鍔犺浇鍣ㄤ細鎷掔粷鎵ц瀹冦€傛棤娉曚粠甯哥敤鐨?Linux 鏂囦欢绯荤粺鎵ц
bzImage.efi锛屽洜涓?EFI 鍥轰欢涓嶆敮鎸佸畠浠€傚浜?ARM锛屽簲灏?arch/arm/boot/zImage 澶嶅埗鍒扮郴缁?鍒嗗尯锛屽彲鑳戒笉闇€瑕佹敼鍚嶃€傜被浼煎湴锛屽浜?arm64锛屽簲澶嶅埗 arch/arm64/boot/Image锛屼絾涓嶄竴瀹氳鏀瑰悕銆?

### 浠?EFI shell 浼犻€掑唴鏍稿弬鏁?

```

	fs0:> bzImage.efi console=ttyS0 root=/dev/sda4


```
### "initrd=" 閫夐」


涓庡ぇ澶氭暟寮曞鍔犺浇鍣ㄤ竴鏍凤紝EFI 瀛樻牴鍏佽鐢ㄦ埛浣跨敤 "initrd=" 閫夐」鎸囧畾澶氫釜 initrd 鏂囦欢銆傝繖鏄?鍞竴 EFI 瀛樻牴鐗瑰畾鐨勫懡浠よ鍙傛暟锛屽叾浣欏唴瀹瑰湪鍐呮牳寮曞鏃朵紶缁欏唴鏍搞€?
initrd 鏂囦欢鐨勮矾寰勫繀椤绘槸浠?ESP 璧峰鐨勭粷瀵硅矾寰勶紝鐩稿璺緞鍚嶄笉璧蜂綔鐢ㄣ€傛澶栵紝璇ヨ矾寰勬槸 EFI
椋庢牸鐨勮矾寰勶紝鐩綍鍏冪礌蹇呴』鐢ㄤ互涓嬪垎闅旂鍒嗛殧锛?
```

  fs0:>
	Kernels\
			bzImage.efi
			initrd-large.img

	Ramdisks\
			initrd-small.img
			initrd-medium.img

```
瑕佸湪褰撳墠宸ヤ綔鐩綍涓嬩互 initrd-large.img 鏂囦欢寮曞锛?
```

	fs0:\Kernels> bzImage.efi initrd=\Kernels\initrd-large.img

```
娉ㄦ剰锛宐zImage.efi 鏄浣曞彲浠ョ敤鐩稿璺緞鎸囧畾鐨勩€傝繖鏄洜涓烘垜浠鍦ㄦ墽琛岀殑闀滃儚鐢?EFI shell
瑙ｉ噴锛岃€?EFI shell 鐞嗚В鐩稿璺緞锛岃€屽懡浠よ鐨勫叾浣欓儴鍒嗗垯浼犵粰 bzImage.efi銆?
   涔熷彲浠ュ湪寮曞鏃朵娇鐢?Linux 鐗瑰畾鐨?UEFI 鍗忚鎻愪緵 initrd銆傝瑙?pe-coff-entry-point銆?
### "dtb=" 閫夐」


瀵逛簬 ARM 涓?arm64 鏋舵瀯锛屽繀椤诲悜鍐呮牳鎻愪緵璁惧鏍戙€傞€氬父鍥轰欢搴旈€氳繃 EFI CONFIGURATION TABLE
鎻愪緵璁惧鏍戙€傜劧鑰岋紝"dtb=" 鍛戒护琛岄€夐」鍙敤浜庤鐩栧浐浠舵彁渚涚殑璁惧鏍戯紝鎴栧湪鍥轰欢鏃犳硶鎻愪緵鏃舵彁渚?涓€涓€?
璇锋敞鎰忥細鍥轰欢鍦ㄥ紩瀵煎唴鏍镐箣鍓嶄細鍚戣澶囨爲娣诲姞杩愯鏃堕厤缃俊鎭€傚鏋滀娇鐢?dtb= 瑕嗙洊璁惧鏍戯紝
鍒欏浐浠舵彁渚涚殑浠讳綍杩愯鏃舵暟鎹兘浼氫涪澶便€?dtb=" 閫夐」鍙簲浣滀负璋冭瘯宸ュ叿锛屾垨浣滀负鍦?EFI
CONFIGURATION TABLE 涓湭鎻愪緵璁惧鏍戞椂鐨勬渶鍚庢墜娈典娇鐢ㄣ€?
"dtb=" 鐨勫鐞嗘柟寮忎笌涓婅堪 "initrd=" 閫夐」鐩稿悓銆?