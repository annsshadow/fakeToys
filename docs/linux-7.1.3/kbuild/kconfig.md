## Configuration targets and editors


本文档提供一些使`make *config` 的帮助

使用 `make help` 列出所有可能的配置目标

xconfigqconf'）、menuconfigmconf'）和 nconfignconf'）程序也内嵌了帮助文本
请务必查看这些关于导航、搜索以及其他通用帮助的文本内容

gconfiggconf'）程序的帮助文本有限


## General


新的内核版本通常会引入新的配置符号。往往更重要的是，新的内核版本可能会重命名配置符号
发生这种情况时，使用之前可用.config 文件并运行“make oldconfig”不一定能为你生成一个可用的
新内核，因此你可能会发现需要查看引入了哪些新的内核符号

```

    cp user/some/old.config .config
    make listnewconfig

```
配置程序会逐行列出所有新的符号

```

    make oldconfig
    scripts/diffconfig .config.old .config | less


```
## Environment variables


`*config` 的环境变量：

`KCONFIG_CONFIG`
    该环境变量可用于指定一个默认的内核配置文件名，以覆盖默认的config”名称

`KCONFIG_DEFCONFIG_LIST`
    该环境变量指定一个配置文件列表，.config 尚不存在时可用作基础配置。列表中的条目以空白
    字符相互分隔，使用第一个存在的条目

`KCONFIG_OVERWRITECONFIG`
    如果你在环境中设置了 KCONFIG_OVERWRITECONFIG，当 .config 是指向其他位置的软链接时，Kconfig
    不会断开该软链接

`KCONFIG_WARN_UNKNOWN_SYMBOLS`
    该环境变量使 Kconfig 对配置输入中所有无法识别的符号发出警告

`KCONFIG_WERROR`
    如果设置，Kconfig 将警告视为错误

`CONFIG_`
    如果你在环境中设置了 `CONFIG_`，Kconfig 在保存配置时将为所有符号加上该值作为前缀，而不
    使用默认`CONFIG_`

`{allyes/allmod/allno/alldef/rand}config` 的环境变量：

`KCONFIG_ALLCONFIG`
    allyesconfig/allmodconfig/alldefconfig/allnoconfig/randconfig 变体也可以使用环境变
    KCONFIG_ALLCONFIG 作为标志或一个包含用户要求设为特定值的配置符号的文件名。如
    KCONFIG_ALLCONFIG 在没有文件名的情况下使用（即 KCONFIG_ALLCONFIG == "" 
    KCONFIG_ALLCONFIG == "1"），`make *config` 会查找名为“all{yes/mod/no/def/random}.config
    （对应于所使用`*config` 命令）的文件，以获取要强制设置的符号值。如果找不到该文件，
    查找名为“all.config”的文件以获取要强制设置的值

    这使你能够创建只包含你感兴趣配置符号的“迷你”配置（miniconfig）或自定义配置文件。然后内
    配置系统会生成完整的 .config 文件，包括你 miniconfig 文件中的符号

    `KCONFIG_ALLCONFIG` 文件是一个包含（通常是全部符号的）预设配置符号的配置文件。这些变
    设置仍需接受常规的依赖检查

```

        KCONFIG_ALLCONFIG=custom-notebook.config make allnoconfig

    or::

        KCONFIG_ALLCONFIG=mini.config make allnoconfig

    or::

        make KCONFIG_ALLCONFIG=mini.config allnoconfig

    These examples will disable most options (allnoconfig) but enable or
    disable the options that are explicitly listed in the specified
    mini-config files.

```
`randconfig` 的环境变量：

`KCONFIG_SEED`
    如果你出于某种原因要调试 kconfig 解析前端的行为，可以将此项设为用于给 RNG 播种的整数值
    如果未设置，将使用当前时间

`KCONFIG_PROBABILITY`
    该变量可用于偏斜概率。该变量可以未设置或为空，或设为三种不同的格式：

    =======================     ==================  =====================
    KCONFIG_PROBABILITY         y:n 鎷嗗垎甯?         y:m:n 鎷嗗垎甯。
    =======================     ==================  =====================
    unset or empty              50  : 50            33  : 33  : 34
    N                            N  : 100-N         N/2 : N/2 : 100-N
    [^1^] N:M                     N+M : 100-(N+M)      N  :  M  : 100-(N+M)
    [^2^] N:M:L                    N  : 100-N          M  :  L  : 100-(M+L)
    =======================     ==================  =====================

其中 N、M L 是范[0,100] 内的整数（十进制），并且满足

    [^1^] N+M 在范[0,100] 

    [^2^] M+L 在范[0,100] 

```

    KCONFIG_PROBABILITY=10
        10% of booleans will be set to 'y', 90% to 'n'
        5% of tristates will be set to 'y', 5% to 'm', 90% to 'n'
    KCONFIG_PROBABILITY=15:25
        40% of booleans will be set to 'y', 60% to 'n'
        15% of tristates will be set to 'y', 25% to 'm', 60% to 'n'
    KCONFIG_PROBABILITY=10:15:15
        10% of booleans will be set to 'y', 90% to 'n'
        15% of tristates will be set to 'y', 15% to 'm', 70% to 'n'

```
`syncconfig` 的环境变量：

`KCONFIG_NOSILENTUPDATE`
    如果该变量具有非空值，它将阻止静默的内核配置更新（需要显式更新）

`KCONFIG_AUTOCONFIG`
    该环境变量可设置以指定“auto.conf”文件的路径和名称。其默认值为
    鈥渋nclude/config/auto.conf鈥濄€?

`KCONFIG_AUTOHEADER`
    该环境变量可设置以指定“autoconf.h”（头文件）文件的路径和名称。其默认值为
    鈥渋nclude/generated/autoconf.h鈥濄€?


## menuconfig


menuconfig 中搜索：

    搜索功能搜索内核配置符号名，因此你必须知道接近你要查找内容的名称

```

        /hotplug
        This lists all config symbols that contain "hotplug",
        e.g., HOTPLUG_CPU, MEMORY_HOTPLUG.

    For search help, enter / followed by TAB-TAB (to highlight
    <Help>) and Enter.  This will tell you that you can also use
    regular expressions (regexes) in the search string, so if you
    are not interested in MEMORY_HOTPLUG, you could try::

        /^hotplug

    When searching, symbols are sorted thus:

    - first, exact matches, sorted alphabetically (an exact match
      is when the search matches the complete symbol name);
    - then, other matches, sorted alphabetically.

    For example, ^ATH.K matches:

        ATH5K ATH9K ATH5K_AHB ATH5K_DEBUG [...] ATH6KL ATH6KL_DEBUG
        [...] ATH9K_AHB ATH9K_BTCOEX_SUPPORT ATH9K_COMMON [...]

    of which only ATH5K and ATH9K match exactly and so are sorted
    first (and in alphabetical order), then come all other symbols,
    sorted in alphabetical order.

    In this menu, pressing the key in the (#) prefix will jump
    directly to that location. You will be returned to the current
    search results after exiting this new menu.

```
'menuconfig' 的用户界面选项

`MENUCONFIG_COLOR`
    可以使用该变量选择不同的配色主
```

        make MENUCONFIG_COLOR=<theme> menuconfig

    Available themes are::

      - mono       => selects colors suitable for monochrome displays
      - blackbg    => selects a color scheme with black background
      - classic    => theme with blue background. The classic look
      - bluetitle  => a LCD friendly version of classic. (default)

```
`MENUCONFIG_MODE`
    该模式将所有子菜单显示在一个大树中

```

        make MENUCONFIG_MODE=single_menu menuconfig


```
## nconfig


nconfig 是一个替代的、基于文本的配置器。它在终端（窗口）底部列出执行命令的功能键。除非你处于
数据输入窗口中，否则也可以直接使用相应的数字键来执行命令。例如，可以6 代替 F6 来保存

使用 F1 获取全局帮助，或 F3 获取简短帮助菜单

nconfig 中搜索：

    你可以在菜单项“prompt”字符串中，或在配置符号中搜索

    使用 / 开始在菜单项中搜索。这不支持正则表达式。使<Down> <Up> 分别用于下一个和上一
    匹配项。使<Esc> 终止搜索模式

    F8（SymSearch）在配置符号中搜索给定字符串或正则表达式（regex）

    SymSearch 中，按下 (#) 前缀中的键将直接跳转到该位置。退出这个新菜单后，你将返回到当
    的搜索结果

环境变量

`NCONFIG_MODE`
    该模式将所有子菜单显示在一个大树中

```

        make NCONFIG_MODE=single_menu nconfig


```
## xconfig


xconfig 中搜索：

    搜索功能搜索内核配置符号名，因此你必须知道接近你要查找内容的名称

```

        Ctrl-F hotplug

    or::

        Menu: File, Search, hotplug

    lists all config symbol entries that contain "hotplug" in
    the symbol name.  In this Search dialog, you may change the
    config setting for any of the entries that are not grayed out.
    You can also enter a different search string without having
    to return to the main menu.


```
## gconfig


gconfig 中搜索：

    gconfig 中没有搜索命令。不过，gconfig 确实有几种不同的查看选项、模式和设置
