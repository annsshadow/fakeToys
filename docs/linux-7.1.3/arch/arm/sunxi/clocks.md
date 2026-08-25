## 关于 sunxi 时钟系统的常见问


本文档包含人们经常询问的有关 sunxi 时钟系统的一些有用信息，并在适当时附ASCII 图示

问：为什么主 24MHz 振荡器可以门控？这不会破坏系统吗

答：24MHz 振荡器允许门控以节省功耗。确实，如果不加注意地进行门控，系统将停止运行，但通过正确的步骤，可以对其进行门控同时保持系统运行。请考虑以下简化的挂起示例

```

      24MHz         32kHz
       |
      PLL1
       \
        \_ CPU Mux
             |
           [CPU]

   When you are about to suspend, you switch the CPU Mux to the 32kHz
   oscillator::

      24Mhz         32kHz
       |              |
      PLL1            |
                     /
           CPU Mux _/
             |
           [CPU]

    Finally you can gate the main oscillator::

                    32kHz
                      |
                      |
                     /
           CPU Mux _/
             |
           [CPU]

```
问：在哪里可以了解更多关sunxi 时钟的信息？

答：linux-sunxi wiki 包含一个记录时钟寄存器的页面，你可以在

        http://linux-sunxi.org/A10/CCM

   找到它。目前权威的信息来源Allwinner 发布ccmu 驱动，你可以

        https://github.com/linux-sunxi/linux-sunxi/tree/sunxi-3.0/arch/arm/mach-sun4i/clock/ccmu

   找到它
