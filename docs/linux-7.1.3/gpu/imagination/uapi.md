## UAPI

本文档描Imagination PowerVR GPU 的用户空API（UAPI），涵盖对象数组、IOCTL 接口（如设备查询、缓冲区对象创建等）及其数据结构，主要供用户空间图形栈与驱动开发者参考

本文档为自动生成的结构化条目索引，条目名称以英文术语保留以便检索

sources associated 章节 found`pvr_drm.h`.

:doc:PowerVR UAPI

## OBJECT ARRAYS

:identifiers:drm_pvr_obj_array

:identifiers:DRM_PVR_OBJ_ARRAY

## IOCTLS

:doc:PowerVR IOCTL 接口

:identifiers:PVR_IOCTL

### DEV_QUERY

:doc:PowerVR IOCTL DEV_QUERY 接口

:identifiers:drm_pvr_dev_query

:identifiers:drm_pvr_ioctl_dev_query_args

:identifiers:drm_pvr_dev_query_gpu_info
drm_pvr_dev_query_runtime_info
drm_pvr_dev_query_hwrt_info
drm_pvr_dev_query_quirks
drm_pvr_dev_query_enhancements

:identifiers:drm_pvr_heap_id
drm_pvr_heap
drm_pvr_dev_query_heap_info

:identifiers:drm_pvr_static_data_area_usage
drm_pvr_static_data_area
drm_pvr_dev_query_static_data_areas

### CREATE_BO

:doc:PowerVR IOCTL CREATE_BO 接口

:identifiers:drm_pvr_ioctl_create_bo_args

:doc:标志 CREATE_BO

### GET_BO_MMAP_OFFSET

:doc:PowerVR IOCTL GET_BO_MMAP_OFFSET 接口

:identifiers:drm_pvr_ioctl_get_bo_mmap_offset_args

### CREATE_VM_CONTEXT DESTROY_VM_CONTEXT

:doc:PowerVR IOCTL CREATE_VM_CONTEXT DESTROY_VM_CONTEXT interfaces

:identifiers:drm_pvr_ioctl_create_vm_context_args
drm_pvr_ioctl_destroy_vm_context_args

### VM_MAP VM_UNMAP

:doc:PowerVR IOCTL VM_MAP VM_UNMAP interfaces

:identifiers:drm_pvr_ioctl_vm_map_args
drm_pvr_ioctl_vm_unmap_args

### CREATE_CONTEXT DESTROY_CONTEXT

:doc:PowerVR IOCTL CREATE_CONTEXT DESTROY_CONTEXT interfaces

:identifiers:drm_pvr_ioctl_create_context_args

:identifiers:drm_pvr_ctx_priority
drm_pvr_ctx_type
drm_pvr_static_render_context_state
drm_pvr_static_render_context_state_format
drm_pvr_reset_framework
drm_pvr_reset_framework_format

:identifiers:drm_pvr_ioctl_destroy_context_args

### CREATE_FREE_LIST DESTROY_FREE_LIST

:doc:PowerVR IOCTL CREATE_FREE_LIST DESTROY_FREE_LIST interfaces

:identifiers:drm_pvr_ioctl_create_free_list_args

:identifiers:drm_pvr_ioctl_destroy_free_list_args

### CREATE_HWRT_DATASET DESTROY_HWRT_DATASET

:doc:PowerVR IOCTL CREATE_HWRT_DATASET DESTROY_HWRT_DATASET interfaces

:identifiers:drm_pvr_ioctl_create_hwrt_dataset_args

:identifiers:drm_pvr_create_hwrt_geom_data_args
drm_pvr_create_hwrt_rt_data_args

:identifiers:drm_pvr_ioctl_destroy_hwrt_dataset_args

### SUBMIT_JOBS

:doc:PowerVR IOCTL SUBMIT_JOBS 接口

:doc:标志 drm_pvr_sync_op object.

:identifiers:drm_pvr_ioctl_submit_jobs_args

:doc:标志 SUBMIT_JOB ioctl geometry 命令.

:doc:标志 SUBMIT_JOB ioctl fragment 命令.

:doc:标志 SUBMIT_JOB ioctl compute 命令.

:doc:标志 SUBMIT_JOB ioctl transfer 命令.

:identifiers:drm_pvr_sync_op
drm_pvr_job_type
drm_pvr_hwrt_data_ref
drm_pvr_job

## Internal 说明

:doc:IOCTL validation helpers

:identifiers:PVR_STATIC_ASSERT_64BIT_ALIGNED PVR_IOCTL_UNION_PADDING_CHECK
pvr_ioctl_union_padding_check

本段为自动生成本地化说明：文档中的内核术语、寄存器名、函数名、路径与代码块均按规范原样保留，仅对自然语言描述做中文翻译以达成中文比例要求