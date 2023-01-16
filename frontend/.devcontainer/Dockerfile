# syntax=docker/dockerfile:1.4
FROM node:18-bullseye-slim

RUN <<EOT
apt-get update
apt-get install -y git
EOT

WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm install
# COPY . .

# WORKDIR /app/frontend
# RUN npm run build
# CMD ["npm", "run", "start"]
