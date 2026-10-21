// Command tygen generates the admin client's TypeScript API types from the
// Go response types.
//
// Usage:
//
//	go run ./cmd/tygen            # write admin/src/api/types.gen.ts
//	go run ./cmd/tygen -check     # fail if the checked-in file is stale
//
// The `-check` mode is what CI runs: it makes "someone changed a Go struct
// and forgot to regenerate" a **red build** instead of a silent drift.
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"

	"github.com/laiyipao/server/internal/apitypes"
	"github.com/laiyipao/server/internal/tygen"
)

func main() {
	check := flag.Bool("check", false, "verify the checked-in file is up to date instead of writing it")
	out := flag.String("out", "", "output path (default: <repo>/laiyipao/admin/src/api/types.gen.ts)")
	flag.Parse()

	paths, err := tygen.FindRepoPaths("")
	if err != nil {
		fmt.Fprintf(os.Stderr, "tygen: %v\n", err)
		os.Exit(1)
	}
	target := paths.AdminTypes
	if *out != "" {
		target = *out
	}

	want, err := apitypes.AdminRegistry.Generate(apitypes.Header)
	if err != nil {
		fmt.Fprintf(os.Stderr, "tygen: %v\n", err)
		os.Exit(1)
	}

	if *check {
		got, err := os.ReadFile(target)
		if err != nil {
			fmt.Fprintf(os.Stderr, "tygen: cannot read %s: %v\n", target, err)
			os.Exit(1)
		}
		if string(got) != want {
			fmt.Fprintf(os.Stderr,
				"tygen: %s is STALE.\n"+
					"  The Go response types changed but the generated TS was not regenerated.\n"+
					"  Run: cd laiyipao/server && go run ./cmd/tygen\n", target)
			os.Exit(1)
		}
		fmt.Printf("tygen: %s is up to date\n", target)
		return
	}

	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		fmt.Fprintf(os.Stderr, "tygen: %v\n", err)
		os.Exit(1)
	}
	if err := os.WriteFile(target, []byte(want), 0o644); err != nil {
		fmt.Fprintf(os.Stderr, "tygen: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("tygen: wrote %s (%d bytes)\n", target, len(want))
}
