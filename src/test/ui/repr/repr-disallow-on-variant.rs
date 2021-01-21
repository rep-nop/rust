struct Test;

enum Foo {
    #[repr(u8)]
    //~^ ERROR attribute should be applied to a struct, enum, function, or union
    Variant,
}

fn main() {}
