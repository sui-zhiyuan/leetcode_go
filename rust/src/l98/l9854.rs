use std::collections::BTreeMap;

pub fn number_of_alternating_groups(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = colors.len();
    let mut colors = colors;

    for i in 0..n - 1 {
        colors.push(colors[i]);
    }

    let mut ranges = BTreeMap::<usize, usize>::new();
    let mut count = BTreeMap::<usize, usize>::new();

    let mut start = 0;
    for (i, &color) in colors.iter().enumerate().skip(1) {
        if color == colors[i - 1] {
            ranges.insert(start, i - 1);
            *count.entry(i - start).or_insert(0) += 1;
            start = i;
        }
    }

    let mut results = Vec::new();
    for q in queries {
        match q[..] {
            [1, size] => {
                let size = size as usize;
                let result = count
                    .range(size..)
                    .map(|(&len, &count)| (len - size + 1) * count)
                    .sum::<usize>() as i32;
                results.push(result);
            }
            [2, index, value] => {
                let index = index as usize;

                
            }
            _ => panic!("unknown query"),
        }
    }
    results
}

fn change_color(colors: &mut Vec<i32>, index: usize, value: i32 , ranges: &mut BTreeMap<usize, usize>, count: &mut BTreeMap<usize, usize>) {
    if colors[index] == value {
        return;
    }
    colors[index] = value;
    if index > 0 && index < colors.len() -1 && colors[index - 1] != colors[index] && colors[index] != colors[index + 1] {
        let left = ranges.range(..index).next_back().map(|(s,e)| (*s , *e)).unwrap();
        let right = ranges.range(index+1..).next().map(|(s,e)| (*s , *e)).unwrap();
        ranges.remove(&left.0);
        *count.entry(left.1 - left.0 +1).or_default() -= 1;
        ranges.remove(&right.0);
        *count.entry(right.1 - right.0 +1).or_default() -= 1;
        ranges.remove(&index);
        *count.entry(1).or_default() -= 1;
        
        ranges.insert(left.0, right.1);
        *count.entry(right.1 - left.0 +1).or_default() += 1;
        return;
    } 
    if index > 0 && colors[index - 1] != colors[index] {
        let left = ranges.range(..index).next_back().map(|(s,e)| (*s , *e)).unwrap();
        ranges.remove(&left.0);
        *count.entry(left.1 - left.0 +1).or_default() -= 1;
        ranges.remove(&index);
        *count.entry(1).or_default() -= 1;

        ranges.insert(left.0, index);
        *count.entry(index - left.0 +1).or_default() += 1;
        return;
    } 
    if index < colors.len() - 1 && colors[index] != colors[index + 1] {
        let right = ranges.range(index+1..).next().map(|(s,e)| (*s , *e)).unwrap();
        ranges.remove(&right.0);
        *count.entry(right.1 - right.0 +1).or_default() -= 1;
        ranges.remove(&index);
        *count.entry(1).or_default() -= 1;

        ranges.insert(index, right.1);
        *count.entry(right.1 - index +1).or_default() += 1;
        return;
    } 
    {
        ranges.insert(index, index);
        *count.entry(1).or_default() += 1;
    }
}