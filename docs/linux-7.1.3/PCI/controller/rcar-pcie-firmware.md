
## Renesas R-Car V4H PCIe 鎺у埗鍣ㄥ浐浠?

Renesas R-Car V4H (r8a779g0) 鏈変竴涓?PCIe 鎺у埗鍣紝闇€瑕佺壒瀹氱殑
鍚姩鏈熼棿涓嬭浇鍥轰欢銆?
涓嶈繃锛岀憺钀ㄧ數瀛愮洰鍓嶆棤娉曞厤璐瑰垎鍙戣鍥轰欢銆?
鍥轰欢鏂囦欢 鈥?04_PCIe_fw_addr_data_ver1.05.txt鈥濓紙璇锋敞鎰忥紝鏂囦欢鍚嶅湪涓嶅悓
鐗堟湰鐨勬暟鎹墜鍐屼箣闂村彲鑳芥湁鎵€涓嶅悓锛夊彲浠ュ湪鏁版嵁鎵嬪唽涓互鏂囨湰缂栫爜鐨勫舰寮忔壘鍒帮紝
鍥犳蹇呴』灏嗚鏂囦欢鐨勫唴瀹硅浆鎹㈠洖浜岃繘鍒跺舰寮忋€傚彲浠ヤ娇鐢ㄤ互涓嬬ず渚嬭剼鏈畬鎴?姝ゆ搷浣滐細


	$ awk '/^\s*0x[0-9A-Fa-f]{4}\s+0x[0-9A-Fa-f]{4}/ { print substr($2,5,2) substr($2,3,2) }' \
		104_PCIe_fw_addr_data_ver1.05.txt | \
			xxd -p -r > rcar_gen4_pcie.bin

灏嗘枃鏈唴瀹硅浆鎹负浜岃繘鍒跺浐浠舵枃浠跺悗锛屾寜濡備笅鏂瑰紡楠岃瘉鍏舵牎楠屽拰锛?

	$ sha1sum rcar_gen4_pcie.bin
	1d0bd4b189b4eb009f5d564b1f93a79112994945  rcar_gen4_pcie.bin

鐢熸垚鐨勫悕涓?鈥渞car_gen4_pcie.bin鈥?鐨勪簩杩涘埗鏂囦欢搴斿湪椹卞姩绋嬪簭杩愯涔嬪墠
鏀剧疆鍦?鈥?lib/firmware鈥?鐩綍涓€?