use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn get_frac(file: File) -> (i32, i32){
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let numerator: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let denominator: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    println!("numerator: {}, denominator: {}", numerator, denominator);

    (numerator, denominator)
}


fn reduce_frac(mut numerator: i32, mut denominator: i32) -> (i32, i32){
    loop {
        if numerator % 7 == 0 && denominator % 7 == 0{
            numerator /= 7;
            denominator /= 7;
            continue;
        }
        if numerator % 5 == 0 && denominator % 5 == 0{
            numerator /= 5;
            denominator /= 5;
            continue;
        }
        if numerator % 3 == 0 && denominator % 3 == 0{
            numerator /= 3;
            denominator /= 3;
            continue;
        }
        if numerator % 2 == 0 && denominator % 2 == 0{
            numerator /= 2;
            denominator /= 2;
            continue;
        }
        break;
    }

    (numerator, denominator)
}


fn main() -> std::io::Result<()> {
    let file = File::open("FRACTION.IN")?;
    let origin = get_frac(file);
    let reduced = reduce_frac(origin.0, origin.1);

    let mut res = File::create("FRACTION.OUT")?;
    res.write_all(reduced.0.to_string().as_bytes())?;
    res.write_all("\n".as_bytes())?;
    res.write_all(reduced.1.to_string().as_bytes())?;

    println!("{}/{}", reduced.0, reduced.1);
    Ok(())
}
