![cdlogo](https://carefuldata.com/images/cdlogo.png)

# morpho-web2

A rust template for front-end web server microservice container using actix web framework with legacy TLS via openssl.

#### Also see the rustls version: https://github.com/jpegleg/morpho-web

The included Dockerfile uses the `FROM ekidd/rust-musl-builder AS build` to compile with cargo
and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI
image has no shell, nothing but the dependencies for the web server.

The base image is less than 12MB for the entire framework. The size of the added content from `static`
will increase the image size etc. Alternatively to doing a copy into the container image,
the /app/static directory can be a volume mount containing the content to load. Unlike the main morpho-web, morpho-web2 does
not do a COPY on the static web dir or the cryptographic files and requires that they are mounted.

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
feature support. This version of morpho uses TLSv1.2, but also can support TLSv1.0 and TLSv1.1.
See https://github.com/jpegleg/morpho-web for using rustls instead of openssl and defaulting to TLSv1.3.

<b>Scan morpho2 before use to ensure TLS configurations are as desired, morpho-web2 is not a compliant TLS build because it can use TLSv1.0. Use https://github.com/jpegleg/morpho-web and rustls instead when possible.</b>

## tokio async io

We can serve a lot of requests with actix use of tokio async io, letting IO-bound workloads scale very well.
The reading of files from the filesystem is not special in terms of performance, peforming much like other
web servers. The performance is very good and reliable.

## cloud native design

This web server template is cloud native, working well in Kubernetes and Docker, etc.
It works well with many replicas, has a minimized set of dependencies and libraries,
and puts security as a priority, other than potentially allowing weaker than
desired TLS configurations like the default on morpho-web2.

## redirecting to HTTPS

Port redirection is included by default now.

## security headers

HSTS and security headers are inserted by default.

