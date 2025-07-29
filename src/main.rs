mod ffi;
mod parse;
use ffi::{solve};
use parse::parse_poly;
use std::env;
use plotly::common::{Mode,Line};
use plotly::{Plot, Scatter,};


#[derive(Debug)]
struct Poly{
    pub degree : usize,
    pub stringrep : String,
    pub coeffs : Vec<f64>
}

fn monoms_to_string(monoms: &Vec<(usize, f64)>) -> Poly {
    
    let degree = if monoms.is_empty() {0} else {monoms.first().unwrap().0};
    let mut as_str= if monoms.is_empty() {"0*X^0".to_string()} else { monoms.iter().map(|(p,c)| format!("{:+}*X^{}",c,p)).collect::<Vec<_>>().join(" ")};
    as_str += " = 0";
    if as_str.starts_with('+')
    {
        as_str.remove(0);
    }
    let mut coeffs = vec![0.0_f64; std::cmp::max(degree+1,3)];
    for (p,c) in monoms.iter()
    {
        coeffs[*p] = *c;
    }
    Poly {degree, stringrep : as_str, coeffs}

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2{
        return  Err("Invalid argument".into());
    }
    let monoms = parse_poly(&args[1])?;
    let p = monoms_to_string(&monoms);
    println!("Reduced form: {}", p.stringrep);
    println!("Degree is: {}", p.degree);
    if p.degree < 3
    {
        let res = unsafe {
            solve(p.coeffs[2], p.coeffs[1], p.coeffs[0])
        };
        println!("{}", res);
        let poly = |x: f64| p.coeffs[2] * x * x + p.coeffs[1] * x + p.coeffs[0];
 
        let x_vals: Vec<f64> = (-100..=100).map(|x| x as f64 / 10.0).collect();
        let y_vals: Vec<f64> = x_vals.iter().map(|&x| poly(x)).collect();
        let zeros = Scatter::new(x_vals.clone(), vec![0.0f64; x_vals.len()]).mode(Mode::Lines).line(Line::new().color("red")).name("y = 0");
        let trace = Scatter::new(x_vals, y_vals)
            .mode(Mode::Lines)
            .name("Polynomial");

        let mut plot = Plot::new();
        plot.set_layout(
            plotly::Layout::new()
                .y_axis(plotly::layout::Axis::new().range(vec![-10.0, 10.0])) // adjust to your range
        );
        plot.add_trace(trace);
        plot.add_trace(zeros);
        
        // plot.show_html("poly_plot.html");
        open::that("poly_plot.html")?;
    }
    else{
        println!("Degree is too high, I don't know how to solve.")
    }
    
    Ok(())
}
