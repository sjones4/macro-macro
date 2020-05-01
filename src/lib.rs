// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, Ident, Token,
};

thread_local! {
    static TEMPLATES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

struct TemplateValidation {
    pub name: String,
}

impl Parse for TemplateValidation {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_template(input, |name, _| TemplateValidation { name })
    }
}

struct TemplateInput {
    pub name: String,
    pub example: DeriveInput,
}

impl Parse for TemplateInput {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_template(input, |name, example| TemplateInput { name, example })
    }
}

struct TemplateName {
    pub name: String,
}

impl Parse for TemplateName {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = Ident::parse(input)?.to_string();
        Ok(TemplateName { name })
    }
}

fn parse_template<T>(input: ParseStream, builder: fn(String, DeriveInput) -> T) -> Result<T> {
    let name = Ident::parse(input)?.to_string();
    let _eq_token: Option<Token![=]> = input.parse()?;
    let content;
    let _brace_token = braced!(content in input);
    let input = DeriveInput::parse(&content)?;
    Ok(builder(name, input))
}

fn match_name(pattern: &str, name: &str) -> bool {
    let (name_prefix, name_suffix) = if pattern.contains("__") {
        let mut splitter = pattern.splitn(2, "__");
        let prefix = splitter.next().unwrap();
        let suffix = splitter.next().unwrap();
        (prefix, suffix)
    } else {
        (pattern, pattern)
    };

    name.starts_with(name_prefix) && name.ends_with(name_suffix)
}

#[proc_macro]
pub fn macro_template(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let template = parse_macro_input!(input as TemplateValidation);

    TEMPLATES.with(|templates| -> () {
        let template_name = template.name.to_string();
        let mut templates_ref = templates.borrow_mut();
        templates_ref.insert(template_name, input_str);
    });

    TokenStream::new()
}

fn lookup_template(name: &str) -> String {
    TEMPLATES.with(|templates| {
        let templates_ref = templates.borrow();
        templates_ref
            .get(name)
            .expect("invalid template name")
            .clone()
    })
}

fn input_fields(input: &DeriveInput) -> Vec<Field> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let field_vec: Vec<Field> = fields.named.iter().map(|f| f.clone()).collect();
            field_vec
        }
        _ => panic!("expected a struct with named fields"),
    }
}

fn template_fields(template_input: &TemplateInput) -> Vec<&Field> {
    match &template_input.example.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let field_vec: Vec<&Field> = fields.named.iter().collect();
            field_vec
        }
        _ => Vec::new(),
    }
}

fn update_fields(fields: &mut Vec<Field>, template_fields: &Vec<&Field>) {
    for field in fields {
        let field_name = field.ident.as_ref().map(|id| id.to_string()).unwrap();
        for template_field in template_fields {
            let template_field_name = template_field
                .ident
                .as_ref()
                .map(|id| id.to_string())
                .unwrap();
            let field_name_match = match_name(&template_field_name, &field_name);

            let field_type = &field.ty;
            let template_field_type = &template_field.ty;
            let type_desc = quote! {#field_type}.to_string();
            let template_type_desc = quote! {#template_field_type}.to_string().replace("!", "__");
            let field_type_match = match_name(&template_type_desc, &type_desc);

            if field_name_match && field_type_match {
                field
                    .attrs
                    .extend(template_field.attrs.iter().map(|f| f.clone()));
            }
        }
    }
}

#[proc_macro_attribute]
pub fn macro_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let template_name = parse_macro_input!(args as TemplateName).name.to_string();
    let input = parse_macro_input!(input as DeriveInput);

    let template_str = lookup_template(&template_name);
    let template_ts = TokenStream::from_str(&template_str).expect("template");
    let template_input = parse_macro_input!(template_ts as TemplateInput);
    let template_struct_name = template_input.example.ident.to_string();

    let struct_name = input.ident.to_string();
    let structure_match = match_name(&template_struct_name, &struct_name);

    let mut fields = input_fields(&input);
    if structure_match {
        let template_fields: Vec<&Field> = template_fields(&template_input);
        update_fields(&mut fields, &template_fields);
    }

    let template_attrs = if structure_match {
        template_input.example.attrs
    } else {
        Vec::new()
    };

    // Output with any additional templated attributes and updated field attributes
    let attrs = input.attrs;
    let generics = input.generics;
    let name = input.ident;
    let vis = input.vis;
    (quote! {
        #(
          #template_attrs
        )*
        #(
          #attrs
        )*
        #vis struct #name #generics {
            #(
                #fields,
            )*
        }
    })
    .into()
}
