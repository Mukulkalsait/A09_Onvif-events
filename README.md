# ONVIF Surveillance Platform

## Overview

This project aims to build a self-hosted surveillance management platform focused on giving users complete control over their IP cameras, events, recordings, and automation.

The system is designed around the idea that cameras should provide their own capabilities (motion detection, audio detection, PTZ control, and events), while the server acts as the central intelligence layer that manages, stores, and exposes this information.

Instead of building a complete NVR from scratch, this project integrates with existing recording solutions while providing a custom backend layer for camera management, event processing, automation, and future expansion.

## Core Idea

The platform acts as a bridge between IP cameras and user applications.

Cameras communicate through standard protocols such as:

- ONVIF
- RTSP
- HTTP

The platform handles:

- Camera discovery
- Camera configuration
- Device information
- PTZ control
- Event handling
- Recording metadata
- Notifications
- Automation
- User access

The goal is to create a flexible system where cameras, storage engines, and user interfaces are independent components.

## Architecture Philosophy

The project follows a modular design.

Each major capability is separated into independent components:

- Camera communication layer
- Event processing system
- Storage layer
- API server
- Notification system
- User interface

This allows individual parts to be replaced or extended without redesigning the entire system.

For example:

- ONVIF handles communication with cameras.
- Frigate can handle video recording.
- The custom backend manages events, metadata, automation, and user interaction.

## Goals

The long-term goal is to create a reliable and extensible surveillance platform that can:

- Automatically discover and manage cameras
- Monitor camera events in real time
- Provide a unified API for applications
- Control PTZ cameras
- Maintain searchable event history
- Send intelligent notifications
- Support multiple cameras and locations
- Provide a foundation for future AI-based features

## Technology Direction

The project is primarily built using:

- Rust for backend services
- Axum for APIs
- Tokio for asynchronous processing
- SQLite/PostgreSQL for data storage
- ONVIF and RTSP for camera communication
- React and TypeScript for future interfaces

## Current Status

The project is in the foundation stage.

Initial development focuses on:

- Designing the core architecture
- Building ONVIF communication
- Creating reusable camera abstractions
- Implementing reliable event handling

Future stages will expand toward complete camera management, automation, and user interfaces.
