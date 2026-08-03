# Data Models

This section describes the core JPA entity classes organized by domain. Entity classes are located in the `x_*_core_entity` modules under `oa/o2server/`.

## Entity Count by Domain

| Domain | Module | Entity Count |
|--------|--------|-------------|
| Organization | x_organization_core_entity | 18 |
| Process Platform | x_processplatform_core_entity | 52 |
| CMS | x_cms_core_entity | 28 |
| Program Center | x_program_center_core_entity | 19 |
| Query | x_query_core_entity | 18 |
| Attendance | x_attendance_core_entity | 40 |
| BBS | x_bbs_core_entity | 18 |
| Message | x_message_core_entity | 8 |
| Mind | x_mind_core_entity | 8 |
| Calendar | x_calendar_core_entity | 6 |
| Meeting | x_meeting_core_entity | 5 |
| General | x_general_core_entity | 5 |
| File | x_file_core_entity | 9 |
| Portal | x_portal_core_entity | 8 |
| AI | x_ai_core_entity | 4 |
| Hotpic | x_hotpic_core_entity | 1 |
| JPush | x_jpush_core_entity | 1 |
| Component | x_component_core_entity | 1 |
| Correlation | x_correlation_core_entity | 1 |
| Base | x_base_core_project | 1 |

## Key Domains

### Organization

Module: `x_organization_core_entity`

Core entities:
- `Definition` — Unit/role definitions
- `Group` — Organizational units
- `Identity` — Person identities
- `Person` — Person profiles
- `Custom` — Custom fields
- `Bind` — Identity-to-group bindings

### Process Platform

Module: `x_processplatform_core_entity`

Core entities:
- `Work` — Work items
- `Activity` — Process activities
- `ProcessUnit` — Process definitions / units
- `Task` — Tasks
- `Document` — Process documents
- `DataRecord` — Form data records
- `Attachment` — Attachments
- `DocSign` — Document signatures

### File

Module: `x_file_core_entity`

Core entities:
- `File` — File metadata
- `OriginFile` — Original file storage
- `Attachment` / `Attachment2` — Attachment wrappers
- `FileConfig` — File configuration

### CMS

Module: `x_cms_core_entity`

Core entities:
- `CategoryInfo` — Content categories
- `AppInfo` — Applications
- `AppInfoConfig` — App configurations
- `Document` — CMS documents
- `CategoryExt` — Category extensions

### Portal

Module: `x_portal_core_entity`

Core entities:
- `Portal` — Portal definitions
- `Page` / `PageVersion` — Portal pages
- `Script` — Portal scripts
- `File` — Portal files

### Query

Module: `x_query_core_entity`

Core entities:
- `Item` — Query items
- `ItemAccess` — Item access control
- `View` — Query views
- `ImportModel` / `ImportRecord` — Import definitions and records

### Attendance

Module: `x_attendance_core_entity`

Core entities:
- `AttendanceDetail` — Attendance records
- `AttendanceAdmin` — Administered scopes
- `AttendanceAppealInfo` / `AttendanceAppealAuditInfo` — Appeals
- `WorkTime` — Work time configurations

### Meeting

Module: `x_meeting_core_entity`

Core entities:
- `Meeting` — Meeting instances
- `Room` / `Building` — Room resources
- `Attachment` — Meeting attachments
- `MeetingConfig` — Meeting settings

### Message / IM

Module: `x_message_core_entity`

Core entities:
- `IMConversation` / `IMConversationExt` — Conversations
- `IMMsg` — Messages
- `IMMsgCollection` — Message collections
- `IMMsgFile` — Message file attachments

### AI

Module: `x_ai_core_entity`

Core entities:
- `AiModel` — AI model definitions
- `Completion` — Completion records
- `Clue` — Clue / training data
- `File` — AI-related files

### Mind

Module: `x_mind_core_entity`

Core entities:
- `MindBaseInfo` — Mind maps
- `MindContentInfo` — Map content
- `MindFolderInfo` — Folder structure
- `MindIconInfo` / `MindRecycleInfo` — Icons and recycle bin

### BBS

Module: `x_bbs_core_entity`

Core entities:
- `BBSForumInfo` — Forums
- `BBSPermissionInfo` / `BBSPermissionRole` — Permissions
- `BBSOperationRecord` — Operation logs
- `BBSTopicInfo` / `BBSDocumentInfo` — Topics and documents

### General

Module: `x_general_core_entity`

Core entities:
- `ApplicationDict` / `ApplicationDictItem` — Dictionaries
- `GeneralFile` — General-purpose files
- `Invoice` — Invoices
- `District` — Geographic districts

## Relationship Patterns

- Most domain entities extend `com.x.base.core.entity.AbstractPersistence` or a similar base class defined in `x_base_core_project`.
- Entities use `@Table` with a schema-qualified table name convention (`{domain}_*`).
- Relationships are expressed via JPA `@OneToMany`, `@ManyToOne`, and `@OneToOne` annotations. Cross-domain references typically go through the `x_*_core_entity` module of the target domain.
