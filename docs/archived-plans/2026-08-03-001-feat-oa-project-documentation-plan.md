---
title: feat: Build oa subproject documentation in docs/oa/
type: feat
status: completed
date: 2026-08-10
origin: docs/brainstorms/2026-08-03-oa-project-documentation-requirements.md
---

# feat: Build oa subproject documentation in docs/oa/

## Summary

Build a complete developer-facing documentation set for the `oa` subproject under `docs/oa/`, using Mermaid architecture diagrams and standardized module/component cards as the primary vehicle, with code-driven auto-extraction for API and data-model reference sections.

**状态更新（2026-08-10）：** 本计划所有 7 个实施单元（U1-U7）已全部完成。`docs/oa/` 目录结构完整，包含架构概览、55 张 o2server 模块卡片（含 Responsibility 字段）、86 张 o2web 组件卡片、58 个自动生成 API 文档、开发环境指南、部署指南、配置参考、数据模型参考、业务功能指南和扩展指南。API 文档生成脚本 `docs/oa/scripts/generate_api_docs.py` 已扩展覆盖全部 o2server 模块。

---

## Problem Frame

The `oa` subproject contains `o2server` (57 Maven modules) and `o2web` (component-based frontend). The existing `oa/README.md` only provides official product introduction and lacks developer-oriented depth. New contributors cannot quickly form a mental model of the system, and there is no module-level or interface-level reference manual. This plan delivers the documentation that closes both gaps while keeping ongoing maintenance cost low through template standardization and code-driven extraction where feasible.

---

## Requirements

The plan satisfies the following origin requirements:

**Document structure**
- R1. Create complete project documentation under `docs/oa/` without replacing `oa/README.md`.
- R2. Provide a unified entry point under `docs/oa/` listing all sections.

**Architecture**
- R3. Provide a system architecture diagram (Mermaid) showing `o2server`, `o2web`, and external dependencies.
- R4. Provide a module dependency diagram (Mermaid) for `o2server` with clustering to keep it readable.
- R20. Keep architecture diagrams in sync with version releases; ownership and mandatory update triggers are explicit.

**Module and component cards**
- R5. Create standardized module cards for every `o2server` Maven module, with template variants by module type.
- R6. Create component cards for `o2web` `x_component_*` components.
- R18. Use a unified template for module and component cards.
- R19. Support periodic auto-regeneration of API docs to stay in sync with code.

**Development environment and deployment**
- R7. Provide `o2server` development environment setup guide (JDK 11, Maven, local startup).
- R8. Provide `o2web` development environment setup guide (Node.js, npm, gulp).
- R9. Provide deployment guide for Windows and Linux (ports, directory layout, startup scripts).
- R10. Provide `o2server` configuration reference based on `configSample/` JSON files.

**API and data models**
- R11. Provide code-driven API documentation extraction with a Swagger coverage pre-check; if coverage is insufficient, fall back to extracting action metadata from `o2_core/o2/xAction/services/*.json` supplemented by hand-written notes for uncovered modules; output to `docs/oa/api/` or equivalent.
- R12. Provide core data model descriptions for organization, process, file, attendance, CMS, and other domains.

**Business functions and developer guides**
- R13. Organize business function descriptions by domain (organization, process engine, forms, portal, attendance, CMS, file, meeting, message, AI, etc.).
- R14. Describe low-code/secondary development capabilities (form customization, page customization, process designer).
- R15. Describe how to extend `o2server` (new assemble_control or service_processing module).
- R16. Describe how to extend `o2web` (new `x_component_*` component).
- R17. Describe platform core concepts and conventions (entity naming, Express scripting, service discovery).

**Origin actors:** New contributors to the `oa` subproject; maintainers responsible for keeping docs current.

**Origin flows:** F1. Onboarding — a new developer reads `docs/oa/` to learn the system. F2. Module lookup — a developer finds a specific module's responsibilities and endpoints. F3. Extension — a developer learns how to add a new module or component.

**Origin acceptance examples:** AE1. A new developer can describe the system boundary, core module dependency direction, and external dependencies after reading the architecture section. AE2. Every `o2server` module and major `o2web` component has a card. AE3. API docs are generated from code rather than hand-copied.

---

## Scope Boundaries

- Do not replace `oa/README.md`; keep official product introduction there.
- Do not output PDF or static site; output Markdown only.
- Do not duplicate the official platform handbook at `https://www.o2oa.net/handbook.html`; focus on developer-facing architecture, modules, and extension.
- Do not cover frontend unit testing, E2E testing, or other engineering-practice specifics.
- Do not specify exact documentation-generation tooling in this plan; select tools during implementation based on the constraints and fallback rules in R11.

### Deferred to Follow-Up Work

- Exact tool selection for API doc extraction (Swagger scan, JPA analysis, or combined) — decided after coverage audit.
- Exact clustering algorithm for R4 module diagram — decided after module dependency analysis.
- R13/R14 content split — decided by the documentation author during content outlining.
- R17 core-concept list — finalized after survey of entity naming conventions, Express scripts, and service discovery mechanisms in code.
- R17 core-concept list scope boundary — the final list is determined by the deferred code survey, not predetermined by the plan.
- Card ownership and update triggers for module cards and component cards — defined in U3, mirroring R20's architecture diagram maintenance clause.

---

## Key Technical Decisions

- **Mermaid for architecture diagrams**: Markdown-native, no extra toolchain, easy to maintain.
- **Template variants for module cards**: One unified template family with conditional sections by module type (API service / assemble_control / service_processing / tool module), avoiding empty endpoint fields on internal modules.
- **Code-driven card skeletons**: Module cards are generated from `pom.xml` metadata (artifactId, packaging, dependencies) and then enriched with prose descriptions; component cards are generated from `x_component_*/Main.js` and `package.json` where present.
- **Diagram clustering by domain**: The 57-module diagram groups modules into domain clusters (organization, process, CMS, portal, query, etc.) to preserve readability.
- **API extraction with coverage gate**: Run a Swagger annotation coverage audit first to establish the baseline and set the threshold; if coverage is insufficient, fall back to extracting action metadata from `o2_core/o2/xAction/services/*.json` supplemented by hand-written notes, rather than producing large blank API references.

---

## Context & Research

### Relevant Code and Patterns

- `oa/o2server/pom.xml` — parent POM enumerating 57 modules with strict naming conventions (`x_{domain}_core_entity`, `x_{domain}_assemble_control`, `x_{domain}_service_processing`, etc.)
- `oa/o2server/configSample/manifest.json` — index of 30+ configuration JSON files to document for R10.
- `oa/o2web/source/x_component_*/Main.js` — standard component entry point for R6 component cards.
- `oa/o2web/source/x_component_*/package.json` — present for components with modern build pipelines (Vue, React, Vite); use to detect build tooling.
- `oa/o2server/o2web/gulpfile.js` — build pipeline that assembles the web layer; relevant for R8 and for understanding component bundling.
- `oa/o2server/o2server/start_*.sh` and `start_*.bat` — deployment scripts relevant to R9.
- `oa/o2server/o2web/source/o2_core/o2/xAction/services/*.json` — REST action definitions that inform API structure for R11/R12.

### Institutional Learnings

No prior `docs/solutions/` entries found. Recommend capturing documentation-generation patterns and maintenance workflows after completion via `/ce-compound`.

### External References

- Official handbook: `https://www.o2oa.net/handbook.html` (explicitly out of scope for duplication).
- Source compilation tutorial: `https://www.o2oa.net/cms/source/335.html` (background for R7/R8).
- Windows deployment: `https://www.o2oa.net/cms/serverdeployment/694.html`
- Linux deployment: `https://www.o2oa.net/cms/serverdeployment/468.html`

---

## Implementation Units

### U1. Scaffold docs/oa/ directory and master index

**Goal:** Create the `docs/oa/` directory structure and a master entry point that lists all sections and their files.

**Requirements:** R1, R2

**Dependencies:** None

**Files:**
- Create: `docs/oa/README.md`
- Create: `docs/oa/INDEX.md`

**Approach:**
- Create `docs/oa/README.md` as the primary entry point with a brief introduction and a table of contents linking to every section.
- Create `docs/oa/INDEX.md` as a machine-readable page index (section name, file path, description) for tooling and cross-reference.
- Establish Markdown conventions (heading levels, code-fence languages, link style) consistent with existing `docs/brainstorms/` and `docs/plans/`.

**Patterns to follow:**
- `docs/brainstorms/2026-08-03-oa-project-documentation-requirements.md` frontmatter and sectioning style.

**Test scenarios:**
- Happy path: A reader opening `docs/oa/README.md` can navigate to every planned section via links.
- Edge case: Links resolve correctly even if a section file is temporarily absent during incremental build-out.

**Verification:**
- `docs/oa/README.md` exists, renders correctly, and links to all planned section files.

---

### U2. Write architecture overview and module dependency diagram

**Goal:** Produce the two Mermaid architecture diagrams (system overview and `o2server` module dependency) and accompanying prose.

**Requirements:** R3, R4, R20 (diagram ownership and update triggers)

**Dependencies:** U1

**Files:**
- Create: `docs/oa/architecture.md`

**Approach:**
- System architecture diagram: one Mermaid graph showing `o2server`, `o2web`, and external dependencies (H2/MySQL/Oracle/SQLServer, Redis, file storage, message push, LDAP, Quartz, Lucene index).
- Module dependency diagram: read `oa/o2server/pom.xml` `<modules>` and `<dependencyManagement>` to produce a Mermaid graph clustered by domain (organization, process, CMS, portal, query, file, attendance, meeting, message, AI, etc.). Keep clusters at a level that avoids a 57-node hairball.
- Document ownership and update triggers inline: system architecture diagram owned by tech lead; module dependency diagram owned by module leads collectively; mandatory update on module boundary or dependency-structure change.

**Patterns to follow:**
- Mermaid `graph TD` / `graph LR` conventions already used elsewhere in the repo.

**Test scenarios:**
- Happy path: A new developer reads `architecture.md` and can correctly describe system boundaries, core module dependency direction, and external dependencies.
- Edge case: Diagram remains readable after a module is added or removed (clustering absorbs the change without a full redraw).

**Verification:**
- `docs/oa/architecture.md` contains two Mermaid diagrams and an ownership/update-trigger note.

---

### U3. Build module-card and component-card generation tooling

**Goal:** Create a script or tool that generates standardized Markdown cards for all `o2server` modules and `o2web` components from code metadata.

**Requirements:** R5, R6, R18, R19, R20

**Dependencies:** U1

**Files:**
- Create: `docs/oa/scripts/generate_cards.py` (or equivalent; implementation detail decided in planning)
- Create: `docs/oa/modules/o2server/` directory
- Create: `docs/oa/modules/o2web/` directory
- Create: `docs/oa/templates/module-card.md`
- Create: `docs/oa/templates/component-card.md`

**Approach:**
- Module cards: parse `oa/o2server/pom.xml` for each `<module>` to collect `artifactId`, `packaging`, and declared dependencies. Produce a Markdown file per module using a template with sections: module name, responsibility description (left blank for human authoring), core classes/interfaces (extracted from `src/main/java` directory listing), dependency list (from POM), REST endpoints (from Swagger annotations or action JSON files where present).
- Template variants: if a module has no REST endpoints and no `assemble_control` / `service_processing` packaging, omit the endpoints section automatically.
- Component cards: scan `oa/o2server/o2web/source/x_component_*/Main.js` for component entry-point class names. If `package.json` exists, capture build tooling. Produce one Markdown file per component with sections: component name, responsibility, key JSON configs, build pipeline.
- Output directories: `docs/oa/modules/o2server/` and `docs/oa/modules/o2web/`.
- Ownership and update triggers: module cards are owned by the corresponding module lead; component cards by the component owner. Mandatory card refresh is triggered on module responsibility change, endpoint structure change, or dependency-structure change, coordinated with the R20 diagram-update triggers.

**Patterns to follow:**
- `oa/o2server/pom.xml` module naming and dependency structure.
- `oa/o2web/source/x_component_*/Main.js` component structure.

**Test scenarios:**
- Happy path: Running the generator produces one Markdown file per POM module and per `x_component_*` directory.
- Edge case: A module with no Swagger annotations and no action JSON gets a card with the endpoints section omitted.
- Edge case: A new module added to `pom.xml` appears in the generated output on the next run.

**Verification:**
- Card-generation script runs cleanly against the current codebase.
- Every POM module and every `x_component_*` directory has a corresponding Markdown card.

---

### U4. Author module and component card prose content

**Goal:** Enrich each generated card with human-written responsibility descriptions, core class summaries, and key JSON config explanations.

**Requirements:** R5, R6, R18

**Dependencies:** U3

**Files:**
- Modify: `docs/oa/modules/o2server/*.md` (all module cards)
- Modify: `docs/oa/modules/o2web/*.md` (all component cards)

**Approach:**
- For `o2server` modules: read the module's `src/main/java` tree and representative class names to draft a one-paragraph responsibility description. Highlight the module's role in the overall system (entity holder, assemble control, service processing, etc.). Summarize 3-5 core classes/interfaces. List dependencies as links to other module cards where possible.
- For `o2web` components: read `Main.js` and any `applications.json` to describe the component's UI role. List key JSON config files (toolbars, list items, templates, skins) with one-line explanations.
- Fill only the "responsibility description" and "core classes/interfaces" fields; leave machine-extracted fields (dependencies, endpoints) as generated.

**Patterns to follow:**
- Existing `oa/o2web/source/x_component_*/Main.js` class definitions and JSDoc conventions.

**Test scenarios:**
- Happy path: Every module card contains a non-empty responsibility description.
- Edge case: A module with only one class still gets a meaningful description rather than a stub.

**Verification:**
- No module or component card has an empty "responsibility description" section.

---

### U5. Write development environment and deployment guides

**Goal:** Produce setup and deployment instructions for `o2server` and `o2web`.

**Requirements:** R7, R8, R9, R10

**Dependencies:** U1

**Files:**
- Create: `docs/oa/development/o2server-setup.md`
- Create: `docs/oa/development/o2web-setup.md`
- Create: `docs/oa/deployment/windows.md`
- Create: `docs/oa/deployment/linux.md`
- Create: `docs/oa/reference/configuration.md`

**Approach:**
- `o2server-setup.md`: JDK 11 requirement, Maven compile (`mvn clean package` or equivalent), local startup via `start_*.bat` / `start_*.sh`, ports 80/20020/20030.
- `o2web-setup.md`: Node.js version (infer from `.nvmrc` or `package.json` engines if present), `npm install`, `gulp` build command, relationship to `o2server` build.
- `windows.md` and `linux.md`: directory layout after unzip, required open ports, which startup script to run, where logs live.
- `configuration.md`: enumerate `oa/o2server/configSample/` files using `manifest.json` as the index. For each file, explain purpose, key fields, and typical values. Prioritize `general.json`, `web.json`, `externalDataSources_*.json`, `externalStorageSources.json`, `processPlatform.json`, `portal.json`.

**Patterns to follow:**
- `oa/README.md` "How to Start" section for Windows/Linux commands.
- `oa/o2server/o2web/README.md` build instructions.

**Test scenarios:**
- Happy path: A developer following `o2server-setup.md` can compile and start `o2server` locally.
- Happy path: A developer following `o2web-setup.md` can compile the web layer.
- Edge case: Windows and Linux instructions do not contradict each other on paths or commands.

**Verification:**
- All five files exist and are consistent with observed scripts and config samples in the repo.

---

### U6. Produce API documentation and data model reference

**Goal:** Generate API reference docs from code and document core JPA entity relationships.

**Requirements:** R11, R12, R19

**Dependencies:** U1, U5

**Files:**
- Create: `docs/oa/api/README.md`
- Create: `docs/oa/api/auto/` (generated output directory)
- Create: `docs/oa/reference/data-models.md`

**Approach:**
- R11 pre-check: write a small script that scans `o2server` modules for `@Operation`, `@Api`, `@Tag`, and other Swagger v3 annotations. Report coverage per module and overall. Run the audit first as a discovery step, then set the coverage threshold based on the measured distribution; document the rationale. If a threshold must be stated upfront, treat it as a placeholder to be revised after the first audit run. If coverage is above the threshold, proceed with Swagger-based extraction. Otherwise, fall back to extracting action metadata from `o2_core/o2/xAction/services/*.json` and supplement with hand-written notes.
- API generation: produce Markdown files per module listing its REST endpoints, methods, request/response shapes, and Swagger descriptions where available. Store generated output in `docs/oa/api/auto/`.
- `api/README.md`: explain how the API docs are generated, how to re-run the generator, and the coverage baseline. Include a milestone: within 30 days of initial build-out, wire the generator into CI or a scheduled job, with ownership assigned to the documentation lead.
- `data-models.md`: for each major domain (organization, process, file, attendance, CMS, portal, message, AI, calendar, meeting, mind, BBS, hotpic, component, general, query, correlation), list the key JPA entity classes and their relationships. Diagrams are optional but encouraged for high-value domains (organization hierarchy, process instance lifecycle).

**Patterns to follow:**
- `io.swagger.v3` annotations in `o2server` source.
- `o2_core/o2/xAction/services/*.json` action definitions.

**Test scenarios:**
- Happy path: Running the API generator produces Markdown files with endpoint listings.
- Edge case: A module with zero Swagger annotations still gets an API stub pointing to the fallback source.
- Error path: If the generator script fails, the failure message tells the operator which module or annotation pattern caused the failure.

**Verification:**
- `docs/oa/api/README.md` documents the generation command, coverage baseline, and refresh cadence.
- `docs/oa/api/auto/` contains generated endpoint references for all modules with at least one discoverable endpoint.

---

### U7. Write business function and developer-guide sections

**Goal:** Document what the platform does and how to extend it.

**Requirements:** R13, R14, R15, R16, R17

**Dependencies:** U2, U4

**Files:**
- Create: `docs/oa/guide/business-functions.md`
- Create: `docs/oa/guide/low-code-capabilities.md`
- Create: `docs/oa/guide/extending-o2server.md`
- Create: `docs/oa/guide/extending-o2web.md`
- Create: `docs/oa/guide/core-concepts.md`

**Approach:**
- `business-functions.md`: one subsection per domain (organization, process engine, form customization, portal, attendance, CMS, file, meeting, message, AI, etc.). For each domain, describe the core capability in 2-4 paragraphs and list the relevant module cards for cross-reference.
- `low-code-capabilities.md`: describe form designer, page designer, and process designer from the perspective of a low-code builder. Cross-reference the relevant module cards and business-function sections.
- `extending-o2server.md`: step-by-step guide for adding a new module. Cover POM inheritance, package naming (`com.x.{domain}.{layer}`), where to place entity/assemble/service classes, and how to register the module in the parent POM. Use an existing module as a reference example.
- `extending-o2web.md`: step-by-step guide for adding a new `x_component_*`. Cover directory layout (`Main.js`, `lp/`, `$Main/`), MWF class registration, build integration via `gulpfile.js`, and language-pack conventions.
- `core-concepts.md`: document the platform's core concepts and conventions. The specific concept list is finalized after a deferred code survey of entity naming conventions, Express scripts, and service discovery mechanisms.

**Patterns to follow:**
- Existing module naming and layering in `o2server/pom.xml`.
- Existing component structure in `o2web/source/x_component_*/`.
- `o2web/source/o2_core/o2/xAction/services/*.json` for action registration.

**Test scenarios:**
- Happy path: A developer reading `extending-o2server.md` can create a new module scaffold that compiles.
- Happy path: A developer reading `extending-o2web.md` can create a new component directory that integrates with the gulp build.
- Edge case: `core-concepts.md` explains the relationship between entity classes, assemble controls, and service processing clearly enough that a reader does not need to open the source to understand the layering.

**Verification:**
- All five guide files exist.
- Cross-references between guide sections and module/component cards resolve correctly.

---

## System-Wide Impact

- **Interaction graph:** Module cards and component cards reference each other through domain clustering; architecture diagrams reference module cards by name. API docs reference module cards for implementation context.
- **Error propagation:** Card-generation scripts should fail fast on missing `pom.xml` or malformed `Main.js`, with clear per-file error messages.
- **State lifecycle risks:** Generated card skeletons and generated API docs are regenerated; human-authored prose lives in the same files and must be preserved across regenerations. Design the generator to overwrite only designated generated blocks, or use a split-file layout (generated header + human-authored body) to avoid clobbering.
- **API surface parity:** Not applicable — documentation only, no runtime API change.
- **Integration coverage:** Cross-reference links between architecture diagrams, module cards, API docs, and guides must be verified during content review.
- **Unchanged invariants:** `oa/README.md` remains the official product introduction. `docs/oa/` is additive.

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| Swagger annotation coverage is lower than expected, forcing full reliance on fallback extraction | R11 mandates a coverage audit first; fallback path is explicit |
| 57 module cards become a maintenance burden | Template variants, code-driven skeletons, and R20 ownership/triggers reduce drift |
| 57-node Mermaid diagram is unreadable | Cluster by domain (U2) to preserve readability |
| R17 core-concept list expands without bound | Keep the list to concepts required for first-time module/component extension |
| `o2web` component count is larger than expected | Component-card generator scans the directory tree; coverage is automatic |
| ConfigSample files drift from production | Document that `configSample/` is the reference and note production overrides where known |

---

## Documentation / Operational Notes

- After initial build-out, schedule a quarterly doc-sync review aligned with release cadence.
- Card generators should be runnable locally by any contributor; document the command in `docs/oa/README.md`.
- API doc generation should be wired into CI or a scheduled job within 30 days of initial build-out, with ownership assigned to the documentation lead.

---

## Sources & References

- **Origin document:** `docs/brainstorms/2026-08-03-oa-project-documentation-requirements.md`
- Related code: `oa/o2server/pom.xml`, `oa/o2server/configSample/manifest.json`, `oa/o2server/o2web/source/x_component_*/Main.js`, `oa/o2server/o2web/gulpfile.js`
- Related docs: `oa/README.md`, `oa/o2web/README.md`
- External: `https://www.o2oa.net/handbook.html`, `https://www.o2oa.net/cms/source/335.html`
