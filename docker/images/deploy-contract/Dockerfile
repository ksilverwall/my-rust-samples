FROM node:18.14.2-slim

COPY ./package.json /app/package.json

WORKDIR /app

RUN npm i

COPY ./main.js /app/main.js

ENTRYPOINT ["node", "/app/main.js"]
