use os_type;

#[derive(Clone)]
pub enum PackageType {
    Binary,
    Source
}

#[derive(Clone)]
pub struct Url {
    pub url: &'static str,
    pub platform: os_type::OSType,
    pub package_type: PackageType
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
        platform: os_type::OSType::OSX,
        package_type: PackageType::Binary
    };
    platform_urls.push(node_osx);

    for os in vec![os_type::OSType::Arch, os_type::OSType::Ubuntu, os_type::OSType::Redhat] {
        let u = Url {
            url: "https://nodejs.org/dist/v{VERSION}/node-v{VERSION}-linux-x64.tar.gz".into(),
            platform: os,
            package_type: PackageType::Binary
        };
        platform_urls.push(u);
    }

    Language {
        name: "node",
        download_urls: platform_urls
    }
}

pub fn ruby() -> Language {
    let mut platform_urls = Vec::new();

    for os in vec![os_type::OSType::Arch, os_type::OSType::Ubuntu, os_type::OSType::Redhat, os_type::OSType::OSX] {
        let u = Url {
            url: "https://cache.ruby-lang.org/pub/ruby/{VERSION_SHORT}/ruby-{VERSION}.tar.gz",
            platform: os,
            package_type: PackageType::Source
        };
        platform_urls.push(u);
    }
    Language {
        name: "ruby",
        download_urls: platform_urls
    }
}
