#![allow(dead_code)]
use std::fmt;

#[repr(C)]
#[derive(Debug)]
pub enum EquationSol{
    ConstantNoSol = 0,
    ConstantInfiniteSol = 1,
    LinearOneSol = 2,
    QuadraticNoSol = 3,
    QuadraticOneSol = 4,
    QuadraticTwoSol = 5,
}

#[repr(C)]
#[derive(Debug)]
pub struct Polyn{
    pub a : f64,
    pub b : f64,
    pub c : f64,
    pub type_: EquationSol,
    pub delta: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[link(name = "polylib")]
unsafe extern "C"{
    pub fn solve(a: f64, b: f64, c: f64) -> Polyn;
}

impl fmt::Display for Polyn{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_
        {
            EquationSol::ConstantNoSol => write!(f, "This is a constant polynom with no solution {} \u{2260} 0.\nS = {{\u{2205}}}", self.c),
            EquationSol::ConstantInfiniteSol => write!(f, "This a constant zero polynom 0 = 0.\nS = \u{211D}"),
            EquationSol::LinearOneSol => write!(f, "This is a linear polynom with one solution x = {}/{}.\nS = {{{}}}",-self.c, self.b ,self.x1),
            EquationSol::QuadraticNoSol => write!(f, "This is a quadratic polynom with determinant \u{0394} = {} < 0.\nThere are two complex solutions:\n z1 = {}/{} + i\u{221A}{}/{} and z2 = {}/{} - i\u{221A}{}/{}\nS = {{{}{:+}i, {}{:+}i}}",
                 self.delta, -self.b, 2.0*self.a, -self.delta, 2.0*self.a,-self.b, 2.0*self.a, -self.delta, 2.0*self.a, self.x1, self.y1, self.x2, self.y2),
            EquationSol::QuadraticOneSol => write!(f, "This is a quadratic polynom with determinant \u{0394} = {} = 0. There is one real solution:\n x = {}/{}.\nS = {{{}}}", self.delta, -self.b,2.0*self.a, self.x1),
            EquationSol::QuadraticTwoSol => write!(f, "This is a quadratic polynom with determinant \u{0394} = {} > 0. There are two real solutions:\n
            x1 = {}/{} + \u{221A}{}/{} and x2 = {}/{} - \u{221A}{}/{}\nS = {{{}, {}}}", self.delta, self.b, 2.0*self.a,  self.delta, 2.0*self.a,self.b, 2.0*self.a,  self.delta, 2.0*self.a,  self.x1, self.x2),
        }

    }   
}