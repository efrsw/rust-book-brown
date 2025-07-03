mod some_module {
    pub mod another_mod {

    }
}

use paths::hosting;

pub use crate::some_module::another_mod;

fn main() {
    hosting::add_to_waitlist();
}
