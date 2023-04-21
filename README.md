# Substrate Raft Setup

Example showing how to plug in custom authority permission logic into `substrate-raft`.
In order to run in locally you need to build `substrate-raft-setup` docker image and run `docker-compose`.

## Authority service
Project requires to run an authority service, which will test dynamic switching of the permission granting. More details about authority-service, can be found in the [`bin/authority-service/README.md`](./bin/authority-service/README.md) file.

## Running locally

#### Building `substrate-raft-setup` docker image
Please run a build script for building a docker image:
```
bash docker/build.sh
```

Output:
```
➜  substrate-raft-setup git:(milestone-2) bash docker/build.sh
~/Development/substrate/substrate-raft-setup ~/Development/substrate/substrate-raft-setup
Building bright/substrate-raft-setup:latest docker image, hang on!
[+] Building 1471.2s (15/15) FINISHED                                                  
 => [internal] load build definition from Dockerfile                                0.0s
 => => transferring dockerfile: 1.53kB                                              0.0s
 => [internal] load .dockerignore                                                   0.0s
 => => transferring context: 2B                                                     0.0s
 => [internal] load metadata for docker.io/library/ubuntu:20.04                     1.3s
 => [internal] load metadata for docker.io/paritytech/ci-linux:1c0fde6a-20220811    1.3s
 => [builder 1/4] FROM docker.io/paritytech/ci-linux:1c0fde6a-20220811@sha256:4e8c072ea12bc17d99cb531adb58dea5a4c7d4880a8a86525052d24d1454e89e 0.0s
 => => resolve docker.io/paritytech/ci-linux:1c0fde6a-20220811@sha256:4e8c072ea12bc17d99cb531adb58dea5a4c7d4880a8a86525052d24d1454e89e  0.0s
 => [stage-1 1/5] FROM docker.io/library/ubuntu:20.04@sha256:db8bf6f4fb351aa7a26e27ba2686cf35a6a409f65603e59d4c203e58387dc6b3   0.0s
 => [internal] load build context                                                  96.3s
 => => transferring context: 8.12GB                                                95.8s
 => CACHED [stage-1 2/5] RUN apt-get update                                         0.0s
 => CACHED [stage-1 3/5] RUN apt-get install -y openssl                             0.0s
 => CACHED [builder 2/4] WORKDIR /node-template                                     0.0s
 => [builder 3/4] COPY . .                                                         29.4s
 => [builder 4/4] RUN cargo build --locked --release                             1342.6s
 => [stage-1 4/5] COPY --from=builder /node-template/target/release/node-template /usr/local/bin               0.2s 
 => [stage-1 5/5] RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev &&   mkdir -p /chain-data /node-dev/.local/share &&   chown -R node-dev:node-dev /chain-data &&   ln -s /chain-data /node-dev/.local/share/node-template &&   rm -rf /usr/bin /usr/sbin &&   /usr/local/bin/node-template -  0.5s 
 => exporting to image                                                              0.4s 
 => => exporting layers                                                             0.3s 
 => => writing image sha256:9b5012600dac6ae143abe2d55bde59b191a72d4c02601c9fb2e5aaea8dc140fc                   0.0s 
 => => naming to docker.io/bright/substrate-raft-setup:latest                       0.0s

Use 'docker scan' to run Snyk tests against images to find vulnerabilities and learn how to fix them

real    24m32.763s
user    0m38.686s
sys     0m35.405s
Image is ready
bright/substrate-raft-setup                  latest    9b5012600dac   2 seconds ago   192MB
bright/substrate-raft-setup                  v4.0.0    9b5012600dac   2 seconds ago   192MB
```

#### Running `docker-compose`
We provide a docker compose file, which start one validator node `Alice` and `authority-service`. 
To run it, type in the terminal:
```
docker-compose up --build
```

Output:
```
 substrate-raft-setup git:(milestone-2) ✗ docker-compose up --build
[+] Building 1.3s (12/13)                         
 => [internal] load build definition from authority-service.Dockerfile  0.0s
 => => transferring dockerfile: 50B          0.0s
 => [internal] load .dockerignore            0.0s
 => => transferring context: 2B              0.0s
 => [internal] load metadata for docker.io/library/rust:latest          1.2s
 => [internal] load build context            0.2s
 => => transferring context: 395.60kB        0.2s
 => [builder 1/6] FROM docker.io/library/rust:latest@sha256:9d78a0a4235f3b63f4e8303f53248a146693fc825c15d0831d1e072e474aefdf                            0.0s
 => CACHED [builder 2/6] RUN rustup default nightly                     0.0s
 => CACHED [builder 3/6] RUN USER=root cargo new authority-service      0.0s
 => CACHED [builder 4/6] WORKDIR /authority-service                     0.0s
 => CACHED [builder 5/6] COPY ./bin/authority-service ./                0.0s
 => CACHED [builder 6/6] RUN cargo build     0.0s
 => CACHED [stage-1 2/3] COPY --from=builder /authority-service/target/debug/authority-service .                                                        0.0s
 => CACHED [stage-1 3/3] COPY --from=builder /authority-service/Rocket.toml .                                                                           0.0s
 => exporting to image                       0.0s
 => => exporting layers                      0.0s
 => => writing image sha256:50a887d477b42120155226929fe033b3fc652f30b953119279b5d3800d855454                                                            0.0s
 => => naming to docker.io/library/substrate-raft-setup-permission-service                                                                              0.0s

Use 'docker scan' to run Snyk tests against images to find vulnerabilities and learn how to fix them
[+] Running 2/0
 ⠿ Container substrate-raft-setup-validator-alice-1     Created         0.0s
 ⠿ Container substrate-raft-setup-permission-service-1  Created         0.0s
Attaching to substrate-raft-setup-permission-service-1, substrate-raft-setup-validator-alice-1
substrate-raft-setup-permission-service-1  | Configured for debug.
substrate-raft-setup-permission-service-1  |    >> address: 0.0.0.0
substrate-raft-setup-permission-service-1  |    >> port: 8000
substrate-raft-setup-permission-service-1  |    >> workers: 4
substrate-raft-setup-permission-service-1  |    >> max blocking threads: 512
substrate-raft-setup-permission-service-1  |    >> ident: Rocket
substrate-raft-setup-permission-service-1  |    >> IP header: X-Real-IP
substrate-raft-setup-permission-service-1  |    >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
substrate-raft-setup-permission-service-1  |    >> temp dir: /tmp
substrate-raft-setup-permission-service-1  |    >> http/2: true
substrate-raft-setup-permission-service-1  |    >> keep-alive: 5s
substrate-raft-setup-permission-service-1  |    >> tls: disabled
substrate-raft-setup-permission-service-1  |    >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
substrate-raft-setup-permission-service-1  |    >> log level: normal
substrate-raft-setup-permission-service-1  |    >> cli colors: true
substrate-raft-setup-permission-service-1  | Warning: found set deprecated profile `development`
substrate-raft-setup-permission-service-1  |    >> profile was replaced by `debug`
substrate-raft-setup-permission-service-1  | Routes:
substrate-raft-setup-permission-service-1  |    >> (authorize_fix_order) PUT /authorize_fix_order/<name>
substrate-raft-setup-permission-service-1  |    >> (authorize_slot) PUT /authorize/slot/<slot_nr>
substrate-raft-setup-permission-service-1  |    >> (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> (authorize_session) PUT /authorize/session/<session_nr>
substrate-raft-setup-permission-service-1  | Fairings:
substrate-raft-setup-permission-service-1  |    >> Shield (liftoff, response, singleton)
substrate-raft-setup-permission-service-1  | Shield:
substrate-raft-setup-permission-service-1  |    >> X-Frame-Options: SAMEORIGIN
substrate-raft-setup-permission-service-1  |    >> X-Content-Type-Options: nosniff
substrate-raft-setup-permission-service-1  |    >> Permissions-Policy: interest-cohort=()
substrate-raft-setup-permission-service-1  | Rocket has launched from http://0.0.0.0:8000
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 Substrate Node    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 ✌️  version 4.0.0-dev-bd755fb245e    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2023    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 📋 Chain specification: Development    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 🏷  Node name: picayune-example-2286    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 👤 Role: AUTHORITY    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 💾 Database: RocksDb at /tmp/substratePwiUgV/chains/dev/db/full    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:07 ⛓  Native runtime: node-template-100 (node-template-1.tx1.au1)    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 🔨 Initializing Genesis block/state (state: 0x7353…50ce, header-hash: 0x19c2…bc91)    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 Using default protocol ID "sup" because none is configured in the chain specs    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 🏷  Local node identity is: 12D3KooWEnNRbFhdFXN7GriqP6dDbk1ws6hjgud1atZqeEvgBm2u    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Operating system: linux    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 CPU architecture: x86_64    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Target environment: gnu    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 CPU: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 CPU cores: 4    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Memory: 7859MB    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Kernel: 5.15.49-linuxkit    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Linux distribution: Ubuntu 20.04.6 LTS    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 💻 Virtual machine: yes    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 📦 Highest known block at #0    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 〽️ Prometheus exporter started at 127.0.0.1:9615    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=None    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:08 Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=None    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/1:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/2:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/3:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/slot/280344522:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_slot) PUT /authorize/slot/<slot_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:12 🙌 Starting consensus session on top of parent 0x19c204f592cfeea7c83513e992639058a3acead8fdc86e8c817e5b748f7bbc91    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:12 🎁 Prepared block for proposing at 1 (0 ms) [hash: 0x5584684a096700cc6a6c1f6f23db0d725de177c647b9fbda25786e855d06b957; parent_hash: 0x19c2…bc91; extrinsics (1): [0xe586…4764]]    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:12 🔖 Pre-sealed block for proposal at 1. Hash now 0xdcb5c8d50fca9b7d23237f6ba2e9653cff0fe0c6f58476a93d80498b152ee13b, previously 0x5584684a096700cc6a6c1f6f23db0d725de177c647b9fbda25786e855d06b957.    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:12 ✨ Imported #1 (0xdcb5…e13b)    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/4:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:13 💤 Idle (0 peers), best: #1 (0xdcb5…e13b), finalized #0 (0x19c2…bc91), ⬇ 0 ⬆ 0    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/5:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/6:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/7:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/slot/280344523:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_slot) PUT /authorize/slot/<slot_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:18 🙌 Starting consensus session on top of parent 0xdcb5c8d50fca9b7d23237f6ba2e9653cff0fe0c6f58476a93d80498b152ee13b    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:18 🎁 Prepared block for proposing at 2 (0 ms) [hash: 0x897a489edd7b22993a0e23664b55090fbef2c3278d39886798fc00b745dc3d73; parent_hash: 0xdcb5…e13b; extrinsics (1): [0x6545…2b64]]    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:18 🔖 Pre-sealed block for proposal at 2. Hash now 0x4d5548637638d2bab7b080579fc2be44a616d89b8f2b76842d9944cd6e51ea78, previously 0x897a489edd7b22993a0e23664b55090fbef2c3278d39886798fc00b745dc3d73.    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:18 ✨ Imported #2 (0x4d55…ea78)    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:18 💤 Idle (0 peers), best: #2 (0x4d55…ea78), finalized #0 (0x19c2…bc91), ⬇ 0 ⬆ 0    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/8:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/9:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/10:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/11:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:23 💤 Idle (0 peers), best: #2 (0x4d55…ea78), finalized #0 (0x19c2…bc91), ⬇ 0 ⬆ 0    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/12:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/slot/280344524:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_slot) PUT /authorize/slot/<slot_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:24 🙌 Starting consensus session on top of parent 0x4d5548637638d2bab7b080579fc2be44a616d89b8f2b76842d9944cd6e51ea78    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:24 🎁 Prepared block for proposing at 3 (0 ms) [hash: 0x8e5be7f0f00bc11326c7ca934a4075b572a68bf95e05da811817e637ddd0ecd5; parent_hash: 0x4d55…ea78; extrinsics (1): [0x4c17…35b3]]    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:24 🔖 Pre-sealed block for proposal at 3. Hash now 0xee5f7079101f13087ae954ec367bd98375998c086415a1fe7c77650a1ce9cde4, previously 0x8e5be7f0f00bc11326c7ca934a4075b572a68bf95e05da811817e637ddd0ecd5.    
substrate-raft-setup-validator-alice-1     | 2023-04-21 08:52:24 ✨ Imported #3 (0xee5f…cde4)    
substrate-raft-setup-permission-service-1  | PUT /authorize/round/13:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/14:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
substrate-raft-setup-permission-service-1  | PUT /authorize/round/15:
substrate-raft-setup-permission-service-1  |    >> Matched: (authorize_round) PUT /authorize/round/<round_nr>
substrate-raft-setup-permission-service-1  |    >> Outcome: Success
substrate-raft-setup-permission-service-1  |    >> Response succeeded.
```
