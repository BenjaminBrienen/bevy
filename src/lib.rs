#![cfg_attr(docsrs, feature(doc_cfg))]
#![expect(
    clippy::doc_markdown,
    reason = "Clippy lints for un-backticked identifiers within the cargo features list, which we don't want."
)]
//! [![Bevy Logo](https://bevy.org/assets/bevy_logo_docs.svg)](https://bevy.org)
//!
//! Bevy is an open-source, modular game engine built in Rust, with a focus on developer productivity
//! and performance.
//!
//! Check out the [Bevy website](https://bevy.org) for more information, read the
//! [Quick Start Guide](https://bevy.org/learn/quick-start/introduction) for a step-by-step introduction, and [engage with our
//! community](https://bevy.org/community/) if you have any questions or ideas!
//!
//! ## Example
//!
//! Here is a simple "Hello, World!" Bevy app:
//! ```
//! use bevy::prelude::*;
//!
//! fn main() {
//!    App::new()
//!        .add_systems(Update, hello_world_system)
//!        .run();
//! }
//!
//! fn hello_world_system() {
//!    println!("hello world");
//! }
//! ```
//!
//! Don't let the simplicity of the example above fool you. Bevy is a [fully featured game engine](https://bevy.org),
//! and it gets more powerful every day!
//!
//! ## This Crate
//!
//! The `bevy` crate is a container crate that makes it easier to consume Bevy subcrates.
//! The defaults provide a "full engine" experience, but you can easily enable or disable features
//! in your project's `Cargo.toml` to meet your specific needs. See Bevy's `Cargo.toml` for a full
//! list of available features.
//!
//! If you prefer, you can also use the individual Bevy crates directly.
//! Each module in the root of this crate, except for the prelude, can be found on crates.io
//! with `bevy_` appended to the front, e.g., `app` -> [`bevy_app`](https://docs.rs/bevy_app/*/bevy_app/).
#![doc = include_str!("../docs/cargo_features.md")]
#![doc(
    html_logo_url = "https://bevy.org/assets/icon.png",
    html_favicon_url = "https://bevy.org/assets/icon.png"
)]
#![no_std]

pub use bevy_internal::*;

// Wasm does not support dynamic linking.
#[cfg(all(feature = "dynamic_linking", not(target_family = "wasm")))]
#[expect(
    unused_imports,
    clippy::single_component_path_imports,
    reason = "This causes Bevy to be compiled as a dylib when using dynamic linking and therefore cannot be removed or changed without affecting dynamic linking."
)]
use bevy_dylib;

#[cfg(feature = "std")]
extern crate std;

// Load the crate’s rustdoc JSON before running this test
// ```bash
// cargo +nightly rustdoc -- --document-private-items
// ```
#[test]
fn ensure_unique_public_paths() {
    #[cfg(feature = "std")]
    {
        use std::collections::HashMap;
        use std::eprint;
        use std::format;
        use std::fs;
        use std::io::Write;
        use std::path::Path;
        use std::string::ToString;
        use std::{string::String, vec::Vec};

        let doc_root = "target/doc/bevy";

        let mut items: HashMap<String, Vec<String>> = HashMap::new();

        fn visit_dir(dir: &Path, items: &mut HashMap<String, Vec<String>>) {
            if dir.is_dir() {
                for entry in fs::read_dir(dir).expect("Failed to read directory") {
                    let entry = entry.expect("Failed to read entry");
                    let path = entry.path();

                    if path.is_dir() {
                        visit_dir(&path, items);
                        continue;
                    }

                    let Some(ext) = path.extension() else {
                        continue;
                    };
                    if ext != "html" {
                        continue;
                    }

                    let file_name = path
                        .file_name()
                        .expect("path has a file name")
                        .to_str()
                        .expect("file name is not valid UTF-8");

                    let parts: Vec<&str> = file_name.split('.').collect();
                    if parts.len() != 3 {
                        // Skip files that don't follow type.name.html format
                        continue;
                    }

                    let item_type = parts[0]; // struct, enum, fn, etc.
                    let item_name = parts[1]; // actual name

                    // Compute Rust module path
                    let mut components: Vec<String> = path
                        .strip_prefix("target/doc")
                        .unwrap()
                        .components()
                        .map(|c| c.as_os_str().to_string_lossy().to_string())
                        .collect();
                    components.pop(); // remove file name
                    let rust_path = format!("{}::{}", components.join("::"), item_name);

                    // Skip items in bevy::prelude or bevy::<crate>::prelude
                    if rust_path.starts_with("bevy::prelude")
                        || rust_path.starts_with("bevy::") && rust_path.contains("::prelude::")
                    {
                        continue;
                    }

                    // Use "type.name" as the key
                    items
                        .entry(format!("{}.{}", item_type, item_name))
                        .or_default()
                        .push(rust_path);
                }
            }
        }

        visit_dir(Path::new(doc_root), &mut items);

        // Collect duplicates
        let duplicates: Vec<_> = items
            .iter()
            .filter(|(_, paths)| paths.len() > 1)
            .map(|(item, paths)| (item, paths.clone()))
            .collect();

        if !duplicates.is_empty() {
            let mut buffer = String::new();
            buffer.push_str("Found duplicate rustdoc items:\n");

            for (item, paths) in duplicates {
                let parts: Vec<&str> = item.split('.').collect();
                let item_type = parts[0];
                let item_name = parts[1];
                buffer.push_str(&format!("{} {}\n", item_type, item_name));
                for path in paths {
                    buffer.push_str(&format!("    {}\n", path));
                }
            }

            eprint!("{buffer}");
            let mut file =
                fs::File::create("duplicates.txt").expect("Failed to create duplicates.txt");
            file.write_all(buffer.as_bytes())
                .expect("Failed to write to duplicates.txt");
            panic!("Rustdoc output contains duplicate items!");
        }
    }
}
