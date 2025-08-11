use std::{ collections::HashMap, env, fs, io::Write, path::PathBuf };

use convert_case::{ Case, Casing };
use zip::write::SimpleFileOptions;

macro_rules! log {
    ($msg:expr, $($items:expr),+) => {
        println!("cargo::warning={}", format!($msg, $($items),+));
    };

    ($msg:expr) => {
        println!("cargo::warning={}", $msg);
    };
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    if !manifest_path.join("resources/icons.zip").exists() || !manifest_path.join("resources/icons.json").exists() {
        let icons_path = manifest_path.parent().unwrap().join("node_modules").join("react-icons");

        println!("cargo:rerun-if-changed={}", icons_path.to_str().unwrap());

        if icons_path.is_dir() {
            let re = regex::RegexBuilder
                ::new(
                    r"module\.exports\.([a-zA-Z]*)\s*=\s*function [a-zA-Z]*\s*\(props\)\s*\{\s*return GenIcon\((.*?)\)\(props\);?\};"
                )
                .multi_line(true)
                .build()
                .unwrap();
            let indices = fs
                ::read_dir(icons_path)
                .unwrap()
                .filter_map(|p| {
                    if let Ok(res) = p {
                        if
                            res.path().is_dir() &&
                            !res.file_name().to_str().unwrap().ends_with("lib")
                        {
                            Some(res.path().join("index.js"))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            log!("Found {} iconsets, parsing...", indices.len());

            let _ = fs::create_dir_all(manifest_path.join("resources"));
            let mut found_icons = 0u64;
            let zipfile = fs::File::create(manifest_path.join("resources/icons.zip")).unwrap();
            let mut archive = zip::ZipWriter::new(zipfile);
            let mut icon_map: HashMap<String, Vec<String>> = HashMap::new();

            for index in indices {
                let mut category_icons: Vec<String> = Vec::new();
                let category = index.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
                let js_text = fs::read_to_string(index.clone()).unwrap().replace("\n", "");
                for capture in re.captures_iter(&js_text) {
                    let (_, [name, json]) = capture.extract();
                    let fname = format!("{}.json", name.to_case(Case::Snake));
                    if
                        let Ok(_) = archive.start_file(
                            &fname,
                            SimpleFileOptions::default().compression_method(
                                zip::CompressionMethod::Deflated
                            )
                        )
                    {
                        archive.write_all(json.as_bytes()).unwrap();
                        category_icons.push(name.to_case(Case::Snake));
                        found_icons += 1;
                    }
                }

                let _ = icon_map.insert(category, category_icons);
            }

            archive.finish().unwrap();

            fs::write(manifest_path.join("resources/icons.json"), serde_json::to_string_pretty(&icon_map).unwrap()).unwrap();

            log!("Resolved {} icons in total.", found_icons);
        } else {
            log!("An existing iconset already exists. Remove it if regeneration is needed.");
        }
    }
    tauri_build::build()
}
