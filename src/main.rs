fn dispense_change(mut amount:u32)-> Vec<u32>{
    let denomination = [1000,500,200,100,50,20,5];
    let mut result = Vec::new();

    for &d in denomination.iter(){
        while amount>= d{
            amount-=d;
            result.push(d);
        }
    }
    if amount !=0{
        panic!("Exact Change cannot be made");
    }
    result
}

fn main() {
    println!("Change Dispenser");

    let amount = 5000;
    let change = dispense_change(amount);

    println!("Change for {}:",amount);
    print!("{:?}",change);
}