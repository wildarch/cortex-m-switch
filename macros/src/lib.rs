extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashSet;
use syn::punctuated::Pair;
use syn::spanned::Spanned;
use syn::{
    parse, Error, FnArg, Ident, Item, ItemFn, ItemStatic, Pat, ReturnType, Stmt, Type, Visibility,
};

fn exc_return_type(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(path) => match path.path.segments.last() {
            Some(Pair::End(name)) => if name.ident.to_string() == "ExceptionReturn" {
                Some(ty.clone())
            } else {
                None
            },
            _ => None,
        },
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return Error::new(
            Span::call_site(),
            "This attribute does not accept arguments",
        ).to_compile_error()
        .into();
    }

    let func = parse_macro_input!(input as ItemFn);

    // Function name check
    const ALLOWED_NAMES: &'static [&'static str] = &["SysTick", "PendSV", "SVCall"];
    let func_name = func.ident.to_string();
    if !ALLOWED_NAMES.contains(&&*func_name) {
        return Error::new(
            func.ident.span(),
            format!("The function name must be one of: {:?}", ALLOWED_NAMES),
        ).to_compile_error()
        .into();
    }

    // Argument check
    let (arg_name, arg_type) = if func.decl.inputs.len() == 1 {
        match func.decl.inputs[0] {
            FnArg::Captured(ref arg) => (
                match arg.pat {
                    Pat::Ident(ref arg_ident) => if arg_ident.by_ref.is_none()
                        && arg_ident.mutability.is_none()
                        && arg_ident.subpat.is_none()
                    {
                        Some(arg_ident.ident.clone())
                    } else {
                        None
                    },
                    _ => None,
                },
                exc_return_type(&arg.ty),
            ),
            _ => (None, None),
        }
    } else {
        (None, None)
    };

    // Return type must have name 'ExceptionReturn'
    let return_type = match func.decl.output {
        ReturnType::Default => None,
        ReturnType::Type(_arrow, ref ty) => match ty.as_ref() {
            Type::Path(path) => match path.path.segments.last() {
                Some(Pair::End(name)) => if name.ident.to_string() == "ExceptionReturn" {
                    Some(ty.clone())
                } else {
                    None
                },
                _ => None,
            },
            _ => None,
        },
    };

    let valid_declaration = func.decl.generics.params.is_empty()
        && func.decl.generics.where_clause.is_none()
        && func.decl.variadic.is_none()
        && arg_name.is_some()
        && arg_type.is_some()
        && return_type.is_some()
        && arg_type.map(Box::new) == return_type;

    let valid_signature =
        // TODO check attributes?
        func.vis == Visibility::Inherited &&
        func.constness.is_none() &&
        // unsafeness whatever
        func.asyncness.is_none() &&
        func.abi.is_none() &&
        valid_declaration;

    if !valid_signature {
        return Error::new(
            func.span(),
            "exceptions must have signature \
             `[unsafe] fn(ExceptionReturn) -> ExceptionReturn`",
        ).to_compile_error()
        .into();
    }

    let stmts = func.block.stmts;

    let (statics, stmts) = match extract_static_muts(stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let vars = statics
        .into_iter()
        .map(|var| {
            let attrs = var.attrs;
            let ident = var.ident;
            let ty = var.ty;
            let expr = var.expr;

            quote!(
		#[allow(non_snake_case)]
		let #ident: &mut #ty = unsafe {
		    #(#attrs)*
		    static mut #ident: #ty = #expr;

		    &mut #ident
		};
	    )
        }).collect::<Vec<_>>();

    let attrs = func.attrs;
    let unsafety = func.unsafety;
    let mangle_name = format!("cortex_m_switch_{}", func_name);
    let mangle_ident = Ident::new(&mangle_name, Span::call_site());
    let func_ident = func.ident;
    let asm = format!(
        "
                    mov r0, lr

                    /* Don't push to psp stack if we weren't using it. */
                    and r1, r0, #4
                    cbz r1, skip_stack_push

                    /* Save psp stack. */
                    mrs r12, psp
                    stmdb r12!, {{r4-r11}}
                    msr psp, r12
                skip_stack_push:
                    bl {}

                    /* Don't pop psp stack if we're not switching to it. */
                    and r1, r0, #4
                    cbz r1, skip_stack_pop

                    /* Restore psp stack. */
                    mrs r12, psp
                    ldmfd r12!, {{r4-r11}}
                    msr psp, r12
                skip_stack_pop:
                    bx r0
                ",
        mangle_name
    );

    quote!(
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn #func_ident() {
            asm!(#asm);
        }

        #(#attrs)*
        #[no_mangle]
        pub #unsafety extern "C" fn #mangle_ident(#arg_name: #return_type) -> #return_type {
	    #(#vars)*

            #(#stmts)*
        }
    ).into()
}

/// Extracts `static mut` vars from the beginning of the given statements
fn extract_static_muts(stmts: Vec<Stmt>) -> Result<(Vec<ItemStatic>, Vec<Stmt>), parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut seen = HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];
    while let Some(stmt) = istmts.next() {
        match stmt {
            Stmt::Item(Item::Static(var)) => {
                if var.mutability.is_some() {
                    if seen.contains(&var.ident) {
                        return Err(parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }

                    seen.insert(var.ident.clone());
                    statics.push(var);
                } else {
                    stmts.push(Stmt::Item(Item::Static(var)));
                }
            }
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);

    Ok((statics, stmts))
}
