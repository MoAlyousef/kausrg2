FROM alpine:latest AS build
WORKDIR /app
COPY . .
RUN cp /app/kausurg /server
RUN cp -r /app/assets /assets 
RUN cp -r /app/db /db

FROM scratch
COPY --from=build /server /server
COPY --from=build /assets /assets
COPY --from=build /db /db
EXPOSE 8000
CMD ["/server"]
