use capture_logger::{begin_capture, end_capture, pop_captured, Level};

#[test]
fn test_capture() {
    begin_capture();
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("OK");
        let record = pop_captured().unwrap();
        println!("{:?}", record);
        assert_eq!("OK", record.message());
        assert_eq!(Level::Trace, record.level());
    }
    assert!(pop_captured().is_none());
}

#[test]
fn test_log_before_capture() {
    log::trace!("OK");

    begin_capture();
    assert!(pop_captured().is_none());
}

#[test]
#[should_panic(expected = "already captured.")]
fn test_begin_twice() {
    begin_capture();
    begin_capture();
}

#[test]
#[should_panic(expected = "not captured.")]
fn test_pop_without_capture() {
    pop_captured();
}

#[test]
#[should_panic(expected = "not captured.")]
fn test_pop_after_end_capture() {
    begin_capture();
    end_capture();
    pop_captured();
}
