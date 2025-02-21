FROM alpine:latest as build
RUN apk add rust cargo
WORKDIR /app
COPY . .
RUN cargo build --release 
RUN strip /app/target/release/kausurg
RUN cp /app/target/release/kausurg /server
RUN cp -r /app/assets /assets 
RUN cp -r /app/db /db

FROM alpine:latest
COPY --from=build /server /server
COPY --from=build /assets /assets
COPY --from=build /db /db
EXPOSE 8000
CMD ["/server"]
