
## USB 椹卞姩

鏈枃浠跺垪鍑哄綋鍓嶅彈濯掍綋椹卞姩鏀寔锛堜笉鍚?staging锛夌殑 USB 瑙嗛涓?DVB 璁惧娓呭崟锛屽寘鍚┍鍔ㄦā鍧楀悕涓庡搴斾骇鍝佸悕绉帮紝渚涢┍鍔ㄩ€傞厤銆佺‖浠堕€夊瀷浠ュ強閫氳繃 card 妯″潡鍙傛暟鎸囧畾璁惧鏃舵煡闃呫€?



USB 鏉垮崱閫氳繃涓€绉嶇О涓?USB ID 鐨勬爣璇嗘潵璇嗗埆銆?

```

    $ lsusb
    ...
    Bus 001 Device 015: ID 046d:082d Logitech, Inc. HD Pro Webcam C920
    Bus 001 Device 074: ID 2040:b131 Hauppauge
    Bus 001 Device 075: ID 2013:024f PCTV Systems nanoStick T2 290e
    ...

```
杈冩柊鐨勬憚鍍忓ご璁惧閫氳繃 USB Video Class 浠ヤ竴绉嶆爣鍑嗘柟寮忓皢鑷繁鏆撮湶涓烘憚鍍忓ご銆傝繖浜涙憚鍍忓ご鐢?`uvc-driver` 鑷姩鏀寔銆?

杈冩棫鐨勬憚鍍忓ご鍜岀數瑙?USB 璁惧浣跨敤 USB Vendor Classes锛堝巶鍟嗙被锛夛細姣忎釜鍘傚晢瀹氫箟鑷繁璁块棶璁惧鐨勬柟寮忋€傛湰鑺傚寘鍚绫诲巶鍟嗙被璁惧鐨勫崱鍒楄〃銆?

铏界劧杩欎笉濡傚湪 PCI 涓婂父瑙侊紝浣嗘湁鏃跺悓涓€ USB ID 浼氳涓嶅悓浜у搧浣跨敤銆傚洜姝わ紝澶氫釜濯掍綋椹卞姩鍏佽浼犲叆 `card=` 鍙傛暟锛屼互璁剧疆涓€涓笌鐗瑰畾浜у搧绫诲瀷鐨勬纭缃浉鍖归厤鐨勫崱鍙枫€?

褰撳墠鏀寔鐨?USB 鍗★紙涓嶅寘鎷?staging 椹卞姩锛夊涓媆 [#]_銆?


   閮ㄥ垎椹卞姩甯︽湁瀛愰┍鍔紝鏈湪鏈〃涓樉绀恒€傜壒鍒槸 gspca 椹卞姩鏈夎澶氬瓙椹卞姩锛岀敤浜庝笉琚?USB Video Class (UVC) 椹卞姩鏀寔鐨勬憚鍍忓ご锛屽 [gspca card list <gspca-cardlist>](gspca card list <gspca-cardlist>) 鎵€绀恒€?

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
