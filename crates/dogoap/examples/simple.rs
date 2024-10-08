use dogoap::prelude::*;

// This example is the same as examples/basic.rs but using the various `simple_*`
// functions to create the data structures instead

fn main() {
    let start = LocalState::new().with_datum("is_hungry", Datum::Bool(true));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Datum::Bool(false)));

    // NOTE This is the "simple" part, where we create an action with just
    // two strings + a field
    let eat_action =
        PlanAction::new("eat").add_mutator(Mutator::Set("is_hungry".to_string(), Datum::Bool(false)));

    let actions: Vec<PlanAction> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal);

    println!("{:#?}", plan);

    print_plan(plan.unwrap());

    println!("");
    println!("[Everything went as expected!]");
}
