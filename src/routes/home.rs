use std::{collections::HashMap, sync::Arc};

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

#[function_component(GitHubIcon)]
fn github_icon() -> Html {
    html! {
        <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="32" height="32">
            <title>{"GitHub"}</title>
            <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
        </svg>
    }
}

#[function_component(YouTubeIcon)]
fn youtube_icon() -> Html {
    html! {
        <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="32" height="32">
            <title>{"YouTube"}</title>
            <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/>
        </svg>
    }
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
            } else if asset.name.ends_with("-Setup.exe") {
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
                    <div style="display: flex; justify-content: center; gap: 20px; margin-top: 20px; margin-bottom: 20px;">
                        <a href="https://github.com/Moulberry/PandoraLauncher" target="_blank" rel="noopener noreferrer" class="social-icon">
                            <GitHubIcon />
                        </a>
                        <a href="https://www.youtube.com/@TheKidReturnsGaming" target="_blank" rel="noopener noreferrer" class="social-icon">
                            <YouTubeIcon />
                        </a>
                    </div>
                </ybc::Container>
                </ybc::Section>

                <div style="display: flex; flex-direction: column; align-items: center;">
                {{
                    if operating_system == OperatingSystem::Windows {
                        html! {
                            <div class="column is-one-third">
                                <DownloadLink name="Download Windows Installer (.exe)" link={releases_by_type.get(&DownloadType::WindowsInstaller).cloned()}/>
                            </div>
                        }
                    } else if operating_system == OperatingSystem::MacOS {
                        html! {
                            <div class="column is-one-third">
                                <DownloadLink name="Download macOS Installer (.dmg)" link={releases_by_type.get(&DownloadType::MacInstaller).cloned()}/>
                            </div>
                        }
                    } else {
                        Default::default()
                    }
                }}
                <div class="column is-one-third">
                    <a href="#downloads">
                        <ybc::Button classes="is-fullwidth">{"View downloads"}</ybc::Button>
                    </a>
                </div>
                </div>

                <ybc::Section>
                <ybc::Tile classes="is-vertical" ctx={Ancestor}>
                    <ybc::Tile>
                        <ybc::Tile ctx={Parent} classes="is-vertical" size={ybc::TileSize::Four}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"Instance Management"}
                                </ybc::Subtitle>
                                <p>{"Easily manage instances and mods. Pandora's unique approach to modpacks makes them simple to manage and update"}</p>
                            </ybc::Tile>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"File Syncing"}
                                </ybc::Subtitle>
                                <p>{"Automatically sync files and folders across instances: options.txt, servers.dat, saves, mod configs, and more"}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent} size={ybc::TileSize::Eight}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <img src="screenshots/instance.png"/>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                    <ybc::Tile>
                        <ybc::Tile ctx={Parent} size={ybc::TileSize::Eight}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <img src="screenshots/modrinth.png"/>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent} classes="is-vertical" size={ybc::TileSize::Four}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"Content Browser"}
                                </ybc::Subtitle>
                                <p>{"Install mods, modpacks, and more directly through the launcher from Modrinth (CurseForge support coming soon)"}</p>
                            </ybc::Tile>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"Content Deduplication"}
                                </ybc::Subtitle>
                                <p>{"When installed through the launcher, Pandora will automatically deduplicate installed mods/resourcepacks/etc. using hard links to save space"}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                    <ybc::Tile>
                        <ybc::Tile ctx={Parent} classes="is-vertical" size={ybc::TileSize::Four}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"Game Output"}
                                </ybc::Subtitle>
                                <p>{"Pandora has a super responsive game output log with no size limit. Supports searching and uploading to mclo.gs"}</p>
                            </ybc::Tile>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                    {"Secure"}
                                </ybc::Subtitle>
                                <p>{"Stores account credentials using platform keyrings, automatically redacts sensitive information from logs and avoids automatic updates for manually installed mods"}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent} size={ybc::TileSize::Eight}>
                            <ybc::Tile ctx={Child} classes="notification is-primary">
                                <img src="screenshots/gameoutput.png"/>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                    <div id="downloads" style="display: flex; flex-direction: column; align-items: center; padding-top: 40px;">
                    <ybc::Subtitle size={ybc::HeaderSize::Is2} classes="has-text-white">
                        {"Downloads"}
                    </ybc::Subtitle>
                    </div>
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
                </ybc::Section>
                </ybc::Container>
            }}>
        </ybc::Hero>

        </>
    }
}
