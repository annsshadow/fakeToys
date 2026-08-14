
## 鍐呮牳绾у紓甯稿鐞?

璇勮鐢?Joerg Pommnitz <joerg@raleigh.ibm.com> 鎾板啓

褰撲竴涓繘绋嬭繍琛屽湪鍐呮牳妯″紡鏃讹紝瀹冨父甯搁渶瑕佽闂敱涓嶅彲淇＄▼搴忎紶鍏ョ殑鐢ㄦ埛鎬佸唴瀛樺湴鍧€銆?涓轰簡淇濇姢鑷韩锛屽唴鏍稿繀椤绘牎楠岃鍦板潃銆?
鍦ㄨ緝鏃х殑 Linux 鐗堟湰涓紝杩欐槸閫氳繃

int verify_area(int type, const void * addr, unsigned long size)

鍑芥暟瀹屾垚鐨勶紙鍚庢潵宸茶 access_ok() 鍙栦唬锛夈€?
璇ュ嚱鏁版牎楠屼粠鍦板潃 'addr' 寮€濮嬨€佸ぇ灏忎负 'size' 鐨勫唴瀛樺尯鍩熷浜?type 涓寚瀹氱殑鎿嶄綔
锛堣鎴栧啓锛夋槸鍚﹀彲璁块棶銆備负姝わ紝verify_read 蹇呴』鏌ユ壘鍖呭惈鍦板潃 addr 鐨勮櫄鎷熷唴瀛樺尯鍩?锛坴ma锛夈€傚湪姝ｅ父鎯呭喌锛堟纭伐浣滅殑绋嬪簭锛変笅锛岃娴嬭瘯浼氭垚鍔熴€傚畠鍙灏戞暟鏈?bug 鐨勭▼搴?澶辫触銆傚湪鏌愪簺鍐呮牳鎬ц兘鍓栨瀽娴嬭瘯涓紝杩欎釜閫氬父涓嶅繀瑕佺殑鏍￠獙娑堣€椾簡鐩稿綋澶氱殑鏃堕棿銆?
涓轰簡鍏嬫湇杩欑鎯呭喌锛孡inus 鍐冲畾璁╂瘡涓敮鎸?Linux 鐨?CPU 涓兘瀛樺湪鐨勮櫄鎷熷唴瀛樼‖浠舵潵澶勭悊
杩欎釜娴嬭瘯銆?
杩欐槸濡備綍宸ヤ綔鐨勫憿锛?
姣忓綋鍐呮牳璇曞浘璁块棶涓€涓綋鍓嶄笉鍙闂殑鍦板潃鏃讹紝CPU 浼氫骇鐢熶竴涓〉閿欒寮傚父锛屽苟璋冪敤
```
  void exc_page_fault(struct pt_regs *regs, unsigned long error_code)
```
arch/x86/mm/fault.c 涓殑鍑芥暟銆傛爤涓婄殑鍙傛暟鐢?arch/x86/entry/entry_32.S 涓殑搴曞眰
姹囩紪鑳舵按浠ｇ爜璁剧疆銆傚弬鏁?regs 鏄寚鍚戞爤涓婂凡淇濆瓨瀵勫瓨鍣ㄧ殑鎸囬拡锛宔rror_code 鍖呭惈浜嗗紓甯哥殑
鍘熷洜鐮併€?
exc_page_fault() 棣栧厛浠?CPU 鎺у埗瀵勫瓨鍣?CR2 涓幏鍙栦笉鍙闂殑鍦板潃銆傚鏋滆鍦板潃浣嶄簬
杩涚▼鐨勮櫄鎷熷湴鍧€绌洪棿鍐咃紝閭ｄ箞鏁呴殰寰堝彲鑳芥槸鐢遍〉闈㈠皻鏈崲鍏ャ€佽鍐欎繚鎶ゆ垨绫讳技鎯呭喌寮曡捣鐨勩€?涓嶈繃锛屾垜浠劅鍏磋叮鐨勬槸鍙︿竴绉嶆儏鍐碉細璇ュ湴鍧€鏃犳晥锛屼笉瀛樺湪鍖呭惈姝ゅ湴鍧€鐨?vma銆傚湪杩欑鎯呭喌涓嬶紝
鍐呮牳璺宠浆鍒?bad_area 鏍囩銆?
鍦ㄩ偅閲岋紝瀹冧娇鐢ㄥ紩鍙戝紓甯哥殑鎸囦护鍦板潃锛堝嵆 regs->eip锛夋潵鏌ユ壘鍙互浠庝腑缁х画鎵ц锛坒ixup锛?鐨勫湴鍧€銆傚鏋滄煡鎵炬垚鍔燂紝鏁呴殰澶勭悊绋嬪簭浼氫慨鏀硅繑鍥炲湴鍧€锛堝悓鏍锋槸 regs->eip锛夊苟杩斿洖銆傛墽琛?灏嗗湪 fixup 涓殑鍦板潃澶勭户缁€?
fixup 鎸囧悜鍝噷锛?
鐢变簬鎴戜滑鏄烦杞埌 fixup 鐨勫唴瀹癸紝fixup 鏄剧劧鎸囧悜鍙墽琛岀殑浠ｇ爜銆傝繖娈典唬鐮侀殣钘忓湪鐢ㄦ埛璁块棶
瀹忓唴閮ㄣ€傛垜閫夊彇浜?arch/x86/include/asm/uaccess.h 涓畾涔夌殑 get_user() 瀹忎綔涓轰緥瀛愩€?鍏跺畾涔夋湁浜涢毦浠ョ悊瑙ｏ紝鎵€浠ヨ鎴戜滑鐪嬩竴涓嬮澶勭悊鍣ㄥ拰缂栬瘧鍣ㄧ敓鎴愮殑浠ｇ爜銆傛垜閫夊彇浜?drivers/char/sysrq.c 涓殑 get_user() 璋冪敤杩涜璇︾粏鍒嗘瀽銆?```
        get_user(c, buf);
```
```
  (
    {
      long __gu_err = - 14 , __gu_val = 0;
      const __typeof__(*( (  buf ) )) *__gu_addr = ((buf));
      if (((((0 + current_set[0])->tss.segment) == 0x18 )  ||
        (((sizeof(*(buf))) <= 0xC0000000UL) &&
        ((unsigned long)(__gu_addr ) <= 0xC0000000UL - (sizeof(*(buf)))))))
        do {
          __gu_err  = 0;
          switch ((sizeof(*(buf)))) {
            case 1:
              __asm__ __volatile__(
                "1:      mov" "b" " %2,%" "b" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "b" " %" "b" "1,%" "b" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"
                "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=q" (__gu_val): "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(  __gu_err  )) ;
                break;
            case 2:
              __asm__ __volatile__(
                "1:      mov" "w" " %2,%" "w" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "w" " %" "w" "1,%" "w" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"
                "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=r" (__gu_val) : "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(  __gu_err  ));
                break;
            case 4:
              __asm__ __volatile__(
                "1:      mov" "l" " %2,%" "" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "l" " %" "" "1,%" "" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"        "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=r" (__gu_val) : "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(__gu_err));
                break;
            default:
              (__gu_val) = __get_user_bad();
          }
        } while (0) ;
      ((c)) = (__typeof__(*((buf))))__gu_val;
      __gu_err;
    }
  );
```
鐪熷帀瀹筹紒鏅︽订鐨?GCC/姹囩紪榄旀硶銆傝繖鏍规湰娌℃硶璇绘噦锛屾墍浠ユ垜浠潵鐪?```
 >         xorl %edx,%edx
 >         movl current_set,%eax
 >         cmpl $24,788(%eax)
 >         je .L1424
 >         cmpl $-1073741825,64(%esp)
 >         ja .L1423
 > .L1424:
 >         movl %edx,%eax
 >         movl 64(%esp),%ebx
 > #APP
 > 1:      movb (%ebx),%dl                /* this is the actual user access */
 > 2:
 > .section .fixup,"ax"
 > 3:      movl $-14,%eax
 >         xorb %dl,%dl
 >         jmp 2b
 > .section __ex_table,"a"
 >         .align 4
 >         .long 1b,3b
 > .text
 > #NO_APP
 > .L1423:
 >         movzbl %dl,%esi
```
浼樺寲鍣ㄥ仛寰楀緢濂斤紝缁欎簡鎴戜滑涓€浜涘疄闄呰兘鐪嬫噦鐨勪笢瑗裤€傛槸鍚楋紵瀹為檯鐨勭敤鎴疯闂潪甯告槑鏄俱€傚浜?缁熶竴鐨勫湴鍧€绌洪棿锛屾垜浠彲浠ョ洿鎺ヨ闂敤鎴峰唴瀛樹腑鐨勫湴鍧€銆備絾閭ｄ簺 .section 鐨勪笢瑗挎槸骞插槢鐨勶紵锛燂紵锛?```
 > objdump --section-headers vmlinux
 >
 > vmlinux:     file format elf32-i386
 >
 > Sections:
 > Idx Name          Size      VMA       LMA       File off  Algn
 >   0 .text         00098f40  c0100000  c0100000  00001000  2**4
 >                   CONTENTS, ALLOC, LOAD, READONLY, CODE
 >   1 .fixup        000016bc  c0198f40  c0198f40  00099f40  2**0
 >                   CONTENTS, ALLOC, LOAD, READONLY, CODE
 >   2 .rodata       0000f127  c019a5fc  c019a5fc  0009b5fc  2**2
 >                   CONTENTS, ALLOC, LOAD, READONLY, DATA
 >   3 __ex_table    000015c0  c01a9724  c01a9724  000aa724  2**2
 >                   CONTENTS, ALLOC, LOAD, READONLY, DATA
 >   4 .data         0000ea58  c01abcf0  c01abcf0  000abcf0  2**4
 >                   CONTENTS, ALLOC, LOAD, DATA
 >   5 .bss          00018e21  c01ba748  c01ba748  000ba748  2**2
 >                   ALLOC
 >   6 .comment      00000ec4  00000000  00000000  000ba748  2**0
 >                   CONTENTS, READONLY
 >   7 .note         00001068  00000ec4  00000ec4  000bb60c  2**0
 >                   CONTENTS, READONLY
```
鏄剧劧锛岀敓鎴愮殑 obj 鏂囦欢涓湁 2 涓潪鏍囧噯鐨?ELF 鑺傘€備絾棣栧厛鎴戜滑鎯冲紕娓呮鎴戜滑鐨勪唬鐮佸湪
```
 > objdump --disassemble --section=.text vmlinux
 >
 > c017e785 <do_con_write+c1> xorl   %edx,%edx
 > c017e787 <do_con_write+c3> movl   0xc01c7bec,%eax
 > c017e78c <do_con_write+c8> cmpl   $0x18,0x314(%eax)
 > c017e793 <do_con_write+cf> je     c017e79f <do_con_write+db>
 > c017e795 <do_con_write+d1> cmpl   $0xbfffffff,0x40(%esp,1)
 > c017e79d <do_con_write+d9> ja     c017e7a7 <do_con_write+e3>
 > c017e79f <do_con_write+db> movl   %edx,%eax
 > c017e7a1 <do_con_write+dd> movl   0x40(%esp,1),%ebx
 > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl
 > c017e7a7 <do_con_write+e3> movzbl %dl,%esi
```
鏁翠釜鐢ㄦ埛鍐呭瓨璁块棶琚缉鍑忎负 10 鏉?x86 鏈哄櫒鎸囦护銆傝 .section 鎸囦护鎷捣鏉ョ殑鎸囦护涓嶅啀澶勪簬
姝ｅ父鐨勬墽琛岃矾寰勪腑銆傚畠浠綅浜庝竴涓笉鍚岀殑鑺傞噷
```
 > objdump --disassemble --section=.fixup vmlinux
 >
 > c0199ff5 <.fixup+10b5> movl   $0xfffffff2,%eax
 > c0199ffa <.fixup+10ba> xorb   %dl,%dl
 > c0199ffc <.fixup+10bc> jmp    c017e7a7 <do_con_write+e3>
```
```
 > objdump --full-contents --section=__ex_table vmlinux
 >
 >  c01aa7c4 93c017c0 e09f19c0 97c017c0 99c017c0  ................
 >  c01aa7d4 f6c217c0 e99f19c0 a5e717c0 f59f19c0  ................
 >  c01aa7e4 080a18c0 01a019c0 0a0a18c0 04a019c0  ................
```
```
 >  c01aa7c4 c017c093 c0199fe0 c017c097 c017c099  ................
 >  c01aa7d4 c017c2f6 c0199fe9 c017e7a5 c0199ff5  ................
                               ^^^^^^^^^^^^^^^^^
                               this is the interesting part!
 >  c01aa7e4 c0180a08 c019a001 c0180a0a c019a004  ................
```
```
  .section .fixup,"ax"
  .section __ex_table,"a"
```
鍛婅瘔姹囩紪鍣ㄥ皢鍚庨潰鐨勪唬鐮佺Щ鍔ㄥ埌鎸囧畾鐨?```
  3:      movl $-14,%eax
          xorb %dl,%dl
          jmp 2b
```
```
        .long 1b,3b
```
鏈€缁堝嚭鐜板湪鐩爣鏂囦欢鐨?__ex_table 鑺備腑銆?b 鍜?3b 鏄眬閮ㄦ爣绛俱€傚眬閮ㄦ爣绛?1b锛?b 琛ㄧず
鍚戝悗鏈€杩戠殑鏍囩 1锛夋槸鍙兘鍑洪敊鎸囦护鐨勫湴鍧€锛屽嵆鍦ㄦ垜浠殑鎯呭喌涓紝鏍囩 1 鐨勫湴鍧€涓?c017e7a5锛?鍘熷姹囩紪浠ｇ爜锛?> 1:      movb (%ebx),%dl
閾炬帴杩?vmlinux 鍚庯細 > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl

灞€閮ㄦ爣绛?3锛堝悓鏍峰悜鍚庯級鏄鐞嗘晠闅滅殑浠ｇ爜鐨勫湴鍧€锛屽湪鎴戜滑鐨勬儏鍐典腑瀹為檯鍊间负 c0199ff5锛?鍘熷姹囩紪浠ｇ爜锛?> 3:      movl $-14,%eax
閾炬帴杩?vmlinux 鍚庯細 > c0199ff5 <.fixup+10b5> movl   $0xfffffff2,%eax

濡傛灉 fixup 鑳藉澶勭悊璇ュ紓甯革紝鎺у埗娴佸彲浠ヨ繑鍥炲埌瑙﹀彂鏁呴殰鐨勯偅鏉℃寚浠や箣鍚庣殑鎸囦护锛屽嵆灞€閮?鏍囩 2b銆?```
 > .section __ex_table,"a"
 >         .align 4
 >         .long 1b,3b
```
```
 >  c01aa7d4 c017c2f6 c0199fe9 c017e7a5 c0199ff5  ................
                               ^this is ^this is
                               1b       3b
```
c017e7a5锛宑0199ff5 浣嶄簬鍐呮牳鐨勫紓甯歌〃涓€?
閭ｄ箞锛屽鏋滃彂鐢熶簡鏉ヨ嚜鍐呮牳妯″紡銆佷笖娌℃湁鍚堥€?vma 鐨勬晠闅滐紝瀹為檯浼氬彂鐢熶粈涔堝憿锛?```
    > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl
```
#. MMU 浜х敓寮傚父
#. CPU 璋冪敤 exc_page_fault()
#. exc_page_fault() 璋冪敤 do_user_addr_fault()
#. do_user_addr_fault() 璋冪敤 kernelmode_fixup_or_oops()
#. kernelmode_fixup_or_oops() 璋冪敤 fixup_exception()锛坮egs->eip == c017e7a5锛夛紱
#. fixup_exception() 璋冪敤 search_exception_tables()
#. search_exception_tables() 鍦ㄥ紓甯歌〃涓煡鎵惧湴鍧€ c017e7a5锛堝嵆 ELF 鑺?__ex_table
   鐨勫唴瀹癸級锛屽苟杩斿洖鐩稿叧鑱旂殑鏁呴殰澶勭悊浠ｇ爜鐨勫湴鍧€ c0199ff5銆?#. fixup_exception() 淇敼鑷韩鐨勮繑鍥炲湴鍧€浠ユ寚鍚戞晠闅滃鐞嗕唬鐮佸苟杩斿洖銆?#. 鎵ц鍦ㄦ晠闅滃鐞嗕唬鐮佷腑缁х画銆?#. a) EAX 鍙樹负 -EFAULT锛?= -14锛?   b) DL 鍙樹负闆讹紙鎴戜滑浠庣敤鎴风┖闂粹€滆鍙栤€濈殑鍊硷級
   c) 鎵ц鍦ㄥ眬閮ㄦ爣绛?2 澶勭户缁紙鍗崇揣鎺ュ湪寮曞彂鏁呴殰鐨勭敤鎴疯闂寚浠や箣鍚庣殑鎸囦护鐨勫湴鍧€锛夈€?
   涓婇潰 a 鍒?c 鐨勬楠ゅ湪鏌愮鎰忎箟涓婃ā鎷熶簡閭ｆ潯鍑洪敊鐨勬寚浠ゃ€?
澶т綋灏辨槸杩欐牱浜嗐€傚鏋滀綘鐪嬫垜浠殑渚嬪瓙锛屽彲鑳戒細闂负浠€涔堟垜浠湪寮傚父澶勭悊浠ｇ爜涓妸 EAX 璁?涓?-EFAULT銆傚棷锛実et_user() 瀹忓疄闄呬笂杩斿洖涓€涓€硷細鐢ㄦ埛璁块棶鎴愬姛鏃朵负 0锛屽け璐ユ椂
涓?-EFAULT銆傛垜浠師鏉ョ殑浠ｇ爜娌℃湁娴嬭瘯杩欎釜杩斿洖鍊硷紝浣?get_user() 涓殑鍐呰仈姹囩紪浠ｇ爜
灏濊瘯杩斿洖 -EFAULT銆侴CC 閫夋嫨浜?EAX 鏉ヨ繑鍥炶繖涓€笺€?
娉ㄦ剰锛?鐢变簬寮傚父琛ㄧ殑鏋勫缓鏂瑰紡浠ュ強闇€瑕佷繚鎸佹湁搴忥紝鍙 .text 鑺備腑鐨勪唬鐮佷娇鐢ㄥ紓甯搞€備换浣曞叾瀹?鑺傞兘浼氬鑷村紓甯歌〃鏃犳硶琚纭帓搴忥紝浠庤€屼娇寮傚父澶勭悊澶辫触銆?
褰?64 浣嶆敮鎸佽鍔犲叆 x86 Linux 鏃讹紝鎯呭喌鍙戠敓浜嗗彉鍖栥€備笌鍏堕€氳繃灏嗕袱涓潯鐩粠 32 浣嶆墿灞?鍒?64 浣嶆潵浣垮紓甯歌〃澶у皬缈诲€嶏紝涓嶅浣跨敤浜嗕竴涓阀濡欑殑鎶€宸э細灏嗗湴鍧€瀛樺偍涓虹浉瀵逛簬琛ㄦ湰韬殑
鍋忕Щ閲忋€傛眹缂栦唬鐮佷粠
```
    .long 1b,3b
  to:
          .long (from) - .
          .long (to) - .
```
鏀逛负浜嗕笂闈㈢殑褰㈠紡锛岃€屼娇鐢ㄨ繖浜涘€肩殑 C 浠ｇ爜灏嗗叾杞崲鍥炵粷瀵瑰湴鍧€
```
	ex_insn_addr(const struct exception_table_entry *x)
	{
		return (unsigned long)&x->insn + x->insn;
	}
```
鍦?v4.6 涓紝寮傚父琛ㄦ潯鐩鎵╁睍浜嗕竴涓柊鐨勫瓧娈?"handler"銆傚畠鍚屾牱鏄?32 浣嶅锛屽寘鍚竴涓?绗笁涓浉瀵瑰嚱鏁版寚閽堬紝鎸囧悜浠ヤ笅涔嬩竴锛?
1) `int ex_handler_default(const struct exception_table_entry *fixup)`
     杩欐槸浼犵粺鐨勬儏褰紝鍙槸璺宠浆鍒?fixup 浠ｇ爜

2) `int ex_handler_fault(const struct exception_table_entry *fixup)`
     杩欑鎯呭舰鎻愪緵鍦?entry->insn 澶勫彂鐢熺殑闄烽槺鐨勬晠闅滃彿銆傚畠鐢ㄤ簬鍖哄垎椤甸敊璇笌鏈哄櫒妫€鏌ャ€?
鍙互寰堝鏄撳湴娣诲姞鏇村鍑芥暟銆?
CONFIG_BUILDTIME_TABLE_SORT 鍏佽閫氳繃涓绘満宸ュ叿 scripts/sorttable 鍦ㄥ唴鏍搁暅鍍忛摼鎺ヤ箣鍚?瀵?__ex_table 鑺傝繘琛屾帓搴忋€傚畠浼氬皢绗﹀彿 main_extable_sort_needed 璁句负 0锛屼粠鑰岄伩鍏?鍦ㄥ惎鍔ㄦ椂瀵?__ex_table 鑺傝繘琛屾帓搴忋€傛湁浜嗘帓搴忓悗鐨勫紓甯歌〃锛屽湪杩愯鏃跺彂鐢熷紓甯告椂锛屾垜浠?鍙互閫氳繃浜屽垎鏌ユ壘蹇€熷畾浣?__ex_table 鏉＄洰銆?
杩欎笉浠呬粎鏄竴涓惎鍔ㄦ椂鐨勪紭鍖栵紝鏌愪簺鏋舵瀯瑕佹眰璇ヨ〃鏄湁搴忕殑锛屼互渚垮湪鍚姩杩囩▼涓浉褰撴棭鐨?闃舵灏辫兘澶勭悊寮傚父銆備緥濡傦紝i386 鐢氳嚦鍦ㄥ垎椤垫敮鎸佸皻鏈惎鐢ㄤ箣鍓嶅氨浣跨敤浜嗚繖绉嶅舰寮忕殑寮傚父
澶勭悊锛?