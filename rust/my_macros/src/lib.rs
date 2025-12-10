use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn entity_ready(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // 2. Extract parts of the function to reuse
    let fn_name = &input_fn.sig.ident;
    if fn_name.to_string() != "ready".to_string() {
        return Error::new_spanned(
            &fn_name,
            "attribute macro can only be applied to godot 'ready' functions"
        ).to_compile_error().into()
    }
    let fn_args = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis; // Visibility (pub, etc.)
    let fn_sig = &input_fn.sig; // The full signature

    let qt = quote! {
        #fn_vis fn #fn_name(#fn_args) #fn_output {
             #fn_block

            let s = self.to_gd().clone();
            self.world.init(s.get_owner().unwrap().cast());
            self.world.bind_mut().register_entity(s);

        }
    };

    TokenStream::from(qt)
}

