clean:
	rm -fr idx
	rm -fr target

compile: target/release/build_index target/release/do_query

index: idx

serve: target/release/do_query
	@target/release/do_query idx

target/release/%:
	@echo "\n\n\n--- Building seekstorm's binary ---" >&2
	@RUSTFLAGS='-C target-cpu=native' cargo build --release --bin $(notdir $@)

idx: target/release/build_index
	@echo "\n\n\n---- Indexing seekstorm ----" >&2
	@mkdir -p idx
	export RUST_LOG=info && target/release/build_index "$@" < ${CORPUS} >&2
