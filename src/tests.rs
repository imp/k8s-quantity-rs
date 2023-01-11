use super::*;

mod valid {
    use super::*;

    macro_rules! valid {
        ($name:ident, $text:literal == $value:expr) => {
            #[test]
            fn $name() {
                let q = $text.parse::<Quantity>().unwrap();
                assert_eq!(q.as_int64(), $value.into());
            }
        };
    }

    valid!(kibi, "258Ki" == 258 * 1024);
    valid!(mebi, "145Mi" == 145 * 1024 * 1024);
    valid!(gibi, "247Gi" == 247 * 1024 * 1024 * 1024);
    valid!(tebi, "624Ti" == 624 * 1024 * 1024 * 1024 * 1024);
    valid!(pebi, "921Pi" == 921 * 1024 * 1024 * 1024 * 1024 * 1024);
    valid!(exbi, "7Ei" == 7 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024);
    valid!(exbi_overflow, "8Ei" == None);
    valid!(kilo, "137k" == 137_000);
    valid!(mega, "894M" == 894_000_000);
    valid!(giga, "713G" == 713_000_000_000);
    valid!(tera, "52T" == 52_000_000_000_000);
    valid!(peta, "378P" == 378_000_000_000_000_000);
    valid!(exa8, "8E" == 8_000_000_000_000_000_000);
    valid!(exa9, "9E" == 9_000_000_000_000_000_000);
    valid!(exa_overflow, "10E" == None);
    valid!(simple, "492" == 492);
    valid!(zero, "0" == 0);
    valid!(single_digit, "8" == 8);
    valid!(double_digit, "27" == 27);
    valid!(more_digits, "9462823" == 9462823);
}

mod invalid {
    use super::*;

    macro_rules! invalid {
        ($name:ident, $text:literal) => {
            #[test]
            #[should_panic]
            fn $name() {
                let _q = $text.parse::<Quantity>().unwrap();
            }
        };
    }

    invalid!(kilo, "290K");
    invalid!(mb, "95MB");
    invalid!(mib, "64MiB");
}
