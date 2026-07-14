I genuinely think this is a strong project, but I'd make one architectural adjustment that will pay off later.

Don't think of it as "a Rust service around Frigate."

Think of it as **a surveillance platform where Frigate is just one backend implementation.**

That subtle distinction affects how you design everything.

## The architecture I'd aim for

```
                    ┌─────────────────────────┐
                    │       Web / Mobile      │
                    └───────────┬─────────────┘
                                │
                          REST / WebSocket
                                │
                 ┌──────────────▼──────────────┐
                 │        Surveillance API      │
                 │        (Rust + Axum)         │
                 └──────────────┬──────────────┘
                                │
      ┌──────────────┬──────────┼──────────────┬──────────────┐
      ▼              ▼          ▼              ▼
 Camera Service   Event Bus   Storage     Notification
 ONVIF/PTZ        Internal    SQLite       Telegram/etc
 Discovery
      │
      │
      ▼
 Recording Provider
      │
      ├── Frigate
      ├── Scrypted (future)
      ├── Shinobi (future)
      └── Native (future?)
```

Notice something important:

Frigate is **not** the center.

Your backend is.

That means if you ever decide Frigate isn't right anymore, you replace one module instead of rewriting the entire system.

---

# Design it as modules

I'd split the project into independent crates.

```
surveillance/

├── api/
├── camera/
├── onvif/
├── rtsp/
├── events/
├── notifications/
├── recording/
├── storage/
├── auth/
├── common/
└── web/
```

Each crate should have one responsibility.

That makes the codebase much easier to maintain.

---

# Treat everything as an event

This is probably the biggest architectural recommendation I'd make.

Instead of calling notification functions directly:

```
Camera detects motion

↓

Send Telegram

↓

Store in DB

↓

Update UI

```

Do this:

```
Camera

↓

MotionDetected event

↓

Event Bus

↓

Notification Handler

↓

Database

↓

WebSocket

↓

Timeline

↓

Automation

```

One event.

Many consumers.

---

Example:

```
MotionDetected {
    camera_id,
    timestamp,
    confidence,
}
```

Now anything can subscribe.

```
Notification

Database

Recording Index

Statistics

Automation

Future AI

Logging

```

without modifying the camera code.

---

# Abstract the camera

Don't let the rest of the system know what ONVIF is.

Instead define a trait.

```rust
trait Camera {
    async fn snapshot();
    async fn move_ptz();
    async fn get_stream();
    async fn capabilities();
}
```

Today:

```
ONVIFCamera
```

Tomorrow:

```
HikvisionCamera

DahuaCamera

AmcrestCamera

VirtualCamera
```

Everything still works.

---

# Same for recording

Don't write code like

```
frigate.get_recordings()
```

Instead

```rust
trait RecordingProvider {
    async fn recordings();
    async fn clip();
    async fn delete();
}
```

Current implementation

```
FrigateProvider
```

Future

```
NativeProvider
```

No rewrite.

---

# Build your own event timeline

I actually think this becomes your killer feature.

Instead of:

```
Recording list
```

Build

```
08:11 Motion

08:12 Person

08:20 Camera moved

08:31 Audio

09:10 Doorbell

09:30 Tamper

```

Every event points into a recording.

This is far more useful than scrolling through hours of video.

---

# Your database should store metadata, not video

SQLite should contain things like:

```
Camera

CameraCapability

Event

Recording

RecordingSegment

PTZPreset

User

Notification

AutomationRule

DeviceHealth
```

Never try to store video blobs in SQLite.

Only paths and metadata.

---

# Add a plugin system early

Even if you don't expose plugins immediately, define extension points.

For example:

```
Notification Provider

↓

Telegram

Discord

Email

ntfy

Gotify

Signal

Webhook
```

Each implements a common interface.

Same for automations.

```
Motion

↓

Automation Engine

↓

Action
```

where actions can be:

```
Turn on light

Send notification

Take snapshot

Move PTZ

Call webhook

Run script
```

---

# Think about observability from day one

A surprising amount of surveillance software becomes hard to debug.

I'd include:

* structured logging (`tracing`)
* metrics
* health endpoints
* camera heartbeat
* recording status
* disk usage
* event processing latency

That will save you a lot of time.

---

# One thing I'd postpone

I would **not** build a React frontend immediately.

Initially, I'd focus on a clean REST API and WebSocket API.

Use:

* Swagger/OpenAPI
* simple HTML templates or API testing
* Frigate's UI for recordings

Once the backend stabilizes, then build a polished frontend.

That way you avoid maintaining two large codebases at once.

---

# A roadmap I'd follow

**Milestone 1 – Camera Foundation**

* ONVIF discovery
* Camera registration
* PTZ control
* Snapshot capture
* RTSP URL management

**Milestone 2 – Recording Integration**

* Frigate integration
* Recording lookup
* Clip export
* Timeline linking

**Milestone 3 – Event Engine**

* Motion events
* Audio events
* Tamper events
* SQLite storage
* Event history

**Milestone 4 – Notifications**

* Telegram
* ntfy
* Email
* Webhooks

**Milestone 5 – Multi-Camera Management**

* Camera groups
* Health monitoring
* Disk management
* User accounts

**Milestone 6 – Frontend**

* React dashboard
* Timeline
* Live view
* PTZ controls
* Settings

**Milestone 7 – Advanced Features**

* Automation rules
* Optional AI integration
* Analytics
* Reports
* Backup and restore

## Why I think this is worth building

This isn't just a "learning project." It sits at the intersection of several valuable backend domains:

* Asynchronous systems with Tokio
* Network protocols (ONVIF, RTSP, HTTP, WebSockets)
* Systems programming in Rust
* Database design and indexing
* Event-driven architecture
* Authentication and authorization
* Media pipeline integration
* API design
* Real-world deployment and observability

It's also modular enough that each milestone produces something usable. Even after Milestone 2 or 3, you'd already have a practical system running at home while continuing to evolve it into a polished portfolio project.

