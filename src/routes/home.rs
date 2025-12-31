use std::{collections::HashMap, convert::TryFrom, sync::Arc};

use serde::Deserialize;
use ybc::{TileCtx::{Ancestor, Child, Parent}};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Debug, Clone, Deserialize)]
struct GitHubReleases {
    assets: Vec<GitHubReleaseAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubReleaseAsset {
    name: Arc<str>,
    browser_download_url: Arc<str>,
}

#[derive(Properties, PartialEq)]
struct DownloadLinkParams {
    name: String,
    link: Option<Arc<str>>,
}

#[function_component(DownloadLink)]
fn download_link(params: &DownloadLinkParams) -> Html {
    if let Some(link) = &params.link {
        html! {
            <a download="true" href={String::from(&**link)}>
                <ybc::Button classes="is-fullwidth is-link">
                    {&params.name}
                </ybc::Button>
            </a>
        }
    } else {
        html! {
            <ybc::Button classes="is-fullwidth is-link">
                {&params.name}
            </ybc::Button>
        }
    }
}

#[derive(PartialEq)]
enum OperatingSystem {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

#[derive(Hash, PartialEq, Eq, PartialOrd)]
enum DownloadType {
    WindowsInstaller,
    WindowsPortable,
    LinuxDebianInstaller,
    LinuxAppImage,
    LinuxPortable,
    MacInstaller,
    MacPortable,
}

#[function_component(Home)]
pub fn home() -> Html {
    let releases = use_async_with_options(
        async {
            let releases: Option<GitHubReleases> = crate::services::request::get("https://api.github.com/repos/Moulberry/PandoraLauncher/releases/latest").await;
            releases.ok_or(())
        },
        UseAsyncOptions::enable_auto()
    );

    let mut releases_by_type = HashMap::new();

    if let Some(data) = &releases.data {
        for asset in &data.assets {
            let download_type = if asset.name.ends_with(".dmg") {
                DownloadType::MacInstaller
            } else if asset.name.ends_with(".AppImage") {
                DownloadType::LinuxAppImage
            } else if asset.name.ends_with(".deb") {
                DownloadType::LinuxDebianInstaller
            } else if asset.name.ends_with("-setup.exe") {
                DownloadType::WindowsInstaller
            } else if asset.name.ends_with(".exe") {
                DownloadType::WindowsPortable
            } else if asset.name.contains("-macOS") {
                DownloadType::MacPortable
            } else if asset.name.contains("-Linux") {
                DownloadType::LinuxPortable
            } else {
                log::info!("Unknown download type for filename: {}", &asset.name);
                continue;
            };

            releases_by_type.insert(download_type, asset.browser_download_url.clone());
        }
    }

    let operating_system = if let Ok(user_agent) = web_sys::window().unwrap().navigator().user_agent() {
        if user_agent.contains("Mac") {
            OperatingSystem::MacOS
        } else if user_agent.contains("Win") {
            OperatingSystem::Windows
        } else if user_agent.contains("Linux") {
            OperatingSystem::Linux
        } else {
            OperatingSystem::Unknown
        }
    } else {
        OperatingSystem::Unknown
    };

    html! {
        <>

        <ybc::Hero
            classes="is-dark"
            size={ybc::HeroSize::FullheightWithNavbar}
            body={html!{
                <ybc::Container classes="is-centered">
                <ybc::Section>
                <ybc::Container classes="has-text-centered">
                    <span class={classes!("hero-banner-text")}>
                        <img src="pandora_main.svg" style="width: 45%"/>
                    </span>
                    <ybc::Subtitle size={ybc::HeaderSize::Is3}>
                        {"Pandora is a modern Minecraft launcher that balances ease-of-use with powerful instance management features "}
                    </ybc::Subtitle>
                </ybc::Container>
                </ybc::Section>

                {{
                    if operating_system == OperatingSystem::Windows {
                        html! {
                            <div style="display: flex; justify-content: center;">
                                <div style="width: 30%">
                                    <DownloadLink name="Download Windows Installer (.exe)" link={releases_by_type.get(&DownloadType::WindowsInstaller).cloned()}/>
                                </div>
                            </div>
                        }
                    } else if operating_system == OperatingSystem::MacOS {
                        html! {
                            <div style="display: flex; justify-content: center;">
                                <div style="width: 30%">
                                    <DownloadLink name="Download macOS Installer (.dmg)" link={releases_by_type.get(&DownloadType::MacInstaller).cloned()}/>
                                </div>
                            </div>
                        }
                    } else {
                        Default::default()
                    }
                }}

                // {
                //     format!("{:?}", releases.data)
                // }
                <ybc::Section>
                <ybc::Tile ctx={Ancestor}>
                    <ybc::Tile classes="is-vertical">
                        <ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"Windows x64"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Installer .exe" link={releases_by_type.get(&DownloadType::WindowsInstaller).cloned()}/>
                                    <DownloadLink name="Portable Executable .exe" link={releases_by_type.get(&DownloadType::WindowsPortable).cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"Linux x64"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Debian Installer .deb" link={releases_by_type.get(&DownloadType::LinuxDebianInstaller).cloned()}/>
                                    <DownloadLink name="AppImage .AppImage" link={releases_by_type.get(&DownloadType::LinuxAppImage).cloned()}/>
                                    <DownloadLink name="Portable Executable" link={releases_by_type.get(&DownloadType::LinuxPortable).cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"macOS"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Installer .dmg" link={releases_by_type.get(&DownloadType::MacInstaller).cloned()}/>
                                    <DownloadLink name="Portable Executable" link={releases_by_type.get(&DownloadType::MacPortable).cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                </ybc::Tile>
                </ybc::Section>
                </ybc::Container>
            }}>
        </ybc::Hero>

        </>
    }
}
