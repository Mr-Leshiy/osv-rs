package main

/*
#include "manifest.h"
*/
import "C"
import (
	"bytes"
	"context"
	"fmt"
	"runtime/cgo"
	"unsafe"

	"github.com/google/osv-scalibr/extractor"
	"github.com/google/osv-scalibr/extractor/filesystem"
	"github.com/google/osv-scalibr/extractor/filesystem/language/python/uvlock"
	"github.com/google/osv-scalibr/extractor/filesystem/language/rust/cargolock"
)

func newExtractor(ecosystem string) (filesystem.Extractor, error) {
	switch ecosystem {
	case "Uv":
		return uvlock.New(nil)
	case "Cargo":
		return cargolock.New(nil)
	default:
		return nil, fmt.Errorf("unsupported ecosystem: %q", ecosystem)
	}
}

//export manifest_parse
func manifest_parse(c_ecosystem *C.char, c_data *C.uint8_t, c_len C.size_t, out *C.manifest) *C.char {
	ecosystem := C.GoString(c_ecosystem)
	data := C.GoBytes(unsafe.Pointer(c_data), C.int(c_len))

	ex, err := newExtractor(ecosystem)
	if err != nil {
		*out = 0
		return C.CString(err.Error())
	}
	inv, err := ex.Extract(context.Background(), &filesystem.ScanInput{
		Reader: bytes.NewReader(data),
	})
	if err != nil {
		*out = 0
		return C.CString(err.Error())
	}

	*out = C.manifest(cgo.NewHandle(inv.Packages))
	return C.CString("")
}

//export manifest_packages_len
func manifest_packages_len(list C.manifest, out *C.size_t) *C.char {
	pkgs := cgo.Handle(list).Value().([]*extractor.Package)
	*out = C.size_t(len(pkgs))
	return C.CString("")
}

//export manifest_package_name
func manifest_package_name(list C.manifest, idx C.size_t, out **C.char) *C.char {
	pkgs := cgo.Handle(list).Value().([]*extractor.Package)
	i := int(idx)
	if i >= len(pkgs) {
		return C.CString(fmt.Sprintf("index %d out of range [0, %d)", i, len(pkgs)))
	}
	*out = C.CString(pkgs[i].Name)
	return C.CString("")
}

//export manifest_package_version
func manifest_package_version(list C.manifest, idx C.size_t, out **C.char) *C.char {
	pkgs := cgo.Handle(list).Value().([]*extractor.Package)
	i := int(idx)
	if i >= len(pkgs) {
		return C.CString(fmt.Sprintf("index %d out of range [0, %d)", i, len(pkgs)))
	}
	*out = C.CString(pkgs[i].Version)
	return C.CString("")
}

//export manifest_package_ecosystem
func manifest_package_ecosystem(list C.manifest, idx C.size_t, out **C.char) *C.char {
	pkgs := cgo.Handle(list).Value().([]*extractor.Package)
	i := int(idx)
	if i >= len(pkgs) {
		return C.CString(fmt.Sprintf("index %d out of range [0, %d)", i, len(pkgs)))
	}
	*out = C.CString(pkgs[i].Ecosystem().String())
	return C.CString("")
}

//export manifest_free
func manifest_free(list C.manifest) {
	cgo.Handle(list).Delete()
}
