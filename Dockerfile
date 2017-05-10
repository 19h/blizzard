FROM kenansulayman/rust-nightly:latest

ADD server /server

RUN cd /server && cargo build -v --release

ADD static /www

CMD ["/server/target/release/blizzard"]
