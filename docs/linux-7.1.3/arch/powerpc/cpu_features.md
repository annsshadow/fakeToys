## CPU 鐗规€э紙CPU Features锛?

Hollis Blanchard <hollis@austin.ibm.com>
2002 骞?6 鏈?5 鏃?
鏈枃妗ｆ弿杩颁簡 PPC Linux 鍐呮牳涓娇鐢ㄧ殑绯荤粺锛堝寘鎷嚜淇敼浠ｇ爜锛夛紝鐢ㄤ簬鏀寔澶氱
PowerPC CPU锛岃€屾棤闇€鍦ㄧ紪璇戞湡杩涜閫夋嫨銆?
鍦ㄥ惎鍔ㄨ繃绋嬬殑鏃╂湡锛宲pc32 鍐呮牳浼氭娴嬪綋鍓嶇殑 CPU 绫诲瀷骞剁浉搴斿湴閫夋嫨涓€缁勭壒鎬с€?涓€浜涗緥瀛愬寘鎷?Altivec 鏀寔銆佹寚浠や笌鏁版嵁鍒嗙鐨勭紦瀛橈紝浠ュ強 CPU 鏄惁鏀寔 DOZE 涓?NAP 鐫＄湢妯″紡銆?
鐗规€ч泦鍚堢殑妫€娴嬪緢绠€鍗曘€傚鐞嗗櫒鍒楄〃鍙湪 arch/powerpc/kernel/cputable.c 涓壘鍒般€?PVR 瀵勫瓨鍣ㄨ鎺╃爜澶勭悊骞朵笌鍒楄〃涓殑姣忎釜鍊艰繘琛屾瘮杈冦€傚鏋滄壘鍒板尮閰嶏紝cur_cpu_spec
鐨?cpu_features 浼氳璧嬪€间负璇ュ鐞嗗櫒鐨勭壒鎬т綅鎺╃爜锛屽苟璋冪敤涓€涓?__setup_cpu 鍑芥暟銆?
C 浠ｇ爜鍙互娴嬭瘯 'cur_cpu_spec[smp_processor_id()]->cpu_features' 鏉ヨ幏鍙栨煇涓?鐗瑰畾鐨勭壒鎬т綅銆傝繖涓€鎿嶄綔鍦ㄥ緢澶氬湴鏂归兘浼氳繘琛岋紝渚嬪鍦?ppc_setup_l2cr() 涓€?
鍦ㄦ眹缂栦腑瀹炵幇 cpufeatures 瑕佺◢寰鏉備竴浜涖€傛湁鑻ュ共鎬ц兘鍏抽敭璺緞锛屽鏋滃姞鍏ユ暟缁?绱㈠紩銆佺粨鏋勪綋瑙ｅ紩鐢ㄥ拰鏉′欢鍒嗘敮灏变細鍙楀奖鍝嶃€備负浜嗛伩鍏嶆€ц兘鎹熷け锛屽悓鏃朵粛鍏佽杩愯鏃?锛堣€岄潪缂栬瘧鏈燂級CPU 閫夋嫨锛屾湭浣跨敤鐨勪唬鐮佷細琚浛鎹负 'nop' 鎸囦护銆傝繖绉?nop 鏇挎崲
鍩轰簬 CPU 0 鐨勮兘鍔涳紝鍥犳鐢遍潪鐩稿悓澶勭悊鍣ㄧ粍鎴愮殑澶氬鐞嗗櫒绯荤粺灏嗘棤娉曞伐浣滐紙涓嶈繃杩欐牱
鐨勭郴缁熸湰鏉ヤ篃鍙兘浼氭湁鍏跺畠闂锛夈€?
鍦ㄦ娴嬪埌澶勭悊鍣ㄧ被鍨嬩箣鍚庯紝鍐呮牳浼氶€氳繃鍐欏叆 nop 鏉ヤ慨琛ユ帀涓嶅簲琚娇鐢ㄧ殑浠ｇ爜娈点€備娇鐢?cpufeatures 鍙渶瑕?2 涓畯锛堜綅浜?arch/powerpc/include/asm/cputable.h 涓級锛?濡?head.S 涓墍绀猴細

```

	#ifdef CONFIG_ALTIVEC
	BEGIN_FTR_SECTION
		mfspr	r22,SPRN_VRSAVE		/* if G4, save vrsave register value */
		stw	r22,THREAD_VRSAVE(r23)
	END_FTR_SECTION_IFSET(CPU_FTR_ALTIVEC)
	#endif /* CONFIG_ALTIVEC */

```
濡傛灉 CPU 0 鏀寔 Altivec锛屽垯浠ｇ爜淇濇寔涓嶅彉銆傚鏋滀笉鏀寔锛屼袱鏉℃寚浠ら兘浼氳鏇挎崲涓?nop銆?
END_FTR_SECTION 瀹忔湁涓や釜鏇寸畝鍗曠殑鍙樹綋锛欵ND_FTR_SECTION_IFSET 涓?END_FTR_SECTION_IFCLR銆傚畠浠垎鍒敤浜庢祴璇曟煇涓爣蹇楋紙鍦?cur_cpu_spec[^0^]->cpu_features 涓級鏄惁琚疆浣嶆垨娓呴櫎銆傚湪澶у鏁版儏鍐典笅搴斾娇鐢?杩欎袱涓畯銆?
END_FTR_SECTION 瀹忕殑瀹炵幇鏂瑰紡鏄皢鏈夊叧杩欐浠ｇ爜鐨勪俊鎭瓨鍌ㄥ湪 '__ftr_fixup' ELF
娈典腑銆傚綋 do_cpu_ftr_fixups锛坅rch/powerpc/kernel/misc.S锛夎璋冪敤鏃讹紝瀹冧細閬嶅巻
__ftr_fixup 涓殑璁板綍锛屽鏋滄墍闇€鐗规€т笉瀛樺湪锛屽氨浼氫粠姣忎釜 BEGIN_FTR_SECTION 鍒?END_FTR_SECTION 寰幆鍐欏叆 nop銆?