use glob::glob;
use std::{collections::{HashMap}, fs::File, io::Read};
use syn::{parse_file, FnArg, GenericArgument, Item, PathArguments, Type, __private::quote::__private::TokenTree};

pub type EventLinks = HashMap<String, EventLink>;

pub fn parse_files(path: &str) -> EventLinks {
    let pattern = format!("{path}/src/**/*.rs");
    let files: Result<Vec<_>, _> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .collect();

    let functions = files
        .unwrap()
        .into_iter()
        .flat_map(|file_name| {
            let file_name = file_name.as_path().to_str().unwrap().to_owned();
            extract_functions(&file_name).into_iter()
        })
        .collect::<Vec<_>>();

    let mut event_links: HashMap<String, EventLink> = HashMap::new();

    for function in functions {
        let readers = function.arguments
            .iter()
            .filter(|arg| arg.type_name == "EventReader");
        let writers = function.arguments
            .iter()
            .filter(|arg| arg.type_name == "EventWriter");

        for reader in readers {
            let event_type = reader.sub_type.first().unwrap();
            let event_name = event_type.type_name.clone();

            let entry = event_links.entry(event_name).or_default();
            entry.readers.push(function.clone());
        }
        for writer in writers {
            let event_type = writer.sub_type.first().unwrap();
            let event_name = event_type.type_name.clone();

            let entry = event_links.entry(event_name).or_default();
            entry.writers.push(function.clone());
        }
    }

    event_links
}

#[derive(Default, Debug)]
pub struct EventLink {
    pub readers: Vec<FunctionPointer>,
    pub writers: Vec<FunctionPointer>,
}

#[derive(Debug, Clone)]
pub struct FunctionPointer {
    pub function_name: String,
    pub file_path: String,
    pub arguments: Vec<GenericsArgument>,
    pub docs: Vec<String>,
}

fn extract_functions(file_path: &str) -> Vec<FunctionPointer> {
    let mut file = File::open(format!("{}", file_path)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let ast = parse_file(&content).unwrap();
    ast.items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(i) => Some(i),
            _ => None,
        })
        .map(|function| {
            let name = format!("{}", function.sig.ident);

            let arguments: Vec<_> = function
                .sig
                .inputs
                .into_iter()
                .filter_map(|arg| match arg {
                    FnArg::Typed(a) => {
                        let ty: Type = *a.ty;
                        Some(extrapolate_type(ty))
                    }
                    _ => None,
                })
                .collect();

            let docs: Vec<_> = function.attrs.iter().flat_map(|attr| {
                let doc = attr.path.segments.iter().filter(|p| p.ident.to_string() == "doc").count();
                if doc == 0 {
                    return vec![];
                }
                let texts: Vec<_> = attr.tokens.clone().into_iter().filter_map(|t| {
                    match t {
                        TokenTree::Literal(l) => {
                            let text = format!("{}", l);
                            let text = text.trim();
                            Some(text[1..(text.len() - 1)].to_owned())
                        },
                        _ => None
                    }
                }).collect();
                texts
            }).collect();
    
            let function_pointer = FunctionPointer {
                function_name: name,
                file_path: file_path.to_owned(),
                arguments,
                docs,
            };

            function_pointer
        })
        .collect::<Vec<FunctionPointer>>()
}

#[derive(Debug, Clone)]
pub struct GenericsArgument {
    type_name: String,
    is_reference: bool,
    sub_type: Vec<GenericsArgument>,
}

fn extrapolate_type(ty: Type) -> GenericsArgument {
    match &ty {
        Type::Path(type_path) => {
            let last = type_path.path.segments.last().unwrap();
            let type_name = format!("{}", &last.ident);
            let args = last.arguments.clone();

            let sub_type: Vec<_> = match &args {
                PathArguments::None => vec![],
                PathArguments::AngleBracketed(a) => a
                    .args
                    .clone()
                    .into_iter()
                    .map(|generic_argument| match generic_argument {
                        GenericArgument::Type(ty) => extrapolate_type(ty),
                        _ => unimplemented!("AA"),
                    })
                    .collect(),
                _ => unreachable!("only angle bracket are allowed here {:?}", args),
            };

            GenericsArgument {
                type_name,
                is_reference: false,
                sub_type,
            }
        }
        Type::Reference(r) => {
            let mut t = extrapolate_type(*r.clone().elem);
            t.is_reference = true;

            t
        }
        Type::Tuple(tuple) => {
            let sub_type = tuple
                .elems
                .clone()
                .into_iter()
                .map(|ty| extrapolate_type(ty))
                .collect::<Vec<_>>();

            GenericsArgument {
                type_name: "__tuple__".to_owned(),
                is_reference: false,
                sub_type,
            }
        }
        _ => unreachable!("{:?}", ty),
    }
}
