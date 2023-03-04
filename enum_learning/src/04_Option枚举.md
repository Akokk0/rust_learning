# Option 枚举
## 定义
· 定义于标准库中

· 在Prelude（预导入模块）中

· 描述了：某个值可能存在（某种类型）或不存在的情况
## Rust没有Null
· 其他语言中：

    - Null是一个值，它表示"没有值"
    - 一个变量可以处于两种状态：空值（null）、非空
· Null引用：Billion Dollar Mistake

· Null的问题在于：当你尝试像使用非Null值那样使用Null值的时候，就会引起某种错误

· Null的概念还是有用的：因某种原因而变为无效或缺失的值
## Rust中类似Null概念的枚举 - Option<T>
    标准库的定义：
    enum Option<T> {
        Some(T),
        None
    }

    它包含在Prelude(预导入模块)中。可直接使用：
    - Option<T>
    - Some(T)
    - None