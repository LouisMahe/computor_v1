use std::collections::{HashMap};
use regex::Regex;


fn parse_side(side : &str, coeff: &mut HashMap<usize, f64>, sign : f64) -> Result<(), Box <dyn std::error::Error>>
{
    let re = Regex::new(r"\s*([+-]?)\s*(\d*(?:\.\d+)?|\.\d+)\s*\*\s*X\^(\d+)").unwrap();

    for cap in re.captures_iter(side)
    {
        let sign_str = cap.get(1).map_or("+", |m| m.as_str());
        let coeff_str = cap.get(2).unwrap().as_str();
        let power_str = cap.get(3).unwrap().as_str();

        let s =if sign_str == "-" {-sign} else {sign};
        let c = coeff_str.parse::<f64>()?;
        let power = power_str.parse::<usize>()?;

        let e = coeff.entry(power).or_insert(0.0);
        *e += c * s;
    }
    Ok(())
}

pub fn parse_poly(line : &str) -> Result<Vec<(usize, f64)>, Box<dyn std::error::Error>>
{
    let mut coeff : HashMap<usize, f64>   = HashMap::new();
    let sides: Vec<&str> = line.split('=').collect();
    parse_side(sides[0], &mut coeff, 1.0)?;
    if sides.len() > 2{ return Err("Equation parsing error".into());}
    if sides.len() == 2 && sides[1].trim() != "0" {
        parse_side(sides[1], &mut coeff, -1.0)?;
    }
    coeff.retain(|_p, c| *c != 0.0_f64);
    let mut monoms : Vec<(usize, f64)> = coeff.into_iter().collect();
    monoms.sort_by(|x,y| y.0.cmp(&x.0));
    
    Ok(monoms)
}