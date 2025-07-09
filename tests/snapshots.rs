use insta::Settings;

/// Configure insta settings for consistent snapshot behavior
pub fn configure_insta() -> Settings {
    let mut settings = Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.set_omit_expression(true);
    settings.set_snapshot_suffix("snap");
    settings
}