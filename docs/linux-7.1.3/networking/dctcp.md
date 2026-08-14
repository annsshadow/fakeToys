## DCTCP（数据中心 TCP）


DCTCP 是对用于数据中心网络的 TCP 拥塞控制算法的增强，它利用数据中心网络中的
显式拥塞通知（ECN）向终端主机提供多位反馈。

```

  sysctl -w net.ipv4.tcp_congestion_control=dctcp
  sysctl -w net.ipv4.tcp_ecn_fallback=0 (optional)

```
运行 DCTCP 的数据中心网络中的所有交换机必须支持 ECN 标记，并被配置为在达到
定义的交换机缓冲区阈值时进行标记。交换机上 DCTCP 的默认 ECN 标记阈值启发式
值为 1Gbps 时 20 个数据包（30KB），10Gbps 时 65 个数据包（约 100KB），但可能
需要进一步仔细调整。

有关更多细节，请参阅以下文档：

论文：

该算法在以下两篇 SIGCOMM/SIGMETRICS 论文中有进一步详细描述：

 i) Mohammad Alizadeh, Albert Greenberg, David A. Maltz, Jitendra Padhye,
    Parveen Patel, Balaji Prabhakar, Sudipta Sengupta, and Murari Sridharan:

      "Data Center TCP (DCTCP)", Data Center Networks session"

      Proc. ACM SIGCOMM, New Delhi, 2010.

    http://simula.stanford.edu/~alizade/Site/DCTCP_files/dctcp-final.pdf
    http://www.sigcomm.org/ccr/papers/2010/October/1851275.1851192

ii) Mohammad Alizadeh, Adel Javanmard, and Balaji Prabhakar:

      "Analysis of DCTCP: Stability, Convergence, and Fairness"
      Proc. ACM SIGMETRICS, San Jose, 2011.

    http://simula.stanford.edu/~alizade/Site/DCTCP_files/dctcp_analysis-full.pdf

IETF 信息性草案：

  http://tools.ietf.org/html/draft-bensley-tcpm-dctcp-00

DCTCP 站点：

  http://simula.stanford.edu/~alizade/Site/DCTCP.html
