pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut cars = nums.iter().map(|v| Car{start: v[0], end: v[1]}).collect::<Vec<_>>();
    cars.sort_by_key(|c| c.start);

    let mut start = cars[0].start;
    let mut end = cars[0].end;
    let mut result = 0;

    for c in cars {
        if c.start <= end {
            end = end.max(c.end);
            continue;
        }

        result += end - start +1;

        start = c.start;
        end = c.end;
    }
    
    result + end - start +1
}

struct Car{
    start: i32,
    end: i32,
}