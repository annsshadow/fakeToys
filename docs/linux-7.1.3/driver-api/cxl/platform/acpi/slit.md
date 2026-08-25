## SLIT - 系统局部性信息表（System Locality Information Table


系统局部性信息表提供访问者（accessor）与内存节点之间的“抽象距离”。没有发
者（initiator，即 cpu）的节点与其他所有节点的距离为无穷大（FF）

该表所描述的抽象距离并不代表任何真实的延迟或带宽信息

```

    Signature : "SLIT"    [System Locality Information Table]
   Localities : 0000000000000004
 Locality   0 : 10 20 20 30
 Locality   1 : 20 10 30 20
 Locality   2 : FF FF 0A FF
 Locality   3 : FF FF FF 0A

```
