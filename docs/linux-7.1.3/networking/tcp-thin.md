
## 薄流（Thin-streams）与 TCP


广泛的一类使用可靠传输协议的、基于互联网的服务，表现出我们称之为薄流的特性这意味着应用程序以如此低的速率发送数据，以至于传输协议的重传机制无法完全
发挥作用。在对时间敏感的场景（如在线游戏、控制系统、股票交易等）中，用体验取决于数据交付的延迟，丢包可能对服务质量造成灾难性影响。极高的延迟源于
TCP 依赖来自应用程序的新数据到达，从而通过快速重传（fast retransmit）有触发重传，而不是等待长时间的超时
在分析了大量对时间敏感的交互式应用后，我们发现它们经常产生薄流，并且在其
整个生命周期中都保持这种流量模式。时间敏感性与这些流在使用 TCP 时引发高
延迟这一事实的结合，是令人遗憾的
为了减少数据包丢失时的应用层延迟，人们提出了一组机制，用于解决薄流的这延迟问题。简而言之，如果内核检测到薄流，重传机制会按以下方式修改：

1) 如果是薄流，在第一个重ACK 时即进行快速重传2) 如果是薄流，不应用指数退避
这些增强仅在流被检测为薄流时才会应用。这是通过为飞行中（in flight）的数据数量定义一个阈值来实现的。如果飞行中的数据包少于 4 个，就无法触发快速重传，
并且该流容易遭遇较高的重传延迟
由于这些机制针对的是对时间敏感的应用，它们必须由应用程序使用
TCP_THIN_LINEAR_TIMEOUTS TCP_THIN_DUPACK IOCTLS，或 tcp_thin_linear_timeouts
tcp_thin_dupack sysctl 来显式激活。这两项修改默认都是关闭的
## 参考资

有关这些修改以及大量实验数据的更多信息，可在此处找到
鈥淚mproving latency for interactive, thin-stream applications over
reliable transport鈥?http://simula.no/research/nd/publications/Simula.nd.477/simula_pdf_file
