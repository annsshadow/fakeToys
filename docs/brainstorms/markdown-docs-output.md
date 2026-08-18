---
date: 2026-07-06
topic: markdown-docs-output
---

# Markdown Output for Kernel Documentation

## Summary

Add a Markdown output target to the kernel documentation build system, generating `.md` files for the entire `Documentation/` tree alongside the existing Sphinx builders. A new `make markdowndocs` target will invoke a Sphinx Markdown builder extension through the existing `Documentation/Makefile` and `conf.py` pipeline, writing output to `Documentation/output/`.

---

## Problem Frame

The kernel's documentation lives in `Documentation/` as reStructuredText and is built through Sphinx into HTML, PDF, EPUB, and other formats. There is no Markdown output path. Consumers who need Markdown — static site generators, chat platforms, LLM ingestion pipelines, or cross-project mirroring — currently must run ad-hoc conversion outside the kernel's verified build system, losing kernel-specific markup and producing inconsistent results. A first-class Markdown builder closes that gap without changing any source files.

---

## Requirements

**Markdown builder integration**
- R1. A new `markdowndocs` target must be added to `Documentation/Makefile` that generates Markdown output for the entire `Documentation/` tree.
- R2. Markdown generation must use a Sphinx Markdown builder extension registered in `Documentation/conf.py`.
- R3. Generated `.md` files must be written to `Documentation/output/` alongside existing builder outputs.
- R4. The new target must support the same `SPHINXDIRS` filtering mechanism as existing targets (e.g., `make SPHINXDIRS=process markdowndocs`).
- R5. The Markdown builder extension and its dependencies must be declared in a Sphinx requirements file under `Documentation/sphinx/`.
- R6. The new target must integrate with the existing `sphinx-build-wrapper` and version-check flow used by other doc targets.

**Output quality and compatibility**
- R7. Kernel custom Sphinx extensions must be evaluated for Markdown builder compatibility; incompatible extensions must be handled gracefully rather than crashing the build.
- R8. Markdown output must preserve document structure: headings, code blocks, lists, tables, and cross-references where the builder supports them.
- R9. The solution must not modify or remove any existing builder targets or output formats.

---

## Success Criteria

- `make markdowndocs` completes successfully and produces `.md` files under `Documentation/output/`.
- Output includes representative files from `process/`, `admin-guide/`, `core-api/`, and `driver-api/`.
- `make markdowndocs SPHINXDIRS=process` filters to the specified subtree, matching existing target behavior.
- A CI step can run `make markdowndocs` to validate that the documentation tree continues to build in Markdown.
- `make htmldocs`, `make pdfdocs`, and all other existing targets produce identical output before and after the change.

---

## Scope Boundaries

- Does not migrate `.rst` source files to Markdown; sources remain unchanged.
- Does not modify kernel source code outside `Documentation/`.
- Does not replace or deprecate any existing builder target.
- Does not add a new top-level Makefile target; the entry point is `Documentation/Makefile` only.

---

## Key Decisions

- **Approach: Sphinx Markdown Builder extension.** Selected over Pandoc post-processing (loses kernel custom directives) and a custom docutils writer (disproportionate implementation cost). The builder integrates into the existing Sphinx pipeline and can be evaluated against kernel extensions incrementally.
- **Coverage: entire `Documentation/` tree.** Partial subsets were considered but rejected; the Markdown output should be as complete as the other builders so consumers can rely on it as a first-class artifact.

---

## Dependencies / Assumptions

- A Sphinx Markdown builder extension exists that can be installed via pip and is compatible with or patchable for Sphinx 3.4.3 (the version pinned in `Documentation/sphinx/min_requirements.txt`).
- Kernel custom Sphinx extensions (`automarkup`, `kerneldoc`, `kernel_abi`, `kernel_feat`, `kfigure`, etc.) can be made compatible with the Markdown builder or gracefully degraded without blocking the build.
- The extension's output quality for kernel-specific constructs (kernel-doc function blocks, ABI tables, YAML netlink specs) is acceptable or improvable without forking the extension.
