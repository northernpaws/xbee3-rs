pub mod commands;

/// Enum of all the valid AT command identifiers for the XBee 3 802.15.4 modules.
/// 
/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 148)
pub enum Identifier {
    // Networking
    OperatingChannel = "CH",
    ExtendedPANID = "ID",
    MACMode = "MM",
    CompatibilityOptions = "C8",

    // Discovery
    NodeIdentifier = "NI",
    DeviceTypeIdentifier = "DD",
    NodeDiscoverTimeout = "NT",
    NetworkDiscoveryOptions = "NO",
    NetworkDiscover = "ND",
    DiscoverNode = "DN",
    ActiveScan = "AS",

    // Coordinator/End Device
    DeviceRole = "CE",
    EndDeviceAssociation = "A1",
    CoordinatorAssociation = "A2",
    ScanChannels = "SC",
    ScanDuration = "SD",
    ForceDisassociation = "DA",
    AssociationIndication = "AI",

    // 802.15.4 Addressing
    SerialNumberHigh = "SH",
    SerialNumberLow = "SL",
    SourceAddress16Bit = "MY",
    DestinationAddressHigh = "DH",
    DestinationAddressLow = "DL",
    XBeeRetries = "RR",
    TransmitOptions = "TO",
    MaximumPacketPayloadBytes = "NP",

    // Security
    EncryptionEnable = "EE",
    AESEncryptionKey = "KY",
    DisableFeatures = "DM",
    OTAUpgradeServer = "US",

    // Secure Session
    SecureAccess = "SA",
    SecureSessionSalt = "*S",
    SecureSessionVerifierV = "*V",
    SecureSessionVerifierW = "*W",
    SecureSessionVerifierX = "*X",
    SecureSessionVerifierY = "*Y",

    // RF interfacing
    TXPowerLevel = "PL",
    OutputPower = "PP", // dBm
    CCAThreshold = "CA",
    RandomDelaySlots = "RN",

    // MAC diagnostics
    LastPacketRSSI = "DB",
    ACKFailures = "EA",
    CCAFailures = "EC",
    EnergyDetect = "ED",

    // Sleep settings
    SleepMode = "SM",
    CyclicSleepPeriod = "SP",
    CyclicSleepWakeTime = "ST",
    DisassociatedCyclicSleepPeriod = "DP",
    NumberOfSleepPeriods = "SN",
    SleepOptions = "SO",
    ForcePoll = "FP",

    // MicroPython
    PythonStartup = "PS",
    MicroPythonCommand = "PY",
    BundledCodeReport = "PYB",
    EraseBundledCode = "PYE",
    VersionReport = "PYV",
    InterruptProgram = "PY^",

    // File System
    FileSystem = "FS",
    FileSystemPublicKey = "FK",

    // Bluetooth Low Energy (BLE)
    BluetoothEnable = "BT",
    BluetoothMACAddress = "BL",
    BluetoothIdentifier = "BI",
    BluetoothPower = "BP",
    SRPSalt = "$S",
    SRPSaltVerifierV = "*V",
    SRPSaltVerifierW = "*W",
    SRPSaltVerifierX = "*X",
    SRPSaltVerifierY = "*Y",

    // API configuration
    APIEnable = "AP",
    APIOutputOptions = "AO",
    ExtendedAPIOptions = "AZ",

    // UART interface
    UARTBaudRate = "BD",
    Parity = "NB",
    StopBits = "SB",
    FlowControlThreshold = "FT",
    PacketizationTimeout = "RO",
    
    // AT Command options
    CommandCharacter = "CC",
    CommandModeTimeout = "CT",
    GuardTimes = "GT",
    ExitCommandMode = "CN",

    // UART pin configuration
    DIO6_RTS_Configuration = "D6",
    DIO7_CTS_Configuration = "D7",
    DIO13_UART_DOUT_Configuration = "P3",
    DIO14_UART_DIN_Configuration = "P4",

    // SMT/MMT SPI interface
    DIO15_SPI_MISO_Configuration = "P5",
    DIO16_SPI_MOSI_Configuration = "P6",
    DIO17_SPI_SSEL_Configuration = "P7",
    DIO18_SPI_CLK_Configuration = "P8",
    DIO19_SPI_ATTN_Configuration = "P9",

    // I/O settings 
    DIO0_ADC0_Commissioning_Configuration = "D0",
    CommissioningButton = "CB",
    DIO1_ADC1_TH_SPI_ATTN_Configuration = "D1",
    DIO2_ADC2_TH_SPI_CLK_Configuration = "D2",
    DIO3_ADC3_TH_SPI_SSEL_Configuration = "D3",
    DIO4_TH_SPI_MOSI_Configuration = "D4",
    DIO5_Associate_Configuratiom = "D5",
    DIO8_DTR_SLP_Request_Configuration = "D8",
    DIO9_ON_SLEEP_Configuration = "D9",
    DIO10_RSSI_PWM0_Configuration = "P0",
    DIO11_PWM1_Configuration = "P1",
    DIO12_TH_SPI_MISO_Configuration = "P2",
    PullUpDownResistorEnable = "PR",
    PullUpDownDirection = "PD",
    PWM0DutyCycle = "M0",
    PWM1DutyCycle = "M1",
    RSSIPWMTimer = "RP",
    AssociateLEDBlinkTime = "LT",

    // I/O sampling
    IOSample = "IS",
    SampleRate = "IR",
    DIOChangeDetect = "IC",
    AnalogVoltageReference = "AV",
    SamplesBeforeTX = "IT",
    DigitalOutputLevel = "IO",

    // I/O line passing
    IOInputAddress = "IA",
    IOOutputEnable = "IU",
    
    D0TimeoutTimer = "T0",
    
    D1OutputTimeoutTimer = "T1",
    D2OutputTimeoutTimer = "T2",
    D3OutputTimeoutTimer = "T3",
    D4OutputTimeoutTimer = "T4",
    D5OutputTimeoutTimer = "T5",
    D6OutputTimeoutTimer = "T6",
    D7OutputTimeoutTimer = "T7",
    
    D8OutputTimer = "T8",
    D9OutputTimer = "T9",

    P0OutputTimer = "Q0",
    P1OutputTimer = "Q1",
    P2OutputTimer = "Q2",

    PWMOutputTimeout = "PT",

    // Location
    LocationXLatitude = "LX",
    LocationYLongitude = "LY",
    LocationZElevation = "LZ",

    // Diagnostic commands
    FirmwareVersion = "VR",
    VersionLong = "VL",
    BootloaderVersion = "VH",
    HardwareVersion = "HV",
    PowerVariant = "R?",
    HardwareSoftwareCompatibility = "%C",
    SupplyVoltage = "%V",
    ModuleTemperature = "TP",
    ConfigurationCRC = "CK",
    InvokeBootloader = "%P",
    ManufacturingDate = "D%",

    // Memory access 
    SoftwareReset = "FR",
    ApplyChanges = "AC",
    Write = "WR",
    RestoreDefaults = "RE",

    // Custom Default 
    SetCustomDefault = "%F",
    ClearCustomDefaults = "!C",
    RestoreFactoryDefaults = "R1",
}

pub trait Command {

}