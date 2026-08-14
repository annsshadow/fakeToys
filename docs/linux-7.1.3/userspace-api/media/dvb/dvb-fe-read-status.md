######## 查询前端状态与统计信息


一旦调用 FE_SET_PROPERTY <FE_GET_PROPERTY>，前端将运行一个内核线程，周期性地检查调谐器锁定状态并提供信号质量的统计信息。

前端调谐器的锁定状态信息可通过 FE_READ_STATUS 查询。

信号统计信息通过 FE_GET_PROPERTY 提供。


   大多数统计信息要求解调器已完全锁定（例如设置了 `FE_HAS_LOCK <fe_status>` 位）。更多细节请参阅 Frontend statistics indicators <frontend-stat-properties>。
