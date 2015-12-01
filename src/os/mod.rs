cfg_if! {
    if #[cfg(not(target_os="windows"))] {
        mod unix;
        pub use self::unix::symlink as symlink;
    } else {
        mod windows;
        pub use self::windows::symlink as symlink;
    }
}
