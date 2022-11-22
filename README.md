# Format Comma library

* Add Comma
* Add Comma for decimal
* Remove Comma
* Remove Comma for decimal

## Installation

```
cargo add fmt_comma
```

## Example(Add comma)
```
use fmt_comma::format;

fn main() {
    let pc_price: f32 = 9800000.0;
    let company_a_send_cost: f32 = 1200.0;
    let company_a_pc_price = pc_price * 0.8 + company_a_send_cost;
    let company_b_pc_price = pc_price * 0.9;
    println!("Company A:{}yen", format(company_a_pc_price as isize));    // → 7,841,200yen
    println!("Company B:{}yen", format(company_b_pc_price as isize));    // → 8,820,000yen
}
```

## Example(Add comma for decimal)
```
use fmt_comma::format_decimal;

fn main() {
    println!("Company C:{}yen", format_decimal(10003.333));  // → 10,003.333yen
    println!("Company D:{}yen", format_decimal(3380.2));     // → 3,380.2yen
    println!("Company E:{}yen", format_decimal(3380.0));     // → 3,380yen
}
```

## Example(Remove comma)
```
use fmt_comma::rm_comma;

fn main() {
    println!("Company F:{}yen", rm_comma("10,003"));               // → 10003yen
}
```

## Example(Remove comma for decimal)
```
use fmt_comma::rm_comma_decimal;

fn main() {
    println!("Company G:{}yen", rm_comma_decimal("10,003.333"));   // → 10003.333yen
}
```