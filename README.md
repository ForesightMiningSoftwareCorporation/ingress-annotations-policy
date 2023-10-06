# Kubewarden policy ingress-annotations-policy

## Description

This policy will inject annotations into Ingress Resources.

## Settings

```yaml
# List of annotations that needs to be added
annotations:
  priority: "[123]"
  cost-center: "^cc-\\d+$"
  nginx.ingress.kubernetes.io/modsecurity-snippet: |
    SecRuleEngine On
    SecRule &REQUEST_HEADERS:X-Azure-FDID \"@eq 0\"  \"log,deny,id:106,status:403,msg:\'Front Door ID not present\'\"
    SecRule REQUEST_HEADERS:X-Azure-FDID \"@rx ^(?!xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx).*$\"  \"log,deny,id:107,status:403,msg:\'Wrong Front Door ID\'\"
```

## License

`ingress-annotations-policy` is free and open source! All code in this repository is dual-licensed under either:

* MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
* Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
