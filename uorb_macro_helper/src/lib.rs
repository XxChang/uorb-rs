use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed};

fn hash_32_fnv1a(data: &str) -> u32 {
    let mut hash_val: u32 = 0x811c9dc5;
    let prime: u32 = 0x1000193;

    for byte in data.bytes() {
        hash_val ^= byte as u32;
        hash_val = hash_val.wrapping_mul(prime);
    }

    hash_val
}

pub fn get_message_hash(item: TokenStream) -> u32 {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    match ast.data {
        syn::Data::Struct(
            DataStruct {
                fields: Fields::Named(
                    FieldsNamed {
                        ref named,
                        ..
                    }
                ),
                ..
            }
        ) => {
            let all_field_str = named.iter().fold(String::default(), |acc, ref field| {
                let name = &field.ident;
                let typ = &field.ty;
                let r = quote! { #typ #name};
                acc + &r.to_string() + "\n"
            });

            hash_32_fnv1a(&all_field_str)
        }
        _ => unimplemented!("Only struct named fields is supported"),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_message_hash() {
        let input = quote! {
            struct GpioIn {
                timestamp: u64,
                device_id: u32,
                state: u32,
            }
        };

        let result = get_message_hash(input);
        assert_eq!(result, 791998605u32);
    }
}

