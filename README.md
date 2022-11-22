# Format Comma library

* Add Comma
* Remove Comma

## Installation

```
cargo add fmt_comma
```

## Example(Add comma)
```
fn main() {
    let pc_price: f32 = 9800000.0;
    let company_a_send_cost: f32 = 1200.0;
    let company_a_pc_price = pc_price * 0.8 + company_a_send_cost;
    let company_b_pc_price = pc_price * 0.9;
    println!("Company A:{}yen", fmt_comma(company_a_pc_price as isize));    // → 7,841,200yen
    println!("Company B:{}yen", fmt_comma(company_b_pc_price as isize));    // → 8,820,000yen
    println!("Company C:{}yen", fmt_comma_decimal(10003.333));  // → 10,003.333yen
    println!("Company D:{}yen", fmt_comma_decimal(3380.2));     // → 3,380.2yen
    println!("Company E:{}yen", fmt_comma_decimal(3380.0));     // → 3,380yen
}
```

## Example(Remove comma)
```
fn main() {
    println!("F社:{}円", rm_comma("10,003"));               // → 10003yen
    println!("G社:{}円", rm_comma_decimal("10,003.333"));   // → 10003.333yen
}
```