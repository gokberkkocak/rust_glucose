pub mod bindings {
    // #![allow(non_upper_case_globals)]
    // #![allow(non_camel_case_types)]
    // #![allow(non_snake_case)]
    // #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/glucose_bindings.rs"));
}

// use bindings::root::getASolver;
// use bindings::root::Glucose::SimpSolver;
// use bindings::root::Glucose::SimpSolver_totalTime;
// use bindings::root::Glucose::SimpSolver_clearAssumptions;
// use bindings::root::Glucose::SimpSolver_addToAssumptionsVec;
// use bindings::root::Glucose::SimpSolver_solveWithAssumpLink;
// use bindings::root::Glucose::SimpSolver_getNbLearnt;
// use bindings::root::Glucose::Lit;
// use bindings::root::Glucose::SimpSolver_addClauseLink;
// use bindings::root::Glucose::SimpSolver_addClauseLink1;
// use bindings::root::Glucose::SimpSolver_addClauseLink2;
// use bindings::root::Glucose::SimpSolver_cleanTmpClauseVec;
// use bindings::root::Glucose::SimpSolver_addToTmpClause;
// use bindings::root::Glucose::SimpSolver_addTmpClause;

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

// .whitelist_function("cglucose_init")
// .whitelist_function("cglucose_assume")
// .whitelist_function("cglucose_solve")
// .whitelist_function("cglucose_val")
// .whitelist_function("cglucose_add_to_clause")
// .whitelist_function("cglucose_commit_clause")
// .whitelist_function("cglucose_clean_clause")
// .whitelist_function("cglucose_solver_nodes")
// .whitelist_function("cglucose_nb_learnt")
// .whitelist_function("cglucose_get_nodes")

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
    // no simplification and turn off simplification. Otherwise new searches are not possible.
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

// pub fn get_glucose_solver_stats(s : *mut CGlucose) -> Vec<f64> {
//     let mut stats : Vec<f64> = Vec::with_capacity(4);
//     unsafe {
//         //time, decisions,propagations,conflicts;
//         stats.push(SimpSolver_totalTime(s));
//         stats.push( (*s)._base.decisions as f64);
//         stats.push( (*s)._base.propagations as f64);
//         stats.push( (*s)._base.conflicts as f64);
//     }
//     stats
// }

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

// pub fn get_glucose_solution(s : *mut CGlucose) -> Vec<i32>{
//     let model : Vec<bindings::root::Glucose::lbool> = unsafe { Vec::from_raw_parts((*s)._base.model.data, (*s)._base.model.sz as usize, (*s)._base.model.cap as usize) };
//     let mut rust_model : Vec<i32> = Vec::with_capacity(model.len());
//     let mut value : i32 = 0;
//     for i in &model {
//         // #define l_True  (Glucose::lbool((uint8_t)0)) 
//         // #define l_False (Glucose::lbool((uint8_t)1))
//         // #define l_Undef (Glucose::lbool((uint8_t)2))
//         value +=1;
//         if i.value == 0 {
//             rust_model.push(value);
//         }
//         else if i.value == 1 {
//             rust_model.push(-value);
//         }
//         else{
//             panic!("Model has an undefined value!");
//         }
//     }
//     //this object belongs to glucose. don't run destructor for it.
//     mem::forget(model);
//     rust_model
// }

pub fn set_glucose_rnd_seed(s : *mut CGlucose, seed: f64){
    unsafe { cglucose_set_random_seed(s, seed) };
}

pub fn get_glucose_solver_nb_learnt(s : *mut CGlucose) -> u64 {
    return unsafe { cglucose_nb_learnt(s) };
}