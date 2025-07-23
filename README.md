# Sedimark Profile

Sedimark Profile is a simple app provided to Sedimark participants to host their personal profile data, like a digital business card. Participants are encouraged to host it on their premises to keep full soverignty over their data.

## Installation

You can run the profile server locally if you have Rust installed: just do `cargo run`. Alternatively, you can run it via Docker:

```bash
docker build -t profile .
docker run -d -p 3005:3005 -v ./data:/app/data -e API_KEY=password --name sedimark-profile profile
```

The API is then reachable at `http://localhost:3005`.

## Usage

The app can hold only one profile at a time. It exposes two sets of endpoints:

- _protected_: to create, edit or delete the profile. These endpoints are protected by an api key, and are not recommended to be exposed publicly.
- _profile_: to get the profile. No api key required. It is intended as a public endpoint for other participants to discover it.

A Bruno collection is provided in the _bruno_ directory to explore the API. Otherwise, let's go through it.

First, let's export the necessary env variables:

```bash
export PROFILE_URL=<app_url>
export API_KEY=<api_key>
```

Then create your profile:

```bash
curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bearer ${API_KEY}" -d '{
    "first_name": "Henry",
    "last_name": "Darnley",
    "company_name": "Stuart Ltd",
    "website": "https://henry.darnley",
    "image_url": "https://example.com/avatar.jpg"
}' "${PROFILE_URL}/protected"
```

To get it:

```bash
curl "${PROFILE_URL}/profile"
```

To make any edits:

```bash
curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bearer ${API_KEY}" -d '{
    "first_name": "James",
    "last_name": "Hepburn",
    "company_name": "Bothwell Ltd",
    "website": "https://james.hepburn",
    "image_url": "https://example.com/avatar.jpg"
}' "${PROFILE_URL}/protected"
```

To delete it:

```bash
curl -X DELETE -H "Authorization: Bearer ${API_KEY}" "${PROFILE_URL}/protected"
```
