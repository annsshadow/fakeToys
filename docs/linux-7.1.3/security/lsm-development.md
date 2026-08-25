## Linux 安全模块开

基于 https://lore.kernel.org/r/20071026073721.618b4778@laptopd505.fenrus.org当一个新LSM 的意图（即它试图防范什么、以及在什么情况下人们会期望使用它）已`Documentation/admin-guide/LSM/` 中得到恰当文档化时，LSM 才会被内核接受这样便于LSM 的代码与其目标进行比较，也使最终用户和发行版能够就更贴合其需求的 LSM 做出更明智的决策
关于可用 LSM 钩子接口的详尽文档，请参`security/security.c` 及相关的结构体：

   :export:
