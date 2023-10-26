pub mod apis;

#[cfg(test)]
mod tests {
    use std::process::Command;

    use anyhow::Error;
    use kube::api::{PostParams};
    use kube::client::ConfigExt;
    use kube::CustomResourceExt;
    use kube::config::{KubeConfigOptions, Kubeconfig};
    use kube::core::ObjectMeta;
    use kube::{Api, Config};
    use tower::ServiceBuilder;
    use uuid::Uuid;

    use crate::apis::statefulsets::{StatefulSet,StatefulSetSpec,StatefulSetSelector};
    use std::collections::HashMap;

    // -------------------------------------------------------------------------
    // Tests
    // -------------------------------------------------------------------------

    // #[ignore]
    #[tokio::test]
    async fn deploy_sts() -> Result<(), Error> {
        let (client, cluster) = get_client().await?;
        let info = client.apiserver_version().await?;

        println!(
            "kind cluster {} is running, server version: {}",
            cluster.name, info.git_version
        );

        let selector = StatefulSetSelector{
            match_labels: None,
            match_expressions: None,
        };

        let map: HashMap<String, serde_json::Value> = HashMap::new();

        let mut sts = StatefulSet{
            metadata: ObjectMeta::default(),
            spec: StatefulSetSpec{
                replicas: Some(1),
                lifecycle: None,
                pod_management_policy: None,
                reserve_ordinals: None,
                scale_strategy:None,
                revision_history_limit: None,
                selector:selector,
                service_name: Some("svc".to_string()),
                update_strategy: None,
                volume_claim_templates: None,
                template: map,
            },
            status: None,
        };
        sts.metadata.name=Some("hello".to_string());
        println!(
            "begin apply sts!"
        );

        // let data = check()?;
        // println!("check.data:{}",data);

        let api: Api<StatefulSet> = Api::namespaced(client.clone(), "default");
        api.create(&PostParams::default(), &sts).await?;

        let exist_sts = api.get("hello").await?;
        if exist_sts.metadata.name.unwrap()!="hello" {
            return Err(Error::msg(String::from("have not exect statefulSet")));
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Test Utilities
    // -------------------------------------------------------------------------

    struct Cluster {
        name: String,
    }

    impl Drop for Cluster {
        fn drop(&mut self) {
            // match delete_kind_cluster(&self.name) {
            //     Err(err) => panic!("failed to cleanup kind cluster {}: {}", self.name, err),
            //     Ok(()) => {}
            // }
        }
    }

    async fn get_client() -> Result<(kube::Client, Cluster), Error> {
        let cluster = create_kind_cluster()?;
        let kubeconfig_yaml = get_kind_kubeconfig(&cluster.name)?;
        let kubeconfig = Kubeconfig::from_yaml(&kubeconfig_yaml)?;
        let config =
            Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .service(hyper::Client::builder().build(config.openssl_https_connector()?));

        let client = kube::Client::new(service, config.default_namespace);

        Ok((client, cluster))
    }

    fn get_kind_kubeconfig(cluster_name: &str) -> Result<String, Error> {
        println!("get kind kubeconfig from cluster:{}",cluster_name);
        let output = Command::new("kind")
            .arg("get")
            .arg("kubeconfig")
            .arg("--name")
            .arg("kind")
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(String::from_utf8(output.stderr)?));
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    fn create_kind_cluster() -> Result<Cluster, Error> {
        Ok(Cluster { name: String::from("kind") })
    }

    fn delete_kind_cluster(cluster_name: &str) -> Result<(), Error> {
        let output = Command::new("kind")
            .arg("delete")
            .arg("cluster")
            .arg("--name")
            .arg(cluster_name)
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(String::from_utf8(output.stderr)?));
        }

        Ok(())
    }

}
