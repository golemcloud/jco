use std::path::PathBuf;

use js_component_bindgen::ts_stubgen::ts_stubgen;

static IS_DEBUG: bool = true;

#[test]
fn test_ts_stubgen() {
    let mut resolve = js_component_bindgen::source::wit_parser::Resolve::default();
    let path = PathBuf::from("tests/wit/")
        .canonicalize()
        .expect("Valid Path");
    resolve.push_path(&path).expect("Valid WIT");

    // println!(
    //     "{:#?}",
    //     resolve
    //         .worlds
    //         .iter()
    //         .map(|(_, w)| w.name.as_str())
    //         .collect::<Vec<_>>()
    // );

    let id = resolve
        .worlds
        .iter()
        .find(|(_, w)| w.name == "caller")
        .expect("caller world exists")
        .0;

    let mut files = js_component_bindgen::files::Files::default();
    ts_stubgen(&resolve, id, &mut files);

    // files.iter().for_each(|(name, data)| {
    //     let name = name.to_string();
    //     let data = String::from_utf8(data.to_vec()).unwrap();
    //     println!("{}:\n{}\n\n", name, data)
    // });

    // Write output to files
    if IS_DEBUG {
        let prefix = PathBuf::from("tests/temp/");

        // delete all files in the directory

        std::fs::remove_dir_all(&prefix).ok();

        files.iter().for_each(|(name, data)| {
            let name = name.to_string();
            let data = String::from_utf8(data.to_vec()).unwrap();
            let path = prefix.join(name);

            // Create the parent directory if it doesn't exist
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("Create parent directory");
            }

            std::fs::write(path, data).expect("Write file");
        });
    }
}
