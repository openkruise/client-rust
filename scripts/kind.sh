#/bin/bash

set -o errexit
set -o nounset
set -o pipefail

wget https://github.com/kubernetes-sigs/kind/releases/download/v0.20.0/kind-linux-amd64
wget https://storage.googleapis.com/kubernetes-release/release/v1.28.0/bin/linux/amd64/kubectl
mv kind-linux-amd64 kind && chmod +x kind && mv kind /usr/local/bin
chmod +x kubectl && mv kubectl /usr/local/bin

kind create cluster
kubectl wait node --all --for condition=ready
kubectl apply -f https://raw.githubusercontent.com/openkruise/kruise/v1.0.0/config/crd/bases/apps.kruise.io_statefulsets.yaml
