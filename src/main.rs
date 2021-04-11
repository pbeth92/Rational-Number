pub mod rational_t;
use crate::rational_t::GetValue;

const EPSILON: f32 = 1e-6;

fn main() {
    let a = rational_t::Rational::new(3.2, 2.3);
    let b = rational_t::Rational::new(5.2, 4.4);

    let mut a = match a {
        Ok(rational) => rational,
        Err(error) => panic!("Error: {:?}", error),
    };

    let mut b = match b {
        Ok(rational) => rational,
        Err(error) => panic!("Error: {:?}", error),
    };

    // TRANSFORMACIONES
    println!("TRANSFORMACIONES");
    // Modificar un valor
    a.set_den(7.1);
    println!("a: {}", a);
    b.set_den(0.0);
    // Opuesto
    println!("El opuesto de {} es: {}", a, -a); //println!("{}", a.opposite());
    // Reciproco
    println!("El reciproco de {} es {}", a, a.reciprocal());
    
    // COMPARACIONES
    println!("COMPARACIONES");
    // a == b
    if a.equal(&b, EPSILON) {
        println!("{} y {} son iguales", a, b)
    } else {
        println!("{} y {} no son iguales", a, b)
    }
    // a > b
    if a.greater(&b, EPSILON) {
        println!("{} es mayor que {}", a, b)
    } else {
        println!("{} no es mayor que {}", a, b)
    }
    // a < b
    if a.less(&b, EPSILON) {
        println!("{} es menor que {}", a, b)
    } else {
        println!("{} no es menor que {}", a, b)
    }
    // a = 0
    if a.cero(EPSILON) {
        println!("{} no es igual a 0", a)
    } else {
        println!("{} no es igual a 0", a)
    }

    // OPERACIONES ARITMÉTICAS
    println!("OPERACIONES ARITMÉTICAS");
    // Suma:
    let suma = a + b;
    println!("{} + {} = {} == {:.2}", a, b, suma, suma.get_value());
    // Resta:
    let resta = a - b;
    println!("{} - {} = {} == {:.2}", a, b, resta, resta.get_value());
    // Multiplicación
    let mul = a * b;
    println!("{} * {} = {} == {:.2}", a, b, mul, mul.get_value());
    // División
    let div = a / b;
    println!("{} / {} = {} == {:.2}", a, b, div, div.get_value());

    // Métodos para tipos concretos:
    // let mut c = rational_t::Rational::read_f();
    // let mut d = rational_t::Rational::read_i();
    //
    // c.set_num(7.0);
    // println!("{}", -c);
    // d.set_den(6);
}
