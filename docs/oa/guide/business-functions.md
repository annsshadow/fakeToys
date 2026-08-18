# Business Functions

This section describes what the platform does, organized by domain. Cross-reference the module cards under `modules/o2server/` for implementation details.

## Organization

The organization domain manages people, groups, roles, identities, and organizational hierarchies.

Core capabilities:
- Person lifecycle (create, edit, delete, lock, ban)
- Identity management and credential verification
- Group/unit tree with recursive operations
- Role assignment and permission settings
- Person cards, attributes, and duty management
- Import and synchronization from external data sources

Relevant modules:
- `x_organization_core_entity`
- `x_organization_core_express`
- `x_organization_assemble_control`
- `x_organization_assemble_surface`
- `x_organization_assemble_authentication`
- `x_organization_assemble_personal`
- `x_organization_assemble_express`

Relevant component:
- `x_component_Org`

## Process Engine

The process engine handles workflow definition, instance execution, task routing, and form integration.

Core capabilities:
- Process definition designer (flow nodes, gateways, events)
- Work item routing (by role, identity, or expression)
- Task center and work item lists
- Process instance lifecycle (start, approve, reject, withdraw, reroute)
- Document signing and versioning
- Process statistics and BAM monitoring

Relevant modules:
- `x_processplatform_core_entity`
- `x_processplatform_core_express`
- `x_processplatform_assemble_designer`
- `x_processplatform_assemble_surface`
- `x_processplatform_assemble_bam`
- `x_processplatform_service_processing`

Relevant components:
- `x_component_process_ProcessDesigner`
- `x_component_process_ProcessManager`
- `x_component_process_Work`
- `x_component_process_TaskCenter`
- `x_component_process_FormDesigner`
- `x_component_process_Xform`

## Forms

Forms are the data-entry surface for process-driven and standalone applications.

Core capabilities:
- Form designer with field types, validation, and layout
- XForm rendering and data binding
- Form data records stored as `DataRecord` entities
- Import/export of form data

Relevant modules:
- `x_processplatform_core_entity` (DataRecord)
- `x_query_assemble_designer`
- `x_query_service_processing`

Relevant components:
- `x_component_process_FormDesigner`
- `x_component_process_Xform`

## Portal

Portals compose pages, widgets, and scripts into role-based workspaces.

Core capabilities:
- Page designer with drag-and-drop layout
- Portal hierarchy and permission scoping
- Script designer for page-level Express scripts
- Widget catalog and configuration

Relevant modules:
- `x_portal_core_entity`
- `x_portal_assemble_designer`
- `x_portal_assemble_surface`

Relevant components:
- `x_component_portal_Portal`
- `x_component_portal_PageDesigner`
- `x_component_portal_WidgetDesigner`
- `x_component_portal_ScriptDesigner`

## Attendance

Attendance tracks work hours, shifts, appeals, and location-based checks.

Core capabilities:
- Shift and work-time configuration
- Daily attendance records
- Appeal and audit workflow
- Mobile attendance support

Relevant modules:
- `x_attendance_core_entity`
- `x_attendance_assemble_control`

Relevant components:
- `x_component_Attendance`
- `x_component_attendancev2`

## CMS

CMS manages structured content, categories, applications, and views.

Core capabilities:
- Category and article management
- Application catalog
- View designer for content presentation
- Batch operations and publishing workflow

Relevant modules:
- `x_cms_core_entity`
- `x_cms_core_express`
- `x_cms_assemble_control`

Relevant components:
- `x_component_cms_FormDesigner`
- `x_component_cms_ViewDesigner`
- `x_component_cms_ScriptDesigner`
- `x_component_cms_Document`
- `x_component_cms_Column`

## File

File management provides attachment handling, storage routing, and preview.

Core capabilities:
- Attachment upload, download, and delete
- Multiple storage backends (local, WebDAV, FTP, SFTP)
- Weighted load balancing across storage endpoints
- Deep-path sharding for large file sets

Relevant modules:
- `x_file_core_entity`
- `x_file_assemble_control`

Relevant components:
- `x_component_File`

## Meeting

Meeting manages room booking, meeting instances, and attachments.

Core capabilities:
- Room and building management
- Meeting creation and lifecycle
- Meeting attachment and minute management

Relevant modules:
- `x_meeting_core_entity`
- `x_meeting_assemble_control`

Relevant component:
- `x_component_Meeting`

## Message

Message covers IM conversations, notifications, and push channels.

Core capabilities:
- IM conversation management
- Message sending and collection
- Push notification via JPush, WeCom, WeLink, DingTalk

Relevant modules:
- `x_message_core_entity`
- `x_message_assemble_communicate`

Relevant components:
- `x_component_IMV2`
- `x_component_Search`

## AI

AI provides model management and completion services.

Core capabilities:
- AI model registration and versioning
- Completion request/response tracking
- Clue management for training data

Relevant modules:
- `x_ai_core_entity`
- `x_ai_assemble_control`

Relevant components:
- `x_component_AI`
- `x_component_ANN`

## Calendar

Calendar manages events, repeats, and sharing.

Core capabilities:
- Event creation and recurrence rules
- Calendar sharing and permissions
- Event comments and reminders

Relevant modules:
- `x_calendar_core_entity`
- `x_calendar_assemble_control`

Relevant component:
- `x_component_Calendar`

## BBS

BBS provides forums, topics, and permissions.

Core capabilities:
- Forum and section management
- Topic and document posting
- Permission roles and operation records

Relevant modules:
- `x_bbs_core_entity`
- `x_bbs_assemble_control`

Relevant components:
- `x_component_Forum`
- `x_component_ForumSection`
- `x_component_ForumCategory`

## Mind

Mind manages mind maps, folders, and icons.

Core capabilities:
- Mind map creation and editing
- Folder organization
- Icon library and recycle bin

Relevant modules:
- `x_mind_core_entity`
- `x_mind_assemble_control`

Relevant components:
- `x_component_Minder`
- `x_component_MinderEditor`

## Hotpic

Hotpic manages featured image carousels.

Core capabilities:
- Carousel image upload and ordering
- Display scope configuration

Relevant modules:
- `x_hotpic_core_entity`
- `x_hotpic_assemble_control`

Relevant component:
- `x_component_HotArticle`

## Query

Query provides report design, import, and execution.

Core capabilities:
- Query/view designer
- Data import with model mapping
- Statement designer for SQL-like queries
- Statistics designer

Relevant modules:
- `x_query_core_entity`
- `x_query_core_express`
- `x_query_assemble_designer`
- `x_query_assemble_surface`
- `x_query_service_processing`

Relevant components:
- `x_component_query_Query`
- `x_component_query_QueryManager`
- `x_component_query_StatementDesigner`
- `x_component_query_StatDesigner`
- `x_component_query_TableDesigner`

## Correlation

Correlation links entities across domains.

Core capabilities:
- Cross-entity relationship tracking

Relevant modules:
- `x_correlation_core_entity`
- `x_correlation_core_express`
- `x_correlation_service_processing`

## General

General utilities include dictionaries, geo data, invoices, and general files.

Core capabilities:
- Application dictionaries and items
- Geographic district data
- Invoice management
- General file attachments

Relevant modules:
- `x_general_core_entity`
- `x_general_assemble_control`

Relevant component:
- `x_component_Common`

## Program Center

Program center manages server-side applications, agents, and deployment packages.

Core capabilities:
- Application deployment and management
- Agent registration and configuration
- APK packaging and distribution
- Captcha and system utilities

Relevant modules:
- `x_program_center_core_entity`
- `x_program_center`
- `x_program_init`
