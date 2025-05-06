use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, Data, DeriveInput, Fields, Ident, Type };

#[proc_macro_derive(Serialize)]
pub fn serialize_json_driver(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name: &Ident = &ast.ident;

    let fields = if let Data::Struct(field_struct) = &ast.data {
        if let Fields::Named(field_name) = &field_struct.fields {
            &field_name.named
        } else {
            return (
                quote! {
                compile_error!("Only struct suopported")
            }
            ).into();
        }
    } else {
        return (quote! {
            compile_error!("Only struct suopported")
        }).into();
    };

    let keys: Vec<&Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let token_stream =
        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let mut ds = f.debug_struct("");
                    #(
                        ds.field(&format!("{:?}",stringify!(#keys)), &self.#keys);
                    )*
                    ds.finish()
                }
            }

            impl #name {
                pub fn serialize(&self) -> String {
                    let mut json_str = String::new();
                    json_str.push('{');
                    #(
                        
                        json_str.push_str(&format!("{:?}: {:?},",stringify!(#keys),&self.#keys));
                    )*
                    if json_str.len() < 3 {
                        json_str.pop();
                        return json_str;
                    }
                    json_str.pop();
                    json_str.push('}');
                    json_str
                }
            }
        };

    TokenStream::from(token_stream)
}

#[proc_macro_derive(Deserialize)]
pub fn deserialize_json_driver(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = &ast.ident;

    let fields = if let Data::Struct(field_struct) = &ast.data {
        if let Fields::Named(field_name) = &field_struct.fields {
            &field_name.named
        } else {
            return (
                quote! {
                compile_error!("Only named fields in structs are supported");
            }
            ).into();
        }
    } else {
        return (quote! {
            compile_error!("Only structs are supported");
        }).into();
    };

    let mut field_decls = Vec::new();
    let mut field_matches = Vec::new();
    let mut field_builders = Vec::new();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_str = field_ident.to_string();
        let field_type = &field.ty;

        field_decls.push(
            quote! {
            let mut #field_ident: Option<#field_type> = None;
        }
        );

        if let Some(inner_type) = extract_vec_inner_type(field_type) {
            field_matches.push(
                quote! {
                #field_str => {
                    let items: Result<Vec<#inner_type>, _> = value
                        .trim_matches(|c| c == '[' || c == ']')
                        .split('|')
                        .map(|v|  v.trim().trim_matches(|c| c == '"' || c == '\'').parse::<#inner_type>())
                        .collect();
                    #field_ident = Some(items.map_err(|e| format!("Failed to parse Vec field '{}': {}", #field_str, e))?);
                }
            }
            );
        } else {
            field_matches.push(
                quote! {
                    #field_str => {
                        #field_ident = Some(value.parse::<#field_type>().map_err(|e| format!("Failed to parse field '{}': {}", #field_str, e))?);
                    }
                }
            );
        }

        field_builders.push(
            quote! {
            #field_ident: #field_ident.ok_or_else(|| format!("Missing field: {}", stringify!(#field_ident)))?
        }
        );
    }

    let token_stream =
        quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #(#field_decls)*

                let mut is_brackets = false;
                let mut new_s = String::new();

                for c in s.chars(){
                    match c {
                        '[' => {
                            is_brackets = true;
                            new_s.push(c);
                        },
                        ']' => {
                            is_brackets = false;
                            new_s.push(c);
                        },
                        ',' if is_brackets => {
                            new_s.push('|');
                        },
                        _ => {
                            new_s.push(c);
                        }
                    }
                }

                for pair in new_s.trim_matches(|c| c == '{' || c == '}').split(",") {
                    let mut kv = pair.splitn(2, ':');
                    let key = kv.next().ok_or("Missing key")?.trim().trim_matches('"');
                    let value = kv.next().ok_or("Missing value")?.trim().trim_matches('"');

                    match key {
                        #(#field_matches),*,
                        _ => return Err(format!("Unknown field: {}", key)),
                    }
                }

                Ok(#name {
                    #(#field_builders),*
                })
            }
        }
    };

    TokenStream::from(token_stream)
}

fn extract_vec_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner);
                    }
                }
            }
        }
    }
    None
}
