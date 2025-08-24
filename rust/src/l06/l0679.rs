pub fn judge_point24(cards: Vec<i32>) -> bool {
    let cards = cards.into_iter().map(|v| v as f64).collect::<Vec<_>>();
    judge_inner(&cards)
}

const EPS: f64 = 1e-6;

pub fn judge_inner(cards: &[f64]) -> bool {
    if cards.len() == 1 {
        return (cards[0] - 24f64).abs() < EPS;
    }
    for i in 0..cards.len() {
        for j in i + 1..cards.len() {
            let mut remain = Vec::new();
            for (k, v) in cards.iter().copied().enumerate() {
                if k != i && k != j {
                    remain.push(v);
                }
            }

            remain.push(0f64);

            for op in Operator::ALL {
                let Some(result) = op.calc(cards[i], cards[j]) else {
                    continue;
                };

                *remain.last_mut().unwrap() = result;
                if judge_inner(&remain) {
                    return true;
                }
            }
        }
    }
    false
}

enum Operator {
    Add,
    Sub,
    SubRev,
    Mul,
    Div,
    DivRev,
}

impl Operator {
    const ALL: [Operator; 6] = [
        Operator::Add,
        Operator::Sub,
        Operator::SubRev,
        Operator::Mul,
        Operator::Div,
        Operator::DivRev,
    ];

    fn calc(&self, left: f64, right: f64) -> Option<f64> {
        match self {
            Operator::Add => Some(left + right),
            Operator::Sub => Some(left - right),
            Operator::SubRev => Some(right - left),
            Operator::Mul => Some(left * right),
            Operator::Div => {
                if right != 0f64 {
                    Some(left / right)
                } else {
                    None
                }
            }
            Operator::DivRev => {
                if left != 0f64 {
                    Some(right / left)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(judge_point24(vec![4, 1, 8, 7]))
    }

    #[test]
    fn test2() {
        assert!(judge_point24(vec![1, 3, 4, 6]))
    }
}
