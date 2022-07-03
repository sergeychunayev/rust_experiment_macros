use proc_macro::TokenStream;

use quote::ToTokens;
use syn::{ItemFn, parse_macro_input};
use uuid::Uuid;

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let replaced_fn_name = format!("___x_{}", Uuid::new_v4()).replace("-", "_");
    let original_fn_def = item.to_string();
    let item_clone = item.clone();
    let item_fn = parse_macro_input!(item_clone as ItemFn);
    // let ItemFn { attrs: _attrs, vis, sig, block } = input;
    let sig = item_fn.sig;
    let original_fn_name = sig.ident.to_string();
    let original_fn_def_with_replaced_name = original_fn_def.replace(
        &format!("{}(", original_fn_name),
        &format!("{}(", replaced_fn_name),
    );
    let sig_inputs: Vec<String> = sig.inputs
        .iter()
        .map(|it| it.to_token_stream().to_string())
        .collect();
    let sig_var_names = sig.inputs
        .iter()
        .map(|it| {
            let stream = it.to_token_stream();
            let s = stream.to_string();
            let parts: Vec<&str> = s.split(":").collect();
            parts[0].to_string()
        })
        .collect::<Vec<String>>();
    let inner_fn_sig_var_names = sig_var_names.join(", ");
    let ret_str = sig.output.to_token_stream().to_string();
    let result_fn_args = sig_inputs.join(", ");
    let result_fn_def = format!(
        r#"
        fn {fn_name}({fn_args}) {fn_ret_str} {{
            {original_fn_def_with_replaced_name}
            let start = std::time::Instant::now();
            let result = {replaced_fn_name}({fn_arg_names});
            let ms = start.elapsed().as_millis();
            println!("{{}} took {{}}ms", "{original_fn_name}", {{ms}});
            return result;
        }}
        "#,
        fn_name = original_fn_name,
        fn_args = result_fn_args,
        fn_ret_str = ret_str,
        replaced_fn_name = replaced_fn_name,
        original_fn_def_with_replaced_name = original_fn_def_with_replaced_name,
        fn_arg_names = inner_fn_sig_var_names,
        original_fn_name = original_fn_name
    );
    let inner_fn: TokenStream = result_fn_def.parse().expect("Generated invalid tokens");
    // println!("{result_fn_def}");
    inner_fn.into()
}
