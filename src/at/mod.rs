pub mod commands;

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

pub struct Command<const N: usize> {
    identifier: Identifier,
    payload: Option<[u8;N]>,
    carriage_returns: u8,
}

pub fn encode_command<const N: usize, C: Into<Command<N>>> (cmd: C) {

}
