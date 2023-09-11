#!/bin/bash


set -eoux pipefail

VERSION="v1.0.0"

APIS=(
    advancedcronjobs
    broadcastjobs
    statefulsets
    containerrecreaterequests
    daemonsets
    ephemeraljobs
    imagepulljobs
    nodeimages
    resourcedistributions
    sidecarsets
    uniteddeployments
    workloadspreads
)

rm -rf src/apis/

mkdir -p src/apis/

echo "// WARNING! generated file do not edit" > src/apis/mod.rs

for API in "${APIS[@]}"
do
    echo "generating  api ${API}"
    curl -sSL "https://raw.githubusercontent.com/openkruise/kruise/${VERSION}/config/crd/bases/apps.kruise.io_${API}.yaml"  | kopium -f - > src/apis/${API}.rs
    echo "//pub mod ${API};" >> src/apis/mod.rs
done

curl -sSL "https://raw.githubusercontent.com/openkruise/kruise/${VERSION}/config/crd/bases/policy.kruise.io_podunavailablebudgets.yaml"  | kopium -f - > src/apis/${API}.rs
echo "//pub mod ${API};" >> src/apis/mod.rs
