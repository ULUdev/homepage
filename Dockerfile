FROM rust:latest
COPY . /homepage
WORKDIR /homepage
RUN apt-get update
RUN apt-get install sassc yarnpkg -y
RUN cd typescript && yarnpkg && cd ..
RUN make
CMD ["./target/release/homepage"]
