## NIC SR-IOV API


强烈建议现代 NIC 聚焦于实现 `switchdev` 模型（参见 switchdev），以配置 SR-IOV 功能的转发与安全性。

## 传统 API


旧的 SR-IOV API 在 `rtnetlink` Netlink 族中实现，作为 `RTM_GETLINK` 和 `RTM_SETLINK` 命令的一部分。在驱动侧，它由若干 `ndo_set_vf_**` 和 `ndo_get_vf_**` 回调组成。

由于传统 API 与协议栈其余部分的集成不佳，该 API 被视为冻结状态；不会接受任何新功能或扩展。新的驱动不应实现那些不常见的回调；即以下回调在限制之外（不得使用）：

 - `ndo_get_vf_port`
 - `ndo_set_vf_port`
 - `ndo_set_vf_rss_query_en`
