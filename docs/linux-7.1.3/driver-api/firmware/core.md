## 固件 API 核心特性

本文记录 Linux 固件加载子系统（firmware API）的核心特性，涵盖固件搜索路径、内置固件、缓存机制、直接文件系统查找、回退加载与查找顺序等，供驱动开发者理解内核加载固件的行为与配置方式。



固件 API 提供了一组丰富的核心特性。本节记录这些特性。

- [fw_search_path](fw_search_path)
- [built-in-fw](built-in-fw)
- [firmware_cache](firmware_cache)
- [direct-fs-lookup](direct-fs-lookup)
- [fallback-mechanisms](fallback-mechanisms)
- [lookup-order](lookup-order)
- [firmware-usage-guidelines](firmware-usage-guidelines)

