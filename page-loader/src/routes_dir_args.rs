use std::{env::var, path::PathBuf};

use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub(crate) struct RouteDirectoryArgs {
    pub(super) actix_app: TokenStream,
    pub(super) directory: PathBuf
}

impl RouteDirectoryArgs {
    fn parse_directory(dir_str: syn::LitStr) -> Result<PathBuf, syn::Error> {
        let path = PathBuf::from(dir_str.value().replace('/', "\\"));

        let full_path = PathBuf::from(
            var("CARGO_MANIFEST_DIR")
            .unwrap())
            .join(path);
        
        if full_path.is_dir() {
            return Ok(full_path);
        }

        Err(syn::Error::new_spanned(dir_str, "Directory not found"))
    }

    pub(crate) fn route_item(&self) -> RouteItems {
        let dir_iter = self.directory
            .read_dir()
            .expect("Unable to read directory");

            RouteItems {
                parent: self.directory.clone(),
                inner: dir_iter
            }    
    }
}

impl syn::parse::Parse for RouteDirectoryArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let actix_app =  input.parse::<syn::Ident>()?.to_token_stream();
        input.parse::<syn::Token!(,)>()?;

        let directory: PathBuf = Self::parse_directory(input.parse()?)?;

        Ok(Self {
            actix_app,
            directory
        })
    }
}

pub (crate) struct RouteItems {
    pub(self) parent: PathBuf,
    pub(self) inner: std::fs::ReadDir
} 

impl RouteItems {
    fn next_skip_maps(&mut self) -> Option<std::fs::DirEntry> {
        while let Some(result) = self.inner.next() {
            let entry = result.expect("Failed to file in directory");

            if !entry.path().ends_with("map") {
                return Some(entry);
            }
        }

        None
    }
}

impl Iterator for RouteItems  {
    type Item =  (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.next_skip_maps() {
            if let Ok(rel_path)  = entry.path()
            .strip_prefix(self.parent.as_path()) {
                return Some((
                    format!("/{}", rel_path.to_str().unwrap()),
                    entry.path()
                            .to_str()
                            .unwrap()
                            .to_owned()
                ));
            }
        }

        None
    }
}
