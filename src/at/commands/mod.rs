pub trait Command {
    fn identifier(&self) -> super::Identifier;
}

mod operating_channel;
pub use operating_channel::*;

mod extended_panid;
pub use extended_panid::*;

mod mac_mode;
pub use mac_mode::*;

mod compatibility_options;
pub use compatibility_options::*;

mod node_identifier;
pub use node_identifier::*;

mod device_type_identifier;
pub use device_type_identifier::*;

mod node_discover_timeout;
pub use node_discover_timeout::*;

mod network_discovery_options;
pub use network_discovery_options::*;

mod network_discover;
pub use network_discover::*;

mod discover_node;
pub use discover_node::*;

mod active_scan;
pub use active_scan::*;

mod device_role;
pub use device_role::*;

mod end_device_association;
pub use end_device_association::*;

mod coordinator_association;
pub use coordinator_association::*;

mod scan_channels;
pub use scan_channels::*;

mod scan_duration;
pub use scan_duration::*;

mod force_disassociation;
pub use force_disassociation::*;

mod association_indication;
pub use association_indication::*;

mod serial_number_high;
pub use serial_number_high::*;

mod serial_number_low;
pub use serial_number_low::*;

mod source_address16_bit;
pub use source_address16_bit::*;

mod destination_address_high;
pub use destination_address_high::*;

mod destination_address_low;
pub use destination_address_low::*;

mod x_bee_retries;
pub use x_bee_retries::*;

mod transmit_options;
pub use transmit_options::*;

mod maximum_packet_payload_bytes;
pub use maximum_packet_payload_bytes::*;

mod encryption_enable;
pub use encryption_enable::*;

mod aes_encryption_key;
pub use aes_encryption_key::*;

mod disable_features;
pub use disable_features::*;

mod ota_upgrade_server;
pub use ota_upgrade_server::*;

mod secure_access;
pub use secure_access::*;

mod secure_session_salt;
pub use secure_session_salt::*;

mod secure_session_verifier_v;
pub use secure_session_verifier_v::*;

mod secure_session_verifier_w;
pub use secure_session_verifier_w::*;

mod secure_session_verifier_x;
pub use secure_session_verifier_x::*;

mod secure_session_verifier_y;
pub use secure_session_verifier_y::*;

mod tx_power_level;
pub use tx_power_level::*;

mod output_power;
pub use output_power::*;

mod cca_threshold;
pub use cca_threshold::*;

mod random_delay_slots;
pub use random_delay_slots::*;

mod last_packet_rssi;
pub use last_packet_rssi::*;

mod ack_failures;
pub use ack_failures::*;

mod cca_failures;
pub use cca_failures::*;

mod energy_detect;
pub use energy_detect::*;

mod sleep_mode;
pub use sleep_mode::*;

mod cyclic_sleep_period;
pub use cyclic_sleep_period::*;

mod cyclic_sleep_wake_time;
pub use cyclic_sleep_wake_time::*;

mod disassociated_cyclic_sleep_period;
pub use disassociated_cyclic_sleep_period::*;

mod number_of_sleep_periods;
pub use number_of_sleep_periods::*;

mod sleep_options;
pub use sleep_options::*;

mod force_poll;
pub use force_poll::*;

mod python_startup;
pub use python_startup::*;

mod micro_python_command;
pub use micro_python_command::*;

mod file_system;
pub use file_system::*;

mod file_system_public_key;
pub use file_system_public_key::*;

mod bluetooth_enable;
pub use bluetooth_enable::*;

mod bluetooth_mac_address;
pub use bluetooth_mac_address::*;

mod bluetooth_identifier;
pub use bluetooth_identifier::*;

mod bluetooth_power;
pub use bluetooth_power::*;

mod srp_salt;
pub use srp_salt::*;

mod srp_salt_verifier_v;
pub use srp_salt_verifier_v::*;

mod srp_salt_verifier_w;
pub use srp_salt_verifier_w::*;

mod srp_salt_verifier_x;
pub use srp_salt_verifier_x::*;

mod srp_salt_verifier_y;
pub use srp_salt_verifier_y::*;

mod api_enable;
pub use api_enable::*;

mod api_output_options;
pub use api_output_options::*;

mod extended_api_options;
pub use extended_api_options::*;

mod uart_baud_rate;
pub use uart_baud_rate::*;

mod parity;
pub use parity::*;

mod stop_bits;
pub use stop_bits::*;

mod flow_control_threshold;
pub use flow_control_threshold::*;

mod packetization_timeout;
pub use packetization_timeout::*;

mod command_character;
pub use command_character::*;

mod command_mode_timeout;
pub use command_mode_timeout::*;

mod guard_times;
pub use guard_times::*;

mod exit_command_mode;
pub use exit_command_mode::*;

mod dio6_configuration;
pub use dio6_configuration::*;

mod dio7_configuration;
pub use dio7_configuration::*;

mod dio13_configuration;
pub use dio13_configuration::*;

mod dio14_configuration;
pub use dio14_configuration::*;

mod dio15_configuration;
pub use dio15_configuration::*;

mod dio16_configuration;
pub use dio16_configuration::*;

mod dio17_configuration;
pub use dio17_configuration::*;

mod dio18_configuration;
pub use dio18_configuration::*;

mod dio19_configuration;
pub use dio19_configuration::*;

mod dio0_configuration;
pub use dio0_configuration::*;

mod commissioning_button;
pub use commissioning_button::*;

mod dio1_configuration;
pub use dio1_configuration::*;

mod dio2_configuration;
pub use dio2_configuration::*;

mod dio3_configuration;
pub use dio3_configuration::*;

mod dio4_configuration;
pub use dio4_configuration::*;

mod dio5_configuration;
pub use dio5_configuration::*;

mod dio8_configuration;
pub use dio8_configuration::*;

mod dio9_configuration;
pub use dio9_configuration::*;

mod dio10_configuration;
pub use dio10_configuration::*;

mod dio11_configuration;
pub use dio11_configuration::*;

mod dio12_configuration;
pub use dio12_configuration::*;

mod pull_up_down_resistor_enable;
pub use pull_up_down_resistor_enable::*;

mod pull_up_down_direction;
pub use pull_up_down_direction::*;

mod pwm0_duty_cycle;
pub use pwm0_duty_cycle::*;

mod pwm1_duty_cycle;
pub use pwm1_duty_cycle::*;

mod rssipwm_timer;
pub use rssipwm_timer::*;

mod associate_led_blink_time;
pub use associate_led_blink_time::*;

mod io_sample;
pub use io_sample::*;

mod sample_rate;
pub use sample_rate::*;

mod dio_change_detect;
pub use dio_change_detect::*;

mod analog_voltage_reference;
pub use analog_voltage_reference::*;

mod samples_before_tx;
pub use samples_before_tx::*;

mod sleep_sample_rate;
pub use sleep_sample_rate::*;

mod digital_output_level;
pub use digital_output_level::*;

mod io_input_address;
pub use io_input_address::*;

mod io_output_enable;
pub use io_output_enable::*;

mod d0_timeout_timer;
pub use d0_timeout_timer::*;

mod d1_output_timeout_timer;
pub use d1_output_timeout_timer::*;

mod d2_output_timeout_timer;
pub use d2_output_timeout_timer::*;

mod d3_output_timeout_timer;
pub use d3_output_timeout_timer::*;

mod d4_output_timeout_timer;
pub use d4_output_timeout_timer::*;

mod d5_output_timeout_timer;
pub use d5_output_timeout_timer::*;

mod d6_output_timeout_timer;
pub use d6_output_timeout_timer::*;

mod d7_output_timeout_timer;
pub use d7_output_timeout_timer::*;

mod d8_output_timer;
pub use d8_output_timer::*;

mod d9_output_timer;
pub use d9_output_timer::*;

mod p0_output_timer;
pub use p0_output_timer::*;

mod p1_output_timer;
pub use p1_output_timer::*;

mod p2_output_timer;
pub use p2_output_timer::*;

mod pwm_output_timeout;
pub use pwm_output_timeout::*;

mod location_x_latitude;
pub use location_x_latitude::*;

mod location_y_longitude;
pub use location_y_longitude::*;

mod location_z_elevation;
pub use location_z_elevation::*;

mod firmware_version;
pub use firmware_version::*;

mod version_long;
pub use version_long::*;

mod bootloader_version;
pub use bootloader_version::*;

mod hardware_version;
pub use hardware_version::*;

mod power_variant;
pub use power_variant::*;

mod hardware_software_compatibility;
pub use hardware_software_compatibility::*;

mod supply_voltage;
pub use supply_voltage::*;

mod module_temperature;
pub use module_temperature::*;

mod configuration_crc;
pub use configuration_crc::*;

mod invoke_bootloader;
pub use invoke_bootloader::*;

mod manufacturing_date;
pub use manufacturing_date::*;

mod software_reset;
pub use software_reset::*;

mod apply_changes;
pub use apply_changes::*;

mod write;
pub use write::*;

mod restore_defaults;
pub use restore_defaults::*;

mod set_custom_default;
pub use set_custom_default::*;

mod clear_custom_defaults;
pub use clear_custom_defaults::*;

mod restore_factory_defaults;
pub use restore_factory_defaults::*;

