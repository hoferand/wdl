# http

## get

```wdl
function get(url: string) -> HttpResponse|null
```

**Example**

```wdl
let response = http::get("http://example.org/");

if !response {
    log::error("Request failed!");
    order::cancel();
}

// use response
```

## post

```wdl
function post(url: string) -> HttpResponse|null
```

**Example**

```wdl
let response = http::post("https://example.org/");

if response {
    log::info(response.body);
}
```
