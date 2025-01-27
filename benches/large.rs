// hyperfine "cargo clean && cargo bench --bench large --no-run"

const LARGE: &str = include_str!("large.txt");
const LARGES: &str = constcat::concat!(LARGE, LARGE);

fn main() {
    let expected = {
        let mut s = String::new();
        s.push_str(LARGE);
        s.push_str(LARGE);
        s
    };
    assert_eq!(LARGES, expected);
}
