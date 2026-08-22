
## Chromebook 启动流程


大多数近期使用设备树（device tree）的 Chromebook 使用开源的 depthcharge_ 引导加载程序。depthcharge_ 期望操作系统被打包为一`FIT Image`_，其中包含操作系统镜像以及一组设备树。由 depthcharge_ `FIT Image`_ 中挑选出正确的设备树并提供给操作系统
depthcharge_ 用来挑选设备树的方案考虑了三个变量：

- 板名（Board name），depthcharge_ 编译时指定。即下面$(BOARD)- 板修订号（Board revision number），在运行时确定（可能通过读取 GPIO 硬件配置（strapping），也可能通过其他方法）。即下面$(REV)- SKU 号，在启动时GPIO 硬件配置中读取。即下面$(SKU)
对于近期Chromebook，depthcharge_ 创建的匹配列表如下：

- google,$(BOARD)-rev$(REV)-sku$(SKU)
- google,$(BOARD)-rev$(REV)
- google,$(BOARD)-sku$(SKU)
- google,$(BOARD)

注意，一些较旧的 Chromebook 使用略有不同、可能不包含 SKU 匹配或可能以不同优先级对SKU/rev 的列表
注意，对于某些板子，可能有额外的板级特定逻辑向列表中注入额外compatible 字符串，但这并不常见
depthcharge_ 会遍`FIT Image`_ 中的所有设备树，试图找到匹配最具体（most specific）compatible 的那一个。然后它会遍`FIT Image`_ 中的所有设备树，试图找到匹*第二具体** compatible 的那一个，依此类推
在搜索设备树时，depthcharge_ 并不关心 compatible 字符串在设备树根 compatible 字符串数组中的位置。例如，如果我们"lazor" 板、rev 4、SKU 0 上，并且有两棵设备树
- "google,lazor-rev5-sku0", "google,lazor-rev4-sku0", "qcom,sc7180"
- "google,lazor", "qcom,sc7180"

那么 depthcharge_ 会选择第一棵设备树，即"google,lazor-rev4-sku0" 是那棵设备树中列出的第二compatible。这是因为它"google,lazor" 更具体
需要注意的是，depthcharge_ 没有任何智能去尝试匹相近"的板子或 SKU 修订版本。也就是说，如果 depthcharge_ 知道自己在某块板"rev4" 上，但没"rev4" 的设备树，那depthcharge_ **不会**去寻"rev3" 的设备树
一般而言，当对一块板子做出任何重大改动时，即使其中没有任何改动需要在设备树中体现，板修订号也会增加。因此看到包含多个修订版本的设备树是相当常见的
应当注意，考虑depthcharge_ 上述的这套机制，如果支持某块板最新修订版本的设备树省略了 "-rev{REV}" compatible 字符串，就能获得最大的灵活性。这样做之后，如果你拿到一块新的板修订版本并试图在其上运行旧软件，那么我们至少能挑选到我们所知的最新的设备树