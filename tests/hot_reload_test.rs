#[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::Write,
        path::PathBuf,
        sync::Arc,
        thread,
        time::Duration,
    };

    use realme::{Adaptor, FileSource, JsonParser, Realme, TomlParser};

    #[test]
    #[cfg(all(feature = "toml", feature = "json", feature = "hot_reload"))]
    fn test_concurrent_multi_source_hot_reload() {
        // 创建临时配置文件
        let toml_path = PathBuf::from("test_config.toml");
        let json_path = PathBuf::from("test_config.json");

        let initial_toml = r#"
            [server]
            port = 8080
            host = "localhost"
        "#;
        let initial_json = r#"
            {
                "database": {
                    "url": "postgres://localhost/mydb",
                    "max_connections": 100
                }
            }
        "#;

        File::create(&toml_path)
            .unwrap()
            .write_all(initial_toml.as_bytes())
            .unwrap();
        File::create(&json_path)
            .unwrap()
            .write_all(initial_json.as_bytes())
            .unwrap();

        // 创建 Realme 实例
        let realme = Arc::new(
            Realme::builder()
                .load(
                    Adaptor::new(FileSource::<TomlParser>::new(
                        toml_path.clone(),
                    ))
                    .watch(),
                )
                .load(
                    Adaptor::new(FileSource::<JsonParser>::new(
                        json_path.clone(),
                    ))
                    .watch(),
                )
                .shared_build()
                .expect("Building configuration object"),
        );

        // 创建多个线程来读取配置
        let mut handles = vec![];
        for _ in 0..5 {
            let realme_clone = Arc::clone(&realme);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let config = realme_clone.get_realme().unwrap();
                    let port: u16 =
                        config.get("server.port").unwrap().try_into().unwrap();
                    let max_connections: u32 = config
                        .get("database.max_connections")
                        .unwrap()
                        .try_into()
                        .unwrap();
                    assert!(port == 8080 || port == 9090);
                    assert!(max_connections == 100 || max_connections == 200);
                    thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        // 主线程修改配置文件
        thread::sleep(Duration::from_millis(100));

        // 修改 TOML 文件
        let mut toml_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&toml_path)
            .unwrap();
        let new_toml = r#"
            [server]
            port = 9090
            host = "localhost"
        "#;
        toml_file.write_all(new_toml.as_bytes()).unwrap();

        // 修改 JSON 文件
        let mut json_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&json_path)
            .unwrap();
        let new_json = r#"
            {
                "database": {
                    "url": "postgres://localhost/mydb",
                    "max_connections": 200
                }
            }
        "#;
        json_file.write_all(new_json.as_bytes()).unwrap();

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 最后检查配置是否已更新
        let final_config = realme.get_realme().unwrap();
        let final_port: u16 =
            final_config.get("server.port").unwrap().try_into().unwrap();
        let final_max_connections: u32 = final_config
            .get("database.max_connections")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(final_port, 9090);
        assert_eq!(final_max_connections, 200);

        // 清理临时文件
        std::fs::remove_file(toml_path).unwrap();
        std::fs::remove_file(json_path).unwrap();
    }
}
