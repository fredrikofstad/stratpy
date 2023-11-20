fn find_ne(matrix: Vec<Vec<Vec<i32>>>) -> Vec<(usize, usize)> {
    let mut nash_equilibria = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, &ref payoff) in row.iter().enumerate() {
            let player1_best = row.iter().all(|p| p[0] <= payoff[0]);
            let player2_best = matrix.iter().all(|r| r[j][1] <= payoff[1]);

            if player1_best && player2_best {
                nash_equilibria.push((i, j));
            }
        }
    }

    nash_equilibria
}

//TODO: algorithm for variable too




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_best_response() {
        let matrix: Vec<Vec<Vec<i32>>> = vec![
            vec![vec![0, 1], vec![3, 4]],
            vec![vec![1, 1], vec![2, 2]],
        ];

        let nash_equilibria = find_ne(matrix);
        for (i, j) in nash_equilibria {
            println!("Nash Equilibrium at strategy: Player 1 chooses {}, Player 2 chooses {}", i, j);
        }

    }

}