## Linux 2.6.x 在 MPC52xx 系列上


最新信息请访问 https://www.246tNt.com/mpc52xx/

要编译/使用 ：

```
     # <编辑 Makefile，设置 ARCH=ppc 与 CROSS_COMPILE=...（如果需要也可设置 EXTRAVERSION）。
     # make lite5200_defconfig
     # make uImage

     然后，在 U-Boot 中：
     => tftpboot 200000 uImage
     => tftpboot 400000 pRamdisk
     => bootm 200000 400000

  - DBug::

     # <编辑 Makefile，设置 ARCH=ppc 与 CROSS_COMPILE=...（如果需要也可设置 EXTRAVERSION）。
     # make lite5200_defconfig
     # cp your_initrd.gz arch/ppc/boot/images/ramdisk.image.gz
     # make zImage.initrd
     # make

     然后在 DBug 中：
     DBug> dn -i zImage.initrd.lite5200

```

一些说明：

 - 该移植名为 mpc52xxx，配置选项为 PPC_MPC52xx。MGT5100 不受支持，我不确定是否有人有兴趣对其进行开发。我没有采用 5xxx，是因为显然存在大量与 MPC5200 毫无关系的 5xxx。出于同样的原因，我加入了 'MPC'。
 - 当然，我借鉴了 2.4 版本的移植。如果你认为我在某些代码的版权声明中遗漏了你/你的公司，我会尽快更正。
