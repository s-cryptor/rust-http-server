enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrString {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Option<T> {
    None,
    Some(T),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_string = IpAddrString::V4(127, 0, 0, 1);
    let loopback_string = IpAddrString::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}
