## 鍘熷瓙鎿嶄綔鎺у埗锛圓TOMCTL锛夊瘎瀛樺櫒


鎴戜滑鏈夊師瀛愭搷浣滄帶鍒讹紙ATOMCTL锛夊瘎瀛樺櫒銆?璇ュ瘎瀛樺櫒鍐冲畾浜嗗湪浣跨敤 S32C1I 鎸囦护鏃讹紝涓庝互涓嬪悇绉嶇粍鍚堟惌閰嶆墍浜х敓鐨勬晥鏋滐細

     1. 鏄惁甯︽湁鑳藉鍦ㄥ唴瀛樺唴閮ㄦ墽琛屽師瀛愪簨鍔★紙Atomic Transactions锛夌殑涓€鑷存€х紦瀛樻帶鍒跺櫒锛圕oherent Cache Controller锛夈€?
     2. 鏄惁甯︽湁鑳藉鑷鎵ц鍘熷瓙浜嬪姟鐨勬櫤鑳藉唴瀛樻帶鍒跺櫒锛圛ntelligent Memory Controller锛夈€?
```

      0x28: (WB: Internal, WT: Internal, BY:Exception)

```
鍦?FPGA 鍗′笂锛屾垜浠€氬父妯℃嫙涓€涓兘澶熸墽琛?RCW 浜嬪姟鐨勬櫤鑳藉唴瀛樻帶鍒跺櫒銆傚浜庡甫鏈夊閮ㄥ唴瀛樻帶鍒跺櫒鐨?FPGA 鍗★紝鎴戜滑鍦ㄦ墽琛岀紦瀛橈紙WB锛変簨鍔℃椂璁╁叾鍦ㄥ唴閮ㄥ畬鎴愬師瀛愭搷浣滐紝骞跺湪闈炵紦瀛樻搷浣滀腑浣跨敤鍐呭瓨 RCW銆?
瀵逛簬娌℃湁涓€鑷存€х紦瀛樻帶鍒跺櫒鐨勭郴缁燂紙闈?MX锛夛紝鎴戜滑濮嬬粓浣跨敤鍐呭瓨鎺у埗鍣ㄧ殑 RCW锛屽敖绠￠潪 MX 鎺у埗鍣ㄥ緢鍙兘鏀寔鍐呴儴鎿嶄綔銆?
CUSTOMER-WARNING锛堝鎴疯鍛婏級锛?   鍑犱箮鎵€鏈夊埗绋嬪鎴烽兘浠庝笉鏀寔鍘熷瓙 RCW 鍐呭瓨浜嬪姟鐨勪緵搴斿晢澶勮喘涔板唴瀛樻帶鍒跺櫒锛屽洜姝や粬浠緢鍙兘甯屾湜灏嗚瀵勫瓨鍣ㄩ厤缃负涓嶄娇鐢?RCW銆?
寮€鍙戜汉鍛樺彲鑳戒細鍙戠幇锛屽湪缂撳瓨琚梺璺紙bypass锛夌殑娴嬭瘯涓紙渚嬪鐮旂┒缂撳瓨鍒悕闂鏃讹級锛屼娇鐢?RCW 鐨勬梺璺紙Bypass锛夋ā寮忎細姣旇緝鏂逛究銆?
```

                             WB     WT      BY
                           5   4 | 3   2 | 1   0

```
=========    ==================      ==================      ===============
  2 Bit
  Field
  Values     WB - 鍥炲啓(Write Back)    WT - 鐩村啓(Write Thru)    BY - 鏃佽矾(Bypass)
=========    ==================      ==================      ===============
    0        Exception               Exception               Exception
    1        RCW Transaction         RCW Transaction         RCW Transaction
    2        Internal Operation      Internal Operation      Reserved
    3        Reserved                Reserved                Reserved
=========    ==================      ==================      ===============
