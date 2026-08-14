## 涓€鑷存€ц澶囧睘鎬ц〃锛圕DAT锛?

CDAT 鎻愪緵璇稿 CXL 鍔犻€熷櫒銆佷氦鎹㈡満鎴栫鐐圭瓑璁惧鐨勫姛鑳戒笌鎬ц兘灞炴€с€傚叾琛ㄦ牸寮忕被浼间簬
ACPI 琛ㄣ€侰DAT 鏁版嵁鍙敱 BIOS 鍦ㄥ惎鍔ㄦ椂瑙ｆ瀽锛屼篃鍙湪杩愯鏃舵灇涓撅紙渚嬪璁惧鐑彃鎷斾箣鍚庯級銆?
鏈锛?DPA 鈥?璁惧鐗╃悊鍦板潃锛圖evice Physical Address锛夛紝鐢?CXL 璁惧鐢ㄦ潵琛ㄧず璇ヨ澶囨墍鏀寔鐨勫湴鍧€銆?
DSMADHandle 鈥?涓庣敱 DSMAS 琛ㄥ畾涔夌殑 DPA 鑼冨洿鐩稿叧鑱旂殑璁惧鍞竴鍙ユ焺銆?

## 璁惧浣滅敤鍩熷唴瀛樹翰鍜屾€х粨鏋勶紙DSMAS锛?

DSMAS 鍖呭惈璇稿 DSMADHandle銆丏PA 鍩哄湴鍧€锛圖PA Base锛夊拰 DPA 闀垮害锛圖PA Length锛夌瓑淇℃伅銆?
璇ヨ〃鐢?Linux 涓庤澶囦綔鐢ㄥ煙寤惰繜涓庡甫瀹戒俊鎭粨鏋勶紙DSLBIS锛夌粨鍚堜娇鐢紝浠ョ‘瀹?CXL 璁惧
鑷韩鐨勬€ц兘灞炴€с€?
```

 Structure Type : 00 [DSMAS]
       Reserved : 00
         Length : 0018              <- 24d, size of structure
    DSMADHandle : 01
          Flags : 00
       Reserved : 0000
       DPA Base : 0000000040000000  <- 1GiB base
     DPA Length : 0000000080000000  <- 2GiB size


```
## 璁惧浣滅敤鍩熷欢杩熶笌甯﹀淇℃伅缁撴瀯锛圖SLBIS锛?

璇ヨ〃鐢?Linux 涓?DSMAS 缁撳悎浣跨敤锛屼互纭畾 CXL 璁惧鐨勬€ц兘灞炴€с€侱SLBIS 鍖呭惈鍩轰簬
DSMADHandle 鍖归厤鐨勫欢杩熶笌甯﹀淇℃伅銆?
```

   Structure Type : 01 [DSLBIS]
         Reserved : 00
           Length : 18                     <- 24d, size of structure
           Handle : 0001                   <- DSMAS handle
            Flags : 00                     <- Matches flag field for HMAT SLLBIS
        Data Type : 00                     <- Latency
 Entry Basee Unit : 0000000000001000       <- Entry Base Unit field in HMAT SSLBIS
            Entry : 010000000000           <- First byte used here, CXL LTC
         Reserved : 0000

   Structure Type : 01 [DSLBIS]
         Reserved : 00
           Length : 18                     <- 24d, size of structure
           Handle : 0001                   <- DSMAS handle
            Flags : 00                     <- Matches flag field for HMAT SLLBIS
        Data Type : 03                     <- Bandwidth
 Entry Basee Unit : 0000000000001000       <- Entry Base Unit field in HMAT SSLBIS
            Entry : 020000000000           <- First byte used here, CXL BW
         Reserved : 0000


```
## 浜ゆ崲鏈轰綔鐢ㄥ煙寤惰繜涓庡甫瀹戒俊鎭粨鏋勶紙SSLBIS锛?

SSLBIS 鍖呭惈鏈夊叧浜ゆ崲鏈哄欢杩熷拰甯﹀鐨勪俊鎭€?
璇ヨ〃鐢?Linux 鐢ㄤ簬璁＄畻浠庤澶囧埌鏍圭鍙ｇ殑 CXL 璺緞鐨勬€ц兘鍧愭爣锛屽叾涓氦鎹㈡満鏄矾寰勭殑涓€閮ㄥ垎銆?
```

  Structure Type : 05 [SSLBIS]
        Reserved : 00
          Length : 20                           <- 32d, length of record, including SSLB entries
       Data Type : 00                           <- Latency
        Reserved : 000000
 Entry Base Unit : 00000000000000001000         <- Matches Entry Base Unit in HMAT SSLBIS

                                                <- SSLB Entry 0
       Port X ID : 0100                         <- First port, 0100h represents an upstream port
       Port Y ID : 0000                         <- Second port, downstream port 0
         Latency : 0100                         <- Port latency
        Reserved : 0000
                                                <- SSLB Entry 1
       Port X ID : 0100
       Port Y ID : 0001
         Latency : 0100
        Reserved : 0000


  Structure Type : 05 [SSLBIS]
        Reserved : 00
          Length : 18                           <- 24d, length of record, including SSLB entry
       Data Type : 03                           <- Bandwidth
        Reserved : 000000
 Entry Base Unit : 00000000000000001000         <- Matches Entry Base Unit in HMAT SSLBIS

                                                <- SSLB Entry 0
       Port X ID : 0100                         <- First port, 0100h represents an upstream port
       Port Y ID : FFFF                         <- Second port, FFFFh indicates any port
       Bandwidth : 1200                         <- Port bandwidth
        Reserved : 0000

```
CXL 椹卞姩缁撳悎浣跨敤 CDAT銆丠MAT銆丼RAT 浠ュ強鍏朵粬鏁版嵁锛屼负 CXL 璁惧鐢熸垚鈥滄暣鏉¤矾寰勬€ц兘鈥濇暟鎹€?