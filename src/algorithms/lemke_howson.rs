fn flatten_matrix(matrix: Vec<Vec<Vec<i32>>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut player1_payoffs = Vec::new();
    let mut player2_payoffs = Vec::new();

    for row in matrix {
        let mut player1_row = Vec::new();
        let mut player2_row = Vec::new();

        for payoffs in row {
            // Assuming each inner vector always contains two elements
            player1_row.push(payoffs[0]);
            player2_row.push(payoffs[1]);
        }

        player1_payoffs.push(player1_row);
        player2_payoffs.push(player2_row);
    }

    (player1_payoffs, player2_payoffs)
}

fn initialize_strategies(matrix: &Vec<Vec<Vec<i32>>>) -> (Vec<f64>, Vec<f64>) {
    let num_strategies_player1 = matrix.len();
    let num_strategies_player2 = if num_strategies_player1 > 0 {
        matrix[0].len()
    } else {
        0
    };

    let initial_strategy_player1 = vec![1.0 / num_strategies_player1 as f64; num_strategies_player1];
    let initial_strategy_player2 = vec![1.0 / num_strategies_player2 as f64; num_strategies_player2];

    (initial_strategy_player1, initial_strategy_player2)
}

fn pivot(
    player1_payoffs: &Vec<Vec<i32>>,
    player2_payoffs: &Vec<Vec<i32>>,
    strategy1: &Vec<f64>,
    strategy2: &Vec<f64>
) -> (Vec<f64>, Vec<f64>) {
    let num_strategies_player1 = player1_payoffs.len();
    let num_strategies_player2 = player2_payoffs[0].len();

    // Placeholder for new strategies - in a real implementation, these would be calculated
    let mut new_strategy1 = vec![0.0; num_strategies_player1];
    let mut new_strategy2 = vec![0.0; num_strategies_player2];

    // This is where the actual pivoting logic would go
    // For now, we'll just assign a simple update for demonstration purposes
    for i in 0..num_strategies_player1 {
        new_strategy1[i] = 1.0 / num_strategies_player1 as f64;
    }
    for j in 0..num_strategies_player2 {
        new_strategy2[j] = 1.0 / num_strategies_player2 as f64;
    }

    (new_strategy1, new_strategy2)
}
fn lemke_howson(matrix: Vec<Vec<Vec<i32>>>) -> (Vec<f64>, Vec<f64>) {
    let (player1_payoffs, player2_payoffs) = flatten_matrix(matrix.clone());
    let (mut strategy1, mut strategy2) = initialize_strategies(&matrix);

    loop {
        let (new_strategy1, new_strategy2) = pivot(&player1_payoffs, &player2_payoffs, &strategy1, &strategy2);

        if new_strategy1 == strategy1 && new_strategy2 == strategy2 {
            break;
        }

        strategy1 = new_strategy1;
        strategy2 = new_strategy2;
    }

    (strategy1, strategy2)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lemke_howson() {
        let matrix: Vec<Vec<Vec<i32>>> = vec![
            vec![vec![4, 4], vec![2, 2]],
            vec![vec![2, 2], vec![1, 1]],
        ];

        let (strategy1, strategy2) = lemke_howson(matrix);
        println!("Player 1 strategy: {:?}", strategy1);
        println!("Player 2 strategy: {:?}", strategy2);

    }

}