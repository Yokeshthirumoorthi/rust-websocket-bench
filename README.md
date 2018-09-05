# rust-websocket-bench
Distributed load testing for rust websocket

RUST is a fast, safe and efficient programming language to develope
highly concurrent and parallel programs. So I believe to acheive something like this blog - [erlang-phoenix 2 million websocket connection.]( http://phoenixframework.org/blog/the-road-to-2-million-websocket-connections).

## Requirements

* Rust
* Docker and Docker Compose
* Ruby
* DigitalOcean account

## Build Instructions

1. Prerequisites for build

    Install Rust by run the following in your terminal

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
    or follow [this official installation instruction](https://www.rust-lang.org/en-US/install.html).

## Run the benchmark

Export your DigitalOcean API key (can also be set in in the configuration)
```
export DO_TOKEN=<your token>
```

Change your directory to tsungcluster
```
cd tsungcluster
```

Copy and edit the example configuration:
```
cp config.yml.example config.yml
```

**And set the server ip in config.yml**


Setup the droplets (this will take a few minutes depending on the amount of workers configured):
```
rake setup
```

When the setup is finished it will output the IP of the Tsung master Droplet. Open the URL in a browser, the site will be available after starting the application.

Run `docker-compose up` to start the Tsung cluster. It will first wait for all slaves to be available and then run the benchmark.

Reload the Tsung master browser tab and watch the graphs :)

After you are done stop and remove the droplets:

```
rake teardown
```

## Authur

1. Yokesh Thirumoorthi - initial author - yokesh@pdx.edu

## Sending Feedback

This project is always open to [your feedback](https://github.com/Yokeshthirumoorthi/rusher/issues).

## Credits
This application uses Open Source components. You can find the source code of their open source projects along with license information below. I acknowledge and am grateful to these developers for their contributions to open source.

```
Project: https://github.com/actix/examples/tree/master/websocket-chat/
Copyright (c) 2017 Nikolay Kim (fafhrd91@gmail.com)
License (MIT) https://github.com/actix/actix-web/blob/master/LICENSE-MIT

Project: https://github.com/diesel-rs/diesel
Copyright (c) 2015-2018 Sean Griffin
License (MIT) https://github.com/diesel-rs/diesel/blob/master/LICENSE-MIT

Project: https://github.com/dsander/phoenix-connection-benchmark
```
