use clap::{self, crate_authors, crate_version, AppSettings, ArgEnum, Args, Parser, Subcommand};

/// solo2 is the go-to tool to interact with a Solo 2 security key.
///
/// Print more logs by setting env SOLO2_LOG='info' or SOLO2_LOG='debug'.
///
/// Project homepage: https://github.com/solokeys/solo2-cli
#[derive(Parser)]
#[clap(setting(AppSettings::InferSubcommands))]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
pub struct Cli {
    /// Prefer CTAP transport.
    #[clap(global = true, help_heading = "TRANSPORT", long)]
    pub ctap: bool,

    /// Prefer PCSC transport.
    #[clap(global = true, help_heading = "TRANSPORT", long)]
    pub pcsc: bool,

    /// Specify UUID of a Solo 2 device.
    #[clap(global = true, help_heading = "SELECTION", long, short)]
    pub uuid: Option<String>,

    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(subcommand)]
    App(Apps),
    #[clap(subcommand)]
    Pki(Pki),

    #[clap(subcommand)]
    Bootloader(Bootloader),

    /// List all available devices
    #[clap(visible_alias = "ls")]
    List,

    /// Update to latest firmware published by SoloKeys. Warns on Major updates.
    Update {
        /// DANGER! Proceed with major updates without prompt
        #[clap(long, short)]
        yes: bool,
        /// Update all connected SoloKeys Solo 2 devices
        #[clap(long, short)]
        all: bool,
        /// Update to a specific firmware secure boot file (.sb2)
        firmware: Option<String>,
    },
}

#[derive(Subcommand)]
/// Interact with bootloader
pub enum Bootloader {
    /// Reboots (into device if firmware is valid)
    Reboot,
    /// List all available bootloaders
    #[clap(visible_alias = "ls")]
    List,
    // NB: If we convert lpc55-host to clap 3, should be possible
    // to slot in its CLI here.

    // /// Run a sequence of bootloader provision commands defined in the config file
    // Provision {
    //     /// Configuration file containing settings
    //     config: String,
    // },
}

#[derive(Subcommand)]
/// PKI-related
pub enum Pki {
    #[clap(subcommand)]
    Ca(Ca),
    #[cfg(feature = "dev-pki")]
    #[clap(subcommand)]
    Dev(Dev),
}

#[derive(Subcommand)]
/// CA-related
pub enum Ca {
    /// Fetch one of the well-known Solo 2 PKI certificates in DER format
    FetchCertificate {
        /// Name of authority, e.g. R1, T1, S3, etc.
        authority: String,
    },
}

#[derive(Subcommand)]
/// PKI for development
pub enum Dev {
    /// Fetch one of the well-known Solo 2 PKI certificates in DER format
    Fido {
        /// Output file for private P256 key in binary format
        key: String,
        /// Output file for self-signed certificate in DER format
        cert: String,
    },
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// Interact with on-device applications
pub enum Apps {
    #[clap(subcommand)]
    Admin(Admin),
    #[clap(subcommand)]
    Fido(Fido),
    #[clap(subcommand)]
    Ndef(Ndef),
    #[clap(subcommand)]
    Oath(Oath),
    #[clap(subcommand)]
    Piv(Piv),
    #[clap(subcommand)]
    Provision(Provision),
    #[clap(subcommand)]
    Qa(Qa),
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// admin app
pub enum Admin {
    /// Print the application's AID
    Aid,
    /// Reboot device (as Solo 2)
    Reboot,
    /// Reboot device (into Lpc 55 bootloader)
    BootToBootrom,
    /// Return device UUID (not available over CTAP in early firmware)
    Uuid,
    /// Return device firmware version
    Version,
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// FIDO app
pub enum Fido {
    /// FIDO init response
    Init,
    /// FIDO wink
    Wink,
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// NDEF app
pub enum Ndef {
    /// Print the application's AID
    Aid,
    /// NDEF capabilities
    Capabilities,
    /// NDEF data
    Data,
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// OATH app
pub enum Oath {
    /// Print the application's AID
    Aid,
    /// Register new credential
    Register(OathRegister),
    // Authenticate,
    /// Delete existing credential
    Delete {
        /// Label of credential
        label: String,
    },
    /// List all credentials
    List,
    /// Reset OATH app, deleting all credentials
    Reset,
    /// Calculate TOTP for a registered credential
    Totp {
        /// Label of credential
        label: String,
        /// timestamp to use to generate the OTP, as seconds since the UNIX epoch
        timestamp: Option<String>,
    },
}

#[derive(Args)]
pub struct OathRegister {
    /// label to use for the OATH secret, e.g. alice@trussed.dev
    pub label: String,
    /// the actual OATH seed, e.g. JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP
    pub secret: String,

    /// (optional) issuer to use for the OATH credential, e.g., example.com
    #[clap(long, short)]
    pub issuer: Option<String>,

    #[clap(arg_enum, default_value = "sha1", long, short)]
    pub algorithm: OathAlgorithm,
    #[clap(arg_enum, default_value = "totp", long, short)]
    pub kind: OathKind,

    /// (only HOTP) initial counter to use for HOTPs
    #[clap(default_value = "0", long, short)] //, required_if_eq("kind", "hotp"))]
    pub counter: u32,

    /// number of digits to output
    #[clap(default_value = "6", possible_values=["6", "7", "8"], long, short)]
    pub digits: u8,

    /// (only TOTP) period in seconds for which a TOTP is valid
    #[clap(default_value = "30", long, short)] //, required_if_eq("kind", "totp"))]
    pub period: u32,
}

// ignore case?
#[derive(ArgEnum, Clone)]
/// hash algorithm to use in OTP generation
pub enum OathAlgorithm {
    Sha1,
    Sha256,
}

// ignore case?
#[derive(ArgEnum, Clone)]
/// kind of OATH credential to register
pub enum OathKind {
    Hotp,
    Totp,
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// PIV app
pub enum Piv {
    /// Print the application's AID
    Aid,
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// Provision app
pub enum Provision {
    /// Print the application's AID
    Aid,
    /// Generate new Trussed Ed255 attestation key
    GenerateEd255Key,
    /// Generate new Trussed P256 attestation key
    GenerateP256Key,
    /// Generate new Trussed X255 attestation key
    GenerateX255Key,

    /// Store Trussed Ed255 attestation certificate
    StoreEd255Cert {
        /// Certificate in DER format
        der: String,
    },
    /// Store Trussed P256 attestation certificate
    StoreP256Cert {
        /// Certificate in DER format
        der: String,
    },
    /// Store Trussed X255 attestation certificate
    StoreX255Cert {
        /// Certificate in DER format
        der: String,
    },

    /// Store Trussed T1 intermediate public key
    StoreT1Pubkey {
        /// Ed255 public key (raw, 32 bytes)
        bytes: String,
    },
    /// Store FIDO batch attestation certificate
    StoreFidoBatchCert {
        /// Attestation certificate
        cert: String,
    },
    /// Store FIDO batch attestation private key
    StoreFidoBatchKey {
        /// P256 private key in internal format
        bytes: String,
    },

    /// Reformat the internal filesystem
    ReformatFilesystem,

    /// Write binary file to specified path
    WriteFile {
        /// binary data file
        data: String,
        /// path in internal filesystem
        path: String,
    },
}

#[derive(Subcommand)]
#[clap(setting(AppSettings::InferSubcommands))]
/// QA app
pub enum Qa {
    /// Print the application's AID
    Aid,
}

///// Return the "long" format of lpc55's version string.
/////
///// If a revision hash is given, then it is used. If one isn't given, then
///// the SOLO2_CLI_BUILD_GIT_HASH env var is inspected for it. If that isn't set,
///// then a revision hash is not included in the version string returned.
//pub fn long_version(revision_hash: Option<&str>) -> String {
//    // Do we have a git hash?
//    // (Yes, if ripgrep was built on a machine with `git` installed.)
//    let hash = match revision_hash.or(option_env!("SOLO2_CLI_BUILD_GIT_HASH")) {
//        None => String::new(),
//        Some(githash) => format!(" (rev {})", githash),
//    };
//    format!("{}{}", crate_version!(), hash)
//}
