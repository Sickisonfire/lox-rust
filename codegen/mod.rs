use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::{fs, path::Path};

pub fn generate_ast() -> Result<(), Box<dyn Error>> {
    let out_dir = "src";
    let base_name = "Expr";
    let types = vec![
        "Binary : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal : Option<Literal> value",
        "Unary : Token operator, Expr right",
    ];

    let dest_path = Path::new(out_dir).join(base_name);
    define_ast(base_name, types)?;
    Ok(())
}

fn define_ast(base_name: &str, types: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    // imports
    writeln!(&mut s, "")?;

    for ttype in types {
        let (class_name, fields) = ttype.split_once(":").unwrap();
        define_type(&mut s, base_name, class_name.trim(), fields.trim())?;
    }

    // enums
    Ok(())
}

fn define_type(
    w: &mut String,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> Result<(), Box<dyn Error>> {
    let mut fields: Vec<&str> = field_list.split(",").collect();
    let names: Vec<_> = fields
        .iter_mut()
        .map(|x| x.split_once(" ").unwrap().0)
        .collect();

    for field in fields {
        writeln!(w, "   {}", field)?;
    }
    Ok(())
}
