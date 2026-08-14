## TC Actions - 环境规则


任何新 tc actions 作者的“环境”规则如下：

1) 如果你窃取或借用任何数据包，你便偏离了正道，应当进行克隆（cloneth）。

   例如，若你的 action 将数据包排队以便稍后处理，或故意通过重定向数据包来分支，则需要克隆该数据包。

2) 如果你修改（munge）任何数据包，在他人正引用该 skb 的情况下，应当调用 pskb_expand_head。此后你便“拥有”该 skb。

3) 丢弃你不拥有的数据包是禁止的。只需向调用者返回 TC_ACT_SHOT，由它们来丢弃。

针对 action 调用者（qdiscs 等）的“环境”规则如下：

#) 你有责任释放任何以 TC_ACT_SHOT/STOLEN/QUEUED 形式返回的对象。如果未返回任何 TC_ACT_SHOT/STOLEN/QUEUED，则一切正常，无需做任何处理。

若有不清楚之处，请在 netdev 上发帖。
