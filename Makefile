build: .build_python .build_c .build_rust .build_erlang

.build_python:
	@cd python && \
	python3 -m venv .env
	@echo 'Python virtualenv created'
	@touch $@

.build_c: $(shell find c -name '*.c')
	@cd c && \
	ls *.c | xargs -L1 basename -s .c | xargs -L1 -I name gcc name.c -o name.out -lm
	@echo 'C binaries built'
	@touch $@

.build_rust: $(shell find rust -name '*.rs')
	@cd rust && \
	cargo build --release
	@echo 'Rust binaries built'
	@touch $@

.build_erlang: $(shell find erlang -name '*.erl')
	@cd erlang && \
	erlc *.erl
	@echo 'Erlang beam files build'
	@touch $@

python: .build_python
c: .build_c
rust: .build_rust
erlang: .build_erlang
