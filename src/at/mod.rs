/// Enum of all the valid AT command identifiers for the XBee 3 802.15.4 modules.
/// 
/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 148)
#[derive(strum_macros::Display)]
pub enum Identifier {
    // Networking
    OperatingChannel,
    ExtendedPANID,
    MACMode,
    CompatibilityOptions,

    // Discovery
    NodeIdentifier,
    DeviceTypeIdentifier,
    NodeDiscoverTimeout,
    NetworkDiscoveryOptions,
    NetworkDiscover,
    DiscoverNode,
    ActiveScan,

    // Coordinator/End Device
    DeviceRole,
    EndDeviceAssociation,
    CoordinatorAssociation,
    ScanChannels,
    ScanDuration,
    ForceDisassociation,
    AssociationIndication,

    // 802.15.4 Addressing
    SerialNumberHigh,
    SerialNumberLow,
    SourceAddress16Bit,
    DestinationAddressHigh,
    DestinationAddressLow,
    XBeeRetries,
    TransmitOptions,
    MaximumPacketPayloadBytes,

    // Security
    EncryptionEnable,
    AESEncryptionKey,
    DisableFeatures,
    OTAUpgradeServer,

    // Secure Session
    SecureAccess,
    SecureSessionSalt,
    SecureSessionVerifierV,
    SecureSessionVerifierW,
    SecureSessionVerifierX,
    SecureSessionVerifierY,

    // RF interfacing
    TXPowerLevel,
    OutputPower, // dBm
    CCAThreshold,
    RandomDelaySlots,

    // MAC diagnostics
    LastPacketRSSI,
    ACKFailures,
    CCAFailures,
    EnergyDetect,

    // Sleep settings
    SleepMode,
    CyclicSleepPeriod,
    CyclicSleepWakeTime,
    DisassociatedCyclicSleepPeriod,
    NumberOfSleepPeriods,
    SleepOptions,
    ForcePoll,

    // MicroPython
    PythonStartup,
    MicroPythonCommand,

    // File System
    FileSystem,
    FileSystemPublicKey,

    // Bluetooth Low Energy (BLE)
    BluetoothEnable,
    BluetoothMACAddress,
    BluetoothIdentifier,
    BluetoothPower,
    SRPSalt,
    SRPSaltVerifierV,
    SRPSaltVerifierW,
    SRPSaltVerifierX,
    SRPSaltVerifierY,

    // API configuration
    APIEnable,
    APIOutputOptions,
    ExtendedAPIOptions,

    // UART interface
    UARTBaudRate,
    Parity,
    StopBits,
    FlowControlThreshold,
    PacketizationTimeout,
    
    // AT Command options
    CommandCharacter,
    CommandModeTimeout,
    GuardTimes,
    ExitCommandMode,

    // UART pin configuration
    DIO6Configuration,
    DIO7Configuration,
    DIO13Configuration,
    DIO14Configuration,

    // SMT/MMT SPI interface
    DIO15Configuration,
    DIO16Configuration,
    DIO17Configuration,
    DIO18Configuration,
    DIO19Configuration,

    // I/O settings 
    DIO0Configuration,
    CommissioningButton,
    DIO1Configuration,
    DIO2Configuration,
    DIO3Configuration,
    DIO4Configuration,
    DIO5Configuration,
    DIO8Configuration,
    DIO9Configuration,
    DIO10Configuration,
    DIO11Configuration,
    DIO12Configuration,
    PullUpDownResistorEnable,
    PullUpDownDirection,
    PWM0DutyCycle,
    PWM1DutyCycle,
    RSSIPWMTimer,
    AssociateLEDBlinkTime,

    // I/O sampling
    IOSample,
    SampleRate,
    DIOChangeDetect,
    AnalogVoltageReference,
    SamplesBeforeTX,
    SleepSampleRate,
    DigitalOutputLevel,

    // I/O line passing
    IOInputAddress,
    IOOutputEnable,
    
    D0TimeoutTimer,
    
    D1OutputTimeoutTimer,
    D2OutputTimeoutTimer,
    D3OutputTimeoutTimer,
    D4OutputTimeoutTimer,
    D5OutputTimeoutTimer,
    D6OutputTimeoutTimer,
    D7OutputTimeoutTimer,
    
    D8OutputTimer,
    D9OutputTimer,

    P0OutputTimer,
    P1OutputTimer,
    P2OutputTimer,

    PWMOutputTimeout,

    // Location
    LocationXLatitude,
    LocationYLongitude,
    LocationZElevation,

    // Diagnostic commands
    FirmwareVersion,
    VersionLong,
    BootloaderVersion,
    HardwareVersion,
    PowerVariant,
    HardwareSoftwareCompatibility,
    SupplyVoltage,
    ModuleTemperature,
    ConfigurationCRC,
    InvokeBootloader,
    ManufacturingDate,

    // Memory access 
    SoftwareReset,
    ApplyChanges,
    Write,
    RestoreDefaults,

    // Custom Default 
    SetCustomDefault,
    ClearCustomDefaults,
    RestoreFactoryDefaults,
}

impl Identifier {
    pub fn code(&self) -> &'static str {
        match self {
            // Networking
            Self::OperatingChannel => "CH",
            Self::ExtendedPANID => "ID",
            Self::MACMode => "MM",
            Self::CompatibilityOptions => "C8",

            // Discovery
            Self::NodeIdentifier => "NI",
            Self::DeviceTypeIdentifier => "DD",
            Self::NodeDiscoverTimeout => "NT",
            Self::NetworkDiscoveryOptions => "NO",
            Self::NetworkDiscover => "ND",
            Self::DiscoverNode => "DN",
            Self::ActiveScan => "AS",

            // Coordinator/End Device
            Self::DeviceRole => "CE",
            Self::EndDeviceAssociation => "A1",
            Self::CoordinatorAssociation => "A2",
            Self::ScanChannels => "SC",
            Self::ScanDuration => "SD",
            Self::ForceDisassociation => "DA",
            Self::AssociationIndication => "AI",

            // 802.15.4 Addressing
            Self::SerialNumberHigh => "SH",
            Self::SerialNumberLow => "SL",
            Self::SourceAddress16Bit => "MY",
            Self::DestinationAddressHigh => "DH",
            Self::DestinationAddressLow => "DL",
            Self::XBeeRetries => "RR",
            Self::TransmitOptions => "TO",
            Self::MaximumPacketPayloadBytes => "NP",

            // Security
            Self::EncryptionEnable => "EE",
            Self::AESEncryptionKey => "KY",
            Self::DisableFeatures => "DM",
            Self::OTAUpgradeServer => "US",

            // Secure Session
            Self::SecureAccess => "SA",
            Self::SecureSessionSalt => "*S",
            Self::SecureSessionVerifierV => "*V",
            Self::SecureSessionVerifierW => "*W",
            Self::SecureSessionVerifierX => "*X",
            Self::SecureSessionVerifierY => "*Y",

            // RF interfacing
            Self::TXPowerLevel => "PL",
            Self::OutputPower => "PP", // dBm
            Self::CCAThreshold => "CA",
            Self::RandomDelaySlots => "RN",

            // MAC diagnostics
            Self::LastPacketRSSI => "DB",
            Self::ACKFailures => "EA",
            Self::CCAFailures => "EC",
            Self::EnergyDetect => "ED",

            // Sleep settings
            Self::SleepMode => "SM",
            Self::CyclicSleepPeriod => "SP",
            Self::CyclicSleepWakeTime => "ST",
            Self::DisassociatedCyclicSleepPeriod => "DP",
            Self::NumberOfSleepPeriods => "SN",
            Self::SleepOptions => "SO",
            Self::ForcePoll => "FP",

            // MicroPython
            Self::PythonStartup => "PS",
            Self::MicroPythonCommand => "PY",

            // File System
            Self::FileSystem => "FS",
            Self::FileSystemPublicKey => "FK",

            // Bluetooth Low Energy (BLE)
            Self::BluetoothEnable => "BT",
            Self::BluetoothMACAddress => "BL",
            Self::BluetoothIdentifier => "BI",
            Self::BluetoothPower => "BP",
            Self::SRPSalt => "$S",
            Self::SRPSaltVerifierV => "*V",
            Self::SRPSaltVerifierW => "*W",
            Self::SRPSaltVerifierX => "*X",
            Self::SRPSaltVerifierY => "*Y",

            // API configuration
            Self::APIEnable => "AP",
            Self::APIOutputOptions => "AO",
            Self::ExtendedAPIOptions => "AZ",

            // UART interface
            Self::UARTBaudRate => "BD",
            Self::Parity => "NB",
            Self::StopBits => "SB",
            Self::FlowControlThreshold => "FT",
            Self::PacketizationTimeout => "RO",
            
            // AT Command options
            Self::CommandCharacter => "CC",
            Self::CommandModeTimeout => "CT",
            Self::GuardTimes => "GT",
            Self::ExitCommandMode => "CN",

            // UART pin configuration
            Self::DIO6Configuration => "D6",
            Self::DIO7Configuration => "D7",
            Self::DIO13Configuration => "P3",
            Self::DIO14Configuration => "P4",

            // SMT/MMT SPI interface
            Self::DIO15Configuration => "P5",
            Self::DIO16Configuration => "P6",
            Self::DIO17Configuration => "P7",
            Self::DIO18Configuration => "P8",
            Self::DIO19Configuration => "P9",

            // I/O settings 
            Self::DIO0Configuration => "D0",
            Self::CommissioningButton => "CB",
            Self::DIO1Configuration => "D1",
            Self::DIO2Configuration => "D2",
            Self::DIO3Configuration => "D3",
            Self::DIO4Configuration => "D4",
            Self::DIO5Configuration => "D5",
            Self::DIO8Configuration => "D8",
            Self::DIO9Configuration => "D9",
            Self::DIO10Configuration => "P0",
            Self::DIO11Configuration => "P1",
            Self::DIO12Configuration => "P2",
            Self::PullUpDownResistorEnable => "PR",
            Self::PullUpDownDirection => "PD",
            Self::PWM0DutyCycle => "M0",
            Self::PWM1DutyCycle => "M1",
            Self::RSSIPWMTimer => "RP",
            Self::AssociateLEDBlinkTime => "LT",

            // I/O sampling
            Self::IOSample => "IS",
            Self::SampleRate => "IR",
            Self::DIOChangeDetect => "IC",
            Self::AnalogVoltageReference => "AV",
            Self::SamplesBeforeTX => "IT",
            Self::SleepSampleRate=> "IF",
            Self::DigitalOutputLevel => "IO",

            // I/O line passing
            Self::IOInputAddress => "IA",
            Self::IOOutputEnable => "IU",
            
            Self::D0TimeoutTimer => "T0",
            
            Self::D1OutputTimeoutTimer => "T1",
            Self::D2OutputTimeoutTimer => "T2",
            Self::D3OutputTimeoutTimer => "T3",
            Self::D4OutputTimeoutTimer => "T4",
            Self::D5OutputTimeoutTimer => "T5",
            Self::D6OutputTimeoutTimer => "T6",
            Self::D7OutputTimeoutTimer => "T7",
            
            Self::D8OutputTimer => "T8",
            Self::D9OutputTimer => "T9",

            Self::P0OutputTimer => "Q0",
            Self::P1OutputTimer => "Q1",
            Self::P2OutputTimer => "Q2",

            Self::PWMOutputTimeout => "PT",

            // Location
            Self::LocationXLatitude => "LX",
            Self::LocationYLongitude => "LY",
            Self::LocationZElevation => "LZ",

            // Diagnostic commands
            Self::FirmwareVersion => "VR",
            Self::VersionLong => "VL",
            Self::BootloaderVersion => "VH",
            Self::HardwareVersion => "HV",
            Self::PowerVariant => "R?",
            Self::HardwareSoftwareCompatibility => "%C",
            Self::SupplyVoltage => "%V",
            Self::ModuleTemperature => "TP",
            Self::ConfigurationCRC => "CK",
            Self::InvokeBootloader => "%P",
            Self::ManufacturingDate => "D%",

            // Memory access 
            Self::SoftwareReset => "FR",
            Self::ApplyChanges => "AC",
            Self::Write => "WR",
            Self::RestoreDefaults => "RE",

            // Custom Default 
            Self::SetCustomDefault => "%F",
            Self::ClearCustomDefaults => "!C",
            Self::RestoreFactoryDefaults => "R1",
        }
    }
}

pub struct Command<'a> {
    /// The identifier to write to start the AT command.
    pub command: Identifier,

    /// The optional parameter to supply to the command.
    pub parameter: Option<&'a [u8]>, // pub parameter: Option<Box<[u8]>>,

    /// Configures the number of carrage returns to write after the command.
    /// 
    /// This is for configuring commands such as the "+++" AT mode transition
    /// command that requre no carriage returns, or a couple others that need
    /// multiple carrage returns.
    pub carriage_returns: usize,
}

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.e. 148)
pub enum Channel {
    Channel11 = 0x0B,
    Channel12 = 0x0C,
    Channel13 = 0x0D,
    Channel14 = 0x0E,
    Channel15 = 0x0F,
    Channel16 = 0x10,
    Channel17 = 0x11,
    Channel18 = 0x12,
    Channel19 = 0x13,
    Channel20 = 0x14,
    Channel21 = 0x15,
    Channel22 = 0x016,
    Channel23 = 0x17,
    Channel24 = 0x18,
    Channel25 = 0x19,
    Channel26 = 0x1A,
}

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.e. 149)
pub enum MACMode {
    NoACKsDigiMode = 0,
    NoACKs802_15_4 = 1,
    ACKs802_15_4 = 2,
    ACKsDigiMode = 3,
}

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.e. 149)
pub struct CompatibilityOptions {
    tx_legacy: bool,
    node_discovery_legacy: bool,
}

impl CompatibilityOptions {
    pub fn encode(&self) -> u8 {
        let mut val: u8 = 0;

        if self.tx_legacy == true {
            val |= 1 << 0;
        }

        if self.node_discovery_legacy == true {
            val |= 1 << 1;
        }

        val
    }
}

pub struct NetworkDiscoveryOptions {
    append_digi_device_dentifier: bool,
    send_own_nd_response: bool,
    last_hop_rssi: bool
}

impl NetworkDiscoveryOptions {
    pub fn encode(&self) -> u8 {
        todo!()
    }
}

type NodeIdentifier = [u8; 20];

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 155)
pub enum DeviceRole {
    EndDevice = 0,
    Coordinator = 1,
}

pub struct EndDeviceAssociation {
    allow_pan_id_reassignment: bool,
    allow_channel_reassignment: bool,
    auto_associate: bool,
    poll_coordinator_on_pin_wake: bool,
}

impl EndDeviceAssociation {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.allow_pan_id_reassignment == true {
            val |= 1 << 0;
        }

        if self.allow_channel_reassignment == true {
            val |= 1 << 1;
        }

        if self.auto_associate == true {
            val |= 1 << 2;
        }

        if self.poll_coordinator_on_pin_wake == true {
            val |= 1 << 3;
        }

        val
    }
}

pub struct CoordinatorAssociation {
    allow_pan_id_reassignment: bool,
    allow_channel_reassignment: bool,
    allow_ssociation: bool,
}

impl CoordinatorAssociation {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.allow_pan_id_reassignment == true {
            val |= 1 << 0;
        }

        if self.allow_channel_reassignment == true {
            val |= 1 << 1;
        }

        if self.allow_ssociation == true {
            val |= 1 << 2;
        }

        val
    }
}

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 157)
pub struct ScanChannels {
    channel_11: bool, // 0x0B - 2.405
    channel_12: bool, // 0x0C - 2.410
    channel_13: bool, // 0x0D - 2.415
    channel_14: bool, // 0x0E - 2.420
    channel_15: bool, // 0x0F - 2.425
    channel_16: bool, // 0x10 - 2.430
    channel_17: bool, // 0x11 - 2.435
    channel_18: bool, // 0x12 - 2.440
    channel_19: bool, // 0x13 - 2.445
    channel_20: bool, // 0x14 - 2.450
    channel_21: bool, // 0x15 - 2.455
    channel_22: bool, // 0x16 - 2.460
    channel_23: bool, // 0x17 - 2.465
    channel_24: bool, // 0x18 - 2.470
    channel_25: bool, // 0x19 - 2.475

    // NOTE: avoid when possible, output power capped on PRO variants.
    channel_26: bool, // 0x1A - 2.480
} // TODO: bitmask

pub enum TransmitOptions {
    // TODO: implement me
}

pub enum DisableFeatures {
    // TODO: implement me
}

pub enum SecureAccess {
    // TODO: implement me
}

pub enum TXPowerLevel {
    Power0 = 0, // -5 dBm / -5 dBm
    Power1 = 1, // -1 dBm / +3 dBm
    Power2 = 2, // +2 dBm / +8 dBm
    Power3 = 3, // +5 dBm / +15 dBm 
    Power4 = 4, // +8 dBm / +19 dBm
}

impl TXPowerLevel {
    pub fn power_level_dbm (&self) -> i8 {
        match self {
            TXPowerLevel::Power0 => -5,
            TXPowerLevel::Power1 => -1,
            TXPowerLevel::Power2 => 2,
            TXPowerLevel::Power3 => 5,
            TXPowerLevel::Power4 => 8,
        }
    }

    pub fn power_level_dbm_pro (&self) -> i8 {
        match self {
            TXPowerLevel::Power0 => -5,
            TXPowerLevel::Power1 => 3,
            TXPowerLevel::Power2 => 8,
            TXPowerLevel::Power3 => 15,
            TXPowerLevel::Power4 => 19,
        }
    }
}

pub enum RandomDelaySlots {
    Exponent0 = 0,
    Exponent1 = 1,
    Exponent2 = 2,
    Exponent3 = 3,
    Exponent4 = 4,
    Exponent5 = 5,
}

pub enum SleepMode {
    NoSleep = 0,
    PinSleep = 1,
    CyclicSleep = 4,
    CyclicSleepWithPinWakeup = 5,
    MicroPythonSleep = 6,
}

pub struct SleepOptions {
    disable_wakeup_poll: bool,
    suppress_sample_on_wakeup: bool,
    always_wake_for_st_time: bool
}


impl SleepOptions {
    pub fn bitfield(&self) -> u16 {
        let mut val: u16 = 0;

        if self.disable_wakeup_poll == true {
            val |= 1 << 0;
        }

        if self.suppress_sample_on_wakeup == true {
            val |= 1 << 1;
        }

        if self.always_wake_for_st_time == true {
            val |= 1 << 8;
        }

        val
    }
}

pub enum MicroPythonCommand {
    BundledCodeReport,
    EraseBundledCode,
    VersionReport,
    InterruptProgram,
}

impl MicroPythonCommand {
    pub fn command(&self) -> &'static str {
        match self {
            MicroPythonCommand::BundledCodeReport => "PYB",
            MicroPythonCommand::EraseBundledCode => "PYE",
            MicroPythonCommand::VersionReport => "PYV",
            MicroPythonCommand::InterruptProgram => "PY^",
        }
    }
}

pub enum FileSystemCommand {
    ListCommands,
    CurrentWorkingDirectory,
    ChangeDirectory(&'static str),
    MakeDirectory(&'static str),
    ListFilesAndDirectories(Option<&'static str>),
    PutFile(&'static str),
    PrintHash(&'static str),
    GetFile(&'static str),
    RemoveFile(&'static str),
    FilesystemInfo,
    FullFilesystemInfo,
    Format,
    FormatConfirm,
}

impl FileSystemCommand {
    pub fn command(&self) -> &'static str {
        match self {
            FileSystemCommand::ListCommands => "",
            FileSystemCommand::CurrentWorkingDirectory => "PWD",
            FileSystemCommand::ChangeDirectory(_) => "CD",
            FileSystemCommand::MakeDirectory(_) => "MD",
            FileSystemCommand::ListFilesAndDirectories(_) => "LS",
            FileSystemCommand::PutFile(_) => "PUT",
            FileSystemCommand::PrintHash(_) =>"HASH",
            FileSystemCommand::GetFile(_) => "GET",
            FileSystemCommand::RemoveFile(_) => "RM",
            FileSystemCommand::FilesystemInfo => "INFO",
            FileSystemCommand::FullFilesystemInfo => "INFO FULL",
            FileSystemCommand::Format => "FORMAT",
            FileSystemCommand::FormatConfirm => "FORMAT confirm",
        }
    }
}

pub enum BluetoothPower {
    Power0 = 0, // -20 dBm
    Power1 = 1, // -10 dBm
    Power2 = 2, // 0 dBm
    Power3 = 3, // 8 dBm
}

pub enum APIEnable {
    TransparentMode = 0,
    APIMode = 1,
    APIModeEscaped = 2,
    APIModeMicroPython = 3,
}

pub enum APIOutputOptions {
    RxIndicator = 0, // recommended for new designs
    ExplicitRxIndicator = 1,
    LegacyIndicator = 2,
}

pub struct ExtendedAPIOptions {
    output_receive_frames_for_fota_update_commands: bool,
    output_extended_modem_status_frames_instead_of_modem_status_frames_when_secure_session_status_change_occurs: bool
}


impl ExtendedAPIOptions {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.output_receive_frames_for_fota_update_commands == true {
            val |= 1 << 1;
        }

        if self.output_extended_modem_status_frames_instead_of_modem_status_frames_when_secure_session_status_change_occurs == true {
            val |= 1 << 3;
        }

        val
    }
}

pub enum UARTBaudRate {
    Baud1200 = 0x0,
    Baud2400 = 0x1,
    Baud4800 = 0x2,
    Baud9600 = 0x3,
    Baud19200 = 0x4,
    Baud38400 = 0x5,
    Baud57600 = 0x6,
    Baud115200 = 0x7,
    Baud230400 = 0x8,
    Baud460800 = 0x9,
    Baud921600 = 0xA,
}

pub enum Parity {
    None = 0,
    Even = 1,
    Odd = 2
}

pub enum DIO6Configuration {
    Disabled = 0,
    RTSFlowControl = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO7Configuration {
    Disabled = 0,
    CTSFlowControl = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
    RS485LowTx = 6,
    RS485HighTx = 7,
}

pub enum DIO13Configuration {
    Disabled = 0,
    UartDout = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO14Configuration {
    Disabled = 0,
    UartDin = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO15Configuration {
    Disabled = 0,
    SPIMISO = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO16Configuration {
    Disabled = 0,
    SPIMOSI = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO17Configuration {
    Disabled = 0,
    SPISSEL = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO18Configuration {
    Disabled = 0,
    SPICLK = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO19Configuration {
    Disabled = 0,
    SPIATTN = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO0Configuration {
    Disabled = 0,
    CommissioningPushbutton = 1,
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum CommissioningButton {
    Wake30Seconds = 1,
    RestoreDefaults = 4,
}

pub enum DIO1Configuration {
    Disabled = 0,
    SPIATTN = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO2Configuration {
    Disabled = 0,
    SPICLK = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO3Configuration {
    Disabled = 0,
    SPISSEL = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO4Configuration {
    Disabled = 0,
    SPIMOSI = 1, // through-hole only
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO5Configuration {
    Disabled = 0,
    AssociateLED = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO8Configuration {
    Disabled = 0,
    DTRSleepRequest = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO9Configuration {
    Disabled = 0,
    ONSLEEPIndicator = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO10Configuration {
    Disabled = 0,
    RSSIIndicator = 1,
    PWM0Output = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO11Configuration {
    Disabled = 0,
    PWM1Output = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

pub enum DIO12Configuration {
    Disabled = 0,
    SPIMISO = 1, // through-hole only
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 193)
pub struct PullUpDownResistorEnable {
    dio4: bool,
    dio3: bool,
    dio2: bool,
    dio1: bool,
    dio0: bool,
    dio6: bool,
    dio8: bool,
    dio14: bool,
    dio5: bool,
    dio9: bool,
    dio12: bool,
    dio10: bool,
    dio11: bool,
    dio7: bool,
    dio13: bool,
    dio15: bool, // not on through-hole
    dio16: bool, // not on through-hole
    dio17: bool, // not on through-hole
    dio18: bool, // not on through-hole
    dio19: bool, // not on through-hole
}

impl PullUpDownResistorEnable {
    // TODO: is bit field math correct
    pub fn bitfield(&self) -> u32 {
        let mut val: u32 = 0;

        if self.dio4 == true {
            val |= 1 << 0;
        }

        if self.dio3 == true {
            val |= 1 << 1;
        }

        if self.dio2 == true {
            val |= 1 << 2;
        }

        if self.dio1 == true {
            val |= 1 << 3;
        }

        if self.dio0 == true {
            val |= 1 << 4;
        }

        if self.dio6 == true {
            val |= 1 << 5;
        }

        if self.dio8 == true {
            val |= 1 << 6;
        }

        if self.dio14 == true {
            val |= 1 << 7;
        }

        if self.dio5 == true {
            val |= 1 << 8;
        }

        if self.dio9 == true {
            val |= 1 << 9;
        }

        if self.dio12 == true {
            val |= 1 << 10;
        }

        if self.dio10 == true {
            val |= 1 << 11;
        }

        if self.dio11 == true {
            val |= 1 << 12;
        }

        if self.dio7 == true {
            val |= 1 << 13;
        }

        if self.dio13 == true {
            val |= 1 << 14;
        }

        if self.dio15 == true {
            val |= 1 << 15;
        }

        if self.dio16 == true {
            val |= 1 << 16;
        }

        if self.dio17 == true {
            val |= 1 << 17;
        }

        if self.dio18 == true {
            val |= 1 << 18;
        }

        if self.dio19 == true {
            val |= 1 << 19;
        }

        val
    }
}

pub struct PullUpDownDirection {
    dio4_pull_up: bool,
    dio3_pull_up: bool,
    dio2_pull_up: bool,
    dio1_pull_up: bool,
    dio0_pull_up: bool,
    dio6_pull_up: bool,
    dio8_pull_up: bool,
    dio14_pull_up: bool,
    dio5_pull_up: bool,
    dio9_pull_up: bool,
    dio12_pull_up: bool,
    dio10_pull_up: bool,
    dio11_pull_up: bool,
    dio7_pull_up: bool,
    dio13_pull_up: bool,
    dio15_pull_up: bool, // not on through-hole
    dio16_pull_up: bool, // not on through-hole
    dio17_pull_up: bool, // not on through-hole
    dio18_pull_up: bool, // not on through-hole
    dio19_pull_up: bool, // not on through-hole
}

impl PullUpDownDirection {
    // TODO: is bit field math correct
    pub fn bitfield(&self) -> u32 {
        let mut val: u32 = 0;

        if self.dio4_pull_up {
            val |= 1 << 0;
        }

        if self.dio3_pull_up {
            val |= 1 << 1;
        }

        if self.dio2_pull_up {
            val |= 1 << 2;
        }

        if self.dio1_pull_up {
            val |= 1 << 3;
        }

        if self.dio0_pull_up {
            val |= 1 << 4;
        }

        if self.dio6_pull_up {
            val |= 1 << 5;
        }

        if self.dio8_pull_up {
            val |= 1 << 6;
        }

        if self.dio14_pull_up {
            val |= 1 << 7;
        }

        if self.dio5_pull_up {
            val |= 1 << 8;
        }

        if self.dio9_pull_up {
            val |= 1 << 9;
        }

        if self.dio12_pull_up {
            val |= 1 << 10;
        }

        if self.dio10_pull_up {
            val |= 1 << 11;
        }

        if self.dio11_pull_up {
            val |= 1 << 12;
        }

        if self.dio7_pull_up {
            val |= 1 << 13;
        }

        if self.dio13_pull_up {
            val |= 1 << 14;
        }

        if self.dio15_pull_up {
            val |= 1 << 15;
        }

        if self.dio16_pull_up {
            val |= 1 << 16;
        }

        if self.dio17_pull_up {
            val |= 1 << 17;
        }

        if self.dio18_pull_up {
            val |= 1 << 18;
        }

        if self.dio19_pull_up {
            val |= 1 << 19;
        }

        val
    }
}

pub struct DIOChangeDetect {
    dio0: bool, // Micro pin 31/SMT pin 33/TH pin 20
    dio1: bool, // Micro pin 30/SMT pin 32/TH pin 19
    dio2: bool, // Micro pin 29/SMT pin 31/TH pin 18
    dio3: bool, // Micro pin 28/SMT pin 30/TH pin 17
    dio4: bool, // Micro pin 23/SMT pin 24/TH pin 11
    dio5: bool, // Micro pin 26/SMT pin 28/TH pin 15
    dio6: bool, // Micro pin 27/SMT pin 29/TH pin 16
    dio7: bool, // Micro pin 24/SMT pin 25/TH pin 12
    dio8: bool, // Micro pin 9/SMT pin 10/TH pin 9
    dio9: bool, // Micro pin 25/SMT pin 26/TH pin 13
    dio10: bool, // Micro pin 7/SMT pin 7/TH pin 6
    dio11: bool, // Micro pin 8/SMT pin 8/TH pin 7
    dio12: bool, // Micro pin 5/SMT pin 5/TH pin 4
    dio13: bool, // Micro pin 3/SMT pin 3/TH pin 2
    dio14: bool, // Micro pin 4/SMT pin 4/TH pin 3
}


impl DIOChangeDetect {
    pub fn bitfield(&self) -> u16 {
        let mut val: u16 = 0;

        if self.dio0 == true {
            val |= 1 << 0;
        }

        if self.dio1 == true {
            val |= 1 << 1;
        }

        if self.dio2 == true {
            val |= 1 << 2;
        }

        if self.dio3 == true {
            val |= 1 << 3;
        }

        if self.dio4 == true {
            val |= 1 << 4;
        }

        if self.dio5 == true {
            val |= 1 << 5;
        }

        if self.dio6 == true {
            val |= 1 << 6;
        }

        if self.dio7 == true {
            val |= 1 << 7;
        }

        if self.dio8 == true {
            val |= 1 << 8;
        }

        if self.dio9 == true {
            val |= 1 << 9;
        }

        if self.dio10 == true {
            val |= 1 << 10;
        }

        if self.dio11 == true {
            val |= 1 << 11;
        }

        if self.dio12 == true {
            val |= 1 << 12;
        }

        if self.dio13 == true {
            val |= 1 << 13;
        }

        if self.dio14 == true {
            val |= 1 << 14;
        }

        val
    }
}

pub enum AnalogVoltageReference {
    Reference1v25 = 0,
    Reference2v5 = 1,
    ReferenceVDD = 2,
}

pub enum Commands {
    OperatingChannel(Channel),
    ExtendedPANID(u16),
    MACMode(MACMode),
    CompatibilityOptions(CompatibilityOptions),

    // Discovery commands
    NodeIdentifier(NodeIdentifier), // string
    DeviceTypeIdentifier(u64),
    NodeDiscoverTimeout(u8), // x 100 ms
    NetworkDiscoveryOptions(NetworkDiscoveryOptions),
    NetworkDiscover(Option<NodeIdentifier>),
    DiscoverNode(NodeIdentifier),
    ActiveScan,

    // Coordinator/End Device configuration
    DeviceRole(DeviceRole),
    EndDeviceAssociation(EndDeviceAssociation),
    CoordinatorAssociation(CoordinatorAssociation),
    ScanChannels(ScanChannels),
    ScanDuration(u8),
    ForceDisassociation,
    AssociationIndication(u8), // TODO: enum read only

    // 802.15.4 Addressing
    SerialNumberHigh(u32), // TODO: read only
    SerialNumberLow(u32),  // TODO: read only
    SourceAddress16Bit(u16),
    DestinationAddressHigh(u32),
    DestinationAddressLow(u32),
    XBeeRetries(u8),
    TransmitOptions(u8), // TODO: struct bitfield
    MaximumPacketPayloadBytes(u8), // TODO: read only

    // Security
    EncryptionEnable(bool),
    AESEncryptionKey([u8; 16]),
    DisableFeatures(u8), // TODO: struct bit mask
    OTAUpgradeServer(u64),

    // Secure Session
    SecureAccess(u16), // TODO: struct for bitfield
    SecureSessionSalt(u32),
    SecureSessionVerifierV(u32),
    SecureSessionVerifierW(u32),
    SecureSessionVerifierX(u32),
    SecureSessionVerifierY(u32),

    // RF interfacing
    TXPowerLevel(TXPowerLevel),
    OutputPower(u8), // dBm TODO: read only
    CCAThreshold(u8),
    RandomDelaySlots(RandomDelaySlots),

    // MAC diagnostics
    LastPacketRSSI(u8), // TODO: read only
    ACKFailures(u16),
    CCAFailures(u16),
    EnergyDetect(u8),

    // Sleep settings
    SleepMode(SleepMode),
    CyclicSleepPeriod(u32), // TODO: type with translation functions
    CyclicSleepWakeTime(u32), // TODO: type with translation functions
    DisassociatedCyclicSleepPeriod(u32),
    NumberOfSleepPeriods(u16),
    SleepOptions(SleepOptions), // TODO: bit field
    ForcePoll,

    // MicroPython
    PythonStartup(bool),
    MicroPythonCommand(MicroPythonCommand),

    // File System
    FileSystem(FileSystemCommand, ),
    FileSystemPublicKey([u8; 65]),

    // Bluetooth Low Energy (BLE)
    BluetoothEnable(bool),
    BluetoothMACAddress, // TODO: read-only
    BluetoothIdentifier([u8; 22]),
    BluetoothPower(BluetoothPower),
    SRPSalt(u32),
    SRPSaltVerifierV(u32),
    SRPSaltVerifierW(u32),
    SRPSaltVerifierX(u32),
    SRPSaltVerifierY(u32),

    // API configuration
    APIEnable(APIEnable),
    APIOutputOptions(APIOutputOptions),
    ExtendedAPIOptions(ExtendedAPIOptions),

    // UART interface
    /// When setting a non-standard baud rate,
    ///  query BD to find the actual buad rate.
    UARTBaudRate(UARTBaudRate),
    Parity(Parity),
    StopBits(bool),
    FlowControlThreshold(u16),
    PacketizationTimeout(u8),
    
    // AT Command options
    CommandCharacter(u8),
    CommandModeTimeout(u16),
    GuardTimes(u16),
    ExitCommandMode,

    // UART pin configuration
    DIO6Configuration(DIO6Configuration),
    DIO7Configuration(DIO7Configuration),
    DIO13Configuration(DIO13Configuration),
    DIO14Configuration(DIO14Configuration),

    // SMT/MMT SPI interface
    DIO15Configuration(DIO15Configuration),
    DIO16Configuration(DIO16Configuration),
    DIO17Configuration(DIO17Configuration),
    DIO18Configuration(DIO18Configuration),
    DIO19Configuration(DIO19Configuration),

    // I/O settings 
    DIO0Configuration(DIO0Configuration),
    CommissioningButton(CommissioningButton),
    DIO1Configuration(DIO1Configuration),
    DIO2Configuration(DIO2Configuration),
    DIO3Configuration(DIO3Configuration),
    DIO4Configuration(DIO4Configuration),
    DIO5Configuration(DIO5Configuration),
    DIO8Configuration(DIO8Configuration),
    DIO9Configuration(DIO9Configuration),
    DIO10Configuration(DIO10Configuration),
    DIO11Configuration(DIO11Configuration),
    DIO12Configuration(DIO12Configuration),
    PullUpDownResistorEnable(PullUpDownResistorEnable),
    PullUpDownDirection(PullUpDownDirection),
    PWM0DutyCycle(u16),
    PWM1DutyCycle(u16),
    RSSIPWMTimer(u8),
    AssociateLEDBlinkTime(u16),

    // I/O sampling
    IOSample,
    SampleRate(u16),
    DIOChangeDetect(DIOChangeDetect),
    AnalogVoltageReference(AnalogVoltageReference),
    SamplesBeforeTX(u8),
    SleepSampleRate(u16),
    DigitalOutputLevel(u8),

    // I/O line passing
    IOInputAddress(u64),
    IOOutputEnable(bool),
    
    D0TimeoutTimer(u8),
    
    D1OutputTimeoutTimer(u8),
    D2OutputTimeoutTimer(u8),
    D3OutputTimeoutTimer(u8),
    D4OutputTimeoutTimer(u8),
    D5OutputTimeoutTimer(u8),
    D6OutputTimeoutTimer(u8),
    D7OutputTimeoutTimer(u8),
    
    D8OutputTimer(u8),
    D9OutputTimer(u8),

    P0OutputTimer(u8),
    P1OutputTimer(u8),
    P2OutputTimer(u8),

    PWMOutputTimeout(u8),

    // Location
    LocationXLatitude([u8;15]),
    LocationYLongitude([u8;15]),
    LocationZElevation([u8;15]),

    // Diagnostic commands
    FirmwareVersion, // read-only
    VersionLong, // read-only
    BootloaderVersion, // read-only
    HardwareVersion, // read-only
    PowerVariant, // read-only
    HardwareSoftwareCompatibility, // read-only
    SupplyVoltage, // read-only
    ModuleTemperature, // read-only
    ConfigurationCRC, // read-only
    InvokeBootloader, // read-only
    ManufacturingDate, // read-only

    // Memory access 
    SoftwareReset,
    ApplyChanges,
    Write,
    RestoreDefaults,

    // Custom Default 
    SetCustomDefault,
    ClearCustomDefaults,
    RestoreFactoryDefaults,
}

/// Converts from a u8 to it's ASCII representaion.
fn u8_ascii(num: u8) -> [u8; 1] {
    return [30+num];
}

impl Commands {
    /// Returns the AT command identifier for the configured AT command.
    pub fn identifier(&self) -> Identifier {
        match self {
            Commands::OperatingChannel(_) => Identifier::OperatingChannel,
            Commands::ExtendedPANID(_) => Identifier::ExtendedPANID,
            Commands::MACMode(_) => Identifier::MACMode,
            Commands::CompatibilityOptions(_) => Identifier::CompatibilityOptions,
            Commands::NodeIdentifier(_) => Identifier::NodeIdentifier,
            Commands::DeviceTypeIdentifier(_) => Identifier::DeviceTypeIdentifier,
            Commands::NodeDiscoverTimeout(_) => Identifier::NodeDiscoverTimeout,
            Commands::NetworkDiscoveryOptions(_) => Identifier::NetworkDiscoveryOptions,
            Commands::NetworkDiscover(_) => Identifier::NetworkDiscover,
            Commands::DiscoverNode(_) => Identifier::DiscoverNode,
            Commands::ActiveScan => Identifier::ActiveScan,
            Commands::DeviceRole(_) => Identifier::DeviceRole,
            Commands::EndDeviceAssociation(_) => Identifier::EndDeviceAssociation,
            Commands::CoordinatorAssociation(_) => Identifier::CoordinatorAssociation,
            Commands::ScanChannels(_) => Identifier::ScanChannels,
            Commands::ScanDuration(_) => Identifier::ScanDuration,
            Commands::ForceDisassociation => Identifier::ForceDisassociation,
            Commands::AssociationIndication(_) => Identifier::AssociationIndication,
            Commands::SerialNumberHigh(_) => Identifier::SerialNumberHigh,
            Commands::SerialNumberLow(_) => Identifier::SerialNumberLow,
            Commands::SourceAddress16Bit(_) => Identifier::SourceAddress16Bit,
            Commands::DestinationAddressHigh(_) => Identifier::DestinationAddressHigh,
            Commands::DestinationAddressLow(_) => Identifier::DestinationAddressLow,
            Commands::XBeeRetries(_) => Identifier::XBeeRetries,
            Commands::TransmitOptions(_) => Identifier::TransmitOptions,
            Commands::MaximumPacketPayloadBytes(_) => Identifier::MaximumPacketPayloadBytes,
            Commands::EncryptionEnable(_) => Identifier::EncryptionEnable,
            Commands::AESEncryptionKey(_) => Identifier::AESEncryptionKey,
            Commands::DisableFeatures(_) => Identifier::DisableFeatures,
            Commands::OTAUpgradeServer(_) => Identifier::OTAUpgradeServer,
            Commands::SecureAccess(_) => Identifier::SecureAccess,
            Commands::SecureSessionSalt(_) => Identifier::SecureSessionSalt,
            Commands::SecureSessionVerifierV(_) => Identifier::SecureSessionVerifierV,
            Commands::SecureSessionVerifierW(_) => Identifier::SecureSessionVerifierW,
            Commands::SecureSessionVerifierX(_) => Identifier::SecureSessionVerifierX,
            Commands::SecureSessionVerifierY(_) => Identifier::SecureSessionVerifierY,
            Commands::TXPowerLevel(_) => Identifier::TXPowerLevel,
            Commands::OutputPower(_) => Identifier::OutputPower,
            Commands::CCAThreshold(_) => Identifier::CCAThreshold,
            Commands::RandomDelaySlots(_) => Identifier::RandomDelaySlots,
            Commands::LastPacketRSSI(_) => Identifier::LastPacketRSSI,
            Commands::ACKFailures(_) => Identifier::ACKFailures,
            Commands::CCAFailures(_) => Identifier::CCAFailures,
            Commands::EnergyDetect(_) => Identifier::EnergyDetect,
            Commands::SleepMode(_) => Identifier::SleepMode,
            Commands::CyclicSleepPeriod(_) => Identifier::CyclicSleepPeriod,
            Commands::CyclicSleepWakeTime(_) => Identifier::CyclicSleepWakeTime,
            Commands::DisassociatedCyclicSleepPeriod(_) => Identifier::DisassociatedCyclicSleepPeriod,
            Commands::NumberOfSleepPeriods(_) => Identifier::NumberOfSleepPeriods,
            Commands::SleepOptions(_) => Identifier::SleepOptions,
            Commands::ForcePoll => Identifier::ForcePoll,
            Commands::PythonStartup(_) => Identifier::PythonStartup,
            Commands::MicroPythonCommand(_) => Identifier::MicroPythonCommand,
            Commands::FileSystem(_) => Identifier::FileSystem,
            Commands::FileSystemPublicKey(_) => Identifier::FileSystemPublicKey,
            Commands::BluetoothEnable(_) => Identifier::BluetoothEnable,
            Commands::BluetoothMACAddress => Identifier::BluetoothMACAddress,
            Commands::BluetoothIdentifier(_) => Identifier::BluetoothIdentifier,
            Commands::BluetoothPower(_) => Identifier::BluetoothPower,
            Commands::SRPSalt(_) => Identifier::SRPSalt,
            Commands::SRPSaltVerifierV(_) => Identifier::SRPSaltVerifierV,
            Commands::SRPSaltVerifierW(_) => Identifier::SRPSaltVerifierW,
            Commands::SRPSaltVerifierX(_) => Identifier::SRPSaltVerifierX,
            Commands::SRPSaltVerifierY(_) => Identifier::SRPSaltVerifierY,
            Commands::APIEnable(_) => Identifier::APIEnable,
            Commands::APIOutputOptions(_) => Identifier::APIOutputOptions,
            Commands::ExtendedAPIOptions(_) => Identifier::ExtendedAPIOptions,
            Commands::UARTBaudRate(_) => Identifier::UARTBaudRate,
            Commands::Parity(_) => Identifier::Parity,
            Commands::StopBits(_) => Identifier::StopBits,
            Commands::FlowControlThreshold(_) => Identifier::FlowControlThreshold,
            Commands::PacketizationTimeout(_) => Identifier::PacketizationTimeout,
            Commands::CommandCharacter(_) => Identifier::CommandCharacter,
            Commands::CommandModeTimeout(_) => Identifier::CommandModeTimeout,
            Commands::GuardTimes(_) => Identifier::GuardTimes,
            Commands::ExitCommandMode => Identifier::ExitCommandMode,
            Commands::DIO6Configuration(_) => Identifier::DIO6Configuration,
            Commands::DIO7Configuration(_) => Identifier::DIO7Configuration,
            Commands::DIO13Configuration(_) => Identifier::DIO13Configuration,
            Commands::DIO14Configuration(_) => Identifier::DIO14Configuration,
            Commands::DIO15Configuration(_) => Identifier::DIO15Configuration,
            Commands::DIO16Configuration(_) => Identifier::DIO16Configuration,
            Commands::DIO17Configuration(_) => Identifier::DIO17Configuration,
            Commands::DIO18Configuration(_) => Identifier::DIO18Configuration,
            Commands::DIO19Configuration(_) => Identifier::DIO19Configuration,
            Commands::DIO0Configuration(_) => Identifier::DIO0Configuration,
            Commands::CommissioningButton(_) => Identifier::CommissioningButton,
            Commands::DIO1Configuration(_) => Identifier::DIO1Configuration,
            Commands::DIO2Configuration(_) => Identifier::DIO2Configuration,
            Commands::DIO3Configuration(_) => Identifier::DIO3Configuration,
            Commands::DIO4Configuration(_) => Identifier::DIO4Configuration,
            Commands::DIO5Configuration(_) => Identifier::DIO5Configuration,
            Commands::DIO8Configuration(_) => Identifier::DIO8Configuration,
            Commands::DIO9Configuration(_) => Identifier::DIO9Configuration,
            Commands::DIO10Configuration(_) => Identifier::DIO10Configuration,
            Commands::DIO11Configuration(_) => Identifier::DIO11Configuration,
            Commands::DIO12Configuration(_) => Identifier::DIO12Configuration,
            Commands::PullUpDownResistorEnable(_) => Identifier::PullUpDownResistorEnable,
            Commands::PullUpDownDirection(_) => Identifier::PullUpDownDirection,
            Commands::PWM0DutyCycle(_) => Identifier::PWM0DutyCycle,
            Commands::PWM1DutyCycle(_) => Identifier::PWM1DutyCycle,
            Commands::RSSIPWMTimer(_) => Identifier::RSSIPWMTimer,
            Commands::AssociateLEDBlinkTime(_) => Identifier::AssociateLEDBlinkTime,
            Commands::IOSample => Identifier::IOSample,
            Commands::SampleRate(_) => Identifier::SampleRate,
            Commands::DIOChangeDetect(_) => Identifier::DIOChangeDetect,
            Commands::AnalogVoltageReference(_) => Identifier::AnalogVoltageReference,
            Commands::SamplesBeforeTX(_) => Identifier::SamplesBeforeTX,
            Commands::SleepSampleRate(_) => Identifier::SleepSampleRate,
            Commands::DigitalOutputLevel(_) => Identifier::DigitalOutputLevel,
            Commands::IOInputAddress(_) => Identifier::IOInputAddress,
            Commands::IOOutputEnable(_) => Identifier::IOOutputEnable,
            Commands::D0TimeoutTimer(_) => Identifier::D0TimeoutTimer,
            Commands::D1OutputTimeoutTimer(_) => Identifier::D1OutputTimeoutTimer,
            Commands::D2OutputTimeoutTimer(_) => Identifier::D2OutputTimeoutTimer,
            Commands::D3OutputTimeoutTimer(_) => Identifier::D3OutputTimeoutTimer,
            Commands::D4OutputTimeoutTimer(_) => Identifier::D4OutputTimeoutTimer,
            Commands::D5OutputTimeoutTimer(_) => Identifier::D5OutputTimeoutTimer,
            Commands::D6OutputTimeoutTimer(_) => Identifier::D6OutputTimeoutTimer,
            Commands::D7OutputTimeoutTimer(_) => Identifier::D7OutputTimeoutTimer,
            Commands::D8OutputTimer(_) => Identifier::D8OutputTimer,
            Commands::D9OutputTimer(_) => Identifier::D9OutputTimer,
            Commands::P0OutputTimer(_) => Identifier::P0OutputTimer,
            Commands::P1OutputTimer(_) => Identifier::P1OutputTimer,
            Commands::P2OutputTimer(_) => Identifier::P2OutputTimer,
            Commands::PWMOutputTimeout(_) => Identifier::PWMOutputTimeout,
            Commands::LocationXLatitude(_) => Identifier::LocationXLatitude,
            Commands::LocationYLongitude(_) => Identifier::LocationYLongitude,
            Commands::LocationZElevation(_) => Identifier::LocationZElevation,
            Commands::FirmwareVersion => Identifier::FirmwareVersion,
            Commands::VersionLong => Identifier::VersionLong,
            Commands::BootloaderVersion => Identifier::BootloaderVersion,
            Commands::HardwareVersion => Identifier::HardwareVersion,
            Commands::PowerVariant => Identifier::PowerVariant,
            Commands::HardwareSoftwareCompatibility => Identifier::HardwareSoftwareCompatibility,
            Commands::SupplyVoltage => Identifier::SupplyVoltage,
            Commands::ModuleTemperature => Identifier::ModuleTemperature,
            Commands::ConfigurationCRC => Identifier::ConfigurationCRC,
            Commands::InvokeBootloader => Identifier::InvokeBootloader,
            Commands::ManufacturingDate => Identifier::ManufacturingDate,
            Commands::SoftwareReset => Identifier::SoftwareReset,
            Commands::ApplyChanges => Identifier::ApplyChanges,
            Commands::Write => Identifier::Write,
            Commands::RestoreDefaults => Identifier::RestoreDefaults,
            Commands::SetCustomDefault => Identifier::SetCustomDefault,
            Commands::ClearCustomDefaults => Identifier::ClearCustomDefaults,
            Commands::RestoreFactoryDefaults => Identifier::RestoreFactoryDefaults,
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            // Discovery commands
            Self::NodeDiscoverTimeout(timeout) => timeout <= &0xFC,

            // Coordinator/End Device configuration
            Self::ScanDuration(duration) => duration <= &0x0F,

            // 802.15.4 Addressing
            Self::XBeeRetries(retries) => retries <= &6,

            // Sleep settings
            Self::CyclicSleepPeriod(period) => period <= &0x15F900,
            Self::CyclicSleepWakeTime(period) => period <= &0x36EE80,
            Self::DisassociatedCyclicSleepPeriod(period) => period > &0 && period <= &0x15F900,

            // UART interface
            Self::FlowControlThreshold(threshold) => threshold >= &0x20 && threshold <= &0x1B0,
    
            // AT Command options
            Self::CommandModeTimeout(timeout) => timeout >= &2 && timeout <= &0x1770,
            Self::GuardTimes(times) => times >= &0x2 && times <= &0x6D3,

            // I/O settings 
            Self::PWM0DutyCycle(cycle) => cycle <= &0x3FF,
            Self::PWM1DutyCycle(cycle) => cycle <= &0x3FF,
            Self::AssociateLEDBlinkTime(time) => time == &0 || (time >= &0x14 && time <= &0xFF),

            // everything that doesn't need validation
            // due to enum, int max, array bounds, etc.
            _ => true, 
        }
    }

    /// Converts the enum to an AT command struct.
    pub fn command<'a>(&'a self) -> Command<'a> {
        match self {
            Commands::OperatingChannel(channel) => Command {
                command: self.identifier(),
                parameter: Some(&[*channel as u8]),
                carriage_returns: 1,
            },
            Commands::ExtendedPANID(panid) => Command {
                command: self.identifier(),
                parameter: Some(&panid.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::MACMode(mac_mode) => Command {
                command: self.identifier(),
                parameter: match mac_mode {
                    MACMode::NoACKsDigiMode => Some(b"0"),
                    MACMode::NoACKs802_15_4 => Some(b"1"),
                    MACMode::ACKs802_15_4 => Some(b"2"),
                    MACMode::ACKsDigiMode => Some(b"3"),
                },
                carriage_returns: 1,
            },
            Commands::CompatibilityOptions(options) => Command {
                command: self.identifier(),
                parameter: Some(&[options.encode()]),
                carriage_returns: 1,
            },
            Commands::NodeIdentifier(identifier) => Command {
                command: self.identifier(),
                parameter: Some(identifier),
                carriage_returns: 1,
            },
            Commands::DeviceTypeIdentifier(identifier) => Command {
                command: self.identifier(),
                parameter: Some(&identifier.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::NodeDiscoverTimeout(timeout) => Command {
                command: self.identifier(),
                parameter: Some(&[*timeout]),
                carriage_returns: 1,
            },
            Commands::NetworkDiscoveryOptions(options) => Command {
                command: self.identifier(),
                parameter: Some(&[options.encode()]),
                carriage_returns: 1,
            },
            Commands::NetworkDiscover(identifier) => Command {
                command: self.identifier(),
                parameter: match identifier {
                    None => None,
                    Some(id) => Some(id)
                },
                carriage_returns: 1,
            },
            Commands::DiscoverNode(identifier) => Command {
                command: self.identifier(),
                parameter: Some(identifier),
                carriage_returns: 1,
            },
            Commands::ActiveScan => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::DeviceRole(role) => Command {
                command: self.identifier(),
                parameter: match role {
                    DeviceRole::EndDevice => Some(b"0"),
                    DeviceRole::Coordinator => Some(b"1"),
                },
                carriage_returns: 1,
            },
            Commands::EndDeviceAssociation(options) => Command {
                command: self.identifier(),
                parameter: Some(&[options.bitfield()]),
                carriage_returns: 1,
            },
            Commands::CoordinatorAssociation(options) => Command {
                command: self.identifier(),
                parameter: Some(&[options.bitfield()]),
                carriage_returns: 1,
            },
            Commands::ScanChannels(_) => todo!(), // TODO: bitmask
            Commands::ScanDuration(duration) => Command {
                command: self.identifier(),
                parameter: Some(&[*duration]),
                carriage_returns: 1,
            },
            Commands::ForceDisassociation => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::AssociationIndication(indicator) => Command {
                command: self.identifier(),
                parameter: Some(&[*indicator]),
                carriage_returns: 1,
            },
            Commands::SerialNumberHigh(serial_nbumber) => Command {
                command: self.identifier(),
                parameter: Some(&serial_nbumber.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SerialNumberLow(serial_nbumber) => Command {
                command: self.identifier(),
                parameter: Some(&serial_nbumber.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SourceAddress16Bit(address) => Command {
                command: self.identifier(),
                parameter: Some(&address.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::DestinationAddressHigh(address) => Command {
                command: self.identifier(),
                parameter: Some(&address.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::DestinationAddressLow(address) => Command {
                command: self.identifier(),
                parameter: Some(&address.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::XBeeRetries(retries) => Command {
                command: self.identifier(),
                parameter: Some(&[*retries]),
                carriage_returns: 1,
            },
            Commands::TransmitOptions(options) => Command {
                command: self.identifier(),
                parameter: Some(&[*options]),
                carriage_returns: 1,
            },
            Commands::MaximumPacketPayloadBytes(payload_bytes) => Command {
                command: self.identifier(),
                parameter: Some(&[*payload_bytes]),
                carriage_returns: 1,
            },
            Commands::EncryptionEnable(enable) => Command {
                command: self.identifier(),
                parameter: match enable {
                    true => Some(b"1"),
                    false => Some(b"0")
                },
                carriage_returns: 1,
            },
            Commands::AESEncryptionKey(key) => Command {
                command: self.identifier(),
                parameter: Some(key),
                carriage_returns: 1,
            },
            Commands::DisableFeatures(features) => Command {
                command: self.identifier(),
                parameter: Some(&[*features]),
                carriage_returns: 1,
            },
            Commands::OTAUpgradeServer(address) => Command {
                command: self.identifier(),
                parameter: Some(&address.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SecureAccess(secure_access) => Command {
                command: self.identifier(),
                parameter: Some(&secure_access.to_be_bytes()), // TODO: struct for bitfield
                carriage_returns: 1,
            },
            Commands::SecureSessionSalt(salt) => Command {
                command: self.identifier(),
                parameter: Some(&salt.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SecureSessionVerifierV(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SecureSessionVerifierW(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SecureSessionVerifierX(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SecureSessionVerifierY(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::TXPowerLevel(power_level) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*power_level as u8)),
                carriage_returns: 1,
            },
            Commands::OutputPower(power) => Command {
                command: self.identifier(),
                parameter: Some(&[*power]),
                carriage_returns: 1,
            },
            Commands::CCAThreshold(threshold) => Command {
                command: self.identifier(),
                parameter: Some(&[*threshold]),
                carriage_returns: 1,
            },
            Commands::RandomDelaySlots(delay_slots) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*delay_slots as u8)),
                carriage_returns: 1,
            },
            Commands::LastPacketRSSI(rssi) => Command {
                command: self.identifier(),
                parameter: Some(&[*rssi]),
                carriage_returns: 1,
            },
            Commands::ACKFailures(failures) => Command {
                command: self.identifier(),
                parameter: Some(&failures.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::CCAFailures(failures) => Command {
                command: self.identifier(),
                parameter: Some(&failures.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::EnergyDetect(option) => Command {
                command: self.identifier(),
                parameter: Some(&[*option]),
                carriage_returns: 1,
            },
            Commands::SleepMode(sleep_mode) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*sleep_mode as u8)),
                carriage_returns: 1,
            },
            Commands::CyclicSleepPeriod(period) => Command {
                command: self.identifier(),
                parameter: Some(&period.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::CyclicSleepWakeTime(wake_time) => Command {
                command: self.identifier(),
                parameter: Some(&wake_time.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::DisassociatedCyclicSleepPeriod(period) => Command {
                command: self.identifier(),
                parameter: Some(&period.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::NumberOfSleepPeriods(periods) => Command {
                command: self.identifier(),
                parameter: Some(&periods.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SleepOptions(options) => Command {
                command: self.identifier(),
                parameter: Some(&options.bitfield().to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::ForcePoll => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::PythonStartup(startup) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*startup as u8)),
                carriage_returns: 1,
            },
            Commands::MicroPythonCommand(command) => Command {
                command: self.identifier(),
                parameter: Some(command.command().as_bytes()),
                carriage_returns: 1,
            },
            Commands::FileSystem(command) => Command {
                command: self.identifier(),
                parameter: Some(command.command().as_bytes()),
                carriage_returns: 1,
            },
            Commands::FileSystemPublicKey(public_key) => Command {
                command: self.identifier(),
                parameter: Some(public_key),
                carriage_returns: 1,
            },
            Commands::BluetoothEnable(enable) => Command {
                command: self.identifier(),
                parameter: match enable {
                    true => Some(b"1"),
                    false => Some(b"0")
                },
                carriage_returns: 1,
            },
            Commands::BluetoothMACAddress => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::BluetoothIdentifier(identifier) => Command {
                command: self.identifier(),
                parameter: Some(identifier),
                carriage_returns: 1,
            },
            Commands::BluetoothPower(power) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*power as u8)),
                carriage_returns: 1,
            },
            Commands::SRPSalt(salt) => Command {
                command: self.identifier(),
                parameter: Some(&salt.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SRPSaltVerifierV(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SRPSaltVerifierW(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SRPSaltVerifierX(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SRPSaltVerifierY(part) => Command {
                command: self.identifier(),
                parameter: Some(&part.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::APIEnable(mode) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*mode as u8)),
                carriage_returns: 1,
            },
            Commands::APIOutputOptions(option) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*option as u8)),
                carriage_returns: 1,
            },
            Commands::ExtendedAPIOptions(options) => Command {
                command: self.identifier(),
                parameter: Some(&[options.bitfield()]),
                carriage_returns: 1,
            },
            Commands::UARTBaudRate(baudrate) => Command {
                command: self.identifier(),
                parameter: Some(&[*baudrate as u8]), // TODO: encoded correctly?
                carriage_returns: 1,
            },
            Commands::Parity(parity) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*parity as u8)),
                carriage_returns: 1,
            },
            Commands::StopBits(stop_bits) => Command {
                command: self.identifier(),
                parameter: match stop_bits {
                    true => Some(b"1"),
                    false => Some(b"0"),
                },
                carriage_returns: 1,
            },
            Commands::FlowControlThreshold(threshold) => Command {
                command: self.identifier(),
                parameter: Some(&threshold.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::PacketizationTimeout(timeout) => Command {
                command: self.identifier(),
                parameter: Some(&timeout.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::CommandCharacter(command_character) => Command {
                command: self.identifier(),
                parameter: Some(&command_character.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::CommandModeTimeout(timeout) => Command {
                command: self.identifier(),
                parameter: Some(&timeout.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::GuardTimes(guard_times) => Command {
                command: self.identifier(),
                parameter: Some(&guard_times.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::ExitCommandMode => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::DIO6Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO7Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO13Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO14Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO15Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO16Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO17Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO18Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO19Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO0Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::CommissioningButton(option) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*option as u8)),
                carriage_returns: 1,
            },
            Commands::DIO1Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO2Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO3Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO4Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO5Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO8Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO9Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO10Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO11Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::DIO12Configuration(configuration) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*configuration as u8)),
                carriage_returns: 1,
            },
            Commands::PullUpDownResistorEnable(enable) => Command {
                command: self.identifier(),
                parameter: Some(&enable.bitfield().to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::PullUpDownDirection(direction) => Command {
                command: self.identifier(),
                parameter: Some(&direction.bitfield().to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::PWM0DutyCycle(cycle) => Command {
                command: self.identifier(),
                parameter: Some(&cycle.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::PWM1DutyCycle(cycle) => Command {
                command: self.identifier(),
                parameter: Some(&cycle.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::RSSIPWMTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::AssociateLEDBlinkTime(blink_time) => Command {
                command: self.identifier(),
                parameter: Some(&blink_time.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::IOSample => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::SampleRate(sample_rate) => Command {
                command: self.identifier(),
                parameter: Some(&sample_rate.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::DIOChangeDetect(detect) => Command {
                command: self.identifier(),
                parameter: Some(&detect.bitfield().to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::AnalogVoltageReference(reference) => Command {
                command: self.identifier(),
                parameter: Some(&u8_ascii(*reference as u8)),
                carriage_returns: 1,
            },
            Commands::SamplesBeforeTX(samples) => Command {
                command: self.identifier(),
                parameter: Some(&samples.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::SleepSampleRate(sample_rate) => Command {
                command: self.identifier(),
                parameter: Some(&sample_rate.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::DigitalOutputLevel(output_level) => Command {
                command: self.identifier(),
                parameter: Some(&output_level.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::IOInputAddress(address) => Command {
                command: self.identifier(),
                parameter: Some(&address.to_be_bytes()),
                carriage_returns: 1,
            },
            Commands::IOOutputEnable(enable) => Command {
                command: self.identifier(),
                parameter: match enable {
                    true => Some(b"1"),
                    false => Some(b"0")
                },
                carriage_returns: 1,
            },
            Commands::D0TimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D1OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D2OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D3OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D4OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D5OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D6OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D7OutputTimeoutTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D8OutputTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::D9OutputTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::P0OutputTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::P1OutputTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::P2OutputTimer(timer) => Command {
                command: self.identifier(),
                parameter: Some(&[*timer]),
                carriage_returns: 1,
            },
            Commands::PWMOutputTimeout(timeout) => Command {
                command: self.identifier(),
                parameter: Some(&[*timeout]),
                carriage_returns: 1,
            },
            Commands::LocationXLatitude(latitude) => Command {
                command: self.identifier(),
                parameter: Some(latitude),
                carriage_returns: 1,
            },
            Commands::LocationYLongitude(longitude) => Command {
                command: self.identifier(),
                parameter: Some(longitude),
                carriage_returns: 1,
            },
            Commands::LocationZElevation(elevation) => Command {
                command: self.identifier(),
                parameter: Some(elevation),
                carriage_returns: 1,
            },
            Commands::FirmwareVersion => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::VersionLong => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::BootloaderVersion => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::HardwareVersion => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::PowerVariant => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::HardwareSoftwareCompatibility => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::SupplyVoltage => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::ModuleTemperature => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::ConfigurationCRC => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::InvokeBootloader => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::ManufacturingDate => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::SoftwareReset => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::ApplyChanges => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::Write => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::RestoreDefaults => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::SetCustomDefault => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::ClearCustomDefaults => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
            Commands::RestoreFactoryDefaults => Command {
                command: self.identifier(),
                parameter: None,
                carriage_returns: 1,
            },
        }
    }
}
