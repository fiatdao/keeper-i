FROM node:14.19.1-alpine
WORKDIR /usr/src/
ADD . .
RUN yarn install
CMD [ "yarn", "start" ]
EXPOSE 9000
