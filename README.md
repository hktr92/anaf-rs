# ANAF API client

This is the unofficial ANAF WebService client, implemented in Rust.

> **Note:** The API client only supports the synchronous web services.

## Goals
- supports following APIs:
  - [x] Balance API;
  - [x] VAT Payer API;
  - [x] Farmers Registry API;
  - [x] Cult Registry API;

## Supported APIs

### Balance API

> **Note:** The Balance API is currently experimental and may change in the future.

This API supports following versions:
- V1 (default)


### VAT Payer API

> **Note:** This crate also supports the async VAT Payer API.

This API supports following versions:
- V8 (default)
- V7


### Farmers Registry API

This API supports following versions:
- V2 (default)

### Cult Registry API

This API supports following versions:
- V2 (default)

## Unsupported APIs

Currently, we don't plan to support following APIs:
- [ ] e-Factura API;
- [ ] e-Transport API;

These might be implemented in the future, if there is a demand for them. However, please note that these APIs will also require JWT authentication.