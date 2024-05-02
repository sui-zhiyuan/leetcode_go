// cspell:ignore mincost

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut workers = quality
        .into_iter()
        .zip(wage)
        .map(|(q, w)| Worker {
            quality: q,
            wage_per_item: w as f64 / q as f64,
        })
        .collect::<Vec<Worker>>();
    workers.sort_by(|a, b| a.wage_per_item.partial_cmp(&b.wage_per_item).unwrap());

    let mut heap = workers.iter().take(k).collect::<Vec<_>>();
    let mut count = workers.iter().take(k).map(|w| w.quality).sum::<i32>();
    for i in (0..heap.len()).rev() {
        adjust(&mut heap, i);
    }

    let mut result = workers[k - 1].wage_per_item * count as f64;
    for worker in workers.iter().skip(k) {
        if worker.quality < heap[0].quality {
            count += worker.quality - heap[0].quality;
            heap[0] = worker;
            adjust(&mut heap, 0);
        }
        result = result.min(count as f64 * worker.wage_per_item);
    }

    result
}

fn adjust(head: &mut [&Worker], mut curr: usize) {
    while curr * 2 + 1 < head.len() {
        let mut next = curr * 2 + 1;
        if curr * 2 + 2 < head.len() && head[curr * 2 + 2].quality > head[curr * 2 + 1].quality {
            next = curr * 2 + 2;
        }
        if head[curr].quality > head[next].quality {
            break;
        }
        head.swap(curr, next);
        curr = next;
    }
}

#[derive(Debug)]
struct Worker {
    quality: i32,
    wage_per_item: f64,
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test1() {
        let quality = vec![10, 20, 5];
        let wage = vec![70, 50, 30];
        let k = 2;
        let except = 105.0;
        assert_eq!(except, mincost_to_hire_workers(quality, wage, k));
    }

    #[test]
    fn test2() {
        let quality = vec![3, 1, 10, 10, 1];
        let wage = vec![4, 8, 2, 2, 7];
        let k = 3;
        let except = 30.66667;
        assert_relative_eq!(except, mincost_to_hire_workers(quality, wage, k));
    }
}
