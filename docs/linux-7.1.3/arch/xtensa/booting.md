## 向内核传递启动参数


启动参数在内存中表示为 TLV 列表。struct bp_tag 与标签值常量的定义请参阅
arch/xtensa/include/asm/bootparam.h。列表中的第一个条目必须具有类型
BP_TAG_FIRST，最后一个条目必须具有类型 BP_TAG_LAST。第一个列表条目的地址
通过寄存器 a2 传递给内核。地址类型取决于 MMU 类型：

- 对于无 MMU、带区域保护或带 MPU 的配置，地址必须是物理地址。
- 对于带区域转换 MMU 或带 MMUv3 且 CONFIG_MMU=n 的配置，地址必须是当前映射中
  的有效地址。内核不会自行更改映射。
- 对于带 MMUv2 的配置，地址必须是默认虚拟映射（0xd0000000..0xffffffff）中的
  虚拟地址。
- 对于带 MMUv3 且 CONFIG_MMU=y 的配置，地址可以是虚拟或物理地址。无论哪种
  情况，它都必须在默认虚拟映射范围内。如果它在默认 KSEG 映射覆盖的物理地址
  范围内（XCHAL_KSEG_PADDR..XCHAL_KSEG_PADDR + XCHAL_KSEG_SIZE），则视为物理
  地址，否则视为虚拟地址。
