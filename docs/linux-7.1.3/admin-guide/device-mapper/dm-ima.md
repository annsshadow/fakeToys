## dm-ima


对于给定系统，各种外部服务/基础设施工具（包括证明服务）会与之交互——既在初始化设置期间，也在系统其余运行时段。它们共享敏感数据，和/或在该系统上执行关键工作负载。在将业务关键型数据/工作负载完全托付给该系统之前，外部服务可能希望先验证相关内核子系统的当前运行时状态。

设备映射器（Device mapper）通过在块设备上使用 crypt、verity、integrity 等多种目标类型，为块设备提供各种重要功能，从而在特定系统上发挥着关键作用。这些目标类型各自的功能都可以通过各种属性进行配置。用于配置这些目标类型的属性选择，会显著影响块设备乃至整个系统的安全配置。例如，加密算法的类型和密钥长度决定了给定块设备的加密强度。

因此，在将业务关键型数据/工作负载完全托付给该系统之前，外部服务验证各种块设备的当前状态及其各种目标属性至关重要。

IMA 内核子系统为设备映射器提供了必要的功能，用于度量（measure）各种块设备的状态与配置——

- 由设备映射器自身在内核内部完成，
- 以一种抗篡改（tamper resistant）的方式，
- 并在状态/配置发生变化时重新度量。

## 设置 IMA 策略：

为了让 IMA 度量给定系统上的数据，需要更新该系统上的 IMA 策略以包含如下一行内容，并且需要重启系统才能使度量生效。

```

 /etc/ima/ima-policy
    measure func=CRITICAL_DATA label=device-mapper template=ima-buf

```
度量结果会反映在 IMA 日志中，这些日志位于：

```

 /sys/kernel/security/integrity/ima/ascii_runtime_measurements
 /sys/kernel/security/integrity/ima/binary_runtime_measurements

```
然后 IMA ASCII 度量日志的格式如下：

```

 <PCR> <TEMPLATE_DATA_DIGEST> <TEMPLATE_NAME> <TEMPLATE_DATA>

 PCR := Platform Configuration Register，即平台配置寄存器，值会被登记在其中。
       这仅在使用了 TPM 芯片时适用。

 TEMPLATE_DATA_DIGEST := IMA 记录的模板数据摘要。
 TEMPLATE_NAME := 登记完整性值（integrity value）的模板名称（如 ima-buf）。

 TEMPLATE_DATA := <ALG> ":" <EVENT_DIGEST> <EVENT_NAME> <EVENT_DATA>
                  它包含要以给定模板数据格式进行度量的特定事件数据。

 ALG := 用于计算事件摘要的算法
 EVENT_DIGEST := 事件数据的摘要
 EVENT_NAME := 事件的描述（如 'dm_table_load'）。
 EVENT_DATA := 要被度量的事件数据。

```
|
| **NOTE #1:**
| IMA 子系统度量的 DM 目标数据，也可以通过设置 DM_IMA_MEASUREMENT_FLAG 与 DM_TABLE_STATUS_CMD，从用户空间查询。
|
|
| **NOTE #2:**
| 内核配置项 CONFIG_IMA_DISABLE_HTABLE 允许度量重复记录。
| 为支持在 IMA 日志中记录重复的 IMA 事件，内核需要配置 CONFIG_IMA_DISABLE_HTABLE=y。

## 支持的设备状态：

以下设备状态变更会触发 IMA 度量：

 1. Table load
 #. Device resume
 #. Device remove
 #. Table clear
 #. Device rename

### 1. 表加载（Table load）：

当一个新的表被加载到设备的非活跃表槽（inactive table slot）时，设备信息以及表中各目标的目标特定细节会被度量。

对于 'dm_table_load'，IMA 度量日志的格式如下：

```

 EVENT_NAME := "dm_table_load"
 EVENT_DATA := <dm_version_str> ";" <device_metadata> ";" <table_load_data>

 dm_version_str := "dm_version=" <N> "." <N> "." <N>
                  与设备映射器驱动版本相同。
 device_metadata := <device_name> "," <device_uuid> "," <device_major> "," <device_minor> ","
                   <minor_count> "," <num_device_targets> ";"

 device_name := "name=" <dm-device-name>
 device_uuid := "uuid=" <dm-device-uuid>
 device_major := "major=" <N>
 device_minor := "minor=" <N>
 minor_count := "minor_count=" <N>
 num_device_targets := "num_targets=" <N>
 dm-device-name := 设备名称。如果其中包含 '\'、','、';' 等特殊字符，
                   会在其前面加上 '\' 前缀。
 dm-device-uuid := 设备的 UUID。如果其中包含 '\'、','、';' 等特殊字符，
                   会在其前面加上 '\' 前缀。

 table_load_data := <target_data>
                    表示正在加载到 DM 设备非活跃表槽中的、来自表中各目标的数据
                    （以 name=value 对的形式）。
 target_data := <target_data_row> | <target_data><target_data_row>

 target_data_row := <target_index> "," <target_begin> "," <target_len> "," <target_name> ","
                    <target_version> "," <target_attributes> ";"
 target_index := "target_index=" <N>
                 表示表中的第 n 个目标（范围从 <num_device_targets> 指定的 0 到 N-1）。
                 如果 N 个目标的数据无法全部放入给定缓冲区，则能够放入缓冲区的那部分数据
                 （例如从目标 0 到 x）会在一次给定的 IMA 事件中被度量。
                 其余从目标 x+1 到 N-1 的数据会在后续的 IMA 事件中度量，
                 其格式与 'dm_table_load' 相同，
                 即 <dm_version_str> ";" <device_metadata> ";" <table_load_data>。

 target_begin := "target_begin=" <N>
 target_len := "target_len=" <N>
 target_name := 目标的名称。'linear'、'crypt'、'integrity' 等。
                支持 IMA 度量的目标在下面的“支持的目标”小节中有文档说明。
 target_version := "target_version=" <N> "." <N> "." <N>
 target_attributes := 包含以逗号分隔的、目标特定属性 name=value 对的数据。

 例如，如果使用以下表项创建一个 linear 设备，
  # dmsetup create linear1
  0 2 linear /dev/loop0 512
  2 2 linear /dev/loop0 512
  4 2 linear /dev/loop0 512
  6 2 linear /dev/loop0 512

 那么 IMA ASCII 度量日志中将包含如下条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 a8c5ff755561c7a28146389d1514c318592af49a ima-buf sha256:4d73481ecce5eadba8ab084640d85bb9ca899af4d0a122989252a76efadc5b72
 dm_table_load
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=0,minor_count=1,num_targets=4;
 target_index=0,target_begin=0,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=1,target_begin=2,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=2,target_begin=4,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;
 target_index=3,target_begin=6,target_len=2,target_name=linear,target_version=1.4.0,device_name=7:0,start=512;

```
### 2. 设备恢复（Device resume）：

当被挂起的设备恢复（resume）时，设备信息以及上次加载的活跃表（active table）数据的哈希会被度量。

对于 'dm_device_resume'，IMA 度量日志的格式如下：

```

 EVENT_NAME := "dm_device_resume"
 EVENT_DATA := <dm_version_str> ";" <device_metadata> ";" <active_table_hash> ";" <current_device_capacity> ";"

 dm_version_str := 如上面“表加载”小节所述。
 device_metadata := 如上面“表加载”小节所述。
 active_table_hash := "active_table_hash=" <table_hash_alg> ":" <table_hash>
                      表示正在度量的、该设备活跃表的 IMA 数据的哈希。
 table_hash_alg := 用于计算哈希的算法。
 table_hash := 对 (<dm_version_str> ";" <device_metadata> ";" <table_load_data> ";")
               的哈希，如上面的 'dm_table_load' 所述。
               注意：如果 table_load 数据跨越了某个设备的多个 IMA 'dm_table_load'
               事件，则哈希是结合所有这些事件的数据
               即 (<dm_version_str> ";" <device_metadata> ";" <table_load_data> ";")
               计算得到的。
 current_device_capacity := "current_device_capacity=" <N>

 例如，如果使用以下命令恢复一个 linear 设备，
 #dmsetup resume linear1

 那么 IMA ASCII 度量日志中将包含一个条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 56c00cc062ffc24ccd9ac2d67d194af3282b934e ima-buf sha256:e7d12c03b958b4e0e53e7363a06376be88d98a1ac191fdbd3baf5e4b77f329b6
 dm_device_resume
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=0,minor_count=1,num_targets=4;
 active_table_hash=sha256:4d73481ecce5eadba8ab084640d85bb9ca899af4d0a122989252a76efadc5b72;current_device_capacity=8;

```
### 3. 设备移除（Device remove）：

当设备被移除时，设备信息以及活跃表与非活跃表（inactive table）数据的 sha256 哈希会被度量。

对于 'dm_device_remove'，IMA 度量日志的格式如下：

```

 EVENT_NAME := "dm_device_remove"
 EVENT_DATA := <dm_version_str> ";" <device_active_metadata> ";" <device_inactive_metadata> ";"
               <active_table_hash> "," <inactive_table_hash> "," <remove_all> ";" <current_device_capacity> ";"

 dm_version_str := 如上面“表加载”小节所述。
 device_active_metadata := 反映当前已加载活跃表的设备元数据。
                           其格式与上面“表加载”小节中描述的 'device_metadata' 相同。
 device_inactive_metadata := 反映非活跃表的设备元数据。
                             其格式与上面“表加载”小节中描述的 'device_metadata' 相同。
 active_table_hash := 当前已加载活跃表的哈希。
                      其格式与上面“设备恢复”小节中描述的 'active_table_hash' 相同。
 inactive_table_hash :=  非活跃表的哈希。
                         其格式与上面“设备恢复”小节中描述的 'active_table_hash' 相同。
 remove_all := "remove_all=" <yes_no>
 yes_no := "y" | "n"
 current_device_capacity := "current_device_capacity=" <N>

 例如，如果使用以下命令移除一个 linear 设备，
  #dmsetup remove l1

 那么 IMA ASCII 度量日志中将包含如下条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 790e830a3a7a31590824ac0642b3b31c2d0e8b38 ima-buf sha256:ab9f3c959367a8f5d4403d6ce9c3627dadfa8f9f0e7ec7899299782388de3840
 dm_device_remove
 dm_version=4.45.0;
 device_active_metadata=name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=2;
 device_inactive_metadata=name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 active_table_hash=sha256:4a7e62efaebfc86af755831998b7db6f59b60d23c9534fb16a4455907957953a,
 inactive_table_hash=sha256:9d79c175bc2302d55a183e8f50ad4bafd60f7692fd6249e5fd213e2464384b86,remove_all=n;
 current_device_capacity=2048;

```
### 4. 表清除（Table clear）：

当非活跃表从设备中清除时，设备信息以及该非活跃表数据的 sha256 哈希会被度量。

对于 'dm_table_clear'，IMA 度量日志的格式如下：

```

 EVENT_NAME := "dm_table_clear"
 EVENT_DATA := <dm_version_str> ";" <device_inactive_metadata> ";" <inactive_table_hash> ";" <current_device_capacity> ";"

 dm_version_str := 如上面“表加载”小节所述。
 device_inactive_metadata := 在加载时捕获的、正被清除的非活跃表的设备元数据。
                             其格式与上面“表加载”小节中描述的 'device_metadata' 相同。
 inactive_table_hash := 正被从设备清除的非活跃表的哈希。
                        其格式与上面“设备恢复”小节中描述的 'active_table_hash' 相同。
 current_device_capacity := "current_device_capacity=" <N>

 例如，如果一个 linear 设备的非活跃表被清除，
  #dmsetup clear l1

 那么 IMA ASCII 度量日志中将包含一个条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 77d347408f557f68f0041acb0072946bb2367fe5 ima-buf sha256:42f9ca22163fdfa548e6229dece2959bc5ce295c681644240035827ada0e1db5
 dm_table_clear
 dm_version=4.45.0;
 name=l1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 inactive_table_hash=sha256:75c0dc347063bf474d28a9907037eba060bfe39d8847fc0646d75e149045d545;current_device_capacity=1024;

```
### 5. 设备重命名（Device rename）：

当设备的 NAME 或 UUID 被更改时，设备信息以及新的 NAME 和 UUID 会被度量。

对于 'dm_device_rename'，IMA 度量日志的格式如下：

```

 EVENT_NAME := "dm_device_rename"
 EVENT_DATA := <dm_version_str> ";" <device_active_metadata> ";" <new_device_name> "," <new_device_uuid> ";" <current_device_capacity> ";"

 dm_version_str := 如上面“表加载”小节所述。
 device_active_metadata := 反映当前已加载活跃表的设备元数据。
                           其格式与上面“表加载”小节中描述的 'device_metadata' 相同。
 new_device_name := "new_name=" <dm-device-name>
 dm-device-name := 与上面“表加载”小节中描述的 <dm-device-name> 相同
 new_device_uuid := "new_uuid=" <dm-device-uuid>
 dm-device-uuid := 与上面“表加载”小节中描述的 <dm-device-uuid> 相同
 current_device_capacity := "current_device_capacity=" <N>

 例 1：如果使用以下命令更改一个 linear 设备的名称，
  #dmsetup rename linear1 --setuuid 1234-5678

 那么 IMA ASCII 度量日志中将包含一个条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 8b0423209b4c66ac1523f4c9848c9b51ee332f48 ima-buf sha256:6847b7258134189531db593e9230b257c84f04038b5a18fd2e1473860e0569ac
 dm_device_rename
 dm_version=4.45.0;
 name=linear1,uuid=,major=253,minor=2,minor_count=1,num_targets=1;new_name=linear1,new_uuid=1234-5678;
 current_device_capacity=1024;

 例 2：如果使用以下命令更改一个 linear 设备的名称，
  # dmsetup rename linear1 linear=2

 那么 IMA ASCII 度量日志中将包含一个条目：
 （为便于阅读，已从 ASCII 转换为文本）

 10 bef70476b99c2bdf7136fae033aa8627da1bf76f ima-buf sha256:8c6f9f53b9ef9dc8f92a2f2cca8910e622543d0f0d37d484870cb16b95111402
 dm_device_rename
 dm_version=4.45.0;
 name=linear1,uuid=1234-5678,major=253,minor=2,minor_count=1,num_targets=1;
 new_name=linear\=2,new_uuid=1234-5678;
 current_device_capacity=1024;

```
## 支持的目标（targets）：


以下目标（targets）支持使用 IMA 度量它们的数据：

 1. cache
 #. crypt
 #. integrity
 #. linear
 #. mirror
 #. multipath
 #. raid
 #. snapshot
 #. striped
 #. verity

### 1. cache

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'cache' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <metadata_mode> "," <cache_metadata_device> ","
                      <cache_device> "," <cache_origin_device> "," <writethrough> "," <writeback> ","
                      <passthrough> "," <no_discard_passdown> ";"

 target_name := "target_name=cache"
 target_version := "target_version=" <N> "." <N> "." <N>
 metadata_mode := "metadata_mode=" <cache_metadata_mode>
 cache_metadata_mode := "fail" | "ro" | "rw"
 cache_device := "cache_device=" <cache_device_name_string>
 cache_origin_device := "cache_origin_device=" <cache_origin_device_string>
 writethrough := "writethrough=" <yes_no>
 writeback := "writeback=" <yes_no>
 passthrough := "passthrough=" <yes_no>
 no_discard_passdown := "no_discard_passdown=" <yes_no>
 yes_no := "y" | "n"

 例如：
 当加载一个 'cache' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'cache' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;name=cache1,uuid=cache_uuid,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=28672,target_name=cache,target_version=2.2.0,metadata_mode=rw,
 cache_metadata_device=253:4,cache_device=253:3,cache_origin_device=253:5,writethrough=y,writeback=n,
 passthrough=n,metadata2=y,no_discard_passdown=n;


```
### 2. crypt

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'crypt' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <allow_discards> "," <same_cpu_crypt> ","
                      <submit_from_crypt_cpus> "," <no_read_workqueue> "," <no_write_workqueue> ","
                      <iv_large_sectors> "," <iv_large_sectors> "," [<integrity_tag_size> ","] [<cipher_auth> ","]
                      [<sector_size> ","] [<cipher_string> ","] <key_size> "," <key_parts> ","
                      <key_extra_size> "," <key_mac_size> ";"

 target_name := "target_name=crypt"
 target_version := "target_version=" <N> "." <N> "." <N>
 allow_discards := "allow_discards=" <yes_no>
 same_cpu_crypt := "same_cpu_crypt=" <yes_no>
 submit_from_crypt_cpus := "submit_from_crypt_cpus=" <yes_no>
 no_read_workqueue := "no_read_workqueue=" <yes_no>
 no_write_workqueue := "no_write_workqueue=" <yes_no>
 iv_large_sectors := "iv_large_sectors=" <yes_no>
 integrity_tag_size := "integrity_tag_size=" <N>
 cipher_auth := "cipher_auth=" <string>
 sector_size := "sector_size="  <N>
 cipher_string := "cipher_string="
 key_size := "key_size="  <N>
 key_parts := "key_parts="  <N>
 key_extra_size := "key_extra_size="  <N>
 key_mac_size := "key_mac_size="  <N>
 yes_no := "y" | "n"

 例如：
 当加载一个 'crypt' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'crypt' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=crypt1,uuid=crypt_uuid1,major=253,minor=0,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=1953125,target_name=crypt,target_version=1.23.0,
 allow_discards=y,same_cpu=n,submit_from_crypt_cpus=n,no_read_workqueue=n,no_write_workqueue=n,
 iv_large_sectors=n,cipher_string=aes-xts-plain64,key_size=32,key_parts=1,key_extra_size=0,key_mac_size=0;

```
### 3. integrity

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'integrity' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <dev_name> "," <start>
                      <tag_size> "," <mode> "," [<meta_device> ","] [<block_size> ","] <recalculate> ","
                      <allow_discards> "," <fix_padding> "," <fix_hmac> "," <legacy_recalculate> ","
                      <journal_sectors> "," <interleave_sectors> "," <buffer_sectors> ";"

 target_name := "target_name=integrity"
 target_version := "target_version=" <N> "." <N> "." <N>
 dev_name := "dev_name=" <device_name_str>
 start := "start=" <N>
 tag_size := "tag_size=" <N>
 mode := "mode=" <integrity_mode_str>
 integrity_mode_str := "J" | "B" | "D" | "R"
 meta_device := "meta_device=" <meta_device_str>
 block_size := "block_size=" <N>
 recalculate := "recalculate=" <yes_no>
 allow_discards := "allow_discards=" <yes_no>
 fix_padding := "fix_padding=" <yes_no>
 fix_hmac := "fix_hmac=" <yes_no>
 legacy_recalculate := "legacy_recalculate=" <yes_no>
 journal_sectors := "journal_sectors=" <N>
 interleave_sectors := "interleave_sectors=" <N>
 buffer_sectors := "buffer_sectors=" <N>
 yes_no := "y" | "n"

 例如：
 当加载一个 'integrity' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'integrity' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=integrity1,uuid=,major=253,minor=1,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=7856,target_name=integrity,target_version=1.10.0,
 dev_name=253:0,start=0,tag_size=32,mode=J,recalculate=n,allow_discards=n,fix_padding=n,
 fix_hmac=n,legacy_recalculate=n,journal_sectors=88,interleave_sectors=32768,buffer_sectors=128;


```
### 4. linear

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'linear' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <device_name> <,> <start> ";"

 target_name := "target_name=linear"
 target_version := "target_version=" <N> "." <N> "." <N>
 device_name := "device_name=" <linear_device_name_str>
 start := "start=" <N>

 例如：
 当加载一个 'linear' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'linear' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=linear1,uuid=linear_uuid1,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=28672,target_name=linear,target_version=1.4.0,
 device_name=253:1,start=2048;

```
### 5. mirror

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'mirror' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <nr_mirrors> ","
                      <mirror_device_data> "," <handle_errors> "," <keep_log> "," <log_type_status> ";"

 target_name := "target_name=mirror"
 target_version := "target_version=" <N> "." <N> "." <N>
 nr_mirrors := "nr_mirrors=" <NR>
 mirror_device_data := <mirror_device_row> | <mirror_device_data><mirror_device_row>
                       mirror_device_row 会重复 <NR> 次——对应 <nr_mirrors> 中描述的 <NR>。
 mirror_device_row := <mirror_device_name> "," <mirror_device_status>
 mirror_device_name := "mirror_device_" <X> "=" <mirror_device_name_str>
                       其中 <X> 的范围从 0 到 (<NR> -1)——对应 <nr_mirrors> 中描述的 <NR>。
 mirror_device_status := "mirror_device_" <X> "_status=" <mirror_device_status_char>
                         其中 <X> 的范围从 0 到 (<NR> -1)——对应 <nr_mirrors> 中描述的 <NR>。
 mirror_device_status_char := "A" | "F" | "D" | "S" | "R" | "U"
 handle_errors := "handle_errors=" <yes_no>
 keep_log := "keep_log=" <yes_no>
 log_type_status := "log_type_status=" <log_type_status_str>
 yes_no := "y" | "n"

 例如：
 当加载一个 'mirror' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'mirror' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=mirror1,uuid=mirror_uuid1,major=253,minor=6,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2048,target_name=mirror,target_version=1.14.0,nr_mirrors=2,
    mirror_device_0=253:4,mirror_device_0_status=A,
    mirror_device_1=253:5,mirror_device_1_status=A,
 handle_errors=y,keep_log=n,log_type_status=;

```
### 6. multipath

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'multipath' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <nr_priority_groups>
                      ["," <pg_state> "," <priority_groups> "," <priority_group_paths>] ";"

 target_name := "target_name=multipath"
 target_version := "target_version=" <N> "." <N> "." <N>
 nr_priority_groups := "nr_priority_groups=" <NPG>
 priority_groups := <priority_groups_row>|<priority_groups_row><priority_groups>
 priority_groups_row := "pg_state_" <X> "=" <pg_state_str> "," "nr_pgpaths_" <X>  "=" <NPGP> ","
                        "path_selector_name_" <X> "=" <string> "," <priority_group_paths>
                        其中 <X> 的范围从 0 到 (<NPG> -1)——对应 <nr_priority_groups> 中描述的 <NPG>。
 pg_state_str := "E" | "A" | "D"
 <priority_group_paths> := <priority_group_paths_row> | <priority_group_paths_row><priority_group_paths>
 priority_group_paths_row := "path_name_" <X> "_" <Y> "=" <string> "," "is_active_" <X> "_" <Y> "=" <is_active_str>
                             "fail_count_" <X> "_" <Y> "=" <N> "," "path_selector_status_" <X> "_" <Y> "=" <path_selector_status_str>
                             其中 <X> 的范围从 0 到 (<NPG> -1)——对应 <nr_priority_groups> 中描述的 <NPG>，
                             而 <Y> 的范围从 0 到 (<NPGP> -1)——对应 <priority_groups_row> 中描述的 <NPGP>。
 is_active_str := "A" | "F"

 例如：
 当加载一个 'multipath' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'multipath' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=mp,uuid=,major=253,minor=0,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2097152,target_name=multipath,target_version=1.14.0,nr_priority_groups=2,
    pg_state_0=E,nr_pgpaths_0=2,path_selector_name_0=queue-length,
        path_name_0_0=8:16,is_active_0_0=A,fail_count_0_0=0,path_selector_status_0_0=,
        path_name_0_1=8:32,is_active_0_1=A,fail_count_0_1=0,path_selector_status_0_1=,
    pg_state_1=E,nr_pgpaths_1=2,path_selector_name_1=queue-length,
        path_name_1_0=8:48,is_active_1_0=A,fail_count_1_0=0,path_selector_status_1_0=,
        path_name_1_1=8:64,is_active_1_1=A,fail_count_1_1=0,path_selector_status_1_1=;

```
### 7. raid

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'raid' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <raid_type> "," <raid_disks> "," <raid_state>
                      <raid_device_status> ["," journal_dev_mode] ";"

 target_name := "target_name=raid"
 target_version := "target_version=" <N> "." <N> "." <N>
 raid_type := "raid_type=" <raid_type_str>
 raid_disks := "raid_disks=" <NRD>
 raid_state := "raid_state=" <raid_state_str>
 raid_state_str := "frozen" | "reshape" |"resync" | "check" | "repair" | "recover" | "idle" |"undef"
 raid_device_status := <raid_device_status_row> | <raid_device_status_row><raid_device_status>
                       <raid_device_status_row> 会重复 <NRD> 次——对应 <raid_disks> 中描述的 <NRD>。
 raid_device_status_row := "raid_device_" <X> "_status=" <raid_device_status_str>
                           其中 <X> 的范围从 0 到 (<NRD> -1)——对应 <raid_disks> 中描述的 <NRD>。
 raid_device_status_str := "A" | "D" | "a" | "-"
 journal_dev_mode := "journal_dev_mode=" <journal_dev_mode_str>
 journal_dev_mode_str := "writethrough" | "writeback" | "invalid"

 例如：
 当加载一个 'raid' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'raid' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=raid_LV1,uuid=uuid_raid_LV1,major=253,minor=12,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=2048,target_name=raid,target_version=1.15.1,
 raid_type=raid10,raid_disks=4,raid_state=idle,
    raid_device_0_status=A,
    raid_device_1_status=A,
    raid_device_2_status=A,
    raid_device_3_status=A;


```
### 8. snapshot

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'snapshot' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <snap_origin_name> ","
                      <snap_cow_name> "," <snap_valid> "," <snap_merge_failed> "," <snapshot_overflowed> ";"

 target_name := "target_name=snapshot"
 target_version := "target_version=" <N> "." <N> "." <N>
 snap_origin_name := "snap_origin_name=" <string>
 snap_cow_name := "snap_cow_name=" <string>
 snap_valid := "snap_valid=" <yes_no>
 snap_merge_failed := "snap_merge_failed=" <yes_no>
 snapshot_overflowed := "snapshot_overflowed=" <yes_no>
 yes_no := "y" | "n"

 例如：
 当加载一个 'snapshot' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'snapshot' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=snap1,uuid=snap_uuid1,major=253,minor=13,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=4096,target_name=snapshot,target_version=1.16.0,
 snap_origin_name=253:11,snap_cow_name=253:12,snap_valid=y,snap_merge_failed=n,snapshot_overflowed=n;

```
### 9. striped

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'striped' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <stripes> "," <chunk_size> ","
                      <stripe_data> ";"

 target_name := "target_name=striped"
 target_version := "target_version=" <N> "." <N> "." <N>
 stripes := "stripes=" <NS>
 chunk_size := "chunk_size=" <N>
 stripe_data := <stripe_data_row>|<stripe_data><stripe_data_row>
 stripe_data_row := <stripe_device_name> "," <stripe_physical_start> "," <stripe_status>
 stripe_device_name := "stripe_" <X> "_device_name=" <stripe_device_name_str>
                       其中 <X> 的范围从 0 到 (<NS> -1)——对应 <stripes> 中描述的 <NS>。
 stripe_physical_start := "stripe_" <X> "_physical_start=" <N>
                          其中 <X> 的范围从 0 到 (<NS> -1)——对应 <stripes> 中描述的 <NS>。
 stripe_status := "stripe_" <X> "_status=" <stripe_status_str>
                  其中 <X> 的范围从 0 到 (<NS> -1)——对应 <stripes> 中描述的 <NS>。
 stripe_status_str := "D" | "A"

 例如：
 当加载一个 'striped' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'striped' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=striped1,uuid=striped_uuid1,major=253,minor=5,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=640,target_name=striped,target_version=1.6.0,stripes=2,chunk_size=64,
    stripe_0_device_name=253:0,stripe_0_physical_start=2048,stripe_0_status=A,
    stripe_1_device_name=253:3,stripe_1_physical_start=2048,stripe_1_status=A;

```
### 10. verity

作为 EVENT_DATA 一部分在上述“表加载”小节中描述的 'target_attributes'，对于 'verity' 目标具有以下数据格式。

```

 target_attributes := <target_name> "," <target_version> "," <hash_failed> "," <verity_version> ","
                      <data_device_name> "," <hash_device_name> "," <verity_algorithm> "," <root_digest> ","
                      <salt> "," <ignore_zero_blocks> "," <check_at_most_once> ["," <root_hash_sig_key_desc>]
                      ["," <verity_mode>] ";"

 target_name := "target_name=verity"
 target_version := "target_version=" <N> "." <N> "." <N>
 hash_failed := "hash_failed=" <hash_failed_str>
 hash_failed_str := "C" | "V"
 verity_version := "verity_version=" <verity_version_str>
 data_device_name := "data_device_name=" <data_device_name_str>
 hash_device_name := "hash_device_name=" <hash_device_name_str>
 verity_algorithm := "verity_algorithm=" <verity_algorithm_str>
 root_digest := "root_digest=" <root_digest_str>
 salt := "salt=" <salt_str>
 salt_str := "-" <verity_salt_str>
 ignore_zero_blocks := "ignore_zero_blocks=" <yes_no>
 check_at_most_once := "check_at_most_once=" <yes_no>
 root_hash_sig_key_desc := "root_hash_sig_key_desc="
 verity_mode := "verity_mode=" <verity_mode_str>
 verity_mode_str := "ignore_corruption" | "restart_on_corruption" | "panic_on_corruption" | "invalid"
 yes_no := "y" | "n"

 例如：
 当加载一个 'verity' 目标时，IMA ASCII 度量日志会有一条类似于下面的条目，
 展示在 'dm_table_load' 事件的 EVENT_DATA 中度量了哪些 'verity' 属性。
 （为便于阅读，已从 ASCII 转换为文本）

 dm_version=4.45.0;
 name=test-verity,uuid=,major=253,minor=2,minor_count=1,num_targets=1;
 target_index=0,target_begin=0,target_len=1953120,target_name=verity,target_version=1.8.0,hash_failed=V,
 verity_version=1,data_device_name=253:1,hash_device_name=253:0,verity_algorithm=sha256,
 root_digest=29cb87e60ce7b12b443ba6008266f3e41e93e403d7f298f8e3f316b29ff89c5e,
 salt=e48da609055204e89ae53b655ca2216dd983cf3cb829f34f63a297d106d53e2d,
 ignore_zero_blocks=n,check_at_most_once=n;

```
