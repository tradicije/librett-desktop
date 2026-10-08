// Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
use librett_application::{
    registry::{https_url, RegistryMedia, MAX_SNAPSHOT_BYTES},
    ApplicationError,
};
use std::{
    io::{Cursor, Read},
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Mutex,
    },
    time::Duration,
};
static DOWNLOAD: Mutex<()> = Mutex::new(());
static DNS_WORKERS: AtomicUsize = AtomicUsize::new(0);
struct DnsSlot;
impl Drop for DnsSlot {
    fn drop(&mut self) {
        DNS_WORKERS.fetch_sub(1, Ordering::SeqCst);
    }
}
fn public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            let n = u32::from(ip);
            ![
                (0, 8),
                (0x0a000000, 8),
                (0x64400000, 10),
                (0x7f000000, 8),
                (0xa9fe0000, 16),
                (0xac100000, 12),
                (0xc0000000, 24),
                (0xc0000200, 24),
                (0xc0586300, 24),
                (0xc0a80000, 16),
                (0xc6120000, 15),
                (0xc6336400, 24),
                (0xcb007100, 24),
                (0xe0000000, 3),
            ]
            .iter()
            .any(|(network, bits)| n & (!0u32 << (32 - bits)) == *network)
        }
        IpAddr::V6(ip) => {
            let s = ip.segments();
            (s[0] & 0xe000) == 0x2000
                && s[0] != 0x2002
                && !(s[0] == 0x3fff && (s[1] & 0xf000) == 0)
                && !(s[0] == 0x2001 && (s[1] < 0x0200 || s[1] == 0x0db8))
        }
    }
}
fn download(value: &str, maximum: usize, mimes: &[&str]) -> Result<Vec<u8>, ApplicationError> {
    let _guard = DOWNLOAD
        .try_lock()
        .map_err(|_| ApplicationError::RegistryFetch)?;
    let url = https_url(value)?;
    let host = url
        .host_str()
        .ok_or(ApplicationError::RegistryFetch)?
        .trim_matches(['[', ']'])
        .to_owned();
    let port = url
        .port_or_known_default()
        .ok_or(ApplicationError::RegistryFetch)?;
    let addresses: Vec<SocketAddr> = if let Ok(ip) = host.parse::<IpAddr>() {
        vec![SocketAddr::new(ip, port)]
    } else {
        if DNS_WORKERS
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                (n < 2).then_some(n + 1)
            })
            .is_err()
        {
            return Err(ApplicationError::RegistryFetch);
        }
        let (sender, receiver) = mpsc::channel();
        let resolver_host = host.clone();
        std::thread::spawn(move || {
            let _slot = DnsSlot;
            let result = (resolver_host.as_str(), port)
                .to_socket_addrs()
                .map(|values| values.take(17).collect::<Vec<_>>());
            let _ = sender.send(result);
        });
        receiver
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| ApplicationError::RegistryFetch)?
            .map_err(|_| ApplicationError::RegistryFetch)?
    };
    if addresses.is_empty()
        || addresses.len() > 16
        || addresses.iter().any(|address| !public_ip(address.ip()))
    {
        return Err(ApplicationError::RegistryFetch);
    }
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .resolve_to_addrs(&host, &addresses)
        .build()
        .map_err(|_| ApplicationError::RegistryFetch)?;
    let response = client
        .get(url)
        .header("Accept-Encoding", "identity")
        .header("Accept", mimes.join(", "))
        .send()
        .map_err(|_| ApplicationError::RegistryFetch)?;
    if response.status() != reqwest::StatusCode::OK
        || response
            .remote_addr()
            .is_none_or(|peer| !addresses.iter().any(|address| address.ip() == peer.ip()))
        || response
            .content_length()
            .is_some_and(|length| length > maximum as u64)
        || response
            .headers()
            .get("content-encoding")
            .is_some_and(|value| value.as_bytes() != b"identity")
    {
        return Err(ApplicationError::RegistryFetch);
    }
    let mime = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(';').next())
        .unwrap_or("")
        .trim();
    if !mimes.contains(&mime) {
        return Err(ApplicationError::RegistryFetch);
    }
    let mut bytes = vec![];
    response
        .take(maximum as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ApplicationError::RegistryFetch)?;
    if bytes.len() > maximum {
        return Err(ApplicationError::RegistryFetch);
    }
    Ok(bytes)
}
pub fn snapshot(url: &str) -> Result<String, ApplicationError> {
    String::from_utf8(download(url, MAX_SNAPSHOT_BYTES, &["application/json"])?)
        .map_err(|_| ApplicationError::InvalidRegistry)
}
pub fn photo(descriptor: &RegistryMedia) -> Result<String, ApplicationError> {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let bytes = download(
        &descriptor.content_url,
        5 * 1024 * 1024,
        &[descriptor.mime_type.as_str()],
    )?;
    if bytes.len() != descriptor.byte_length as usize
        || format!("{:x}", Sha256::digest(&bytes)) != descriptor.content_sha256
    {
        return Err(ApplicationError::RegistryFetch);
    }
    let mut reader = image::ImageReader::new(Cursor::new(&bytes))
        .with_guessed_format()
        .map_err(|_| ApplicationError::InvalidRegistry)?;
    let expected = match descriptor.mime_type.as_str() {
        "image/jpeg" => image::ImageFormat::Jpeg,
        "image/png" => image::ImageFormat::Png,
        _ => return Err(ApplicationError::InvalidRegistry),
    };
    if reader.format() != Some(expected) {
        return Err(ApplicationError::InvalidRegistry);
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(4096);
    limits.max_image_height = Some(4096);
    limits.max_alloc = Some(67108864);
    reader.limits(limits);
    let image = reader
        .decode()
        .map_err(|_| ApplicationError::InvalidRegistry)?;
    if image.width() != descriptor.width
        || image.height() != descriptor.height
        || u64::from(image.width()) * u64::from(image.height()) > 16777216
    {
        return Err(ApplicationError::InvalidRegistry);
    }
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn special_networks_rejected() {
        for value in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "192.168.1.1",
            "198.18.0.1",
            "203.0.113.1",
            "::1",
            "::ffff:127.0.0.1",
            "64:ff9b::7f00:1",
            "2001:db8::1",
            "3fff:fff::1",
            "2002:7f00:1::",
        ] {
            assert!(!public_ip(value.parse().unwrap()), "{value}");
        }
        assert!(public_ip("8.8.8.8".parse().unwrap()));
        assert!(public_ip("2606:4700:4700::1111".parse().unwrap()));
    }
}
