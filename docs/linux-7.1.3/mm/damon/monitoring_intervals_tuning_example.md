
## DAMON 监控间隔参数调优示例

DAMON 的监控参数需要根据给定的工作负载和监控目的进行调优。对此有一份 :ref:`调优指南 <damon_design_monitoring_params_tuning_guide>`。本文档根据该指南提供一个调优示例。

## 环境搭建

在下面的示例中，使用了 Linux 内核 v6.11 的 DAMON 以及 `damo <https://github.com/damonitor/damo>`_（DAMON 用户空间工具）v2.5.9，来监控并可视化运行真实服务器工作负载的系统物理地址空间上的访问模式。

## 5ms/100ms 间隔：间隔过短

让我们从使用 DAMON 捕获系统物理地址空间上的访问模式快照开始，使用默认的间隔参数（采样间隔和聚合间隔分别为 5 毫秒和 100 毫秒）。在 DAMON 启动与捕获快照之间等待十分钟，以展示有意义的时间维度访问模式。

```
    # damo start
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop

```
然后，列出 DAMON 发现的具有不同访问模式的区域，按 “访问温度”（access temperature）排序。“访问温度” 是一个表示区域访问热度（hotness）的指标。它计算为访问频率和区域存在时间（age）的加权和。如果访问频率为 0%，则温度乘以负一。也就是说，如果一个区域未被访问，它会得到负温度，且随着未被访问的时间变长而变得更低。排序按温度升序排列，因此位于列表顶部的区域是
```
    # damo report access --sort_regions_by temperature
    0   addr 16.052 GiB   size 5.985 GiB   access 0 %   age 5.900 s    # coldest
    1   addr 22.037 GiB   size 6.029 GiB   access 0 %   age 5.300 s
    2   addr 28.065 GiB   size 6.045 GiB   access 0 %   age 5.200 s
    3   addr 10.069 GiB   size 5.983 GiB   access 0 %   age 4.500 s
    4   addr 4.000 GiB    size 6.069 GiB   access 0 %   age 4.400 s
    5   addr 62.008 GiB   size 3.992 GiB   access 0 %   age 3.700 s
    6   addr 56.795 GiB   size 5.213 GiB   access 0 %   age 3.300 s
    7   addr 39.393 GiB   size 6.096 GiB   access 0 %   age 2.800 s
    8   addr 50.782 GiB   size 6.012 GiB   access 0 %   age 2.800 s
    9   addr 34.111 GiB   size 5.282 GiB   access 0 %   age 2.300 s
    10  addr 45.489 GiB   size 5.293 GiB   access 0 %   age 1.800 s    # hottest
    total size: 62.000 GiB

```
该列表显示没有明显的热区域，并且只有最小的访问模式多样性。每个区域的访问频率都是零。区域数量是 10，也就是默认的 `min_nr_regions` 值。每个区域的大小也几乎相同。我们可以怀疑这是因为 “自适应区域调整”（adaptive regions adjustment）机制没能很好地工作。正如指南所建议的，我们可以使用 `age` 作为近期性（recency）信息来获取区域的相对热度。这总比没有好，但鉴于最长存在时间只有约 6 秒，而我们等待了约十分钟，尚不清楚这有多大用处。

按温度范围的区域总大小直方图
```
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-,590,000,000, -,549,000,000) 5.985 GiB  |**********          |
    [-,549,000,000, -,508,000,000) 12.074 GiB |********************|
    [-,508,000,000, -,467,000,000) 0 B        |                    |
    [-,467,000,000, -,426,000,000) 12.052 GiB |********************|
    [-,426,000,000, -,385,000,000) 0 B        |                    |
    [-,385,000,000, -,344,000,000) 3.992 GiB  |*******             |
    [-,344,000,000, -,303,000,000) 5.213 GiB  |*********           |
    [-,303,000,000, -,262,000,000) 12.109 GiB |********************|
    [-,262,000,000, -,221,000,000) 5.282 GiB  |*********           |
    [-,221,000,000, -,180,000,000) 0 B        |                    |
    [-,180,000,000, -,139,000,000) 5.293 GiB  |*********           |
    total size: 62.000 GiB

```
简而言之，这些参数在检测热区域方面提供的监控结果质量很差。根据 :ref:`指南 <damon_design_monitoring_params_tuning_guide>`，这是由于聚合间隔过短。

## 100ms/2s 间隔：开始显现出小的热区域

遵循指南，将间隔增大 20 倍（100 毫秒和 2 秒）

```
    # damo start -s 100ms -a 2s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 10.180 GiB   size 6.117 GiB   access 0 %   age 7 m 8 s    # coldest
    1   addr 49.275 GiB   size 6.195 GiB   access 0 %   age 6 m 14 s
    2   addr 62.421 GiB   size 3.579 GiB   access 0 %   age 6 m 4 s
    3   addr 40.154 GiB   size 6.127 GiB   access 0 %   age 5 m 40 s
    4   addr 16.296 GiB   size 6.182 GiB   access 0 %   age 5 m 32 s
    5   addr 34.254 GiB   size 5.899 GiB   access 0 %   age 5 m 24 s
    6   addr 46.281 GiB   size 2.995 GiB   access 0 %   age 5 m 20 s
    7   addr 28.420 GiB   size 5.835 GiB   access 0 %   age 5 m 6 s
    8   addr 4.000 GiB    size 6.180 GiB   access 0 %   age 4 m 16 s
    9   addr 22.478 GiB   size 5.942 GiB   access 0 %   age 3 m 58 s
    10  addr 55.470 GiB   size 915.645 MiB access 0 %   age 3 m 6 s
    11  addr 56.364 GiB   size 6.056 GiB   access 0 %   age 2 m 8 s
    12  addr 56.364 GiB   size 4.000 KiB   access 95 %  age 16 s
    13  addr 49.275 GiB   size 4.000 KiB   access 100 % age 8 m 24 s   # hottest
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-42,800,000,000, -33,479,999,000) 22.018 GiB |*****************   |
    [-33,479,999,000, -24,159,998,000) 27.090 GiB |********************|
    [-24,159,998,000, -14,839,997,000) 6.836 GiB  |******              |
    [-14,839,997,000, -5,519,996,000)  6.056 GiB  |*****               |
    [-5,519,996,000, 3,800,005,000)    4.000 KiB  |*                   |
    [3,800,005,000, 13,120,006,000)    0 B        |                    |
    [13,120,006,000, 22,440,007,000)   0 B        |                    |
    [22,440,007,000, 31,760,008,000)   0 B        |                    |
    [31,760,008,000, 41,080,009,000)   0 B        |                    |
    [41,080,009,000, 50,400,010,000)   0 B        |                    |
    [50,400,010,000, 59,720,011,000)   4.000 KiB  |*                   |
    total size: 62.000 GiB

```
DAMON 发现了两个截然不同的、相当热的 4 KiB 区域。这些区域的存在时间也很好。最热的 4 KiB 区域保持访问频率约 8 分钟，最冷的区域保持无访问约 7 分钟。直方图上的分布看起来也像是有某种模式。

特别是，在总共 62 GiB 内存中发现 4 KiB 区域，表明 DAMON 的自适应区域调整正按设计工作。

不过，区域数量仍然接近 `min_nr_regions`，且冷区域的大小也相似。显然它有所改善，但仍有改进空间。

## 400ms/8s 间隔：相当不错的结果

将间隔再增大四倍（400 毫秒和 8 秒）

```
    # damo start -s 400ms -a 8s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 64.492 GiB   size 1.508 GiB   access 0 %   age 6 m 48 s    # coldest
    1   addr 21.749 GiB   size 5.674 GiB   access 0 %   age 6 m 8 s
    2   addr 27.422 GiB   size 5.801 GiB   access 0 %   age 6 m
    3   addr 49.431 GiB   size 8.675 GiB   access 0 %   age 5 m 28 s
    4   addr 33.223 GiB   size 5.645 GiB   access 0 %   age 5 m 12 s
    5   addr 58.321 GiB   size 6.170 GiB   access 0 %   age 5 m 4 s
    [...]
    25  addr 6.615 GiB    size 297.531 MiB access 15 %  age 0 ns
    26  addr 9.513 GiB    size 12.000 KiB  access 20 %  age 0 ns
    27  addr 9.511 GiB    size 108.000 KiB access 25 %  age 0 ns
    28  addr 9.513 GiB    size 20.000 KiB  access 25 %  age 0 ns
    29  addr 9.511 GiB    size 12.000 KiB  access 30 %  age 0 ns
    30  addr 9.520 GiB    size 4.000 KiB   access 40 %  age 0 ns
    [...]
    41  addr 9.520 GiB    size 4.000 KiB   access 80 %  age 56 s
    42  addr 9.511 GiB    size 12.000 KiB  access 100 % age 6 m 16 s
    43  addr 58.321 GiB   size 4.000 KiB   access 100 % age 6 m 24 s
    44  addr 9.512 GiB    size 4.000 KiB   access 100 % age 6 m 48 s
    45  addr 58.106 GiB   size 4.000 KiB   access 100 % age 6 m 48 s    # hottest
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-40,800,000,000, -32,639,999,000) 21.657 GiB  |********************|
    [-32,639,999,000, -24,479,998,000) 17.938 GiB  |*****************   |
    [-24,479,998,000, -16,319,997,000) 16.885 GiB  |****************    |
    [-16,319,997,000, -8,159,996,000)  586.879 MiB |*                   |
    [-8,159,996,000, 5,000)            4.946 GiB   |*****               |
    [5,000, 8,160,006,000)             260.000 KiB |*                   |
    [8,160,006,000, 16,320,007,000)    0 B         |                    |
    [16,320,007,000, 24,480,008,000)   0 B         |                    |
    [24,480,008,000, 32,640,009,000)   0 B         |                    |
    [32,640,009,000, 40,800,010,000)   16.000 KiB  |*                   |
    [40,800,010,000, 48,960,011,000)   8.000 KiB   |*                   |
    total size: 62.000 GiB

```
具有不同访问模式的区域数量显著增加。每个区域的大小也更具多样性。非零访问频率区域的总大小也显著增加。也许这已经足够好，可以带来一些有意义的存储器管理效率变化。

## 800ms/16s 间隔：另一种偏向

进一步将间隔翻倍（采样和聚合间隔分别为 800 毫秒和 16 秒）。结果对
```
    # damo start -s 800ms -a 16s
    # sleep 600
    # damo record --snapshot 0 1
    # damo stop
    # damo report access --sort_regions_by temperature
    0   addr 64.781 GiB   size 1.219 GiB   access 0 %   age 4 m 48 s
    1   addr 24.505 GiB   size 2.475 GiB   access 0 %   age 4 m 16 s
    2   addr 26.980 GiB   size 504.273 MiB access 0 %   age 4 m
    3   addr 29.443 GiB   size 2.462 GiB   access 0 %   age 4 m
    4   addr 37.264 GiB   size 5.645 GiB   access 0 %   age 4 m
    5   addr 31.905 GiB   size 5.359 GiB   access 0 %   age 3 m 44 s
    [...]
    20  addr 8.711 GiB    size 40.000 KiB  access 5 %   age 2 m 40 s
    21  addr 27.473 GiB   size 1.970 GiB   access 5 %   age 4 m
    22  addr 48.185 GiB   size 4.625 GiB   access 5 %   age 4 m
    23  addr 47.304 GiB   size 902.117 MiB access 10 %  age 4 m
    24  addr 8.711 GiB    size 4.000 KiB   access 100 % age 4 m
    25  addr 20.793 GiB   size 3.713 GiB   access 5 %   age 4 m 16 s
    26  addr 8.773 GiB    size 4.000 KiB   access 100 % age 4 m 16 s
    total size: 62.000 GiB
    # damo report access --style temperature-sz-hist
    <temperature> <total size>
    [-28,800,000,000, -23,359,999,000) 12.294 GiB  |*****************   |
    [-23,359,999,000, -17,919,998,000) 9.753 GiB   |*************       |
    [-17,919,998,000, -12,479,997,000) 15.131 GiB  |********************|
    [-12,479,997,000, -7,039,996,000)  0 B         |                    |
    [-7,039,996,000, -1,599,995,000)   7.506 GiB   |**********          |
    [-1,599,995,000, 3,840,006,000)    6.127 GiB   |*********           |
    [3,840,006,000, 9,280,007,000)     0 B         |                    |
    [9,280,007,000, 14,720,008,000)    136.000 KiB |*                   |
    [14,720,008,000, 20,160,009,000)   40.000 KiB  |*                   |
    [20,160,009,000, 25,600,010,000)   11.188 GiB  |***************     |
    [25,600,010,000, 31,040,011,000)   4.000 KiB   |*                   |
    total size: 62.000 GiB

```
它发现了更多非零访问频率的区域。区域数量仍远高于 `min_nr_regions`，但比之前配置的有所减少。而且显然分布看起来有点偏向热区域。

## 结论

根据上述实验性调优结果，我们可以得出结论：该理论和指南至少对这种工作负载是合理的，并且可以应用于类似情况。
