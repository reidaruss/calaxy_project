#!/bin/bash

sudo chmod -R 777 /home/ubuntu/CALAXY_PROJECT

sudo docker build . -t CALAXY_PROJECT --no-cache

sudo docker run -d CALAXY_PROJECT -p8000:8000 --name CALAXY_PROJECT-i