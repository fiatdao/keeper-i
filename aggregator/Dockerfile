FROM node:14.19.1-alpine
WORKDIR /usr/src/
ADD . .
RUN yarn install && yarn build
CMD [ "yarn", "start" ]
EXPOSE 8000
