use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::{quote};

mod routes_dir_args;
use routes_dir_args::RouteDirectoryArgs;

#[proc_macro]
pub fn route_directory(tokens: TokenStream) -> TokenStream {
    let args = parse_macro_input!(tokens as RouteDirectoryArgs);

    let mut buffer = args.actix_app.clone();

    for (route_path, physical_path) in args.route_item() {
        let route = quote!{
            .route(#route_path,
                actix_web::web::get()
                    .to(move || async move {
                        const CONTENT : &[u8] = include_bytes!(#physical_path);
                        actix_web::HttpResponse::Ok().body(CONTENT)
                }))
        };

        if route_path != "/index.html" {
            buffer = quote!(#buffer #route)
        }
        else {
            buffer = quote!(#buffer #route .route("/",
            actix_web::web::get()
                .to(move || async move {
                    const CONTENT : &[u8] = include_bytes!(#physical_path);
                    actix_web::HttpResponse::Ok().body(CONTENT)
            })))
        }
    }

    return TokenStream::from(buffer);
}