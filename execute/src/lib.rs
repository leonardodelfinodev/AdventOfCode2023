pub fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (day01a::main, "day01a"),
        (day01b::main, "day01b"),
    ]
}
