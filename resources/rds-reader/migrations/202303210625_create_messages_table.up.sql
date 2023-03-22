CREATE TABLE IF NOT EXISTS messages (
    message_id bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
    message_write_id VARCHAR(36) NOT NULL,
    account_id int(10) UNSIGNED NOT NULL,
    channel_id int(10) UNSIGNED NOT NULL,
    message text NOT NULL,
    created_at DATETIME,
    updated_at DATETIME,
    deleted_at DATETIME,
    PRIMARY KEY (message_id)
) ENGINE=InnoDB;
