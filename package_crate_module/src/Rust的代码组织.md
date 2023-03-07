# Rust的代码组织

· 代码组织主要包括：
- 哪些细节可以暴露，哪些细节是私有的
- 作用域内哪些名称有效

· 模块系统：
- Package(包)：Cargo的特性，让你构建、测试、共享crate
- Crate(单元包)：一个模块树，它可产生一个library或可执行文件
- Module(模块)：use：让你控制代码的组织、作用域、私有路径
- Path(路径)：为struct、function、或module等项命名的方式

## Package和Crate
· Crate的类型：
- binary
- library

· Crate Root：
- 是源代码文件
- Rust编译器从这里开始，组成你的Crate的根Module

· 一个Package：
- 包含1个Cargo.toml，它描述了如何构建这些Crates
- 只能包含0-1个library crate
- 可以包含任意数量的binary crate
- 但必须至少包含一个crate(library或binary)

## Cargo的惯例
· src/main.rs：
- binary crate的crate root
- crate名与package名相同

· src/lib.rs：
- package包含一个library crate
- library crate的crate root
- crate名与package名相同

· Cargo把crate root文件交给rustc来构建library或binary

·一个Package可以同时包含src/main.rs和src/lib.rs
- 一个binary crate，一个library crate
- 名称与package名相同

· 一个Package可以有多个binary crate：
- 文件放在src/bin
- 每个文件是单独的binary crate

## Crate的作用
· 将相关功能组合到一个作用域内，便于在项目间进行共享
- 防止冲突

·例如rand crate，访问它的功能需要通过它的名字：rand

## 定义module来控制作用域和私有性
· Module:
- 在一个crate内，将代码进行分组
- 增加可读性，易于复用
- 控制项目(item)的私有性。public、private

·建立module:
- mod关键字
- 可嵌套
- 可包含其他项(struct、enum、常量、trait、函数等)的定义

## Module
· src/main.rs和src/lib.rs叫做crate roots：
- 这两个文件(任意一个)的内容形成了名为crate的模块，位于整个模块树的根部
- 整个模块树在隐式的crate模块下

## 路径(Path)
· 为了在Rust的模块中找到某个条目，需要使用路径
· 路径的两种形式：
- 绝对路径： 从crate root开始，使用crate名或字面值crate
- 相对路径：从当前模块开始，使用self，super或当前模块的标识符

·路径至少由一个标识符组成，标识符之间使用::

## 私有边界(privacy boundary)
· 模块不仅可以组织代码，还可以定义私有边界
· 如果想把函数或struct等设为私有，可以将它放到某个模块中
· Rust中所有的条目(函数，方法，struct，enum，模块，常量)默认是私有的
· 父级模块无法访问子模块中的私有条目
· 子模块里可以使用所有祖先模块中的条目

## Pub关键字
· 使用pub关键字来将某些条目标记为公共的
· 在同一个文件里，无论是公有还是私有都是可以访问的

## Super关键字
· super：用来访问父级模块路径中的内容，类似文件系统中的..

## Pub struct
· pub放在struct前：
- struct是公共的
- struct的字段默认是私有的

· struct的字段需要单独设置pub来变成公有

## Pub enum
· pub放在enum前：
- enum是公共的
- enum的变体也都是公共的

## use关键字
· 可以使用use关键字将路径导入到作用域内
- 仍遵循私有性规则

## use的习惯用法
· 函数：将函数的父级模块引入作用域(指定到父级)
· struct，enum，其他：指定完整路径(指定到本身)
· 同名条目：指定到父级

## as关键字
· as关键字可以为引入的路径指定本地的别名

## 使用pub use重新导出名称
· 使用use将路径(名称)导入到作用域内后，该名称在此作用域内是私有的
· pub use：重导出
- 将条目引入作用域
- 该条目可以被外部代码引入到它们的作用域

## 使用外部包(package)
· Cargo.toml添加依赖的包(package)
- https://crates.io/

· use将特定条目引入作用域
· 标准库(std)也被当做外部包
- 不需要修改Cargo.toml来包含std
- 需要使用use将std中的特定条目引入当前作用域

## 使用嵌套路径清理大量的use语句
· 如果使用一个包或模块下的多个条目
·可使用嵌套路径在同一行内将上述条目进行引入：
- 路径相同的部分::{路径差异的部分}
- 如果两个use路径之一是另一个的子路径
  - 使用self

## 通配符
- 使用*可以把路径中所有的公共条目都引入到作用域
- 注意：谨慎使用
- 应用场景
  - 测试。将所有被测试代码引入到tests模块
  - 有时被用于预导入(prelude)模块

## 将模块拆分为不同的文件
· 模块定义时，如果模块名后边是";"，而不是代码块：
- Rust会从与模块同名的文件中加载内容
- 模块树的结构不会变化

· 随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中