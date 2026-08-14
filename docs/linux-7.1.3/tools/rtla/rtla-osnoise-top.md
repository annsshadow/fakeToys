
## rtla-osnoise-top

### 鏄剧ず鎿嶄綔绯荤粺鍣０锛坥perating system noise锛夌殑鎽樿


:Manual section: 1

## 姒傝锛圫YNOPSIS锛?
**rtla osnoise top** [**OPTIONS**]

## 鎻忚堪锛圖ESCRIPTION锛?

**rtla osnoise top** 浠?**osnoise** tracer 鏀堕泦鍛ㄦ湡鎬ф憳瑕侊紝鍖呮嫭骞叉壈婧愶紙interference source锛夊彂鐢熺殑璁℃暟锛屽苟浠ョ敤鎴峰弸濂界殑鏍煎紡鏄剧ず缁撴灉銆?
璇ュ伐鍏疯繕鍏佽瀵?**osnoise** tracer 杩涜璁稿閰嶇疆浠ュ強鏀堕泦 tracer 鐨勮緭鍑恒€?
## 閫夐」锛圤PTIONS锛?


## 绀轰緥锛圗XAMPLE锛?
鍦ㄤ笅闈㈢殑绀轰緥涓紝**rtla osnoise top** 宸ュ叿琚缃负浠ュ疄鏃朵紭鍏堢骇 **FIFO:1**锛屽湪 CPU **0-3** 涓婅繍琛岋紝姣忎釜鍛ㄦ湡杩愯 **900ms**锛堥粯璁?**1s**锛夈€傚噺灏戣繍琛屾椂闂寸殑鍘熷洜鏄负浜嗛伩鍏嶄娇 rtla 宸ュ叿楗挎銆傝宸ュ叿杩樿璁剧疆涓鸿繍琛?**涓€鍒嗛挓**锛屽苟鏄剧ず
```

  [root@f34 ~]# rtla osnoise top -P F:1 -c 0-3 -r 900000 -d 1M -q
                                          Operating System Noise
  duration:   0 00:01:00 | time is in us
  CPU Period       Runtime        Noise  % CPU Aval   Max Noise   Max Single          HW          NMI          IRQ      Softirq       Thread
    0 #59         53100000       304896    99.42580        6978           56         549            0        53111         1590           13
    1 #59         53100000       338339    99.36282        8092           24         399            0        53130         1448           31
    2 #59         53100000       290842    99.45227        6582           39         855            0        53110         1406           12
    3 #59         53100000       204935    99.61405        6251           33         290            0        53156         1460           12

```
## 鍙傝锛圫EE ALSO锛?

**rtla-osnoise**\(1), **rtla-osnoise-hist**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 浣滆€咃紙AUTHOR锛?

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
