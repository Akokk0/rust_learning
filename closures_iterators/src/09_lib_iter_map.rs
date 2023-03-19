#[cfg(test)]
mod tests {

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        // 如果不执行消耗性方法，迭代器的操作就不会生效
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4])
    }
}