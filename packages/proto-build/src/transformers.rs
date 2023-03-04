use std::collections::HashMap;
use std::path::Path;

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use prost_types::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorSet, ServiceDescriptorProto,
};
use regex::Regex;
use syn::__private::quote::__private::TokenStream as TokenStream2;
use syn::{parse_quote, Attribute, Fields, Ident, Item, ItemEnum, ItemImpl, ItemStruct, Type};

use crate::{format_ident, quote};

/// Regex substitutions to apply to the prost-generated output
pub const REPLACEMENTS: &[(&str, &str)] = &[
    // Use `tendermint-proto` proto definitions
    ("(super::)+tendermint", "tendermint_proto"),
    // Feature-gate gRPC client modules
    (
        "/// Generated client implementations.",
        "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    ),
    // Feature-gate gRPC client impls which use `tonic::transport`
    (
        "impl (.+)Client<tonic::transport::Channel>",
        "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl ${1}Client<tonic::transport::Channel>",
    ),
];

pub fn append_struct_attrs(
    src: &Path,
    s: &ItemStruct,
    descriptor: &FileDescriptorSet,
) -> ItemStruct {
    let mut s = s.clone();
    let query_services = extract_query_services(descriptor);
    let type_url = get_type_url(src, &s.ident, descriptor);

    s.attrs.append(&mut vec![
        syn::parse_quote! { #[derive(schemars::JsonSchema, serde::Serialize, serde::Deserialize, std_derive::CosmwasmExt)] },
        syn::parse_quote! { #[proto_message(type_url = #type_url)] },
        syn::parse_quote! { #[serde(rename_all = "snake_case")] },
    ]);

    if let Some(attr) = get_query_attr(src, &s.ident, &query_services) {
        s.attrs.append(&mut vec![attr])
    }
    s
}

pub fn allow_serde_number_as_str(s: ItemStruct) -> ItemStruct {
    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            let number_types = vec![
                parse_quote!(i64),
                parse_quote!(i128),
                parse_quote!(u64),
                parse_quote!(u128),
            ];

            if number_types.contains(&field.ty) {
                let from_str: syn::Attribute = parse_quote! {
                    #[serde(
                        serialize_with = "crate::serde::as_str::serialize",
                        deserialize_with = "crate::serde::as_str::deserialize"
                    )]
                };
                field.attrs.append(&mut vec![from_str]);
            }

            field
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

pub fn allow_serde_byte_as_base64(s: ItemStruct) -> ItemStruct {
    let as_base64: syn::Attribute = parse_quote! {
        #[serde(
            serialize_with = "crate::serde::as_base64::serialize",
            deserialize_with = "crate::serde::as_base64::deserialize"
        )]
    };

    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            let byte_types = vec![parse_quote!(::prost::alloc::vec::Vec<u8>)];

            if byte_types.contains(&field.ty) {
                field.attrs.append(&mut vec![as_base64.clone()]);
                field
            } else {
                field
            }
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

pub fn allow_serde_enum_as_str(s: ItemStruct) -> ItemStruct {
    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            // Add custom serde methods for field having enumeration attribute
            if let Some(v) = find_prost_enumeration_value(&field.attrs) {
                let as_enum_serialize = format!("{}::serialize", v);
                let as_enum_derserialize = format!("{}::deserialize", v);
                let as_enum_serde: syn::Attribute = parse_quote! {
                    #[serde(
                        serialize_with = #as_enum_serialize,
                        deserialize_with = #as_enum_derserialize,
                    )]
                };

                field.attrs.append(&mut vec![as_enum_serde]);
            }

            // Add serde flatten for filed having one of attribute
            if has_prost_one_of_attr(&field.attrs) {
                let flatten: syn::Attribute = parse_quote! {
                    #[serde(flatten)]
                };

                field.attrs.append(&mut vec![flatten]);
            }

            field
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

/// Searches for the "enumeration" attribute in an attribute list of the form
/// #[prost(enumeration = "target" )] and returns the target value if it does exist.
fn find_prost_enumeration_value(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        // Parse the attribute's meta information.
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => return None,
        };

        // Check if the attribute name is "prost".
        let list = match meta {
            syn::Meta::List(list)
                if list
                    .path
                    .get_ident()
                    .filter(|ident| ident.to_string() == "prost")
                    .is_some() =>
            {
                list
            }
            _ => return None,
        };

        // Search all nested attributes and look for the "enumeration" attribute.
        list.nested.iter().find_map(|nested| {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nested {
                if nv.path.is_ident("enumeration") {
                    if let syn::Lit::Str(s) = &nv.lit {
                        return Some(s.value());
                    }
                }
            }
            None
        })
    })
}

pub fn has_prost_one_of_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        // Parse the attribute's meta information.
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => return false,
        };

        // Check if the attribute name is "prost".
        let list = match meta {
            syn::Meta::List(list)
                if list
                    .path
                    .get_ident()
                    .filter(|ident| ident.to_string() == "prost")
                    .is_some() =>
            {
                list
            }
            _ => return false,
        };

        // Search all nested attributes and look for the "oneof" attribute.
        list.nested.iter().any(|nested| {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nested {
                return nv.path.is_ident("oneof");
            }

            false
        })
    })
}

pub fn append_enum_attrs(s: &ItemEnum) -> ItemEnum {
    let mut s = s.clone();

    if has_repr_attribute(&s.attrs) {
        s.attrs.append(&mut vec![syn::parse_quote! {
            #[derive(strum_macros::FromRepr)]
        }]);
    }

    s.attrs.append(&mut vec![
        syn::parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        },
        syn::parse_quote! {
           #[serde(rename_all = "snake_case")]
        },
    ]);

    s
}

pub fn has_repr_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        // Parse the attribute's meta information.
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => return false,
        };

        // Check if the attribute name is "prost".
        match meta {
            syn::Meta::List(list) => list
                .path
                .get_ident()
                .filter(|ident| ident.to_string() == "repr")
                .is_some(),

            _ => return false,
        }
    })
}

pub fn add_serde_impl_for_enum_impl(item_impl: &ItemImpl) -> ItemImpl {
    let mut item = item_impl.clone();

    // Check if impl has both `as_str_name` and `from_str_name` methods
    let has_as_str_name = item
        .items
        .iter()
        .any(|item| matches!(item, syn::ImplItem::Method(m) if m.sig.ident == "as_str_name"));

    let has_from_str_name = item
        .items
        .iter()
        .any(|item| matches!(item, syn::ImplItem::Method(m) if m.sig.ident == "from_str_name"));

    if !has_as_str_name || !has_from_str_name {
        return item;
    }

    // Add a custom serde methods for impl
    let serialize_method: syn::ImplItemMethod = parse_quote! {
        pub  fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where  S: serde::Serializer {
            let enum_value = Self::from_repr(*v);
            match enum_value {
                Some(v) => serializer.serialize_str(v.as_str_name()),
                None => Err(serde::ser::Error::custom("unknown value")),
            }
        }
    };

    let deserialize_method: syn::ImplItemMethod = parse_quote! {
        pub  fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
        where  D: serde::Deserializer<'de> {
            let s: &str = serde::Deserialize::deserialize(deserializer)?;
            match Self::from_str_name(s) {
                Some(v) => Ok(v as i32),
                None => Err(serde::de::Error::custom("unknown value")),
            }
        }
    };

    item.items.append(&mut vec![
        serialize_method.into(),
        deserialize_method.into(),
    ]);

    item
}

// ====== helpers ======

fn get_query_attr(
    src: &Path,
    ident: &Ident,
    query_services: &HashMap<String, ServiceDescriptorProto>,
) -> Option<Attribute> {
    let package = src.file_stem().unwrap().to_str().unwrap();
    let service = query_services.get(package);

    let method = service?.method.iter().find(|m| {
        let input_type = (*m).input_type.clone().unwrap();
        let input_type = input_type.split('.').last().unwrap();
        *ident == input_type.to_upper_camel_case()
    });

    let method_name = method?.name.clone().unwrap();
    let response_type = method?.output_type.clone().unwrap();
    let response_type = response_type.split('.').last().unwrap();
    let response_type = format_ident!("{}", response_type.to_upper_camel_case());

    let path = format!("/{}.Query/{}", package, method_name);
    Some(syn::parse_quote! { #[proto_query(path = #path, response_type = #response_type  )] })
}

fn get_type_url(src: &Path, ident: &Ident, descriptor: &FileDescriptorSet) -> String {
    let type_path = src.file_stem().unwrap().to_str().unwrap();
    let init_path = "";

    let name: Option<String> = descriptor
        .file
        .clone()
        .into_iter()
        .filter(|f| f.package.to_owned().unwrap() == type_path)
        .flat_map(|f| {
            let target = ident.to_string();
            vec![
                extract_type_path_from_enum(&target, &f.enum_type, init_path),
                extract_type_path_from_descriptor(&target, &f.message_type, init_path),
            ]
        })
        .filter(|r| r.is_some())
        .collect();

    format!("/{}.{}", type_path, name.unwrap())
}

fn extract_type_path_from_descriptor(
    target: &str,
    message_type: &[DescriptorProto],
    path: &str,
) -> Option<String> {
    message_type.iter().find_map(|descriptor| {
        let message_name = descriptor.name.to_owned().unwrap();

        if message_name.to_upper_camel_case() == target {
            Some(append_type_path(path, &message_name))
        } else if let Some(message_name) = extract_type_path_from_descriptor(
            target,
            &descriptor.nested_type,
            &append_type_path(path, &message_name),
        ) {
            Some(message_name)
        } else {
            extract_type_path_from_enum(
                target,
                &descriptor.enum_type,
                &append_type_path(path, &message_name),
            )
        }
    })
}

fn extract_type_path_from_enum(
    target: &str,
    enum_type: &[EnumDescriptorProto],
    path: &str,
) -> Option<String> {
    enum_type
        .iter()
        .find(|e| e.name.to_owned().unwrap().to_upper_camel_case() == target)
        .map(|e| append_type_path(path, &e.name.to_owned().unwrap()))
}

pub fn extract_query_services(
    descriptor: &FileDescriptorSet,
) -> HashMap<String, ServiceDescriptorProto> {
    descriptor
        .clone()
        .file
        .into_iter()
        .filter_map(|f| {
            let service = f
                .service
                .into_iter()
                .find(|s| s.name == Some("Query".to_string()));

            if let Some(service) = service {
                Some((
                    f.package.expect("Missing package name in file descriptor"),
                    service,
                ))
            } else {
                None
            }
        })
        .collect()
}

fn append_type_path(path: &str, name: &str) -> String {
    if path.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", path, name)
    }
}

pub fn append_querier(
    items: Vec<Item>,
    src: &Path,
    nested_mod: bool,
    descriptor: &FileDescriptorSet,
) -> Vec<Item> {
    let package = src.file_stem().unwrap().to_str().unwrap();
    let re = Regex::new(r"([^.]*)(\.v\d+(beta\d+)?)?$").unwrap();

    let package_stem = re.captures(package).unwrap().get(1).unwrap().as_str();

    let querier_wrapper_ident = format_ident!("{}Querier", &package_stem.to_upper_camel_case());

    let query_services = extract_query_services(descriptor);
    let query_fns = query_services.get(package).map(|service| service.method.iter().map(|method_desc| {
        if nested_mod {
            return quote! {};
        }

        let method_desc = method_desc.clone();

        let name = format_ident!("{}", method_desc.name.unwrap().as_str().to_snake_case());
        let req_type = format_ident!("{}", method_desc.input_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());
        let res_type = format_ident!("{}", method_desc.output_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());

        let req_args = items.clone().into_iter()
            .find_map(|item| match item {
                Item::Struct(s) => {
                    if s.ident == req_type {
                        match s.fields {
                            Fields::Named(fields_named) => {
                                Some(fields_named.named)
                            }
                            _ => None
                        }
                    } else {
                        None
                    }
                }
                _ => None
            });

        let arg_idents = req_args.clone().unwrap().into_iter().map(|arg| arg.ident.unwrap()).collect::<Vec<Ident>>();
        let arg_ty = req_args.unwrap().into_iter().map(|arg| arg.ty).collect::<Vec<Type>>();

        quote! {
          pub fn #name( &self, #(#arg_idents : #arg_ty),* ) -> std::result::Result<#res_type, cosmwasm_std::StdError> {
            #req_type { #(#arg_idents),* }.query(self.querier)
          }
        }
    }).collect::<Vec<TokenStream2>>());

    let querier = if let Some(query_fns) = query_fns {
        if !nested_mod {
            vec![
                parse_quote! {
                  pub struct #querier_wrapper_ident<'a, Q: cosmwasm_std::CustomQuery> {
                      querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
                  }
                },
                parse_quote! {
                  impl<'a, Q: cosmwasm_std::CustomQuery> #querier_wrapper_ident<'a, Q> {
                      pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
                    Self { querier }
                    }
                    #(#query_fns)*
                  }
                },
            ]
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    vec![items, querier].concat()
}
