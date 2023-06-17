extern crate darling;
extern crate proc_macro;

use darling::{export::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::Path;

#[derive(Default, Debug, FromMeta)]
struct LookupOverride {
    #[darling(default)]
    lookup_method: Option<Path>,
    #[darling(default)]
    batch_method: Option<Path>,
}

#[derive(Debug, FromMeta, Default)]
struct LookupStyle {
    #[darling(default)]
    batch: bool,
    #[darling(default)]
    lookup: bool,
    #[darling(default)]
    overrides: LookupOverride,
}

impl LookupStyle {
    fn get_lookup_method(&self) -> Path {
        match self.overrides.lookup_method.clone() {
            Some(path) => path,
            None => Path::from_string("Method::GET").unwrap(),
        }
    }

    fn get_batch_method(&self) -> Path {
        match self.overrides.batch_method.clone() {
            Some(path) => path,
            None => Path::from_string("Method::POST").unwrap(),
        }
    }
}

#[derive(Debug, FromMeta, Default)]
struct ResultHandler {
    #[darling(default)]
    lookup: bool,
    #[darling(default)]
    batch: bool,
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    default_url: String,
    api_path: String,
    lookup_style: LookupStyle,
    lookup_type: Path,
    result_type: Path,
    #[darling(default)]
    multi_result_type: Option<Path>,
    #[darling(default)]
    custom_send: bool,
    #[darling(default)]
    result_handler: ResultHandler,
}

#[proc_macro_attribute]
pub fn smarty_api(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };

    let mut ast = syn::parse(input).unwrap();

    let args = match MacroArgs::from_list(attr_args.as_ref()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    impl_smarty_api_macro(&args, &mut ast)
}

fn impl_smarty_api_macro(attrs: &MacroArgs, ast: &mut syn::DeriveInput) -> TokenStream {
    // This is the identifier of the struct that this is being implemented on.
    let name = &ast.ident;

    let default_url = attrs.default_url.clone();
    let api_path = attrs.api_path.clone();

    let lookup_type = attrs.lookup_type.clone();
    let result_type = attrs.result_type.clone();
    let multi_result_type = match attrs.multi_result_type.clone() {
        Some(r) => r,
        None => result_type.clone(),
    };

    // Lets make sure that the API Type has the values it needs.
    let mut result = quote! {
        pub struct #name {
            pub(crate) client: Client
        }

        impl #name {
            pub fn new(options: Options) -> Result<Self, ParseError> {
                Self::new_custom_base_url(#default_url.parse()?, options)
            }

            pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
                Ok(Self {client: Client::new(base_url, options, #api_path)?})
            }
        }

    };

    let lookup_handler = match attrs.result_handler.lookup {
        true => {
            quote! { self.handle_lookup_results(lookup, results); }
        }
        false => {
            quote! {
                lookup.results = candidates;
            }
        }
    };

    let lookup_method = attrs.lookup_style.get_lookup_method();

    if !attrs.custom_send {
        if attrs.lookup_style.batch && attrs.lookup_style.lookup {
            unimplemented!("Only one between batch and lookup allowed for lookup style");
        } else if attrs.lookup_style.batch {
            let batch_handler = match attrs.result_handler.batch {
                true => {
                    quote! { self.handle_batch_results(batch, results); }
                }
                false => {
                    quote! {
                        let records = batch.records_mut();
                        for i in 0..records.len() {
                            records[i].results = results[i].clone();
                        }
                    }
                }
            };

            let batch_method = attrs.lookup_style.get_batch_method();

            result = quote! {
                #result

                impl #name {
                    async fn send_lookup(&self, lookup: &mut #lookup_type) -> Result<(), SDKError> {
                        let mut req = self
                            .client
                            .reqwest_client
                            .request(#lookup_method, self.client.url.clone());
                        req = self.client.build_request(req);
                        req = req.query(&lookup.clone().into_param_array());

                        let candidates = send_request::<#result_type>(req).await?;

                        #lookup_handler

                        Ok(())
                    }
                    pub async fn send(&self, batch: &mut Batch<#lookup_type>) -> Result<(), SDKError> {
                        if batch.is_empty() {
                            return Ok(())
                        }

                        if batch.length() == 1 {
                            return self.send_lookup(&mut batch.records_mut()[0]).await
                        }

                        let mut req = self.client.reqwest_client.request(#batch_method, self.client.url.clone());
                        req = self.client.build_request(req);
                        req = req.json(batch.records());
                        req = req.header("Content-Type", "application/json");

                        let results: #multi_result_type = send_request(req).await?;

                        #batch_handler

                        Ok(())
                    }
                }
            }
        } else if attrs.lookup_style.lookup {
            result = quote! {
                #result

                impl #name {
                    pub async fn send(&self, lookup: &mut #lookup_type) -> Result<(), SDKError> {
                        let mut req = self
                            .client
                            .reqwest_client
                            .request(#lookup_method, self.client.url.clone());
                        req = self.client.build_request(req);
                        req = req.query(&lookup.clone().into_param_array());

                        let candidates = send_request::<#result_type>(req).await?;

                        #lookup_handler

                        Ok(())
                    }
                }
            }
        } else {
            unimplemented!("Add batch or lookup to lookup style");
        }
    }

    result.into()
}
