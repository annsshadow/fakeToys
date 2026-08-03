# API Reference

This section documents the REST API surface of `o2server`.

## Generation Method

### Swagger Coverage Audit

A scan of `o2server` source code for Swagger v3 annotations (`@Operation`, `@Api`, `@Tag`) found coverage in **11 out of 55 modules** (~20%). Because coverage is below a useful threshold, API documentation is produced via the **fallback path**: extracting action metadata from `o2_core/o2/xAction/services/*.json` and supplementing with hand-written notes where the action files are sparse or malformed.

### Source Priority

1. `o2web/source/o2_core/o2/xAction/services/*.json` — action name, URI, HTTP method
2. `o2server` source code — class-level context and request/response shapes
3. Hand-written notes — modules with no discoverable endpoints

## Regeneration

To regenerate the API reference:

```bash
python docs/oa/scripts/generate_api_docs.py
```

Generated output is written to `docs/oa/api/auto/`. Human-authored supplements should be placed in the same files below the generated block, or in separate hand-written files.

## Refresh Cadence

- API docs should be refreshed when module responsibilities change or endpoints are added/removed.
- A CI job or scheduled regeneration within 30 days of initial build-out is recommended; ownership: documentation lead.

## Modules

| Module | Endpoint Source | Notes |
|--------|----------------|-------|
| x_organization_assemble_control | Action JSON (112 actions) | Rich coverage |
| x_processplatform_assemble_surface | Action JSON (250 actions) | Largest action surface |
| x_processplatform_assemble_designer | Action JSON (81 actions) | Designer APIs |
| x_portal_assemble_designer | Action JSON (64 actions) | Portal designer APIs |
| x_processplatform_assemble_bam | Action JSON (35 actions) | BAM APIs |
| x_organization_assemble_personal | Action JSON (35 actions) | Personal APIs |
| x_file_assemble_control | Action JSON (30 actions) | File attachment APIs |
| x_message_assemble_communicate | Action JSON (14 actions) | Message APIs |
| x_general_assemble_control | Action JSON (6 actions) | General utilities |
| x_faceset_control | Action JSON (10 actions) | Face recognition |
| x_hotpic_assemble_control | Action JSON (5 actions) | Hot picture APIs |
| x_component_assemble_control | Action JSON (5 actions) | Component management |
| x_collaboration_assemble_websocket | Action JSON (2 actions) | WebSocket collaboration |
| x_smartoffice_control | Action JSON (6 actions) | Smart office APIs |
| Modules with Swagger annotations | Swagger scan | See `docs/oa/api/auto/` |
