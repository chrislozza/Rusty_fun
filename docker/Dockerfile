FROM debian:latest
RUN mkdir /mycode
RUN apt update && apt -y install vim python
COPY hello /mycode/hello
CMD [ "/mycode/hello" ]


# RUN debian:latest
# CMD [ "sleep", "infinity" ]
# docker exec -it NAME_OF_CONTAINER bash
