#![feature(exhaustive_patterns, never_type)]
#![allow(clippy::let_and_return)]

enum SingleVariantEnum {
    Variant(i32),
}

struct TupleStruct(i32);

enum EmptyEnum {}

fn infallible_destructuring_match_enum() {
    let wrapper = SingleVariantEnum::Variant(0);

    // This should lint!
    let data = match wrapper {
        SingleVariantEnum::Variant(i) => i,
    };

    // This shouldn't!
    let data = match wrapper {
        SingleVariantEnum::Variant(_) => -1,
    };

    // Neither should this!
    let data = match wrapper {
        SingleVariantEnum::Variant(i) => -1,
    };

    let SingleVariantEnum::Variant(data) = wrapper;
}

fn infallible_destructuring_match_struct() {
    let wrapper = TupleStruct(0);

    // This should lint!
    let data = match wrapper {
        TupleStruct(i) => i,
    };

    // This shouldn't!
    let data = match wrapper {
        TupleStruct(_) => -1,
    };

    // Neither should this!
    let data = match wrapper {
        TupleStruct(i) => -1,
    };

    let TupleStruct(data) = wrapper;
}

fn never_enum() {
    let wrapper: Result<i32, !> = Ok(23);

    // This should lint!
    let data = match wrapper {
        Ok(i) => i,
    };

    // This shouldn't!
    let data = match wrapper {
        Ok(_) => -1,
    };

    // Neither should this!
    let data = match wrapper {
        Ok(i) => -1,
    };

    let Ok(data) = wrapper;
}

impl EmptyEnum {
    fn match_on(&self) -> ! {
        // The lint shouldn't pick this up, as `let` won't work here!
        let data = match *self {};
        data
    }
}

fn main() {}
