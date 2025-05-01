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

    let values: Vec<String> = keys
        .iter()
        .map(|x| x.to_string())
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
                        
                        json_str.push_str(&format!("{}: {:?},",stringify!(#values),&self.#keys));
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

    let values: Vec<String> = keys
        .iter()
        .map(|x| x.to_string())
        .collect();

    let token_stream =
        quote! {
        impl #name {
           fn deserialize(&self,json_data:String)->Self {
            let json_data = json_data.trim();
            let trimed = json_data.trim_matches(|c| c == '{' || c == '}');

            for pair in trimed.split(','){
                let parts :Vec<&str> = pair.splitn(2,':').collect();

            }
           }
        }
    };
    TokenStream::from(token_stream)
}
