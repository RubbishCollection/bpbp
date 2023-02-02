extern crate proc_macro;
use proc_macro::TokenStream;

#[derive(Clone)]
struct Replacer {
    replacer: Vec<u8>,

    is_end: bool,
}

#[proc_macro]
pub fn bp(item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let item_str = item_str[1..item_str.len() - 1].to_string();

    let replacer = make_replacer(&item_str);

    let mut result = String::new();

    for r in replacer {
        result.push_str(&replace_x_with_replacer(&item_str, r));
        result.push('|');
    }

    result.remove(result.len() - 1);

    result.parse().unwrap()
}

fn make_replacer(str: &str) -> Replacer {
    let mut result = Vec::new();

    let len = str.matches('x').count();

    for _ in 0..len {
        result.push(0b0);
    }

    Replacer {
        replacer: result,
        is_end: false,
    }
}

impl Iterator for Replacer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            None
        } else {
            let result = self.replacer.clone();

            let mut did_something = false;

            for v in self.replacer.iter_mut() {
                if *v == 0b0 {
                    *v = 0b1;
                    did_something = true;
                    break;
                } else {
                    *v = 0b0;
                }
            }

            if !did_something {
                self.is_end = true;
            }

            Some(result)
        }
    }
}

fn replace_x_with_replacer(pattern: &str, replacer: Vec<u8>) -> String {
    let mut result = pattern.to_string();

    for r in replacer {
        result = result.replacen("x", &r.to_string(), 1);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replacer_test() {
        let mut replacer = Replacer { replacer: vec![0b0, 0b0], is_end: false };

        assert_eq!(vec![0b0, 0b0], replacer.next().unwrap());
        assert_eq!(vec![0b1, 0b0], replacer.next().unwrap());
        assert_eq!(vec![0b0, 0b1], replacer.next().unwrap());
        assert_eq!(vec![0b1, 0b1], replacer.next().unwrap());
        assert_eq!(None, replacer.next());
    }

    #[test]
    fn replace_x_with_replacer_test() {
        let v = vec![0b0, 0b0, 0b0];

        assert_eq!("101010", replace_x_with_replacer("1x1x1x", v));

        let v = vec![0b0, 0b1, 0b0, 0b1];

        assert_eq!("01_00_01_01", replace_x_with_replacer("xx_00_x1_0x", v));
    }

    #[test]
    fn total_test() {
        let item_str = "(0bxxx1, 0bx0)";

        let replacer = make_replacer(&item_str);
    
        let mut result = String::new();
    
        for r in replacer {
            result.push_str(&replace_x_with_replacer(&item_str, r));
            result.push('|');
        }

        result.remove(result.len() - 1);

        println!("{:#?}", result)
    }
}