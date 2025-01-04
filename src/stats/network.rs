use sysinfo::Networks;

pub fn get_network_stats(
    n: &Networks,
    interval: u32,
) -> String {
    let mut total_received: u64 = 0;
    let mut total_transmitted: u64 = 0;

    for (_, data) in n.iter() {
        total_received += data.received();
        total_transmitted += data.transmitted();
    }

    format!(
        "NETWORK_RX=\"{}KB/s\" NETWORK_TX=\"{}KB/s\" ",
        (total_received / 1024) / interval as u64,
        (total_transmitted / 1024) / interval as u64
    )
}
