# Concourse

Upgrade Concourse pipeline

```grit
engine marzano(0.1)
language yaml

`job: $_` => `foo: boo`
```

## Sample

```yaml
--
jobs:
- name: job
  public: true
  plan:
  - task: simple-task
    config:
      platform: linux
      image_resource:
        type: registry-image
        source: { repository: busybox }
      run:
        path: echo
        args: ["Hello world!"]
test: good
```

```yaml
--
jobs:
- name: like
  public: true
  plan:
  - task: simple-task
    config:
      platform: linux
      image_resource:
        type: registry-image
        source: { repository: busybox }
      run:
        path: echo
        args: ["Hello world!"]
test: good
```