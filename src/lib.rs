pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/glucose_bindings.rs"));
}

use bindings::CGlucose;
use bindings::cglucose_init;
use bindings::cglucose_assume;
use bindings::cglucose_add_to_clause;
use bindings::cglucose_commit_clause;
use bindings::cglucose_clean_clause;
use bindings::cglucose_solve;
use bindings::cglucose_val;
use bindings::cglucose_solver_nodes;
use bindings::cglucose_nb_learnt;
use bindings::cglucose_set_random_seed;

pub fn init_glucose_solver() -> *mut CGlucose {
    let s : *mut CGlucose =  unsafe { cglucose_init() };
    s
}

pub fn add_assumptions_to_glucose_solver(s : *mut CGlucose, assumptions : Vec<i32>){
    unsafe {
        for i in assumptions{
            cglucose_assume(s, i);
        }
    } 
}

pub fn run_glucose(s : *mut CGlucose) -> i32 {
    let ret = unsafe { cglucose_solve(s) };
    ret
} 

pub fn add_clause_to_glucose_solver(s : *mut CGlucose, given : Vec<i32>){
    unsafe { 
        cglucose_clean_clause(s);
        for i in given{
            cglucose_add_to_clause(s, i);
        }
        cglucose_commit_clause(s);
    }
}

pub fn get_glucose_solver_stats(s : *mut CGlucose) -> u64 {
    let nodes = unsafe { cglucose_solver_nodes(s) };
    nodes
}

/// Gets a solution from Glucose solver while using the given nb_vars 
/// to allocate a new Rust solution vec to write and return.
pub fn get_glucose_solution(s : *mut CGlucose, nb_vars : usize) -> Vec<i32>{
    let mut model : Vec<i32> = Vec::with_capacity(nb_vars);
    for i in 1..nb_vars+1{
        let b = unsafe { cglucose_val(s, (i-1) as i32)}; 
        // #define l_True  (Glucose::lbool((uint8_t)0)) 
        // #define l_False (Glucose::lbool((uint8_t)1))
        // #define l_Undef (Glucose::lbool((uint8_t)2))
        if b == 0 { 
            model.push(i as i32);
        } else if b == 1 {
            model.push(-(i as i32));
        } else if b == 2 {
            panic!("Model has an undefined value!");
        }
    }
    model
}

/// Gets a solution from Glucose solver and writes on to the given Vector.
/// No memory allocation is done during calling this.
pub fn get_glucose_solution_no_malloc(s : *mut CGlucose, model : &mut Vec<i32>){
    model.clear();
    let cap = model.capacity();
    for i in 1..cap+1{
        let b = unsafe { cglucose_val(s, (i-1) as i32)}; 
        // #define l_True  (Glucose::lbool((uint8_t)0)) 
        // #define l_False (Glucose::lbool((uint8_t)1))
        // #define l_Undef (Glucose::lbool((uint8_t)2))
        if b == 0 { 
            model.push(i as i32);
        } else if b == 1 {
            model.push(-(i as i32));
        } else if b == 2 {
            panic!("Model has an undefined value!");
        }
    }
}

pub fn set_glucose_rnd_seed(s : *mut CGlucose, seed: f64){
    unsafe { cglucose_set_random_seed(s, seed) };
}

pub fn get_glucose_solver_nb_learnt(s : *mut CGlucose) -> u64 {
    return unsafe { cglucose_nb_learnt(s) };
}