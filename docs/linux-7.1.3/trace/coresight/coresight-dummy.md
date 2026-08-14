
## Coresight 铏氭嫙璺熻釜妯″潡


    :Author:   Hao Zhang <quic_hazha@quicinc.com>
    :Date:     June 2023

### 绠€浠?

Coresight 铏氭嫙璺熻釜妯″潡閫傜敤浜庡唴鏍告棤鏉冭闂垨閰嶇疆鐨勭壒瀹氳澶囷紝渚嬪
Qualcomm 骞冲彴涓婄殑 CoreSight TPDM銆傚浜庤繖浜涜澶囷紝闇€瑕佷竴涓櫄鎷熼┍鍔ㄧ▼搴?灏嗗畠浠敞鍐屼负 Coresight 璁惧銆傝妯″潡涔熷彲鐢ㄤ簬瀹氫箟鍙兘娌℃湁浠讳綍缂栫▼鎺ュ彛鐨?缁勪欢锛屼粠鑰屽彲浠ュ湪椹卞姩绋嬪簭涓垱寤鸿矾寰勩€傚畠涓鸿櫄鎷熻澶囦笂鐨勬搷浣滄彁渚?Coresight
API锛屼緥濡傚惎鐢ㄥ拰绂佺敤瀹冧滑銆傚畠杩樻彁渚涚敤浜庤皟璇曠殑 Coresight 铏氭嫙 sink/source
璺緞銆?
### 閰嶇疆璇︽儏


鏈変袱绉嶇被鍨嬬殑鑺傜偣锛氳櫄鎷?sink 鍜岃櫄鎷?source銆傝繖浜涜妭鐐逛綅浜?`/sys/bus/coresight/devices`銆?
```

    $ ls -l /sys/bus/coresight/devices | grep dummy
    dummy_sink0 -> ../../../devices/platform/soc@0/soc@0:sink/dummy_sink0
    dummy_source0 -> ../../../devices/platform/soc@0/soc@0:source/dummy_source0

```
