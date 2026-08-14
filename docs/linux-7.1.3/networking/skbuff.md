## struct sk_buff


`sk_buff` 是表示数据包的主要网络结构。

### 基本 sk_buff 几何布局


   :doc: 基本 sk_buff 几何布局

### 共享 skb 与 skb 克隆


:c`sk_buff.users` 是一个简单的引用计数，允许多个实体保持 struct sk_buff 存活。具有 `sk_buff.users != 1` 的 skb 被称为共享 skb（见 skb_shared()）。

skb_clone() 允许快速复制 skb。没有任何数据缓冲区被复制，但调用者会获得一个新的元数据结构体（struct sk_buff）。
&skb_shared_info.refcount 表示指向同一数据包数据（即克隆）的 skb 数量。

### dataref 与无头部 skb


   :doc: dataref 与无头部 skb

### 校验和信息


   :doc: skb 校验和
