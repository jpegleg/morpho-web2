![net-gargoyle2](https://carefuldata.com/images/cdlogo.png)

# morpho-web2

A rust template for front-end web server microservice container using actix web framework with legacy TLS via openssl.

#### Also see the rustls version: https://github.com/jpegleg/morpho-web

The included Dockerfile uses the `FROM ekidd/rust-musl-builder AS build` to compile with cargo
and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI
image has no shell, nothing but the dependencies for the web server.

The base image is less than 12MB for the entire framework. The size of the added content from `static`
will increase the image size etc. Alternatively to doing a copy into the container image,
the /app/static directory can be a volume mount containing the content to load. Note that by default the cert and key pair are in /app/ which is the workdir for the server, while the webroot is /app/static/.

From the test docker-compose.yml:

```
    volumes:
      - /opt/protean-gitops/static/:/app/static/
      - /opt/protean-gitops/cert.pem:/app/cert.pem
      - /opt/protean-gitops/privkey.pem:/app/privkey.pem
```

In production, rather than using Docker, we can use Kubernetes and mount those more appropriately.
The purpose of the docker-compose.yml and the protean references are for some testing systems usage.

## openssl for HTTPS

This program uses openssl for TLS, leveraging the wide range of support. Some types of PKI systems
and ASN1 structures don't work in rustls at this time, so openssl can be used to improve certificate
feature support. This version of morpho uses only TLSv1.2.
See https://github.com/jpegleg/morpho-web for using rustls instead of openssl and defaulting to TLSv1.3.

## tokio async io

We can serve a lot of requests with actix use of tokio async io, letting IO-bound workloads scale very well.
The reading of files from the filesystem is not special in terms of performance, peforming much like other
web servers. The performance is very good and reliable.

## cloud native design

This web server template is cloud native, working well in Kubernetes and Docker, etc.
It works well with many replicas, has a minimized set of dependencies and libraries,
and puts security as a priority.

## redirecting to HTTPS

Port redirection is included by default now.

## security headers

HSTS and security headers are inserted by default.

