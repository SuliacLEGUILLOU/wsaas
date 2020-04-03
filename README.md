# Web Socket as a Service

This project aim to help system scale their websocket interface.
It should be used with a distributed or a lambda base system.

It is inspired by the Amazon Web Service API Gateway with the Websocket flavor,
but aim to remove most of the complexity of the service.

## Why

When you want to support websocket on a distributed system, you will often end up
in the situation where the event you want to forward to the front end is not detected
on the server holding the websocket connection. This server will provide you a web hook
interface so pushing and getting clients information is just a classic REST http interface.

## General interface description

This server rely on two environment variable to work (Those are my test setup):

```
export API_URI=localhost:80/websocket
export WS_HOSTNAME=localhost:8080
```

The `API_URI` parameter is the hostname + path that the WSAAS will use to push information to your system.

This path should support 3 different HTTP method:

- POST
Will be called when a new connection is created
- PUT
Will be called when the client push information through the websocket.
- DELETE
Will be called after the client close the connection. Use it to maintain integrity in your system as the

The POST request will contain a `ws_uri` parameter that will look like `<WS_HOSTNAME>/some_id`

This endpoint support the following method:

- PUT
Push information to the client
- DELETE
Close connection

## Interfaces and protocol details

### Websocket creation

POST request:
```JSON
Header: {
    "Authorization": "Content of the initial query Authorization header",
},
Body: {
    "ws_uri": "some_uri_to_use_later/id",
}
```

POST response:
```JSON
Body: {
    "code": "OK",
}
```
Sending any other code value will kill the connection

## Security

### Client side

TODO (I think I can just summon the https over the interface but double check WS security)

### Push to api

Have your api support https

### Server push

- Have a firewall/security groups so WSAAS only get talked to by your api
- TODO: HTTPS settings

## Additional settings

```
# Set log level
export LOG_LEVEL=INFO       # value: ERROR|WARN|INFO|DEBUG
export WS_TIMEOUT=30000     # Timeout value in ms
```