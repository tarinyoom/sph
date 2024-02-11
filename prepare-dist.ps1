if (!(Test-Path './out/assets')) {
	New-Item -ItemType Directory -Path './out/assets' -Force
}

Copy-Item ./public/index.html ./out/
wasm-bindgen --out-dir ./out/ --no-typescript --target web ./target/wasm32-unknown-unknown/release/browser-sph.wasm
