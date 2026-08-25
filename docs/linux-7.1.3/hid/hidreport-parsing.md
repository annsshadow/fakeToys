
## 手动解析 HID 报告描述

再次考虑鼠标 HID 报告描述```

  $ hexdump -C /sys/bus/hid/devices/0003\:093A\:2510.0002/report_descriptor
  00000000  05 01 09 02 a1 01 09 01  a1 00 05 09 19 01 29 03  |..............).|
  00000010  15 00 25 01 75 01 95 03  81 02 75 05 95 01 81 01  |..%.u.....u.....|
  00000020  05 01 09 30 09 31 09 38  15 81 25 7f 75 08 95 03  |...0.1.8..%.u...|
  00000030  81 06 c0 c0                                       |....|
  00000034

```
并尝试手动解析它
从第一个数0x05 开始：它用 2 位表示项的长度，2 位表示项的类型，4 位表```

  +----------+
  | 00000101 |
  +----------+
          ^^
          ---- Length of data (see HID spec 6.2.2.2)
        ^^
        ------ Type of the item (see HID spec 6.2.2.2, then jump to 6.2.2.7)
    ^^^^
    --------- Function of the item (see HID spec 6.2.2.7, then HUT Sec 3)

```
在我们的例子中，长度1 字节，类型为 `Global`，函数为 `Usage Page`，因此要解析第二字节中的0x01，我们需要参HUT Sec 3
第二个数字是实际数据，其含义可在 HUT 中找到。我们有一`Usage Page`，因此我们需要参HUT Sec. 3，“Usage Pages”；从那里可以看`0x01` 代表 `Generic Desktop Page`
现在移动到后两个字节，并遵循相同的方案，`0x09`（即 `00001001`）后面跟一个字节（`01`），并且是一`Local` 项（`10`）。因此，剩余四位（`0000`）的含义HID 规范 Sec. 6.2.2.8 “Local Items中给出，于是我们得到一`Usage`。从 HUT Sec. 4，“Generic Desktop Page可知x02 代表 `Mouse`
后续的数字可以用相同的方式解析