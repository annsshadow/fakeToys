## Highpoint RocketRAID 3xxx/4xxx 适配器驱动（hptiop）


### 控制器寄存器映射


对于基于 RR44xx Intel IOP 的适配器，控制器 IOP 通过 PCI BAR0 与 BAR2 访问

     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
            0x11C5C Link Interface IRQ Set
            0x11C60 Link Interface IRQ Clear
     ============== ==================================

     ============== ==================================
     BAR2 offset    Register
     ============== ==================================
            0x10    Inbound Message Register 0
            0x14    Inbound Message Register 1
            0x18    Outbound Message Register 0
            0x1C    Outbound Message Register 1
            0x20    Inbound Doorbell Register
            0x24    Inbound Interrupt Status Register
            0x28    Inbound Interrupt Mask Register
            0x30    Outbound Interrupt Status Register
            0x34    Outbound Interrupt Mask Register
            0x40    Inbound Queue Port
            0x44    Outbound Queue Port
     ============== ==================================

对于基于 Intel IOP 的适配器，控制器 IOP 通过 PCI BAR0 访问：

     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
            0x10    Inbound Message Register 0
            0x14    Inbound Message Register 1
            0x18    Outbound Message Register 0
            0x1C    Outbound Message Register 1
            0x20    Inbound Doorbell Register
            0x24    Inbound Interrupt Status Register
            0x28    Inbound Interrupt Mask Register
            0x30    Outbound Interrupt Status Register
            0x34    Outbound Interrupt Mask Register
            0x40    Inbound Queue Port
            0x44    Outbound Queue Port
     ============== ==================================

对于基于 Marvell（非 Frey）IOP 的适配器，IOP 通过 PCI BAR0 与 BAR1 访问：

     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
         0x20400    Inbound Doorbell Register
         0x20404    Inbound Interrupt Mask Register
         0x20408    Outbound Doorbell Register
         0x2040C    Outbound Interrupt Mask Register
     ============== ==================================

     ============== ==================================
     BAR1 offset    Register
     ============== ==================================
             0x0    Inbound Queue Head Pointer
             0x4    Inbound Queue Tail Pointer
             0x8    Outbound Queue Head Pointer
             0xC    Outbound Queue Tail Pointer
            0x10    Inbound Message Register
            0x14    Outbound Message Register
     0x40-0x1040    Inbound Queue
     0x1040-0x2040  Outbound Queue
     ============== ==================================

对于基于 Marvell Frey IOP 的适配器，IOP 通过 PCI BAR0 与 BAR1 访问：

     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
             0x0    IOP configuration information.
     ============== ==================================

     ============== ===================================================
     BAR1 offset    Register
     ============== ===================================================
          0x4000    Inbound List Base Address Low
          0x4004    Inbound List Base Address High
          0x4018    Inbound List Write Pointer
          0x402C    Inbound List Configuration and Control
          0x4050    Outbound List Base Address Low
          0x4054    Outbound List Base Address High
          0x4058    Outbound List Copy Pointer Shadow Base Address Low
          0x405C    Outbound List Copy Pointer Shadow Base Address High
          0x4088    Outbound List Interrupt Cause
          0x408C    Outbound List Interrupt Enable
         0x1020C    PCIe Function 0 Interrupt Enable
         0x10400    PCIe Function 0 to CPU Message A
         0x10420    CPU to PCIe Function 0 Message A
         0x10480    CPU to PCIe Function 0 Doorbell
         0x10484    CPU to PCIe Function 0 Doorbell Enable
     ============== ===================================================


### 非 Marvell Frey 的 I/O 请求工作流


所有排队的请求都通过入站/出站队列端口处理。
请求包可以在 IOP 或主机内存中分配。

要向控制器发送请求：

    - 通过读取入站队列端口获取一个空闲请求包，或
      在主机 DMA 一致性内存中分配一个空闲请求。

      从入站队列端口返回的值是一个相对于 IOP BAR0 的偏移量。

      在主机内存中分配的请求必须按 32 字节边界对齐。

    - 填充该包。

    - 通过将包写入入站队列将其投递给 IOP。对于在 IOP 内存中分配的请求，
      将偏移量写入入站队列端口。对于在主机内存中分配的请求，将 (0x80000000|(bus_addr>>5))
      写入入站队列端口。

    - IOP 处理该请求。当请求完成时，它将被放入出站队列。将产生一个出站中断。

      对于在 IOP 内存中分配的请求，请求偏移量被投递到出站队列。

      对于在主机内存中分配的请求，(0x80000000|(bus_addr>>5))
      被投递到出站队列。如果请求中设置了 IOP_REQUEST_FLAG_OUTPUT_CONTEXT 标志，
      则改为投递低 32 位上下文值。

    - 主机读取出站队列并完成请求。

      对于在 IOP 内存中分配的请求，主机驱动通过将其写入出站队列来释放该请求。

非排队请求（reset/flush 等）可以通过入站消息寄存器 0 发送。带有相同值的出站消息表示
入站消息的完成。


### Marvell Frey 的 I/O 请求工作流


所有排队的请求都通过入站/出站列表处理。

要向控制器发送请求：

    - 在主机 DMA 一致性内存中分配一个空闲请求。

      在主机内存中分配的请求必须按 32 字节边界对齐。

    - 用请求在标志中的索引填充请求。

      用一个空闲入站列表单元填充请求的物理地址与大小。

      用前一个单元的索引设置入站列表写指针，当索引达到支持的请求计数时回绕到 0。

    - 将入站列表写指针投递给 IOP。

    - IOP 处理该请求。当请求完成时，带有或运算了 IOPMU_QUEUE_MASK_HOST_BITS 标志的请求将被放入一个
      空闲出站列表单元，并且出站列表单元的索引将被放入复制指针影子（copy pointer shadow）寄存器。将产生一个出站中断。

    - 主机读取出站列表复制指针影子寄存器，并与之前保存的读指针 N 比较。如果它们不同，主机将
      读取第 (N+1) 个出站列表单元。

      主机从第 (N+1) 个出站列表单元获取请求的索引并完成该请求。

非排队请求（reset communication/reset/flush 等）可以通过 PCIe Function 0 to CPU Message A 寄存器发送。带有相同值的
CPU to PCIe Function 0 Message 寄存器表示该消息的完成。


### 用户级接口


该驱动导出以下 sysfs 属性：

     ==================   ===    ========================
     NAME                 R/W    Description
     ==================   ===    ========================
     driver-version        R     driver version string
     firmware-version      R     firmware version string
     ==================   ===    ========================


-----------------------------------------------------------------------------

Copyright |copy| 2006-2012 HighPoint Technologies, Inc. All Rights Reserved.

  本文件以“希望它有用”的方式分发，
  但 WITHOUT ANY WARRANTY（不提供任何担保）；甚至不暗示对
  MERCHANTABILITY（适销性）或 FITNESS FOR A PARTICULAR PURPOSE（特定用途适用性）的担保。详见
  GNU General Public License。

  linux@highpoint-tech.com

  http://www.highpoint-tech.com
