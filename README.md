# Armcontroller
This goal of this project is to transform a move from a checker ai into actions that a robotic arm can execute.

## Software stages
* get move from host
* transform move into endeffector path
** take speed into account
* calculate actuator forces required
** do feedforward control
* send to realtime armcontroller
