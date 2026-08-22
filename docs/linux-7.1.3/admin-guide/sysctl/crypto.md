## /proc/sys/crypto/


这些文件是否出现`/proc/sys/crypto/`，取决于内核配置

## fips_enabled


只读标志，指示是否启用了 FIPS 模式
- `0`：禁FIPS 模式（默认）- `1`：启FIPS 模式
该值在启动时通过 `fips=1` 内核命令行参数设置。启用后，加API 将限制某算法的使用并进行自检，以确保符合 FIPS（联邦信息处理标准）要求，例FIPS 140-2 与较新的 FIPS 140-3，具体取决于内核配置与所用模块
## fips_name


只读文件，包含当前所FIPS 模块的名称该值通常通过 `CONFIG_CRYPTO_FIPS_NAME` 内核配置选项配置
## fips_version


只读文件，包FIPS 模块的版本字符串如果设置`CONFIG_CRYPTO_FIPS_CUSTOM_VERSION`，则使用 `CONFIG_CRYPTO_FIPS_VERSION`
的值。否则默认为内核发布版本（`UTS_RELEASE`）
Copyright (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

有关一般信息与法律声明，请参阅
Documentation/admin-guide/sysctl/index.rst銆。