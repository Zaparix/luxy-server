## Introduction
Luxy server is a REST based interface for RGB strip controller. The processing of these commands is done by a controller (e.g. RPI controller for MiLight protocol). The REST communication is the focus of Luxy. It is implemented in Rust.

## Prerequisites
OpenMilight (see https://www.torsten-traenkner.de/wissen/smarthome/openmilight.php)

## Build Commands
| Action  | Command                      |
|---------|------------------------------|
| Compile | `CONTROLLER=MILI cargo build` |
| Run     | `./start_dev.sh`             |
