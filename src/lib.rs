pub mod utils;

pub mod core {
    pub mod hashing {
        pub mod abstractions {
            pub mod gen_range;
            pub mod hasher;
        }
        pub mod types {
            pub mod numbers;
        }
    }
}

pub mod usecases {
    pub mod find_hashes;
}

pub mod external {
    pub mod logging;
}

pub mod view {
    pub mod apps {
        pub mod cli {
            pub mod formatting;
            pub mod functionality;
            pub mod layout;
        }
    }
}
