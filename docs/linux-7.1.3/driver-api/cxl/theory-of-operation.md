
## Compute Express Link 椹卞姩鎿嶄綔鍘熺悊

涓€涓?Compute Express Link 鍐呭瓨璁惧锛圕XL Memory Device锛夋槸瀹炵幇 CXL.mem 鍗忚鐨?CXL 缁勪欢銆傚畠
鍖呭惈涓€瀹氭暟閲忕殑鏄撳け鎬у唴瀛樸€佹寔涔呭寲鍐呭瓨锛屾垨涓よ€呭吋鏈夈€傚畠琚灇涓句负涓€涓?PCI 璁惧锛岀敤浜庨厤缃互鍙婇€氳繃
MMIO 閭浼犻€掓秷鎭€傚畠瀵圭郴缁熺墿鐞嗗湴鍧€绌洪棿锛圫ystem Physical Address锛夌殑璐＄尞閫氳繃 HDM锛圚ost
Managed Device Memory锛屼富鏈虹鐞嗚澶囧唴瀛橈級瑙ｇ爜鍣ㄦ潵澶勭悊锛岃繖浜涜В鐮佸櫒鍙€夊湴瀹氫箟璁惧鍦ㄤ富鏈烘ˉ涔嬩笅
澶氫釜璁惧涔嬮棿浜ら敊锛坕nterleaved锛夊湴鍧€鑼冨洿鐨勮础鐚紝鎴栬法涓绘満妗ヤ氦閿欍€?
## CXL 鎬荤嚎

绫讳技浜?RAID 椹卞姩灏嗙鐩樺璞＄粍瑁呮垚鏂扮殑閫昏緫璁惧锛孋XL 瀛愮郴缁熻礋璐ｅ皢 PCIe 鍜?ACPI 瀵硅薄缁勮鎴?CXL.mem 瑙ｇ爜鎷撴墤銆侰XL.mem 鎷撴墤闇€瑕佽繍琛屾椂閰嶇疆锛岃繖涓€鐐逛篃绫讳技浜?RAID锛氬叿鏈夌浉鍚岀‖浠堕厤缃殑涓嶅悓
鐜鍙兘浼氬喅瀹氫互鎴劧涓嶅悓鐨勬柟寮忕粍瑁呮嫇鎵戙€備竴涓幆澧冨彲鑳戒负浜嗘€ц兘锛圧AID0锛夎€屽皢鍐呭瓨璺ㄥ涓富鏈烘ˉ鍜?绔偣鍋氭潯甯﹀寲锛坰triping锛夛紝鑰屽彟涓€涓幆澧冨彲鑳戒负浜嗗閿欒€岀鐢?CXL.mem 鎷撴墤涓殑浠讳綍鏉″甫鍖栥€?
骞冲彴鍥轰欢鍦ㄢ€淐XL 鏍圭鍙ｂ€濓紙Linux 瀵?CXL 瑙ｇ爜鎷撴墤椤跺眰鐨勬湳璇級澶勬灇涓惧嚭涓€浠戒氦閿欓€夐」鑿滃崟銆備粠閭ｉ噷寮€濮嬶紝
PCIe 鎷撴墤鍐冲畾浜嗗摢浜涚鐐瑰彲浠ュ弬涓庡摢浜涗富鏈烘ˉ瑙ｇ爜鏈哄埗銆傛牴涓庣鐐逛箣闂磋矾寰勪笂鐨勬瘡涓?PCIe 浜ゆ崲鏈洪兘寮曞叆
浜嗕竴涓彲浠ュ皢浜ら敊鎷嗗垎鐨勭偣銆備緥濡傦紝骞冲彴鍥轰欢鍙兘璇存煇涓粰瀹氳寖鍥村彧瑙ｇ爜鍒版煇涓€涓富鏈烘ˉ锛屼絾璇ヤ富鏈烘ˉ
鍙嶈繃鏉ュ張鍙兘璺ㄥ涓牴绔彛浜ら敊鍛ㄦ湡銆傜鍙ｄ笌绔偣涔嬮棿鐨勪腑闂翠氦鎹㈡満鍙兘璺ㄥ涓笅娓镐氦鎹㈡満绔彛浜ら敊鍛ㄦ湡锛?绛夌瓑銆?
涓嬮潰鏄竴涓敱 'cxl_test' 瀹氫箟鐨?CXL 鎷撴墤绀轰緥銆?cxl_test' 妯″潡鐢熸垚涓€涓ā鎷熺殑 CXL 鎷撴墤锛屽寘鍚?2 涓?涓绘満妗ワ紝姣忎釜涓绘満妗ュ悇鏈?2 涓牴绔彛銆傝繖浜涙牴绔彛鍚勮嚜杩炴帴鍒板甫鏈夌鐐圭殑鍙岃矾浜ゆ崲鏈猴細

```
    # cxl list -BEMPu -b cxl_test
    {
      "bus":"root3",
      "provider":"cxl_test",
      "ports:root3":[
        {
          "port":"port5",
          "host":"cxl_host_bridge.1",
          "ports:port5":[
            {
              "port":"port8",
              "host":"cxl_switch_uport.1",
              "endpoints:port8":[
                {
                  "endpoint":"endpoint9",
                  "host":"mem2",
                  "memdev":{
                    "memdev":"mem2",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x1",
                    "numa_node":1,
                    "host":"cxl_mem.1"
                  }
                },
                {
                  "endpoint":"endpoint15",
                  "host":"mem6",
                  "memdev":{
                    "memdev":"mem6",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x5",
                    "numa_node":1,
                    "host":"cxl_mem.5"
                  }
                }
              ]
            },
            {
              "port":"port12",
              "host":"cxl_switch_uport.3",
              "endpoints:port12":[
                {
                  "endpoint":"endpoint17",
                  "host":"mem8",
                  "memdev":{
                    "memdev":"mem8",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x7",
                    "numa_node":1,
                    "host":"cxl_mem.7"
                  }
                },
                {
                  "endpoint":"endpoint13",
                  "host":"mem4",
                  "memdev":{
                    "memdev":"mem4",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x3",
                    "numa_node":1,
                    "host":"cxl_mem.3"
                  }
                }
              ]
            }
          ]
        },
        {
          "port":"port4",
          "host":"cxl_host_bridge.0",
          "ports:port4":[
            {
              "port":"port6",
              "host":"cxl_switch_uport.0",
              "endpoints:port6":[
                {
                  "endpoint":"endpoint7",
                  "host":"mem1",
                  "memdev":{
                    "memdev":"mem1",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0",
                    "numa_node":0,
                    "host":"cxl_mem.0"
                  }
                },
                {
                  "endpoint":"endpoint14",
                  "host":"mem5",
                  "memdev":{
                    "memdev":"mem5",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x4",
                    "numa_node":0,
                    "host":"cxl_mem.4"
                  }
                }
              ]
            },
            {
              "port":"port10",
              "host":"cxl_switch_uport.2",
              "endpoints:port10":[
                {
                  "endpoint":"endpoint16",
                  "host":"mem7",
                  "memdev":{
                    "memdev":"mem7",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x6",
                    "numa_node":0,
                    "host":"cxl_mem.6"
                  }
                },
                {
                  "endpoint":"endpoint11",
                  "host":"mem3",
                  "memdev":{
                    "memdev":"mem3",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x2",
                    "numa_node":0,
                    "host":"cxl_mem.2"
                  }
                }
              ]
            }
          ]
        }
      ]
    }

```
鍦ㄨ鍒楄〃涓紝姣忎釜 "root"銆?port" 鍜?"endpoint" 瀵硅薄閮藉搴斾竴涓唴鏍?'struct cxl_port' 瀵硅薄銆?'cxl_port' 鏄竴涓兘澶熷皢鍏跺悗浠ｈВ鐮佷负 CXL.mem 鐨勮澶囥€傚洜姝?"root" 澹版槑闈?PCIe 鍙灇涓剧殑骞冲彴瑙ｇ爜
鑼冨洿锛屽苟灏嗗畠浠В鐮佸埌 "ports"锛?ports" 瑙ｇ爜鍒?"endpoints"锛涜€?"endpoints" 琛ㄧず浠?SPA锛堢郴缁熺墿鐞?鍦板潃锛夊埌 DPA锛堣澶囩墿鐞嗗湴鍧€锛夌殑瑙ｇ爜銆?
寤剁画 RAID 鐨勭被姣旓紝纾佺洏鏃㈡湁鍐冲畾 RAID 闆嗗悎缁勮鐨勬嫇鎵戝厓鏁版嵁锛屼篃鏈夎澶囦笂鐨勫厓鏁版嵁銆侰XL 绔彛鎷撴墤鍜?CXL 绔彛閾捐矾鐘舵€佸氨鏄?CXL.mem 闆嗗悎缁勮鐨勫厓鏁版嵁銆侰XL 绔彛鎷撴墤鐢?CXL.mem 璁惧鐨勫埌杈捐€屾灇涓俱€傚嵆锛?闄ら潪涓旂洿鍒?PCIe 鏍稿績灏?cxl_pci 椹卞姩缁戝畾鍒颁竴涓?CXL 鍐呭瓨鎵╁睍鍣紙Memory Expander锛夛紝鍚﹀垯 CXL 绔彛
瀵硅薄娌℃湁鐢ㄦ涔嬪湴銆傚弽涔嬶紝瀵逛簬鐑嫈鎻?绉婚櫎鍦烘櫙锛孡inux PCI 鏍稿績涓嶉渶瑕佹媶闄や氦鎹㈡満绾у埆鐨?CXL 璧勬簮锛?鍥犱负 endpoint 鐨?->remove() 浜嬩欢浼氭竻鐞嗕负鏀寔璇ュ唴瀛樻墿灞曞櫒鑰屽缓绔嬬殑绔彛鏁版嵁銆?
缁欏畾鍐呭瓨璁惧鍙兘鎷ユ湁鐨勭鍙ｅ厓鏁版嵁鍜屾綔鍦ㄨВ鐮佹柟妗堬細

```
    # cxl list -BDMu -d root -m mem3
    {
      "bus":"root3",
      "provider":"cxl_test",
      "decoders:root3":[
        {
          "decoder":"decoder3.1",
          "resource":"0x8030000000",
          "size":"512.00 MiB (536.87 MB)",
          "volatile_capable":true,
          "nr_targets":2
        },
        {
          "decoder":"decoder3.3",
          "resource":"0x8060000000",
          "size":"512.00 MiB (536.87 MB)",
          "pmem_capable":true,
          "nr_targets":2
        },
        {
          "decoder":"decoder3.0",
          "resource":"0x8020000000",
          "size":"256.00 MiB (268.44 MB)",
          "volatile_capable":true,
          "nr_targets":1
        },
        {
          "decoder":"decoder3.2",
          "resource":"0x8050000000",
          "size":"256.00 MiB (268.44 MB)",
          "pmem_capable":true,
          "nr_targets":1
        }
      ],
      "memdevs:root3":[
        {
          "memdev":"mem3",
          "pmem_size":"256.00 MiB (268.44 MB)",
          "ram_size":"256.00 MiB (268.44 MB)",
          "serial":"0x2",
          "numa_node":0,
          "host":"cxl_mem.2"
        }
      ]
    }

```
鈥︹€﹁鍛戒护鏌ヨ CXL 鎷撴墤浠ヨ闂€滅粰瀹氫竴涓唴鏍歌澶囧悕涓?'mem3' 鐨?CXL 鍐呭瓨鎵╁睍鍣紝璇ヨ澶囧彲浠ュ弬涓庡摢浜?骞冲彴绾цВ鐮佽寖鍥粹€濄€傛牴鎹粰瀹氱殑鎵╁睍鍣ㄦ嫢鏈夊灏戜釜瑙ｇ爜鍣ㄨ祫婧愶紝瀹冨彲浠ュ悓鏃跺弬涓庡涓?CXL.mem 浜ら敊闆嗗悎銆?鍦ㄦ绀轰緥涓紝mem3 鍙互鍙備笌浠ヤ笅涓€涓垨澶氫釜锛氳法涓や釜涓绘満妗ョ殑 PMEM 浜ら敊銆侀潰鍚戝崟涓富鏈烘ˉ鐨?PMEM 浜ら敊銆?璺?2 涓富鏈烘ˉ鐨勬槗澶辨€у唴瀛樹氦閿欙紝浠ュ強浠呴潰鍚戝崟涓富鏈烘ˉ鐨勬槗澶辨€у唴瀛樹氦閿欍€?
鍙嶄箣锛屽彲浠ュ弬涓庣粰瀹氬钩鍙扮骇瑙ｇ爜鑼冨洿鐨勫唴瀛樿澶囷細

```
    # cxl list -MDu -d 3.2
    [
      {
        "memdevs":[
          {
            "memdev":"mem1",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0",
            "numa_node":0,
            "host":"cxl_mem.0"
          },
          {
            "memdev":"mem5",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x4",
            "numa_node":0,
            "host":"cxl_mem.4"
          },
          {
            "memdev":"mem7",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x6",
            "numa_node":0,
            "host":"cxl_mem.6"
          },
          {
            "memdev":"mem3",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x2",
            "numa_node":0,
            "host":"cxl_mem.2"
          }
        ]
      },
      {
        "root decoders":[
          {
            "decoder":"decoder3.2",
            "resource":"0x8050000000",
            "size":"256.00 MiB (268.44 MB)",
            "pmem_capable":true,
            "nr_targets":1
          }
        ]
      }
    ]

```
鈥︹€﹀叾涓В鐮佸櫒鐨勫懡鍚嶆柟妗堜负 "decoder<port_id>.<instance_id>"銆?
## 椹卞姩鍩虹璁炬柦

鏈妭浠嬬粛 CXL 鍐呭瓨璁惧鐨勯┍鍔ㄥ熀纭€璁炬柦銆?
### CXL 鍐呭瓨璁惧

   :doc: cxl pci

   :internal:

   :doc: cxl mem

   :internal:

   :identifiers:

### CXL 绔彛

   :doc: cxl port

### CXL 鏍稿績

   :doc: cxl objects

   :internal:

   :identifiers: add_cxl_resources

   :doc: cxl core hdm

   :identifiers:

   :identifiers:

   :doc: cxl core

   :identifiers:

   :doc: cxl core pci

   :identifiers:

   :doc: cxl pmem

   :identifiers:

   :doc: cxl registers

   :identifiers:

   :doc: cxl mbox

   :identifiers:

   :doc: cxl features

API 璇︽儏鍙傝 `devm_cxl_setup_features`銆?
### CXL 鍖哄煙

   :doc: cxl core region

   :identifiers:

## 澶栭儴鎺ュ彛

### CXL IOCTL 鎺ュ彛

   :doc: UAPI

   :internal:
