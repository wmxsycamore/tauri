use config_struct::{create_struct, SerdeSupport, StructOptions};

use serde_json::Value;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

static DEFAULT_CONFIG: &str = r#"
{
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:4000",
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
  "ctx": {},
  "tauri": {
    "embeddedServer": {
      "active": true
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.tauri.dev",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "resources": [],
      "externalBin": [],
      "copyright": "",
      "category": "DeveloperTool",
      "shortDescription": "",
      "longDescription": "",
      "deb": {
        "depends": [],
        "useBootstrapper": false
      },
      "osx": {
        "frameworks": [],
        "minimumSystemVersion": "",
        "useBootstrapper": false
      },
      "exceptionDomain": ""
    },
    "whitelist": {
      "all": true
    },
    "window": {
      "title": "Tauri App",
      "width": 800,
      "height": 600,
      "resizable": true,
      "fullscreen": false
    },
    "security": {
      "csp": "default-src blob: data: filesystem: ws: http: https: 'unsafe-eval' 'unsafe-inline'"
    },
    "edge": {
      "active": true
    },
    "inliner": {
      "active": true
    }
  }
}
"#;

#[cfg(not(feature = "dev-server"))]
pub fn main() {
  let config_string_path = "tauri.conf.json";
  println!("cargo:rerun-if-changed={}", config_string_path);

  let path = Path::new(config_string_path);

  if !path.exists() {
    let mut file = match File::create("tauri.conf.json") {
      Err(e) => panic!("Could not create a default tauri.conf.json file: {}", e),
      Ok(f) => f,
    };

    match file.write_all(DEFAULT_CONFIG.as_bytes()) {
      Err(e) => panic!("Couldn't write to tauri.conf.json: {}", e),
      Ok(_) => (),
    }
  }

  match create_struct(
    "tauri.conf.json",
    "src/config.rs",
    &StructOptions {
      serde_support: SerdeSupport::Yes,
      generate_load_fns: true,
      ..StructOptions::default()
    },
  ) {
    Ok(_) => (),
    Err(e) => panic!("Unable to generate data: {}", e),
  }

  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  let v: Value = serde_json::from_reader(reader).unwrap();

  let pointer: String = v.pointer("/build/distDir").unwrap().to_string();
  let asset_path = Path::new(&pointer);

  let inlined_assets = match std::env::var_os("TAURI_INLINED_ASSETS") {
    Some(assets) => assets
      .into_string()
      .unwrap()
      .split('|')
      .map(|s| s.to_string())
      .collect(),
    None => Vec::new(),
  };

  tauri_includedir_codegen::start("ASSETS")
    .dir(asset_path, tauri_includedir_codegen::Compression::None)
    .build("data.rs", inlined_assets)
    .expect("failed to build data.rs")
}

#[cfg(feature = "dev-server")]
pub fn main() {}
