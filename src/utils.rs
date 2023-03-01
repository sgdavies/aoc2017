pub fn neighbours(x: i32, y: i32) -> Vec<(i32, i32)> {
    let v = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    v.iter().map(|(dx, dy)| (x + dx, y + dy)).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_neighbours() {
        let result = crate::utils::neighbours(0, 0);
        assert_eq!(
            result,
            vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ]
        );

        let result = crate::utils::neighbours(-7, 100);
        assert_eq!(
            result,
            vec![
                (-8, 99),
                (-8, 100),
                (-8, 101),
                (-7, 99),
                (-7, 101),
                (-6, 99),
                (-6, 100),
                (-6, 101)
            ]
        );
    }
}
