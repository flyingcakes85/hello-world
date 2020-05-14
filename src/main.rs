fn main() {
    println!("Hello, world!");

    println!("{}", addfive(67));
}

fn addfive(ip: i32) -> i32 {
    ip + 3
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        assert_eq!(addfive(89), 92);
    }
}
