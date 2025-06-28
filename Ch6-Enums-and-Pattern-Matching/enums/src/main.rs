enum IpAddrKind {
    v4,
    v6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     v4(String),
//     v6(String),
// }

struct Ipv4Addr {
    address: (u8, u8, u8, u8),
}

struct Ipv6Addr {
    address: String,
}

enum IpAddr {
    v4(Ipv4Addr),
    v6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called message");
    }
}

struct QuitMessage;
struct MoveMessage {
    x: u32,
    y: u32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    // let home = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::v6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::v4(String::from("127.0.0.1"));
    // let loopback = IpAddr::v6(String::from("::1"));

    // let home = IpAddr::v4(127, 0, 0, 1);
    // let loopback = IpAddr::v6(String::from("::1"));

    let home = IpAddr::v4(Ipv4Addr {
        address: (127, 0, 0, 1),
    });
    let loopback = IpAddr::v6(Ipv6Addr {
        address: String::from("::1"),
    });

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(6);

    // let z = x + y;
}

fn route(ip_kind: IpAddrKind) {}
