## SPI NOR 框架


### 如何提议新增一款 flash


大多数 SPI NOR flash 都符合 JEDEC JESD216 串行闪存可发现参数（SFDP）标准。SFDP 通过一组标准的内部只读参数表来描述串行闪存设备的功能与特性能力。

SPI NOR 驱动会查询 SFDP 表，以确定 flash 的参数与设置。如果 flash 定义了 SFDP 表，很可能你根本不需要一条 flash 表项，而是依赖通用 flash 驱动——它仅根据 flash 的 SFDP 数据进行探测。你只需在设备树中指定 "jedec,spi-nor" 这一 compatible 即可。

不过，在某些情况下你需要显式定义一条 flash 表项。这通常发生在 flash 拥有 SFDP 表未涵盖的设置或支持（例如块保护，Block Protection）时，或者 flash 包含的 SFDP 数据被破坏时。若是后者，需要实现 `spi_nor_fixups` 钩子，以便用正确的值修正 SFDP 参数。

### 最低测试要求


执行下面所有测试，并将它们粘贴到提交的注释部分中、`---` 标记之后。

1) 说明你用来测试该 flash 的控制器，并说明
```

    This flash is populated on the X board and was tested at Y
    frequency using the Z (put compatible) SPI controller.

```
```

    root@1:~# cat /sys/bus/spi/devices/spi0.0/spi-nor/partname
    sst26vf064b
    root@1:~# cat /sys/bus/spi/devices/spi0.0/spi-nor/jedec_id
    bf2643
    root@1:~# cat /sys/bus/spi/devices/spi0.0/spi-nor/manufacturer
    sst
    root@1:~# xxd -p /sys/bus/spi/devices/spi0.0/spi-nor/sfdp
    53464450060102ff00060110300000ff81000106000100ffbf0001180002
    0001fffffffffffffffffffffffffffffffffd20f1ffffffff0344eb086b
    083b80bbfeffffffffff00ffffff440b0c200dd80fd810d820914824806f
    1d81ed0f773830b030b0f7ffffff29c25cfff030c080ffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    fffffffffffffffffffffffffffffffffffffff0004fff37f0000f57f0000f9ff
    7d00f57f0000f37f0000ffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ffffbf2643ffb95ffdff30f260f332ff0a122346ff0f19320f1919ffffff
    ffffffff00669938ff05013506040232b03072428de89888a585c09faf5a
    ffff06ec060c0003080bffffffffff07ffff0202ff060300fdfd040700fc
    0300fefe0202070e
    root@1:~# sha256sum /sys/bus/spi/devices/spi0.0/spi-nor/sfdp
    428f34d0461876f189ac97f93e68a05fa6428c6650b3b7baf736a921e5898ed1  /sys/bus/spi/devices/spi0.0/spi-nor/sfdp

   Please dump the SFDP tables using ``xxd -p``. It enables us to do
   the reverse operation and convert the hexdump to binary with
   ``xxd -rp``. Dumping the SFDP data with ``hexdump -Cv`` is accepted,
   but less desirable.

```
```

    root@1:~# cat /sys/kernel/debug/spi-nor/spi0.0/capabilities
    Supported read modes by the flash
     1S-1S-1S
      opcode		0x03
      mode cycles	0
      dummy cycles	0
     1S-1S-1S (fast read)
      opcode		0x0b
      mode cycles	0
      dummy cycles	8
     1S-1S-2S
      opcode		0x3b
      mode cycles	0
      dummy cycles	8
     1S-2S-2S
      opcode		0xbb
      mode cycles	4
      dummy cycles	0
     1S-1S-4S
      opcode		0x6b
      mode cycles	0
      dummy cycles	8
     1S-4S-4S
      opcode		0xeb
      mode cycles	2
      dummy cycles	4
     4S-4S-4S
      opcode		0x0b
      mode cycles	2
      dummy cycles	4

    Supported page program modes by the flash
     1S-1S-1S
      opcode	0x02

    root@1:~# cat /sys/kernel/debug/spi-nor/spi0.0/params
    name		sst26vf064b
    id			bf 26 43 bf 26 43
    size		8.00 MiB
    write size		1
    page size		256
    address nbytes	3
    flags		HAS_LOCK | HAS_16BIT_SR | SOFT_RESET | SWP_IS_VOLATILE

    opcodes
     read		0xeb
      dummy cycles	6
     erase		0x20
     program		0x02
     8D extension	none

    protocols
     read		1S-4S-4S
     write		1S-1S-1S
     register		1S-1S-1S

    erase commands
     20 (4.00 KiB) [0]
     d8 (8.00 KiB) [1]
     d8 (32.0 KiB) [2]
     d8 (64.0 KiB) [3]
     c7 (8.00 MiB)

    sector map
     region (in hex)   | erase mask | flags
     ------------------+------------+----------
     00000000-00007fff |     [01  ] |
     00008000-0000ffff |     [0 2 ] |
     00010000-007effff |     [0  3] |
     007f0000-007f7fff |     [0 2 ] |
     007f8000-007fffff |     [01  ] |

```
4) Use `mtd-utils <https://git.infradead.org/mtd-utils.git>`__
```

    root@1:~# dd if=/dev/urandom of=./spi_test bs=1M count=2
    2+0 records in
    2+0 records out
    2097152 bytes (2.1 MB, 2.0 MiB) copied, 0.848566 s, 2.5 MB/s

    root@1:~# mtd_debug erase /dev/mtd0 0 2097152
    Erased 2097152 bytes from address 0x00000000 in flash

    root@1:~# mtd_debug read /dev/mtd0 0 2097152 spi_read
    Copied 2097152 bytes from address 0x00000000 in flash to spi_read

    root@1:~# hexdump spi_read
    0000000 ffff ffff ffff ffff ffff ffff ffff ffff
    *
    0200000

    root@1:~# sha256sum spi_read
    4bda3a28f4ffe603c0ec1258c0034d65a1a0d35ab7bd523a834608adabf03cc5  spi_read

    root@1:~# mtd_debug write /dev/mtd0 0 2097152 spi_test
    Copied 2097152 bytes from spi_test to address 0x00000000 in flash

    root@1:~# mtd_debug read /dev/mtd0 0 2097152 spi_read
    Copied 2097152 bytes from address 0x00000000 in flash to spi_read

    root@1:~# sha256sum spi*
    c444216a6ba2a4a66cccd60a0dd062bce4b865dd52b200ef5e21838c4b899ac8  spi_read
    c444216a6ba2a4a66cccd60a0dd062bce4b865dd52b200ef5e21838c4b899ac8  spi_test

   If the flash comes erased by default and the previous erase was ignored,
   we won't catch it, thus test the erase again::

    root@1:~# mtd_debug erase /dev/mtd0 0 2097152
    Erased 2097152 bytes from address 0x00000000 in flash

    root@1:~# mtd_debug read /dev/mtd0 0 2097152 spi_read
    Copied 2097152 bytes from address 0x00000000 in flash to spi_read

    root@1:~# sha256sum spi*
    4bda3a28f4ffe603c0ec1258c0034d65a1a0d35ab7bd523a834608adabf03cc5  spi_read
    c444216a6ba2a4a66cccd60a0dd062bce4b865dd52b200ef5e21838c4b899ac8  spi_test

   Dump some other relevant data::

    root@1:~# mtd_debug info /dev/mtd0
    mtd.type = MTD_NORFLASH
    mtd.flags = MTD_CAP_NORFLASH
    mtd.size = 8388608 (8M)
    mtd.erasesize = 4096 (4K)
    mtd.writesize = 1
    mtd.oobsize = 0
    regions = 0

```
