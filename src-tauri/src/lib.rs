use bollard_next::Docker;

/// dockerの環境にアクセスできるか(ローカル限定)をチェックする
///
/// # Returns
/// bool: true: dockerの環境にアクセスできる
///
/// # Examples
/// ```
/// use src_tauri::docker::check_docker_env;
/// ```
async fn check_docker_env() -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let version = docker.version().await;
    version.is_ok()
}




/// check_docker_env()のテストコード
/// Docker環境を立ち上げていればtrueです、失敗を見たければdocker engineを停止してください
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_docker_env() {
        assert_eq!(check_docker_env().await, true);
    }
}
