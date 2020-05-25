use std::env;

// prints an ascii table from 32 to 126

fn print_table(columns: usize) {
    let range = 32..128;
    let rows_raw = range.len() as f32 / columns as f32;
    let rows = rows_raw.round() as usize;

    for i in 0..rows {
        for j in 0..columns {
            let byte = (i + j * rows + 32) as u8;
            if byte >= 127 {
                if columns == 1 {
                    return;
                }
                break;
            }

            let num_fill =
                // last number of this row
               if 32 + (j + 1) * rows - 1 >= 100 {
                   3
               } else {
                   2
               };

            print!("'{0}' {1:2$} 0x{1:2x}",
                   byte as char,
                   byte,
                   num_fill,);
            if j != columns - 1 {
                print!(" | ");
            }
        }
        println!("");
    }
}

fn usage() {
    println!("usage: ashy [columns (default 3)]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let columns = args.get(1);
    match columns {
        Some(arg) => {
            if let Ok(val) = arg.parse::<usize>() {
                print_table(val);
            } else {
                println!("cannot parse \"{}\" as a number", arg);
                usage();
            }
        },
        None => print_table(3)
    }
}
