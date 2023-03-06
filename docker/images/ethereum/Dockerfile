FROM ubuntu:latest

RUN apt-get update && apt-get install -y software-properties-common && apt-get clean
RUN apt-get update && add-apt-repository -y ppa:ethereum/ethereum && apt-get clean
RUN apt-get update && apt-get -y install ethereum=1.11.2+build28359+jammy && apt-get clean
RUN apt-get update && apt-get -y install curl ssh git && apt-get clean

RUN mkdir /opt/scripts
COPY ./setup.sh /opt/scripts/setup.sh
COPY ./boot.sh /boot.sh

RUN chmod 700 /boot.sh

ENTRYPOINT ["/boot.sh"]
