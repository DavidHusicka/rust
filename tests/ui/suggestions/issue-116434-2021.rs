// edition:2021

trait Foo {
    type Clone;
    fn foo() -> Clone;
    //~^ ERROR the trait `Clone` cannot be made into an object [E0038]
}

trait DbHandle: Sized {}

trait DbInterface {
    type DbHandle;
    fn handle() -> DbHandle;
    //~^ ERROR the trait `DbHandle` cannot be made into an object [E0038]
}

fn main() {}
