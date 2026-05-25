fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use freshed_rs_macros::html;

    #[test]
    fn test_1() {
        let div = html!(
            <div>divided</div>
        )
        .to_string();
        assert_eq!(div, "<div>divided</div>");
    }
    #[test]
    fn test_2() {
        let div = html!(
            <div>{123}</div>
        )
        .to_string();
        assert_eq!(div, "<div>123</div>");
    }
}
