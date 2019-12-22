build: .build_python .build_c .build_rust

.build_python:
	@cd python && \
	python3 -m venv .env
	@echo 'Python virtualenv created'
	@touch $@

.build_c: $(shell find c -name '*.c')
	@cd c && \
	ls *.c | xargs -L1 basename -s .c | xargs -L1 -I name gcc name.c -o name.out
	@echo 'C binaries built'
	@touch $@

.build_rust: $(shell find rust -name '*.rs')
	@cd rust && \
	cargo build --release
	@echo 'Rust binaries built'
	@touch $@

python: .build_python
c: .build_c
rust: .build_rust

