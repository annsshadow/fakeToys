
## DAMON：数据访问监控和访问感知系统操作


DAMON 是一个 Linux 内核子系统，用于高效的数据访问监控
<damon_design_monitoring>` and :ref:`访问感知系统操作
<damon_design_damos>`。  它的设计目的是

 - **准确**（对于 DRAM 级内存管理），
 - **轻量级**（用于生产在线使用），
 - **可扩展**（就内存大小而言），
 - **可调**（用于灵活使用），以及
 - **自动化**（用于生产操作，无需手动调整）。

- [faq](faq)
- [design](design)
- [api](api)
- [maintainer-profile](maintainer-profile)

要从用户空间使用和控制 DAMON，请参阅
管理[guide </admin-guide/mm/damon/index>](guide </admin-guide/mm/damon/index>)。

如果您更喜欢学术论文阅读和引用，请使用论文
从 `HPDC'22 <https://dl.acm.org/doi/abs/10.1145/3502181.3531466>`_ 和
`Middleware19 Industry <https://dl.acm.org/doi/abs/10.1145/3366626.3368125>`_ .
请注意，这些涵盖了 Linux v5.16 和 v5.15 中的 DAMON 实现，
分别。
