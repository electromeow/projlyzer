pub enum License {
  MIT,
  GPLv2,
  GPLv3,
  Apachev2,
  BSD,
  LGPLv2,
  LGPLv3,
  EPLv1,
  EPLv2,
  SimplifiedBSD,
  MSPL,
  CodeProjectOpenLicense,
  MPL,
  AGPLv3,
  AGPLv2,
  NotResolved
}

pub fn find_license(license_text: &String) -> License {
  let license_lower = license_text.to_lowercase();
  if license_lower.contains("permission is hereby granted, free of charge"){
    return License::MIT;
  }
  if license_lower.contains("gnu general public license") {
    if license_lower.contains("version 2"){
      return License::GPLv2;
    }
    if license_lower.contains("version 3"){
      return License::GPLv3;
    }
  }
  if license_lower.contains("apache license") && license_lower.contains("version 2"){
    return License::Apachev2;
  }
  if license_lower.contains("redistribution and use in source and binary forms"){
    if license_lower.contains("3. neither the name of the copyright holder"){
      return License::BSD;
    }
    else {
      return License::SimplifiedBSD;
    }
  }
  if license_lower.contains("gnu lesser general public license") {
    if license_lower.contains("version 2"){
      return License::LGPLv2;
    }
    if license_lower.contains("version 3"){
      return License::LGPLv3;
    }
  }
  if license_lower.contains("eclipse public license"){
    if license_lower.contains("v 1"){
      return License::EPLv1;
    }
    if license_lower.contains("v 2"){
      return License::EPLv2;
    }
  }
  if license_lower.contains("this license governs use of the accompanying software"){
    return License::MSPL;
  }
  if license_lower.contains("this License is intended to allow developers to use the source code and executable files"){
    return License::CodeProjectOpenLicense;
  }
  if license_lower.contains("mozilla public"){
    return License::MPL;
  }
  if license_lower.contains("gnu affero general public license") {
    if license_lower.contains("version 2"){
      return License::AGPLv2;
    }
    if license_lower.contains("version 3"){
      return License::AGPLv3;
    }
  }
  License::NotResolved
}

impl std::fmt::Display for License {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        &License::MIT => write!(f, "MIT"),
        &License::GPLv2 => write!(f, "GNU GPL v2"),
        &License::GPLv3 => write!(f, "GNU GPL v3"),
        &License::Apachev2 => write!(f, "Apache License 2.0"),
        &License::BSD => write!(f, "BSD(3-clause)"),
        &License::LGPLv2 => write!(f, "GNU LGPL v2/2.1"),
        &License::LGPLv3=> write!(f, "GNU LGPL v3"),
        &License::EPLv1 => write!(f, "Eclipse Public License 1.0"),
        &License::EPLv2 => write!(f, "Eclipse Public License 2.0"),
        &License::SimplifiedBSD => write!(f, "Simplified BSD(2-clause), also known as FreeBSD license"),
        &License::MSPL => write!(f, "Microsoft Public License"),
        &License::CodeProjectOpenLicense => write!(f, "Code Project Open License(Non Free/Not Open Source)"),
        &License::MPL => write!(f, "Mozilla Public License"),
        &License::AGPLv2 => write!(f, "GNU AGPL v2"),
        &License::AGPLv3 => write!(f, "GNU AGPL v3"),
        &License::NotResolved => write!(f, "projlyzer couldn't resolve this license. You can open a PR at github.com/electromeow/projlyzer and make projlyzer resolve this license next time!"),
      }
  }
} 