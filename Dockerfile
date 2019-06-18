FROM rust:stretch

RUN git clone https://github.com/irevoire/crustyline && \
	cd crustyline && \
	cargo build --release && \
	mv target/release/crustyline . && \
	rm -rf src target

WORKDIR crustyline

EXPOSE 7878

CMD ./crustyline
