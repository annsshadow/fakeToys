## 鎺у埗缁勭粺璁★紙Control Groupstats锛?

Control Groupstats 鐨勭伒鎰熸潵鑷?https://lore.kernel.org/r/461CF883.2030308@sw.ru 鐨勮璁猴紝骞跺疄鐜颁簡 Andrew Morton 鍦?https://lore.kernel.org/r/20070411114927.1277d7c9.akpm@linux-foundation.org 涓缓璁殑鎸?cgroup 缁熻銆?
鎸?cgroup 缁熻鐨勫熀纭€璁炬柦澶嶇敤浜?taskstats 鎺ュ彛鐨勪唬鐮併€備竴缁勬柊鐨?cgroup 鎿嶄綔
浠?cgroup 鐗瑰畾鐨勫懡浠や笌灞炴€ф敞鍐屻€傞€氳繃鍚?cgroupstats 缁撴瀯娣诲姞鎴愬憳锛屾墿灞?鎸?cgroup 缁熻搴斿綋闈炲父瀹规槗銆?
cgroupstats 褰撳墠鐨勬ā鍨嬫槸鎷夊彇寮忥紝鎺ㄩ€佸紡妯″瀷锛堝湪鍙戠敓鏈夎叮浜嬩欢鏃朵笂鎶ョ粺璁★級搴斿綋
闈炲父瀹规槗娣诲姞銆傚綋鍓嶇敤鎴风┖闂撮€氳繃浼犻€?cgroup 璺緞鏉ヨ姹傜粺璁°€?鍏充簬 cgroup 涓墍鏈変换鍔＄姸鎬佺殑缁熻杩斿洖缁欑敤鎴风┖闂淬€?
娉ㄦ剰锛氱洰鍓嶆垜浠緷璧栧欢杩熺粺璁℃潵鎻愬彇琚?I/O 闃诲鐨勪换鍔′俊鎭€傚鏋滅鐢ㄤ簡
CONFIG_TASK_DELAY_ACCT锛岃淇℃伅灏嗕笉鍙敤銆?
瑕佹彁鍙?cgroup 缁熻锛屼娇鐢ㄤ竴涓笌 getdelays.c 闈炲父鐩镐技鐨勫伐鍏?```

  ~/balbir/cgroupstats # ./getdelays  -C "/sys/fs/cgroup/a"
  sleeping 1, blocked 0, running 1, stopped 0, uninterruptible 0
  ~/balbir/cgroupstats # ./getdelays  -C "/sys/fs/cgroup"
  sleeping 155, blocked 0, running 1, stopped 0, uninterruptible 2

```
