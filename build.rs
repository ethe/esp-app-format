use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("REBUILD_ESP_APP_FORMAT").is_err() {
        return;
    }

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_args(&[
            "-Iinclude",
            "-Iinclude/esp-idf/components/esp_common/include",
            "-Iinclude/esp-idf/components/bootloader_support/include/",
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
