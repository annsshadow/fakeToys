
## PCI Peer-to-Peer DMA 鏀寔


The PCI 鎬荤嚎 鍏锋湁 pretty decent 鏀寔 鐢ㄤ簬 performing DMA transfers
涔嬮棿 two 璁惧 鍦?the 鎬荤嚎. 姝?绫诲瀷 鐨?transaction 鏄?henceforth
called Peer-to-Peer (鎴?P2P). 鐒惰€? 瀛樺湪 涓€涓?鏁板瓧 鐨?issues 璇?
make P2P transactions tricky 鍒?鎵ц 鍦?涓€涓?perfectly safe way.

鐢ㄤ簬 PCIe the routing 鐨?Transaction Layer Packets (TLPs) 鏄?well-defined up
鐩村埌 瀹冧滑 reach 涓€涓?host bridge 鎴?root 绔彛. 鑻?the path 鍖呭惈 PCIe switches
鐒跺悗 鍩轰簬 the ACS 璁剧疆 the transaction 鍙?route entirely 涔嬪唴
the PCIe hierarchy 鍜?浠庝笉 reach the root 绔彛. The 鍐呮牳 灏?evaluate
the PCIe topology 鍜?濮嬬粓 permit P2P 鍦?杩欎簺 well-defined cases.

鐒惰€? 鑻?the P2P transaction reaches the host bridge 鐒跺悗 瀹?鍙兘 鍏锋湁 鍒?
hairpin back out the 鐩稿悓 root 绔彛, 涓?routed inside the CPU SOC 鍒?another
PCIe root 绔彛, 鎴?routed internally 鍒?the SOC.

The PCIe specification doesn't 瀹氫箟 the forwarding 鐨?transactions 涔嬮棿
hierarchy domains 鍜?鍐呮牳 defaults 鍒?blocking 姝ょ被 routing. 瀛樺湪 涓€涓?
鍏佽 鍒楀嚭 鍒?鍏佽 detecting known-good HW, 鍦?鍏?case P2P 涔嬮棿 浠讳綍
two PCIe 璁惧 灏?涓?permitted.

Since P2P inherently 鏄?doing transactions 涔嬮棿 two 璁惧 瀹?闇€瑕?two
椹卞姩 鍒?涓?co-operating inside the 鍐呮牳. The providing 椹卞姩 鍏锋湁 鍒?convey
鍏?MMIO 鍒?the consuming 椹卞姩. 鍒?meet the 椹卞姩 鍨嬪彿 lifecycle rules the
MMIO 蹇呴』 鍏锋湁 鍏ㄩ儴 DMA 鏄犲皠 removed, 鍏ㄩ儴 CPU accesses prevented, 鍏ㄩ儴 椤?
琛?mappings undone 涔嬪墠 the providing 椹卞姩 completes remove().

姝?闇€瑕?the providing 鍜?consuming 椹卞姩 鍒?actively work together 鍒?
guarantee 璇?the consuming 椹卞姩 鍏锋湁 stopped 浣跨敤 the MMIO 鏈熼棿 涓€涓?removal
cycle. 杩欐槸 宸插畬鎴?鐢?浠讳竴涓?涓€涓?synchronous invalidation shutdown 鎴?waiting
鐢ㄤ簬 鍏ㄩ儴 usage refcounts 鍒?reach zero.

鍦?the lowest level the P2P 瀛愮郴缁?offers 涓€涓?naked 缁撴瀯浣?p2p_provider 璇?
delegates lifecycle 绠＄悊 鍒?the providing 椹卞姩. 瀹冩槸 expected 璇?
椹卞姩 浣跨敤 姝?閫夐」 灏?wrap 瀹冧滑鐨?MMIO 鍐呭瓨 鍦?DMABUF 鍜?浣跨敤 DMABUF
鍒?鎻愪緵 涓€涓?invalidation shutdown. 杩欎簺 MMIO 鍦板潃 鍏锋湁 鏃?缁撴瀯浣?椤? 鍜?
鑻?浣跨敤 涓?mmap() 蹇呴』 鍒涘缓 鐗规畩 PTEs. 浣滀负 姝ょ被 瀛樺湪 very 灏戦噺
鍐呮牳 uAPIs 璇?鍙?accept 鎸囬拡 鍒?them; 鐗瑰埆鏄?瀹冧滑 cannot 涓?浣跨敤
涓?璇诲彇()/鍐欏叆(), including O_DIRECT.

Building 鍦?姝? the 瀛愮郴缁?offers 涓€涓?layer 鍒?wrap the MMIO 鍦?涓€涓?ZONE_璁惧
pgmap 鐨?鍐呭瓨_璁惧_PCI_P2PDMA 鍒?鍒涘缓 缁撴瀯浣?椤? The lifecycle 鐨?
pgmap ensures 璇?褰?the pgmap 鏄?destroyed 鍏ㄩ儴 鍏朵粬 椹卞姩 鍏锋湁 stopped
浣跨敤 the MMIO. 姝?閫夐」 works 涓?O_DIRECT flows, 鍦?涓€浜?cases, 鑻?the
underlying 瀛愮郴缁?supports handling 鍐呭瓨_璁惧_PCI_P2PDMA through
FOLL_PCI_P2PDMA. The 浣跨敤 鐨?FOLL_LONGTERM 鏄?prevented. 浣滀负 姝?relies 鍦?pgmap
瀹?涔?relies 鍦?architecture 鏀寔 along 涓?alignment 鍜?鏈€灏?澶у皬
limitations.


## 椹卞姩 Writer's Guide


鍦?涓€涓?given P2P implementation 閭ｉ噷 鍙?涓?three 鎴?鏇村 涓嶅悓
types 鐨?鍐呮牳 椹卞姩 鍦?play:

- Provider - 涓€涓?椹卞姩 鍏?鎻愪緵 鎴?publishes P2P resources 绫讳技
  鍐呭瓨 鎴?doorbell 瀵勫瓨鍣?鍒?鍏朵粬 椹卞姩.
- Client - 涓€涓?椹卞姩 鍏?makes 浣跨敤 鐨?涓€涓?resource 鐢?璁剧疆 up 涓€涓?
  DMA transaction 鍒?鎴?鏉ヨ嚜 瀹?
- Orchestrator - 涓€涓?椹卞姩 鍏?orchestrates the flow 鐨?鏁版嵁 涔嬮棿
  clients 鍜?providers.

鍦?璁稿 cases 閭ｉ噷 鍙互 涓?overlap 涔嬮棿 杩欎簺 three types (i.e.,
瀹?鍙?涓?鍏稿瀷 鐢ㄤ簬 涓€涓?椹卞姩 鍒?涓?涓よ€?涓€涓?provider 鍜?涓€涓?client).

渚嬪, 鍦?the NVMe Target Copy Offload implementation:

- The NVMe PCI 椹卞姩 鏄?涓よ€?涓€涓?client, provider 鍜?orchestrator
  鍦?璇?瀹?exposes 浠讳綍 CMB (鎺у埗鍣?鍐呭瓨 缂撳啿鍖? 浣滀负 涓€涓?P2P 鍐呭瓨
  resource (provider), 瀹?accepts P2P 鍐呭瓨 椤?浣滀负 缂撳啿鍖?鍦?requests
  鍒?涓?浣跨敤 directly (client) 鍜?瀹?鍙?涔?make 浣跨敤 鐨?the CMB 浣滀负
  submission 闃熷垪 鏉＄洰 (orchestrator).
- The RDMA 椹卞姩 鏄?涓€涓?client 鍦?姝?arrangement 鍥犳 璇?涓€涓?RNIC
  鍙?DMA directly 鍒?the 鍐呭瓨 exposed 鐢?the NVMe 璁惧.
- The NVMe Target 椹卞姩 (nvmet) 鍙?orchestrate the 鏁版嵁 鏉ヨ嚜 the RNIC
  鍒?the P2P 鍐呭瓨 (CMB) 鍜?鐒跺悗 鍒?the NVMe 璁惧 (鍜?vice versa).

杩欐槸 currently the 浠?arrangement 鍙楁敮鎸?鐢?the 鍐呮牳 浣?
one 鍙互 imagine slight tweaks 鍒?姝?璇?灏嗕細 鍏佽 鐢ㄤ簬 the 鐩稿悓
functionality. 渚嬪, 鑻?涓€涓?鐗瑰畾 RNIC added 涓€涓?BAR 涓?涓€浜?
鍐呭瓨 behind 瀹? 鍏?椹卞姩 鍙互 add 鏀寔 浣滀负 涓€涓?P2P provider 鍜?
鐒跺悗 the NVMe Target 鍙互 浣跨敤 the RNIC's 鍐呭瓨 鑰岄潪 the CMB
鍦?cases 浣曞 the NVMe 鍗?鍦?浣跨敤 鎵ц 涓?鍏锋湁 CMB 鏀寔.


### Provider 椹卞姩


涓€涓?provider simply needs 鍒?娉ㄥ唽 涓€涓?BAR (鎴?涓€涓?portion 鐨?涓€涓?BAR)
浣滀负 涓€涓?P2P DMA resource 浣跨敤 `pci_p2pdma_add_resource()`.
姝?灏?娉ㄥ唽 缁撴瀯浣?椤?鐢ㄤ簬 鍏ㄩ儴 the specified 鍐呭瓨.

涔嬪悗 璇?瀹?鍙?optionally publish 鍏ㄩ儴 鐨?鍏?resources 浣滀负
P2P 鍐呭瓨 浣跨敤 `pci_p2pmem_publish()`. 姝?灏?鍏佽
浠讳綍 orchestrator 椹卞姩 鍒?find 鍜?浣跨敤 the 鍐呭瓨. 褰?marked 鍦?
姝?way, the resource 蹇呴』 涓?regular 鍐呭瓨 涓?鏃?side effects.

鐢ㄤ簬 the time 姝ｅ湪 杩欐槸 fairly rudimentary 鍦?璇?鍏ㄩ儴 resources
鏄?typically going 鍒?涓?P2P 鍐呭瓨. Future work 灏?likely expand
姝?鍒?鍖呭惈 鍏朵粬 types 鐨?resources 绫讳技 doorbells.


### Client 椹卞姩


涓€涓?client 椹卞姩 浠?鍏锋湁 鍒?浣跨敤 the 鏄犲皠 API `dma_map_sg()`
鍜?`dma_unmap_sg()` 鍑芥暟 浣滀负 usual, 鍜?the implementation
灏?鎵ц the right thing 鐢ㄤ簬 the P2P capable 鍐呭瓨.


### Orchestrator 椹卞姩


The 绗竴 task 涓€涓?orchestrator 椹卞姩 蹇呴』 鎵ц 鏄?compile 涓€涓?鍒楀嚭 鐨?
鍏ㄩ儴 client 璁惧 璇?灏?涓?involved 鍦?涓€涓?given transaction. 鐢ㄤ簬
绀轰緥, the NVMe Target 椹卞姩 creates 涓€涓?鍒楀嚭 including the namespace
鍧?璁惧 鍜?the RNIC 鍦?浣跨敤. 鑻?the orchestrator 鍏锋湁 access 鍒?
涓€涓?鐗瑰畾 P2P provider 鍒?浣跨敤 瀹?鍙?check compatibility 浣跨敤
`pci_p2pdma_distance()` 鍚﹀垯 瀹?鍙?find 涓€涓?鍐呭瓨 provider
璇?s compatible 涓?鍏ㄩ儴 clients 浣跨敤  `pci_p2pmem_find()`.
鑻?澶氫簬 one provider 鏄?鍙楁敮鎸? the one nearest 鍒?鍏ㄩ儴 the clients 灏?
涓?chosen 绗竴. 鑻?澶氫簬 one provider 鏄?涓€涓?equal distance away, the
one returned 灏?涓?chosen 鍦?random (瀹冩槸 涓?涓€涓?arbitrary 浣?
truly random). 姝?鍑芥暟 returns the PCI 璁惧 鍒?浣跨敤 鐢ㄤ簬 the provider
涓?涓€涓?鍙傝€?taken 鍜?鍥犳 褰?瀹?s 鏃?longer needed 瀹?搴斿綋 涓?
returned 涓?PCI_dev_put().

涓€鏃?涓€涓?provider 鏄?selected, the orchestrator 鍙?鐒跺悗 浣跨敤
`pci_alloc_p2pmem()` 鍜?`pci_free_p2pmem()` 鍒?
allocate P2P 鍐呭瓨 鏉ヨ嚜 the provider. `pci_p2pmem_alloc_sgl()`
鍜?`pci_p2pmem_free_sgl()` 鏄?convenience 鍑芥暟 鐢ㄤ簬
allocating scatter-gather 鍒楄〃 涓?P2P 鍐呭瓨.

### 缁撴瀯浣?椤?Caveats


鍚屾椂 the 鍐呭瓨_璁惧_PCI_P2PDMA 椤?鍙?涓?installed 鍦?VMAs,
pin_鐢ㄦ埛_椤?) 鍜?related 灏?涓?return them 闄ら潪 FOLL_PCI_P2PDMA 鏄?set.

The 鍐呭瓨_璁惧_PCI_P2PDMA 椤?闇€瑕?care 鍒?鏀寔 鍦?the 鍐呮牳. The
KVA 鏄?浠嶇劧 MMIO 鍜?蹇呴』 浠嶇劧 涓?accessed through the 姝ｅ父
readX()/writeX()/绛?helpers. Direct CPU access (e.g. memcpy) 鏄?forbidden, just
绫讳技 浠讳綍 鍏朵粬 MMIO 鏄犲皠. 鍚屾椂 姝?灏?actually work 鍦?涓€浜?
architectures, others 灏?experience corruption 鎴?just crash 鍦?the 鍐呮牳.
Supporting FOLL_PCI_P2PDMA 鍦?涓€涓?瀛愮郴缁?闇€瑕?scrubbing 瀹?鍒?ensure 鏃?CPU
access happens.


## Usage 涓?DMABUF


DMABUF 鎻愪緵 涓€涓?alternative 鍒?the 涓婃枃 缁撴瀯浣?page-based
client/provider/orchestrator 绯荤粺 鍜?搴斿綋 涓?浣跨敤 褰?缁撴瀯浣?椤?
doesn't exist. 鍦?姝?妯″紡 the exporting 椹卞姩 灏?wrap
涓€浜?鐨?鍏?MMIO 鍦?涓€涓?DMABUF 鍜?give the DMABUF FD 鍒?userspace.

Userspace 鍙?鐒跺悗 pass the FD 鍒?涓€涓?importing 椹卞姩 鍏?灏?ask the
exporting 椹卞姩 鍒?map 瀹?鍒?the importer.

鍦?姝?case the initiator 鍜?target PCI_璁惧 鏄?known 鍜?the P2P 瀛愮郴缁?
鏄?浣跨敤 鍒?determine the 鏄犲皠 绫诲瀷. The phys_addr_t-based DMA API 鏄?浣跨敤 鍒?
establish the dma_addr_t.

Lifecycle 鏄?controlled 鐢?DMABUF move_notify(). 褰?the exporting 椹卞姩 wants
鍒?remove() 瀹?蹇呴』 deliver 涓€涓?invalidation shutdown 鍒?鍏ㄩ儴 DMABUF importing
椹卞姩 through move_notify() 鍜?synchronously DMA unmap 鍏ㄩ儴 the MMIO.

鏃?importing 椹卞姩 鍙?continue 鍒?鍏锋湁 涓€涓?DMA map 鍒?the MMIO 涔嬪悗 the
exporting 椹卞姩 鍏锋湁 destroyed 鍏?p2p_provider.


## P2P DMA 鏀寔 搴?


   :export:
