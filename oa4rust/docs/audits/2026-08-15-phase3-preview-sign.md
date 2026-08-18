# OA4RUST Phase 3 Feasibility Audit — Document Preview & E-Signature — 2026-08-15

> READ-ONLY assessment. No `.rs` / `Cargo.toml` modified; `cargo build`/`cargo test` not run.
> Scope: `crates/file_assemble_control/src`, `crates/cms_assemble_control/src`, any `office*/`,
> `ai*/`, plus a workspace-wide grep of `preview|signature|sign|pdf|render|thumbnail|convert|office|docx|seal`
> and a scan of `Cargo.lock`. No `office*` crates exist; the `ai*` crates are reviewed below.

---

## 1. Current State (what exists today)

### 1.1 File metadata & storage — Phase 2 baseline (done)
- Metadata entity `file_file`: `crates/file_core_entity/src/entities/file_file.rs:3-23`
  (id, name, person, reference_id, reference_type, extension, length, mime_type, timestamps).
  **No `content`/`storage` column** in the SeaORM entity.
- Actual binary payload is stored **inline in the DB as base64** in `FILE_FILE.content`, read directly
  by the preview handler: `crates/file_assemble_control/src/lib.rs:1372`
  `SELECT id, name, person, extension, mime_type, content FROM FILE_FILE ...`.
- File metadata CRUD + download-stream routes:
  `crates/file_assemble_control/src/routes.rs:18-28` (list/get/upload/create/delete + download stream).
- AI file metadata + **fake** download endpoints (no real bytes):
  `crates/ai/src/file.rs` (`file_download` returns `.bin` stub, `file_download_scale` returns fake `.png` metadata).

### 1.2 Document preview — PARTIAL, real for plain-text DOCX only
- Route (registered): `crates/file_assemble_control/src/routes.rs:29`
  `GET /jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}`
  → `attachment2_id_office_preview_type_type`.
- Handler: `crates/file_assemble_control/src/lib.rs:1364`. For `.docx` (or `wordprocessingml` mime)
  it calls `docx_to_html(&bytes)` (`lib.rs:1306`); for any other type or parse failure it **falls back
  to returning the raw base64 content** (`lib.rs:1399-1420`).
- `docx_to_html` (`lib.rs:1306-1334`): unzips the docx, reads `word/document.xml`, extracts `<w:p>`/`<w:t>`
  text, HTML-escapes, wraps each paragraph in `<p>`. It **bails out (returns `None` → fallback)** the moment
  the document contains a table `<w:tbl` or a drawing/image `<w:drawing` (`lib.rs:1315-1317`).
  → No styling, no images, no tables, no headers/footers, no lists, no pagination.
- Route registration + fallback behaviour covered by test: `crates/file_assemble_control/src/tests.rs:430-540`.

### 1.3 PDF preview — STUB only (not wired)
- Handler exists: `crates/cms_assemble_control/src/lib.rs:1880` `fileinfo_id_preview_pdf`
  returns `{"success": true}` only — **no rendering, no bytes**.
- **NOT registered in any `routes.rs`** (dead code); the generated test even marks it
  `SKIPPED: fileinfo_id_preview_pdf not accessible` (`crates/cms_assemble_control/src/tests_generated.rs:238`).
- No PDF library in the dependency tree (see §1.6).

### 1.4 Other "preview" routes — NOT document rendering
These return mock HTML snippets / preview-link URLs and are unrelated to document content:
- `portal_assemble_surface/src/lib.rs:146` → `format!("/preview/{}", id)`
- `processplatform_assemble_surface/src/lib.rs:142-143` → hard-coded `<div>Process Platform Preview</div>`
- `query_assemble_surface/src/lib.rs:199-200`, `processplatform_assemble_designer/src/lib.rs:269`,
  `attendance_assemble_control/src/lib.rs:1013` (`attendancedetail_mobile_mobilepreview`).
→ UI-builder / preview-link stubs; must not be confused with file-content preview.

### 1.5 E-signature / seal primitives
- **Personal "signature" image asset** (closest existing primitive):
  `crates/personal/src/signature.rs` (upload/list/delete/manager_list; routes at
  `crates/personal/src/lib.rs:194-197`). Stores a user's **handwritten signature IMAGE** (≤5 MB)
  as base64 in the `x_custom` table. This is a **signature-image repository (stamp asset)** —
  NOT a cryptographic e-signature. No PDF embedding, no certificate, no timestamp, no verification.
- **Process-platform "sign"** (`processplatform_assemble_surface/src/lib.rs:8020-8170`,
  `sign not found`): workflow approval sign-off **nodes**, not document e-signature.
- **Crypto signing present is auth-only** and irrelevant to documents:
  HMAC session token (`shared/src/session.rs:64-65`), WeChat JS-SDK signature
  (`auth/src/qiyeweixin.rs:277`), 3DES SSO/password tokens (`auth/src/sso.rs`, `auth/src/password.rs`).
- **No** seal/stamp embedding into documents, **no** watermark, **no** PKI/X.509,
  **no** PKCS#7 / PAdES, **no** trusted timestamp (TSA), **no** signature verification.

### 1.6 External services & libraries
- `docker-compose.yml`: only `postgres:14`. **No** OnlyOffice / KKFileView / LibreOffice / Collabora /
  any conversion service. (Grep for `onlyoffice|kkfileview|libreoffice|collabora` across the repo: 0 hits.)
- `Cargo.lock` relevant deps: `base64`, `image`, `openssl`, `rustls`, `sha1`, `sha2`, `zip`.
  **No** `pdf`/`lopdf`/`printpdf`, **no** `pkcs`/`rcgen`/`x509`, **no** `docx`/`calamine`,
  **no** `libreoffice`/`onlyoffice` bindings.
- `image` crate is **not directly used** anywhere (`use image` / `image::` → 0 hits); it is transitive only.
  → No thumbnail generation is implemented (the AI `file_download_scale` returns fake PNG metadata).

---

## 2. Gaps

### (a) Online document preview
- **Office → HTML**: only plain-text DOCX without tables/images. No XLSX/PPTX/DOC/legacy Office;
  no styling/layout, no pagination, no images, no tables.
- **PDF preview**: non-existent (stub only).
- No thumbnail generation, no image preview, no paginated/zoomable viewer.
- **Inline DB base64 storage** (`FILE_FILE.content`) works for small files but is unsuitable for large
  Office/PDF; there is no object storage, streaming cache, or pre-rendered cache layer.

### (b) E-signature
- **No digital (cryptographic) signature at all**: no PDF signing, no X.509 cert issuance/integration,
  no PKCS#7/PAdES, no trusted timestamp (TSA), no signature verification/validation, no LTV archival.
- No seal/company-chop embedding into documents; only a raw signature-image store with no positioning
  or rendering into the document.
- No audit trail / signature evidence / certificate of completion.

---

## 3. Recommended Approach

### 3.1 Preview — prefer an external conversion service (recommended, lowest risk)
- Stand up a document-conversion microservice (OnlyOffice Docs, **or** KKFileView, **or** Collabora Online)
  as a separate container. oa4rust calls it over HTTP to obtain HTML/image/PDF. This is the highest-fidelity,
  lowest-effort path for faithful Office **and** PDF rendering (layout, tables, images, pagination).
  - Add an env config (service URL) and add the service container to `docker-compose.yml` / `deploy/`.
- **Alternative (Rust-native, higher effort / lower fidelity)**: drive `libreoffice --headless
  --convert-to` as a sidecar process via `std::process::Command`; for PDF rasterization use
  `pdfium`/`lopdf`. Pure-Rust Office fidelity (tables/images/layout) is weak — `calamine` (xlsx) and a
  real docx parser exist but layout fidelity is poor.
- **Build a render/cache layer**: convert once → store rendered output (object storage or a cache table),
  serve HTML/images through a JS viewer. Decouple from the inline `FILE_FILE.content` column.

### 3.2 E-signature — external service + Rust glue
- **Cryptographic signing must use a library/service**:
  - PDF digital signature: Rust `lopdf` (or `pdf`/`printpdf`) + `pkcs11`/`rcgen` for key/cert, **or**
    delegate to a CA / e-signature SaaS / internal signing service. PAdES baseline (B-B / B-T / B-LT)
    requires X.509 + TSA — non-trivial to build in-house.
  - Seal/image stamp: overlay a seal PNG (reuse the personal signature-image store at
    `crates/personal/src/signature.rs`) onto a PDF page at a coordinate via a PDF library. No external
    service strictly required, but version/edge-case handling is the risk.
  - PKI/cert lifecycle, TSA, identity verification, and legal compliance should be **treated as
    integration points (external CA / e-sign SaaS)** rather than built from scratch.
- **Recommended**: adopt an e-signature service (or OnlyOffice/LibreOffice "insert signature" + a signing
  module) and wrap it with Rust handlers; keep oa4rust as the orchestration / metadata / audit-trail layer.
  Store signature evidence (cert serial, signer, timestamp, document hash) in a new table.

### 3.3 Own vs. Integrate split
- **oa4rust owns**: routing, session/auth gating, file metadata, storage of rendered cache +
  signature evidence, audit trail, and the personal signature-image asset store.
- **Integrate (external)**: Office/PDF → view rendering; PDF digital signing & cert/PKI/TSA;
  possibly seal-image embedding into the document.

---

## 4. Risk Register

| ID | Risk | Likelihood | Impact | Mitigation |
|----|------|-----------|--------|------------|
| R1 | Pure-Rust Office rendering fidelity poor (tables/images/layout lost) | High | High | Use external conversion service (OnlyOffice/KKFileView) instead of native parsers |
| R2 | Large files in inline base64 DB column cause perf/memory issues | High | Medium | Move to object storage / rendered-cache layer; stream conversion |
| R3 | Building PAdES + TSA + LTV in-house is legally & technically risky | High | High | Integrate CA / e-sign SaaS or a mature lib; treat as external |
| R4 | PDF digital-signature libs (`lopdf`/`pkcs11`) have version/edge-case gaps | Medium | High | Vendor via signing service; thorough test corpus |
| R5 | No external service in `docker-compose` today → Phase 3 adds infra | Medium | Medium | Add container(s) + env config; document under `deploy/` |
| R6 | Personal signature store is plaintext base64, no integrity/access control beyond owner | Medium | Low | Add verification/audit; treat as stamp asset only |
| R7 | "preview" routes elsewhere are UI stubs, could be mistaken for doc preview | Low | Low | Scope Phase 3 to file/cms document content specifically |

---

## 5. Conclusion

Today oa4rust has real but minimal document-preview capability (plain-text DOCX→HTML only, `file_assemble_control/src/lib.rs:1306`) and a PDF-preview stub that renders nothing (`cms_assemble_control/src/lib.rs:1880`); e-signature exists merely as a personal signature-image store (`crates/personal/src/signature.rs`), with no cryptographic signing, certificate, seal embedding, or verification anywhere in the workspace. Both features must therefore be treated as **integration points**: preview should be backed by an external conversion service (OnlyOffice/KKFileView/LibreOffice) reached over HTTP, and e-signature by a signing/CA service or a PAdES+PKCS#11 library stack, with oa4rust owning only routing, metadata, cache, and the audit trail. The principal risks are fidelity of native Office rendering (R1) and the legal/technical complexity of in-house PAdES+TSA signing (R3), both of which argue strongly for external services rather than building rendering or signing from scratch.
