mod tram;

fn main() {
    let my_instance = tram::TransportationProblem::new(5);
    let b = my_instance.is_end(4);
    println!("{}", b);
    println!("{:?}", my_instance.next_act_cost(1));
}
