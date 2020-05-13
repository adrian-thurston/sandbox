package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"bytes"

	"golang.org/x/tools/go/packages"
	"go/types"
)

func showFluxUsage(args []string) error {
	cfg := &packages.Config{
		Mode: packages.NeedName |
			packages.NeedFiles |
			packages.NeedCompiledGoFiles |
			packages.NeedImports |
			packages.NeedDeps |
			packages.NeedExportsFile |
			packages.NeedTypes |
			packages.NeedSyntax |
			packages.NeedTypesInfo |
			packages.NeedTypesSizes,
		Tests: false,
		Dir:   "",
	}

	initial, err := packages.Load(cfg, args...)
	if err != nil {
		return err
	}
	if packages.PrintErrors(initial) > 0 {
		return fmt.Errorf("packages contain errors")
	}

	structs := make([]types.Object, 0)
	interfaces := make([]types.Object, 0)

	// Collect interfaces and structs
	visit := func(pkg *packages.Package) {
		typs := pkg.Types
		for _, name := range typs.Scope().Names() {
			object := typs.Scope().Lookup(name)
			if _, ok := object.(*types.TypeName); !ok {
				continue
			}

			if ! strings.HasPrefix( object.Pkg().Path(), "github.com/influxdata/" ) {
				continue
			}

			typ := object.Type()
			named, ok := typ.(*types.Named)
			if !ok {
				continue
			}

			// The named thing has an underlying type.
			underlying := named.Underlying()

			// Collect the structs.
			if _, ok := underlying.(*types.Struct); ok {
				structs = append(structs, object)
			}

			// Collect the interfaces.
			if _, ok := underlying.(*types.Interface); ok {
				interfaces = append(interfaces, object)
			}
		}
	}
	packages.Visit( initial, nil, visit )

	for _, ifobj := range interfaces {
		// Take apart the interface.
		typ := ifobj.Type();
		niface, _ := (typ).(*types.Named);
		iface, _ := niface.Underlying().(*types.Interface);

		println( "=====", ifobj.Pkg().Path(), ifobj.Name() )

		for i := 0; i < iface.NumMethods(); i++ {
			funcc := iface.Method(i)
			var buf bytes.Buffer
            types.WriteSignature(&buf, funcc.Type().(*types.Signature), nil)
			println("       ", funcc.Name(), buf.String() )
		}

		for _, stobj := range structs {
			if ( types.Implements( types.NewPointer(stobj.Type()), iface ) ) {
				println( "   ", stobj.Pkg().Path(), stobj.Name(), )
			}
		}
		println()
	}

	return nil
}

func main() {
	flag.Parse()

	if err := showFluxUsage(flag.Args()); err != nil {
		fmt.Fprintf(os.Stderr, "error: %s\n", err)
		os.Exit(1)
	}
}
