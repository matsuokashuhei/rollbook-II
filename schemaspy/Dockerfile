# syntax=docker/dockerfile:1.4
FROM openjdk:8u212-jdk-alpine

WORKDIR /app
# COPY schemaspy/schemaspy.properties .

RUN <<EOT
apk --update add graphviz ttf-dejavu curl
curl -L https://dev.mysql.com/get/Downloads/Connector-J/mysql-connector-java-8.0.30.tar.gz | tar -xz
mv mysql-connector-java-8.0.30/mysql-connector-java-8.0.30.jar mysql-connector-java.jar
wget -O schemaspy.jar https://github.com/schemaspy/schemaspy/releases/download/v6.1.0/schemaspy-6.1.0.jar
EOT
