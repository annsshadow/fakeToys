## NUMA 资源亲和

亲和性（associativity）表示将各种平台资源分组为一些域，这些域相对于该域之的资源具有实质上相近的平均性能。某个给定域中、彼此之间相比域外其它资源子表现出更好性能的资源子集，被表示为某个子分组域的成员。这一性能特征Linux
内核中以 NUMA 节点距离的形式呈现。从平台的角度看，这些组也被称为域
PAPR 接口目前支持以不同方式将这些资源分组细节传达给操作系统。它们被称为
Form 0、Form 1 Form2 关联分组。Form 0 是最旧的格式，现在已被认为过时
Hypervisor 通过 "ibm,architecture-vec-5 property" 指示所使用的关联类格式"ibm,architecture-vec-5" 属性中5 字节的第 0 位指示使Form 0 还是 Form 1值为 1 表示使用 Form 1 关联。对Form 2 关联，使"ibm,architecture-vec-5"
属性中5 字节的第 2 位
### Form 0

Form 0 关联仅支持两NUMA 距离（LOCAL REMOTE）
### Form 1

Form 1 通过组合 ibm,associativity-reference-points ibm,associativity 设备属性来确定资源域之间的 NUMA 距离
"ibm,associativity" 属性包含一个或多个数字（domainID）的列表，表示资源的平台
分组域
"ibm,associativity-reference-points" 属性包含一个或多个数字（domainID 索引）的
列表，表示关联列表中的从 1 开始的序数。domainID 索引列表表示资源分组不断升高层级
例如{ primary domainID index, secondary domainID index, tertiary domainID index.. }

Linux 内核使用domainID 索引处的 domainID 作为 NUMA 节点 id。Linux 内核通过
递归比较两个域是否属于相同的更高层级域来计算两个域之间的 NUMA 距离。对于资组中每高一层的不匹配，内核将比较域之间NUMA 距离加倍
### Form 2

Form 2 关联格式新增了独立的设备树属性来表示 NUMA 节点距离，从而使节点距离计算
更加灵活。Form 2 还允许灵活的主域编号。由NUMA 距离计算现在"ibm,associativity-reference-points" 属性中的索引值解耦，Form 2 允许在相domainID 索引处存在大量主 domainID，以表示具有不同性能/延迟特征的资源组
Hypervisor 使用 "ibm,architecture-vec-5" 属性中5 字节的第 2 位来指示使用
FORM2 关联
"ibm,numa-lookup-index-table" 属性包含一个或多个数字（表示系统中存在domainID）的列表。该属性中 domainID 的偏移被用作通过 "ibm,numa-distance-table"
计算 NUMA 距离信息时的索引
prop-encoded-array：以 encode-int 方式编码domainID 数量 N，后N 个以
encode-int 方式编码domainID
例如"ibm,numa-lookup-index-table" =  {4, 0, 8, 250, 252}。在计算 domain 8 与系统中
其它域的距离时，使用 domainID 8 的偏移（2）。在本文档的其余部分，该偏移将被称为
域距离偏移（domain distance offset）
"ibm,numa-distance-table" 属性包含一个或多个数字（表示系统中存在的资源组/域之间的
NUMA 距离）的列表
prop-encoded-array：以 encode-int 方式编码的距离值数N，后N 个以
encode-bytes 方式编码的距离值。我们能够编码的最大距离值为 255。N 必须等于 m 平方，其m numa-lookup-index-table domainID 的数量
例如ibm,numa-lookup-index-table = <3 0 8 40>;
ibm,numa-distace-table = <9>, /bits/ 8 < 10  20  80 20  10 160 80 160  10>;

```
	  | 0    8   40
	--|------------
	  |
	0 | 10   20  80
	  |
	8 | 20   10  160
	  |
	40| 80   160  10

```
节点 0 40 中资源可能的 "ibm,associativity" 属
{ 3, 6, 7, 0 }
{ 3, 6, 9, 8 }
{ 3, 6, 7, 40}

配合 "ibm,associativity-reference-points"  { 0x3 }

"ibm,lookup-index-table" 有助于以紧凑的方式表示距离矩阵。由domainID 可以稀疏的，距离矩阵也可以有效地是稀疏的。借助 "ibm,lookup-index-table"，我们可实现距离信息的紧凑表示