/*
 * Copyright (c) 2024, Stalwart Labs Ltd.
 *
 * This file is part of Stalwart Mail Web-based Admin.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use crate::core::schema::*;

impl Builder<Schemas, ()> {
    pub fn build_tls(self) -> Self {
        self.new_schema("acme")
            .names("ACME provider", "ACME providers")
            .prefix("acme")
            .suffix("directory")
            // Id
            .new_id_field()
            .label("Directory Id")
            .help("Unique identifier for the ACME provider")
            .build()
            // Directory
            .new_field("directory")
            .label("Directory URL")
            .help("The URL of the ACME directory endpoint")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required, Validator::IsUrl])
            .default("https://acme-v02.api.letsencrypt.org/directory")
            .build()
            // Domains
            .new_field("domains")
            .typ(Type::Array)
            .input_check([Transformer::Trim], [Validator::Required])
            .label("Subject names")
            .help("Hostnames covered by this ACME manager")
            .build()
            // Default provider
            .new_field("default")
            .typ(Type::Boolean)
            .label("Default provider")
            .help(concat!(
                "Whether the certificates generated by this provider ",
                "should be the default when no SNI is provided"
            ))
            .build()
            // Contact
            .new_field("contact")
            .label("Contact Email")
            .help(concat!(
                "the contact email address, which is used for important ",
                "communications regarding your ACME account and certificates"
            ))
            .typ(Type::Array)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsEmail],
            )
            .build()
            // Renew before
            .new_field("renew-before")
            .typ(Type::Duration)
            .label("Renew before")
            .help("Determines how early before expiration the certificate should be renewed.")
            .input_check([], [Validator::Required])
            .default("30d")
            .build()
            // Challenge type
            .new_field("challenge")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("tls-alpn-01", "TLS-ALPN-01"),
                    ("dns-01", "DNS-01"),
                    ("http-01", "HTTP-01"),
                ]),
                multi: false,
            })
            .label("Challenge type")
            .help("The ACME challenge type used to validate domain ownership")
            .input_check([], [Validator::Required])
            .default("tls-alpn-01")
            .build()
            // Polling interval (DNS-01)
            .new_field("polling-interval")
            .typ(Type::Duration)
            .label("Polling interval")
            .help("How often to check for DNS records to propagate")
            .display_if_eq("challenge", ["dns-01"])
            .input_check([], [Validator::Required])
            .default("15s")
            // Propagation timeout (DNS-01)
            .new_field("propagation-timeout")
            .label("Propagation timeout")
            .help("How long to wait for DNS records to propagate")
            .default("1m")
            // TTL (DNS-01)
            .new_field("ttl")
            .label("TTL")
            .help("The TTL for the DNS record used in the DNS-01 challenge")
            .default("5m")
            .build()
            // Provider
            .new_field("provider")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("rfc2136-tsig", "RFC2136"),
                    ("cloudflare", "Cloudflare"),
                ]),
                multi: false,
            })
            .label("DNS Provider")
            .help("The DNS provider used to manage DNS records for the DNS-01 challenge")
            .input_check([], [Validator::Required])
            .default("rfc2136-tsig")
            .display_if_eq("challenge", ["dns-01"])
            .build()
            // Secret
            .new_field("secret")
            .typ(Type::Secret)
            .label("Secret")
            .help("The TSIG secret or token used to authenticate with the DNS provider")
            .input_check([], [Validator::Required])
            .display_if_eq("challenge", ["dns-01"])
            .build()
            // Request timeout (DNS-01)
            .new_field("timeout")
            .typ(Type::Duration)
            .label("Timeout")
            .help("Request timeout for the DNS provider")
            .display_if_eq("provider", ["cloudflare"])
            .input_check([], [Validator::Required])
            .default("30s")
            .build()
            // TSIG Algorithm
            .new_field("tsig-algorithm")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("hmac-md5", "HMAC-MD5"),
                    ("gss", "GSS"),
                    ("hmac-sha1", "HMAC-SHA1"),
                    ("hmac-sha224", "HMAC-SHA224"),
                    ("hmac-sha256", "HMAC-SHA256"),
                    ("hmac-sha256-128", "HMAC-SHA256-128"),
                    ("hmac-sha384", "HMAC-SHA384"),
                    ("hmac-sha384-192", "HMAC-SHA384-192"),
                    ("hmac-sha512", "HMAC-SHA512"),
                    ("hmac-sha512-256", "HMAC-SHA512-256"),
                ]),
                multi: false,
            })
            .label("TSIG Algorithm")
            .help("The TSIG algorithm used to authenticate with the DNS provider")
            .input_check([], [Validator::Required])
            .default("hmac-sha512")
            .display_if_eq("provider", ["rfc2136-tsig"])
            // Protocol
            .new_field("protocol")
            .typ(Type::Select {
                source: Source::Static(&[("udp", "UDP"), ("tcp", "TCP")]),
                multi: false,
            })
            .label("Protocol")
            .help("The protocol used to communicate with the DNS server")
            .default("udp")
            // Port
            .new_field("port")
            .typ(Type::Input)
            .label("Port")
            .help("The port used to communicate with the DNS server")
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsPort],
            )
            .default("53")
            // Host
            .new_field("host")
            .label("Host")
            .help("The IP address of the DNS server")
            .placeholder("127.0.0.1")
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsIpOrMask],
            )
            // Key
            .new_field("key")
            .label("Key")
            .help("The TSIG key used to authenticate with the DNS provider")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            // Account key
            .new_field("account-key")
            .label("Account key")
            .help(concat!(
                "The account key used to authenticate with the ACME ",
                "provider (auto-generated)"
            ))
            .typ(Type::Secret)
            .build()
            // Account key
            .new_field("cert")
            .label("TLS Certificate")
            .help(concat!(
                "The TLS certificate generated by the ACME provider ",
                "(auto-generated, do not modify)"
            ))
            .typ(Type::Secret)
            .build()
            // Lists
            .list_title("ACME providers")
            .list_subtitle("Manage ACME TLS certificate providers")
            .list_fields(["_id", "contact", "renew-before", "default"])
            // Form
            .new_form_section()
            .title("ACME provider")
            .fields([
                "_id",
                "directory",
                "challenge",
                "contact",
                "domains",
                "renew-before",
                "default",
            ])
            .build()
            .new_form_section()
            .title("DNS settings")
            .display_if_eq("challenge", ["dns-01"])
            .fields([
                "provider",
                "host",
                "port",
                "protocol",
                "tsig-algorithm",
                "key",
                "secret",
                "polling-interval",
                "propagation-timeout",
                "ttl",
                "timeout",
            ])
            .build()
            .new_form_section()
            .title("Certificate")
            .fields(["account-key", "cert"])
            .build()
            .build()
            // ---- TLS certificates ----
            .new_schema("certificate")
            .reload_prefix("certificate")
            .names("certificate", "certificates")
            .prefix("certificate")
            .suffix("cert")
            // Id
            .new_id_field()
            .label("Certificate Id")
            .help("Unique identifier for the TLS certificate")
            .build()
            // Default provider
            .new_field("default")
            .typ(Type::Boolean)
            .label("Default certificate")
            .help(concat!(
                "Whether this certificate ",
                "should be the default when no SNI is provided"
            ))
            .build()
            // Cert
            .new_field("cert")
            .label("Certificate")
            .typ(Type::Text)
            .help("TLS certificate in PEM format")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            // PK
            .new_field("private-key")
            .label("Private Key")
            .typ(Type::Text)
            .help("Private key in PEM format")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            .new_field("subjects")
            .typ(Type::Array)
            .input_check([Transformer::Trim], [Validator::IsDomain])
            .label("Subject Alternative Names")
            .help("Subject Alternative Names (SAN) for the certificate")
            .build()
            .list_title("TLS certificates")
            .list_subtitle("Manage TLS certificates")
            .list_fields(["_id", "subjects", "default"])
            .new_form_section()
            .title("TLS certificate")
            .fields(["_id", "cert", "private-key", "subjects", "default"])
            .build()
            .build()
            // ---- TLS settings ----
            .new_schema("tls")
            // TLS fields
            .add_tls_fields(false)
            // Forms
            .new_form_section()
            .title("Default TLS options")
            .fields([
                "server.tls.disable-protocols",
                "server.tls.disable-ciphers",
                "server.tls.timeout",
                "server.tls.ignore-client-order",
            ])
            .build()
            .build()
    }
}

impl Builder<Schemas, Schema> {
    pub fn add_tls_fields(self, is_listener: bool) -> Self {
        let do_override: &'static [&'static str] =
            if is_listener { &["true"][..] } else { &[][..] };

        // Ignore client order
        self.new_field(if is_listener {
            "tls.ignore-client-order"
        } else {
            "server.tls.ignore-client-order"
        })
        .label("Ignore client order")
        .help("Whether to ignore the client's cipher order")
        .typ(Type::Boolean)
        .default("true")
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Timeout
        .new_field(if is_listener {
            "tls.timeout"
        } else {
            "server.tls.timeout"
        })
        .label("Handshake Timeout")
        .help("TLS handshake timeout")
        .typ(Type::Duration)
        .default("1m")
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Protocols
        .new_field(if is_listener {
            "tls.disable-protocols"
        } else {
            "server.tls.disable-protocols"
        })
        .label("Disabled Protocols")
        .help("Which TLS protocols to disable")
        .typ(Type::Select {
            multi: true,
            source: Source::Static(TLS_PROTOCOLS),
        })
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Ciphersuites
        .new_field(if is_listener {
            "tls.disable-ciphers"
        } else {
            "server.tls.disable-ciphers"
        })
        .label("Disabled Ciphersuites")
        .help("Which ciphersuites to disable")
        .typ(Type::Select {
            multi: true,
            source: Source::Static(TLS_CIPHERSUITES),
        })
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
    }
}

pub static TLS_PROTOCOLS: &[(&str, &str)] = &[
    ("TLSv1.2", "TLS version 1.2"),
    ("TLSv1.3", "TLS version 1.3"),
];

pub static TLS_CIPHERSUITES: &[(&str, &str)] = &[
    ("TLS13_AES_256_GCM_SHA384", "TLS1.3 AES256 GCM SHA384"),
    ("TLS13_AES_128_GCM_SHA256", "TLS1.3 AES128 GCM SHA256"),
    (
        "TLS13_CHACHA20_POLY1305_SHA256",
        "TLS1.3 CHACHA20 POLY1305 SHA256",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "ECDHE ECDSA AES256 GCM SHA384",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "ECDHE ECDSA AES128 GCM SHA256",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "ECDHE ECDSA CHACHA20 POLY1305 SHA256",
    ),
    (
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "ECDHE RSA AES256 GCM SHA384",
    ),
    (
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "ECDHE RSA AES128 GCM SHA256",
    ),
    (
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "ECDHE RSA CHACHA20 POLY1305 SHA256",
    ),
];
