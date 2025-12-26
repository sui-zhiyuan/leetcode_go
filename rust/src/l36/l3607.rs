// pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
//     let mut list = List { nodes: Vec::new() };
//     let list_heads = (0..c + 1).map(|v| list.new_list(v)).collect();
//     let mut disjoinSet = DisjoinSet::new(list_heads);
//
//     for connection in connections {
//         let &[v1, v2] = connection.as_slice() else {
//             panic!("invalid connection");
//         };
//
//         disjoinSet.join(v1 as usize, v2 as usize, |tv1, tv2| {
//             list.merge_list(tv1, tv2)
//         });
//     }
//
//     let mut offline = vec![false; c as usize + 1];
//     let mut result = Vec::new();
//
//     for query in queries {
//         match &query {
//             &[1, t] => {
//                 let t = t as usize;
//                 if !offline[t] {
//                     result.push(t as i32);
//                     continue;
//                 }
//
//                 let (_ , head) = disjoinSet.query(t);
//                 while offline[list.nodes[head].value]{
//                     // head = list.nodes[head].next;
//                 }
//
//             }
//             &[2, t] => offline[t as usize] = true,
//             _ => unreachable!("invalid query"),
//         }
//     }
//
//     result
// }

// struct DisjoinSet<T> {
//     links: Vec<usize>,
//     values: Vec<Option<T>>,
// }
//
// impl<T> DisjoinSet<T> {
//     fn new(values: Vec<T>) -> DisjoinSet<T> {
//         let links = (0..values.len()).collect::<Vec<_>>();
//         let values = values.into_iter().map(|v| Some(v)).collect();
//         DisjoinSet { links, values }
//     }
//
//     fn join(&mut self, a: usize, b: usize, mut merge: impl FnOnce(T, T) -> T) {
//         self.links[b] = self.links[a];
//         let va = self.values[a].take().unwarp();
//         let vb = self.values[b].take().unwarp();
//         va = merge(va, vb);
//         self.values[a] = va;
//     }
//
//     fn query(&mut self, a: usize) -> (usize, &mut T) {
//         let mut target = a;
//         while self.links[target] != target {
//             target = self.links[target];
//         }
//
//         &mut self.values[a].unwarp();
//     }
// }
//
// struct Node {
//     next: Option<usize>,
//     value: i32,
// }
//
// struct List {
//     nodes: Vec<Node>,
// }
//
// impl List {
//     fn new_list(&mut self, first_value: i32) -> usize {
//         let next = self.nodes.len();
//         self.nodes.push(Node {
//             value: first_value,
//             next: None,
//         });
//         next
//     }
//
//     fn merge_list(&mut self, head_left: usize, head_right: usize) -> usize {
//         let mut new_head: Option<usize> = None;
//         let mut new_tail = None;
//         let mut head_left = Some(head_left);
//         let mut head_right = Some(head_right);
//
//         while let (Some(left), Some(right)) = (head_left, head_right) {
//             let min_index = if self.nodes[left].value < self.nodes[right].value {
//                 head_left = self.nodes[left].next;
//                 left
//             } else {
//                 head_right = self.nodes[right].next;
//                 right
//             };
//
//             let Some(tail) = &mut new_tail else {
//                 new_head = Some(min_index);
//                 new_tail = Some(min_index);
//                 continue;
//             };
//
//             self.nodes[*tail].next = Some(min_index);
//             *tail = min_index;
//             self.nodes[*tail].next = None;
//         }
//
//         new_head.expect("should be at least 2 values")
//     }
// }
