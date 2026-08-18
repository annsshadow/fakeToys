
## CEDT - CXL 鏃╂湡鍙戠幇琛紙CXL Early Discovery Table锛?

CXL 鏃╂湡鍙戠幇琛紙CXL Early Discovery Table锛夌敱 BIOS 鐢熸垚锛岀敤浜庢弿杩?BIOS 鍦ㄥ惎鍔ㄦ椂閰嶇疆鐨?CXL 鍐呭瓨鍖哄煙銆?
## CHBS


CXL 涓绘満妗ョ粨鏋勶紙CXL Host Bridge Structure锛夋弿杩?CXL 涓绘満妗ャ€傞櫎浜嗘弿杩拌澶囧瘎瀛樺櫒淇℃伅澶栵紝瀹冭繕鎶ュ憡姝や富鏈烘ˉ鐗瑰畾鐨勪富鏈烘ˉ UID銆傝繖浜涗富鏈烘ˉ ID 灏嗗湪鍏朵粬琛ㄤ腑琚紩鐢ㄣ€?
```

          Subtable Type : 00 [CXL Host Bridge Structure]
               Reserved : 00
                 Length : 0020
 Associated host bridge : 00000007    <- Host bridge _UID
  Specification version : 00000001
               Reserved : 00000000
          Register base : 0000010370400000
        Register length : 0000000000010000

```
## CFMWS


CXL 鍥哄畾鍐呭瓨绐楀彛缁撴瀯锛圕XL Fixed Memory Window structure锛夋弿杩颁笌涓€涓垨澶氫釜 CXL 涓绘満妗ワ紙濡?CHBS 鎵€杩帮級鍏宠仈鐨勫唴瀛樺尯鍩熴€傛澶栵紝瀹冭繕鎻忚堪浠讳綍鍙兘鐢?BIOS 缂栫▼鐨勪富鏈烘ˉ闂翠氦閿欙紙interleave锛夐厤缃€?
```

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 000000C050000000   <- Memory Region
              Window size : 0000003CA0000000
 Interleave Members (2^n) : 01                 <- Interleave configuration
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007           <- Host Bridge _UID
              Next Target : 00000006           <- Host Bridge _UID

```
restriction 瀛楁瑙勫畾姝?SPA 鑼冨洿鍙敤浜庝粈涔堬紙鍐呭瓨绫诲瀷锛夛紝
```

  Bit[0]: CXL Type 2 Memory
  Bit[1]: CXL Type 3 Memory
  Bit[2]: Volatile Memory
  Bit[3]: Persistent Memory
  Bit[4]: Fixed Config (HPA cannot be reused)

```
涓绘満妗ュ唴锛坕ntra-host-bridge锛変氦閿欙紙涓€涓富鏈烘ˉ涓婄殑澶氫釜璁惧锛変笉鍦ㄦ缁撴瀯涓姤鍛婏紝鑰屾槸瀹屽叏閫氳繃 CXL 璁惧瑙ｇ爜鍣ㄧ紪绋嬶紙涓绘満妗ヤ笌绔偣瑙ｇ爜鍣級瀹氫箟銆?