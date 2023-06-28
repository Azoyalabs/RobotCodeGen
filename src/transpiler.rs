use std::fs::File;
use std::io::{Read, Write};

use inflections::case::{to_pascal_case, to_snake_case};
use proc_macro2::TokenStream;
use syn::visit::Visit;
use syn::{Field, Ident, ItemEnum, Type};

struct MsgArg {
    ident: Option<Ident>,
    r#type: Type,
}

impl MsgArg {
    fn from_field(field_data: Field) -> Self {
        return MsgArg {
            ident: field_data.ident,
            r#type: field_data.ty,
        };
    }

    fn _to_quote(&self) -> TokenStream {
        return quote::quote!(
            self.#(ident): #(self.r#type)
        );
    }
}

enum MsgDescr {
    ExecuteMsg {
        ident: Ident,
        args: Vec<MsgArg>,
    },
    QueryMsg {
        ident: Ident,
        args: Vec<MsgArg>,
        return_type: String,
    },
}

impl MsgDescr {
    fn to_trait_def(&self) -> String {
        let (ident, args) = match self {
            MsgDescr::ExecuteMsg { ident, args } => (ident, args),
            MsgDescr::QueryMsg {
                ident,
                args,
                return_type: _,
            } => (ident, args),
        };

        let camel_name = to_snake_case(&ident.to_string());

        let args_quoted: Vec<TokenStream> = args
            .iter()
            .map(|arg| {
                let arg_ident = arg.ident.clone().unwrap().to_owned();
                let arg_type = arg.r#type.clone();
                quote::quote! {
                    #arg_ident: #arg_type
                }
            })
            .collect();

        // let's try to stringify it directly
        let args_fn_string = quote::quote! {
            #(#args_quoted,)*
        }
        .to_string();

        return match self {
            MsgDescr::ExecuteMsg { ident: _, args: _ } => {
                format!("\tfn {camel_name}(app: &mut App, {args_fn_string} funds: Vec<Coin>);")
            }
            MsgDescr::QueryMsg {
                ident: _,
                args: _,
                return_type,
            } => format!("\tfn {camel_name}(app: &App, {args_fn_string}) -> {return_type};"),
        };
    }

    fn to_trait_impl(&self) -> String {
        let (ident, args) = match self {
            MsgDescr::ExecuteMsg { ident, args } => (ident, args),
            MsgDescr::QueryMsg {
                ident,
                args,
                return_type: _,
            } => (ident, args),
        };

        let name = ident.to_string();
        let camel_name = to_snake_case(&ident.to_string());

        let args_quoted: Vec<TokenStream> = args
            .iter()
            .map(|arg| {
                let arg_ident = arg.ident.clone().unwrap().to_owned();
                let arg_type = arg.r#type.clone();
                quote::quote! {
                    #arg_ident: #arg_type
                }
            })
            .collect();

        let args_names: Vec<Ident> = args.iter().map(|arg| arg.ident.clone().unwrap()).collect();

        // let's try to stringify it directly
        let args_fn_string = quote::quote! {
            #(#args_quoted,)*
        }
        .to_string();

        let args_msg = quote::quote! {
            #(#args_names,)*
        }
        .to_string();

        return match self {
            MsgDescr::ExecuteMsg { ident: _, args: _ } => {
                format!("\tfn {camel_name}(app: &mut App, contract: &Addr, caller: &Addr, {args_fn_string} funds: Vec<Coin>){{\n\t\tlet msg = {name} {{{args_msg}}};\n\t\tapp.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();\n\t}}\n")
            }
            MsgDescr::QueryMsg {
                ident: _,
                args: _,
                return_type,
            } => format!("\tfn {camel_name}(app: &App, contract: &Addr, {args_fn_string}) -> {return_type}{{\n\t\tlet msg = {name} {{{args_msg}}};\n\t\treturn app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();\n\t}}\n"),
        };
    }
}

struct EnumVisitor {
    execute_messages: Vec<MsgDescr>,
    query_messages: Vec<MsgDescr>,
}

impl EnumVisitor {
    fn new() -> Self {
        return EnumVisitor {
            execute_messages: vec![],
            query_messages: vec![],
        };
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        // check if this is a target enum
        println!("Enum with name: {}", node.ident.to_string());

        let descr: Vec<MsgDescr> = match node.ident.to_string().as_str() {
            "ExecuteMsg" => node
                .variants
                .iter()
                .filter_map(|elem| {
                    let name = elem.ident.to_owned();
                    let fields: Vec<MsgArg> = elem
                        .fields
                        .iter()
                        .map(|f| MsgArg::from_field(f.to_owned()))
                        .collect();

                    if fields.len() > 0 && fields[0].ident.is_none() {
                        None
                    } else {
                        Some(MsgDescr::ExecuteMsg {
                            ident: name,
                            args: fields,
                        })
                    }
                })
                .collect(),
            "QueryMsg" => node
                .variants
                .iter()
                .map(|elem| {
                    let name = elem.ident.to_owned();
                    let fields: Vec<MsgArg> = elem
                        .fields
                        .iter()
                        .map(|f| MsgArg::from_field(f.to_owned()))
                        .collect();

                    MsgDescr::QueryMsg {
                        ident: name.clone(),
                        args: fields,
                        return_type: format!("{name}Response"),
                    }
                })
                .collect(),
            _ => vec![],
        };

        match node.ident.to_string().as_str() {
            "ExecuteMsg" => self.execute_messages = descr,
            "QueryMsg" => self.query_messages = descr,
            _ => (),
        }

        // Delegate to the default impl to visit any nested functions.
        //visit::visit_item_enum(self, node);
    }
}

pub fn generate_robot_code_from_str(content: String, outfile: &str, crate_name: Option<String>) {
    let ast = syn::parse_file(&content).unwrap();

    let mut enum_visitor = EnumVisitor::new();
    enum_visitor.visit_file(&ast);

    let mut file = File::create(outfile).unwrap();

    // enforce pascal case
    let crate_name = to_pascal_case(&match crate_name {
        None => env!("CARGO_CRATE_NAME").to_string(),
        Some(cn) => cn,
    });

    let trait_name = format!("{crate_name}Robot");
    file.write(format!("pub trait {trait_name} {{\n").as_bytes())
        .unwrap();
    let _res: Vec<usize> = enum_visitor
        .execute_messages
        .iter()
        .map(|elem| {
            let temp = elem.to_trait_def();
            file.write(format!("{temp}\n").as_bytes()).unwrap()
        })
        .collect();

    let _res: Vec<usize> = enum_visitor
        .query_messages
        .iter()
        .map(|elem| {
            let temp = elem.to_trait_def();
            file.write(format!("{temp}\n").as_bytes()).unwrap()
        })
        .collect();
    file.write("}\n\n".as_bytes()).unwrap();

    file.write(format!("impl {crate_name}Robot for Robot {{\n").as_bytes())
        .unwrap();

    let _res: Vec<usize> = enum_visitor
        .execute_messages
        .iter()
        .map(|elem| file.write(elem.to_trait_impl().as_bytes()).unwrap())
        .collect(); //.collect();
    let _res: Vec<usize> = enum_visitor
        .query_messages
        .iter()
        .map(|elem| file.write(elem.to_trait_impl().as_bytes()).unwrap())
        .collect();
    file.write("}".as_bytes()).unwrap();
}

pub fn generate_robot_code(input_file: &str, outfile: &str, crate_name: Option<String>) {
    let mut file = File::open(input_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    generate_robot_code_from_str(content, outfile, crate_name);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TARGET_FILE: &'static str = "src/input.rs";

    #[test]
    fn initial_stuff() {
        generate_robot_code(TARGET_FILE, "dev_out.rs", None);
    }
}
