// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Mutex;

mod app;
mod config;
mod i18n;
mod icons;

fn main() -> cosmic::iced::Result {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn,examine=info,warn")).init();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    icons::ICON_CACHE.get_or_init(|| Mutex::new(icons::IconCache::new()));

    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );
    cosmic::app::run::<app::AppModel>(settings, ())
}
