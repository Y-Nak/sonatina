// TODO: We'll uncomment this tests when we implement the same optimization pass
// with egglog.
//
//
//use std::path::{Path, PathBuf};
//
//use sonatina_codegen::{domtree::DomTree, optim::gvn::GvnSolver};
//
//use sonatina_ir::{ControlFlowGraph, Function};
//
//use super::{FuncTransform, FIXTURE_ROOT};
//
//#[derive(Default)]
//pub struct GvnTransform {
//    domtree: DomTree,
//    cfg: ControlFlowGraph,
//}
//
//impl FuncTransform for GvnTransform {
//    fn transform(&mut self, func: &mut Function) {
//        self.cfg.compute(func);
//        self.domtree.compute(&self.cfg);
//        let mut solver = GvnSolver::new();
//        solver.run(func, &mut self.cfg, &mut self.domtree);
//    }
//
//    fn test_root(&self) -> PathBuf {
//        Path::new(FIXTURE_ROOT).join("gvn")
//    }
//}
