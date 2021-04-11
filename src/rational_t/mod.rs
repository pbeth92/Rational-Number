use std::io;
use num::{FromPrimitive, Num, Signed, traits::Zero};
use std::fmt;
use std::ops:: { Div, Mul, Add, Sub, Neg };


/// Rational<T>
/// 
/// Estructura para definir un número racional que usa tipos genéricos.
/// #[derive] Atributo para implementar traits básicos
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rational<T> {
    numerator: T,
    denominator: T,
}

/// Instancias para tipos concretos:
///
/// Métodos para iniciar estructuras por defecto
/// Métodos para iniciar estructuras mediante entrada de teclado
impl Rational<f32> {
    pub fn default_f() -> Self {
        Self {
            numerator: 0.0,
            denominator: 1.0,
        }
    }

    pub fn read_f() -> Self {
        let mut n: String = String::new();
        let mut d: String = String::new();
        
        println!("Numerador:");
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");

        println!("Denominador:");
        io::stdin()
            .read_line(&mut d)
            .expect("Failed to read line.");
        
        Self {
            numerator: n.trim().parse().unwrap(),
            denominator: d.trim().parse().unwrap(),
        }
    }
}

impl Rational<i32> {
    pub fn default_i() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    pub fn read_i() -> Self {
        let mut n: String = String::new();
        let mut d: String = String::new();
        
        println!("Numerador:");
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");

        println!("Denominador:");
        io::stdin()
            .read_line(&mut d)
            .expect("Failed to read line.");
        
        Self {
            numerator: n.trim().parse().unwrap(),
            denominator: d.trim().parse().unwrap(),
        }
    }
}

/// Instancias para el tipo genéricos. Rational<T>
///
/// Métodos:
/// - Crear un Rational<T>
/// - Transformaciones
impl<T> Rational<T>
where T: Num + PartialOrd + PartialEq + Copy + FromPrimitive + Signed + Zero,
{
    pub fn new(n: T, d: T) -> Result<Self, &'static str> {
        if num::Zero::is_zero(&d) {
            Err("El denominador no puede ser 0")
        } else {
            Ok(
                Self {
                    numerator: n,
                    denominator: d,
                }
            )   
        }
    }

    pub fn num(&self) -> &T {
        &self.numerator
    }

    pub fn den(&self) -> &T {
        &self.denominator
    }

    pub fn set_num(&mut self, num: T) {
        self.numerator = num
    }

    pub fn set_den(&mut self, den: T) {
        if num::Zero::is_zero(&den) {
            println!("Error: El denominador no puede ser 0");
        } else {
            self.denominator = den
        }
    }

    pub fn opposite(&self) -> Self {
        self.neg()
    }

    pub fn reciprocal(&self) -> Self {
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}


/// Método auxiliar para obtener el valor absoluto de un tipo genérico
pub fn abs<T>(x: T) -> T
where
    T: Signed + Zero + Neg + Copy + PartialOrd,
{
    match x {
        x if x < T::zero() => -x,
        _ => x,
    }
}

// Métodos para las operaciones de comparación
pub trait GetValue<T> {
    fn get_value(&self) -> T;
    fn equal(&self, r: &Self, precision: f32) -> bool;
    fn greater(&self, r: &Self, precision: f32) -> bool;
    fn less(&self, r: &Self, precision: f32) -> bool;
    fn cero(&self, precision: f32) -> bool;
}

impl<T> GetValue<T> for Rational<T> 
where 
    T: Num + Copy + PartialOrd + FromPrimitive + Signed,
{
    fn get_value(&self) -> T {
        self.numerator / self.denominator
    }

    fn equal(&self, r: &Self, precision: f32) -> bool {
        abs(self.get_value() - r.get_value()) < T::from_f32(precision).unwrap()
    }

    fn greater(&self, r: &Self, precision: f32) -> bool {
        self.get_value() - r.get_value() > T::from_f32(precision).unwrap()
    }

    fn less(&self, r: &Self, precision: f32) -> bool {
        self.get_value() - r.get_value() < T::from_f32(precision).unwrap()
    }

    fn cero(&self, precision: f32) -> bool {
        abs(self.get_value()) < T::from_f32(precision).unwrap()
    }
}

/// Implementación de Displat para Rational<T>
impl<T: fmt::Display> fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}/{:.2}", self.numerator, self.denominator)
    }
}


/// Sobrecarga de operadores. 
/// operator /
///     - Num o Mul<Output = T>
impl<T: Num> Div for Rational<T> {
    type Output = Self;

    fn div(self, r: Self) -> Self::Output {    
        Self {
            numerator: self.numerator * r.denominator,
            denominator: self.denominator * r.numerator,
        }
    }
}

/// operator *
///     - Num o Mul<Output = T>
impl<T: Mul<Output = T>> Mul for Rational<T> {
    type Output = Self;

    fn mul(self, r: Self) -> Self::Output {    
        Self {
            numerator: self.numerator * r.numerator,
            denominator: self.denominator * r.denominator,
        }
    }
}

/// operator +
impl<T> Add for Rational<T> 
where
    T: Num + Copy + PartialOrd,
{
    type Output = Self;

    fn add(self, r: Self) -> Self::Output {    
        Self {
            numerator: {
                if self.denominator == r.denominator {
                    self.numerator + r.numerator
                } else {
                    self.numerator * r.denominator + r.numerator * self.denominator
                }
            },
            denominator: {
                if self.denominator == r.denominator {
                    self.denominator
                } else {
                    self.denominator * r.denominator
                }
            }
        }
    }
}

/// operator -
impl<T> Sub for Rational<T> 
where
    T: PartialOrd + Num + Copy,
{
    type Output = Self;

    fn sub(self, r: Self) -> Self::Output {    
        Self {
            numerator: {
                if self.denominator == r.denominator {
                    self.numerator - r.numerator
                } else {
                    self.numerator * r.denominator - r.numerator * self.denominator
                }
            },
            denominator: {
                if self.denominator == r.denominator {
                    self.denominator
                } else {
                    self.denominator * r.denominator
                }
            }
        }
    }
}

/// Operador unario -
impl<T> Neg for Rational<T>
where
    T: Signed + Num + Copy + FromPrimitive,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numerator: - self.numerator, //* T::from_i32(-1).unwrap(),
            denominator: self.denominator,
        }
    }
}

#[cfg(test)]
mod test {
   use super::*;

    #[test]
    fn create_rational() {
        let rat = Rational::new(3.2, 2.3);
        assert!(rat.is_ok());
        
        let rat = Rational::new(4, 0); 
        assert!(rat.is_err());
   }

   #[test]
   fn test_operations() {
        let a = Rational::new(3, 2);
        let b = Rational::new(5, 4);
   
        let a = match a {
            Ok(rational) => rational,
            Err(error) => panic!("Error: {:?}", error),
        };
   
        let b = match b {
            Ok(rational) => rational,
            Err(error) => panic!("Error: {:?}", error),
        };

        let mut rat = Rational::default_i();
        rat.set_num(15);
        rat.set_den(8);
        assert_eq!(a*b, rat);

        rat.set_num(12);
        rat.set_den(10);
        assert_eq!(a/b, rat);

        rat.set_num(22);
        rat.set_den(8);
        assert_eq!(a+b, rat);

        rat.set_num(2);
        rat.set_den(8);
        assert_eq!(a-b, rat);
    }

    #[test]
    fn test_comparations() {
        let a = Rational::new(1, 2);
        let b = Rational::new(6, 4);

        let a = match a {
            Ok(rational) => rational,
            Err(error) => panic!("Error: {:?}", error),
        };
        let b = match b {
            Ok(rational) => rational,
            Err(error) => panic!("Error: {:?}", error),
        };
        
        assert!(a.less(&b, 0.0001));
        assert_ne!(a.greater(&b, 0.0001), true);
        assert_eq!(a.equal(&b, 0.0001), false);
    }

}