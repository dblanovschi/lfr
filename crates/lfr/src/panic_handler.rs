//! FIXME: docs
use indoc::indoc;

/// Installs the panic handler.
pub fn init()
{
    let settings = better_panic::Settings::default().message(indoc!(
        r#"
        Well, this is embarrassing.

        lfr had a problem and crashed. To help us diagnose the problem you can send us a crash report.
        
        Submit an issue or email with the subject of "lfr Crash Report" and include the backtrace as an attachment.
        - Homepage: https://github.com/dblanovschi/lfr
        - Issue tracker: https://github.com/dblanovschi/lfr/issues
        
        Thank you kindly!
    "#
    )).verbosity(better_panic::Verbosity::Full);

    settings.install();
}
