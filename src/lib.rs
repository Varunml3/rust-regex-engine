// #![allow(dead_code, unused_mut, unused_imports, unused_variables, unreachable_patterns)]
#![allow(dead_code, unused_imports, unreachable_patterns, unused_variables)]
#![feature(test, assoc_char_funcs)]

extern crate fnv;
extern crate fxhash;
extern crate test;
#[macro_use]
extern crate educe;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::Regex;
    use config::*;
    use test::Bencher;

    fn debug_print(r: &Regex) {
        for (index, node) in r.node_vec.iter().enumerate() {
            println!("{} -- {:?}", index, node);
        }
    }

    fn test_bench(function: fn() -> ()) {
        let now = std::time::Instant::now();
        function();
        let elapsed = now.elapsed().as_millis().to_string();
        let mut file = std::fs::File::create("./log.txt").unwrap();
        use std::io::Write;
        write!(file, "{}", elapsed).unwrap();
    }

    #[test]
    fn compile_test() {
        let _r = Regex::new(r"[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?");
        // let _r = Regex::new(r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+");
        // println!("{:?}", _r);
        // println!("{:?}", _r.optimized_root_node);
        debug_print(&_r);
    }

    #[test]
    fn basic_test() {
        let r = Regex::new("hello");
        assert_eq!(r.match_str("hello"), true);
        assert_eq!(r.match_str("hi"), false);
        assert_eq!(r.match_str("hell"), false);
    }

    #[test]
    fn add_operator() {
        let r = Regex::new("a+b");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("ab"), true);
        assert_eq!(r.match_str("aaaaaaaaaaaaaaaaaaaaaaab"), true);
        assert_eq!(r.match_str("no"), false);
    }

    #[test]
    fn or_operator() {
        let r = Regex::new("a|b|c");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("ab"), true);
        assert_eq!(r.match_str("a"), true);
        assert_eq!(r.match_str("b"), true);
        assert_eq!(r.match_str("f"), false);
    }

    #[test]
    fn in_the_middle() {
        // Global search enabled by default
        let r = Regex::new("abc");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("ksjfdweriwukjdkabcdkjaifejs"), true);
        assert_eq!(r.match_str("ksjfdweriwukjdkadkbjaiabfcejs"), false);
    }

    #[test]
    fn star_operator() {
        let r = Regex::new("abcd*e");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("abcddddddddddddddddddddddddddde"), true);
        assert_eq!(r.match_str("abcddddddddddddddddddddddddddd"), false);
        assert_eq!(r.match_str("abce"), true);
    }

    #[test]
    fn add_and_star_with_brackets() {
        let r = Regex::new("(a|b|c)*d(e|f|g)+h");
        // println!("{:?}", r.node_vec);
        // debug_print(&r);
        assert_eq!(r.match_str("adgh"), true);
        assert_eq!(r.match_str("aaaaaadh"), false);
        assert_eq!(r.match_str("abcabcabacbacdfh"), true);
        assert_eq!(r.match_str("deh"), true);
        assert_eq!(r.match_str("beh"), false);
    }

    #[test]
    fn bigger_brackets() {
        let r = Regex::new(r"(hello|hi|hey) there");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("hello there"), true);
        assert_eq!(r.match_str("hi there"), true);
        assert_eq!(r.match_str("hey there"), true);
        assert_eq!(r.match_str("h there"), false);
        assert_eq!(r.match_str("bye there"), false);
        assert_eq!(r.match_str("llo there"), false);
    }

    #[test]
    fn square_brackets_simple() {
        let r = Regex::new("abc[def]ghi");
        // println!("{:?}", r.node_vec);
        // debug_print(&r);
        assert_eq!(r.match_str("abcdghi"), true);
        assert_eq!(r.match_str("abceghi"), true);
        assert_eq!(r.match_str("abcfghi"), true);
        assert_eq!(r.match_str("abcghi"), false);
        assert_eq!(r.match_str("abdeghi"), false);
    }

    #[test]
    fn range_of_chars_simple() {
        let r = Regex::new("[a-zA-Z]");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("g"), true);
        assert_eq!(r.match_str("G"), true);
        assert_eq!(r.match_str("9"), false);
    }
    #[test]
    fn range_of_chars_and_other() {
        let r = Regex::new("[a-zA-Z136]");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("g"), true);
        assert_eq!(r.match_str("G"), true);
        assert_eq!(r.match_str("9"), false);
        assert_eq!(r.match_str(")"), false);
        assert_eq!(r.match_str("1"), true);
        assert_eq!(r.match_str("3"), true);
        assert_eq!(r.match_str("6"), true);
    }

    #[test]
    fn square_brackets_with_quantifiers() {
        let r = Regex::new("[a-zA-Z136]+");
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("g13az"), true);
        assert_eq!(r.match_str("G6zA1"), true);
        assert_eq!(r.match_str("9254582"), false);
        assert_eq!(r.match_str("1GES"), true);
        assert_eq!(r.match_str("3"), true);
        assert_eq!(r.match_str("6"), true);
    }

    #[test]
    fn inclusive_d() {
        let r = Regex::new(r"\d+");
        assert_eq!(r.match_str("05421345689484651326549876532163846981351"), true);
        assert_eq!(r.match_str("asdfakjsdfklasldfajsdkhljfhalsjfd"), false);
    }

    #[test]
    fn exclusive_d() {
        let r = Regex::new(r"\D+");
        assert_eq!(r.match_str("05421345689484651326549876532163846981351"), false);
        assert_eq!(r.match_str("asdfakjsdfklasldfajsdkhljfhalsjfd"), true);
    }

    #[test]
    fn inclusive_s() {
        let r = Regex::new(r"\s+");
        assert_eq!(r.match_str("        "), true);
        assert_eq!(r.match_str("a"), false);
    }

    #[test]
    fn exclusive_s() {
        let r = Regex::new(r"\S+");
        assert_eq!(r.match_str("  "), false);
        assert_eq!(r.match_str("aaadjkfalksdfujha"), true);
    }

    #[test]
    fn inclusive_w() {
        let r = Regex::new(r"\w+");
        assert_eq!(r.match_str("0a9sd87f0a8pwoeihnpva"), true);
        assert_eq!(r.match_str("                "), false);
    }

    #[test]
    fn exclusive_w() {
        let r = Regex::new(r"\W+");
        assert_eq!(r.match_str("0a9sd87f0a8pwoeihnpva"), false);
        assert_eq!(r.match_str("                "), true);
    }

    #[test]
    fn question_mark() {
        let r = Regex::new("abcde?f");
        println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("abcdefg"), true);
        assert_eq!(r.match_str("abcdfg"), true);
        assert_eq!(r.match_str("abcfge"), false);
        assert_eq!(r.match_str("abcfge"), false);
    }

    #[test]
    fn question_mark_with_brackets() {
        let r = Regex::new("abc(d|e|f)?hij");
        debug_print(&r);
        assert_eq!(r.match_str("abcdhij"), true);
        assert_eq!(r.match_str("abcehij"), true);
        assert_eq!(r.match_str("abcfhij"), true);
        assert_eq!(r.match_str("abchij"), true);
        assert_eq!(r.match_str("abchi"), false);
        assert_eq!(r.match_str("abcdefhij"), false);
        assert_eq!(r.match_str("abfhij"), false);
    }

    #[test]
    fn question_mark_with_square_brackets() {
        let r = Regex::new("abc[def]?hij");
        // debug_print(&r);
        // println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("abcdhij"), true);
        assert_eq!(r.match_str("abcehij"), true);
        assert_eq!(r.match_str("abcfhij"), true);
        assert_eq!(r.match_str("abchij"), true);
        assert_eq!(r.match_str("abchi"), false);
        assert_eq!(r.match_str("abcdefhij"), false);
        assert_eq!(r.match_str("abfhij"), false);
    }

    #[test]
    fn exclusive_square_brackets() {
        let r = Regex::new("a[^bcd]+e");
        assert_eq!(r.match_str("ammmmmmmmmmmmmmmmmmmmmme"), true);
        assert_eq!(r.match_str("aee"), true);
        assert_eq!(r.match_str("abcde"), false);
        assert_eq!(r.match_str("ade"), false);
        assert_eq!(r.match_str("arb"), false);
    }

    #[test]
    fn difficult_real_world_tests() {
        let phone = Regex::new(r"^\+*\(?[0-9]+\)?[-\s\.0-9]*$");
        let email = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}");
        assert_eq!(phone.match_str("+447777666555"), true);
        assert_eq!(phone.match_str("test@gmail.com"), false);
        assert_eq!(email.match_str("realemailaddress@realcompany.com"), true);
        assert_eq!(email.match_str("100%_fake_email_address_here"), false);
    }

    #[test]
    fn single_character_curly_brackets_one() {
        let r = Regex::new("^a{4}b{2}c$");
        assert_eq!(r.match_str("aaaabbc"), true);
        assert_eq!(r.match_str("aaabc"), false);
    }

    #[test]
    fn single_character_curly_brackets_comma() {
        let r = Regex::new("^a{4,}b{2,}c$");
        assert_eq!(r.match_str("aaaaaaaaaaaaabbc"), true);
        assert_eq!(r.match_str("aaabc"), false);
    }

    #[test]
    fn single_character_curly_brackets_both() {
        let r = Regex::new("^a{4,6}b{2,4}c$");
        // println!("{:?}", r.node_vec);
        debug_print(&r);
        assert_eq!(r.match_str("aaaabbc"), true);
        assert_eq!(r.match_str("abbbc"), false);
    }

    #[test]
    fn brackets_curly_brackets_simple() {
        let r = Regex::new("(a|b|c){4}");
        for (index, node) in r.node_vec.iter().enumerate() {
            println!("{} -- {:?}", index, node);
        }
        // assert_eq!(r.match_str("abca"), true);
        // assert_eq!(r.match_str("aadbc"), false);
    }

    #[test]
    fn brackets_curly_brackets_comma_simple() {
        let r = Regex::new("(a|b|c){4,}");
        // debug_print(&r);
        assert_eq!(r.match_str("abaaaaaaaaaaaaaacaad"), true);
        assert_eq!(r.match_str("aadbc"), false);
    }

    #[test]
    fn brackets_curly_brackets_both() {
        let r = Regex::new("^(a|b|c){4,5}$");
        // debug_print(&r);
        assert_eq!(r.match_str("abcb"), true);
        assert_eq!(r.match_str("abcba"), true);
        assert_eq!(r.match_str("aab"), false);
    }

    #[test]
    fn sq_brackets_curly_brackets() {
        let r = Regex::new("[abc]{4}");
        assert_eq!(r.match_str("abca"), true);
        assert_eq!(r.match_str("aadbc"), false);
    }

    #[test]
    fn sq_brackets_curly_brackets_comma() {
        let r = Regex::new("[abc]{4,}");
        assert_eq!(r.match_str("abaaaaaaaaaaaaaacaad"), true);
        assert_eq!(r.match_str("aadbc"), false);
    }

    #[test]
    fn escaped_character_curly_brackets() {
        let r = Regex::new(r"\w{4,6}");
        // debug_print(&r);
        assert_eq!(r.match_str("abdc"), true);
    }

    #[test]
    fn positive_lookahead() {
        let r = Regex::new("^abc(?=def)d");
        assert_eq!(r.match_str("abcdef"), true);
        assert_eq!(r.match_str("abcdeg"), false);
        let r = Regex::new(r"a(?=bcde|bc)bcef");
        assert_eq!(r.match_str("abcef"), true);
        assert_eq!(r.match_str("abcde"), false);
    }

    #[test]
    fn negative_lookahead() {
        let r = Regex::new("^abc(?!def)d");
        assert_eq!(r.match_str("abcdef"), false);
        assert_eq!(r.match_str("abcdeg"), true);
    }

    #[test]
    fn atomic_groups() {
        let r = Regex::new("a(?>bc|b)c");
        // println!("{:?}", r.node_vec);
        debug_print(&r);
        assert_eq!(r.match_str("abcc"), true);
        assert_eq!(r.match_str("abc"), false);
    }

    #[test]
    fn possessive() {
        let r = Regex::new(r#"".*+""#);
        assert!(!r.is_match(r#""abc"x"#));
        let r = Regex::new(r#"".*""#);
        assert!(r.is_match(r#""abc"x"#));
    }

    #[test]
    fn lazy() {
        let r = Regex::new("[ab]+?(?>b)c");
        assert_eq!(r.match_str("aba"), false);
        assert_eq!(r.match_str("aabc"), true);
        let r = Regex::new(r"\w+?a");
        assert_eq!((0,3), r.match_indices("wwawwwwwwwwwwa")[0]);
        let r = Regex::new(r"\w+a");
        assert_eq!(vec![(0,14)], r.match_indices("wwawwwwwwwwwwa"));
    }

    #[test]
    fn recurse() {
        let r = Regex::new(r"(?:a|b)(?R)?");
        println!("{:?}", r.node_vec);
        assert_eq!(r.match_str("aa"), true);
        assert_eq!(r.match_str("baaaabaaaa"), true);
        // Need a better test here
        assert_eq!(r.match_str("c"), false);
    }

    #[test]
    fn boundary() {
        let r = Regex::new(r"\b\w+\b");
        let string = "This is a group of words";
        // println!("{}", string.len());
        // debug_print(&r);
        let matches = r.match_indices(&string);
        // println!("{:?}", matches);
    }

    #[test]
    fn possessive_curly_brackets() {
        let r = Regex::new(r"a{2,}+b");
    }

    #[test]
    fn benchmark_but_run_only_once() {
        let r = Regex::new(r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+");
        let input = include_str!(r"../input_text.txt");
        let now = std::time::Instant::now();
        println!("{:?}", r.match_indices(input));
        let elapsed: String = format!("{}", now.elapsed().as_millis());
        let mut file = std::fs::File::create("./log.txt").unwrap();
        use std::io::Write;
        write!(file, "{}", elapsed).unwrap();
    }

    #[bench]
    fn match_benchmark_short(b: &mut Bencher) {
        let phone = Regex::new(r"^\+*\(?[0-9]+\)?[-\s\.0-9]*$");
        b.iter(|| {
            assert_eq!(phone.match_str("+447777-666-555"), true);
            assert_eq!(phone.match_str("test@gmail.com"), false);
        });
    }

    #[bench]
    fn match_benchmark_long(b: &mut Bencher) {
        let phone = Regex::new(r"^\+*\(?[0-9]+\)?[-\s\.0-9]*$");
        b.iter(|| {
            assert_eq!(phone.match_str("+447777-666-5555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555"), true);
        });
    }

    #[bench]
    fn compile_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let _i = test::black_box(Regex::new(r"^\+*\(?[0-9]+\)?[-\s\.0-9]*$"));
        });
    }

    // #[bench]
    fn email_bench(b: &mut Bencher) {
        let email = Regex::new(r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+");
        let input = include_str!(r"../input_text.txt");
        b.iter(|| {
            test::black_box(email.match_indices(input));
        })
    }

    // #[test]
    fn email_test() {
        test_bench(|| {let email = Regex::new(r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+");
        let input = include_str!(r"../input_text.txt");
        email.match_indices(input);})
    }
    // #[test]
    // fn email_test2() {
    //     let email = Regex::new(r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+");
    //     let input = include_str!(r"../input_text.txt");
    //     email.is_match(input);
    // }

    // #[bench]
    fn uri_bench(b: &mut Bencher) {
        let uri = Regex::new(r"[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?");
        let input = include_str!(r"../input_text.txt");
        b.iter(|| {
            test::black_box(uri.match_indices(input));
        })
    }

    // #[test]
    fn uri_test() {
        test_bench(|| {
            let uri = Regex::new(r"[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?");
        let input = include_str!(r"../input_text.txt");
        test::black_box(uri.match_indices(input));});
    }

    // #[bench]
    fn ipv4_bench(b: &mut Bencher) {
        let ipv4 = Regex::new(r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])");
        let input = include_str!(r"../input_text.txt");
        b.iter(|| {
            test::black_box(ipv4.match_indices(input));
        })
    }

    // #[test]
    fn ipv4_test() {
        test_bench(|| {let ipv4 = Regex::new(r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])");
        let input = include_str!(r"../input_text.txt");
        test::black_box(ipv4.match_indices(input));});
    }

    // // #[bench]
    // fn test_str_to_char_conversion(b: &mut Bencher) {
    //     let s = include_str!(r"../input_text.txt");
    //     b.iter(|| {
    //         let _ = test::black_box(super::utils::str_to_char_vec(s));
    //     })
    // }
}

mod backtrack_matcher;
mod compiled_node;
pub mod config;
#[macro_use]
mod constants;
mod dfa_matcher;
mod matcher;
mod nfa;
mod optimize;
mod parallel_nfa;
mod parse;
pub mod regex;
mod replace;
mod root_node_optimizer;
mod sorted_vec;
mod unicode_ranges;
mod utf_8;
mod utils;
