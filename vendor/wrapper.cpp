#include "simp/SimpSolver.h"
#include "simp/SolverHelper.h"
#include "utils/System.h"
#include "core/Solver.h"

namespace Glucose {

struct Wrapper {
  SimpSolver * solver;
    Wrapper () : solver (new SimpSolver ()){ }
    ~Wrapper () { delete solver; }
};

}

using namespace Glucose;

extern "C" {

#include "wrapper.h"

CGlucose * cglucose_init (void) {
  return (CGlucose*) new Wrapper ();
}

void cglucose_add_to_clause (CGlucose * wrapper, int lit) {
  Lit clit;
  if (lit > 0) clit = mkLit(lit-1, false);
  else clit = mkLit(-lit-1, true);
  while (lit >= ((Wrapper*) wrapper)->solver->nVarsLink()) {
    ((Wrapper*) wrapper)->solver->newVarLink(true, true);
  } 
  ((Wrapper*) wrapper)->solver->addToTmpClause (clit);
}

void cglucose_clean_clause(CGlucose * wrapper) {
    ((Wrapper*) wrapper)->solver->cleanTmpClauseVec();
}

void cglucose_commit_clause(CGlucose * wrapper) {
    ((Wrapper*) wrapper)->solver->addTmpClause ();
}

void cglucose_assume (CGlucose * wrapper, int lit) {
  Lit clit;
  if (lit > 0) clit = mkLit(lit-1, false);
  else clit = mkLit(-lit-1, true);
  ((Wrapper*) wrapper)->solver->addToAssumptionsVec (clit);
}

int cglucose_solve (CGlucose * wrapper) {
  bool ret = ((Wrapper*) wrapper)->solver->solveWithAssumpLink ();
  ((Wrapper*) wrapper)->solver->clearAssumptions ();
  return !ret;
}

int cglucose_val (CGlucose * wrapper, int lit) {
  return ((Wrapper*) wrapper)->solver->getVal (lit);
}

unsigned long long cglucose_solver_nodes (CGlucose * ptr){
  return ((Wrapper*) ptr)->solver->decisions;
}

unsigned long long cglucose_nb_learnt(CGlucose * ptr){
  return ((Wrapper*) ptr)->solver->getNbLearnt();
}

void cglucose_set_random_seed(CGlucose * ptr, double seed ){
  ((Wrapper*) ptr)->solver->random_seed = seed;
}

}
