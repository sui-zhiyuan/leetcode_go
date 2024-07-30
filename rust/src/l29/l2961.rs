pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    let mut result = vec![];

    for (i , v) in variables.iter().enumerate(){
        let [a , b, c ,m ] = v[0..4] else {
            panic!("Invalid input");
        };
        let p1 = mod_power(a , b , 10);
        let p2 = mod_power(p1 ,c , m);
        if p2 == target{
            result.push(i as i32);
        }
    }

    result
}

fn mod_power(a:i32 , p:i32 , m:i32) -> i32{
    let mut result = 1;
    let mut a = a;
    let mut p = p;
    while p > 0{
        if p % 2 == 1{
            result = (result * a) % m;
        }
        a = (a * a) % m;
        p /= 2;
    }
    result
}