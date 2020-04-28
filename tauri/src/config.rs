#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub build: _Config__build,
    pub ctx: _Config__ctx,
    pub tauri: _Config__tauri,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__build {
    pub beforeBuildCommand: Cow<'static, str>,
    pub beforeDevCommand: Cow<'static, str>,
    pub devPath: Cow<'static, str>,
    pub distDir: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__ctx {

}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri {
    pub bundle: _Config__tauri__bundle,
    pub edge: _Config__tauri__edge,
    pub embeddedServer: _Config__tauri__embeddedServer,
    pub inliner: _Config__tauri__inliner,
    pub security: _Config__tauri__security,
    pub whitelist: _Config__tauri__whitelist,
    pub window: _Config__tauri__window,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__bundle {
    pub active: bool,
    pub category: Cow<'static, str>,
    pub copyright: Cow<'static, str>,
    pub deb: _Config__tauri__bundle__deb,
    pub exceptionDomain: Cow<'static, str>,
    pub externalBin: Cow<'static, [()]>,
    pub icon: Cow<'static, [Cow<'static, str>]>,
    pub identifier: Cow<'static, str>,
    pub longDescription: Cow<'static, str>,
    pub osx: _Config__tauri__bundle__osx,
    pub resources: Cow<'static, [()]>,
    pub shortDescription: Cow<'static, str>,
    pub targets: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__bundle__deb {
    pub depends: Cow<'static, [()]>,
    pub useBootstrapper: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__bundle__osx {
    pub frameworks: Cow<'static, [()]>,
    pub minimumSystemVersion: Cow<'static, str>,
    pub useBootstrapper: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__edge {
    pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__embeddedServer {
    pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__inliner {
    pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__security {
    pub csp: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__whitelist {
    pub all: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__tauri__window {
    pub fullscreen: bool,
    pub height: i64,
    pub resizable: bool,
    pub title: Cow<'static, str>,
    pub width: i64,
}

pub const CONFIG: Config = Config {
    build: _Config__build {
        beforeBuildCommand: Cow::Borrowed(""),
        beforeDevCommand: Cow::Borrowed(""),
        devPath: Cow::Borrowed("http://localhost:4000"),
        distDir: Cow::Borrowed("examples/communication/dist/"),
    },
    ctx: _Config__ctx {
    },
    tauri: _Config__tauri {
        bundle: _Config__tauri__bundle {
            active: true,
            category: Cow::Borrowed("DeveloperTool"),
            copyright: Cow::Borrowed(""),
            deb: _Config__tauri__bundle__deb {
                depends: Cow::Borrowed(&[]),
                useBootstrapper: false,
            },
            exceptionDomain: Cow::Borrowed(""),
            externalBin: Cow::Borrowed(&[]),
            icon: Cow::Borrowed(&[Cow::Borrowed("icons/32x32.png"), Cow::Borrowed("icons/128x128.png"), Cow::Borrowed("icons/128x128@2x.png"), Cow::Borrowed("icons/icon.icns"), Cow::Borrowed("icons/icon.ico")]),
            identifier: Cow::Borrowed("com.tauri.dev"),
            longDescription: Cow::Borrowed(""),
            osx: _Config__tauri__bundle__osx {
                frameworks: Cow::Borrowed(&[]),
                minimumSystemVersion: Cow::Borrowed(""),
                useBootstrapper: false,
            },
            resources: Cow::Borrowed(&[]),
            shortDescription: Cow::Borrowed(""),
            targets: Cow::Borrowed("all"),
        },
        edge: _Config__tauri__edge {
            active: true,
        },
        embeddedServer: _Config__tauri__embeddedServer {
            active: true,
        },
        inliner: _Config__tauri__inliner {
            active: true,
        },
        security: _Config__tauri__security {
            csp: Cow::Borrowed("default-src blob: data: filesystem: ws: http: https: 'unsafe-eval' 'unsafe-inline'"),
        },
        whitelist: _Config__tauri__whitelist {
            all: true,
        },
        window: _Config__tauri__window {
            fullscreen: false,
            height: 600,
            resizable: true,
            title: Cow::Borrowed("Tauri App"),
            width: 800,
        },
    },
};

#[cfg(debug_assertions)]
impl Config {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/tauri.conf.json");
        Self::load_from(filepath.as_ref()).expect("Failed to load Config.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::serde_json::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl Config {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&CONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&CONFIG))
    }
}
