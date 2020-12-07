use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Node {
    incoming_connections: HashMap<String, i32>,
    outgoing_connections: HashMap<String, i32>,
}

impl Node {
    fn new() -> Node {
        Node {
            incoming_connections: HashMap::<String, i32>::new(),
            outgoing_connections: HashMap::<String, i32>::new(),
        }
    }

    fn add_incoming(&mut self, source: String, count: i32) {
        self.incoming_connections.insert(source, count);
    }

    fn add_outgoing(&mut self, target: String, count: i32) {
        self.outgoing_connections.insert(target, count);
    }
}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn from_description(s: &str) -> Graph {
        let mut nodes = HashMap::<String, Node>::new();

        for line in s.split("\n") {
            let mut iter = line.split(" bags contain ");
            let name = String::from(iter.next().unwrap());

            if !nodes.contains_key(&name) {
                nodes.insert(name.clone(), Node::new());
            }

            for contains in iter.next().unwrap().split(", ") {
                if &contains[..2] == "no" {
                    break;
                }

                let tokens = contains.split(" ").collect::<Vec<&str>>();
                let count = tokens[0].parse::<i32>().unwrap();
                
                let mut contains_name = String::from(tokens[1]);
                contains_name.push_str(" ");
                contains_name.push_str(tokens[2]);

                if !nodes.contains_key(&contains_name) {
                    nodes.insert(contains_name.clone(), Node::new());
                }

                nodes
                    .get_mut(&name)
                    .unwrap()
                    .add_outgoing(contains_name.clone(), count);
                nodes
                    .get_mut(&contains_name)
                    .unwrap()
                    .add_incoming(name.clone(), count);
            }
        }

        Graph { nodes }
    }

    fn dfs_reverse(&self, n: &Node) -> HashSet<String>{
        let mut ret = HashSet::<String>::new();

        for (source, _) in &n.incoming_connections{
            ret.insert(source.clone());
            let transitive_sources = self.dfs_reverse(self.nodes.get(source).unwrap());
            ret = ret.union(&transitive_sources).cloned().collect();
        }

        return ret;
    }

    fn dfs_count(&self, n: &Node) -> i32{
        let mut cnt = 0i32;

        for (target, amount) in &n.outgoing_connections{
            cnt += amount * self.dfs_count(self.nodes.get(target).unwrap());
        }

        return cnt + 1;
    }


    fn find_all_containing_nodes(&self, name: &str) -> i32{
        return self.dfs_reverse(self.nodes.get(name).unwrap()).len() as i32;
    }

    fn count_total_bags(&self, name: &str) -> i32{
        // Subtract the bag itself
        return self.dfs_count(self.nodes.get(name).unwrap()) - 1;
    }
}

fn main() {
    let input = fs::read_to_string("src/day07/input.txt").unwrap();
    let g = Graph::from_description(&input);

    println!("Solution 1: {}", g.find_all_containing_nodes("shiny gold"));
    println!("Solution 2: {}", g.count_total_bags("shiny gold"));
}
