#[macro_export]
macro_rules! require_non_zero {
    ($array: expr) => {
        require!(!$array.contains(&0u64), AmmError::ZeroBalance)
    };
}

#[macro_export]
macro_rules! require_not_locked {
    ($locked: expr) => {
        require!(!$locked, AmmError::PoolLocked);
    };
}

// #[macro_export]
// macro_rules! require_has_update_authority {
//     ($x: expr) => {
//         match $x.config.authority{
//             Some(authority)=>{
//                 require_keys_eq!()
//             }
//         }
//     };
// }
