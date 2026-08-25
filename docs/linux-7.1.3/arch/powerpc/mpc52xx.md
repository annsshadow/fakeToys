## Linux 2.6.x MPC52xx 系列

最新信息请访问 https://www.246tNt.com/mpc52xx/

要编使用 
```
     # <编辑 Makefile，设ARCH=ppc CROSS_COMPILE=...（如果需要也可设EXTRAVERSION）     # make lite5200_defconfig
     # make uImage

     然后，在 U-Boot 中：
     => tftpboot 200000 uImage
     => tftpboot 400000 pRamdisk
     => bootm 200000 400000

  - DBug::

     # <编辑 Makefile，设ARCH=ppc CROSS_COMPILE=...（如果需要也可设EXTRAVERSION）     # make lite5200_defconfig
     # cp your_initrd.gz arch/ppc/boot/images/ramdisk.image.gz
     # make zImage.initrd
     # make

     然后DBug 中：
     DBug> dn -i zImage.initrd.lite5200

```

一些说明：

 - 该移植名mpc52xxx，配置选项PPC_MPC52xx。MGT5100 不受支持，我不确定是否有人有兴趣对其进行开发。我没有采用 5xxx，是因为显然存在大量MPC5200 毫无关系5xxx。出于同样的原因，我加入'MPC' - 当然，我借鉴2.4 版本的移植。如果你认为我在某些代码的版权声明中遗漏了你/你的公司，我会尽快更正