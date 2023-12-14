use bollard_next::{Docker, image::ListImagesOptions, volume::ListVolumesOptions};

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

/// 指定したイメージが存在するかをチェックする
/// # Arguments
/// * `image_name` - イメージ名
/// # Returns
/// bool: true: イメージが存在する
async fn check_image_exist(image_name: &str) -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let options = ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    };
    let images = docker.list_images(Some(options)).await.unwrap();
    for image in images {
        if let Some(name) = image.repo_tags.first() {
            if name == image_name {
                return true;
            }
        }
    }
    false
}

/// 指定したボリュームが存在するかを確認する
/// # Arguments
/// * `volume_name` - ボリューム名
/// # Returns
/// bool: true: ボリュームが存在する
async fn check_volume_exist(volume_name: &str) -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let options = ListVolumesOptions::<String> {
        ..Default::default()
    };
    let volumes = docker.list_volumes(Some(options)).await.unwrap();
    for volume in volumes.volumes.unwrap() {
        if volume_name == volume.name {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    /// check_docker_env()のテストコード
    /// Docker環境を立ち上げていればtrueです、失敗を見たければdocker engineを停止してください
    #[tokio::test]
    async fn test_check_docker_env() {
        assert_eq!(check_docker_env().await, true);
    }

    /// check_image_exist()のテストコード
    /// alpineイメージが存在すればtrueです
    #[tokio::test]
    async fn test_check_image_exist() {
        assert_eq!(check_image_exist("alpine:latest").await, true);
    }

    /// check_volume_exist()のテストコード
    /// ubuntu-homeがあればtrueです
    #[tokio::test]
    async fn test_check_volume_exist() {
        assert_eq!(check_volume_exist("ubuntu-home").await, true);
    }
}
