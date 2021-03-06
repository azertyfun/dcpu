use std::collections::HashMap;

use assembler::types::*;

#[derive(Debug)]
pub enum Error {
    UnknownLabel(String),
    UnknownLocalLabel(String),
    DuplicatedLabel(String),
    DuplicatedLocalLabel(String),
    LocalBeforeGlobal(String),
}

pub fn link(ast: &[ParsedItem]) -> Result<Vec<u16>, Error> {

    let mut bin = Vec::new();
    let (mut globals, mut locals) = try!(extract_labels(ast));
    let mut last_global = None;
    let mut changed = true;

    while changed {
        changed = false;
        let mut index = 0u16;
        for item in ast {
            match *item {
                ParsedItem::Directive(ref d) => index += d.append_to(&mut bin),
                ParsedItem::LabelDecl(ref s) => {
                    let ptr = globals.get_mut(s).unwrap();
                    if *ptr != index {
                        *ptr = index;
                        changed = true;
                    }
                    last_global = Some(s);
                }
                ParsedItem::LocalLabelDecl(ref s) => {
                    let ptr = locals.get_mut(*last_global.as_ref().unwrap())
                                    .unwrap()
                                    .get_mut(s)
                                    .unwrap();
                    if *ptr != index {
                        changed = true;
                        *ptr = index;
                    }
                }
                ParsedItem::ParsedInstruction(ref i) => {
                    let solved = match last_global {
                        Some(ref s) => try!(i.solve(&globals, locals.get(*s).unwrap())),
                        None => try!(i.solve(&globals, &HashMap::new())),
                    };
                    bin.extend(&[0xbeaf; 3]);
                    index += solved.encode(&mut bin[index as usize..]);
                    bin.truncate(index as usize);
                }
                _ => (),
            }
        }
    }

    Ok(bin)
}

fn extract_labels
    (ast: &[ParsedItem])
     -> Result<(HashMap<String, u16>, HashMap<String, HashMap<String, u16>>), Error> {
    let mut prev_label = None;
    let mut globals = HashMap::new();
    let mut locals = HashMap::new();

    for item in ast.iter() {
        match *item {
            ParsedItem::LabelDecl(ref s) => {
                prev_label = Some(s.clone());
                if globals.contains_key(s) {
                    return Err(Error::DuplicatedLabel(s.clone()));
                } else {
                    globals.insert(s.clone(), 0);
                    locals.insert(s.clone(), HashMap::new());
                }
            }
            ParsedItem::LocalLabelDecl(ref s) => {
                if prev_label.is_none() {
                    return Err(Error::LocalBeforeGlobal(s.clone()));
                }
                let locals = locals.get_mut(prev_label.as_ref().unwrap()).unwrap();
                if locals.contains_key(s) {
                    return Err(Error::DuplicatedLocalLabel(s.clone()));
                } else {
                    locals.insert(s.clone(), 0);
                }
            }
            _ => (),
        }
    }

    Ok((globals, locals))
}
