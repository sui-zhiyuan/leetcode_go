pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut cost = [0, 0, 0];
    let g_map = ['M' , 'P' , 'G' ];

    for (i, g) in garbage.iter().enumerate().rev() {
        for (gi , &gv) in g_map.iter().enumerate(){
            if cost[gi] > 0 {
                cost[gi] += travel[i];
            }

            cost[gi] += g.chars().filter(|&c| c == gv).count() as i32;
        }
    }

    cost.iter().sum()
}
