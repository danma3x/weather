build:
	cargo fmt --all -- --check && cargo clippy -- -D warnings && cargo build

hurl_save:
	hurl hurl/accuweather.hurl --very-verbose --output ./hurl_responses/accuweather.json