#[cfg(any(debug_assertions, feature = "force"))]
mod armed {
    use always_assert::{always, never};

    #[test]
    #[should_panic = "assertion failed: 2 + 2 == 5"]
    fn syntax1() {
        if always!(2 + 2 == 5) {
            loop {}
        }
    }

    #[test]
    #[should_panic = "custom"]
    fn syntax2() {
        if always!(2 + 2 == 5, "custom") {
            loop {}
        }
    }

    #[test]
    #[should_panic = "custom 92"]
    fn syntax3() {
        if always!(2 + 2 == 5, "custom {}", 92) {
            loop {}
        }
    }

    #[test]
    #[should_panic = "assertion failed: !(2 + 2 == 4)"]
    fn syntax4() {
        if never!(2 + 2 == 4) {
            loop {}
        }
    }

    #[test]
    #[should_panic = "custom"]
    fn syntax5() {
        if never!(2 + 2 == 4, "custom") {
            loop {}
        }
    }

    #[test]
    #[should_panic = "custom 92"]
    fn syntax6() {
        if never!(2 + 2 == 4, "custom {}", 92) {
            loop {}
        }
    }
}

#[cfg(all(not(debug_assertions), not(feature = "force")))]
mod disarmed {
    use always_assert::{always, never};

    #[test]
    fn syntax1() {
        assert!(!always!(false));
    }

    #[test]
    fn syntax2() {
        assert!(never!(true));
    }
}
