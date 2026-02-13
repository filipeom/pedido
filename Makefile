default: build

install:
	cargo install wasm-pack

build:
	wasm-pack build --target web --out-dir site/pkg

clean:
	cargo clean
	rm -rf site/pkg
