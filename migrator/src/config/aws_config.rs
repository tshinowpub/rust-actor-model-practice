use aws_config::SdkConfig;

#[derive(Default)]
pub struct AwsConfig {}

impl AwsConfig {
    pub async fn aws_config() -> SdkConfig {
        aws_config::load_from_env().await
    }
}
