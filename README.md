
# Ding server

A tool to notify you when long running commands are done.
## Installation

Install ding-server with docker:

```bash
docker run -v $PWD/Ding.toml:/config/Ding.toml -p 8000:8000 ghcr.io/henrikcoll/ding-server
```

Install ding server with docker-compose:

```yaml
version: '3'

services:
  ding:
    image: ghcr.io/henrikcoll/ding-server
    restart: always
    volumes:
      - ./config:/config
    ports:
      - 8000:8000
```
## Configuration

Example:
```toml
[[users]]
username = "username"
secret = "my_secret"

[users.discord_webhook]
url = "https://discord.com/api/webhooks/..."

[users.email]
email = "username@example.com"

[email]
from = "ding@example.com"
username = "ding@example.com"
password = "smtp password"
server = "smtp.example.com"
```
## Usage/Examples

curl: 
```bash
curl localhost:8000/v1/my_secret
```

([ding-cli](https://github.com/henrikcoll/ding-cli)):
```bash
export DING_SERVER=localhost:8000
export DING_SECRET=my_secret

ding
```
