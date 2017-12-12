extern crate moeda;

use moeda::repl::Repl;


#[cfg(test)]
mod operations {
    use super::*;

    #[test]
    fn repl_eval_sum() {
        let source_code = String::from("(+ 1 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("2"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_sub() {
        let source_code = String::from("(- 1 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("0"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_div() {
        let source_code = String::from("(/ 2 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("2"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_mul() {
        let source_code = String::from("(* 3 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("3"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_rem() {
        let source_code = String::from("(rem 10 4)");
        let mut repl = Repl::new();
        assert_eq!(format!("2"), repl.eval(source_code))
    }
}

#[cfg(test)]
mod comparison {
    use super::*;

    #[test]
    fn repl_eval_eq() {
        let source_code = String::from("(= 1 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("true"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_neq() {
        let source_code = String::from("(/= 1 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("false"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_gt() {
        let source_code = String::from("(> 2 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("true"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_lt() {
        let source_code = String::from("(< 3 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("false"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_gte() {
        let source_code = String::from("(>= 2 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("true"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_lte() {
        let source_code = String::from("(<= 3 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("false"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_max() {
        let source_code = String::from("(max 2 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("2"), repl.eval(source_code))
    }
    #[test]
    fn repl_eval_min() {
        let source_code = String::from("(min 2 1)");
        let mut repl = Repl::new();
        assert_eq!(format!("1"), repl.eval(source_code))
    }
}

#[cfg(test)]
mod logical {
    use super::*;

    #[test]
    fn repl_eval_and() {
        let mut repl = Repl::new();
        assert_eq!(
            format!("false"),
            repl.eval(String::from("(and false true)"))
        );
        assert_eq!(
            format!("false"),
            repl.eval(String::from("(and true false)"))
        );
        assert_eq!(
            format!("false"),
            repl.eval(String::from("(and false false)"))
        );
        assert_eq!(format!("true"), repl.eval(String::from("(and true true)")))
    }

    #[test]
    fn repl_eval_or() {
        let mut repl = Repl::new();
        assert_eq!(
            format!("false"),
            repl.eval(String::from("(or false false)"))
        );
        assert_eq!(format!("true"), repl.eval(String::from("(or true false)")));
        assert_eq!(format!("true"), repl.eval(String::from("(or true true)")));
        assert_eq!(format!("true"), repl.eval(String::from("(or false true)")))
    }

    #[test]
    fn repl_eval_not() {
        let mut repl = Repl::new();
        assert_eq!(format!("false"), repl.eval(String::from("(not true)")));
        assert_eq!(format!("true"), repl.eval(String::from("(not false)")))
    }
}

#[cfg(test)]
mod conditional {
    use super::*;

    #[test]
    fn repl_eval_if() {
        let mut repl = Repl::new();
        assert_eq!(format!("1"), repl.eval(String::from("(if true (1) (0))")));
        assert_eq!(format!("1"), repl.eval(String::from("(if true 1 0)")));
        assert_eq!(format!("0"), repl.eval(String::from("(if false (1) (0))")));
        assert_eq!(format!("0"), repl.eval(String::from("(if false 1 0)")));
    }

    #[test]
    fn repl_eval_when() {
        let mut repl = Repl::new();
        assert_eq!(format!("199"), repl.eval(String::from("(when true (199))")));
        assert_eq!(format!(""), repl.eval(String::from("(when false (0))")));
    }
}

#[cfg(test)]
mod indentifier {
    use super::*;

    #[test]
    fn repl_eval_variables() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(def x \"moeda\")")));
        assert_eq!(
            format!("eq"),
            repl.eval(String::from("(if (/= x \"moeda\") (\"neq\") (\"eq\"))"))
        )
    }

    #[test]
    fn repl_eval_variables_already_defined() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(def x \"moeda\")")));
        assert_eq!(
            format!("Value error: variable x has already defined."),
            repl.eval(String::from("(def x \"rust\")"))
        );
    }

    #[test]
    fn repl_eval_variables_not_defined() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(def y \"moeda\")")));
        assert_eq!(
            format!("Variable x doesn't exist in this context"),
            repl.eval(String::from("(print x)"))
        );
    }
}

#[cfg(test)]
mod functions {
    use super::*;

    #[test]
    fn repl_eval_defn() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(defn f [] (true))")));
        assert_eq!(format!("true"), repl.eval(String::from("(f)")));
    }

    #[test]
    fn repl_eval_defn_with_args() {
        let mut repl = Repl::new();
        assert_eq!(
            format!(""),
            repl.eval(String::from("(defn f [n t] (* n t))"))
        );
        assert_eq!(format!("8"), repl.eval(String::from("(f 2 4)")));
    }

    #[test]
    fn repl_eval_defn_already_defined() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(defn f [] (true))")));
        assert_eq!(
            format!("Value error: variable f has already defined."),
            repl.eval(String::from("(defn f [] (false))"))
        );
    }

    #[test]
    fn repl_eval_defn_call_function() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(defn f [n] (* n n))")));
        assert_eq!(format!("4"), repl.eval(String::from("(f 2)")));
    }

    #[test]
    fn repl_eval_defn_call_function_with_more_args() {
        let mut repl = Repl::new();
        assert_eq!(
            format!(""),
            repl.eval(String::from("(defn f [n t] (+ (* n n) t))"))
        );
        assert_eq!(format!("9"), repl.eval(String::from("(f 2 5)")));
    }

    #[test]
    fn repl_eval_defn_call_function_not_defined() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(defn f [n] (* n n))")));
        assert_eq!(
            format!("Value error: g is not callable"),
            repl.eval(String::from("(g 2)"))
        );
    }
}

#[cfg(test)]
mod stdout {
    use super::*;

    #[test]
    fn repl_eval_print() {
        let mut repl = Repl::new();
        assert_eq!(format!(""), repl.eval(String::from("(print true)")));
    }
}
