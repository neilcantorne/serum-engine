use std::{env::var, path::PathBuf};

#[derive(Debug)]
pub(crate) struct RouteDirectoryArgs {
    pub(super) actix_app: syn::Path,
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
}

impl syn::parse::Parse for RouteDirectoryArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let actix_app =  input.parse::<syn::Path>()?;
        input.parse::<syn::Token!(,)>()?;
        let directory: PathBuf = Self::parse_directory(input.parse()?)?;

        Ok(Self {
            actix_app,
            directory
        })
    }
}