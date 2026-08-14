
## rtla-osnoise-hist

### 鏄剧ず osnoise 璺熻釜鍣ㄩ噰鏍风粨鏋滅殑鐩存柟鍥?

:Manual section: 1

## 姒傝锛圫YNOPSIS锛?

**rtla osnoise hist** [**OPTIONS**]

## 鎻忚堪锛圖ESCRIPTION锛?

**rtla osnoise hist** 宸ュ叿灏嗘墍鏈夌殑 **osnoise:sample_threshold** 浜嬩欢鍙戠敓鎯呭喌鏀堕泦
鍒颁竴寮犵洿鏂瑰浘涓紝骞朵互瀵圭敤鎴峰弸濂界殑鏂瑰紡鏄剧ず缁撴灉銆傝宸ュ叿杩樺厑璁稿 **osnoise** 璺熻釜鍣?杩涜璁稿閰嶇疆骞舵敹闆嗚窡韪櫒杈撳嚭銆?
## 閫夐」锛圤PTIONS锛?

## 绀轰緥锛圗XAMPLE锛?

鍦ㄤ笅闈㈢殑绀轰緥涓紝**osnoise** 璺熻釜鍣ㄧ嚎绋嬭璁剧疆涓轰互瀹炴椂浼樺厛绾?**FIFO:1** 鍦?CPUs **0-11** 涓婅繍琛岋紝姣忎釜鍛ㄦ湡杩愯 **900ms**锛堥粯璁?**1s**锛夈€傜缉鐭繍琛屾椂闂寸殑鍘熷洜
鏄负浜嗛伩鍏嶉タ姝?**rtla** 宸ュ叿銆傝宸ュ叿杩樿璁剧疆涓鸿繍琛?**one minute**銆傝緭鍑哄涓嬶細

```

  [root@f34 ~/]# rtla osnoise hist -P F:1 -c 0-11 -r 900000 -d 1M -b 10 -E 25
  # RTLA osnoise histogram
  # Time unit is microseconds (us)
  # Duration:   0 00:01:00
  Index   CPU-000   CPU-001   CPU-002   CPU-003   CPU-004   CPU-005   CPU-006   CPU-007   CPU-008   CPU-009   CPU-010   CPU-011
  0         42982     46287     51779     53740     52024     44817     49898     36500     50408     50128     49523     52377
  10        12224      8356      2912       878      2667     10155      4573     18894      4214      4836      5708      2413
  20            8         5        12         2        13        24        20        41        29        53        39        39
  30            1         1         0         0        10         3         6        19        15        31        30        38
  40            0         0         0         0         0         4         2         7         2         3         8        11
  50            0         0         0         0         0         0         0         0         0         1         1         2
  over:         0         0         0         0         0         0         0         0         0         0         0         0
  count:    55215     54649     54703     54620     54714     55003     54499     55461     54668     55052     55309     54880
  min:          0         0         0         0         0         0         0         0         0         0         0         0
  avg:          0         0         0         0         0         0         0         0         0         0         0         0
  max:         30        30        20        20        30        40        40        40        40        50        50        50

```
## 鍙﹁鍙傞槄锛圫EE ALSO锛?

**rtla-osnoise**\(1), **rtla-osnoise-top**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 浣滆€咃紙AUTHOR锛?

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
