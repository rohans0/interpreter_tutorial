// ch5.4 - UNUSED FILE
// created for "practice" (oops)

use std::env;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::process;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        println!("Usage: ...");
        process::exit(64);
    }
    let output_dir: &str = &args[0];
    println!("{}",args[0]);
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right",
        ],
    )
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let path: String = format!("{}/{}.java", output_dir, base_name);
    let file = File::create(&path)?;
    let mut writer: BufWriter<File> = BufWriter::new(file);

    writeln!(writer, "package com.craftinginterpreters.lox;")?;
    writeln!(writer,)?;
    writeln!(writer, "import java.util.List;")?;
    writeln!(writer,)?;
    writeln!(writer, "abstract class {} {{", base_name)?;

    for t in types {
        let parts: Vec<&str> = t.split(':').collect();

        let class_name = parts[0].trim();
        let fields = parts[1].trim();

        define_type(&mut writer, base_name, class_name, fields)?;
    }

    writeln!(writer, "}}")?;
    writer.flush()?;

    Ok(())
}

fn define_type(
    writer: &mut BufWriter<File>,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> io::Result<()> {
    writeln!(
        writer,
        "  static class {} extends {} {{",
        class_name, base_name
    )?;

    writeln!(writer, "    {}({}) {{", class_name, field_list)?;

    let fields: Vec<&str> = field_list.split(", ").collect();
    for field in &fields {
        let name: &str = field.split(' ').nth(1).unwrap();
        writeln!(writer, "      this.{} = {};", name, name)?;
    }
    writeln!(writer, "    }}\n")?;

    for field in &fields {
        writeln!(writer, "     final {};", field)?;
    }

    writeln!(writer, "    }}")?;
    Ok(())
}
