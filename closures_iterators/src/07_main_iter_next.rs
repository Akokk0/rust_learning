fn main() {
    // iter方法：在不可变引用上创建迭代器
    // into_iter方法：创建的迭代器会获取所有权
    // iter_mut方法：迭代可变的引用
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&3));
}