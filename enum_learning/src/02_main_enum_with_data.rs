// 还可以直接将数据附加到枚举的变体中
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
    /*
        · 优点：
            - 不需要额外使用 struct
            - 每个变体可以拥有不同的类型以及关联的数据量
    */
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

/* 标准库中的IpAddr
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }
*/