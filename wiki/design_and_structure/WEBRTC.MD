# WebRTC
[return to list of guides](../README.MD)

```mermaid
sequenceDiagram
participant Engine
participant API
participant Peer
participant Client

    Engine->>API: Request peer
    API-->>Engine: Return peer info
    Engine->>Peer: Connect to peer
    loop New Client Connection
        Peer->>Engine: Notify new client connected
        Engine->>API: Check if client has access
        API-->>Engine: Access confirmation
        alt Client Has Access
            Engine->>Client: Stream video
            Engine->>Client: Handle interactions
        else Client Does Not Have Access
            Engine->>Client: Deny access
        end
    end
```

[return to list of guides](../README.MD)