// AT Command Mode

/// Structure for encoding and decoding AT commands.
pub struct ATCommand<'a> {
    pub command: &'a str,
    pub parameter: &'a Option<&'a [u8]>,
}

/// Enum that encapsulates all the well-known AT commands
/// and translates them to AT command structs.
pub enum ATCommands<'a> {
    /// Causes the XBee radio to enter AT command mode from other states.
    CommandMode(bool),
    
    /// Should cause the radio to reply with OK
    Attention(),
    
    /// Sets or retrieves the PAN ID for the radio.
    PersonalAreaNetworkID(Option<&'a [u8]>),

    /// Retrieves or set the upper and lower halfs of the XBee's address.
    AddressUpper(Option<&'a [u8]>),
    AddressLower(Option<&'a [u8]>),

    /// Retrieves or sets the address the XBee is communicating with.
    DestinationAddressUpper(Option<&'a [u8]>),
    DestinationAddressLower(Option<&'a [u8]>),

    /// Writes the current configuration to the device memory for next boot.
    PersistConfiguration(),
    /// Returns configuration to factory default.
    ResetConfiguration(),

    /// Returns the XBee's 16 bit address.
    MyAddress(),
}

impl ATCommands<'_> {
    pub fn create(&self) -> ATCommand {
        match *self {
            ATCommands::CommandMode(ref state) => match state {
                true => ATCommand {
                    command: "+++",
                    parameter: &None,
                },
                false => ATCommand {
                    command: "CN",
                    parameter: &None,
                },
            },
            ATCommands::Attention() => ATCommand {
                command: "AT",
                parameter: &None,
            },
            ATCommands::PersonalAreaNetworkID(ref id) => ATCommand {
                command: "ATID",
                parameter: id,
            },
            ATCommands::AddressUpper(ref id) => ATCommand {
                command: "ATSH",
                parameter: id,
            },
            ATCommands::AddressLower(ref id) => ATCommand {
                command: "ATSL",
                parameter: id,
            },
            ATCommands::DestinationAddressUpper(ref id) => ATCommand {
                command: "ATDH",
                parameter: id,
            },
            ATCommands::DestinationAddressLower(ref id) => ATCommand {
                command: "ATDL",
                parameter: id,
            },
            ATCommands::PersistConfiguration() => ATCommand {
                command: "ATWR",
                parameter: &None,
            },
            ATCommands::ResetConfiguration() => ATCommand {
                command: "ATRE",
                parameter: &None,
            },
            ATCommands::MyAddress() => ATCommand {
                command: "ATMY",
                parameter: &None,
            },
            // TODO: ATD0…ATD7, ATP0…ATP1, ATIR, ATPR
        }
    }
}