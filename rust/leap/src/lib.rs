pub fn is_leap_year(year: i32) -> bool {
    if is_exceptional_century(&year) {
        true
    } else {
        is_not_century(&year) && is_divisible_by_4(&year)
    }
}

fn is_exceptional_century(year: &i32) -> bool {
    year % 400 == 0
}

fn is_not_century(year: &i32) -> bool {
    year % 100 != 0
}

fn is_divisible_by_4(year: &i32) -> bool {
    year % 4 == 0
}
