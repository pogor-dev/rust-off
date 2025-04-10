//! A macro that mimics the old Salsa-style `#[query_group]` macro.

use core::fmt;
use std::vec;

use proc_macro::TokenStream;
use proc_macro2::Span;
use queries::{
    GeneratedInputStruct, InputQuery, InputSetter, InputSetterWithDurability, Intern, Lookup,
    Queries, SetterKind, TrackedQuery, Transparent,
};
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{Attribute, FnArg, ItemTrait, Path, TraitItem, TraitItemFn, parse_quote};

mod queries;

#[proc_macro_attribute]
pub fn query_group(args: TokenStream, input: TokenStream) -> TokenStream {
    match query_group_impl(args, input.clone()) {
        Ok(tokens) => tokens,
        Err(e) => token_stream_with_error(input, e),
    }
}

#[derive(Debug)]
struct InputStructField {
    name: proc_macro2::TokenStream,
    ty: proc_macro2::TokenStream,
}

impl fmt::Display for InputStructField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct SalsaAttr {
    name: String,
    tts: TokenStream,
    span: Span,
}

impl std::fmt::Debug for SalsaAttr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", self.name)
    }
}

impl TryFrom<syn::Attribute> for SalsaAttr {
    type Error = syn::Attribute;

    fn try_from(attr: syn::Attribute) -> Result<SalsaAttr, syn::Attribute> {
        if is_not_salsa_attr_path(attr.path()) {
            return Err(attr);
        }

        let span = attr.span();

        let name = attr.path().segments[1].ident.to_string();
        let tts = match attr.meta {
            syn::Meta::Path(path) => path.into_token_stream(),
            syn::Meta::List(ref list) => {
                let tts = list
                    .into_token_stream()
                    .into_iter()
                    .skip(attr.path().to_token_stream().into_iter().count());
                proc_macro2::TokenStream::from_iter(tts)
            }
            syn::Meta::NameValue(nv) => nv.into_token_stream(),
        }
        .into();

        Ok(SalsaAttr { name, tts, span })
    }
}

fn is_not_salsa_attr_path(path: &syn::Path) -> bool {
    path.segments.first().map(|s| s.ident != "salsa").unwrap_or(true) || path.segments.len() != 2
}

fn filter_attrs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<SalsaAttr>) {
    let mut other = vec![];
    let mut salsa = vec![];
    // Leave non-salsa attributes untouched. These are
    // attributes that don't start with `salsa::` or don't have
    // exactly two segments in their path.
    for attr in attrs {
        match SalsaAttr::try_from(attr) {
            Ok(it) => salsa.push(it),
            Err(it) => other.push(it),
        }
    }
    (other, salsa)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum QueryKind {
    Input,
    Tracked,
    TrackedWithSalsaStruct,
    Transparent,
    Interned,
}

pub(crate) fn query_group_impl(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, syn::Error> {
    let mut item_trait = syn::parse::<ItemTrait>(input)?;

    let supertraits = &item_trait.supertraits;

    let db_attr: Attribute = parse_quote! {
        #[salsa::db]
    };
    item_trait.attrs.push(db_attr);

    let trait_name_ident = &item_trait.ident.clone();
    let input_struct_name = format_ident!("{}Data", trait_name_ident);
    let create_data_ident = format_ident!("create_data_{}", trait_name_ident);

    let mut input_struct_fields: Vec<InputStructField> = vec![];
    let mut trait_methods = vec![];
    let mut setter_trait_methods = vec![];
    let mut lookup_signatures = vec![];
    let mut lookup_methods = vec![];

    for item in item_trait.clone().items {
        if let syn::TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let signature = &method.sig.clone();

            let (_attrs, salsa_attrs) = filter_attrs(method.attrs);

            let mut query_kind = QueryKind::Tracked;
            let mut invoke = None;
            let mut cycle = None;
            let mut interned_struct_path = None;
            let mut lru = None;

            let params: Vec<FnArg> = signature.inputs.clone().into_iter().collect();
            let pat_and_tys = params
                .into_iter()
                .filter(|fn_arg| matches!(fn_arg, FnArg::Typed(_)))
                .map(|fn_arg| match fn_arg {
                    FnArg::Typed(pat_type) => pat_type.clone(),
                    FnArg::Receiver(_) => unreachable!("this should have been filtered out"),
                })
                .collect::<Vec<syn::PatType>>();

            for SalsaAttr { name, tts, span } in salsa_attrs {
                match name.as_str() {
                    "cycle" => {
                        let path = syn::parse::<Parenthesized<Path>>(tts)?;
                        cycle = Some(path.0.clone())
                    }
                    "input" => {
                        if !pat_and_tys.is_empty() {
                            return Err(syn::Error::new(
                                span,
                                "input methods cannot have a parameter",
                            ));
                        }
                        query_kind = QueryKind::Input;
                    }
                    "interned" => {
                        let syn::ReturnType::Type(_, ty) = &signature.output else {
                            return Err(syn::Error::new(
                                span,
                                "interned queries must have return type",
                            ));
                        };
                        let syn::Type::Path(path) = &**ty else {
                            return Err(syn::Error::new(
                                span,
                                "interned queries must have return type",
                            ));
                        };
                        interned_struct_path = Some(path.path.clone());
                        query_kind = QueryKind::Interned;
                    }
                    "invoke" => {
                        let path = syn::parse::<Parenthesized<Path>>(tts)?;
                        invoke = Some(path.0.clone());
                    }
                    "invoke_actual" => {
                        let path = syn::parse::<Parenthesized<Path>>(tts)?;
                        invoke = Some(path.0.clone());
                        query_kind = QueryKind::TrackedWithSalsaStruct;
                    }
                    "lru" => {
                        let lru_count = syn::parse::<Parenthesized<syn::LitInt>>(tts)?;
                        let lru_count = lru_count.0.base10_parse::<u32>()?;

                        lru = Some(lru_count);
                    }
                    "transparent" => {
                        query_kind = QueryKind::Transparent;
                    }
                    _ => return Err(syn::Error::new(span, format!("unknown attribute `{name}`"))),
                }
            }

            let syn::ReturnType::Type(_, return_ty) = signature.output.clone() else {
                return Err(syn::Error::new(signature.span(), "Queries must have a return type"));
            };

            if let syn::Type::Path(ref ty_path) = *return_ty {
                if matches!(query_kind, QueryKind::Input) {
                    let field = InputStructField {
                        name: method_name.to_token_stream(),
                        ty: ty_path.path.to_token_stream(),
                    };

                    input_struct_fields.push(field);
                }
            }

            match (query_kind, invoke) {
                // input
                (QueryKind::Input, None) => {
                    let query = InputQuery {
                        signature: method.sig.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    let value = Queries::InputQuery(query);
                    trait_methods.push(value);

                    let setter = InputSetter {
                        signature: method.sig.clone(),
                        return_type: *return_ty.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    setter_trait_methods.push(SetterKind::Plain(setter));

                    let setter = InputSetterWithDurability {
                        signature: method.sig.clone(),
                        return_type: *return_ty.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    setter_trait_methods.push(SetterKind::WithDurability(setter));
                }
                (QueryKind::Interned, None) => {
                    let interned_struct_path = interned_struct_path.unwrap();
                    let method = Intern {
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        interned_struct_path: interned_struct_path.clone(),
                    };

                    trait_methods.push(Queries::Intern(method));

                    let mut method = Lookup {
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        return_ty: *return_ty,
                        interned_struct_path,
                    };
                    method.prepare_signature();

                    lookup_signatures
                        .push(TraitItem::Fn(make_trait_method(method.signature.clone())));
                    lookup_methods.push(method);
                }
                // tracked function. it might have an invoke, or might not.
                (QueryKind::Tracked, invoke) => {
                    let method = TrackedQuery {
                        trait_name: trait_name_ident.clone(),
                        generated_struct: Some(GeneratedInputStruct {
                            input_struct_name: input_struct_name.clone(),
                            create_data_ident: create_data_ident.clone(),
                        }),
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke,
                        cycle,
                        lru,
                    };

                    trait_methods.push(Queries::TrackedQuery(method));
                }
                (QueryKind::TrackedWithSalsaStruct, Some(invoke)) => {
                    let method = TrackedQuery {
                        trait_name: trait_name_ident.clone(),
                        generated_struct: None,
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke: Some(invoke),
                        cycle,
                        lru,
                    };

                    trait_methods.push(Queries::TrackedQuery(method))
                }
                // while it is possible to make this reachable, it's not really worthwhile for a migration aid.
                // doing this would require attaching an attribute to the salsa struct parameter in the query.
                (QueryKind::TrackedWithSalsaStruct, None) => unreachable!(),
                (QueryKind::Transparent, invoke) => {
                    let method = Transparent {
                        signature: method.sig.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke,
                    };
                    trait_methods.push(Queries::Transparent(method));
                }
                // error/invalid constructions
                (QueryKind::Interned, Some(path)) => {
                    return Err(syn::Error::new(
                        path.span(),
                        "Interned queries cannot be used with an `#[invoke]`".to_string(),
                    ));
                }
                (QueryKind::Input, Some(path)) => {
                    return Err(syn::Error::new(
                        path.span(),
                        "Inputs cannot be used with an `#[invoke]`".to_string(),
                    ));
                }
            }
        }
    }

    let fields = input_struct_fields
        .into_iter()
        .map(|input| {
            let name = input.name;
            let ret = input.ty;
            quote! { #name: Option<#ret> }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let input_struct = quote! {
        #[salsa::input]
        pub(crate) struct #input_struct_name {
            #(#fields),*
        }
    };

    let field_params = std::iter::repeat_n(quote! { None }, fields.len())
        .collect::<Vec<proc_macro2::TokenStream>>();

    let create_data_method = quote! {
        #[allow(non_snake_case)]
        #[salsa::tracked]
        fn #create_data_ident(db: &dyn #trait_name_ident) -> #input_struct_name {
            #input_struct_name::new(db, #(#field_params),*)
        }
    };

    let mut setter_signatures = vec![];
    let mut setter_methods = vec![];
    for trait_item in setter_trait_methods
        .iter()
        .map(|method| method.to_token_stream())
        .map(|tokens| syn::parse2::<syn::TraitItemFn>(tokens).unwrap())
    {
        let mut methods_sans_body = trait_item.clone();
        methods_sans_body.default = None;
        methods_sans_body.semi_token = Some(syn::Token![;](trait_item.span()));

        setter_signatures.push(TraitItem::Fn(methods_sans_body));
        setter_methods.push(TraitItem::Fn(trait_item));
    }

    item_trait.items.append(&mut setter_signatures);
    item_trait.items.append(&mut lookup_signatures);

    let trait_impl = quote! {
        #[salsa::db]
        impl<DB> #trait_name_ident for DB
        where
            DB: #supertraits,
        {
            #(#trait_methods)*

            #(#setter_methods)*

            #(#lookup_methods)*
        }
    };
    RemoveAttrsFromTraitMethods.visit_item_trait_mut(&mut item_trait);

    let out = quote! {
        #item_trait

        #trait_impl

        #input_struct

        #create_data_method
    }
    .into();

    Ok(out)
}

/// Parenthesis helper
pub(crate) struct Parenthesized<T>(pub(crate) T);

impl<T> syn::parse::Parse for Parenthesized<T>
where
    T: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        content.parse::<T>().map(Parenthesized)
    }
}

fn make_trait_method(sig: syn::Signature) -> TraitItemFn {
    TraitItemFn {
        attrs: vec![],
        sig: sig.clone(),
        semi_token: Some(syn::Token![;](sig.span())),
        default: None,
    }
}

struct RemoveAttrsFromTraitMethods;

impl VisitMut for RemoveAttrsFromTraitMethods {
    fn visit_item_trait_mut(&mut self, i: &mut syn::ItemTrait) {
        for item in &mut i.items {
            if let TraitItem::Fn(trait_item_fn) = item {
                trait_item_fn.attrs = vec![];
            }
        }
    }
}

pub(crate) fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
