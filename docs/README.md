# Docs App

TODO[chai]: add info docs on how to setup the app with Motif.

## FAQ

- Why is authentication not enabled on the docsite?

  A: The docsite is hosted on a different domain (currently `docs.grit.fyi`) than the web app. We authenticate using cookies, which are only shared within the same domain.

## Dev

```shell
yarn dev
```

## Test

Install Playwright if you haven't already:

```shell
yarn playwright install
```

Run the tests using Playwright against your local instance:

```shell
NEXT_PUBLIC_DOCS_URL=http://localhost:3200 yarn test
```
