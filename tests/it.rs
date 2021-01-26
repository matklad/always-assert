#[cfg(any(debug_assertions, feature = "force"))]
mod armed {
    use always_assert::{always, never};

    #[test]
    #[should_panic = "assertion failed: 2 + 2 == 5"]
    fn syntax1() {
        always!(2 + 2 == 5);
    }

    #[test]
    #[should_panic = "custom"]
    fn syntax2() {
        always!(2 + 2 == 5, "custom");
    }

    #[test]
    #[should_panic = "custom 92"]
    fn syntax3() {
        always!(2 + 2 == 5, "custom {}", 92);
    }

    #[test]
    #[should_panic = "assertion failed: !(2 + 2 == 4)"]
    fn syntax4() {
        never!(2 + 2 == 4);
    }

    #[test]
    #[should_panic = "custom"]
    fn syntax5() {
        never!(2 + 2 == 4, "custom");
    }

    #[test]
    #[should_panic = "custom 92"]
    fn syntax6() {
        never!(2 + 2 == 4, "custom {}", 92);
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
