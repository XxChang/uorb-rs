use proc_macro::TokenStream;
use uorb_macro_helper::get_message_hash;

#[proc_macro_attribute]
pub fn orb(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let hash = get_message_hash(item.into());
    TokenStream::new()
}
