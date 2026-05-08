package main

import (
	"runtime"
	"runtime/debug"
)

// Disable the concurrent GC when this library is loaded as a C archive.
//
// The Go runtime's background GC goroutines run on separate OS threads and
// sweep heap spans concurrently. When a CGo function is called from a
// non-Go (e.g. Rust) OS thread the GC stack scanner cannot see the non-Go
// frames on that thread's stack and corrupts sweep-generation state, which
// manifests as "fatal error: bad sweepgen in refill".
//
// Disabling automatic GC and invoking it explicitly after each exported call
// eliminates the concurrent-sweep race.  For a short-lived parsing library
// this is the correct trade-off: memory per call is small and bounded.
func init() {
	debug.SetGCPercent(-1) // disable automatic GC
	runtime.GOMAXPROCS(1)  // single Go thread — no concurrent background workers
}

// main is required by `-buildmode=c-shared` and `-buildmode=c-archive`.
func main() {}
