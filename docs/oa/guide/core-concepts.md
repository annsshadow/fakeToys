# Core Concepts

This section documents the platform's foundational conventions and mechanisms that every contributor should understand.

## Entity Naming Conventions

JPA entity classes are defined in `x_*_core_entity` modules and follow these rules:

- **Package**: `com.x.{domain}.entity` where `{domain}` is the functional area (e.g., `organization`, `processplatform`, `cms`).
- **Class name**: PascalCase, typically singular noun (e.g., `Person`, `Group`, `Meeting`).
- **Table name**: Set via `@Table(name = "...")`; convention is snake_case domain prefix (e.g., `organization_person`, `process_work`).
- **Base class**: Most entities extend `com.x.base.core.entity.AbstractPersistence` or implement `Persistable`.
- **Field naming**: camelCase for Java fields; column mappings may use snake_case via `@Column`.

## Layering

Each functional domain follows a four-layer module structure:

| Layer | Suffix | Responsibility |
|-------|--------|----------------|
| Entity | `_core_entity` | JPA entities, table mappings |
| Express | `_core_express` | Script-accessible wrappers around entities |
| Assemble Control | `_assemble_control` | Business logic, REST filters, action classes |
| Assemble Surface | `_assemble_surface` | Presentation rendering, view models |

Additional layers:
- **Service Processing** (`_service_processing`) — background jobs and scheduled tasks.
- **Designer** (`_assemble_designer`) — low-code designers (form, portal, process).

## Express Scripting

Express is the platform's scripting language for dynamic behavior in process conditions, portal pages, query expressions, and data validations.

- Express wrappers are generated or hand-written in `x_*_core_express` modules.
- Each wrapper class exposes entity fields and business methods as script-callable properties.
- Scripts are evaluated server-side and can reference context variables, action results, and entity fields.
- The script designer components provide syntax-aware editing for non-developers.

## Action Registration and Service Discovery

REST endpoints are defined in two places:

1. **Server-side action JSON files**: `o2web/source/o2_core/o2/xAction/services/*.json` map action names to URIs and HTTP methods. These files use JSON with single-line comments. Each key is an action name; the value contains `uri`, `method`, and optional flags like `enctype` and `progress`.

2. **Server-side JAX-RS classes**: `x_*_assemble_control` modules contain `jaxrs/action/{resource}/Action*.java` classes annotated with JAX-RS path bindings. Swagger annotations (`@Operation`, `@Api`, `@Tag`) are present in some modules but not all.

Client-side discovery:
- `MWF.Actions.get("{module_name}")` loads the action map for a module.
- Actions are called by name from components; the MWF framework resolves the URI and method.

## Module Registration

New backend modules must be added to the parent POM:

- `oa/o2server/pom.xml` contains a `<modules>` section listing all 55 module directories.
- The build order is derived from Maven's reactor; entity modules must precede assemble modules that depend on them.

## Component Registration

New frontend components must be registered in `o2web/gulpapps.js` so the gulp build includes them. The build pipeline:
- Reads `gulpapps.js` for component source paths.
- Concatenates, minifies, and outputs to `dest/`.
- Supports language-pack compilation via `@o2oa/language-tools`.

## Configuration

Runtime configuration lives in `config/` (created at first start) and is copied from `configSample/` (reference). The `manifest.json` in `configSample/` indexes all available configuration files by name.

Key conventions:
- JSON files are loaded at startup and hot-reloaded where supported.
- Override values in `local/` take precedence over `configSample/`.
- Database connections are configured in `externalDataSources*.json` per vendor.

## Update Mechanism

The server supports hot-update via `local/update/o2server/`:
- Place the update package (containing `console.jar`, `index.html`, module directories, and scripts) into `local/update/o2server/`.
- The startup script detects this directory, copies files into place, and exits.
- Restart the server to run the updated version.
