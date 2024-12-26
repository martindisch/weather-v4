# weather-v4-abandonware

This was supposed to be the the fourth iteration of my simple weather station
using an AM2302 sensor, this time with a Leptos web app. I stopped developing
it after the Wasm build by `cargo leptos watch` in debug mode consumed enough
memory to get OOM killed. Release mode worked, but drove already aggravating
compile times into territory even I found infuriating. Rust compile times and
frontend development just aren't a good fit.

## License

[MIT License](LICENSE)
