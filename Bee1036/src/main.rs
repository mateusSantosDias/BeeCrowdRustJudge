use std::io;

fn convert_to_float(data_input: &String) -> f64{
    let x: f64 = data_input
                 .trim()
                 .parse::<f64>()
                 .unwrap();
    return x;
}

fn get_value() -> String{
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Erro");
    
    return value;
}

fn baskara_function_delta(value1: &f64, value2: &f64, value3: &f64) -> f64{
   let delta = value2.powf(2.0) - 4.0 * value1 * value3;
   return delta;
}


fn main() {
    
    let _values__: Vec<String> = get_value()
        .split_whitespace()
        .map(|s| s.parse::<String>()
        .expect("Erro em Converter"))
        .collect();


    let a: f64 = convert_to_float(&_values__[0]);
    let b: f64 = convert_to_float(&_values__[1]);
    let c: f64 = convert_to_float(&_values__[2]);

    let x1: f64;
    let x2: f64;

    x1 = (- b + baskara_function_delta(&a, &b, &c).sqrt())/(2.0*a);
    x2 = (- b - baskara_function_delta(&a, &b, &c).sqrt())/(2.0*a);
    
     
    if x1.is_nan() || x2.is_nan(){
        println!("Impossivel calcular");
    }
    else {
        println!("R1 = {:.5}", x1);
        println!("R2 = {:.5}", x2);
    }
    
    //println!("{}", baskara_function_delta(&a, &b, &c));
}
