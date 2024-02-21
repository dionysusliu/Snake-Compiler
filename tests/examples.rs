use snake::runner;

macro_rules! mk_test {
    ($test_name:ident, $file_name:expr, $expected_output:expr) => {
        #[test]
        fn $test_name() -> std::io::Result<()> {
            test_example_file($file_name, $expected_output)
        }
    };
}

macro_rules! mk_fail_test {
    ($test_name:ident, $file_name:expr, $expected_output:expr) => {
        #[test]
        fn $test_name() -> std::io::Result<()> {
            test_example_fail($file_name, $expected_output)
        }
    };
}

/*
 * YOUR TESTS GO HERE
 */


 mk_fail_test!(g1, "g1.garter", "arithmetic expected a number but got a boolean"); /* Garter tests */
 mk_fail_test!(g2, "g2.garter", "comparison expected an integer but got a boolean"); /* Garter tests */
 mk_fail_test!(g3, "g3.garter", "comparison expected an integer but got a floating point"); /* Garter tests */
 mk_fail_test!(g4, "g4.garter", "expected a boolean but got a numbern"); /* Garter tests */
 mk_test!(g5, "g5.garter", "4.3415926"); /* Garter tests */
 mk_test!(g6, "g6.garter", "7.0\n-1.0\n12.0\n0.75"); /* Garter tests */
 mk_fail_test!(g7, "g7.garter", "if logic expected a boolean but got a number"); /* Garter tests */
 mk_fail_test!(g8, "g8.garter", "overflow"); /* Garter tests */
 mk_fail_test!(g9, "g9.garter", "divided by zero"); /* Garter tests */





// IMPLEMENTATION
fn test_example_file(f: &str, expected_str: &str) -> std::io::Result<()> {
    use std::path::Path;
    let p_name = format!("examples/{}", f);
    let path = Path::new(&p_name);

    // Test the compiler
    let tmp_dir = tempfile::TempDir::new()?;
    let mut w = Vec::new();
    match runner::compile_and_run_file(&path, tmp_dir.path(), &mut w) {
        Ok(()) => {
            let stdout = std::str::from_utf8(&w).unwrap();
            assert_eq!(stdout.trim(), expected_str)
        }
        Err(e) => {
            assert!(false, "Expected {}, got an error: {}", expected_str, e)
        }
    }

    Ok(())
}

fn test_example_fail(f: &str, includes: &str) -> std::io::Result<()> {
    use std::path::Path;
    let p_name = format!("examples/{}", f);
    let path = Path::new(&p_name);

    // Test the compiler
    let tmp_dir = tempfile::TempDir::new()?;
    let mut w_run = Vec::new();
    match runner::compile_and_run_file(
        &Path::new(&format!("examples/{}", f)),
        tmp_dir.path(),
        &mut w_run,
    ) {
        Ok(()) => {
            let stdout = std::str::from_utf8(&w_run).unwrap();
            assert!(false, "Expected a failure but got: {}", stdout.trim())
        }
        Err(e) => {
            let msg = format!("{}", e);
            assert!(
                msg.contains(includes),
                "Expected error message to include the string \"{}\" but got the error: {}",
                includes,
                msg
            )
        }
    }

    Ok(())
}
