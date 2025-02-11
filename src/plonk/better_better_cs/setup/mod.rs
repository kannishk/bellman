use cfg_if::*;

cfg_if!{
    if #[cfg(feature = "allocator")]{
        pub mod unstable;
        use self::unstable as setup;        
    }else{        
        pub mod stable;        
        use self::stable as setup;
    }
}

use super::*;
pub use self::setup::*;