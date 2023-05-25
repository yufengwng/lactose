use proc_macro::TokenStream;

mod testing;

/// Proc-macro for generating integration test functions for script files in test suite directory.
/// Script files must end with extension ".tl". Expects two arguments separated by comma:
///
/// 1. An identifier for name of top-level module that will hold the tests.
/// 2. A string literal for the path to test suite top-level directory.
///
/// Each subdirectory will map to a new nested module. Each test function will call a
/// `crate::test_script` function that must be implemented by the user and takes one argument for
/// the script path.
#[proc_macro]
pub fn test_suite(input: TokenStream) -> TokenStream {
    testing::proc_test_suite(input)
}
