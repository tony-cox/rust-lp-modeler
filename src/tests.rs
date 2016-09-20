use variables::*;
use variables::LpExpression::*;
use problem::*;
use std::rc::Rc;


#[test]
fn expressions_creation() {
    let ref a = LpVariable::new("a", LpType::Integer)
        .lower_bound(10.0)
        .upper_bound(20.0);
    let ref b = LpVariable::new("b", LpType::Integer);

    assert_eq!(a + b, AddExpr(Rc::new(a.clone()), Rc::new(b.clone())));
}

