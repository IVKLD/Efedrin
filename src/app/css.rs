use super::base_css::BASE_CSS;
use super::tag_css::TAG_CSS;

pub fn load_app_css() -> String {
    format!("{}\n{}", BASE_CSS, TAG_CSS)
}
