use core::mem;

pub mod bindings {
    // #![allow(non_upper_case_globals)]
    // #![allow(non_camel_case_types)]
    // #![allow(non_snake_case)]
    // #![allow(dead_code)]
    include!("glucose_bindings.rs");
}

use bindings::root::getASolver;
use bindings::root::Glucose::SimpSolver;
use bindings::root::Glucose::SimpSolver_totalTime;
use bindings::root::Glucose::SimpSolver_clearAssumptions;
use bindings::root::Glucose::SimpSolver_addToAssumptionsVec;
use bindings::root::Glucose::SimpSolver_solveWithAssumpLink;
use bindings::root::Glucose::SimpSolver_getNbLearnt;
use bindings::root::Glucose::Lit;

pub fn init_glucose_solver() -> *mut SimpSolver {
    let s : *mut SimpSolver =  unsafe { getASolver() };
    s
}

pub fn add_assumptions_to_glucose_solver(s : *mut SimpSolver, assumptions : Vec<i32>){
    unsafe {
        for i in assumptions{
            SimpSolver_addToAssumptionsVec(s, make_lit(i));
        }
    } 
}

pub fn run_glucose(s : *mut SimpSolver) -> bool {
    // no simplification and turn off simplification. Otherwise new searches are not possible.
    let ret = unsafe { SimpSolver_solveWithAssumpLink(s, false, true) };
    unsafe { SimpSolver_clearAssumptions(s) };
    ret
} 

pub fn add_clause_to_glucose_solver(s : *mut SimpSolver, given : Vec<i32>) -> bool{
    let ret : bool;
    if given.len() == 1 {
        unsafe { solver_new_var(s, given[0]) };
        ret = unsafe { bindings::root::Glucose::SimpSolver_addClauseLink(s, make_lit(given[0])) };
    } else if given.len() == 2 {
        unsafe { solver_new_var(s, given[0]) };
        unsafe { solver_new_var(s, given[1]) };
        ret = unsafe { bindings::root::Glucose::SimpSolver_addClauseLink1(s, make_lit(given[0]), make_lit(given[1])) };
    } else if given.len() == 3 {
        unsafe { solver_new_var(s, given[0]) };
        unsafe { solver_new_var(s, given[1]) };
        unsafe { solver_new_var(s, given[2]) };
        ret = unsafe { bindings::root::Glucose::SimpSolver_addClauseLink2(s, make_lit(given[0]), make_lit(given[1]), make_lit(given[2])) };
    } else {
        unsafe { 
            bindings::root::Glucose::SimpSolver_cleanTmpClauseVec(s);
            for i in given{
                solver_new_var(s, i);
                bindings::root::Glucose::SimpSolver_addToTmpClause(s, make_lit(i));
            }
            ret = bindings::root::Glucose::SimpSolver_addTmpClause(s);
        }
    }
    ret
}

unsafe fn solver_new_var(s : *mut SimpSolver, parsed_lit : i32 ){
    let var = parsed_lit.abs()-1;
    while var >= bindings::root::Glucose::SimpSolver_nVarsLink(s) {
        bindings::root::Glucose::SimpSolver_newVarLink(s, true, true);
    } 
}


unsafe fn make_lit(parsed_lit : i32 ) -> Lit {
    let var = parsed_lit.abs()-1;
    let b : i32;
    if parsed_lit > 0{
        b = 0; // sign false, default
    } else {
        b = 1;
    }
    let a : Lit = Lit { x : var+var+b as ::std::os::raw::c_int };
    a
}

pub fn get_glucose_solver_stats(s : *mut SimpSolver) -> Vec<f64> {
    let mut stats : Vec<f64> = Vec::with_capacity(4);
    unsafe {
        //time, decisions,propagations,conflicts;
        stats.push(SimpSolver_totalTime(s));
        stats.push( (*s)._base.decisions as f64);
        stats.push( (*s)._base.propagations as f64);
        stats.push( (*s)._base.conflicts as f64);
    }
    stats
}

pub fn get_glucose_solution(s : *mut SimpSolver) -> Vec<i32>{
    let model : Vec<bindings::root::Glucose::lbool> = unsafe { Vec::from_raw_parts((*s)._base.model.data, (*s)._base.model.sz as usize, (*s)._base.model.cap as usize) };
    let mut rust_model : Vec<i32> = Vec::with_capacity(model.len());
    let mut value : i32 = 0;
    for i in &model {
        // #define l_True  (Glucose::lbool((uint8_t)0)) 
        // #define l_False (Glucose::lbool((uint8_t)1))
        // #define l_Undef (Glucose::lbool((uint8_t)2))
        value +=1;
        if i.value == 0 {
            rust_model.push(value);
        }
        else if i.value == 1 {
            rust_model.push(-value);
        }
        else{
            panic!("Model has an undefined value!");
        }
    }
    //this object belongs to glucose. don't run destructor for it.
    mem::forget(model);
    rust_model
}

pub fn set_glucose_rnd_seed(s : *mut SimpSolver, seed: f64){
    unsafe { (*s)._base.random_seed = seed; }
}

pub fn get_glucose_solver_nb_learnt(s : *mut SimpSolver) -> i32 {
    return unsafe { SimpSolver_getNbLearnt(s) };
}