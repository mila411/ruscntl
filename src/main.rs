use futures::StreamExt;
use k8s_openapi::serde_json::json;
use kube::{Api, Client};
use kube_runtime::reflector::Lookup;
use kube_runtime::watcher::{self, watcher};
use tracing::{debug, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging setup
    tracing_subscriber::fmt::init();

    // Initialize the Kubernetes API client
    let client = Client::try_default().await?;
    let pods: Api<k8s_openapi::api::core::v1::Pod> = Api::all(client.clone());

    // Start the controller that monitors the Pod.
    info!("Starting label controller...");

    // Watcher monitors Pod events
    let watcher = watcher(pods, watcher::Config::default());
    tokio::pin!(watcher);

    while let Some(event) = watcher.next().await {
        match event? {
            watcher::Event::Apply(pod) => {
                handle_pod(&client, pod).await?;
            }
            watcher::Event::Delete(pod) => {
                handle_pod_deletion(&client, pod).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn handle_pod(
    client: &Client,
    pod: k8s_openapi::api::core::v1::Pod,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("handle_pod called for Pod {:?}", pod.name());

    let mut labels = pod.metadata.labels.clone().unwrap_or_default();
    if !labels.contains_key("learning") || labels["learning"] != "rust" {
        labels.insert("learning".to_string(), "rust".to_string());

        let name = pod.name().unwrap_or_else(|| "unknown".into());
        let namespace = Lookup::namespace(&pod).unwrap_or_else(|| "default".to_string().into());
        let pods: Api<k8s_openapi::api::core::v1::Pod> =
            Api::namespaced(client.clone(), &namespace);

        let patch = json!({
            "metadata": {
                "labels": labels
            }
        });

        pods.patch(
            &name,
            &kube::api::PatchParams::apply("my_controller"),
            &kube::api::Patch::Merge(&patch),
        )
        .await?;
        info!("Added label 'learning: rust' to Pod {:?}", name);
    }

    Ok(())
}

async fn handle_pod_deletion(
    client: &Client,
    pod: k8s_openapi::api::core::v1::Pod,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = pod.name();
    info!("Pod {:?} has been deleted", name);
    Ok(())
}
