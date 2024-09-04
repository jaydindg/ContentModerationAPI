
# Content Moderation API

This project provides a simple API for checking, censoring, and replacing profanity in text content. It is built using Rust and the Actix-web framework.

## Table of Contents

- Installation
- Usage
- API Endpoints
  - Check Text
  - Censor Text
  - Replace Text
- Structs
  - IncomingReqBody
  - GrawlixQueryParams
- License

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/jaydindg/ContentModerationAPI.git
    cd ContentModerationAPI
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

## Usage

Send HTTP POST requests to the provided endpoints with the appropriate JSON payloads to check, censor, or replace profanity in text content.

## Request Body
**Request Body:**
```json
{
  "content": "string",
  "extra_filters": ["string"], // Optional
  "excludes": ["string"] // Optional
}
```


## API Endpoints

### Check Text

**Endpoint:** `/api/v1/check-text`

**Method:** `POST`

**Description:** This endpoint checks if the provided content contains profanity.

**Request Body:**
- `content`: The text content to be checked.
- `extra_filters` (optional): Additional words to be censored.
- `excludes` (optional): Words to be excluded from censorship.

**Returns:** `BOOL`
- `true` if the content contains profanity.
- `false` if the content does not contain profanity.

### Censor Text

**Endpoint:** `/api/v1/censor-text`

**Method:** `POST`

**Description:** This endpoint censors words that contain profanity and returns a friendly string.

**Request Body:**
- `content`: The text content to be censored.
- `extra_filters` (optional): Additional words to be censored.
- `excludes` (optional): Words to be excluded from censorship.

**Returns:** `String`
- The censored content if profanity is detected.
- The original content if no profanity is found.

### Replace Text

**Endpoint:** `/api/v1/replace-text`

**Method:** `POST`

**Description:** This endpoint replaces the censored words in the content with the provided grawlix string.

**Request Body:**
- `content`: The text content to be processed.
- `extra_filters` (optional): Additional words to be censored.
- `excludes` (optional): Words to be excluded from censorship.

**Query Parameters:**
- `grawlix`: The string to replace censored words.

**Returns:** `String`
- The content with censored words replaced by the grawlix string if profanity is detected.
- The original content if no profanity is found.
