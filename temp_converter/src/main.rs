mod convert;

use convert::{convert, parse_scale};

use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3
    {
        println!("Usage: convert <value> <C|F>");
        return;
    };

    let value: f64 = match args[1].parse()
    {
        Ok(v) => v,
        Err(_) =>
        {
            println!("Invalid number: {}", args[1]);
            return;
        }
    };

    let scale = match parse_scale(&args[2])
    {
        Some(s) => s,
        None =>
        {
            println!("Invalid scale: {}", args[2]);
            return;
        }
    };

    let result = convert(value, &scale);
    let out_scale = match scale
    {
        convert::Scale::Celsius => "F",
        convert::Scale::Fahrenheit => "C",
    };

    println!("Result: {:.2} {}", result, out_scale);
}