#[derive(Debug)]
enum USAState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Conneticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisana,
    Maine,
    Maryland,
    Massachussets,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennesse,
    Texas,
    Utah,
    Vermont,
    Virgina,
    Washington,
    WestVirgina,
    Wisconsin,
    Wyoming,
}

impl USAState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            USAState::Alabama => year >= 1819,
            USAState::Alaska => year >= 1959,
            USAState::Arizona => year >= 1912,
            USAState::Arkansas => year >= 1836,
            USAState::California => year >= 1850,
            USAState::Colorado => year >= 1876,
            USAState::Conneticut => year >= 1788,
            USAState::Delaware => year >= 1787,
            USAState::Florida => year >= 1845,
            USAState::Georgia => year >= 1788,
            USAState::Hawaii => year >= 1959,
            USAState::Idaho => year >= 1890,
            USAState::Illinois => year >= 1818,
            USAState::Indiana => year >= 1816,
            USAState::Iowa => year >= 1846,
            USAState::Kansas => year >= 1861,
            USAState::Kentucky => year >= 1792,
            USAState::Louisana => year >= 1812,
            USAState::Maine => year >= 1820,
            USAState::Maryland => year >= 1788,
            USAState::Massachussets => year >= 1788,
            USAState::Michigan => year >= 1837,
            USAState::Minnesota => year >= 1858,
            USAState::Mississippi => year >= 1817,
            USAState::Missouri => year >= 1821,
            USAState::Montana => year >= 1889,
            USAState::Nebraska => year >= 1867,
            USAState::Nevada => year < 1864,
            USAState::NewHampshire => year >= 1788,
            USAState::NewJersey => year >= 1787,
            USAState::NewMexico => year >= 1912,
            USAState::NewYork => year >= 1788,
            USAState::NorthCarolina => year >= 1789,
            USAState::NorthDakota => year >= 1889,
            USAState::Ohio => year >= 1803,
            USAState::Oklahoma => year >= 1907,
            USAState::Oregon => year >= 1859,
            USAState::Pennsylvania => year >= 1787,
            USAState::RhodeIsland => year >= 1790,
            USAState::SouthCarolina => year >= 1788,
            USAState::SouthDakota => year >= 1889,
            USAState::Tennesse => year >= 1796,
            USAState::Texas => year >= 1845,
            USAState::Utah => year >= 1896,
            USAState::Vermont => year >= 1791,
            USAState::Virgina => year >= 1788,
            USAState::Washington => year >= 1889,
            USAState::WestVirgina => year >= 1863,
            USAState::Wisconsin => year >= 1848,
            USAState::Wyoming => year >= 1890,
        }
    }
}

enum Coin {
    Penny,
    Pickle,
    Dime,
    Quarter(USAState),
}

fn describe_state(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("The state {state:?} is old"))
        } else {
            Some(format!("The state {state:?} is new"))
        }
    } else {
        None
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Pickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quater from state {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let val_penny = value_in_cents(penny);

    println!("The value is {val_penny}");

    let quarer = Coin::Quarter(USAState::California);
    let val_quarter = value_in_cents(quarer);

    println!("The value is {val_quarter}");

    let x = Some(5);
    let y = plus_one(x);
    let none = plus_one(None);

    let mass = USAState::Massachussets;
    let utah = USAState::Utah;

    let ex1 = mass.existed_in(1990);
    let ex2 = utah.existed_in(1770);

    println!("Massachussets existed in 1990: {ex1}");
    println!("Utah existed in 1770: {ex2}");

    let miss_coin = Coin::Quarter(USAState::Missouri);
    let desc = describe_state(miss_coin);

    match desc {
        Some(str) => {
            println!("{str}");
        }
        None => (),
    }
}
