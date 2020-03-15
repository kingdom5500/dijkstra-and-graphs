#[macro_use]
mod graph;

use graph::Graph;

fn main() {
    let graph: Graph<'_, &str, u32> = graph!(
        "A" => [6 => "B", 5 => "C"],
        "B" => [3 => "C", 4 => "D"],
        "C" => [3 => "D", 7 => "E", 10 => "F"],
        "D" => [5 => "E"],
        "E" => [4 => "F"],
        "F" => []
    );

    println!("Graph: {:#?}", graph);

    let x = graph.dijkstra_paths(&"A");
    println!("Shortest distances between A and: {:#?}", x);
}
