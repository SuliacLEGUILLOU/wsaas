# Web Socket as a Service

## Current status: ALPHA, Use at your own risk

The project aim is to help systems scale their websocket interfaces.
It should be used with a distributed or a lambda base system.

It is inspired by the Amazon Web Service [API Gateway](https://aws.amazon.com/api-gateway/) with the Websocket flavor,
but aims to remove most of the complexity of the service.

## Why

When you want to support websockets on a distributed system, you will often end up
in the situation where the event you want to forward to the front end is not detected
on the server holding the websocket connection. This server provides you with a web hook
interface so pushing and getting clients information is just a classic REST HTTP interface.

## General interface description

This server relies on two environment variables to work (Those are my test setup and the default value):

```Bash
export TARGET_ADDRESS=http://localhost:3000/websocket
export LOCAL_ADDRESS=http://localhost:8081
```

The `API_URI` parameter is the hostname + path that the WSAAS will use to push information to your system.

This path should support 3 different HTTP method:

- POST <endpoint>/<someId>
Will be called when a new connection is created
- PUT <endpoint>/<someId>
Will be called when the client push information through the websocket.
- DELETE <endpoint>/<someId>
Will be called after the client close the connection. Use it to maintain integrity in your system as the

The POST request will contain a `ws_uri` parameter that will look like `<WS_HOSTNAME>/<someId>`

This endpoint supports the following method:

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
Sending any other code value will kill the connection.

### Pushing data

Once you have validated the creation of the websocket, making any put request to the
http interface of the service will push the body of your request as it is.

### Listening to user data

As with the websocket creation query, you will see some PUT request sent to your api.

You can use the given url id to refer to the session data.

## Security

### Client side

TODO (I think I can just summon the https over the interface but double check WS security)

### Push to api

Have your API support HTTPS.

### Server push

- Have a firewall/security groups so WSAAS only get talked to by your api
- TODO: HTTPS settings

## Additional settings

```
export LOG_LEVEL=INFO       # value: ERROR|WARN|INFO|DEBUG
export WS_TIMEOUT=30000     # Timeout value in ms
export WS_PORT=8080         # Change the default WS post to listen to
export HTTP_PORT=8081       # Change the default HTTP post to listen to. Remember that it can have influence over the LOCAL_ADDRESS setting
```