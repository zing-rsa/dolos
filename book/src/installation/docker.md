# Docker

_Dolos_ provides already built public Docker images through Github Packages. To execute _Dolos_ via Docker, use the following command:

```sh
docker run ghcr.io/txpipe/dolos:latest
```

The result of the above command should show _Dolos'_ command-line help message.


## Entry Point

The entry-point of the image points to _Dolos_ executable. You can pass the same command-line arguments that you would pass to the binary release running bare-metal. For example:

```
docker run -it ghcr.io/txpipe/dolos:latest \
    --config my-custom-config.toml
```

For more information on available command-line arguments, check the [usage](../usage/index.md) section.


## Using a Configuration File

The default daemon configuration file for _Dolos_ is located in `/etc/dolos/daemon.toml`. To run _Dolos_ in daemon mode with a custom configuration file, you need to mount it in the correct location. The following example runs a docker container in background using a configuration file named `daemon.toml` located in the current folder:

```
docker run -d -v $(pwd)/daemon.toml:/etc/dolos/daemon.toml \
    ghcr.io/txpipe/dolos:latest daemon
```

## Versioned Images

Images are also tagged with the corresponding version number. It is highly recommended to use a fixed image version in production environments to avoid the effects of new features being included in each release (please remember dolos hasn't reached v1 stability guarantees).

To use a versioned image, replace the `latest` tag by the desired version with the `v` prefix. For example, to use version `1.0.0`, use the following image:

```
ghcr.io/txpipe/dolos:v1.0.0
```
