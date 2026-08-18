## PCI 绔偣娴嬭瘯鍔熻兘椹卞姩


濡傛灉鏍瑰鍚堜綋杩炴帴鍒拌繍琛?`pci_epf_test` 鍔熻兘椹卞姩鐨勫彲閰嶇疆 PCI 绔偣锛堟寜鐓?[^1^]_
閰嶇疆锛夛紝鍒欒椹卞姩搴斾綔涓轰富鏈虹椹卞姩浣跨敤銆?
鈥減ci_endpoint_test鈥濋┍鍔ㄥ彲鐢ㄤ簬鎵ц浠ヤ笅娴嬭瘯銆?
娴嬭瘯璁惧鐨?PCI 椹卞姩鎵ц浠ヤ笅娴嬭瘯锛?
	#) 楠岃瘉 BAR 涓紪绋嬬殑鍦板潃
	#) 瑙﹀彂浼犵粺 IRQ
	#) 瑙﹀彂 MSI IRQ
	#) 瑙﹀彂 MSI-X IRQ
	#) 璇诲彇鏁版嵁
	#) 鍐欏叆鏁版嵁
	#) 澶嶅埗鏁版嵁

璇?misc 椹卞姩涓烘瘡涓繛鎺ュ埌鏍瑰鍚堜綋鐨?`pci_epf_test` 鍔熻兘鍒涘缓
/dev/pci-endpoint-test.<num>锛屽苟搴斾娇鐢ㄢ€渋octls鈥濇潵鎵ц涓婅堪娴嬭瘯銆?
### ioctl


 PCITEST_BAR:
	      娴嬭瘯 BAR銆傚簲浼犲叆瑕佹祴璇曠殑 BAR 缂栧彿浣滀负鍙傛暟銆? PCITEST_LEGACY_IRQ:
	      娴嬭瘯浼犵粺 IRQ
 PCITEST_MSI:
	      娴嬭瘯娑堟伅淇″彿涓柇銆傚簲浼犲叆瑕佹祴璇曠殑 MSI 缂栧彿浣滀负鍙傛暟銆? PCITEST_MSIX:
	      娴嬭瘯娑堟伅淇″彿涓柇銆傚簲浼犲叆瑕佹祴璇曠殑 MSI-X 缂栧彿浣滀负鍙傛暟銆? PCITEST_SET_IRQTYPE:
	      鏇存敼椹卞姩 IRQ 绫诲瀷閰嶇疆銆傚簲浼犲叆 IRQ 绫诲瀷浣滀负鍙傛暟
	      锛?锛歀egacy锛?锛歁SI锛?锛歁SI-X锛夈€? PCITEST_GET_IRQTYPE:
	      鑾峰彇椹卞姩 IRQ 绫诲瀷閰嶇疆銆? PCITEST_WRITE:
	      鎵ц鍐欐祴璇曘€傚簲浼犲叆缂撳啿鍖哄ぇ灏忎綔涓哄弬鏁般€? PCITEST_READ:
	      鎵ц璇绘祴璇曘€傚簲浼犲叆缂撳啿鍖哄ぇ灏忎綔涓哄弬鏁般€? PCITEST_COPY:
	      鎵ц璇绘祴璇曘€傚簲浼犲叆缂撳啿鍖哄ぇ灏忎綔涓哄弬鏁般€?