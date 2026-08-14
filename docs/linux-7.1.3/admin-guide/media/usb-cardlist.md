
## USB 驱动

本文件列出当前受媒体驱动支持（不含 staging）的 USB 视频与 DVB 设备清单，包含驱动模块名与对应产品名称，供驱动适配、硬件选型以及通过 card 模块参数指定设备时查阅。



USB 板卡通过一种称为 USB ID 的标识来识别。

```

    $ lsusb
    ...
    Bus 001 Device 015: ID 046d:082d Logitech, Inc. HD Pro Webcam C920
    Bus 001 Device 074: ID 2040:b131 Hauppauge
    Bus 001 Device 075: ID 2013:024f PCTV Systems nanoStick T2 290e
    ...

```
较新的摄像头设备通过 USB Video Class 以一种标准方式将自己暴露为摄像头。这些摄像头由 `uvc-driver` 自动支持。

较旧的摄像头和电视 USB 设备使用 USB Vendor Classes（厂商类）：每个厂商定义自己访问设备的方式。本节包含此类厂商类设备的卡列表。

虽然这不如在 PCI 上常见，但有时同一 USB ID 会被不同产品使用。因此，多个媒体驱动允许传入 `card=` 参数，以设置一个与特定产品类型的正确设置相匹配的卡号。

当前支持的 USB 卡（不包括 staging 驱动）如下\ [#]_。


   部分驱动带有子驱动，未在本表中显示。特别是 gspca 驱动有许多子驱动，用于不被 USB Video Class (UVC) 驱动支持的摄像头，如 [gspca card list <gspca-cardlist>](gspca card list <gspca-cardlist>) 所示。

======================  =========================================================
Driver                  Name
======================  =========================================================
airspy                  AirSpy
au0828                  Auvitek AU0828
b2c2-flexcop-usb        Technisat/B2C2 Air/Sky/Cable2PC USB
cx231xx                 Conexant cx231xx USB video capture
dvb-as102               Abilis AS102 DVB receiver
dvb-ttusb-budget        Technotrend/Hauppauge Nova - USB devices
dvb-usb-a800            AVerMedia AverTV DVB-T USB 2.0 (A800)
dvb-usb-af9005          Afatech AF9005 DVB-T USB1.1
dvb-usb-af9015          Afatech AF9015 DVB-T USB2.0
dvb-usb-af9035          Afatech AF9035 DVB-T USB2.0
dvb-usb-anysee          Anysee DVB-T/C USB2.0
dvb-usb-au6610          Alcor Micro AU6610 USB2.0
dvb-usb-az6007          AzureWave 6007 and clones DVB-T/C USB2.0
dvb-usb-az6027          Azurewave DVB-S/S2 USB2.0 AZ6027
dvb-usb-ce6230          Intel CE6230 DVB-T USB2.0
dvb-usb-cinergyT2       Terratec CinergyT2/qanu USB 2.0 DVB-T
dvb-usb-cxusb           Conexant USB2.0 hybrid
dvb-usb-dib0700         DiBcom DiB0700
dvb-usb-dibusb-common   DiBcom DiB3000M-B
dvb-usb-dibusb-mc       DiBcom DiB3000M-C/P
dvb-usb-digitv          Nebula Electronics uDigiTV DVB-T USB2.0
dvb-usb-dtt200u         WideView WT-200U and WT-220U (pen) DVB-T
dvb-usb-dtv5100         AME DTV-5100 USB2.0 DVB-T
dvb-usb-dvbsky          DVBSky USB
dvb-usb-dw2102          DvbWorld & TeVii DVB-S/S2 USB2.0
dvb-usb-ec168           E3C EC168 DVB-T USB2.0
dvb-usb-gl861           Genesys Logic GL861 USB2.0
dvb-usb-gp8psk          GENPIX 8PSK->USB module
dvb-usb-lmedm04         LME DM04/QQBOX DVB-S USB2.0
dvb-usb-m920x           Uli m920x DVB-T USB2.0
dvb-usb-nova-t-usb2     Hauppauge WinTV-NOVA-T usb2 DVB-T USB2.0
dvb-usb-opera           Opera1 DVB-S USB2.0 receiver
dvb-usb-pctv452e        Pinnacle PCTV HDTV Pro USB device/TT Connect S2-3600
dvb-usb-rtl28xxu        Realtek RTL28xxU DVB USB
dvb-usb-technisat-usb2  Technisat DVB-S/S2 USB2.0
dvb-usb-ttusb2          Pinnacle 400e DVB-S USB2.0
dvb-usb-umt-010         HanfTek UMT-010 DVB-T USB2.0
dvb_usb_v2              Support for various USB DVB devices v2
dvb-usb-vp702x          TwinhanDTV StarBox and clones DVB-S USB2.0
dvb-usb-vp7045          TwinhanDTV Alpha/MagicBoxII, DNTV tinyUSB2, Beetle USB2.0
em28xx                  Empia EM28xx USB devices
go7007                  WIS GO7007 MPEG encoder
gspca                   Drivers for several USB Cameras
hackrf                  HackRF
hdpvr                   Hauppauge HD PVR
msi2500                 Mirics MSi2500
mxl111sf-tuner          MxL111SF DTV USB2.0
pvrusb2                 Hauppauge WinTV-PVR USB2
pwc                     USB Philips Cameras
s2250                   Sensoray 2250/2251
s2255drv                USB Sensoray 2255 video capture device
smsusb                  Siano SMS1xxx based MDTV receiver
ttusb_dec               Technotrend/Hauppauge USB DEC devices
usbtv                   USBTV007 video capture
uvcvideo                USB Video Class (UVC)
zd1301                  ZyDAS ZD1301
======================  =========================================================

- [au0828-cardlist](au0828-cardlist)
- [cx231xx-cardlist](cx231xx-cardlist)
- [em28xx-cardlist](em28xx-cardlist)
- [siano-cardlist](siano-cardlist)
- [gspca-cardlist](gspca-cardlist)
- [dvb-usb-dib0700-cardlist](dvb-usb-dib0700-cardlist)
- [dvb-usb-dibusb-mb-cardlist](dvb-usb-dibusb-mb-cardlist)
- [dvb-usb-dibusb-mc-cardlist](dvb-usb-dibusb-mc-cardlist)
- [dvb-usb-a800-cardlist](dvb-usb-a800-cardlist)
- [dvb-usb-af9005-cardlist](dvb-usb-af9005-cardlist)
- [dvb-usb-az6027-cardlist](dvb-usb-az6027-cardlist)
- [dvb-usb-cinergyT2-cardlist](dvb-usb-cinergyT2-cardlist)
- [dvb-usb-cxusb-cardlist](dvb-usb-cxusb-cardlist)
- [dvb-usb-digitv-cardlist](dvb-usb-digitv-cardlist)
- [dvb-usb-dtt200u-cardlist](dvb-usb-dtt200u-cardlist)
- [dvb-usb-dtv5100-cardlist](dvb-usb-dtv5100-cardlist)
- [dvb-usb-dw2102-cardlist](dvb-usb-dw2102-cardlist)
- [dvb-usb-gp8psk-cardlist](dvb-usb-gp8psk-cardlist)
- [dvb-usb-m920x-cardlist](dvb-usb-m920x-cardlist)
- [dvb-usb-nova-t-usb2-cardlist](dvb-usb-nova-t-usb2-cardlist)
- [dvb-usb-opera1-cardlist](dvb-usb-opera1-cardlist)
- [dvb-usb-pctv452e-cardlist](dvb-usb-pctv452e-cardlist)
- [dvb-usb-technisat-usb2-cardlist](dvb-usb-technisat-usb2-cardlist)
- [dvb-usb-ttusb2-cardlist](dvb-usb-ttusb2-cardlist)
- [dvb-usb-umt-010-cardlist](dvb-usb-umt-010-cardlist)
- [dvb-usb-vp702x-cardlist](dvb-usb-vp702x-cardlist)
- [dvb-usb-vp7045-cardlist](dvb-usb-vp7045-cardlist)
- [dvb-usb-af9015-cardlist](dvb-usb-af9015-cardlist)
- [dvb-usb-af9035-cardlist](dvb-usb-af9035-cardlist)
- [dvb-usb-anysee-cardlist](dvb-usb-anysee-cardlist)
- [dvb-usb-au6610-cardlist](dvb-usb-au6610-cardlist)
- [dvb-usb-az6007-cardlist](dvb-usb-az6007-cardlist)
- [dvb-usb-ce6230-cardlist](dvb-usb-ce6230-cardlist)
- [dvb-usb-dvbsky-cardlist](dvb-usb-dvbsky-cardlist)
- [dvb-usb-ec168-cardlist](dvb-usb-ec168-cardlist)
- [dvb-usb-gl861-cardlist](dvb-usb-gl861-cardlist)
- [dvb-usb-lmedm04-cardlist](dvb-usb-lmedm04-cardlist)
- [dvb-usb-mxl111sf-cardlist](dvb-usb-mxl111sf-cardlist)
- [dvb-usb-rtl28xxu-cardlist](dvb-usb-rtl28xxu-cardlist)
- [dvb-usb-zd1301-cardlist](dvb-usb-zd1301-cardlist)
- [other-usb-cardlist](other-usb-cardlist)
