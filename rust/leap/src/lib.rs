pub fn is_leap_year(year: i16) -> bool {
    // Not all centuries are leap years
    if is_century(&year) { return is_leap_century(&year) }

    // Years that are not centuries simply need to be divisible by 4
    year % 4 == 0
}

fn is_century(year: &i16) -> bool {
    // A century occurs every 100 years
    year % 100 == 0
}

fn is_leap_century(year: &i16) -> bool {
    // Only every fourth century is a leap year
    year % 400 == 0
}
