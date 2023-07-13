wget https://github.com/harfbuzz/harfbuzz/releases/download/8.0.1/harfbuzz-8.0.1.tar.xz
rm -rf harfbuzz
tar xvf harfbuzz-8.0.1.tar.xz
rm harfbuzz-8.0.1.tar.xz
mv harfbuzz-8.0.1 harfbuzz
bindgen --no-prepend-enum-name --whitelist-function hb_.\* --whitelist-type hb_.\* wrapper.h | sed 's/ f32/ ::std::os::raw::c_float/' | sed 's/u32_/u32/' | sed 's/i32_/i32/' | sed 's/u16_/u16/' | sed 's/i16_/i16/' | sed 's/u8_/u8/' | sed 's/i8_/i8/' >src/bindings.rs
