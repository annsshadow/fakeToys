
## ARM64 ELF hwcaps

鏈枃妗ｆ弿杩颁簡 arm64 ELF hwcaps 鐨勭敤娉曚笌璇箟銆?
### 1. Introduction锛堢畝浠嬶級

鏌愪簺纭欢鎴栬蒋浠剁壒鎬т粎鍦ㄩ儴鍒?CPU 瀹炵幇涓娿€佸拰/鎴栭厤鍚堢壒瀹氱殑鍐呮牳閰嶇疆鎵嶅彲鐢紝浣嗗湪 EL0 澶勬病鏈夊彲渚涚敤鎴风┖闂翠唬鐮佷娇鐢ㄧ殑鏋舵瀯鍖栧彂鐜版満鍒躲€傚唴鏍搁€氳繃涓€缁勭О涓?hwcaps 鐨勬爣蹇楋紙鏆撮湶鍦ㄨ緟鍔╁悜閲忎腑锛夊皢杩欎簺鐗规€х殑瀛樺湪鏆撮湶缁欑敤鎴风┖闂淬€?
鐢ㄦ埛绌洪棿杞欢鍙互閫氳繃鑾峰彇杈呭姪鍚戦噺鐨?AT_HWCAP銆丄T_HWCAP2 鎴?AT_HWCAP3 鏉＄洰锛屽苟娴嬭瘯
```

	bool floating_point_is_present(void)
	{
		unsigned long hwcaps = getauxval(AT_HWCAP);
		if (hwcaps & HWCAP_FP)
			return true;

		return false;
	}

```
鍦ㄨ蒋浠朵緷璧栦簬鏌愪釜鐢?hwcap 鎻忚堪鐨勭壒寰佹椂锛屽畠搴斿綋鍦ㄥ皾璇曚娇鐢ㄨ鐗瑰緛涔嬪墠锛屾鏌ョ浉鍏崇殑 hwcap 鏍囧織浠ョ‘璁よ鐗瑰緛纭疄瀛樺湪銆?
鏃犳硶鍙潬鍦伴€氳繃鍏朵粬鎵嬫鎺㈡祴杩欎簺鐗瑰緛銆傚綋鏌愪釜鐗瑰緛涓嶅彲鐢ㄦ椂锛屽皾璇曚娇鐢ㄥ畠鍙兘瀵艰嚧涓嶅彲棰勬祴鐨勮涓猴紝骞朵笖涓嶄繚璇佷細浜х敓浠讳綍鍙潬鐨勬寚绀猴紙渚嬪 SIGILL锛夎〃鏄庤鐗瑰緛涓嶅彲鐢ㄣ€?
### 2. Interpretation of hwcaps锛坔wcaps 鐨勮В閲婏級

澶у鏁?hwcaps 鏃ㄥ湪鎸囩ず閭ｄ簺鐢辨灦鏋勫寲 ID 瀵勫瓨鍣紙鍦?EL0 澶勭敤鎴风┖闂翠唬鐮佹棤娉曡闂級鎵€鎻忚堪鐨勭壒寰佺殑瀛樺湪銆傝繖浜?hwcaps 鏄牴鎹?ID 瀵勫瓨鍣ㄥ瓧娈垫潵瀹氫箟鐨勶紝骞朵笖搴斿綋鍙傝€?ARM 鏋舵瀯鍙傝€冩墜鍐岋紙ARM ARM锛変腑杩欎簺瀛楁鐨勫畾涔夋潵瑙ｉ噴銆?
```

    Functionality implied by idreg.field == val.

```
杩欑被 hwcaps 鎸囩ず浜?ARM ARM 瀹氫箟涓哄湪 idreg.field 鍙栧€间负 val 鏃跺瓨鍦ㄧ殑鍔熻兘锛屼絾骞朵笉鎰忓懗鐫€ idreg.field 绮剧‘绛変簬 val锛屼篃涓嶆剰鍛崇潃鎸囩ず浜嗙敱 idreg.field 鍏朵粬鍙栧€兼墍闅愬惈鐨勫姛鑳界殑缂哄け銆?
鍏朵粬 hwcaps 鍙兘鎸囩ず鏃犳硶浠呯敱 ID 瀵勫瓨鍣ㄦ弿杩扮殑鐗瑰緛鐨勫瓨鍦ㄣ€傝繖浜涘彲浠ュ湪涓嶅弬鑰?ID 瀵勫瓨鍣ㄧ殑鎯呭喌涓嬫弿杩帮紝骞朵笖鍙兘寮曠敤鍏朵粬鏂囨。銆?
### 3. The hwcaps exposed in AT_HWCAP锛堟毚闇插湪 AT_HWCAP 涓殑 hwcaps锛?
HWCAP_FP
    鐢?ID_AA64PFR0_EL1.FP == 0b0000 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ASIMD
    鐢?ID_AA64PFR0_EL1.AdvSIMD == 0b0000 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_EVTSTRM
    閫氱敤瀹氭椂鍣ㄨ閰嶇疆涓轰互澶х害 10KHz 鐨勯鐜囩敓鎴愪簨浠躲€?
HWCAP_AES
    鐢?ID_AA64ISAR0_EL1.AES == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_PMULL
    鐢?ID_AA64ISAR0_EL1.AES == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SHA1
    鐢?ID_AA64ISAR0_EL1.SHA1 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SHA2
    鐢?ID_AA64ISAR0_EL1.SHA2 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_CRC32
    鐢?ID_AA64ISAR0_EL1.CRC32 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ATOMICS
    鐢?ID_AA64ISAR0_EL1.Atomic == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_FPHP
    鐢?ID_AA64PFR0_EL1.FP == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ASIMDHP
    鐢?ID_AA64PFR0_EL1.AdvSIMD == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_CPUID
    EL0 瀵规煇浜?ID 瀵勫瓨鍣ㄧ殑璁块棶鏄彲鐢ㄧ殑锛岃寖鍥村 Documentation/arch/arm64/cpu-feature-registers.rst 鎵€杩般€?
    杩欎簺 ID 瀵勫瓨鍣ㄥ彲鑳芥殫绀轰簡鏌愪簺鐗瑰緛鐨勫彲鐢ㄦ€с€?
HWCAP_ASIMDRDM
    鐢?ID_AA64ISAR0_EL1.RDM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_JSCVT
    鐢?ID_AA64ISAR1_EL1.JSCVT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_FCMA
    鐢?ID_AA64ISAR1_EL1.FCMA == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_LRCPC
    鐢?ID_AA64ISAR1_EL1.LRCPC == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_DCPOP
    鐢?ID_AA64ISAR1_EL1.DPB == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SHA3
    鐢?ID_AA64ISAR0_EL1.SHA3 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SM3
    鐢?ID_AA64ISAR0_EL1.SM3 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SM4
    鐢?ID_AA64ISAR0_EL1.SM4 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ASIMDDP
    鐢?ID_AA64ISAR0_EL1.DP == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SHA512
    鐢?ID_AA64ISAR0_EL1.SHA2 == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ASIMDFHM
   鐢?ID_AA64ISAR0_EL1.FHM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_DIT
    鐢?ID_AA64PFR0_EL1.DIT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_USCAT
    鐢?ID_AA64MMFR2_EL1.AT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_ILRCPC
    鐢?ID_AA64ISAR1_EL1.LRCPC == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_FLAGM
    鐢?ID_AA64ISAR0_EL1.TS == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SSBS
    鐢?ID_AA64PFR1_EL1.SSBS == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SB
    鐢?ID_AA64ISAR1_EL1.SB == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_PACA
    鐢?ID_AA64ISAR1_EL1.APA == 0b0001 鎴?ID_AA64ISAR1_EL1.API == 0b0001 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/pointer-authentication.rst 鎵€杩般€?
HWCAP_PACG
    鐢?ID_AA64ISAR1_EL1.GPA == 0b0001 鎴?ID_AA64ISAR1_EL1.GPI == 0b0001 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/pointer-authentication.rst 鎵€杩般€?
HWCAP_GCS
    鐢?ID_AA64PFR1_EL1.GCS == 0b1 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/gcs.rst 鎵€杩般€?
HWCAP_CMPBR
    鐢?ID_AA64ISAR2_EL1.CSSC == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_FPRCVT
    鐢?ID_AA64ISAR3_EL1.FPRCVT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_F8MM8
    鐢?ID_AA64FPFR0_EL1.F8MM8 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_F8MM4
    鐢?ID_AA64FPFR0_EL1.F8MM4 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE_F16MM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.F16MM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE_ELTPERM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.ELTPERM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE_AES2
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.AES == 0b0011 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE_BFSCALE
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.B16B16 == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SVE2P2
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.SVEver == 0b0011 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME2P2
    鐢?ID_AA64SMFR0_EL1.SMEver == 0b0011 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME_SBITPERM
    鐢?ID_AA64SMFR0_EL1.SBitPerm == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME_AES
    鐢?ID_AA64SMFR0_EL1.AES == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME_SFEXPA
    鐢?ID_AA64SMFR0_EL1.SFEXPA == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME_STMOP
    鐢?ID_AA64SMFR0_EL1.STMOP == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP_SME_SMOP4
    鐢?ID_AA64SMFR0_EL1.SMOP4 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_DCPODP
    鐢?ID_AA64ISAR1_EL1.DPB == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVE2
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.SVEver == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEAES
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.AES == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEPMULL
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.AES == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEBITPERM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.BitPerm == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVESHA3
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.SHA3 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVESM4
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.SM4 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_FLAGM2
    鐢?ID_AA64ISAR0_EL1.TS == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_FRINT
    鐢?ID_AA64ISAR1_EL1.FRINTTS == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEI8MM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.I8MM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEF32MM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.F32MM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEF64MM
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.F64MM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVEBF16
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.BF16 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_I8MM
    鐢?ID_AA64ISAR1_EL1.I8MM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_BF16
    鐢?ID_AA64ISAR1_EL1.BF16 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_DGH
    鐢?ID_AA64ISAR1_EL1.DGH == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_RNG
    鐢?ID_AA64ISAR0_EL1.RNDR == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_BTI
    鐢?ID_AA64PFR1_EL1.BT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_MTE
    鐢?ID_AA64PFR1_EL1.MTE == 0b0010 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/memory-tagging-extension.rst 鎵€杩般€?
HWCAP2_ECV
    鐢?ID_AA64MMFR0_EL1.ECV == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_AFP
    鐢?ID_AA64MMFR1_EL1.AFP == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_RPRES
    鐢?ID_AA64ISAR2_EL1.RPRES == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_MTE3
    鐢?ID_AA64PFR1_EL1.MTE == 0b0011 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/memory-tagging-extension.rst 鎵€杩般€?
HWCAP2_SME
    鐢?ID_AA64PFR1_EL1.SME == 0b0001 鎵€闅愬惈鐨勫姛鑳斤紝濡?Documentation/arch/arm64/sme.rst 鎵€杩般€?
HWCAP2_SME_I16I64
    鐢?ID_AA64SMFR0_EL1.I16I64 == 0b1111 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_F64F64
    鐢?ID_AA64SMFR0_EL1.F64F64 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_I8I32
    鐢?ID_AA64SMFR0_EL1.I8I32 == 0b1111 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_F16F32
    鐢?ID_AA64SMFR0_EL1.F16F32 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_B16F32
    鐢?ID_AA64SMFR0_EL1.B16F32 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_F32F32
    鐢?ID_AA64SMFR0_EL1.F32F32 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_FA64
    鐢?ID_AA64SMFR0_EL1.FA64 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_WFXT
    鐢?ID_AA64ISAR2_EL1.WFXT == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_EBF16
    鐢?ID_AA64ISAR1_EL1.BF16 == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVE_EBF16
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.BF16 == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_CSSC
    鐢?ID_AA64ISAR2_EL1.CSSC == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_RPRFM
    鐢?ID_AA64ISAR2_EL1.RPRFM == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVE2P1
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.SVEver == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME2
    鐢?ID_AA64SMFR0_EL1.SMEver == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME2P1
    鐢?ID_AA64SMFR0_EL1.SMEver == 0b0010 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SMEI16I32
    鐢?ID_AA64SMFR0_EL1.I16I32 == 0b0101 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SMEBI32I32
    鐢?ID_AA64SMFR0_EL1.BI32I32 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SMEB16B16
    鐢?ID_AA64SMFR0_EL1.B16B16 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SMEF16F16
    鐢?ID_AA64SMFR0_EL1.F16F16 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_MOPS
    鐢?ID_AA64ISAR2_EL1.MOPS == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_HBC
    鐢?ID_AA64ISAR2_EL1.BC == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SVE_B16B16
    鐢?ID_AA64PFR0_EL1.SVE == 0b0001 浠ュ強 ID_AA64ZFR0_EL1.B16B16 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_LRCPC3
    鐢?ID_AA64ISAR1_EL1.LRCPC == 0b0011 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_LSE128
    鐢?ID_AA64ISAR0_EL1.Atomic == 0b0011 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_FPMR
    鐢?ID_AA64PFR2_EL1.FMR == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_LUT
    鐢?ID_AA64ISAR2_EL1.LUT == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_FAMINMAX
    鐢?ID_AA64ISAR3_EL1.FAMINMAX == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8CVT
    鐢?ID_AA64FPFR0_EL1.F8CVT == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8FMA
    鐢?ID_AA64FPFR0_EL1.F8FMA == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8DP4
    鐢?ID_AA64FPFR0_EL1.F8DP4 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8DP2
    鐢?ID_AA64FPFR0_EL1.F8DP2 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8E4M3
    鐢?ID_AA64FPFR0_EL1.F8E4M3 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_F8E5M2
    鐢?ID_AA64FPFR0_EL1.F8E5M2 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_LUTV2
    鐢?ID_AA64SMFR0_EL1.LUTv2 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_F8F16
    鐢?ID_AA64SMFR0_EL1.F8F16 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_F8F32
    鐢?ID_AA64SMFR0_EL1.F8F32 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_SF8FMA
    鐢?ID_AA64SMFR0_EL1.SF8FMA == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_SF8DP4
    鐢?ID_AA64SMFR0_EL1.SF8DP4 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_SF8DP2
    鐢?ID_AA64SMFR0_EL1.SF8DP2 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_SME_SF8DP4
    鐢?ID_AA64SMFR0_EL1.SF8DP4 == 0b1 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP2_POE
    鐢?ID_AA64MMFR3_EL1.S1POE == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP3_MTE_FAR
    鐢?ID_AA64PFR2_EL1.MTEFAR == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP3_MTE_STORE_ONLY
    鐢?ID_AA64PFR2_EL1.MTESTOREONLY == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP3_LSFE
    鐢?ID_AA64ISAR3_EL1.LSFE == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€?
HWCAP3_LS64
    鐢?ID_AA64ISAR1_EL1.LS64 == 0b0001 鎵€闅愬惈鐨勫姛鑳姐€傛敞鎰忥紝鎸囦护 ld64b/st64b 鐨勫姛鑳介渶瑕?CPU銆佺郴缁熶互鍙婄洰鏍囷紙璁惧锛夊唴瀛樹綅缃殑閰嶅悎鏀寔锛岃€?HWCAP3_LS64 浠呮殫绀轰簡瀵?CPU 鐨勬敮鎸併€傜敤鎴峰簲褰撳彧鍦ㄥ彈鏀寔鐨勭洰鏍囷紙璁惧锛夊唴瀛樹綅缃笂浣跨敤 ld64b/st64b锛屽惁鍒欏簲鍥為€€鍒伴潪鍘熷瓙鏇夸唬鏂规銆?
### 4. Unused AT_HWCAP bits锛堟湭浣跨敤鐨?AT_HWCAP 浣嶏級

涓轰簡涓庣敤鎴风┖闂翠簰鎿嶄綔锛屽唴鏍镐繚璇?AT_HWCAP 鐨勭 62 鍜?63 浣嶅皢濮嬬粓琚繑鍥炰负 0銆?