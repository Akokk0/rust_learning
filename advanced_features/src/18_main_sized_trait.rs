/*fn generic<T>(t: T) {

}*/
// 上面会隐式转换为下面
/*fn generic<T: Sized>(t: T) {

}*/
// ? 只能用在sized 后边参数必须加引用
/*fn generic<T: ?Sized>(t: &T) {

}*/

// 默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法接触这一限制