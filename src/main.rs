use std::f32::consts::E;

use textplots::*;
use text_io::*;
struct MedicationType{
    name: String,
    fit_dose: f32,
    d: f32,
    k1: f32,
    k2: f32,
    k3: f32,
}

fn main() {
    // define medication types, data from https://transfemscience.org/articles/injectable-e2-meta-analysis/
    let estradiol_valerate = MedicationType{
        name: "Estradiol valerate".to_string(),
        d: 2596.05956,
        fit_dose: 5.0,
        k1: 2.38229125,
        k2: 0.23345814,
        k3: 1.37642769,
    };
    let estradiol_cypionate = MedicationType{
        name: "Estradiol cypionate".to_string(),
        d: 1920.89671,
        fit_dose: 5.0,
        k1: 0.10321089,
        k2: 0.89854779,
        k3: 0.89359759,
    };
    let estradiol_enanthate = MedicationType{
        name: "Estradiol enanthate".to_string(),
        d: 333.874181,
        fit_dose: 5.0,
        k1: 0.42412968,
        k2: 0.43452980,
        k3: 0.15291485,
    };
    let current_medication:MedicationType; // declare variable to store analyzed medication type
    println!("Estradiol Pharmacokinetics Simulation");
    println!("--------------------------------------");
    println!("Select medication type:");
    println!("--------------------------------------");
    println!("1 - Estradiol valerate");
    println!("2 - Estradiol cypionate");
    println!("3 - Estradiol enanthate");
    let i: i32 = try_read!().unwrap_or(100);
    match i { // match user input to medication type
        1 => current_medication = estradiol_valerate,
        2 => current_medication = estradiol_cypionate,
        3 => current_medication = estradiol_enanthate,
        _ => {
            current_medication = estradiol_valerate;
            println!("Invalid input, defaulting to Estradiol valerate");},
    }
    println!("--------------------------------------");
    // prompt user for dose and simulation time, store in variables
    println!("Enter dose (in mg, the default dose for {} is {} mg):", current_medication.name, current_medication.fit_dose);
    let dose: f32 = try_read!().unwrap_or(current_medication.fit_dose);
    println!("--------------------------------------");
    println!("Enter simulation time (in days):");
    let simulation_time: f32 = try_read!().unwrap_or(7.0);
    // get parameters from medication type, making closure declaration less verbose
    let d: f32 = current_medication.d;
    let k1: f32 = current_medication.k1;
    let k2: f32 = current_medication.k2;
    let k3: f32 = current_medication.k3;
    // define closure for estradiol concentration over time
    let closure = |x: f32| dose/current_medication.fit_dose * (d * k1 * k2 * (E.powf(-1.0 * x * k1)/((k1-k2)*(k1-k3)) + E.powf(-1.0 * x * k3)/((k1-k3)*(k2-k3)) + (E.powf(-1.0 * x * k2)*(-1.0 * k1 + k3))/((k1-k2)*(k1-k3)*(k2-k3))));
    println!("--------------------------------------");
    println!("Estradiol concentration in blood (pg/mL) over time ({} mg {})", dose, current_medication.name);
    // use textplots crate to plot estradiol concentration over time
    Chart::new(120, 60, 0.0, simulation_time)
        .lineplot(&Shape::Continuous(Box::new(closure)))
        .display();
}
