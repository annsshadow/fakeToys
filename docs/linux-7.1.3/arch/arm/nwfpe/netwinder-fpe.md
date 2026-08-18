## Current State


涓嬮潰鎻忚堪浜?NetWinder 娴偣浠跨湡鍣ㄧ殑褰撳墠鐘舵€併€?
涓嬮潰鐨勫懡鍚嶆硶鐢ㄤ簬鎻忚堪娴偣鎸囦护銆傚畠閬靛惊 ARM 鎵嬪唽涓殑绾﹀畾銆?
```

  <S|D|E> = <single|double|extended>, no default
  {P|M|Z} = {round to +infinity,round to -infinity,round to zero},
            default = round to nearest

```
娉ㄦ剰锛歿} 鎷捣鏉ョ殑椤规槸鍙€夌殑銆?
### Floating Point Coprocessor Data Transfer Instructions (CPDT)


LDF/STF - 鍔犺浇涓庡瓨鍌ㄦ诞鐐规暟鎹?
<LDF|STF>{cond}<S|D|E> Fd, Rn
<LDF|STF>{cond}<S|D|E> Fd, [Rn, #<expression>]{!}
<LDF|STF>{cond}<S|D|E> Fd, [Rn], #<expression>

杩欎簺鎸囦护宸插畬鏁村疄鐜般€?
LFM/SFM - 鍔犺浇涓庡瓨鍌ㄥ涓诞鐐规暟鎹?
Form 1 璇硶锛?<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn]
<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn, #<expression>]{!}
<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn], #<expression>

Form 2 璇硶锛?<LFM|SFM>{cond}<FD,EA> Fd, <count>, [Rn]{!}

杩欎簺鎸囦护宸插畬鏁村疄鐜般€傚畠浠负姣忎釜娴偣瀵勫瓨鍣ㄥ悜鎸囦护缁欏畾鐨勫唴瀛樹綅缃瓨鍌?鍔犺浇涓変釜瀛椼€傚唴瀛樹腑鐨勬牸寮忎笉澶彲鑳戒笌鍏朵粬瀹炵幇锛堝挨鍏舵槸瀹為檯纭欢锛夊吋瀹广€侫RM 鎵嬪唽涓姝ゆ湁鐗瑰埆璇存槑銆?
### Floating Point Coprocessor Register Transfer Instructions (CPRT)


杞崲銆佽/鍐欑姸鎬?鎺у埗瀵勫瓨鍣ㄦ寚浠?
FLT{cond}<S,D,E>{P,M,Z} Fn, Rd          Convert integer to floating point
FIX{cond}{P,M,Z} Rd, Fn                 Convert floating point to integer
WFS{cond} Rd                            Write floating point status register
RFS{cond} Rd                            Read floating point status register
WFC{cond} Rd                            Write floating point control register
RFC{cond} Rd                            Read floating point control register

FLT/FIX 宸插畬鏁村疄鐜般€?
RFS/WFS 宸插畬鏁村疄鐜般€?
RFC/WFC 宸插畬鏁村疄鐜般€俁FC/WFC 鏄粎 supervisor 鎸囦护锛岀洰鍓嶄細妫€鏌?CPU 妯″紡锛岃嫢涓嶆槸浠?supervisor 妯″紡璋冪敤鍒欎骇鐢熼潪娉曟寚浠ら櫡闃便€?
Compare 鎸囦护

CMF{cond} Fn, Fm        Compare floating
CMFE{cond} Fn, Fm       Compare floating with exception
CNF{cond} Fn, Fm        Compare negated floating
CNFE{cond} Fn, Fm       Compare negated floating with exception

杩欎簺鍧囧凡瀹屾暣瀹炵幇銆?
### Floating Point Coprocessor Data Instructions (CPDT)


鍙岀洰杩愮畻锛?
ADF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - add
SUF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - subtract
RSF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse subtract
MUF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - multiply
DVF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - divide
RDV{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse divide

杩欎簺鍧囧凡瀹屾暣瀹炵幇銆?
FML{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast multiply
FDV{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast divide
FRD{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast reverse divide

杩欎簺涔熼兘宸插畬鏁村疄鐜般€傚畠浠娇鐢ㄤ笌闈炲揩閫熺増鏈浉鍚岀殑绠楁硶銆傚洜姝わ紝鍦ㄦ湰瀹炵幇涓畠浠殑鎬ц兘绛夊悓浜?MUF/DVF/RDV 鎸囦护銆傝繖绗﹀悎 ARM 鎵嬪唽鐨勮瀹氥€傛墜鍐屾寚鍑鸿繖浜涗粎閽堝鍗曠簿搴︽搷浣滄暟瀹氫箟锛屽湪瀹為檯鐨?FPA11 纭欢涓婂畠浠鍙岀簿搴︽垨鎵╁睍绮惧害鎿嶄綔鏁版棤鏁堛€備豢鐪熷櫒鐩墠涓嶆鏌ユ墍璇锋眰鐨勬潈闄愭潯浠讹紝鑰屾槸鐩存帴鎵ц鎵€璇锋眰鐨勬搷浣溿€?
RMF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - IEEE remainder

杩欏凡瀹屾暣瀹炵幇銆?
鍗曠洰杩愮畻锛?
MVF{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - move
MNF{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - move negated

杩欎簺鍧囧凡瀹屾暣瀹炵幇銆?
ABS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - absolute value
SQT{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - square root
RND{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - round

杩欎簺鍧囧凡瀹屾暣瀹炵幇銆?
URD{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - unnormalized round
NRM{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - normalize

杩欎簺宸插疄鐜般€俇RD 浣跨敤涓?RND 鎸囦护鐩稿悓鐨勪唬鐮佸疄鐜般€傜敱浜?URD 涓嶈兘杩斿洖闈炶鏍煎寲鏁帮紝NRM 鍙樻垚浜嗙┖鎿嶄綔锛圢OP锛夈€?
搴撹皟鐢細

POW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - power
RPW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse power
POL{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - polar angle (arctan2)

LOG{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base 10
LGN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base e
EXP{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - exponent
SIN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - sine
COS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - cosine
TAN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - tangent
ASN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arcsine
ACS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arccosine
ATN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arctangent

杩欎簺灏氭湭瀹炵幇銆傜紪璇戝櫒褰撳墠涓嶄細鍙戝嚭瀹冧滑锛岃€屾槸鐢?libc 涓殑渚嬬▼澶勭悊銆侳PA11 纭欢涔熸湭瀹炵幇瀹冧滑锛岃€屾槸鐢辨诞鐐规敮鎸佷唬鐮佸鐞嗐€傚畠浠簲鍦ㄦ湭鏉ョ殑鐗堟湰涓疄鐜般€?
Signalling锛?
淇″彿宸插疄鐜般€傜劧鑰屽綋鍓嶇敱 Rebel.com 鐢熸垚鐨?ELF 鍐呮牳涓瓨鍦ㄤ竴涓?bug锛屽鑷磋妯″潡鏃犳硶鐢熸垚 SIGFPE銆傝繖鏄敱浜庢湭鑳芥纭湴鎶?fp_current 鍒悕鍒板唴鏍稿彉閲?current_set[^0^]銆?
鏈彂琛岀増鑷甫鐨勫唴鏍革紙vmlinux-nwfpe-0.93锛夊寘鍚拡瀵硅闂鐨勪慨澶嶏紝骞朵笖鐩存帴闆嗘垚浜嗗綋鍓嶇増鏈殑浠跨湡鍣ㄣ€備娇鐢ㄨ鍐呮牳鍙互涓嶅姞杞戒换浣曟诞鐐规ā鍧楄繍琛屻€傚畠浣滀负璇ユ妧鏈殑婕旂ず锛屼互鍙婁负閭ｄ簺渚濊禆淇″彿杩涜娴偣宸ヤ綔鐨勪汉鑰屾彁渚涖€備娇鐢ㄦā鍧楀苟闈炰弗鏍艰姹傘€?
涓€涓ā鍧楋紙鐢?Russell King 鎻愪緵鐨勶紝鎴栨湰鍙戣鐗堜腑鐨勶級鍙互琚姞杞戒互鏇挎崲鍐呭缓浜庡唴鏍哥殑浠跨湡鍣ㄥ姛鑳姐€?