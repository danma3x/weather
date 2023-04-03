build:
	cargo fmt --all -- --check && cargo clippy -- -D warnings && cargo build
	
nightly_fmt:
	cargo +nightly fmt

hurl_accuweather:
	hurl hurl/accuweather.hurl --very-verbose --json > ./hurl_responses/accuweather.log 2>&1
hurl_aerisweather:
	hurl hurl/aerisweather.hurl --very-verbose --json > ./hurl_responses/aerisweather.log 2>&1
hurl_openweathermap:
	hurl hurl/openweathermap.hurl --very-verbose --json > ./hurl_responses/openweathermap.log 2>&1
hurl_weatherapi:
	hurl hurl/weatherapi.hurl --very-verbose --json > ./hurl_responses/weatherapi.log 2>&1