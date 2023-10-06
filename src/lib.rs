pub mod utils;

pub mod domain {
    pub mod hashing {
        pub mod abstractions {
            pub mod gen_range;
            pub mod hasher;
        }
        pub mod value_objects {
            pub mod numbers;
        }
    }
}

pub mod application {
    pub mod find_hashes;
}

pub mod infrastructure {
    pub mod logging;
}

pub mod presentation {
    pub mod apps {
        pub mod cli;
    }
}
