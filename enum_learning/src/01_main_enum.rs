/* 枚举
    · 枚举允许我们列举所有可能的值来定义一个类型
    · 枚举的变体都位于标识符的命名空间下，使用两个冒号::进行分隔
*/
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loop_back = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}

fn route(ip_kind: IpAddrKind) {}
