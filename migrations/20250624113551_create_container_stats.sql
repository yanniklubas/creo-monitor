CREATE TABLE IF NOT EXISTS container_stats (
    timestamp  BIGINT UNSIGNED NOT NULL,
    container_id VARCHAR(255) NOT NULL,
    machine_id BINARY(16) NOT NULL,
    cpu_usage_usec BIGINT UNSIGNED,
    cpu_user_usec BIGINT UNSIGNED,
    cpu_system_usec BIGINT UNSIGNED,
    cpu_nr_periods BIGINT UNSIGNED,
    cpu_nr_throttled BIGINT UNSIGNED,
    cpu_throttled_usec BIGINT UNSIGNED,
    cpu_nr_bursts BIGINT UNSIGNED,
    cpu_burst_usec BIGINT UNSIGNED,
    cpu_quota BIGINT UNSIGNED,
    cpu_period BIGINT UNSIGNED,
    memory_anon BIGINT UNSIGNED,
    memory_file BIGINT UNSIGNED,
    memory_kernel_stack BIGINT UNSIGNED,
    memory_slab BIGINT UNSIGNED,
    memory_sock BIGINT UNSIGNED,
    memory_shmem BIGINT UNSIGNED,
    memory_file_mapped BIGINT UNSIGNED,
    memory_usage_bytes BIGINT UNSIGNED,
    memory_limit_bytes BIGINT UNSIGNED,
    io_rbytes BIGINT UNSIGNED,
    io_wbytes BIGINT UNSIGNED,
    io_rios BIGINT UNSIGNED,
    io_wios BIGINT UNSIGNED,
    net_rx_bytes BIGINT UNSIGNED,
    net_rx_packets BIGINT UNSIGNED,
    net_tx_bytes BIGINT UNSIGNED,
    net_tx_packets BIGINT UNSIGNED,

    PRIMARY KEY (timestamp, container_id, machine_id)
);
