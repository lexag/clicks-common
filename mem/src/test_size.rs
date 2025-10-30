#[cfg(test)]
mod tests {
    macro_rules! print_size_of {
        ($name:ty) => {
            println!("{} is {} bytes", stringify!($name), mem::size_of::<$name>());
        };
    }

    use std::mem;

    #[test]
    fn size_printout() {
        print_size_of!(nostd::status::Notification);
        print_size_of!(nostd::status::AudioSourceState);
        print_size_of!(nostd::config::JACKServerConfiguration);
        print_size_of!(nostd::config::JACKClientConfiguration);
        print_size_of!(nostd::config::ChannelConfiguration);
        print_size_of!(nostd::config::SystemConfiguration);
        print_size_of!(nostd::config::LoggerConfiguration);
        print_size_of!(nostd::config::AudioConfiguration);
        print_size_of!(nostd::timecode::TimecodeInstant);
        print_size_of!(nostd::status::NotificationKind);
        print_size_of!(nostd::network::SubscriberInfo);
        print_size_of!(nostd::network::ConnectionInfo);
        print_size_of!(nostd::event::BeatEventContainer);
        print_size_of!(nostd::status::TransportState);
        print_size_of!(nostd::network::NetworkStatus);
        print_size_of!(nostd::status::CombinedStatus);
        print_size_of!(nostd::status::PlaybackState);
        print_size_of!(nostd::network::AudioDevice);
        print_size_of!(nostd::network::JACKStatus);
        print_size_of!(nostd::show::ShowMetadata);
        print_size_of!(nostd::config::LogContext);
        print_size_of!(nostd::network::IpAddress);
        print_size_of!(nostd::network::Heartbeat);
        print_size_of!(nostd::status::BeatState);
        print_size_of!(nostd::status::CueState);
        print_size_of!(nostd::cue::CueMetadata);
        print_size_of!(nostd::config::LogKind);
        print_size_of!(nostd::show::Show);
        print_size_of!(nostd::beat::Beat);
        print_size_of!(nostd::cue::Cue);
        print_size_of!(nostd::cue::CueSkeleton);
        print_size_of!(nostd::show::ShowSkeleton);
    }
}
