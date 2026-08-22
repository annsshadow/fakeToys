## struct sk_buff


`sk_buff` 是表示数据包的主要网络结构

### 基本 sk_buff 几何布局


   :doc: 基本 sk_buff 几何布局

### 共享 skb skb 克隆


:c`sk_buff.users` 是一个简单的引用计数，允许多个实体保struct sk_buff 存活。具`sk_buff.users != 1` skb 被称为共skb（见 skb_shared()）

skb_clone() 允许快速复skb。没有任何数据缓冲区被复制，但调用者会获得一个新的元数据结构体（struct sk_buff）
&skb_shared_info.refcount 表示指向同一数据包数据（即克隆）skb 数量

### dataref 与无头部 skb


   :doc: dataref 与无头部 skb

### 鏍￠獙鍜屼俊鎭。


   :doc: skb 鏍￠獙鍜。
