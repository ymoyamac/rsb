use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

#[proc_macro_derive(Deserialize)]
pub fn derive_csv_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Deserialize solo funciona con structs con campos nombrados"),
            }
        }
        _ => panic!("Deserialize solo funciona con structs"),
    };
    
    let field_names: Vec<_> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    
    let field_types: Vec<_> = fields.iter()
        .map(|f| &f.ty)
        .collect();
    
    let field_parsing = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        let name_str = name.to_string();
        
        let is_string = matches!(ty, Type::Path(tp) if tp.path.segments.last().unwrap().ident == "String");
        
        if is_string {
            quote! {
                #name_str => {
                    #name = Some(value.to_string());
                }
            }
        } else {
            quote! {
                #name_str => {
                    #name = value.parse().ok();
                }
            }
        }
    });
    
    let field_declarations = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            let mut #name: Option<#ty> = None;
        }
    });
    
    let field_construction = field_names.iter().map(|name| {
        quote! {
            #name: #name?
        }
    });
    
    let expanded = quote! {
        impl Read for #name {
            fn read_file(headers: &[&str], values: &[&str]) -> Option<Self> {
                #(#field_declarations)*
                
                for (i, &header) in headers.iter().enumerate() {
                    if let Some(&value) = values.get(i) {
                        match header {
                            #(#field_parsing)*
                            _ => {}
                        }
                    }
                }
                
                Some(Self {
                    #(#field_construction),*
                })
            }
        }
    };
    
    TokenStream::from(expanded)
}