use std::collections::HashSet;

pub fn common_email_domains() -> HashSet<&'static str> {
    [
    "sbcglobal.net",
    "gmail.com",
    "yahoo.com",
    "hotmail.com",
    "aol.com",
    "outlook.com",
    "msn.com",
    "icloud.com",
    "rocketmail.com",
    "live.com",
    "comcast.net",
    "cox.net",
    "me.com",
    "gmx.com",
    "ymail.com",
    "mail.com",
    "mac.com",
    "verizon.net",
    "googlemail.com",
    "sbcglobal.net",
    "cox.net",
    "att.net",
    "bellsouth.net",
    "rocketmail.com",
    "earthlink.net",
    "optonline.net",
    "juno.com",
    "protonmail.com"].iter().cloned().collect()
}

