# AWS Cloudflare IP Whitelist

Docker image for automating IP whitelist of Cloudflare IPs. Created using **Rust** language.

Cloudflare range of IPs are available under the following links:

- https://www.cloudflare.com/ips-v4
- https://www.cloudflare.com/ips-v6

## Usage

Pull from ghcr:

```bash
docker pull ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest
```

Build locally:

```bash
docker build . -t ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest
```

Run:

```bash
docker run --rm -v ${HOME}/.aws:/root/.aws:ro ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest <security-group-id> [<ports>, ...]
```

- `security-group-id` is the AWS security group ID.
- `ports` is the port that will be whitelisted. default is set to `80` (HTTP port). Specify the port using comma separated or space separated.

```bash
docker run --rm -v ${HOME}/.aws:/root/.aws:ro ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest sg-0cff43a33f085df79
docker run --rm -v ${HOME}/.aws:/root/.aws:ro ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest sg-0cff43a33f085df79 80,443
docker run --rm -v ${HOME}/.aws:/root/.aws:ro ghcr.io/novalagung/aws-cloudflare-ip-whitelist:latest sg-0cff43a33f085df79 80 443
```

> WARNING! This approach will mount `~/.aws` into docker, and this is not a good practice

## License

MIT License

## Author

Noval Agung Prayogo
