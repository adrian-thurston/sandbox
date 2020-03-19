NOTES:

1. This is in the style of `autoconf`, but it is not `autoconf`.

2. You can alter either `configure` or `Makefile.in` and on the next `make` the
right things get remade.

3. You can alter the Rust code and the Go bridge package is rebuilt, leading to
a correct incremental build.

4. You must do an initial `./configure && make` but after that you can just use
`go build` if all you touch are Go files.

5. This example `configure` script  has one option `--release` that is passed
through as an option to `cargo`.

6. Options can be extended to receive and pass through any kind of build
options. For example, for controlling static linking, cross-compiling,
optimization etc.

7. Once you `./configure --with-some-options` the options are recorded and you
can consult them. They persist until you run `./configure` again with new
options, or until you `make distclean` (unimplemented ATM)

8. Options can be temporarily overridden on the `make` command line, for
example `make CARGO_FLAGS=--something`.

9. The `configure` script can be extended to do anything we want really, so
long as it is portable can be described as “inspecting the system” or
“preparing the software to build given these build/system parameters”

