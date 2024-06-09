FROM rust:1.68

#Switching workDir to our app
WORKDIR /app
#Install Linkers to produce single exectuable
RUN apt update && apt install lld clang -y

# copy all files from our local directory to our working dir app

COPY . .

#Lets build our production ready rust
RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]