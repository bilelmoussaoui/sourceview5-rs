# copied from gtk-rs
GIR = gir/target/bin/gir
GIR_SRC != find gir/src -name '*.rs'
GIR_SRC += gir/Cargo.toml gir/Cargo.lock gir/build.rs
GIR_FILES = gir-files/GtkSource-5.gir

# Run `gir` generating the bindings
gir : sourceview5/src/auto/mod.rs
	cargo fmt

gir-sys : sourceview5-sys/src/lib.rs

doc: $(GIR) $(GIR_FILES)
	$(GIR) -m doc -c sourceview5/Gir.toml

not_bound: $(GIR) $(GIR_FILES)
	$(GIR) -m not_bound -c sourceview5/Gir.toml

regen_check: $(GIR) $(GIR_FILES)
	cp GtkSource-5.gir ./gir-files
	rm sourceview5/src/auto/*
	$(GIR) -c sourceview5/Gir.toml
	cargo fmt
	rm $(GIR_FILES)
	git diff -R --exit-code

sourceview5/src/auto/mod.rs : sourceview5/Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c sourceview5/Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
