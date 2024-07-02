# smolk8s

This is a super bare-bones setup of k8s just for people to get it on their systems and start messing around.

## Create the cluster

Pick a name for your cluster, let's call our `steve`. Run the following command:

```sh
./setup
```

It will ask you for the name of the cluster, and then it will create the cluster and configure the container registry.

## Deploy a simple app

To deploy a simple app, cd into the directory, e.g., `hello-py` and run:

```sh
CLUSTER_NAME=steve make deploy
```

This will build the docker image, push the image to the cluster, and then deploy the app to the cluster.

Afterwards you can ping it with:

```sh
make test
```

From here, feel free to play around with the `manifest.yaml` to learn about how that all works, mess around with the app, go hog wild.

## Cleanup

To clean up the cluster, run:

```sh
kind delete cluster --name steve
```
