FROM postgres:15rc2-alpine3.16
COPY ./init.sql /docker-entrypoint-initdb.d/
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD fallvacation
ENV POSTGRES_DB fallvacation