## Monitor stall锛堝仠婊炰换鍔＄洃瑙嗗櫒锛?


- 鍚嶇О锛歴tall - 鍋滄粸浠诲姟鐩戣鍣?
- 绫诲瀷锛氭瘡浠诲姟娣峰悎鑷姩鏈?
- 浣滆€咃細Gabriele Monaco <gmonaco@redhat.com>

### 鎻忚堪


鍋滄粸浠诲姟锛坰tall锛夌洃瑙嗗櫒鏄竴涓ず渚嬫€х殑姣忎换鍔″畾鏃剁洃瑙嗗櫒锛岀敤浜庢鏌?
```

                        |
                        |
                        v
                      #==========================#
  +-----------------> H         dequeued         H
  |                   #==========================#
  |                     |
 sched_switch_wait      | sched_wakeup;reset(clk)
  |                     v
  |                   +--------------------------+ <+
  |                   |         enqueued         |  | sched_wakeup
  |                   | clk < threshold_jiffies  | -+
  |                   +--------------------------+
  |                     |                 ^
  |              sched_switch_in    sched_switch_preempt;reset(clk)
  |                     v                 |
  |                   +--------------------------+
  +------------------ |         running          |
                      +--------------------------+
                        ^ sched_switch_in      |
                        | sched_wakeup         |
                        +----------------------+

```
闃堝€煎彲浣滀负涓€涓弬鏁拌繘琛岄厤缃紝鏃㈠彲浠ラ€氳繃鍦ㄥ唴鏍稿惎鍔ㄦ椂浼犲叆
`stall.threshold_jiffies=<鏂板€?` 鍙傛暟锛屼篃鍙互鍚?
`/sys/module/stall/parameters/threshold_jiffies` 鍐欏叆鏂板€笺€?

### 瑙勬牸璇存槑

Graphviz Dot 鏂囦欢浣嶄簬 tools/verification/models/stall.dot
