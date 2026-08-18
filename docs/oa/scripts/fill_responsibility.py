#!/usr/bin/env python3
"""Fill Responsibility fields in o2server module cards."""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
MODULES_DIR = REPO_ROOT / "docs/oa/modules/o2server"

# Responsibility descriptions by module name
RESPONSIBILITY = {
    "x_base_core_project": "基础核心项目模块，提供平台公共基础设施、工具类和通用配置。",
    "x_organization_assemble_authentication": "组织认证模块，负责用户登录、登出、会话管理、OAuth 第三方登录及验证码功能。",
    "x_organization_assemble_control": "组织控制模块，提供人员、单位、角色、用户组的完整 CRUD 业务编排和权限管理。",
    "x_organization_core_entity": "组织核心实体模块，定义人员、单位、角色、用户组等组织数据的核心实体和基础查询。",
    "x_organization_core_express": "组织核心表达式模块，提供组织数据的表达式引擎和动态查询能力。",
    "x_organization_assemble_express": "组织表达式模块，提供组织相关的脚本表达式和动态处理能力。",
    "x_personal": "个人信息模块，处理当前登录用户的个人信息查询、密码修改和重置。",
    "x_personal_extend": "个人扩展模块，提供个人信息详情、头像上传和个人扩展属性管理。",
    "x_program_init": "系统初始化模块，负责系统初始化检查、密钥设置和系统初始状态管理。",
    "x_message": "消息模块，提供消息的消费、创建、已读未读标记等基础消息功能。",
    "x_message_assemble_communicate": "消息通信模块，处理消息的发送、接收、已读未读统计等通信逻辑。",
    "x_message_core_entity": "消息核心实体模块，定义消息数据模型和基础查询能力。",
    "x_file": "文件模块，提供文件上传、下载、文件夹管理等基础文件操作。",
    "x_file_assemble_control": "文件管控模块，处理文件配置、存储池、分类管理等文件业务编排。",
    "x_file_core_entity": "文件核心实体模块，定义文件和文件夹数据模型及基础 CRUD。",
    "x_calendar": "日历模块，提供日历事件的基础 CRUD 和日历管理功能。",
    "x_calendar_assemble_control": "日历管控模块，处理日历配置和日历业务编排。",
    "x_calendar_core_entity": "日历核心实体模块，定义日历事件数据模型和基础查询。",
    "x_attendance": "考勤模块，提供打卡记录和排班管理的基础考勤功能。",
    "x_attendance_assemble_control": "考勤管控模块，处理考勤规则、排班管理和申诉审批流程。",
    "x_attendance_core_entity": "考勤核心实体模块，定义打卡记录、排班规则等考勤数据模型。",
    "x_meeting": "会议模块，提供会议室管理、会议创建和参与人管理功能。",
    "x_meeting_assemble_control": "会议管控模块，处理会议业务编排、日程关联和会议室调度。",
    "x_meeting_core_entity": "会议核心实体模块，定义会议室和会议数据模型。",
    "x_portal": "门户模块，提供门户页面和部件的基础管理功能。",
    "x_portal_assemble_designer": "门户设计器模块，处理门户页面的设计和配置管理。",
    "x_portal_assemble_surface": "门户展现模块，处理门户页面的预览、发布和渲染。",
    "x_portal_core_entity": "门户核心实体模块，定义门户页面和部件的数据模型。",
    "x_process_designer": "流程设计器模块，提供流程应用的设计、表单定义和路由配置。",
    "x_process_express": "流程表达式模块，提供流程相关的脚本表达式和执行能力。",
    "x_process_surface": "流程展现模块，处理流程实例的查询和工作流状态展示。",
    "x_process_bam": "流程 BAM 模块，提供流程业务活动监控和统计分析。",
    "x_processplatform_assemble_bam": "流程平台 BAM 管控模块，处理流程监控配置和统计报表。",
    "x_processplatform_assemble_designer": "流程平台设计器管控模块，处理流程应用的预览和发布管理。",
    "x_processplatform_assemble_surface": "流程平台展现管控模块，处理流程表面的预览和发布功能。",
    "x_processplatform_core_entity": "流程平台核心实体模块，定义工作、任务、工单等流程数据模型。",
    "x_processplatform_core_express": "流程平台核心表达式模块，提供流程任务的终止、撤回等操作能力。",
    "x_processplatform_service_processing": "流程平台服务处理模块，处理流程实例的创建、执行和取消等操作。",
    "x_query_assemble_designer": "查询设计器管控模块，提供查询视图的设计、创建和管理。",
    "x_query_assemble_surface": "查询展现管控模块，处理查询视图的预览和结果展示。",
    "x_query_core_entity": "查询核心实体模块，定义查询项、视图和导入等查询数据模型。",
    "x_query_core_express": "查询核心表达式模块，提供查询执行和历史记录能力。",
    "x_query_express": "查询表达式模块，提供查询列表和基础查询能力。",
    "x_query_service": "查询服务模块，处理查询服务的神经生成和列表管理。",
    "x_query_service_processing": "查询服务处理模块，提供查询执行、生成和列表管理能力。",
    "x_cms_assemble_control": "CMS 管控模块，处理 CMS 栏目、文章、字典等内容的配置和管理。",
    "x_cms_control": "CMS 控制模块，提供 CMS 配置和基础管理功能。",
    "x_cms_core_entity": "CMS 核心实体模块，定义栏目、应用、配置等 CMS 数据模型。",
    "x_cms_core_express": "CMS 核心表达式模块，提供 CMS 内容列表和查询能力。",
    "x_cms_express": "CMS 表达式模块，提供 CMS 的 UUID 生成、模板和视图查询。",
    "x_bbs": "论坛模块，提供论坛分类、帖子和版主管理功能。",
    "x_bbs_assemble_control": "论坛管控模块，处理论坛配置、版块管理和主题回复。",
    "x_bbs_core_entity": "论坛核心实体模块，定义论坛、版块、主题等 BBS 数据模型。",
    "x_general": "通用服务模块，提供区域管理和安全许可等通用功能。",
    "x_general_assemble_control": "通用管控模块，处理参会范围、区域管理、发票和二维码等综合配置。",
    "x_general_core_entity": "通用核心实体模块，定义字典、文件、发票等通用数据模型。",
    "x_component": "组件模块，提供组件应用的基本列表和查询功能。",
    "x_component_assemble_control": "组件管控模块，处理应用中心、市场配置和部署管理。",
    "x_component_core_entity": "组件核心实体模块，定义组件数据模型和 CRUD 能力。",
    "x_hotpic": "热点图片模块，提供轮播图和推荐图片的基础功能。",
    "x_hotpic_assemble_control": "热点图片管控模块，处理轮播配置、面板和应用管理。",
    "x_hotpic_core_entity": "热点图片核心实体模块，定义热点图片数据模型。",
    "x_jpush": "推送模块，提供设备管理和消息推送的基础功能。",
    "x_jpush_assemble_control": "推送管控模块，处理推送配置、应用和消息管理。",
    "x_jpush_core_entity": "推送核心实体模块，定义设备和模板等推送数据模型。",
    "x_mind": "思维导图模块，提供思维导图的创建、编辑和共享功能。",
    "x_mind_assemble_control": "思维导图管控模块，处理导图配置和文件夹管理。",
    "x_mind_core_entity": "思维导图核心实体模块，定义导图、文件夹和版本数据模型。",
    "x_ai": "AI 模块，提供 AI 模型管理、对话历史和配置功能。",
    "x_ai_assemble_control": "AI 管控模块，处理 AI 应用、模型和对话的业务编排。",
    "x_ai_core_entity": "AI 核心实体模块，定义 AI 应用、模型和对话数据模型。",
    "x_correlation": "关联关系模块，提供数据关联和引用管理的基础功能。",
    "x_correlation_core_entity": "关联关系核心实体模块，定义关联关系数据模型。",
    "x_correlation_core_express": "关联关系核心表达式模块，提供关联状态和同步能力。",
    "x_correlation_service_processing": "关联关系服务处理模块，处理关联数据的创建、保存和删除。",
    "x_program_center": "程序中心模块，提供应用程序管理、脚本集合和配置功能。",
    "x_program_center_core_entity": "程序中心核心实体模块，定义应用程序、脚本、调用、代理和结构数据模型。",
    "x_console": "控制台模块，提供命令行、日志查看和系统监控功能。",
    "x_express": "快递模块，提供快递查询、区域管理和订阅功能。",
}


def fill_responsibility(card_path):
    content = card_path.read_text(encoding="utf-8")

    # Check if Responsibility section is empty (only whitespace between heading and next section)
    pattern = r"(## Responsibility\s*\n)(\s*\n)(\n##)"
    match = re.search(pattern, content)
    if not match:
        return False  # Already has content or different format

    module_name = card_path.stem
    description = RESPONSIBILITY.get(module_name, f"{module_name.replace('x_', '').replace('_', ' ')} 模块。")

    new_content = re.sub(
        pattern,
        lambda m: m.group(1) + description + "\n" + m.group(3),
        content,
    )

    if new_content == content:
        return False

    card_path.write_text(new_content, encoding="utf-8")
    return True


def main():
    count = 0
    total = 0
    for card in sorted(MODULES_DIR.glob("*.md")):
        total += 1
        if fill_responsibility(card):
            count += 1
            print(f"  Filled: {card.name}")

    print(f"\nTotal: {count} cards filled out of {total}")


if __name__ == "__main__":
    main()
