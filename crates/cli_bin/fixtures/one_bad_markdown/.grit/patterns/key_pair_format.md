---
title: Key value pair
---

Find a key-value pair in Terraform HCL.

```grit
engine marzano(0.1)
language hcl

`$arg: $red` => `$arg: "Hello"`
```

## Matches a key-value pair

```hcl
default_address = "127.0.0.1"
default_message = upper("Incident: ${incident}")
default_options = {
  priority: "High",
  color: "Red"
}

incident_rules {
  # Rule number 1
  rule "down_server" "infrastructure" {
    incident = 100
    options  = var.override_options ? var.override_options : var.default_options
    server   = default_address
    message  = default_message
  }
}
```

```hcl
default_address = "127.0.0.1"
default_message = upper("Incident: ${incident}")
default_options = {
  priority: "Hello",
  color: "Hello"
}

incident_rules {
  # Rule number 1
  rule "down_server"     "infrastructure" {
    incident = 100
    options  = var.override_options ? var.override_options : var.default_options
    server   = default_address
    message  = default_message
  }
}
```
