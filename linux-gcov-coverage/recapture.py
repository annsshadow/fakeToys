#!/usr/bin/env python3
"""Re-capture gcov runtime data WITHOUT rebuilding the kernel.

The kernel (vmlinux + bzImage) is already built in build-qemu. This script
only re-runs the QEMU boot + gcov capture + harvest + report, using the
fixed .gcda-only init-script copy so the capture completes well within the
timeout. Used to recover from a capture-timeout that left valid .gcda on the
9p share but never harvested them.
"""
import os
import sys
import shutil

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import baseline_measurement as b

SOURCE = "."
BUILD = "build-qemu"
SHARE = os.path.join(BUILD, "gcov_share")
TIMEOUT = 2400

print("=== recapture: cleaning stale gcov_share ===")
if os.path.isdir(SHARE):
    shutil.rmtree(SHARE, ignore_errors=True)
# Drop any .gcda already placed in the build tree by a prior (failed) harvest.
for root, dirs, files in os.walk(BUILD):
    if "gcov_share" in root:
        continue
    for fn in files:
        if fn.endswith(".gcda"):
            try:
                os.remove(os.path.join(root, fn))
            except OSError:
                pass

print(f"=== recapture: booting QEMU (timeout {TIMEOUT}s) ===")
ok = b.boot_qemu_collect_gcov(SOURCE, BUILD, timeout=TIMEOUT)
if not ok:
    print("RECAPTURE_FAILED: QEMU boot/capture did not complete in time.")
    sys.exit(2)

print("=== recapture: collecting + classifying coverage ===")
cov = b.collect_and_classify_coverage(SOURCE, BUILD, "gcov")
summ = b.compute_subsystem_summary(cov)
tb = sum(s["total_branches"] for s in summ)
cb = sum(s["covered_branches"] for s in summ)
tl = sum(s["total_lines"] for s in summ)
cl = sum(s["covered_lines"] for s in summ)
b.write_baseline_report(
    os.path.join("tools", "testing", "coverage", "baseline", "baseline_report.json"),
    "gcov", tb, cb, tl, cl, summ)
print(f"RECAPTURE_BRANCH_PCT={cb/tb*100 if tb else 0:.2f}")
print(f"RECAPTURE_LINE_PCT={cl/tl*100 if tl else 0:.2f}")
print(f"RECAPTURE_TOTAL_BRANCHES={tb} COVERED={cb}")
print(f"RECAPTURE_TOTAL_LINES={tl} COVERED={cl}")
print(f"RECAPTURE_SUBSTEMS={len(summ)}")
print("RECAPTURE_DONE")
