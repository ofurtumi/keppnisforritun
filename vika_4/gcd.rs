use std::io::{self, BufRead};

fn main() {
    let lines: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let output = combat(lines);
    println!("{output}");
}

fn combat(values: Vec<Vec<i64>>) -> String {
    let (h, d) = (values[1][0], values[1][1]);

    let mut shop: Vec<i64> = values[2].clone();
    shop.sort();
    shop.dedup();
    let enemies = values[3].clone();

    if will_survive(h, d, &enemies) {
        return "Veski Eyleifs er bjargad".to_string();
    } else if !will_survive(h, d + shop.last().unwrap(), &enemies) {
        return "Nu er Eyleifur i bobba".to_string();
    }

    // for item in shop {
    //     if will_survive(h, d + item, &enemies) {
    //         return item.to_string();
    //     }
    // }
    let best_item = binary_shopping(h, h, shop, enemies);
    return best_item.to_string();
    // return "none".to_string();
}

fn binary_shopping(health: i64, damage: i64, shop: Vec<i64>, enemies: Vec<i64>) -> i64 {
    let (mut bot, mut top) = (0, shop.len() - 1);
    let mut mid = top / 2;

    while bot != top {
        // println!("{:?} <- {mid}", shop);
        if will_survive(health, damage + shop[mid], &enemies) {
            if mid == 0 {
                return shop[mid];
            }
            top = mid;
        } else {
            bot = mid + 1;
        }
        mid = bot + (top - bot) / 2
    }

    return shop[mid];
}

fn will_survive(total_health: i64, damage: i64, enemies: &Vec<i64>) -> bool {
    let mut hp: i64 = total_health;
    for enemy in enemies {
        if damage < *enemy {
            hp -= (enemy + (damage - 1)) / damage - 1
        }
        if hp <= 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        assert_eq!(
            combat(vec![
                vec![4, 3],
                vec![10, 10],
                vec![20, 5, 1, 10],
                vec![10, 55, 100]
            ]),
            5.to_string()
        );
        assert_eq!(
            combat(vec![
                vec![4, 3],
                vec![10, 10],
                vec![1, 5, 10, 20],
                vec![10, 15, 20]
            ]),
            "Veski Eyleifs er bjargad".to_string()
        );

        assert_eq!(
            combat(vec![
                vec![4, 3],
                vec![10, 10],
                vec![1, 5, 10, 20],
                vec![10, 15, 400000]
            ]),
            "Nu er Eyleifur i bobba".to_string()
        );
    }
}
