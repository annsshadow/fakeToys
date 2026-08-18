锘?
## 璋冭皭鍣紙Tuner锛夐┍鍔?

鏈枃浠嬬粛 V4L2 濯掍綋瀛愮郴缁熶笅璋冭皭鍣紙Tuner锛夐┍鍔ㄧ殑缂栫▼鎺ュ彛锛岃鏄庝笉鍚岄鏍硷紙LG銆丳hilips銆乀emic銆丄LPS 绛夛級璋冭皭鍣ㄧ殑棰戞鍒囨崲瀛楄妭宸紓锛屼互鍙婁富瑕佽皟璋愬櫒鍒堕€犲晢鐨勫瀷鍙锋爣璇嗚鍒欙紝渚涢┍鍔ㄥ紑鍙戣€呭弬鑰冦€?



### 绠€鍗曠殑璋冭皭鍣ㄧ紪绋?


鏈夊嚑绉嶄笉鍚岄鏍肩殑璋冭皭鍣ㄧ紪绋?API銆?
瀹冧滑鐨勪富瑕佸尯鍒湪浜庨娈靛垏鎹㈠瓧鑺傘€?

- L= LG_API       (VHF_LO=0x01, VHF_HI=0x02, UHF=0x08, radio=0x04)
- P= PHILIPS_API  (VHF_LO=0xA0, VHF_HI=0x90, UHF=0x30, radio=0x04)
- T= TEMIC_API    (VHF_LO=0x02, VHF_HI=0x04, UHF=0x01)
- A= ALPS_API     (VHF_LO=0x14, VHF_HI=0x12, UHF=0x11)
- M= PHILIPS_MK3  (VHF_LO=0x01, VHF_HI=0x02, UHF=0x04, radio=0x19)

### 璋冭皭鍣ㄥ埗閫犲晢


- Samsung 璋冭皭鍣ㄦ爣璇嗭細锛堜緥濡?TCPM9091PD27锛?


 TCP [ABCJLMNQ] 90[^89^][^125^] [DP] [ACD] 27 [ABCD]
 [ABCJLMNQ]:
   A= BG+DK
   B= BG
   C= I+DK
   J= NTSC-Japan
   L= Secam LL
   M= BG+I+DK
   N= NTSC
   Q= BG+I+DK+LL
 [^89^]: ?
 [^125^]:
   2: 鏃?FM
   5: 甯?FM
 [DP]:
   D= NTSC
   P= PAL
 [ACD]:
   A= F 杩炴帴鍣?
   C= Phono 杩炴帴鍣?
   D= Din 鎻掑骇
 [ABCD]:
   3 绾?I2C 璋冭皭锛? 棰戞/3 棰戞

杩欎簺璋冭皭鍣ㄤ笌 PHILIPS_API 鍏煎銆?

Philips 璋冭皭鍣ㄦ爣璇嗭細锛堜緥濡?FM1216MF锛?


  F[IRMQ]12[^1345^]6{MF|ME|MP}
  F[IRMQ]:
   FI12x6: 璋冭皭鍣ㄧ郴鍒?
   FR12x6: 璋冭皭鍣?+ 鏀堕煶鏈?IF
   FM12x6: 璋冭皭鍣?+ FM
   FQ12x6: 鐗规畩
   FMR12x6: 鐗规畩
   TD15xx: 鏁板瓧璋冭皭鍣?ATSC
  12[^1345^]6:
   1216: PAL BG
   1236: NTSC
   1246: PAL I
   1256: Pal DK
  {MF|ME|MP}
   MF: BG LL 甯?Secam锛圡ulti France锛?
   ME: BG DK I LL   (Multi Europe)
   MP: BG DK I      (Multi PAL)
   MR: BG DK M 锛?
   MG: BG DKI M 锛?
  MK2 绯诲垪 PHILIPS_API锛屽ぇ澶氭暟璋冭皭鍣ㄤ笌姝ゅ吋瀹癸紒
  MK3 绯诲垪浜?2002 骞村紩鍏ワ紝浣跨敤 PHILIPS_MK3_API

Temic 璋冭皭鍣ㄦ爣璇嗭細锛堜緥濡?4006FH5锛?


   4[^01^][^0136^][^269^]F[HYNR]5
    40x2: 璋冭皭鍣?(5V/33V)锛孴EMIC_API銆?
    40x6: 璋冭皭鍣?5V
    41xx: 璋冭皭鍣?compact
    40x9: 璋冭皭鍣?FM compact
   [^0136^]
    xx0x: PAL BG
    xx1x: Pal DK, Secam LL
    xx3x: NTSC
    xx6x: PAL I
   F[HYNR]5
    FH5: Pal BG
    FY5: 鍏朵粬
    FN5: 澶氭爣鍑?
    FR5: 甯?FM 鏀堕煶鏈?
   3X xxxx: 甯︽湁鐗瑰畾杩炴帴鍣ㄧ殑璁㈠崟鍙?
  娉ㄦ剰锛氬彧鏈?40x2 绯诲垪浣跨敤 TEMIC_API锛屾墍鏈夋洿鏂扮殑璋冭皭鍣ㄩ兘浣跨敤 PHILIPS_API銆?

LG Innotek 璋冭皭鍣細

- TPI8NSR11 : NTSC J/M    (TPI8NSR01 w/FM)  (P,210/497)
- TPI8PSB11 : PAL B/G     (TPI8PSB01 w/FM)  (P,170/450)
- TAPC-I701 : PAL I       (TAPC-I001 w/FM)  (P,170/450)
- TPI8PSB12 : PAL D/K+B/G (TPI8PSB02 w/FM)  (P,170/450)
- TAPC-H701P: NTSC_JP     (TAPC-H001P w/FM) (L,170/450)
- TAPC-G701P: PAL B/G     (TAPC-G001P w/FM) (L,170/450)
- TAPC-W701P: PAL I       (TAPC-W001P w/FM) (L,170/450)
- TAPC-Q703P: PAL D/K     (TAPC-Q001P w/FM) (L,170/450)
- TAPC-Q704P: PAL D/K+I   (L,170/450)
- TAPC-G702P: PAL D/K+B/G (L,170/450)

- TADC-H002F: NTSC (L,175/410?; 2-B, C-W+11, W+12-69)
- TADC-M201D: PAL D/K+B/G+I (L,143/425)  (澹伴煶鎺у埗鍦?I2C 鍦板潃 0xc8)
- TADC-T003F: NTSC 鍙版咕  (L,175/410?; 2-B, C-W+11, W+12-69)

鍚庣紑锛?
  - P= 鏍囧噯 phono 姣嶅骇
  - D= IEC 姣嶅骇
  - F= F 杩炴帴鍣?

鍏朵粬璋冭皭鍣細

- TCL2002MB-1 : PAL BG + DK       =TUNER_LG_PAL_NEW_TAPC
- TCL2002MB-1F: PAL BG + DK w/FM  =PHILIPS_PAL
- TCL2002MI-2 : PAL I		= ??

ALPS 璋冭皭鍣細

- 澶у鏁颁笌 LG_API 鍏煎
- TSCH6 浣跨敤 ALPS_API锛圱SCH5 ?锛?
- TSBE1 鏈夐澶栫殑 API 05,02,08 鎺у埗瀛楄妭=0xCB 鏉ユ簮:[#f1]_

