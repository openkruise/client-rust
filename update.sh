#!/bin/bash


set -eoux pipefail

VERSION="v1.0.0"

APIS=(
    advancedcronjobs
    broadcastjobs
    statefulsets
)

rm -rf src/apis/

mkdir -p src/apis/

echo "// WARNING! generated file do not edit" > src/apis/mod.rs

for API in "${APIS[@]}"
do
    echo "generating  api ${API}"
    curl -sSL "https://raw.githubusercontent.com/openkruise/kruise/master/config/crd/bases/apps.kruise.io_${API}.yaml"  | kopium -Af - > src/apis/${API}.rs
    echo "//pub mod ${API};" >> src/apis/mod.rs
done
