#!/usr/bin/env python3
"""Generate a minimal create_app body in src/main.rs for diagnosis.

Mode templates (only the body between the fn signature and `Ok(app)`):
  bare   : Router::new()  (no merge, no layers)
  shared : Router::new().merge(shared::router::router())
  layers : shared + the 6 middleware layers
  full   : layers + K crate merges (K from argv[2])

Usage: gen_create_app.py <bare|shared|layers|full> [K]
"""
import sys
import re

MAIN = "src/main.rs"


def read_main():
    with open(MAIN, "r", encoding="utf-8") as f:
        return f.read()


def write_main(s):
    with open(MAIN, "w", encoding="utf-8") as f:
        f.write(s)


def replace_body(src, body_lines):
    # locate create_app fn signature and the `Ok(app)` line; replace everything in between.
    sig_re = re.compile(
        r"pub async fn create_app\(\s*pool: Pool,\s*session_manager: SessionManager,\s*rate_limiter: RateLimiter,\s*\) -> anyhow::Result<Router> \{"
    )
    m = sig_re.search(src)
    if not m:
        raise SystemExit("create_app signature not found")
    start = m.end()
    ok_idx = src.index("Ok(app)", start)
    end = src.index("\n", ok_idx) + 1
    new_block = "\n" + "".join(body_lines) + "\n"
    return src[:start] + new_block + src[end:]


def main():
    mode = sys.argv[1]
    k = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    src = read_main()

    crate_merges = []
    # we need the ordered list of crate merge lines from a full main.rs; read them from a backup?
    # Simpler: import from create_app by scanning original lines. We'll reconstruct from constants.
    # For diagnosis we only need bare/shared/layers; full uses bisect_merge separately.
    if mode == "bare":
        body = [
            "    let app = Router::new();",
        ]
    elif mode == "shared":
        body = [
            "    let app = Router::new()",
            "        .merge(shared::router::router());",
        ]
    elif mode == "layers":
        body = [
            "    let security_state = SecurityState {",
            "        session_manager: session_manager.clone(),",
            "        rate_limiter: rate_limiter.clone(),",
            "        pool: pool.clone(),",
            "    };",
            "    let app = Router::new()",
            "        .merge(shared::router::router())",
            "        .layer(middleware::from_fn_with_state(security_state.clone(), authorize_middleware))",
            "        .layer(middleware::from_fn_with_state(security_state.clone(), auth_middleware))",
            "        .layer(middleware::from_fn_with_state(security_state.clone(), rate_limit_middleware))",
            "        .layer(cors_middleware())",
            "        .layer(middleware::from_fn(security_headers_middleware))",
            "        .layer(middleware::from_fn(trace_middleware));",
        ]
    else:
        raise SystemExit("unknown mode")

    out = replace_body(src, body)
    write_main(out)
    print(f"generated mode={mode}")


if __name__ == "__main__":
    main()
