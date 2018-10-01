build:
	cargo build --release
	rm -rf releases
	mkdir -p releases
	tar -czf releases/kubesecrets-0.1.0.tar.gz --directory=target/release kubesecrets
	shasum -a 256 releases/kubesecrets-0.1.0.tar.gz > releases/kubesecrets-0.1.0.tar.gz.shasum
