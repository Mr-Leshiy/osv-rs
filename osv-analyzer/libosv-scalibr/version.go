package main

/*
#include "version.h"
*/
import "C"
import (
	"runtime"
	"runtime/cgo"

	"github.com/google/osv-scalibr/semantic"
)

//export version_new
func version_new(c_str *C.char, c_ecosystem *C.char, out *C.version) *C.char {
	v, err := semantic.Parse(C.GoString(c_str), C.GoString(c_ecosystem))
	if err != nil {
		*out = 0
		return C.CString(err.Error())
	}
	*out = C.version(cgo.NewHandle(v))
	return C.CString("")
}

//export version_cmp
func version_cmp(a C.version, b C.version, result *C.int) *C.char {
	va := cgo.Handle(a).Value().(semantic.Version)
	vb := cgo.Handle(b).Value().(semantic.Version)
	cmp, err := va.Compare(vb)
	if err != nil {
		*result = 0
		return C.CString(err.Error())
	}
	*result = C.int(cmp)
	return C.CString("")
}

//export version_free
func version_free(ref C.version) {
	cgo.Handle(ref).Delete()
	runtime.GC()
}
