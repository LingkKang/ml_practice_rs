mod blind_search;

fn main() {
    // let my_instance = tram::TransportationProblem::new(17);
    let my_instance = blind_search::TramProblem::new(17);
    let mut path: blind_search::Path;
    let mut cost: blind_search::Cost;

    (path, cost) = blind_search::generic_blind_search(&my_instance, blind_search::SearchType::BFS);
    println!(
        "The path found by Breadth First Search is {:?}, with time cost {}",
        path, cost
    );

    (path, cost) = blind_search::generic_blind_search(&my_instance, blind_search::SearchType::DFS);
    println!(
        "The path found by Depth First Search is {:?}, with time cost {}",
        path, cost
    );

    (path, cost) = blind_search::generic_blind_search(&my_instance, blind_search::SearchType::UCS);
    println!(
        "The path found by Uniform Cost Search is {:?}, with time cost {}",
        path, cost
    );
}
