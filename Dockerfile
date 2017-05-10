FROM kenansulayman/rust-nightly:latest

ADD server /server
ADD static /www

RUN cd /server && cargo build -v --release

CMD ["/server/target/release/blizzard"]
