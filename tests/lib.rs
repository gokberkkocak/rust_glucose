#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use rust_glucose::bindings::CGlucose;
    use rust_glucose::init_glucose_solver;
    use rust_glucose::add_clause_to_glucose_solver;
    use rust_glucose::run_glucose;
    use rust_glucose::get_glucose_solution;

    #[test]
    pub fn test_solver(){
        let solver = init_glucose_solver();
        let nb_v = parse_dimacs("test.cnf", solver);
        let ret = run_glucose(solver);
        match ret {
            0 => { 
                let sol = get_glucose_solution(solver, nb_v); 
            },
            _ => panic!("aaa"),
        }
    }

    pub fn parse_dimacs(path: &str, solver : *mut CGlucose) -> usize {
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        let mut _nb_c: usize;
        let mut nb_v: usize = 0;
        for line in buffered.lines() {
            let l = line.unwrap();
            if l.contains("p") && l.contains("cnf") {
                let i : Vec<&str> = l.split_whitespace().collect();
                nb_v = i[2].to_string().parse().unwrap();
                _nb_c = i[3].to_string().parse().unwrap();
            }
            else if l.is_empty() || l.contains("c") {
                continue;
            }  else {
                let iter = l.split_whitespace();
                let mut v_clause : Vec<i32> = vec![];
                'iter: for i in iter {
                    let int: i32 = i.parse().unwrap();
                    if int == 0 {
                        break 'iter;
                    }
                    v_clause.push(int);
                }
                add_clause_to_glucose_solver(solver, v_clause);
            }
        }
        nb_v
    }
}