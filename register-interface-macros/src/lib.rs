use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ItemStruct, LitInt, Result, Token};

struct FieldArgs {
    name: Ident,
    _s0: Token![,],
    from: LitInt,
    _s1: Token![,],
    to: LitInt,
}

impl Parse for FieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(FieldArgs {
            name: input.parse()?,
            _s0: input.parse()?,
            from: input.parse()?,
            _s1: input.parse()?,
            to: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn field(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_p = parse_macro_input!(args as FieldArgs);
    let field_name = args_p.name.clone();
    let field_from = args_p.from.base10_digits().parse().unwrap();
    let field_to = args_p.to.base10_digits().parse().unwrap();
    let clonned = input.clone();
    let input_p = parse_macro_input!(input as ItemStruct);
    let struct_name = input_p.ident;
    let set_field_name = Ident::new(&format!("set_{}", args_p.name), Span::call_site());

    let mask = gen_mask(field_from, field_to);
    let n_mask = !mask;
    let extended = quote!(
        impl #struct_name {
            pub fn #field_name(&self) -> usize {
                use core::ptr::read_volatile;
                ((unsafe { read_volatile(self.addr)}) & #mask) >> #field_from
            }
            pub fn #set_field_name(&mut self, value: usize) {
                use core::ptr::{read_volatile, write_volatile};
                unsafe {
                  let original = read_volatile(self.addr) & #n_mask;
                  let value = value << #field_from;
                  write_volatile(self.addr, original | value);
                }
            }
        }
    );

    let mut ts = TokenStream::from(extended);
    ts.extend(clonned);
    ts
}

struct RegisterArgs {
    name: Ident,
    _s0: Token![,],
    typ: Ident,
    _s1: Token![,],
    offset: LitInt,
}

impl Parse for RegisterArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RegisterArgs {
            name: input.parse()?,
            _s0: input.parse()?,
            typ: input.parse()?,
            _s1: input.parse()?,
            offset: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn register(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_p = parse_macro_input!(args as RegisterArgs);
    let reg_name = args_p.name.clone();
    let reg_offset: usize = args_p.offset.base10_digits().parse().unwrap();
    let reg_type = args_p.typ;
    let clonned = input.clone();
    let input_p = parse_macro_input!(input as ItemStruct);
    let struct_name = input_p.ident;

    let extended = quote!(
        impl #struct_name {
            pub fn #reg_name(&self) -> #reg_type {
                use core::mem::transmute;
                unsafe {
                    let addr = (self.addr as usize + #reg_offset) as *mut usize;
                    #reg_type{addr}
                }
            }
        }
    );

    let mut ts = TokenStream::from(extended);
    ts.extend(clonned);
    ts
}

fn gen_mask(from: u32, to: u32) -> usize {
    let mut mask = 0;
    for i in from..=to {
        mask |= 1 << i;
    }
    mask
}
