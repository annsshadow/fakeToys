## 待办（TODO）


在分配用于写入 aoe 存储的 struct sk_buff 时，存在发生死锁的可能性。如果数据是从脏页写出以释放该页，且没有其他可用页面，那么在需要空闲页用于 sk_buff 分配时可能发生死锁。这种情况尚未被观察到，但最好能在内存压力下消除任何死锁的可能性。

由于 ATA over Ethernet 不会被内核的 IP 代码分片，struct sk_buff 的 destructor 成员可供 aoe 驱动使用。通过为除前几个之外的所有 sk_buff 分配使用内存池（mempool），并注册一个析构函数，我们应该能够高效地分配 sk_buff，而不会引入任何死锁的可能性。
