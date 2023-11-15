fn is_nash_equilibrium(
    matrix: &Vec<Vec<Vec<i32>>>,
    player_1_strategies: &Vec<usize>,
    player_2_strategies: &Vec<usize>,
) -> bool {

    let player_1_payoff = matrix[0][player_1_strategies[0]][player_2_strategies[0]];
    let player_2_payoff = matrix[1][player_1_strategies[0]][player_2_strategies[0]];

    let player_1_alternative_payoff = matrix[0][player_1_strategies[0]][player_2_strategies[1]];
    let player_2_alternative_payoff = matrix[1][player_1_strategies[1]][player_2_strategies[0]];

    // Check if it's a Nash equilibrium
    return player_1_payoff >= player_1_alternative_payoff
        && player_2_payoff >= player_2_alternative_payoff;
}

fn combinations(arr: &mut Vec<usize>, data: &mut Vec<usize>, start: usize, end: usize, index: usize, support_size: usize, equilibria: &mut Vec<(Vec<usize>, Vec<usize>)>, matrix: &Vec<Vec<Vec<i32>>>) {
    if index == support_size {
        let player_1_strategies = data.clone();

        for i in 0..end {
            if !player_1_strategies.contains(&i) {
                let mut player_2_strategies = player_1_strategies.clone();
                player_2_strategies.push(i);

                if is_nash_equilibrium(matrix, &player_1_strategies, &player_2_strategies) {
                    equilibria.push((player_1_strategies.clone(), player_2_strategies.clone()));
                }
            }
        }
        return;
    }

    for i in start..end {
        data[index] = i;
        combinations(arr, data, i + 1, end, index + 1, support_size, equilibria, matrix);
    }
}

fn enumerate_nash_equilibria(matrix: &Vec<Vec<Vec<i32>>>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let num_strategies = matrix[0].len();
    let mut equilibria = Vec::new();

    for support_size in 1..=num_strategies {
        let mut data = vec![0; support_size];
        combinations(&mut vec![], &mut data, 0, num_strategies, 0, support_size, &mut equilibria, matrix);
    }

    equilibria
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_support_enumeration() {
        let matrix: Vec<Vec<Vec<i32>>> = vec![
            vec![vec![0, 1], vec![3, 4]],
            vec![vec![1, 1], vec![2, 2]],
        ];

        let equilibria = enumerate_nash_equilibria(&matrix);

        println!("Nash Equilibria:");
        for (player_1_strategies, player_2_strategies) in equilibria {
            println!(
                "Player 1 strategies: {:?}, Player 2 strategies: {:?}",
                player_1_strategies, player_2_strategies
            );
        }
    }

}