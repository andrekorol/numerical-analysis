// Implementation of the Bisection method algorithm as found on
// CENGAGE Learning's Numerical Analysis, Tenth Edition,
// by Richard L. Burden, J. Douglas Faires, Annete M. Burden.

// Copyright © 2019 Andre Rossi Korol <anrobits@yahoo.com.br>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

#[macro_use]
extern crate text_io;

extern crate meval;

fn main() {
    println!("Enter the function expression (e.g., x - 2 * sin(x)): ");
    let expression_str: String = read!("{}\n");

    println!("Enter the first interval endpoint (a): ");
    let mut a: f64 = read!();

    println!("Enter the second interval endpoint (b): ");
    let mut b: f64 = read!();

    println!("Enter the tolerance (ε): ");
    let tol: f64 = read!();

    println!("Enter the maximum number of iterations (N₀): ");
    let max_iter: usize = read!();

    let expr: meval::Expr = expression_str.parse().unwrap();
    let func = expr.bind("x").unwrap();

    let mut p: f64;
    let mut fp: f64;
    let dim: f64 = 1.0 / tol;
    let mut rounded_fp: f64;
    let mut prev_p_opt: Option<f64> = None;

    // Step 1
    let mut i: usize = 1;
    let mut fa: f64 = func(a);
    // Step 2
    while i <= max_iter {
        // Step 3
        p = a + (b - a) / 2.0;
        fp = func(p);
        // Step 4
        rounded_fp = (fp * dim).round() / dim;
        match prev_p_opt {
            None => {
                if rounded_fp == 0.0 {
                    println!("{}", p);
                    return;
                }
            }
            Some(prev_p) => {
                if (p - prev_p).abs() / p.abs() < tol {
                    println!("{}", p);
                    return;
                }
            }
        }
        // Step 5
        i += 1;
        // Step 6
        if fa * fp > 0.0 {
            a = p;
            fa = fp;
        } else {
            b = p;
        }
        prev_p_opt = Some(p);
    }
    // Step 7
    println!("Method failed after {} iterations", max_iter);
}
