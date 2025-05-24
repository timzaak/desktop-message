### Tiny-PROTOCOL

PC <- tiny-protocol -> devices

tiny-protocol = mqtt server + http static file server + mdns-sd service discovery

mqtt server used [rmqtt](https://github.com/rmqtt/rmqtt), the [$sys System topic](https://github.com/rmqtt/rmqtt/blob/master/docs/en_US/sys-topic.md) you may need.

mdns-sd used [mdns-sd](https://github.com/keepsimple1/mdns-sd).

### how to use it with desktop software

It provides C header file and dynamic library, the [CmakeLists.txt](./CMakeLists.txt) is an example to use it in Windows environment.


### TODO:
1. -[ ] plugin config read from string rather than dir. https://github.com/rmqtt/rmqtt/issues/196
