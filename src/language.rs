use os_type;

#[derive(Clone)]
pub struct Url {
    pub url: &'static str,
    pub platform: os_type::OSType
}

#[derive(Clone)]
pub struct Language {
    pub name: &'static str,
    pub download_urls: Vec<Url>
}

pub fn nodejs() -> Language {
    let mut platform_urls = Vec::new();

    let node_osx = Url {
        url: "https://nodejs.org/dist/v{VERSION}/node-v{VERSION}-darwin-x64.tar.gz".into(),
        platform: os_type::OSType::OSX
    };
    platform_urls.push(node_osx);

    for os in vec![os_type::OSType::Arch, os_type::OSType::Ubuntu, os_type::OSType::Redhat] {
        let u = Url {
            url: "https://nodejs.org/dist/v{VERSION}/node-v{VERSION}-linux-x64.tar.gz".into(),
            platform: os
        };
        platform_urls.push(u);
    }

    Language {
        name: "node",
        download_urls: platform_urls
    }
}

pub fn ruby() -> Language {
    Language {
        name: "ruby",
        download_urls: vec!()
    }
}
