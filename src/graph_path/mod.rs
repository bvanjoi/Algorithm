pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn dfs(graph: &Vec<Vec<i32>>, now: usize, path: &mut Vec<i32>, all_path: &mut Vec<Vec<i32>>) {
        path.push(now as i32);

        if now == graph.len() - 1 {
            // 依照输入要求，最后一个节点为终点
            all_path.push(path.clone());
            return;
        }

        for &i in &graph[now] {
            dfs(graph, i as usize, path, all_path);
            path.pop();
        }
    }

    let mut all_path = vec![];
    let mut path = vec![];

    dfs(&graph, 0, &mut path, &mut all_path);
    all_path
}

#[test]
fn test_all_paths_source_target() {
    assert_eq!(
        all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );

    assert_eq!(
        all_paths_source_target(vec![vec![1], vec![]]),
        vec![vec![0, 1]]
    );

    assert_eq!(
        all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]]
    );

    assert_eq!(
        all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 1, 2, 3], vec![0, 3]]
    );

    assert_eq!(
        all_paths_source_target(vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]]),
        vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4]
        ]
    );
}
