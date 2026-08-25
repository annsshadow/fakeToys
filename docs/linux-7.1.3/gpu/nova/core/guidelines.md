## 指南（Guidelines


本文档包nova-core 的指南。此外，Nova 项目的所有通用指南均适用

## 驱动 API


nova-core 的主要目的之一是实现围GSP 固件接口的抽象，并为第二级驱动（nova-drm vGPU 管理VFIO 驱动）提供与固件（版本）无关API

因此，不得通过驱动 API 将固件（版本）相关的细节泄漏给第二级驱动

## 验收标准


- 在可能的范围内，提交nova-core 的补丁必须与所有第二级驱动一起进行回归测试
