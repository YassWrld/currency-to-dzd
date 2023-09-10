use std::env;

#[derive(PartialEq)]
struct Currency<'a> {
    name: &'a str,
    value: f32,
}

impl<'a> Currency<'a> {
    const fn new(code: &'a str, price: f32) -> Self {
        Currency {
            name: code,
            value: price,
        }
    }
}

const CURRENCIES: [Currency; 3] = [
    Currency::new("USD", 210f32),
    Currency::new("EUR", 220f32),
    Currency::new("GBP", 250f32),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let sum = &args[1];
    let sum: f32 = sum
        .trim()
        .parse()
        .expect("ERROR: Please introduce a correct value!");
    let unit = &args[2];
    let unit = unit.to_uppercase();
    let unit = unit.as_str();
    let unit = get_currency(unit);
    print_converted(calculate(sum, unit));
}

fn get_currency(unit: &str) -> Currency {
    for i in CURRENCIES {
        if unit == i.name {
            return i;
        }
    }
    panic!("ERROR: This currency does not exist!");
}

fn calculate(sum: f32, unit: Currency) -> u32 {
    if CURRENCIES.contains(&unit) {
        return (sum * unit.value) as u32;
    }
    panic!("ERROR: This currency does not exist!");
}

fn print_converted(mut amount: u32) {
    print!("{} dzd AKA: ", amount);
    let b = amount / 10_000_000; // mlyar
    amount = amount % 10_000_000;
    let m = amount / 10_000; // mlyoun
    amount = amount % 10_000;
    let k = (amount / 1_000) * 100; // 100alf
    let d = amount % 1_000;
    // println!("{} mlyar {} mlyoun {} alf {} dzd", b, m, k, d);
    if b != 0 {
        print!("{} mlyar", b);
    }
    if m != 0 {
        print!(" {} mlyoun", m);
    }
    if k != 0 {
        print!(" {} alf", k);
    }
    if d != 0 {
        print!(" {}", d);
    }
    println!(" dzd");
}
