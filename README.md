# RusCntl – Custom Kubernetes Pod Labeling Controller

⚠️⚠️⚠️<br>
This is a toy I created to play with Rust and kubernetes. Please note that it lacks many features that are Production Ready<br>
⚠️⚠️⚠️

## Project Overview

**RusCntl** is a custom Kubernetes controller written in Rust that monitors Pods within a Kubernetes cluster and automatically adds a specific label (`learning: rust`) to them. This controller ensures that Pods are consistently labeled, facilitating easier management and identification within your cluster.

## Features

- **Automatic Labeling:** Adds the `learning: rust` label to Pods that do not already have it.
- **Event Monitoring:** Watches for Pod creation (`Apply`) and deletion (`Delete`) events in real-time.
- **Duplicate Processing Prevention:** Utilizes annotations to prevent the controller from processing the same Pod multiple times.
- **Comprehensive Logging:** Logs all significant actions, including label additions and Pod deletions, for easy monitoring and troubleshooting.

## Requirements

- **Kubernetes Cluster:** Ensure you have access to a Kubernetes cluster where you have permissions to deploy custom controllers.
- **Rust Environment:** Rust compiler and Cargo must be installed. You can install them from the [official Rust website](https://www.rust-lang.org/tools/install).
- **Docker:** Required for building and pushing Docker images.
- **kubectl:** Kubernetes command-line tool for interacting with your cluster.

## Installation and Setup

### 1. Clone the Repository

```sh
git clone https://github.com/yourusername/ruscntl.git
cd ruscntl
```

### 2. Set Up Rust Environment

If Rust is not installed, follow the instructions on [the official Rust installation page](https://www.rust-lang.org/tools/install) to set it up.

### 3. Build the Controller

Build the Rust project in release mode:

`cargo build --release`

### 4. Run controller

```sh
$ ./target/release/ruscntl
2024-12-01T11:28:35.258350Z  INFO ruscntl: Starting label controller...
```

## Usage

1. Create a Sample Pod

Deploy a sample Pod to see RusCntl in action. Below is an example `pod.yaml:`

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  containers:
    - name: nginx
      image: nginx:1.14.2
      ports:
        - containerPort: 80
```

Apply the Pod:

`kubectl apply -f pod.yaml`

### 2. Verify Label Addition

After the Pod is created, RusCntl will automatically add the learning: rust label if it’s not already present. Verify the label:

`kubectl get pod nginx --show-labels`

You should see the `learning=rust` label associated with the Pod.

### 3. Monitor Logs

To view the logs of RusCntl and ensure it’s functioning correctly:

You should see logs indicating that labels have been added to Pods and when Pods are deleted.

### Code Structure

- main.rs: Contains the main logic for initializing the controller, setting up watchers, and handling Pod events.
- Dependencies:
  - kube: Kubernetes client library for Rust.
  - kube-runtime: Provides runtime utilities for building Kubernetes controllers.
  - tracing: For structured logging.
  - tokio: Asynchronous runtime for Rust.

### How It Works

1. Event Watching:
    - RusCntl utilizes kube_runtime::watcher to monitor Pod events (Apply and Delete) in the Kubernetes cluster.

2. Label Addition:
    - When a Pod is created or updated (Apply event), RusCntl checks if the learning: rust label is present.
    - If the label is missing, it adds the label and annotates the Pod to mark it as processed, ensuring the label is added only once.
3. Logging:
    - All significant actions, such as adding labels and deleting Pods, are logged using the tracing crate for easy monitoring and debugging.

### Known Issues

- **Duplicate Logging:** In some cases, RusCntl may log actions multiple times for a single Pod event. This can occur if the controller processes its own updates. The current implementation uses annotations to prevent duplicate processing, but additional checks may be required for environments with multiple controllers or complex Pod update patterns.

### License
This project is licensed under the MIT License.
